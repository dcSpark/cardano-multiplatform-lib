use clap::Parser;
// TODO: make non-annotation generate different DeserializeError that is simpler
//       and works with From<cbor_event:Error> only

#[derive(Debug, Default, Parser)]
#[clap()]
pub struct Cli {
    /// Command to call cddl-codegen. e.g. ~/cddl-codegen/target/release/cddl-codegen
    #[clap(short, long, value_parser, value_name = "CDDL_CODEGEN")]
    pub cddl_codegen: String,

    /// Input .cddl file to generate from. If this is a directory then it will read all *.cddl files and generate one output for each.
    #[clap(short, long, value_parser, value_name = "INPUT_FILE/INPUT_DIR")]
    pub input: std::path::PathBuf,

    /// Output directory for the generated code.
    #[clap(short, long, value_parser, value_name = "OUTPUT_DIR")]
    pub output: std::path::PathBuf,

    // /// Change the directory of the static files
    // #[clap(short, long, value_parser, value_name = "STATIC_DIR", default_value_os_t = std::path::PathBuf::from("static"))]
    // pub static_dir: std::path::PathBuf,

    /// Name to use for exported library.
    /// Will be used directly for rust lib and will have -wasm appended for the wasm bindings.
    /// This will appear EXACTLY as-is in the Cargo.toml's. use Cli::lib_name_code() for use in rust code
    #[clap(
        long,
        value_parser,
        value_name = "EXPORT_LIB_NAME",
        default_value = "cddl-lib"
    )]
    pub lib_name: String,

    // TODO: rest of these

    // /// Include additional information about where deserialization errors are encountered. This will slightly increase code size.
    // #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = true)]
    // pub annotate_fields: bool,

    // /// Generate to_bytes() / from_bytes() methods on all types
    // #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = true)]
    // pub to_from_bytes_methods: bool,

    // /// Generate byte string definitions as new rust types (TODO: look into this or remove it)
    // #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = false)]
    // pub binary_wrappers: bool,

    // /// Preserves CBOR encoding upon deserialization e.g. definite vs indefinite, map ordering
    // #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = false)]
    // pub preserve_encodings: bool,

    // /// Allows serialization to canonical CBOR. if preserve-encodings is enabled, this will be as a toggle on serialization functions
    // #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = false)]
    // pub canonical_form: bool,

    // /// Generates a wasm_bindgen crate for wasm bindings
    // #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = true)]
    // pub wasm: bool,

    // /// Derives serde::Serialize/serde::Deserialize for types to allow to/from JSON
    // #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = false)]
    // pub json_serde_derives: bool,

    // /// Tags types with sonSchema derives and generates a crate to export them
    // #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = false)]
    // pub json_schema_export: bool,

    // /// Generates a npm package.json along with build scripts
    // #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = false)]
    // pub package_json: bool,

    // /// Location override for default common types (error, serialization, etc)
    // /// This is useful for integrating into an exisitng project that is based on
    // /// these types.
    // #[clap(long, value_parser, value_name = "COMMON_IMPORT_OVERRIDE")]
    // common_import_override: Option<String>,

    // /// An external macro to be called instead of manually emitting functions for
    // /// conversions to/from CBOR bytes or JSON.
    // /// If the macro is scoped it will be imported using the supplied path.
    // /// e.g. foo::bar::qux will result in importing foo::bar::qux and then
    // /// calling qux!(A); for every struct A with a CBOR/JSON API
    // #[clap(long, value_parser)]
    // pub wasm_cbor_json_api_macro: Option<String>,

    // /// An external macro to be called instead of manually emitting traits for
    // /// WASM conversions to/from the inner rust type + AsRef.
    // /// If the macro is scoped it will be imported using the supplied path.
    // /// e.g. foo::bar::qux will result in importing foo::bar::qux and then
    // /// calling qux!(rust::path::A, A); for every struct A with a CBOR/JSON API
    // #[clap(long, value_parser)]
    // pub wasm_conversions_macro: Option<String>,
}