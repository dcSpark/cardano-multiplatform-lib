#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use wasm_bindgen::prelude::*;

use crate::{TransactionBody, metadata::AuxiliaryData, crypto::{AuxiliaryDataHash, blake2b256, DataHash, TransactionHash, ScriptDataHash, blake2b224, ScriptHash, self}, plutus::{PlutusData, Redeemers, Costmdls, PlutusList}};


#[wasm_bindgen]
pub fn hash_auxiliary_data(auxiliary_data: &AuxiliaryData) -> AuxiliaryDataHash {
  AuxiliaryDataHash::from(blake2b256(&auxiliary_data.to_bytes()))
}
#[wasm_bindgen]
pub fn hash_transaction(tx_body: &TransactionBody) -> TransactionHash {
    TransactionHash::from(crypto::blake2b256(tx_body.to_bytes().as_ref()))
}
#[wasm_bindgen]
pub fn hash_plutus_data(plutus_data: &PlutusData) -> DataHash {
    DataHash::from(blake2b256(&plutus_data.to_bytes()))
}
#[wasm_bindgen]
pub fn hash_script_data(redeemers: &Redeemers, cost_models: &Costmdls, datums: Option<PlutusList>) -> ScriptDataHash {
    let mut buf = Vec::new();
    if redeemers.len() == 0 && datums.is_some() {
        /*
        ; Finally, note that in the case that a transaction includes datums but does not
        ; include any redeemers, the script data format becomes (in hex):
        ; [ 80 | datums | A0 ]
        ; corresponding to a CBOR empty list and an empty map (our apologies).
        */
        buf.push(0x80);
        if let Some(d) = &datums {
            buf.extend(d.to_bytes());
        }
        buf.push(0xA0);
    } else {
        /*
        ; script data format:
        ; [ redeemers | datums | language views ]
        ; The redeemers are exactly the data present in the transaction witness set.
        ; Similarly for the datums, if present. If no datums are provided, the middle
        ; field is an empty string.
        */
        buf.extend(redeemers.to_bytes());
        if let Some(d) = &datums {
            buf.extend(d.to_bytes());
        }
        buf.extend(cost_models.language_views_encoding());
    }
    ScriptDataHash::from(blake2b256(&buf))
}

/// Each new language uses a different namespace for hashing its script
/// This is because you could have a language where the same bytes have different semantics
/// So this avoids scripts in different languages mapping to the same hash
/// Note that the enum value here is different than the enum value for deciding the cost model of a script
/// https://github.com/input-output-hk/cardano-ledger/blob/9c3b4737b13b30f71529e76c5330f403165e28a6/eras/alonzo/impl/src/Cardano/Ledger/Alonzo.hs#L127
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ScriptHashNamespace {
    NativeScript,
    PlutusV1,
    PlutusV2
}

pub (crate) fn hash_script(namespace: ScriptHashNamespace, script: Vec<u8>) -> ScriptHash {
    let mut bytes = Vec::with_capacity(script.len() + 1);
    bytes.extend_from_slice(&[namespace as u8]);
    bytes.extend_from_slice(&script);
    ScriptHash::from(blake2b224(bytes.as_ref()))
}
