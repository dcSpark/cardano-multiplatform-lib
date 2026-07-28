//! Hand-written umbrella exporter for the `cardano-multiplatform-lib` npm package.
//!
//! The generated per-crate json-gen crates each write their OWN one-document
//! `schemas/<lib>.schema.json` via `export_schemas()`. The cml package publishes the union of
//! chain + cip25 + cip36, so this crate instead threads all three crates' `add_schemas` (public
//! exactly for this composition, per cddl-codegen's `--json-schema-export` docs) through a
//! single `schemars::SchemaGenerator` and writes ONE document. One generator is what makes every
//! referenced type a declared `$defs` entry and every name claimed exactly once.
//!
//! The document assembly mirrors the generated `export_schemas()`; the reference-closure check
//! comes from the shared `cml_core::json_schema_gen` module (the per-workspace helper machinery
//! the request-14 cycle moved out of every generated json-gen crate).

use cml_core::json_schema_gen::check_schema_ref_closure;

fn main() {
    let schema_path = std::path::Path::new("schemas");
    if !schema_path.exists() {
        std::fs::create_dir(schema_path).unwrap();
    }
    let mut generator = schemars::SchemaGenerator::default();
    // Same order as the old per-crate exporter calls. chain last: on a cross-crate name
    // collision, first-registered keeps the bare name, so the smaller cip crates' names are
    // stable and a collision surfaces in chain's (much larger, injectivity-guarded) row set.
    cml_cip25_json_schema_gen::add_schemas(&mut generator);
    cml_cip36_json_schema_gen::add_schemas(&mut generator);
    cml_chain_json_schema_gen::add_schemas(&mut generator);
    let meta_schema = generator.settings().meta_schema.clone();
    let definitions_path = generator.settings().definitions_path.to_string();
    let mut document = serde_json::Map::new();
    if let Some(meta_schema) = meta_schema {
        document.insert("$schema".to_owned(), meta_schema.into_owned().into());
    }
    document.insert("title".to_owned(), "cardano_multiplatform_lib".into());
    document.insert("$defs".to_owned(), generator.take_definitions(true).into());
    let document = serde_json::Value::Object(document);
    check_schema_ref_closure(&document, &definitions_path);
    std::fs::write(
        schema_path.join("cardano_multiplatform_lib.schema.json"),
        serde_json::to_string_pretty(&document).unwrap(),
    )
    .unwrap();
}
