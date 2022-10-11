#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Block(pub(crate) core::Block);

#[wasm_bindgen]

impl Block {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Block, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Block, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn header(&self) -> Header {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> TransactionBodys {
        self.0.transaction_bodies.clone().into()
    }

    pub fn transaction_witness_sets(&self) -> TransactionWitnessSets {
        self.0.transaction_witness_sets.clone().into()
    }

    pub fn auxiliary_data_set(&self) -> MapTransactionIndexToAuxiliaryData {
        self.0.auxiliary_data_set.clone().into()
    }

    pub fn invalid_transactions(&self) -> Vec<TransactionIndex> {
        self.0.invalid_transactions.clone().into()
    }

    pub fn new(header: &Header, transaction_bodies: &TransactionBodys, transaction_witness_sets: &TransactionWitnessSets, auxiliary_data_set: &MapTransactionIndexToAuxiliaryData, invalid_transactions: Vec<TransactionIndex>) -> Self {
        Self(core::Block::new(header.clone().into(), transaction_bodies.clone().into(), transaction_witness_sets.clone().into(), auxiliary_data_set.clone().into(), invalid_transactions))
    }
}

impl From<core::Block> for Block {
    fn from(native: core::Block) -> Self {
        Self(native)
    }
}

impl From<Block> for core::Block {
    fn from(wasm: Block) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Header(pub(crate) core::Header);

#[wasm_bindgen]

impl Header {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Header, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Header, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn header_body(&self) -> HeaderBody {
        self.0.header_body.clone().into()
    }

    pub fn body_signature(&self) -> KesSignature {
        self.0.body_signature.clone().into()
    }

    pub fn new(header_body: &HeaderBody, body_signature: &KesSignature) -> Self {
        Self(core::Header::new(header_body.clone().into(), body_signature.clone().into()))
    }
}

impl From<core::Header> for Header {
    fn from(native: core::Header) -> Self {
        Self(native)
    }
}

impl From<Header> for core::Header {
    fn from(wasm: Header) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct HeaderBody(pub(crate) core::HeaderBody);

#[wasm_bindgen]

impl HeaderBody {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<HeaderBody, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<HeaderBody, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn block_number(&self) -> u64 {
        self.0.block_number
    }

    pub fn slot(&self) -> u64 {
        self.0.slot
    }

    pub fn prev_hash(&self) -> Option<Hash32> {
        self.0.prev_hash.clone().map(std::convert::Into::into)
    }

    pub fn issuer_vkey(&self) -> Vkey {
        self.0.issuer_vkey.clone().into()
    }

    pub fn vrf_vkey(&self) -> VrfVkey {
        self.0.vrf_vkey.clone().into()
    }

    pub fn vrf_result(&self) -> VrfCert {
        self.0.vrf_result.clone().into()
    }

    pub fn block_body_size(&self) -> u64 {
        self.0.block_body_size
    }

    pub fn block_body_hash(&self) -> Hash32 {
        self.0.block_body_hash.clone().into()
    }

    pub fn operational_cert(&self) -> OperationalCert {
        self.0.operational_cert.clone().into()
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.0.protocol_version.clone().into()
    }

    pub fn new(block_number: u64, slot: u64, prev_hash: Option<Hash32>, issuer_vkey: &Vkey, vrf_vkey: &VrfVkey, vrf_result: &VrfCert, block_body_size: u64, block_body_hash: &Hash32, operational_cert: &OperationalCert, protocol_version: &ProtocolVersion) -> Self {
        Self(core::HeaderBody::new(block_number, slot, prev_hash.map(Into::into), issuer_vkey.clone().into(), vrf_vkey.clone().into(), vrf_result.clone().into(), block_body_size, block_body_hash.clone().into(), operational_cert.clone().into(), protocol_version.clone().into()))
    }
}

impl From<core::HeaderBody> for HeaderBody {
    fn from(native: core::HeaderBody) -> Self {
        Self(native)
    }
}

impl From<HeaderBody> for core::HeaderBody {
    fn from(wasm: HeaderBody) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct OperationalCert(pub(crate) core::OperationalCert);

#[wasm_bindgen]

impl OperationalCert {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<OperationalCert, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<OperationalCert, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn hot_vkey(&self) -> KesVkey {
        self.0.hot_vkey.clone().into()
    }

    pub fn sequence_number(&self) -> u64 {
        self.0.sequence_number
    }

    pub fn kes_period(&self) -> u64 {
        self.0.kes_period
    }

    pub fn sigma(&self) -> Signature {
        self.0.sigma.clone().into()
    }

    pub fn new(hot_vkey: &KesVkey, sequence_number: u64, kes_period: u64, sigma: &Signature) -> Self {
        Self(core::OperationalCert::new(hot_vkey.clone().into(), sequence_number, kes_period, sigma.clone().into()))
    }
}

impl From<core::OperationalCert> for OperationalCert {
    fn from(native: core::OperationalCert) -> Self {
        Self(native)
    }
}

impl From<OperationalCert> for core::OperationalCert {
    fn from(wasm: OperationalCert) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ProtocolVersion(pub(crate) core::ProtocolVersion);

#[wasm_bindgen]

impl ProtocolVersion {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ProtocolVersion, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ProtocolVersion, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_0(&self) -> u64 {
        self.0.index_0
    }

    pub fn index_1(&self) -> u64 {
        self.0.index_1
    }

    pub fn new(index_0: u64, index_1: u64) -> Self {
        Self(core::ProtocolVersion::new(index_0, index_1))
    }
}

impl From<core::ProtocolVersion> for ProtocolVersion {
    fn from(native: core::ProtocolVersion) -> Self {
        Self(native)
    }
}

impl From<ProtocolVersion> for core::ProtocolVersion {
    fn from(wasm: ProtocolVersion) -> Self {
        wasm.0
    }
}

use super::*;