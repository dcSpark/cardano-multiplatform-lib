use std::collections::{BTreeMap, BTreeSet};

use cli::Cli;
use clap::Parser;

use cddl::{ast::*, token::*};

mod cli;
mod dep_graph;

fn verify_group(types: &BTreeMap<&str, BTreeSet<PlutusType>>, group: &Group, is_map: bool) -> Result<(), String> {
    for group_choice in group.group_choices.iter() {
        for (entry, _comma) in group_choice.group_entries.iter() {
            verify_group_entry(types, &entry, is_map).map_err(|e| format!("{}: {}", entry, e))?;
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

// fn ident_to_type(ident: &Identifier) -> Result<PlutusType, String> {

// }

fn create_base_idents<'a>() -> BTreeMap<&'a str, BTreeSet<PlutusType>> {
    BTreeMap::from([
        ("uint", BTreeSet::from([PlutusType::Int])),
        ("int", BTreeSet::from([PlutusType::Int])),
        ("nint", BTreeSet::from([PlutusType::Int])),
        ("u32", BTreeSet::from([PlutusType::Int])),
        ("i32", BTreeSet::from([PlutusType::Int])),
        ("u64", BTreeSet::from([PlutusType::Int])),
        ("i64", BTreeSet::from([PlutusType::Int])),
        // TODO: would be nice to use @custom_serialize to use text
        ("text", BTreeSet::from([PlutusType::Bytes])),
        ("tstr", BTreeSet::from([PlutusType::Bytes])),
        ("bytes", BTreeSet::from([PlutusType::Bytes])),
        ("bstr", BTreeSet::from([PlutusType::Bytes])),
    ])
}

fn verify_ident(ident: &Identifier, is_key: bool) -> Result<(), String> {
    match ident.ident {
        // this can refer to valid standard prelude types
        "uint"       |
        "int"        |
        "nint"       |
        "text"       |
        "tstr"       |
        "bytes"      |
        "bstr"       => Ok(()),
        // these are non-standard types referring to the cddl-codgen tool
        "u32"        |
        "i32"        |
        "u64"        |
        "i64"        => Ok(()),
        // or invalid standard prelude types
        "bool"       |
        "float"      |
        "float16"    |
        "float32"    |
        "float64"    |
        "float16-32" |
        "float32-64" |
        "tdate"      |
        "time"       |
        "number"     |
        "biguint"    |
        "bignint"    |
        "bigint"     |
        "integer"    |
        "unsigned"   |
        "decfrac"    |
        "bigfloat"   |
        "eb64url"    |
        "eb64legacy" |
        "eb16"       |
        "encoded-cbor" |
        "uri"        |
        "b64url"     |
        "b64legacy"  |
        "regexp"     |
        "mime-message" |
        "cbor-any"   |
        "null"       |
        "nil"        |
        "undefined"  |
        "true"       |
        "false" => Err(format!("invalid standard prelude type: {}", ident)),
        // refers to user-defined type
        other => if is_key {
            verify_len(other.len())
        } else {
            // always okay since verified before
            Ok(())
        }
    }
}

fn verify_tagged_type(types: &BTreeMap<&str, BTreeSet<PlutusType>>, tag: &Option<usize>, t: &Type) -> Result<PlutusType, String> {
    // tagged could ONLY mean tagged plutus constructor OR big integer!
    if *tag == Some(102) {
        if let Type2::Array{ group, .. } = &t.type_choices[0].type1.type2 {
            assert_eq!(group.group_choices.len(), 1);
            let entries = &group.group_choices[0].group_entries;
            assert_eq!(entries.len(), 2);
            match (&entries[0].0, &entries[1].0) {
                (GroupEntry::ValueMemberKey{ ge: ge1, .. }, GroupEntry::ValueMemberKey{ ge: ge2, .. }) => {
                    println!("matched");
                    println!("[0] = {:?}\n", ge1.entry_type);
                    println!("[1] = {:?}\n", ge2.entry_type);
                    // check first field is uint
                    for tc in &ge1.entry_type.type_choices {
                        match &tc.type1.type2 {
                            Type2::UintValue { .. } => (),
                            Type2::Typename { ident, .. } => {
                                // TODO: check aliases
                                assert_eq!(ident.to_string(), "uint");
                            },
                            _ => panic!("expected uint for const data discrim, found {:?}", tc),
                        }
                    }
                    // check other field is a list of datums
                    verify_datum_list(types, &ge2.entry_type)?;
                },
                _ => panic!(),
            }
            Ok(PlutusType::Ctor)
        } else {
            Err(format!("102-tag must be 2-elem array (plutus tagged ctor)"))
        }
    } else if tag.map(|tag| (tag >= 121 && tag <= 127) || (tag >= 1280 && tag <= 1400)).unwrap_or(false) {
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
            Type2::UTF8ByteString{ value, .. } => verify_len(value.len()),
            Type2::B16ByteString{ value, .. } => verify_len(value.len()),
            Type2::B64ByteString{ value, .. } => verify_len(value.len()),
            Type2::Typename{ ident, .. } => {
                let plutus_types = types.get(ident.ident).expect("Entered in first phase");
                if plutus_types.len() == 1 && plutus_types.contains(&PlutusType::Bytes) {
                    Ok(())
                } else {
                    Err(format!("Tag 2/3 (bigint) types must be ONLY bytes. Found: {plutus_types:?}"))
                }
            },
            other => Err(format!("Expected bytes, found: {:?}", other)),
        }?;
    }
    Ok(())
}

fn verify_group_entry(types: &BTreeMap<&str, BTreeSet<PlutusType>>, entry: &GroupEntry, is_map: bool) -> Result<(), String> {
    match entry {
        GroupEntry::ValueMemberKey { ge, .. } => {
            // keys are only serialized in cddl maps, not array structs
            if is_map {
                match &ge.member_key {
                    Some(key) => match key {
                        MemberKey::Type1 { t1, .. } => verify_type2(types, &t1.type2).map(|_| ())?,
                        MemberKey::Bareword { ident, .. } => verify_ident(&ident, true)?,
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
                        }
                        MemberKey::NonMemberKey { .. } => {
                            panic!("Please open a github issue with repro steps (non-member key)")
                        }
                    },
                    None => panic!("Please open a github issue with repro steps (member key without key)"),
                }
            }
            verify_type(types, &ge.entry_type).map(|_| ())
        },
        // verify type referred to here where it's defined instead
        GroupEntry::TypeGroupname { ge, .. } => verify_ident(&ge.name, false),
        GroupEntry::InlineGroup { group, .. } => verify_group(types, &group, true),
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

fn verify_type<'a>(types: &BTreeMap<&str, BTreeSet<PlutusType>>, ty: &'a Type) -> Result<BTreeSet<PlutusType>, String> {
    let mut plutus_types = BTreeSet::new();
    for type_choice in ty.type_choices.iter() {
        plutus_types.extend(verify_type2(types, &type_choice.type1.type2)?);
    }
    Ok(plutus_types)
}

fn verify_type2(types: &BTreeMap<&str, BTreeSet<PlutusType>>, type2: &Type2) -> Result<BTreeSet<PlutusType>, String> {
    match type2 {
        Type2::UintValue { .. } => Ok([PlutusType::Int].into()),
        Type2::IntValue { .. } => Ok([PlutusType::Int].into()),
        Type2::TextValue { value, .. } => verify_len(value.len()).map(|()| [PlutusType::Int].into()),
        Type2::UTF8ByteString { value, .. } => verify_len(value.len()).map(|()| [PlutusType::Int].into()),
        Type2::B16ByteString { value, .. } => verify_len(value.len()).map(|()| [PlutusType::Int].into()),
        Type2::B64ByteString { value, .. } => verify_len(value.len()).map(|()| [PlutusType::Int].into()),
        Type2::Typename { ident, .. } => verify_ident(&ident, false)
            .and_then(|()| types.get(ident.ident).cloned().ok_or_else(|| format!("Type alias not found: {}", ident.ident))),
        Type2::Map { group, .. } => verify_group(types, group, true).map(|()| [PlutusType::Int].into()),
        Type2::Array { group, .. } => verify_group(types, group, false).map(|()| [PlutusType::Int].into()),
        Type2::TaggedData { tag, t, .. } => verify_tagged_type(types, tag, t).map(|t| [t].into()),
        unsupported => Err(format!("Invalid (not plutus datum) type: {}", unsupported)),
    }
}

fn verify_datum_list(types: &BTreeMap<&str, BTreeSet<PlutusType>>, t: &Type) -> Result<(), String> {
    if t.type_choices.is_empty() {
        return Err(format!("Datum list empty: {:?}", t));
    }
    for tc in &t.type_choices {
        if let Type2::Array{ group, .. } = &t.type_choices[0].type1.type2 {
            for gc in group.group_choices.iter() {
                for ge in gc.group_entries.iter() {
                    verify_group_entry(types, &ge.0, false)?;
                }
            }
        } else {
            return Err(format!("Datum list not array, found: {:?}", tc))
        }
    }
    Ok(())
}

fn verify_rule<'a>(types: &mut BTreeMap<&'a str, BTreeSet<PlutusType>>, cddl_rule: &'a Rule) -> Result<(), String> {
    match cddl_rule {
        Rule::Type{ rule, .. } => {
            types.insert(rule.name.ident, verify_type(types, &rule.value)?);
        },
        Rule::Group{ rule, .. } => {
            match &rule.entry {
                GroupEntry::InlineGroup{ group, .. } => {
                    // TODO: be less strict on array type keys for plain groups but this is probably ok
                    verify_group(types, &group, true)?;
                },
                x => panic!("Group rule with non-inline group? {:?}", x),
            }
        },
    }
    Ok(())
}

fn verify(cli: &Cli, cddl: &CDDL) -> Result<(), Box<dyn std::error::Error>> {
    let mut types = create_base_idents();
    for cddl_rule in dep_graph::topological_rule_order(cddl.rules.iter().collect::<Vec<_>>().as_slice()) {
        verify_rule(&mut types, cddl_rule).map_err(|e| format!("type {} not valid metadata: {}", cddl_rule.name(), e))?;
    }
    Ok(())
}

fn generate_utils(cddl: &CDDL) -> Result<codegen::Scope, Box<dyn std::error::Error>> {
    let mut utils = codegen::Scope::new();
    for cddl_rule in &cddl.rules {
        let mut try_from = codegen::Impl::new(cddl_rule.name());
        // TODO: if we look into the structure we could avoid the bytes interace
        try_from
            .impl_trait("TryFrom<&PlutusData>")
            .new_fn("try_from")
            .arg("datum", "&PlutusData")
            .ret("Result<Self, Self::Err>")
            .line("Self::from_cbor_bytes(&datum.to_cbor_bytes())");
        utils.push_impl(try_from);
        let mut try_from = codegen::Impl::new("PlutusData");
        // TODO: if we look into the structure we could avoid the bytes interace
        try_from
            .impl_trait(format!("From<&{}>", cddl_rule.name()))
            .new_fn("try_from")
            .arg("datum", format!("&{}", cddl_rule.name()))
            .ret("Self")
            .line("Self::from_cbor_bytes(&datum.to_cbor_bytes()).unwrap()");
        utils.push_impl(try_from);
    }
    Ok(utils)
}


fn run_cddl_codegen(cli: &Cli) -> Result<(), String> {
    let mut cddl_codegen_run = std::process::Command::new(&cli.cddl_codegen);
    cddl_codegen_run.arg(format!("--input={}", cli.input.to_str().unwrap()));
    cddl_codegen_run.arg(format!("--output={}",cli.output.to_str().unwrap()));
    cddl_codegen_run.arg(format!("--lib-name={}", cli.lib_name));
    cddl_codegen_run.arg("--preserve-encodings=true");
    cddl_codegen_run.arg("--canonical-form=true");
    cddl_codegen_run.arg("--json-serde-derives=true");
    cddl_codegen_run.arg("--json-schema-export=true");
    cddl_codegen_run.arg("--common-import-override=cml_core");
    cddl_codegen_run.arg("--wasm-cbor-json-api-macro=cml_core_wasm::impl_wasm_cbor_json_api");
    cddl_codegen_run.arg("--wasm-conversions-macro=cml_core_wasm::impl_wasm_conversions");
    let cddl_codegen_run_result = cddl_codegen_run.output().unwrap();
    if !cddl_codegen_run_result.status.success() {
        return Err(format!("{}", String::from_utf8(cddl_codegen_run_result.stderr).unwrap()));
    }
    // let mut lib_rs = std::fs::OpenOptions::new()
    //     .append(true)
    //     .open(cli.output.join("rust/src/lib.rs"))
    //     .unwrap();
    // some external files/tests pasted in might need this
    // lib_rs
    //     .write_all("\nuse serialization::*;\n".as_bytes())
    //     .unwrap();
    // copy external file in too (if needed) too
    // if let Some(external_rust_file_path) = external_rust_file_path {
    //     let extern_rs = std::fs::read_to_string(external_rust_file_path).unwrap();
    //     lib_rs.write_all("\n\n".as_bytes()).unwrap();
    //     lib_rs.write_all(extern_rs.as_bytes()).unwrap();
    // }
    // let deser_test_rs = std::fs::read_to_string(
    //     std::path::PathBuf::from_str("tests")
    //         .unwrap()
    //         .join("deser_test"),
    // )
    // .unwrap();
    // lib_rs.write_all("\n\n".as_bytes()).unwrap();
    // lib_rs.write_all(deser_test_rs.as_bytes()).unwrap();
    // let test_rs = std::fs::read_to_string(test_path.join("tests.rs")).unwrap();
    // lib_rs.write_all("\n\n".as_bytes()).unwrap();
    // lib_rs.write_all(test_rs.as_bytes()).unwrap();
    // std::mem::drop(lib_rs);
    // // add extra deps used within tests
    // let mut cargo_toml = std::fs::OpenOptions::new()
    //     .append(true)
    //     .open(test_path.join(format!("{export_path}/rust/Cargo.toml")))
    //     .unwrap();
    // for dep in test_deps {
    //     cargo_toml.write_all(dep.as_bytes()).unwrap();
    // }
    // std::mem::drop(cargo_toml);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let cddl_in = std::fs::read_to_string("input.cddl").expect("input.cddl file not present or could not be opened");
    let cddl = cddl::parser::cddl_from_str(&cddl_in, true)?;
    // check that the input cddl is 100% a subset of the plutus datum CDDL
    verify(&cli, &cddl)?;
    // run and export code via cddl-codegen to output folder
    run_cddl_codegen(&cli)?;
    // generate utilty functions
    let utils = generate_utils(&cddl)?;
    std::fs::write(cli.output.join("src").join("utils.rs"), utils.to_string())?;

    Ok(())
}
