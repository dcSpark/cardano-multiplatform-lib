// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::{
    MapTransactionIndexToAuxiliaryData, TransactionBodyList, TransactionIndex,
    TransactionWitnessSetList,
};
use crate::crypto::{KESSignature, VRFCert, Vkey};
use cml_crypto_wasm::{BlockBodyHash, BlockHeaderHash, Ed25519Signature, KESVkey, VRFVkey};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Block(cml_chain::block::Block);

#[wasm_bindgen]
impl Block {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Block, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<Block, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn header(&self) -> Header {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> TransactionBodyList {
        self.0.transaction_bodies.clone().into()
    }

    pub fn transaction_witness_sets(&self) -> TransactionWitnessSetList {
        self.0.transaction_witness_sets.clone().into()
    }

    pub fn auxiliary_data_set(&self) -> MapTransactionIndexToAuxiliaryData {
        self.0.auxiliary_data_set.clone().into()
    }

    pub fn invalid_transactions(&self) -> Vec<TransactionIndex> {
        self.0.invalid_transactions.clone()
    }

    pub fn new(
        header: &Header,
        transaction_bodies: &TransactionBodyList,
        transaction_witness_sets: &TransactionWitnessSetList,
        auxiliary_data_set: &MapTransactionIndexToAuxiliaryData,
        invalid_transactions: Vec<TransactionIndex>,
    ) -> Self {
        Self(cml_chain::block::Block::new(
            header.clone().into(),
            transaction_bodies.clone().into(),
            transaction_witness_sets.clone().into(),
            auxiliary_data_set.clone().into(),
            invalid_transactions,
        ))
    }
}

impl From<cml_chain::block::Block> for Block {
    fn from(native: cml_chain::block::Block) -> Self {
        Self(native)
    }
}

impl From<Block> for cml_chain::block::Block {
    fn from(wasm: Block) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::block::Block> for Block {
    fn as_ref(&self) -> &cml_chain::block::Block {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Header(cml_chain::block::Header);

#[wasm_bindgen]
impl Header {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Header, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<Header, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn header_body(&self) -> HeaderBody {
        self.0.header_body.clone().into()
    }

    pub fn body_signature(&self) -> KESSignature {
        self.0.body_signature.clone().into()
    }

    pub fn new(header_body: &HeaderBody, body_signature: &KESSignature) -> Self {
        Self(cml_chain::block::Header::new(
            header_body.clone().into(),
            body_signature.clone().into(),
        ))
    }
}

impl From<cml_chain::block::Header> for Header {
    fn from(native: cml_chain::block::Header) -> Self {
        Self(native)
    }
}

impl From<Header> for cml_chain::block::Header {
    fn from(wasm: Header) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::block::Header> for Header {
    fn as_ref(&self) -> &cml_chain::block::Header {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct HeaderBody(cml_chain::block::HeaderBody);

#[wasm_bindgen]
impl HeaderBody {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<HeaderBody, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<HeaderBody, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn block_number(&self) -> u64 {
        self.0.block_number
    }

    pub fn slot(&self) -> u64 {
        self.0.slot
    }

    pub fn prev_hash(&self) -> Option<BlockHeaderHash> {
        self.0.prev_hash.clone().map(std::convert::Into::into)
    }

    pub fn issuer_vkey(&self) -> Vkey {
        self.0.issuer_vkey.clone().into()
    }

    pub fn vrf_vkey(&self) -> VRFVkey {
        self.0.vrf_vkey.clone().into()
    }

    pub fn vrf_result(&self) -> VRFCert {
        self.0.vrf_result.clone().into()
    }

    pub fn block_body_size(&self) -> u64 {
        self.0.block_body_size
    }

    pub fn block_body_hash(&self) -> BlockBodyHash {
        self.0.block_body_hash.clone().into()
    }

    pub fn operational_cert(&self) -> OperationalCert {
        self.0.operational_cert.clone().into()
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.0.protocol_version.clone().into()
    }

    pub fn new(
        block_number: u64,
        slot: u64,
        prev_hash: Option<BlockHeaderHash>,
        issuer_vkey: &Vkey,
        vrf_vkey: &VRFVkey,
        vrf_result: &VRFCert,
        block_body_size: u64,
        block_body_hash: &BlockBodyHash,
        operational_cert: &OperationalCert,
        protocol_version: &ProtocolVersion,
    ) -> Self {
        Self(cml_chain::block::HeaderBody::new(
            block_number,
            slot,
            prev_hash.map(Into::into),
            issuer_vkey.clone().into(),
            vrf_vkey.clone().into(),
            vrf_result.clone().into(),
            block_body_size,
            block_body_hash.clone().into(),
            operational_cert.clone().into(),
            protocol_version.clone().into(),
        ))
    }
}

impl From<cml_chain::block::HeaderBody> for HeaderBody {
    fn from(native: cml_chain::block::HeaderBody) -> Self {
        Self(native)
    }
}

impl From<HeaderBody> for cml_chain::block::HeaderBody {
    fn from(wasm: HeaderBody) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::block::HeaderBody> for HeaderBody {
    fn as_ref(&self) -> &cml_chain::block::HeaderBody {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct OperationalCert(cml_chain::block::OperationalCert);

#[wasm_bindgen]
impl OperationalCert {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<OperationalCert, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<OperationalCert, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn hot_vkey(&self) -> KESVkey {
        self.0.hot_vkey.clone().into()
    }

    pub fn sequence_number(&self) -> u64 {
        self.0.sequence_number
    }

    pub fn kes_period(&self) -> u64 {
        self.0.kes_period
    }

    pub fn sigma(&self) -> Ed25519Signature {
        self.0.sigma.clone().into()
    }

    pub fn new(
        hot_vkey: &KESVkey,
        sequence_number: u64,
        kes_period: u64,
        sigma: &Ed25519Signature,
    ) -> Self {
        Self(cml_chain::block::OperationalCert::new(
            hot_vkey.clone().into(),
            sequence_number,
            kes_period,
            sigma.clone().into(),
        ))
    }
}

impl From<cml_chain::block::OperationalCert> for OperationalCert {
    fn from(native: cml_chain::block::OperationalCert) -> Self {
        Self(native)
    }
}

impl From<OperationalCert> for cml_chain::block::OperationalCert {
    fn from(wasm: OperationalCert) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::block::OperationalCert> for OperationalCert {
    fn as_ref(&self) -> &cml_chain::block::OperationalCert {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ProtocolVersion(cml_chain::block::ProtocolVersion);

#[wasm_bindgen]
impl ProtocolVersion {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ProtocolVersion, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<ProtocolVersion, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn major(&self) -> u64 {
        self.0.major
    }

    pub fn minor(&self) -> u64 {
        self.0.minor
    }

    pub fn new(major: u64, minor: u64) -> Self {
        Self(cml_chain::block::ProtocolVersion::new(major, minor))
    }
}

impl From<cml_chain::block::ProtocolVersion> for ProtocolVersion {
    fn from(native: cml_chain::block::ProtocolVersion) -> Self {
        Self(native)
    }
}

impl From<ProtocolVersion> for cml_chain::block::ProtocolVersion {
    fn from(wasm: ProtocolVersion) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::block::ProtocolVersion> for ProtocolVersion {
    fn as_ref(&self) -> &cml_chain::block::ProtocolVersion {
        &self.0
    }
}
