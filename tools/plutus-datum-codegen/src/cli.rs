use clap::Parser;
// TODO: make non-annotation generate different DeserializeError that is simpler
//       and works with From<cbor_event:Error> only

#[derive(Debug, Default, Parser)]
#[clap()]
pub struct Cli {
    /// Location of cddl-codegen. e.g. ~/cddl-codegen/target/release/cddl-codegen (binary) or ~/cddl-codegen (repo)
    /// If it is a directory cargo run will be used and the /static/ dir there will be defaulted to.
    #[clap(short, long, value_parser, value_name = "CDDL_CODEGEN")]
    pub cddl_codegen: std::path::PathBuf,

    /// Input .cddl file to generate from. If this is a directory then it will read all *.cddl files and generate one output for each.
    #[clap(short, long, value_parser, value_name = "INPUT_FILE/INPUT_DIR")]
    pub input: std::path::PathBuf,

    /// Output directory for the generated code.
    #[clap(short, long, value_parser, value_name = "OUTPUT_DIR")]
    pub output: std::path::PathBuf,

    /// Change the directory of the static files.
    /// Does not need to be provided if --cddl-codegen is a directory, but will override that if it is.
    #[clap(short, long, value_parser, value_name = "STATIC_DIR")]
    pub static_dir: Option<std::path::PathBuf>,

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

    /// Generates a wasm_bindgen crate for wasm bindings
    #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = true)]
    pub wasm: bool,

    /// Derives serde::Serialize/serde::Deserialize for types to allow to/from JSON
    #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = false)]
    pub json_serde_derives: bool,

    /// Tags types with sonSchema derives and generates a crate to export them
    #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = false)]
    pub json_schema_export: bool,

    /// Generates a npm package.json along with build scripts
    #[clap(long, value_parser, action = clap::ArgAction::Set, default_value_t = false)]
    pub package_json: bool,
}

impl Cli {
    /// lib name from code i.e. with underscores
    pub fn lib_name_code(&self) -> String {
        self.lib_name.replace('-', "_")
    }
}
