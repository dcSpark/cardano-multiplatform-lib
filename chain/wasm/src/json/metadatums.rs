use crate::auxdata::TransactionMetadatum;
pub use cml_chain::json::metadatums::MetadataJsonSchema;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

/// Converts JSON to Metadata according to MetadataJsonSchema
#[wasm_bindgen]
pub fn encode_json_str_to_metadatum(
    json: &str,
    schema: MetadataJsonSchema,
) -> Result<TransactionMetadatum, JsError> {
    cml_chain::json::metadatums::encode_json_str_to_metadatum(json, schema)
        .map(Into::into)
        .map_err(Into::into)
}

/// Converts Metadata to JSON according to MetadataJsonSchema
#[wasm_bindgen]
pub fn decode_metadatum_to_json_str(
    metadatum: &TransactionMetadatum,
    schema: MetadataJsonSchema,
) -> Result<String, JsError> {
    cml_chain::json::metadatums::decode_metadatum_to_json_str(metadatum.as_ref(), schema)
        .map_err(Into::into)
}
