// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::byron::{ByronPubKey, ByronSignature, EpochId};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronDelegation(cml_multi_era::byron::delegation::ByronDelegation);

#[wasm_bindgen]
impl ByronDelegation {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronDelegation, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<ByronDelegation, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn epoch(&self) -> EpochId {
        self.0.epoch
    }

    pub fn issuer(&self) -> ByronPubKey {
        self.0.issuer.clone()
    }

    pub fn delegate(&self) -> ByronPubKey {
        self.0.delegate.clone()
    }

    pub fn certificate(&self) -> ByronSignature {
        self.0.certificate.clone()
    }

    pub fn new(
        epoch: EpochId,
        issuer: ByronPubKey,
        delegate: ByronPubKey,
        certificate: ByronSignature,
    ) -> Self {
        Self(cml_multi_era::byron::delegation::ByronDelegation::new(
            epoch,
            issuer,
            delegate,
            certificate,
        ))
    }
}

impl From<cml_multi_era::byron::delegation::ByronDelegation> for ByronDelegation {
    fn from(native: cml_multi_era::byron::delegation::ByronDelegation) -> Self {
        Self(native)
    }
}

impl From<ByronDelegation> for cml_multi_era::byron::delegation::ByronDelegation {
    fn from(wasm: ByronDelegation) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::delegation::ByronDelegation> for ByronDelegation {
    fn as_ref(&self) -> &cml_multi_era::byron::delegation::ByronDelegation {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronDelegationSignature(cml_multi_era::byron::delegation::ByronDelegationSignature);

#[wasm_bindgen]
impl ByronDelegationSignature {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronDelegationSignature, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<ByronDelegationSignature, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn byron_delegation(&self) -> ByronDelegation {
        self.0.byron_delegation.clone().into()
    }

    pub fn byron_signature(&self) -> ByronSignature {
        self.0.byron_signature.clone()
    }

    pub fn new(byron_delegation: &ByronDelegation, byron_signature: ByronSignature) -> Self {
        Self(
            cml_multi_era::byron::delegation::ByronDelegationSignature::new(
                byron_delegation.clone().into(),
                byron_signature,
            ),
        )
    }
}

impl From<cml_multi_era::byron::delegation::ByronDelegationSignature> for ByronDelegationSignature {
    fn from(native: cml_multi_era::byron::delegation::ByronDelegationSignature) -> Self {
        Self(native)
    }
}

impl From<ByronDelegationSignature> for cml_multi_era::byron::delegation::ByronDelegationSignature {
    fn from(wasm: ByronDelegationSignature) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::delegation::ByronDelegationSignature>
    for ByronDelegationSignature
{
    fn as_ref(&self) -> &cml_multi_era::byron::delegation::ByronDelegationSignature {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct EpochRange(cml_multi_era::byron::delegation::EpochRange);

#[wasm_bindgen]
impl EpochRange {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<EpochRange, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<EpochRange, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn epoch_id(&self) -> EpochId {
        self.0.epoch_id
    }

    pub fn epoch_id2(&self) -> EpochId {
        self.0.epoch_id2
    }

    pub fn new(epoch_id: EpochId, epoch_id2: EpochId) -> Self {
        Self(cml_multi_era::byron::delegation::EpochRange::new(
            epoch_id, epoch_id2,
        ))
    }
}

impl From<cml_multi_era::byron::delegation::EpochRange> for EpochRange {
    fn from(native: cml_multi_era::byron::delegation::EpochRange) -> Self {
        Self(native)
    }
}

impl From<EpochRange> for cml_multi_era::byron::delegation::EpochRange {
    fn from(wasm: EpochRange) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::delegation::EpochRange> for EpochRange {
    fn as_ref(&self) -> &cml_multi_era::byron::delegation::EpochRange {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct LightWeightDelegationSignature(
    cml_multi_era::byron::delegation::LightWeightDelegationSignature,
);

#[wasm_bindgen]
impl LightWeightDelegationSignature {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<LightWeightDelegationSignature, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<LightWeightDelegationSignature, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn light_weight_dlg(&self) -> LightWeightDlg {
        self.0.light_weight_dlg.clone().into()
    }

    pub fn byron_signature(&self) -> ByronSignature {
        self.0.byron_signature.clone()
    }

    pub fn new(light_weight_dlg: &LightWeightDlg, byron_signature: ByronSignature) -> Self {
        Self(
            cml_multi_era::byron::delegation::LightWeightDelegationSignature::new(
                light_weight_dlg.clone().into(),
                byron_signature,
            ),
        )
    }
}

impl From<cml_multi_era::byron::delegation::LightWeightDelegationSignature>
    for LightWeightDelegationSignature
{
    fn from(native: cml_multi_era::byron::delegation::LightWeightDelegationSignature) -> Self {
        Self(native)
    }
}

impl From<LightWeightDelegationSignature>
    for cml_multi_era::byron::delegation::LightWeightDelegationSignature
{
    fn from(wasm: LightWeightDelegationSignature) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::delegation::LightWeightDelegationSignature>
    for LightWeightDelegationSignature
{
    fn as_ref(&self) -> &cml_multi_era::byron::delegation::LightWeightDelegationSignature {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct LightWeightDlg(cml_multi_era::byron::delegation::LightWeightDlg);

#[wasm_bindgen]
impl LightWeightDlg {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::ToBytes::to_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<LightWeightDlg, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<LightWeightDlg, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn epoch_range(&self) -> EpochRange {
        self.0.epoch_range.clone().into()
    }

    pub fn issuer(&self) -> ByronPubKey {
        self.0.issuer.clone()
    }

    pub fn delegate(&self) -> ByronPubKey {
        self.0.delegate.clone()
    }

    pub fn certificate(&self) -> ByronSignature {
        self.0.certificate.clone()
    }

    pub fn new(
        epoch_range: &EpochRange,
        issuer: ByronPubKey,
        delegate: ByronPubKey,
        certificate: ByronSignature,
    ) -> Self {
        Self(cml_multi_era::byron::delegation::LightWeightDlg::new(
            epoch_range.clone().into(),
            issuer,
            delegate,
            certificate,
        ))
    }
}

impl From<cml_multi_era::byron::delegation::LightWeightDlg> for LightWeightDlg {
    fn from(native: cml_multi_era::byron::delegation::LightWeightDlg) -> Self {
        Self(native)
    }
}

impl From<LightWeightDlg> for cml_multi_era::byron::delegation::LightWeightDlg {
    fn from(wasm: LightWeightDlg) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::delegation::LightWeightDlg> for LightWeightDlg {
    fn as_ref(&self) -> &cml_multi_era::byron::delegation::LightWeightDlg {
        &self.0
    }
}
