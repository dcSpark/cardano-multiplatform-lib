// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::byron::delegation::{ByronDelegationSignature, LightWeightDelegationSignature};
use crate::byron::mpc::{Ssc, SscProof};
use crate::byron::transaction::{ByronAttributes, ByronTx, ByronTxProof};
use crate::byron::update::{ByronBlockVersion, ByronSoftwareVersion, ByronUpdate};
use crate::byron::{Blake2b256, ByronBlockId, ByronPubKey, ByronSignature, ByronSlotId, EpochId};
use crate::{ByronAttributesList, ByronDelegationList, ByronTxWitnessList, StakeholderIdList};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BlockHeaderExtraData(cml_multi_era::byron::block::BlockHeaderExtraData);

#[wasm_bindgen]
impl BlockHeaderExtraData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<BlockHeaderExtraData, JsValue> {
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

    pub fn from_json(json: &str) -> Result<BlockHeaderExtraData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn block_version(&self) -> ByronBlockVersion {
        self.0.block_version.clone().into()
    }

    pub fn software_version(&self) -> ByronSoftwareVersion {
        self.0.software_version.clone().into()
    }

    pub fn byron_attributes(&self) -> ByronAttributes {
        self.0.byron_attributes.clone().into()
    }

    pub fn extra_proof(&self) -> Blake2b256 {
        self.0.extra_proof.clone().into()
    }

    pub fn new(
        block_version: &ByronBlockVersion,
        software_version: &ByronSoftwareVersion,
        byron_attributes: &ByronAttributes,
        extra_proof: &Blake2b256,
    ) -> Self {
        Self(cml_multi_era::byron::block::BlockHeaderExtraData::new(
            block_version.clone().into(),
            software_version.clone().into(),
            byron_attributes.clone().into(),
            extra_proof.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::block::BlockHeaderExtraData> for BlockHeaderExtraData {
    fn from(native: cml_multi_era::byron::block::BlockHeaderExtraData) -> Self {
        Self(native)
    }
}

impl From<BlockHeaderExtraData> for cml_multi_era::byron::block::BlockHeaderExtraData {
    fn from(wasm: BlockHeaderExtraData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::BlockHeaderExtraData> for BlockHeaderExtraData {
    fn as_ref(&self) -> &cml_multi_era::byron::block::BlockHeaderExtraData {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlock(cml_multi_era::byron::block::ByronBlock);

#[wasm_bindgen]
impl ByronBlock {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronBlock, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronBlock, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_epoch_boundary(epoch_boundary: &ByronEbBlock) -> Self {
        Self(cml_multi_era::byron::block::ByronBlock::new_epoch_boundary(
            epoch_boundary.clone().into(),
        ))
    }

    pub fn new_main(main: &ByronMainBlock) -> Self {
        Self(cml_multi_era::byron::block::ByronBlock::new_main(
            main.clone().into(),
        ))
    }

    pub fn kind(&self) -> ByronBlockKind {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlock::EpochBoundary(_) => {
                ByronBlockKind::EpochBoundary
            }
            cml_multi_era::byron::block::ByronBlock::Main(_) => ByronBlockKind::Main,
        }
    }

    pub fn as_epoch_boundary(&self) -> Option<ByronEbBlock> {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlock::EpochBoundary(epoch_boundary) => {
                Some(epoch_boundary.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_main(&self) -> Option<ByronMainBlock> {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlock::Main(main) => Some(main.clone().into()),
            _ => None,
        }
    }
}

impl From<cml_multi_era::byron::block::ByronBlock> for ByronBlock {
    fn from(native: cml_multi_era::byron::block::ByronBlock) -> Self {
        Self(native)
    }
}

impl From<ByronBlock> for cml_multi_era::byron::block::ByronBlock {
    fn from(wasm: ByronBlock) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronBlock> for ByronBlock {
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronBlock {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockBody(cml_multi_era::byron::block::ByronBlockBody);

#[wasm_bindgen]
impl ByronBlockBody {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronBlockBody, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronBlockBody, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn tx_payload(&self) -> TxPayload {
        self.0.tx_payload.clone().into()
    }

    pub fn ssc_payload(&self) -> Ssc {
        self.0.ssc_payload.clone().into()
    }

    pub fn dlg_payload(&self) -> ByronDelegationList {
        self.0.dlg_payload.clone().into()
    }

    pub fn upd_payload(&self) -> ByronUpdate {
        self.0.upd_payload.clone().into()
    }

    pub fn new(
        tx_payload: &TxPayload,
        ssc_payload: &Ssc,
        dlg_payload: &ByronDelegationList,
        upd_payload: &ByronUpdate,
    ) -> Self {
        Self(cml_multi_era::byron::block::ByronBlockBody::new(
            tx_payload.clone().into(),
            ssc_payload.clone().into(),
            dlg_payload.clone().into(),
            upd_payload.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::block::ByronBlockBody> for ByronBlockBody {
    fn from(native: cml_multi_era::byron::block::ByronBlockBody) -> Self {
        Self(native)
    }
}

impl From<ByronBlockBody> for cml_multi_era::byron::block::ByronBlockBody {
    fn from(wasm: ByronBlockBody) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronBlockBody> for ByronBlockBody {
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronBlockBody {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockConsensusData(cml_multi_era::byron::block::ByronBlockConsensusData);

#[wasm_bindgen]
impl ByronBlockConsensusData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronBlockConsensusData, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronBlockConsensusData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn byron_slot_id(&self) -> ByronSlotId {
        self.0.byron_slot_id.clone().into()
    }

    pub fn byron_pub_key(&self) -> ByronPubKey {
        self.0.byron_pub_key.clone()
    }

    pub fn byron_difficulty(&self) -> ByronDifficulty {
        self.0.byron_difficulty.clone().into()
    }

    pub fn byron_block_signature(&self) -> ByronBlockSignature {
        self.0.byron_block_signature.clone().into()
    }

    pub fn new(
        byron_slot_id: &ByronSlotId,
        byron_pub_key: ByronPubKey,
        byron_difficulty: &ByronDifficulty,
        byron_block_signature: &ByronBlockSignature,
    ) -> Self {
        Self(cml_multi_era::byron::block::ByronBlockConsensusData::new(
            byron_slot_id.clone().into(),
            byron_pub_key,
            byron_difficulty.clone().into(),
            byron_block_signature.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::block::ByronBlockConsensusData> for ByronBlockConsensusData {
    fn from(native: cml_multi_era::byron::block::ByronBlockConsensusData) -> Self {
        Self(native)
    }
}

impl From<ByronBlockConsensusData> for cml_multi_era::byron::block::ByronBlockConsensusData {
    fn from(wasm: ByronBlockConsensusData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronBlockConsensusData> for ByronBlockConsensusData {
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronBlockConsensusData {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockHeader(cml_multi_era::byron::block::ByronBlockHeader);

#[wasm_bindgen]
impl ByronBlockHeader {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronBlockHeader, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronBlockHeader, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn protocol_magic(&self) -> u32 {
        self.0.protocol_magic
    }

    pub fn prev_block(&self) -> ByronBlockId {
        self.0.prev_block.clone().into()
    }

    pub fn body_proof(&self) -> ByronBodyProof {
        self.0.body_proof.clone().into()
    }

    pub fn consensus_data(&self) -> ByronBlockConsensusData {
        self.0.consensus_data.clone().into()
    }

    pub fn extra_data(&self) -> BlockHeaderExtraData {
        self.0.extra_data.clone().into()
    }

    pub fn new(
        protocol_magic: u32,
        prev_block: &ByronBlockId,
        body_proof: &ByronBodyProof,
        consensus_data: &ByronBlockConsensusData,
        extra_data: &BlockHeaderExtraData,
    ) -> Self {
        Self(cml_multi_era::byron::block::ByronBlockHeader::new(
            protocol_magic,
            prev_block.clone().into(),
            body_proof.clone().into(),
            consensus_data.clone().into(),
            extra_data.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::block::ByronBlockHeader> for ByronBlockHeader {
    fn from(native: cml_multi_era::byron::block::ByronBlockHeader) -> Self {
        Self(native)
    }
}

impl From<ByronBlockHeader> for cml_multi_era::byron::block::ByronBlockHeader {
    fn from(wasm: ByronBlockHeader) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronBlockHeader> for ByronBlockHeader {
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronBlockHeader {
        &self.0
    }
}

#[wasm_bindgen]
pub enum ByronBlockKind {
    EpochBoundary,
    Main,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockSignature(cml_multi_era::byron::block::ByronBlockSignature);

#[wasm_bindgen]
impl ByronBlockSignature {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronBlockSignature, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronBlockSignature, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_signature(signature: &ByronBlockSignatureNormal) -> Self {
        Self(
            cml_multi_era::byron::block::ByronBlockSignature::new_signature(
                signature.clone().into(),
            ),
        )
    }

    pub fn new_proxy_light(proxy_light: &ByronBlockSignatureProxyLight) -> Self {
        Self(
            cml_multi_era::byron::block::ByronBlockSignature::new_proxy_light(
                proxy_light.clone().into(),
            ),
        )
    }

    pub fn new_proxy_heavy(proxy_heavy: &ByronBlockSignatureProxyHeavy) -> Self {
        Self(
            cml_multi_era::byron::block::ByronBlockSignature::new_proxy_heavy(
                proxy_heavy.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> ByronBlockSignatureKind {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlockSignature::Signature(_) => {
                ByronBlockSignatureKind::Signature
            }
            cml_multi_era::byron::block::ByronBlockSignature::ProxyLight(_) => {
                ByronBlockSignatureKind::ProxyLight
            }
            cml_multi_era::byron::block::ByronBlockSignature::ProxyHeavy(_) => {
                ByronBlockSignatureKind::ProxyHeavy
            }
        }
    }

    pub fn as_signature(&self) -> Option<ByronBlockSignatureNormal> {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlockSignature::Signature(signature) => {
                Some(signature.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_proxy_light(&self) -> Option<ByronBlockSignatureProxyLight> {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlockSignature::ProxyLight(proxy_light) => {
                Some(proxy_light.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_proxy_heavy(&self) -> Option<ByronBlockSignatureProxyHeavy> {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlockSignature::ProxyHeavy(proxy_heavy) => {
                Some(proxy_heavy.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cml_multi_era::byron::block::ByronBlockSignature> for ByronBlockSignature {
    fn from(native: cml_multi_era::byron::block::ByronBlockSignature) -> Self {
        Self(native)
    }
}

impl From<ByronBlockSignature> for cml_multi_era::byron::block::ByronBlockSignature {
    fn from(wasm: ByronBlockSignature) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronBlockSignature> for ByronBlockSignature {
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronBlockSignature {
        &self.0
    }
}

#[wasm_bindgen]
pub enum ByronBlockSignatureKind {
    Signature,
    ProxyLight,
    ProxyHeavy,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockSignatureNormal(cml_multi_era::byron::block::ByronBlockSignatureNormal);

#[wasm_bindgen]
impl ByronBlockSignatureNormal {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronBlockSignatureNormal, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronBlockSignatureNormal, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn signature(&self) -> ByronSignature {
        self.0.signature.clone()
    }

    pub fn new(signature: ByronSignature) -> Self {
        Self(cml_multi_era::byron::block::ByronBlockSignatureNormal::new(
            signature,
        ))
    }
}

impl From<cml_multi_era::byron::block::ByronBlockSignatureNormal> for ByronBlockSignatureNormal {
    fn from(native: cml_multi_era::byron::block::ByronBlockSignatureNormal) -> Self {
        Self(native)
    }
}

impl From<ByronBlockSignatureNormal> for cml_multi_era::byron::block::ByronBlockSignatureNormal {
    fn from(wasm: ByronBlockSignatureNormal) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronBlockSignatureNormal> for ByronBlockSignatureNormal {
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronBlockSignatureNormal {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockSignatureProxyHeavy(
    cml_multi_era::byron::block::ByronBlockSignatureProxyHeavy,
);

#[wasm_bindgen]
impl ByronBlockSignatureProxyHeavy {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronBlockSignatureProxyHeavy, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronBlockSignatureProxyHeavy, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn signature(&self) -> ByronDelegationSignature {
        self.0.signature.clone().into()
    }

    pub fn new(signature: &ByronDelegationSignature) -> Self {
        Self(
            cml_multi_era::byron::block::ByronBlockSignatureProxyHeavy::new(
                signature.clone().into(),
            ),
        )
    }
}

impl From<cml_multi_era::byron::block::ByronBlockSignatureProxyHeavy>
    for ByronBlockSignatureProxyHeavy
{
    fn from(native: cml_multi_era::byron::block::ByronBlockSignatureProxyHeavy) -> Self {
        Self(native)
    }
}

impl From<ByronBlockSignatureProxyHeavy>
    for cml_multi_era::byron::block::ByronBlockSignatureProxyHeavy
{
    fn from(wasm: ByronBlockSignatureProxyHeavy) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronBlockSignatureProxyHeavy>
    for ByronBlockSignatureProxyHeavy
{
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronBlockSignatureProxyHeavy {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockSignatureProxyLight(
    cml_multi_era::byron::block::ByronBlockSignatureProxyLight,
);

#[wasm_bindgen]
impl ByronBlockSignatureProxyLight {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronBlockSignatureProxyLight, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronBlockSignatureProxyLight, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn signature(&self) -> LightWeightDelegationSignature {
        self.0.signature.clone().into()
    }

    pub fn new(signature: &LightWeightDelegationSignature) -> Self {
        Self(
            cml_multi_era::byron::block::ByronBlockSignatureProxyLight::new(
                signature.clone().into(),
            ),
        )
    }
}

impl From<cml_multi_era::byron::block::ByronBlockSignatureProxyLight>
    for ByronBlockSignatureProxyLight
{
    fn from(native: cml_multi_era::byron::block::ByronBlockSignatureProxyLight) -> Self {
        Self(native)
    }
}

impl From<ByronBlockSignatureProxyLight>
    for cml_multi_era::byron::block::ByronBlockSignatureProxyLight
{
    fn from(wasm: ByronBlockSignatureProxyLight) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronBlockSignatureProxyLight>
    for ByronBlockSignatureProxyLight
{
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronBlockSignatureProxyLight {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBodyProof(cml_multi_era::byron::block::ByronBodyProof);

#[wasm_bindgen]
impl ByronBodyProof {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronBodyProof, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronBodyProof, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn tx_proof(&self) -> ByronTxProof {
        self.0.tx_proof.clone().into()
    }

    pub fn ssc_proof(&self) -> SscProof {
        self.0.ssc_proof.clone().into()
    }

    pub fn dlg_proof(&self) -> Blake2b256 {
        self.0.dlg_proof.clone().into()
    }

    pub fn upd_proof(&self) -> Blake2b256 {
        self.0.upd_proof.clone().into()
    }

    pub fn new(
        tx_proof: &ByronTxProof,
        ssc_proof: &SscProof,
        dlg_proof: &Blake2b256,
        upd_proof: &Blake2b256,
    ) -> Self {
        Self(cml_multi_era::byron::block::ByronBodyProof::new(
            tx_proof.clone().into(),
            ssc_proof.clone().into(),
            dlg_proof.clone().into(),
            upd_proof.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::block::ByronBodyProof> for ByronBodyProof {
    fn from(native: cml_multi_era::byron::block::ByronBodyProof) -> Self {
        Self(native)
    }
}

impl From<ByronBodyProof> for cml_multi_era::byron::block::ByronBodyProof {
    fn from(wasm: ByronBodyProof) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronBodyProof> for ByronBodyProof {
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronBodyProof {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronDifficulty(cml_multi_era::byron::block::ByronDifficulty);

#[wasm_bindgen]
impl ByronDifficulty {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronDifficulty, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronDifficulty, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn u64(&self) -> u64 {
        self.0.u64
    }

    pub fn new(u64: u64) -> Self {
        Self(cml_multi_era::byron::block::ByronDifficulty::new(u64))
    }
}

impl From<cml_multi_era::byron::block::ByronDifficulty> for ByronDifficulty {
    fn from(native: cml_multi_era::byron::block::ByronDifficulty) -> Self {
        Self(native)
    }
}

impl From<ByronDifficulty> for cml_multi_era::byron::block::ByronDifficulty {
    fn from(wasm: ByronDifficulty) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronDifficulty> for ByronDifficulty {
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronDifficulty {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronEbBlock(cml_multi_era::byron::block::ByronEbBlock);

#[wasm_bindgen]
impl ByronEbBlock {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronEbBlock, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronEbBlock, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn header(&self) -> EbbHead {
        self.0.header.clone().into()
    }

    pub fn body(&self) -> StakeholderIdList {
        self.0.body.clone().into()
    }

    pub fn extra(&self) -> ByronAttributesList {
        self.0.extra.clone().into()
    }

    pub fn new(header: &EbbHead, body: &StakeholderIdList, extra: &ByronAttributesList) -> Self {
        Self(cml_multi_era::byron::block::ByronEbBlock::new(
            header.clone().into(),
            body.clone().into(),
            extra.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::block::ByronEbBlock> for ByronEbBlock {
    fn from(native: cml_multi_era::byron::block::ByronEbBlock) -> Self {
        Self(native)
    }
}

impl From<ByronEbBlock> for cml_multi_era::byron::block::ByronEbBlock {
    fn from(wasm: ByronEbBlock) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronEbBlock> for ByronEbBlock {
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronEbBlock {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronMainBlock(cml_multi_era::byron::block::ByronMainBlock);

#[wasm_bindgen]
impl ByronMainBlock {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ByronMainBlock, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ByronMainBlock, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn header(&self) -> ByronBlockHeader {
        self.0.header.clone().into()
    }

    pub fn body(&self) -> ByronBlockBody {
        self.0.body.clone().into()
    }

    pub fn extra(&self) -> ByronAttributesList {
        self.0.extra.clone().into()
    }

    pub fn new(
        header: &ByronBlockHeader,
        body: &ByronBlockBody,
        extra: &ByronAttributesList,
    ) -> Self {
        Self(cml_multi_era::byron::block::ByronMainBlock::new(
            header.clone().into(),
            body.clone().into(),
            extra.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::block::ByronMainBlock> for ByronMainBlock {
    fn from(native: cml_multi_era::byron::block::ByronMainBlock) -> Self {
        Self(native)
    }
}

impl From<ByronMainBlock> for cml_multi_era::byron::block::ByronMainBlock {
    fn from(wasm: ByronMainBlock) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::ByronMainBlock> for ByronMainBlock {
    fn as_ref(&self) -> &cml_multi_era::byron::block::ByronMainBlock {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct EbbConsensusData(cml_multi_era::byron::block::EbbConsensusData);

#[wasm_bindgen]
impl EbbConsensusData {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<EbbConsensusData, JsValue> {
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

    pub fn from_json(json: &str) -> Result<EbbConsensusData, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn epoch_id(&self) -> EpochId {
        self.0.epoch_id
    }

    pub fn byron_difficulty(&self) -> ByronDifficulty {
        self.0.byron_difficulty.clone().into()
    }

    pub fn new(epoch_id: EpochId, byron_difficulty: &ByronDifficulty) -> Self {
        Self(cml_multi_era::byron::block::EbbConsensusData::new(
            epoch_id,
            byron_difficulty.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::block::EbbConsensusData> for EbbConsensusData {
    fn from(native: cml_multi_era::byron::block::EbbConsensusData) -> Self {
        Self(native)
    }
}

impl From<EbbConsensusData> for cml_multi_era::byron::block::EbbConsensusData {
    fn from(wasm: EbbConsensusData) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::EbbConsensusData> for EbbConsensusData {
    fn as_ref(&self) -> &cml_multi_era::byron::block::EbbConsensusData {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct EbbHead(cml_multi_era::byron::block::EbbHead);

#[wasm_bindgen]
impl EbbHead {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<EbbHead, JsValue> {
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

    pub fn from_json(json: &str) -> Result<EbbHead, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn protocol_magic(&self) -> u32 {
        self.0.protocol_magic
    }

    pub fn prev_block(&self) -> ByronBlockId {
        self.0.prev_block.clone().into()
    }

    pub fn body_proof(&self) -> Blake2b256 {
        self.0.body_proof.clone().into()
    }

    pub fn consensus_data(&self) -> EbbConsensusData {
        self.0.consensus_data.clone().into()
    }

    pub fn extra_data(&self) -> ByronAttributesList {
        self.0.extra_data.clone().into()
    }

    pub fn new(
        protocol_magic: u32,
        prev_block: &ByronBlockId,
        body_proof: &Blake2b256,
        consensus_data: &EbbConsensusData,
        extra_data: &ByronAttributesList,
    ) -> Self {
        Self(cml_multi_era::byron::block::EbbHead::new(
            protocol_magic,
            prev_block.clone().into(),
            body_proof.clone().into(),
            consensus_data.clone().into(),
            extra_data.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::block::EbbHead> for EbbHead {
    fn from(native: cml_multi_era::byron::block::EbbHead) -> Self {
        Self(native)
    }
}

impl From<EbbHead> for cml_multi_era::byron::block::EbbHead {
    fn from(wasm: EbbHead) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::EbbHead> for EbbHead {
    fn as_ref(&self) -> &cml_multi_era::byron::block::EbbHead {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TxAux(cml_multi_era::byron::block::TxAux);

#[wasm_bindgen]
impl TxAux {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<TxAux, JsValue> {
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

    pub fn from_json(json: &str) -> Result<TxAux, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn byron_tx(&self) -> ByronTx {
        self.0.byron_tx.clone().into()
    }

    pub fn byron_tx_witnesss(&self) -> ByronTxWitnessList {
        self.0.byron_tx_witnesss.clone().into()
    }

    pub fn new(byron_tx: &ByronTx, byron_tx_witnesss: &ByronTxWitnessList) -> Self {
        Self(cml_multi_era::byron::block::TxAux::new(
            byron_tx.clone().into(),
            byron_tx_witnesss.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::block::TxAux> for TxAux {
    fn from(native: cml_multi_era::byron::block::TxAux) -> Self {
        Self(native)
    }
}

impl From<TxAux> for cml_multi_era::byron::block::TxAux {
    fn from(wasm: TxAux) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::block::TxAux> for TxAux {
    fn as_ref(&self) -> &cml_multi_era::byron::block::TxAux {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TxPayload(Vec<cml_multi_era::byron::block::TxAux>);

#[wasm_bindgen]
impl TxPayload {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TxAux {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TxAux) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::block::TxAux>> for TxPayload {
    fn from(native: Vec<cml_multi_era::byron::block::TxAux>) -> Self {
        Self(native)
    }
}

impl From<TxPayload> for Vec<cml_multi_era::byron::block::TxAux> {
    fn from(wasm: TxPayload) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::block::TxAux>> for TxPayload {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::block::TxAux> {
        &self.0
    }
}
