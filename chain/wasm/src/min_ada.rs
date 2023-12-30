use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use crate::{assets::Coin, transaction::TransactionOutput};

#[wasm_bindgen]
pub fn min_ada_required(
    output: &TransactionOutput,
    coins_per_utxo_byte: Coin, // protocol parameter (in lovelace)
) -> Result<Coin, JsError> {
    cml_chain::min_ada::min_ada_required(output.as_ref(), coins_per_utxo_byte).map_err(Into::into)
}
