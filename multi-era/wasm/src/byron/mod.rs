// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod block;
pub mod delegation;
pub mod mpc;
pub mod transaction;
pub mod update;

use std::collections::BTreeMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

pub type ByronBlockId = Blake2b256;

pub type ByronPubKey = Vec<u8>;

pub type ByronSignature = Vec<u8>;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronSlotId(cml_multi_era::byron::ByronSlotId);

#[wasm_bindgen]
impl ByronSlotId {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronSlotId, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ByronSlotId, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn epoch(&self) -> EpochId {
        self.0.epoch
    }

    pub fn slot(&self) -> u64 {
        self.0.slot
    }

    pub fn new(epoch: EpochId, slot: u64) -> Self {
        Self(cml_multi_era::byron::ByronSlotId::new(epoch, slot))
    }
}

impl From<cml_multi_era::byron::ByronSlotId> for ByronSlotId {
    fn from(native: cml_multi_era::byron::ByronSlotId) -> Self {
        Self(native)
    }
}

impl From<ByronSlotId> for cml_multi_era::byron::ByronSlotId {
    fn from(wasm: ByronSlotId) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::ByronSlotId> for ByronSlotId {
    fn as_ref(&self) -> &cml_multi_era::byron::ByronSlotId {
        &self.0
    }
}

pub type ByronTxId = Blake2b256;

pub type ByronUpdateId = Blake2b256;

pub type EpochId = u64;
