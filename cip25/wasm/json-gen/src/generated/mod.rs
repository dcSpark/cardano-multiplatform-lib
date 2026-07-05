macro_rules! gen_json_schema {
    ($name:ty) => {
        let dest_path =
            std::path::Path::new(&"schemas").join(&format!("{}.json", stringify!($name)));
        std::fs::write(
            &dest_path,
            serde_json::to_string_pretty(&schemars::schema_for!($name)).unwrap(),
        )
        .unwrap();
    };
}

pub fn export_schemas() {
    let schema_path = std::path::Path::new(&"schemas");
    if !schema_path.exists() {
        std::fs::create_dir(schema_path).unwrap();
    }
    gen_json_schema!(cml_cip25::CIP25Metadata);
    gen_json_schema!(cml_cip25::CIP25Version);
    gen_json_schema!(cml_cip25::CIP25ChunkableString);
    gen_json_schema!(cml_cip25::CIP25FilesDetails);
    gen_json_schema!(cml_cip25::CIP25LabelMetadata);
    gen_json_schema!(cml_cip25::CIP25MetadataDetails);
    gen_json_schema!(cml_cip25::CIP25String64);
}
