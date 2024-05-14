use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use crate::{
    auxdata::AuxiliaryData,
    plutus::{CostModels, PlutusData, Redeemers},
    transaction::{TransactionBody, TransactionWitnessSet},
    utils::LanguageList,
    PlutusDataList,
};

use cml_crypto_wasm::{AuxiliaryDataHash, DatumHash, ScriptDataHash, TransactionHash};

use cml_chain::NonemptySet;

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
    //    encoding: Option<TransactionWitnessSetEncoding>,
) -> ScriptDataHash {
    let datums = datums.map(|datums| NonemptySet::from(Into::<Vec<_>>::into(datums)));
    cml_chain::crypto::hash::hash_script_data(
        redeemers.as_ref(),
        cost_models.as_ref(),
        datums.as_ref(),
        None,
    )
    .into()
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
    //    encoding: Option<TransactionWitnessSetEncoding>,
) -> Result<Option<ScriptDataHash>, JsError> {
    cml_chain::crypto::hash::calc_script_data_hash(
        redeemers.as_ref(),
        &NonemptySet::from(Into::<Vec<_>>::into(datums.clone())),
        cost_models.as_ref(),
        used_langs.as_ref(),
        None,
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
