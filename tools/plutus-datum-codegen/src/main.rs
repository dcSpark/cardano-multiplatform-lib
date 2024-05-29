use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::Path;

use cddl::{ast::*, token::*};
use clap::Parser;
use cli::Cli;

use utils::*;

mod cli;
mod dep_graph;
mod utils;

const MERGED_INPUT_DIR: &str = "MERGED_INPUT_DIR";
const CDDL_CODEGEN_EXTERN_MARKER: &str = "_CDDL_CODEGEN_EXTERN_TYPE_";
const CDDL_CODEGEN_RAW_BYTES_MARKER: &str = "_CDDL_CODEGEN_RAW_BYTES_TYPE_";

#[derive(Debug, Clone)]
struct Error(String);

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn verify_group(
    types: &BTreeMap<&str, BTreeSet<PlutusType>>,
    group: &Group,
    is_map: bool,
) -> Result<(), String> {
    for group_choice in group.group_choices.iter() {
        for (entry, _comma) in group_choice.group_entries.iter() {
            verify_group_entry(types, entry, is_map).map_err(|e| format!("{}: {}", entry, e))?;
        }
    }
    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
enum PlutusType {
    Bytes,
    Map,
    Array,
    Int,
    Ctor,
}

fn create_base_idents<'a>() -> BTreeMap<&'a str, BTreeSet<PlutusType>> {
    BTreeMap::from([
        ("uint", BTreeSet::from([PlutusType::Int])),
        ("int", BTreeSet::from([PlutusType::Int])),
        ("nint", BTreeSet::from([PlutusType::Int])),
        ("u32", BTreeSet::from([PlutusType::Int])),
        ("i32", BTreeSet::from([PlutusType::Int])),
        ("u64", BTreeSet::from([PlutusType::Int])),
        ("i64", BTreeSet::from([PlutusType::Int])),
        // prelude too
        ("bounded_bytes", BTreeSet::from([PlutusType::Bytes])),
        ("utf8_text", BTreeSet::from([PlutusType::Bytes])),
    ])
}

fn verify_ident(ident: &Identifier, is_key: bool) -> Result<(), String> {
    match ident.ident {
        // this can refer to valid standard prelude types
        "uint" | "int" | "nint" => Ok(()),
        // these are non-standard types referring to the cddl-codgen tool
        "u32" | "i32" | "u64" | "i64" => Ok(()),
        "bytes" | "bstr" => Err(format!(
            "arbitrary bytes not valid: {}. use bounded_bytes instead",
            ident
        )),
        "text" | "tstr" => Err(format!(
            "text not valid datum: {}. use utf8_bytes instead",
            ident
        )),
        // or invalid standard prelude types
        "bool" | "float" | "float16" | "float32" | "float64" | "float16-32" | "float32-64"
        | "tdate" | "time" | "number" | "biguint" | "bignint" | "bigint" | "integer"
        | "unsigned" | "decfrac" | "bigfloat" | "eb64url" | "eb64legacy" | "eb16"
        | "encoded-cbor" | "uri" | "b64url" | "b64legacy" | "regexp" | "mime-message"
        | "cbor-any" | "null" | "nil" | "undefined" | "true" | "false" => {
            Err(format!("invalid standard prelude type: {}", ident))
        }
        // refers to user-defined type
        other => {
            if is_key {
                verify_len(other.len())
            } else {
                // always okay since verified before
                Ok(())
            }
        }
    }
}

fn verify_tagged_type(
    types: &BTreeMap<&str, BTreeSet<PlutusType>>,
    tag: &Option<usize>,
    t: &Type,
) -> Result<PlutusType, String> {
    // tagged could ONLY mean tagged plutus constructor OR big integer!
    if *tag == Some(102) {
        if let Type2::Array { group, .. } = &t.type_choices[0].type1.type2 {
            assert_eq!(group.group_choices.len(), 1);
            let entries = &group.group_choices[0].group_entries;
            assert_eq!(entries.len(), 2);
            match (&entries[0].0, &entries[1].0) {
                (
                    GroupEntry::ValueMemberKey { ge: ge1, .. },
                    GroupEntry::ValueMemberKey { ge: ge2, .. },
                ) => {
                    // check first field is uint
                    for tc in &ge1.entry_type.type_choices {
                        match &tc.type1.type2 {
                            Type2::UintValue { .. } => (),
                            Type2::Typename { ident, .. } => {
                                // TODO: check aliases
                                assert_eq!(ident.to_string(), "uint");
                            }
                            _ => panic!("expected uint for const data discrim, found {:?}", tc),
                        }
                    }
                    // check other field is a list of datums
                    verify_datum_list(types, &ge2.entry_type)?;
                }
                _ => panic!(),
            }
            Ok(PlutusType::Ctor)
        } else {
            Err("102-tag must be 2-elem array (plutus tagged ctor)".to_owned())
        }
    } else if tag
        .map(|tag| (121..=127).contains(&tag) || (1280..=1400).contains(&tag))
        .unwrap_or(false)
    {
        verify_datum_list(types, t).map(|()| PlutusType::Ctor)
    } else if *tag == Some(2) || *tag == Some(3) {
        // can only be bigint (bytes)
        verify_bytes(types, t).map(|()| PlutusType::Bytes)
    } else {
        // invalid tag
        Err(format!("Invalid tag: {:?}", tag))
    }
}

fn verify_bytes(types: &BTreeMap<&str, BTreeSet<PlutusType>>, t: &Type) -> Result<(), String> {
    for tc in &t.type_choices {
        match &tc.type1.type2 {
            Type2::UTF8ByteString { value, .. } => verify_len(value.len()),
            Type2::B16ByteString { value, .. } => verify_len(value.len()),
            Type2::B64ByteString { value, .. } => verify_len(value.len()),
            Type2::Typename { ident, .. } => {
                let plutus_types = types.get(ident.ident).expect("Entered in first phase");
                if plutus_types.len() == 1 && plutus_types.contains(&PlutusType::Bytes) {
                    Ok(())
                } else {
                    Err(format!(
                        "Tag 2/3 (bigint) types must be ONLY bytes. Found: {plutus_types:?}"
                    ))
                }
            }
            other => Err(format!("Expected bytes, found: {:?}", other)),
        }?;
    }
    Ok(())
}

fn verify_group_entry(
    types: &BTreeMap<&str, BTreeSet<PlutusType>>,
    entry: &GroupEntry,
    is_map: bool,
) -> Result<(), String> {
    match entry {
        GroupEntry::ValueMemberKey { ge, .. } => {
            // keys are only serialized in cddl maps, not array structs
            if is_map {
                match &ge.member_key {
                    Some(key) => match key {
                        MemberKey::Type1 { t1, .. } => {
                            verify_type2(types, &t1.type2).map(|_| ())?
                        }
                        MemberKey::Bareword { ident, .. } => verify_ident(ident, true)?,
                        MemberKey::Value { value, .. } => match value {
                            Value::BYTE(bv) => match bv {
                                // TODO: technically can be longer but must be chunked
                                // you can't verify this encoding from the CDDL definition
                                // as it's an encoding detail so we'll just check to make sure
                                // that everything is <=64 and thus *every* encoding is valid
                                ByteValue::UTF8(bytes) => verify_len(bytes.len())?,
                                ByteValue::B16(bytes) => verify_len(bytes.len())?,
                                ByteValue::B64(bytes) => verify_len(bytes.len())?,
                            },
                            Value::UINT(_) | Value::INT(_) => {
                                // nothing to verify here
                            }
                            _ => return Err(format!("invalid key: {:?}", key)),
                        },
                        MemberKey::NonMemberKey { .. } => {
                            panic!("Please open a github issue with repro steps (non-member key)")
                        }
                    },
                    None => panic!(
                        "Please open a github issue with repro steps (member key without key)"
                    ),
                }
            }
            verify_type(types, &ge.entry_type).map(|_| ())
        }
        // verify type referred to here where it's defined instead
        GroupEntry::TypeGroupname { ge, .. } => verify_ident(&ge.name, false),
        GroupEntry::InlineGroup { group, .. } => verify_group(types, group, true),
    }
}

fn verify_len(len: usize) -> Result<(), String> {
    // technically could be bigger
    // TODO: force special serialization and allow >64 byte literals
    if len <= 64 {
        Ok(())
    } else {
        Err(format!("literal len too big: {}, limit is 64", len))
    }
}

fn verify_type(
    types: &BTreeMap<&str, BTreeSet<PlutusType>>,
    ty: &Type,
) -> Result<BTreeSet<PlutusType>, String> {
    let mut plutus_types = BTreeSet::new();
    for type_choice in ty.type_choices.iter() {
        plutus_types.extend(verify_type2(types, &type_choice.type1.type2)?);
    }
    Ok(plutus_types)
}

fn verify_type2(
    types: &BTreeMap<&str, BTreeSet<PlutusType>>,
    type2: &Type2,
) -> Result<BTreeSet<PlutusType>, String> {
    match type2 {
        Type2::UintValue { .. } => Ok([PlutusType::Int].into()),
        Type2::IntValue { .. } => Ok([PlutusType::Int].into()),
        Type2::TextValue { .. } => Err("Text not allowed. Please use utf8_bytes.".to_owned()),
        Type2::UTF8ByteString { value, .. } => {
            verify_len(value.len()).map(|()| [PlutusType::Bytes].into())
        }
        Type2::B16ByteString { value, .. } => {
            verify_len(value.len()).map(|()| [PlutusType::Bytes].into())
        }
        Type2::B64ByteString { value, .. } => {
            verify_len(value.len()).map(|()| [PlutusType::Bytes].into())
        }
        Type2::Typename { ident, .. } => match ident.ident {
            CDDL_CODEGEN_RAW_BYTES_MARKER => Ok([PlutusType::Bytes].into()),
            // we can't know what this is
            CDDL_CODEGEN_EXTERN_MARKER => Ok([
                PlutusType::Bytes,
                PlutusType::Map,
                PlutusType::Array,
                PlutusType::Ctor,
                PlutusType::Int,
            ]
            .into()),
            _ => verify_ident(ident, false).and_then(|()| {
                types
                    .get(ident.ident)
                    .cloned()
                    .ok_or_else(|| format!("Type alias not found: {}", ident.ident))
            }),
        },
        Type2::Map { group, .. } => {
            verify_group(types, group, true).map(|()| [PlutusType::Map].into())
        }
        Type2::Array { group, .. } => {
            verify_group(types, group, false).map(|()| [PlutusType::Array].into())
        }
        Type2::TaggedData { tag, t, .. } => verify_tagged_type(types, tag, t).map(|t| [t].into()),
        unsupported => Err(format!("Invalid (not plutus datum) type: {}", unsupported)),
    }
}

fn verify_datum_list(types: &BTreeMap<&str, BTreeSet<PlutusType>>, t: &Type) -> Result<(), String> {
    if t.type_choices.is_empty() {
        return Err(format!("Datum list empty: {:?}", t));
    }
    for tc in &t.type_choices {
        if let Type2::Array { group, .. } = &t.type_choices[0].type1.type2 {
            for gc in group.group_choices.iter() {
                for ge in gc.group_entries.iter() {
                    verify_group_entry(types, &ge.0, false)?;
                }
            }
        } else {
            return Err(format!("Datum list not array, found: {:?}", tc));
        }
    }
    Ok(())
}

fn verify_rule<'a>(
    types: &mut BTreeMap<&'a str, BTreeSet<PlutusType>>,
    cddl_rule: &'a Rule,
) -> Result<(), String> {
    match cddl_rule {
        Rule::Type { rule, .. } => {
            types.insert(rule.name.ident, verify_type(types, &rule.value)?);
        }
        Rule::Group { rule, .. } => {
            match &rule.entry {
                GroupEntry::InlineGroup { group, .. } => {
                    // TODO: be less strict on array type keys for plain groups but this is probably ok
                    verify_group(types, group, true)?;
                }
                x => panic!("Group rule with non-inline group? {:?}", x),
            }
        }
    }
    Ok(())
}

fn verify(cddl: &CDDL) -> Result<(), Box<dyn std::error::Error>> {
    let mut types = create_base_idents();
    for cddl_rule in
        dep_graph::topological_rule_order(cddl.rules.iter().collect::<Vec<_>>().as_slice())
    {
        let debug = format!("{cddl_rule:?}");
        let custom_serialize = debug.contains("@custom_serialize");
        let custom_deserialize = debug.contains("@custom_deserialize");
        if !custom_serialize && !custom_deserialize {
            verify_rule(&mut types, cddl_rule)
                .map_err(|e| format!("type {} not valid plutus datum: {}", cddl_rule.name(), e))?;
        }
    }
    Ok(())
}

fn is_struct(t: &Type) -> bool {
    if t.type_choices.len() == 1 {
        match &t.type_choices[0].type1.type2 {
            Type2::Map { group, .. } | Type2::Array { group, .. } => {
                if group.group_choices.len() == 1 {
                    group.group_choices[0].group_entries.len() > 1
                } else {
                    true
                }
            }
            Type2::TaggedData { t, .. } => is_struct(t),
            _ => false,
        }
    } else {
        true
    }
}

fn generate_utils(
    cddl: &CDDL,
    export_utf8_utils: bool,
    user_input_str_stripped: &str,
) -> Result<codegen::Scope, Box<dyn std::error::Error>> {
    let mut utils = codegen::Scope::new();
    utils
        .push_import("std::convert", "TryFrom", None)
        .push_import("cml_chain::plutus", "PlutusData", None)
        .push_import("cml_core::serialization", "Serialize", None)
        .push_import("cml_core::serialization", "Deserialize", None)
        .push_import("cml_core", "DeserializeError", None);
    for cddl_rule in &cddl.rules {
        let is_struct = match cddl_rule {
            Rule::Type { rule, .. } => is_struct(&rule.value),
            Rule::Group { .. } => true,
        };
        let is_user_defined = user_input_str_stripped.contains(&format!("{}=", cddl_rule.name()));
        if is_struct && is_user_defined {
            let rust_rule_name = convert_to_camel_case(&cddl_rule.name());
            utils.push_import("super", &rust_rule_name, None);
            let mut try_from = codegen::Impl::new(&rust_rule_name);
            // TODO: if we look into the structure we could avoid the bytes interace
            try_from
                .impl_trait("TryFrom<&PlutusData>")
                .associate_type("Error", "DeserializeError")
                .new_fn("try_from")
                .arg("datum", "&PlutusData")
                .ret("Result<Self, Self::Error>")
                .line("Self::from_cbor_bytes(&datum.to_cbor_bytes())");
            utils.push_impl(try_from);
            let mut from = codegen::Impl::new("PlutusData");
            // TODO: if we look into the structure we could avoid the bytes interace
            from.impl_trait(format!("From<&{}>", rust_rule_name))
                .new_fn("from")
                .arg("datum", format!("&{}", rust_rule_name))
                .ret("Self")
                .line("Self::from_cbor_bytes(&datum.to_cbor_bytes()).unwrap()");
            utils.push_impl(from);
        }
    }
    if export_utf8_utils {
        let mut serialize_utf8_bytes = codegen::Function::new("serialize_utf8_bytes");
        serialize_utf8_bytes
            .vis("pub")
            .generic("'se")
            .generic("W: Write")
            .arg("serializer", "&'se mut Serializer<W>")
            .arg("text", "&str")
            .arg("enc", "StringEncoding")
            .arg("force_canonical", "bool")
            .ret("cbor_event::Result<&'se mut Serializer<W>>")
            .line("serializer.write_bytes_sz(text.as_bytes(), enc.to_str_len_sz(text.len() as u64, force_canonical))");
        let mut deserialize_utf8_bytes = codegen::Function::new("deserialize_utf8_bytes");
        deserialize_utf8_bytes
            .vis("pub")
            .generic("R: BufRead + Seek")
            .arg("raw", "&mut cbor_event::de::Deserializer<R>")
            .ret("Result<(String, StringEncoding), DeserializeError>")
            .line("let (bytes, enc) = raw.bytes_sz()?;")
            .line("let text = String::from_utf8(bytes).map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)))?;")
            .line("Ok((text, enc.into()))");
        utils
            .push_import("cml_core", "DeserializeFailure", None)
            .push_import("cml_core::serialization", "StringEncoding", None)
            .push_import("cbor_event::se", "Serializer", None)
            .push_import("cbor_event::de", "Deserializer", None)
            .push_import("std::io", "BufRead", None)
            .push_import("std::io", "Seek", None)
            .push_import("std::io", "Write", None)
            .push_fn(serialize_utf8_bytes)
            .push_fn(deserialize_utf8_bytes);
    }
    Ok(utils)
}

fn generate_wasm_utils(
    cli: &Cli,
    cddl: &CDDL,
    user_input_str_stripped: &str,
) -> Result<codegen::Scope, Box<dyn std::error::Error>> {
    let mut utils = codegen::Scope::new();
    utils
        .push_import("std::convert", "TryFrom", None)
        .push_import("cml_chain_wasm::plutus", "PlutusData", None)
        .push_import("cml_core::serialization", "Serialize", None)
        .push_import("cml_core::serialization", "Deserialize", None)
        .push_import("wasm_bindgen", "JsError", None);
    for cddl_rule in &cddl.rules {
        let is_struct = match cddl_rule {
            Rule::Type { rule, .. } => is_struct(&rule.value),
            Rule::Group { .. } => true,
        };
        let is_user_defined = user_input_str_stripped.contains(&format!("{}=", cddl_rule.name()));
        if is_struct && is_user_defined {
            let rust_rule_name = convert_to_camel_case(&cddl_rule.name());
            utils.push_import("super", &rust_rule_name, None);
            let mut util_impl = codegen::Impl::new(&rust_rule_name);

            let mut from_datum = codegen::Function::new("try_from_datum");
            from_datum
                .vis("pub")
                .arg("datum", "&PlutusData")
                .ret("Result<Self, JsError>")
                .line(format!(
                    "{}::{}::try_from(datum.as_ref()).map(Into::into).map_err(Into::into)",
                    cli.lib_name_code(),
                    rust_rule_name
                ));
            util_impl.push_fn(from_datum);

            let mut to_datum = codegen::Function::new("to_datum");
            to_datum
                .vis("pub")
                .arg_ref_self()
                .ret("PlutusData")
                // we must use cml_chain's since JsError doesn't implement Debug thus no expect()/unwrap()
                .line("cml_chain::plutus::PlutusData::from_cbor_bytes(&self.to_cbor_bytes()).unwrap().into()");
            util_impl.push_fn(to_datum);

            utils.push_impl(util_impl);
        }
    }
    Ok(utils)
}

fn run_cddl_codegen(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let mut cddl_codegen_run = if cli.cddl_codegen.is_dir() {
        let mut run = std::process::Command::new("cargo");
        run.current_dir(&cli.cddl_codegen);
        // we must create it or else cli.output.canonicalize() will fail
        std::fs::create_dir_all(&cli.output)?;
        run.arg("run").arg("--").arg(format!(
            "--output={}",
            cli.output.canonicalize().unwrap().to_str().unwrap()
        ));
        if let Some(static_dir_override) = cli.static_dir.as_ref() {
            run.arg(format!(
                "--static-dir={}",
                static_dir_override
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
            ));
        }
        run
    } else {
        let mut run = std::process::Command::new(&cli.cddl_codegen);
        run.arg(format!("--output={}", cli.output.to_str().unwrap()))
            .arg(format!(
                "--static-dir={}",
                cli.static_dir
                    .as_ref()
                    .expect("--static-dir is mandatory when --cddl-codegen is an executable")
                    .to_str()
                    .unwrap()
            ));
        run
    };
    cddl_codegen_run
        .arg(format!(
            "--input={}",
            Path::new(MERGED_INPUT_DIR)
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
        ))
        .arg(format!("--lib-name={}", cli.lib_name));
    // hard-coded ones to interface with CML
    cddl_codegen_run
        .arg("--preserve-encodings=true")
        .arg("--canonical-form=true")
        .arg("--common-import-override=cml_core");
    if cli.json_serde_derives {
        cddl_codegen_run.arg("--wasm-cbor-json-api-macro=cml_core_wasm::impl_wasm_cbor_json_api");
    } else {
        cddl_codegen_run.arg("--wasm-cbor-json-api-macro=cml_core_wasm::impl_wasm_cbor_api");
    }
    cddl_codegen_run.arg("--wasm-conversions-macro=cml_core_wasm::impl_wasm_conversions");
    // user-passable optional ones
    cddl_codegen_run
        .arg(format!("--wasm={}", cli.wasm))
        .arg(format!("--json-serde-derives={}", cli.json_serde_derives))
        .arg(format!("--json-schema-export={}", cli.json_schema_export))
        .arg(format!("--package-json={}", cli.package_json));
    let cddl_codegen_run_result = cddl_codegen_run.output().unwrap();
    if !cddl_codegen_run_result.status.success() {
        return Err(format!(
            "cddl-codegen failed:\n{}",
            String::from_utf8(cddl_codegen_run_result.stderr).unwrap()
        )
        .into());
    }
    println!(
        "{}",
        String::from_utf8(cddl_codegen_run_result.stdout).unwrap()
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::fmt::Write;

    let cli = Cli::parse();

    // merge user's input with the tool's prelude
    let merged_input_dir = Path::new(MERGED_INPUT_DIR);
    let prelude_dir = Path::new("prelude");
    utils::copy_dir_all(prelude_dir, merged_input_dir)?;
    let mut prelude_input_map = Vec::new();
    utils::read_dir_to_string_map(&mut prelude_input_map, prelude_dir)?;
    let user_input_map = if cli.input.is_dir() {
        utils::copy_dir_all(&cli.input, merged_input_dir)?;
        let mut input_map = Vec::new();
        utils::read_dir_to_string_map(&mut input_map, &cli.input)?;
        input_map
    } else {
        std::fs::copy(&cli.input, merged_input_dir.join("lib.cddl"))?;
        let input_str = std::fs::read_to_string(&cli.input)?;
        vec![(Path::new("lib.cddl").to_path_buf(), input_str)]
    };

    // we need to parse the entire CDDL with the prelude/etc because
    // the cddl lib does not support incomplete CDDL definitions
    // we also need to declare these two idents.
    // these are inserted normally by cddl-codegen but we need them here too
    // so that the verification code that only uses cddl can parse.
    // We can't do it in the prelude.cddl or there will be duplicates in cddl-codegen's run
    let mut merged_input_str =
        "_CDDL_CODEGEN_EXTERN_TYPE_ = [0]\n\n_CDDL_CODEGEN_RAW_BYTES_TYPE_ = [1]\n\n".to_owned();
    for (_path, prelude_file_str) in prelude_input_map.iter() {
        write!(merged_input_str, "{}\n\n", prelude_file_str)?;
    }
    for (_path, user_file_str) in user_input_map.iter() {
        write!(merged_input_str, "{}\n\n", user_file_str)?;
    }
    let merged_cddl = cddl::parser::cddl_from_str(&merged_input_str, true)?;
    // we then need to filter out which definitions are from which
    // check that the input cddl is 100% a subset of the plutus datum CDDL
    verify(&merged_cddl)?;

    // run and export code via cddl-codegen to output folder
    run_cddl_codegen(&cli)?;

    // we might need to import RawBytesEncoding from cml_crypto
    // a better solution long-term might be to refactor CML and place it in cml_core::serialization
    fn prepend_import_raw_bytes_encoding(dir: impl AsRef<Path>) -> std::io::Result<()> {
        for entry_res in std::fs::read_dir(&dir)? {
            let entry = entry_res?;
            if entry.file_type()?.is_dir() {
                prepend_import_raw_bytes_encoding(entry.path())?;
            } else if entry.path().as_path().ends_with("serialization.rs") {
                let mut serialization_rs = std::fs::OpenOptions::new()
                    .append(true)
                    .open(entry.path().as_path())
                    .unwrap();
                serialization_rs.write_all("use cml_crypto::RawBytesEncoding;\n".as_bytes())?;
            }
        }
        Ok(())
    }
    prepend_import_raw_bytes_encoding(cli.output.join("rust").join("src"))?;

    // generate utilty functions
    let export_utf8_utils = user_input_map
        .iter()
        .any(|(_path, file_str)| file_str.contains("utf8_text"));
    // to check for assignment (to verify which types are user-defined)
    // we process the user input by striping whitespace and also sockets
    let stripped_user_input_str = user_input_map
        .iter()
        .map(|(_path, file_str)| {
            file_str
                .chars()
                .filter(|c| !c.is_whitespace() && *c != '/')
                .collect::<String>()
        })
        .collect::<String>();
    let utils = generate_utils(&merged_cddl, export_utf8_utils, &stripped_user_input_str)?;
    std::fs::write(
        cli.output.join("rust").join("src").join("utils.rs"),
        utils.to_string(),
    )?;
    let mut rust_lib = std::fs::OpenOptions::new()
        .append(true)
        .open(cli.output.join("rust").join("src").join("lib.rs"))
        .unwrap();
    rust_lib.write_all("pub mod utils;\n".as_bytes())?;
    if cli.wasm {
        // we need to change all imports from cml_chain to cml_chain_wasm
        fn swap_to_wasm_imports(dir: impl AsRef<Path>) -> std::io::Result<()> {
            for entry_res in std::fs::read_dir(&dir)? {
                let entry = entry_res?;
                if entry.file_type()?.is_dir() {
                    swap_to_wasm_imports(entry.path())?;
                } else {
                    let orig = std::fs::read_to_string(entry.path().as_path())?;
                    std::fs::write(
                        entry.path().as_path(),
                        orig.replace("use cml_chain::", "use cml_chain_wasm::"),
                    )?;
                }
            }
            Ok(())
        }
        swap_to_wasm_imports(cli.output.join("wasm").join("src"))?;

        // utils
        let wasm_utils = generate_wasm_utils(&cli, &merged_cddl, &stripped_user_input_str)?;
        std::fs::write(
            cli.output.join("wasm").join("src").join("utils.rs"),
            wasm_utils.to_string(),
        )?;
        let mut wasm_lib = std::fs::OpenOptions::new()
            .append(true)
            .open(cli.output.join("wasm").join("src").join("lib.rs"))
            .unwrap();
        wasm_lib.write_all("pub mod utils;\n".as_bytes())?;
    }

    // hook into CML
    let mut rust_cargo = std::fs::OpenOptions::new()
        .append(true)
        .open(cli.output.join("rust").join("Cargo.toml"))
        .unwrap();
    rust_cargo.write_all("cml-core = \"5.3.1\"\n".as_bytes())?;
    rust_cargo.write_all("cml-chain = \"5.3.1\"\n".as_bytes())?;
    rust_cargo.write_all("cml-crypto = \"5.3.1\"\n".as_bytes())?;
    if cli.wasm {
        let mut wasm_cargo = std::fs::OpenOptions::new()
            .append(true)
            .open(cli.output.join("wasm").join("Cargo.toml"))
            .unwrap();
        wasm_cargo.write_all("cml-core = \"5.3.1\"\n".as_bytes())?;
        wasm_cargo.write_all("cml-core-wasm = \"5.3.1\"\n".as_bytes())?;
        wasm_cargo.write_all("cml-chain = \"5.3.1\"\n".as_bytes())?;
        wasm_cargo.write_all("cml-chain-wasm = \"5.3.1\"\n".as_bytes())?;
        // needed for cml-core's cbor/json macros
        wasm_cargo.write_all("hex = \"0.4.3\"\n".as_bytes())?;
    }
    Ok(())
}
