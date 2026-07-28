use cml_core::json_schema_gen::Registrar;
use cml_core::json_schema_gen::check_schema_ref_closure;

pub fn add_schemas(generator: &mut schemars::SchemaGenerator) {
    let mut reg = Registrar::new(generator);
    reg.add::<cml_cip25::CIP25ChunkableString>();
    reg.add::<cml_cip25::CIP25FilesDetails>();
    reg.add::<cml_cip25::CIP25LabelMetadata>();
    reg.add::<cml_cip25::CIP25Metadata>();
    reg.add::<cml_cip25::CIP25MetadataDetails>();
    reg.add::<cml_cip25::CIP25String64>();
    reg.add::<cml_cip25::utils::CIP25MiniMetadataDetails>();
}

pub fn export_schemas() {
    let schema_path = std::path::Path::new("schemas");
    if !schema_path.exists() {
        std::fs::create_dir(schema_path).unwrap();
    }
    let mut generator = schemars::SchemaGenerator::default();
    add_schemas(&mut generator);
    let meta_schema = generator.settings().meta_schema.clone();
    let definitions_path = generator.settings().definitions_path.to_string();
    let mut document = serde_json::Map::new();
    if let Some(meta_schema) = meta_schema {
        document.insert("$schema".to_owned(), meta_schema.into_owned().into());
    }
    document.insert("title".to_owned(), "cml_cip25".into());
    document.insert("$defs".to_owned(), generator.take_definitions(true).into());
    let document = serde_json::Value::Object(document);
    check_schema_ref_closure(&document, &definitions_path);
    std::fs::write(
        schema_path.join("cml_cip25.schema.json"),
        serde_json::to_string_pretty(&document).unwrap(),
    )
    .unwrap();
}
