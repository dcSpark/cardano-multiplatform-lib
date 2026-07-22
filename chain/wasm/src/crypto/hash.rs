use wasm_bindgen::prelude::{JsError, wasm_bindgen};

use crate::{
    PlutusDataList,
    auxdata::AuxiliaryData,
    plutus::{CostModels, PlutusData, Redeemers},
    transaction::{TransactionBody, TransactionWitnessSet},
    utils::LanguageList,
};

use cml_crypto_wasm::{AuxiliaryDataHash, DatumHash, ScriptDataHash, TransactionHash};

use cml_chain::NonemptySetPlutusData;

#[wasm_bindgen]
pub fn hash_auxiliary_data(auxiliary_data: &AuxiliaryData) -> AuxiliaryDataHash {
    cml_chain::crypto::hash::hash_auxiliary_data(auxiliary_data.as_ref()).into()
}

#[wasm_bindgen]
pub fn hash_transaction(tx_body: &TransactionBody) -> TransactionHash {
    cml_chain::crypto::hash::hash_transaction(tx_body.as_ref()).into()
}

#[wasm_bindgen]
pub fn hash_plutus_data(plutus_data: &PlutusData) -> DatumHash {
    cml_chain::crypto::hash::hash_plutus_data(plutus_data.as_ref()).into()
}

/// Calculates the hash for script data (no plutus scripts) if it is necessary.
/// Returns None if it was not necessary (no datums/redeemers) to include.
///
/// Most users will not directly need this as when using the builders
/// it will be invoked for you.
///
/// Note: This WASM binding does not work with non-standard witness set
/// encodings. If you created the witness set manually this is not an issue
/// but for constructing it from deserializing a transaction/witness then
/// please use calc_script_data_hash_from_witness()
#[wasm_bindgen]
pub fn hash_script_data(
    redeemers: &Redeemers,
    cost_models: &CostModels,
    datums: Option<PlutusDataList>,
) -> Result<ScriptDataHash, JsError> {
    // try_opt_from: an empty list means no datums (a witness set cannot carry an empty datum
    // list); a duplicate datum is a caller error (the field is a set as of Conway).
    let datums = datums
        .map(|datums| NonemptySetPlutusData::try_opt_from(datums.into()))
        .transpose()
        .map_err(|e| JsError::new(&e.to_string()))?
        .flatten();
    Ok(cml_chain::crypto::hash::hash_script_data(
        Some(redeemers.as_ref()),
        cost_models.as_ref(),
        datums.as_ref(),
    )
    .into())
}

/// Calculates the hash for script data (with plutus scripts) if it is necessary.
/// Returns None if it was not necessary (no datums/redeemers) to include.
///
/// Most users will not directly need this as when using the builders
/// it will be invoked for you.
///
/// Note: This WASM binding does not work with non-standard witness set
/// encodings. If you created the witness set manually this is not an issue
/// but for constructing it from deserializing a transaction/witness then
/// please use calc_script_data_hash_from_witness()
#[wasm_bindgen]
pub fn calc_script_data_hash(
    redeemers: &Redeemers,
    datums: &PlutusDataList,
    cost_models: &CostModels,
    used_langs: &LanguageList,
) -> Result<Option<ScriptDataHash>, JsError> {
    // try_opt_from: an empty list means no datums; a duplicate datum is a caller error (the
    // field is a set as of Conway).
    let datums = NonemptySetPlutusData::try_opt_from(datums.clone().into())
        .map_err(|e| JsError::new(&e.to_string()))?;
    cml_chain::crypto::hash::calc_script_data_hash(
        Some(redeemers.as_ref()),
        datums.as_ref(),
        cost_models.as_ref(),
        used_langs.as_ref(),
    )
    .map(|sdh| sdh.map(Into::into))
    .map_err(Into::into)
}

/// Calculates the hash for script data from a witness if it is necessary.
/// Returns None if it was not necessary (no datums/redeemers) to include.
///
/// Most users will not directly need this as when using the builders
/// it will be invoked for you.
#[wasm_bindgen]
pub fn calc_script_data_hash_from_witness(
    witnesses: &TransactionWitnessSet,
    cost_models: &CostModels,
) -> Result<Option<ScriptDataHash>, JsError> {
    cml_chain::crypto::hash::calc_script_data_hash_from_witness(
        witnesses.as_ref(),
        cost_models.as_ref(),
    )
    .map(|sdh| sdh.map(Into::into))
    .map_err(Into::into)
}
