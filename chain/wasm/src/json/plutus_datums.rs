pub use cml_chain::json::plutus_datums::CardanoNodePlutusDatumSchema;

use crate::plutus::PlutusData;

use wasm_bindgen::prelude::{wasm_bindgen, JsError};

#[wasm_bindgen]
pub fn encode_json_str_to_plutus_datum(
    json: &str,
    schema: CardanoNodePlutusDatumSchema,
) -> Result<PlutusData, JsError> {
    cml_chain::json::plutus_datums::encode_json_str_to_plutus_datum(json, schema)
        .map(Into::into)
        .map_err(Into::into)
}

#[wasm_bindgen]
pub fn decode_plutus_datum_to_json_str(
    datum: &PlutusData,
    schema: CardanoNodePlutusDatumSchema,
) -> Result<String, JsError> {
    cml_chain::json::plutus_datums::decode_plutus_datum_to_json_str(datum.as_ref(), schema)
        .map_err(Into::into)
}
