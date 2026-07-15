fn main() {
    cml_multi_era_json_schema_gen::export_schemas();
    // cddl-codegen:insert-start
    cml_chain_json_schema_gen::export_schemas();
    // cddl-codegen:insert-end
}
