fn main() {
    cml_multi_era_json_schema_gen::export_schemas();
    // We export ALL of the JSON types included in cml-chain
    cml_chain_json_schema_gen::export_schemas();
}
