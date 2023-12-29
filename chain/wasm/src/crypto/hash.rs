use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use crate::{
    auxdata::AuxiliaryData,
    plutus::{CostModels, Language, PlutusData},
    transaction::TransactionBody,
    PlutusDataList, RedeemerList,
};

use cml_core_wasm::impl_wasm_list;

use cml_crypto_wasm::{AuxiliaryDataHash, DatumHash, ScriptDataHash, TransactionHash};

impl_wasm_list!(Language, Language, LanguageList, true, true);

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

#[wasm_bindgen]
pub fn hash_script_data(
    redeemers: &RedeemerList,
    cost_models: &CostModels,
    datums: Option<PlutusDataList>,
    //    encoding: Option<TransactionWitnessSetEncoding>,
) -> ScriptDataHash {
    cml_chain::crypto::hash::hash_script_data(
        redeemers.as_ref(),
        cost_models.as_ref(),
        datums.as_ref().map(AsRef::as_ref),
        None,
    )
    .into()
}

#[wasm_bindgen]
pub fn calc_script_data_hash(
    redeemers: &RedeemerList,
    datums: &PlutusDataList,
    cost_models: &CostModels,
    used_langs: &LanguageList,
    //    encoding: Option<TransactionWitnessSetEncoding>,
) -> Result<Option<ScriptDataHash>, JsError> {
    cml_chain::crypto::hash::calc_script_data_hash(
        redeemers.as_ref(),
        datums.as_ref(),
        cost_models.as_ref(),
        used_langs.as_ref(),
        None,
    )
    .map(|sdh| sdh.map(Into::into))
    .map_err(Into::into)
}
