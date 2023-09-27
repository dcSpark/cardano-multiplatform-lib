use cml_cip25::*;

fn main() {
    macro_rules! gen_json_schema {
        ($name:ident) => {
            let dest_path =
                std::path::Path::new(&"schemas").join(&format!("{}.json", stringify!($name)));
            std::fs::write(
                &dest_path,
                serde_json::to_string_pretty(&schemars::schema_for!($name)).unwrap(),
            )
            .unwrap();
        };
    }
    let schema_path = std::path::Path::new(&"schemas");
    if !schema_path.exists() {
        std::fs::create_dir(schema_path).unwrap();
    }
    gen_json_schema!(CIP25Metadata);
    gen_json_schema!(CIP25Version);
    gen_json_schema!(ChunkableString);
    gen_json_schema!(FilesDetails);
    gen_json_schema!(LabelMetadata);
    gen_json_schema!(MetadataDetails);
    gen_json_schema!(String64);
}
