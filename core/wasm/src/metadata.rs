#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use cml_core::metadata as core;

use super::*;

pub use cml_core::metadata::TransactionMetadatumLabel;

impl_wasm_conversions!(core::MetadatumMap, MetadatumMap);

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct MetadatumList(Vec<core::TransactionMetadatum>);

#[wasm_bindgen]
impl MetadatumList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionMetadatum {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionMetadatum) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::TransactionMetadatum>> for MetadatumList {
    fn from(native: Vec<core::TransactionMetadatum>) -> Self {
        Self(native)
    }
}

impl From<MetadatumList> for Vec<core::TransactionMetadatum> {
    fn from(wrapper: MetadatumList) -> Self {
        wrapper.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionMetadatumLabels(Vec<TransactionMetadatumLabel>);

#[wasm_bindgen]
impl TransactionMetadatumLabels {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionMetadatumLabel {
        self.0[index]
    }

    pub fn add(&mut self, elem: TransactionMetadatumLabel) {
        self.0.push(elem);
    }
}

impl From<Vec<TransactionMetadatumLabel>> for TransactionMetadatumLabels {
    fn from(native: Vec<TransactionMetadatumLabel>) -> Self {
        Self(native)
    }
}

impl From<TransactionMetadatumLabels> for Vec<TransactionMetadatumLabel> {
    fn from(wrapper: TransactionMetadatumLabels) -> Self {
        wrapper.0
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct MetadatumMap(core::MetadatumMap);

#[wasm_bindgen]
impl MetadatumMap {
    pub fn new() -> Self {
        Self(core::MetadatumMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Replaces all metadatums of a given key, if any exist.
    pub fn set(&mut self, key: &TransactionMetadatum, value: &TransactionMetadatum) {
        self.0.set(key.clone().into(), value.clone().into())
    }

    /// Gets the Metadatum corresponding to a given key, if it exists.
    /// Note: In the case of duplicate keys this only returns the first metadatum.
    /// This is an extremely rare occurence (2 total on mainnet) on-chain but can happen.
    pub fn get(&self, key: &TransactionMetadatum) -> Option<TransactionMetadatum> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    /// In the extremely unlikely situation there are duplicate keys, this gets all of a single key
    pub fn get_all(&self, key: &TransactionMetadatum) -> Option<TransactionMetadatumList> {
        self.0
            .get_all(key.as_ref())
            .map(|datums| datums.into_iter().cloned().collect::<Vec<_>>().into())
    }

    pub fn keys(&self) -> MetadatumList {
        MetadatumList(
            self.0
                .entries
                .iter()
                .map(|(k, _v)| k.clone())
                .collect::<Vec<_>>(),
        )
    }
}

impl_wasm_list!(
    core::TransactionMetadatum,
    TransactionMetadatum,
    TransactionMetadatumList
);

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Metadata(core::Metadata);

impl_wasm_conversions!(core::Metadata, Metadata);

#[wasm_bindgen]
impl Metadata {
    pub fn new() -> Self {
        Self(core::Metadata::new())
    }

    /// How many metadatum labels there are.
    pub fn len(&self) -> usize {
        self.0.entries.len()
    }

    /// Replaces all metadatums of a given label, if any exist.
    pub fn set(&mut self, key: TransactionMetadatumLabel, value: &TransactionMetadatum) {
        self.0.set(key, value.clone().into())
    }

    /// Gets the Metadatum corresponding to a given label, if it exists.
    /// Note: In the case of duplicate labels this only returns the first metadatum.
    /// This is an extremely rare occurence on-chain but can happen.
    pub fn get(&self, label: TransactionMetadatumLabel) -> Option<TransactionMetadatum> {
        self.0.get(label).map(|x| x.clone().into())
    }

    /// In the extremely unlikely situation there are duplicate labels, this gets all of a single label
    pub fn get_all(&self, label: TransactionMetadatumLabel) -> Option<TransactionMetadatumList> {
        self.0
            .get_all(label)
            .map(|mds| mds.into_iter().cloned().collect::<Vec<_>>().into())
    }

    pub fn labels(&self) -> TransactionMetadatumLabels {
        TransactionMetadatumLabels(self.0.entries.iter().map(|(k, _v)| *k).collect::<Vec<_>>())
    }
}

impl AsMut<core::Metadata> for Metadata {
    fn as_mut(&mut self) -> &mut core::Metadata {
        &mut self.0
    }
}

#[wasm_bindgen]
pub enum TransactionMetadatumKind {
    Map,
    List,
    Int,
    Bytes,
    Text,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionMetadatum(core::TransactionMetadatum);

#[wasm_bindgen]
impl TransactionMetadatum {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<TransactionMetadatum, JsValue> {
        Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {e}")))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {e}")))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {e}")))
    }

    pub fn from_json(json: &str) -> Result<TransactionMetadatum, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {e}")))
    }

    pub fn new_map(map: &MetadatumMap) -> Self {
        Self(core::TransactionMetadatum::new_map(map.clone().into()))
    }

    pub fn new_list(elements: &MetadatumList) -> Self {
        Self(core::TransactionMetadatum::new_list(
            elements.clone().into(),
        ))
    }

    pub fn new_int(int: &Int) -> Self {
        Self(core::TransactionMetadatum::new_int(int.clone().into()))
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self(core::TransactionMetadatum::new_bytes(bytes))
    }

    pub fn new_text(text: String) -> Self {
        Self(core::TransactionMetadatum::new_text(text))
    }

    pub fn kind(&self) -> TransactionMetadatumKind {
        match &self.0 {
            core::TransactionMetadatum::Map { .. } => TransactionMetadatumKind::Map,
            core::TransactionMetadatum::List { .. } => TransactionMetadatumKind::List,
            core::TransactionMetadatum::Int(_) => TransactionMetadatumKind::Int,
            core::TransactionMetadatum::Bytes { .. } => TransactionMetadatumKind::Bytes,
            core::TransactionMetadatum::Text { .. } => TransactionMetadatumKind::Text,
        }
    }

    pub fn as_map(&self) -> Option<MetadatumMap> {
        match &self.0 {
            core::TransactionMetadatum::Map(map) => Some(map.clone().into()),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<MetadatumList> {
        match &self.0 {
            core::TransactionMetadatum::List { elements, .. } => Some(elements.clone().into()),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<Int> {
        match &self.0 {
            core::TransactionMetadatum::Int(int) => Some(int.clone().into()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        match &self.0 {
            core::TransactionMetadatum::Bytes { bytes, .. } => Some(bytes.clone()),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<String> {
        match &self.0 {
            core::TransactionMetadatum::Text { text, .. } => Some(text.clone()),
            _ => None,
        }
    }
}

impl From<core::TransactionMetadatum> for TransactionMetadatum {
    fn from(native: core::TransactionMetadatum) -> Self {
        Self(native)
    }
}

impl From<TransactionMetadatum> for core::TransactionMetadatum {
    fn from(wasm: TransactionMetadatum) -> Self {
        wasm.0
    }
}

impl AsRef<core::TransactionMetadatum> for TransactionMetadatum {
    fn as_ref(&self) -> &core::TransactionMetadatum {
        &self.0
    }
}
