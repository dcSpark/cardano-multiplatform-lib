// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_chain_wasm::{
    GenesisHashList,
    ProtocolVersionStruct,
};
use cml_chain_wasm::address::Address;
use cml_chain_wasm::assets::Coin;
use cml_chain_wasm::auxdata::Metadata;
use cml_chain_wasm::block::{OperationalCert, ProtocolVersion};
use cml_chain_wasm::certs::MIRPot;
use cml_chain_wasm::crypto::{
    KESSignature, Nonce, VRFCert, Vkey
};
use cml_crypto_wasm::{
    AuxiliaryDataHash, BlockBodyHash, BlockHeaderHash, Ed25519KeyHash, GenesisHash, VRFVkey,
};
use cml_chain_wasm::{Epoch, Rational, UnitInterval, Withdrawals};
use cml_chain_wasm::{
    BootstrapWitnessList, CertificateList, VkeywitnessList, TransactionInputList,
};
use crate::{
    MapStakeCredentialToCoin, MultisigScriptList,
    ShelleyTransactionBodyList, ShelleyTransactionOutputList, ShelleyTransactionWitnessSetList,
};
use cml_core::ordered_hash_map::OrderedHashMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MoveInstantaneousReward(cml_multi_era::shelley::MoveInstantaneousReward);

#[wasm_bindgen]
impl MoveInstantaneousReward {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MoveInstantaneousReward, JsValue> {
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

    pub fn from_json(json: &str) -> Result<MoveInstantaneousReward, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn pot(&self) -> MIRPot {
        self.0.pot
    }

    pub fn to_stake_credentials(&self) -> MapStakeCredentialToCoin {
        self.0.to_stake_credentials.clone().into()
    }

    pub fn new(pot: MIRPot, to_stake_credentials: &MapStakeCredentialToCoin) -> Self {
        Self(cml_multi_era::shelley::MoveInstantaneousReward::new(
            pot.into(),
            to_stake_credentials.clone().into(),
        ))
    }
}

impl From<cml_multi_era::shelley::MoveInstantaneousReward> for MoveInstantaneousReward {
    fn from(native: cml_multi_era::shelley::MoveInstantaneousReward) -> Self {
        Self(native)
    }
}

impl From<MoveInstantaneousReward> for cml_multi_era::shelley::MoveInstantaneousReward {
    fn from(wasm: MoveInstantaneousReward) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::MoveInstantaneousReward> for MoveInstantaneousReward {
    fn as_ref(&self) -> &cml_multi_era::shelley::MoveInstantaneousReward {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultisigAll(cml_multi_era::shelley::MultisigAll);

#[wasm_bindgen]
impl MultisigAll {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MultisigAll, JsValue> {
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

    pub fn from_json(json: &str) -> Result<MultisigAll, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn multisig_scripts(&self) -> MultisigScriptList {
        self.0.multisig_scripts.clone().into()
    }

    pub fn new(multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigAll::new(
            multisig_scripts.clone().into(),
        ))
    }
}

impl From<cml_multi_era::shelley::MultisigAll> for MultisigAll {
    fn from(native: cml_multi_era::shelley::MultisigAll) -> Self {
        Self(native)
    }
}

impl From<MultisigAll> for cml_multi_era::shelley::MultisigAll {
    fn from(wasm: MultisigAll) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::MultisigAll> for MultisigAll {
    fn as_ref(&self) -> &cml_multi_era::shelley::MultisigAll {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultisigAny(cml_multi_era::shelley::MultisigAny);

#[wasm_bindgen]
impl MultisigAny {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MultisigAny, JsValue> {
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

    pub fn from_json(json: &str) -> Result<MultisigAny, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn multisig_scripts(&self) -> MultisigScriptList {
        self.0.multisig_scripts.clone().into()
    }

    pub fn new(multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigAny::new(
            multisig_scripts.clone().into(),
        ))
    }
}

impl From<cml_multi_era::shelley::MultisigAny> for MultisigAny {
    fn from(native: cml_multi_era::shelley::MultisigAny) -> Self {
        Self(native)
    }
}

impl From<MultisigAny> for cml_multi_era::shelley::MultisigAny {
    fn from(wasm: MultisigAny) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::MultisigAny> for MultisigAny {
    fn as_ref(&self) -> &cml_multi_era::shelley::MultisigAny {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultisigNOfK(cml_multi_era::shelley::MultisigNOfK);

#[wasm_bindgen]
impl MultisigNOfK {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MultisigNOfK, JsValue> {
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

    pub fn from_json(json: &str) -> Result<MultisigNOfK, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn n(&self) -> u64 {
        self.0.n
    }

    pub fn multisig_scripts(&self) -> MultisigScriptList {
        self.0.multisig_scripts.clone().into()
    }

    pub fn new(n: u64, multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigNOfK::new(
            n,
            multisig_scripts.clone().into(),
        ))
    }
}

impl From<cml_multi_era::shelley::MultisigNOfK> for MultisigNOfK {
    fn from(native: cml_multi_era::shelley::MultisigNOfK) -> Self {
        Self(native)
    }
}

impl From<MultisigNOfK> for cml_multi_era::shelley::MultisigNOfK {
    fn from(wasm: MultisigNOfK) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::MultisigNOfK> for MultisigNOfK {
    fn as_ref(&self) -> &cml_multi_era::shelley::MultisigNOfK {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultisigPubkey(cml_multi_era::shelley::MultisigPubkey);

#[wasm_bindgen]
impl MultisigPubkey {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MultisigPubkey, JsValue> {
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

    pub fn from_json(json: &str) -> Result<MultisigPubkey, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn ed25519_key_hash(&self) -> Ed25519KeyHash {
        self.0.ed25519_key_hash.clone().into()
    }

    pub fn new(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_multi_era::shelley::MultisigPubkey::new(
            ed25519_key_hash.clone().into(),
        ))
    }
}

impl From<cml_multi_era::shelley::MultisigPubkey> for MultisigPubkey {
    fn from(native: cml_multi_era::shelley::MultisigPubkey) -> Self {
        Self(native)
    }
}

impl From<MultisigPubkey> for cml_multi_era::shelley::MultisigPubkey {
    fn from(wasm: MultisigPubkey) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::MultisigPubkey> for MultisigPubkey {
    fn as_ref(&self) -> &cml_multi_era::shelley::MultisigPubkey {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultisigScript(cml_multi_era::shelley::MultisigScript);

#[wasm_bindgen]
impl MultisigScript {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MultisigScript, JsValue> {
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

    pub fn from_json(json: &str) -> Result<MultisigScript, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_multisig_pubkey(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_multi_era::shelley::MultisigScript::new_multisig_pubkey(
            ed25519_key_hash.clone().into(),
        ))
    }

    pub fn new_multisig_all(multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigScript::new_multisig_all(
            multisig_scripts.clone().into(),
        ))
    }

    pub fn new_multisig_any(multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigScript::new_multisig_any(
            multisig_scripts.clone().into(),
        ))
    }

    pub fn new_multisig_n_of_k(n: u64, multisig_scripts: &MultisigScriptList) -> Self {
        Self(cml_multi_era::shelley::MultisigScript::new_multisig_n_of_k(
            n,
            multisig_scripts.clone().into(),
        ))
    }

    pub fn kind(&self) -> MultisigScriptKind {
        match &self.0 {
            cml_multi_era::shelley::MultisigScript::MultisigPubkey(_) => {
                MultisigScriptKind::MultisigPubkey
            }
            cml_multi_era::shelley::MultisigScript::MultisigAll(_) => {
                MultisigScriptKind::MultisigAll
            }
            cml_multi_era::shelley::MultisigScript::MultisigAny(_) => {
                MultisigScriptKind::MultisigAny
            }
            cml_multi_era::shelley::MultisigScript::MultisigNOfK(_) => {
                MultisigScriptKind::MultisigNOfK
            }
        }
    }

    pub fn as_multisig_pubkey(&self) -> Option<MultisigPubkey> {
        match &self.0 {
            cml_multi_era::shelley::MultisigScript::MultisigPubkey(multisig_pubkey) => {
                Some(multisig_pubkey.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_multisig_all(&self) -> Option<MultisigAll> {
        match &self.0 {
            cml_multi_era::shelley::MultisigScript::MultisigAll(multisig_all) => {
                Some(multisig_all.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_multisig_any(&self) -> Option<MultisigAny> {
        match &self.0 {
            cml_multi_era::shelley::MultisigScript::MultisigAny(multisig_any) => {
                Some(multisig_any.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_multisig_n_of_k(&self) -> Option<MultisigNOfK> {
        match &self.0 {
            cml_multi_era::shelley::MultisigScript::MultisigNOfK(multisig_n_of_k) => {
                Some(multisig_n_of_k.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cml_multi_era::shelley::MultisigScript> for MultisigScript {
    fn from(native: cml_multi_era::shelley::MultisigScript) -> Self {
        Self(native)
    }
}

impl From<MultisigScript> for cml_multi_era::shelley::MultisigScript {
    fn from(wasm: MultisigScript) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::MultisigScript> for MultisigScript {
    fn as_ref(&self) -> &cml_multi_era::shelley::MultisigScript {
        &self.0
    }
}

#[wasm_bindgen]
pub enum MultisigScriptKind {
    MultisigPubkey,
    MultisigAll,
    MultisigAny,
    MultisigNOfK,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyBlock(cml_multi_era::shelley::ShelleyBlock);

#[wasm_bindgen]
impl ShelleyBlock {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ShelleyBlock, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ShelleyBlock, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn header(&self) -> ShelleyHeader {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> ShelleyTransactionBodyList {
        self.0.transaction_bodies.clone().into()
    }

    pub fn transaction_witness_sets(&self) -> ShelleyTransactionWitnessSetList {
        self.0.transaction_witness_sets.clone().into()
    }

    pub fn transaction_metadata_set(&self) -> Metadata {
        self.0.transaction_metadata_set.clone().into()
    }

    pub fn new(
        header: &ShelleyHeader,
        transaction_bodies: &ShelleyTransactionBodyList,
        transaction_witness_sets: &ShelleyTransactionWitnessSetList,
        transaction_metadata_set: &Metadata,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyBlock::new(
            header.clone().into(),
            transaction_bodies.clone().into(),
            transaction_witness_sets.clone().into(),
            transaction_metadata_set.clone().into(),
        ))
    }
}

impl From<cml_multi_era::shelley::ShelleyBlock> for ShelleyBlock {
    fn from(native: cml_multi_era::shelley::ShelleyBlock) -> Self {
        Self(native)
    }
}

impl From<ShelleyBlock> for cml_multi_era::shelley::ShelleyBlock {
    fn from(wasm: ShelleyBlock) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::ShelleyBlock> for ShelleyBlock {
    fn as_ref(&self) -> &cml_multi_era::shelley::ShelleyBlock {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyHeader(cml_multi_era::shelley::ShelleyHeader);

#[wasm_bindgen]
impl ShelleyHeader {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ShelleyHeader, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ShelleyHeader, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn body(&self) -> ShelleyHeaderBody {
        self.0.body.clone().into()
    }

    pub fn signature(&self) -> KESSignature {
        self.0.signature.clone().into()
    }

    pub fn new(body: &ShelleyHeaderBody, signature: &KESSignature) -> Self {
        Self(cml_multi_era::shelley::ShelleyHeader::new(
            body.clone().into(),
            signature.clone().into(),
        ))
    }
}

impl From<cml_multi_era::shelley::ShelleyHeader> for ShelleyHeader {
    fn from(native: cml_multi_era::shelley::ShelleyHeader) -> Self {
        Self(native)
    }
}

impl From<ShelleyHeader> for cml_multi_era::shelley::ShelleyHeader {
    fn from(wasm: ShelleyHeader) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::ShelleyHeader> for ShelleyHeader {
    fn as_ref(&self) -> &cml_multi_era::shelley::ShelleyHeader {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyHeaderBody(cml_multi_era::shelley::ShelleyHeaderBody);

#[wasm_bindgen]
impl ShelleyHeaderBody {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ShelleyHeaderBody, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ShelleyHeaderBody, JsValue> {
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

    pub fn v_r_f_vkey(&self) -> VRFVkey {
        self.0.v_r_f_vkey.clone().into()
    }

    pub fn nonce_vrf(&self) -> VRFCert {
        self.0.nonce_vrf.clone().into()
    }

    pub fn leader_vrf(&self) -> VRFCert {
        self.0.leader_vrf.clone().into()
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
        v_r_f_vkey: &VRFVkey,
        nonce_vrf: &VRFCert,
        leader_vrf: &VRFCert,
        block_body_size: u64,
        block_body_hash: &BlockBodyHash,
        operational_cert: &OperationalCert,
        protocol_version: &ProtocolVersion,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyHeaderBody::new(
            block_number,
            slot,
            prev_hash.map(Into::into),
            issuer_vkey.clone().into(),
            v_r_f_vkey.clone().into(),
            nonce_vrf.clone().into(),
            leader_vrf.clone().into(),
            block_body_size,
            block_body_hash.clone().into(),
            operational_cert.clone().into(),
            protocol_version.clone().into(),
        ))
    }
}

impl From<cml_multi_era::shelley::ShelleyHeaderBody> for ShelleyHeaderBody {
    fn from(native: cml_multi_era::shelley::ShelleyHeaderBody) -> Self {
        Self(native)
    }
}

impl From<ShelleyHeaderBody> for cml_multi_era::shelley::ShelleyHeaderBody {
    fn from(wasm: ShelleyHeaderBody) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::ShelleyHeaderBody> for ShelleyHeaderBody {
    fn as_ref(&self) -> &cml_multi_era::shelley::ShelleyHeaderBody {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyProposedProtocolParameterUpdates(
    cml_multi_era::shelley::ShelleyProposedProtocolParameterUpdates,
);

#[wasm_bindgen]
impl ShelleyProposedProtocolParameterUpdates {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &GenesisHash,
        value: &ShelleyProtocolParamUpdate,
    ) -> Option<ShelleyProtocolParamUpdate> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &GenesisHash) -> Option<ShelleyProtocolParamUpdate> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> GenesisHashList {
        self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>().into()
    }
}

impl From<cml_multi_era::shelley::ShelleyProposedProtocolParameterUpdates>
    for ShelleyProposedProtocolParameterUpdates
{
    fn from(native: cml_multi_era::shelley::ShelleyProposedProtocolParameterUpdates) -> Self {
        Self(native)
    }
}

impl From<ShelleyProposedProtocolParameterUpdates>
    for cml_multi_era::shelley::ShelleyProposedProtocolParameterUpdates
{
    fn from(wasm: ShelleyProposedProtocolParameterUpdates) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::ShelleyProposedProtocolParameterUpdates>
    for ShelleyProposedProtocolParameterUpdates
{
    fn as_ref(&self) -> &cml_multi_era::shelley::ShelleyProposedProtocolParameterUpdates {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyProtocolParamUpdate(cml_multi_era::shelley::ShelleyProtocolParamUpdate);

#[wasm_bindgen]
impl ShelleyProtocolParamUpdate {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ShelleyProtocolParamUpdate, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ShelleyProtocolParamUpdate, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn set_minfee_a(&mut self, minfee_a: u64) {
        self.0.minfee_a = Some(minfee_a)
    }

    pub fn minfee_a(&self) -> Option<u64> {
        self.0.minfee_a
    }

    pub fn set_minfee_b(&mut self, minfee_b: u64) {
        self.0.minfee_b = Some(minfee_b)
    }

    pub fn minfee_b(&self) -> Option<u64> {
        self.0.minfee_b
    }

    pub fn set_max_block_body_size(&mut self, max_block_body_size: u64) {
        self.0.max_block_body_size = Some(max_block_body_size)
    }

    pub fn max_block_body_size(&self) -> Option<u64> {
        self.0.max_block_body_size
    }

    pub fn set_max_transaction_size(&mut self, max_transaction_size: u64) {
        self.0.max_transaction_size = Some(max_transaction_size)
    }

    pub fn max_transaction_size(&self) -> Option<u64> {
        self.0.max_transaction_size
    }

    pub fn set_max_block_header_size(&mut self, max_block_header_size: u64) {
        self.0.max_block_header_size = Some(max_block_header_size)
    }

    pub fn max_block_header_size(&self) -> Option<u64> {
        self.0.max_block_header_size
    }

    pub fn set_key_deposit(&mut self, key_deposit: Coin) {
        self.0.key_deposit = Some(key_deposit)
    }

    pub fn key_deposit(&self) -> Option<Coin> {
        self.0.key_deposit
    }

    pub fn set_pool_deposit(&mut self, pool_deposit: Coin) {
        self.0.pool_deposit = Some(pool_deposit)
    }

    pub fn pool_deposit(&self) -> Option<Coin> {
        self.0.pool_deposit
    }

    pub fn set_maximum_epoch(&mut self, maximum_epoch: Epoch) {
        self.0.maximum_epoch = Some(maximum_epoch)
    }

    pub fn maximum_epoch(&self) -> Option<Epoch> {
        self.0.maximum_epoch
    }

    pub fn set_n_opt(&mut self, n_opt: u64) {
        self.0.n_opt = Some(n_opt)
    }

    pub fn n_opt(&self) -> Option<u64> {
        self.0.n_opt
    }

    pub fn set_pool_pledge_influence(&mut self, pool_pledge_influence: &Rational) {
        self.0.pool_pledge_influence = Some(pool_pledge_influence.clone().into())
    }

    pub fn pool_pledge_influence(&self) -> Option<Rational> {
        self.0
            .pool_pledge_influence
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_expansion_rate(&mut self, expansion_rate: &UnitInterval) {
        self.0.expansion_rate = Some(expansion_rate.clone().into())
    }

    pub fn expansion_rate(&self) -> Option<UnitInterval> {
        self.0.expansion_rate.clone().map(std::convert::Into::into)
    }

    pub fn set_treasury_growth_rate(&mut self, treasury_growth_rate: &UnitInterval) {
        self.0.treasury_growth_rate = Some(treasury_growth_rate.clone().into())
    }

    pub fn treasury_growth_rate(&self) -> Option<UnitInterval> {
        self.0
            .treasury_growth_rate
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_decentralization_constant(&mut self, decentralization_constant: &UnitInterval) {
        self.0.decentralization_constant = Some(decentralization_constant.clone().into())
    }

    pub fn decentralization_constant(&self) -> Option<UnitInterval> {
        self.0
            .decentralization_constant
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_extra_entropy(&mut self, extra_entropy: &Nonce) {
        self.0.extra_entropy = Some(extra_entropy.clone().into())
    }

    pub fn extra_entropy(&self) -> Option<Nonce> {
        self.0.extra_entropy.clone().map(std::convert::Into::into)
    }

    pub fn set_protocol_version(&mut self, protocol_version: &ProtocolVersionStruct) {
        self.0.protocol_version = Some(protocol_version.clone().into())
    }

    pub fn protocol_version(&self) -> Option<ProtocolVersionStruct> {
        self.0
            .protocol_version
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_min_utxo_value(&mut self, min_utxo_value: Coin) {
        self.0.min_utxo_value = Some(min_utxo_value)
    }

    pub fn min_utxo_value(&self) -> Option<Coin> {
        self.0.min_utxo_value
    }

    pub fn new() -> Self {
        Self(cml_multi_era::shelley::ShelleyProtocolParamUpdate::new())
    }
}

impl From<cml_multi_era::shelley::ShelleyProtocolParamUpdate> for ShelleyProtocolParamUpdate {
    fn from(native: cml_multi_era::shelley::ShelleyProtocolParamUpdate) -> Self {
        Self(native)
    }
}

impl From<ShelleyProtocolParamUpdate> for cml_multi_era::shelley::ShelleyProtocolParamUpdate {
    fn from(wasm: ShelleyProtocolParamUpdate) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::ShelleyProtocolParamUpdate> for ShelleyProtocolParamUpdate {
    fn as_ref(&self) -> &cml_multi_era::shelley::ShelleyProtocolParamUpdate {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTransaction(cml_multi_era::shelley::ShelleyTransaction);

#[wasm_bindgen]
impl ShelleyTransaction {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ShelleyTransaction, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ShelleyTransaction, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn body(&self) -> ShelleyTransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> ShelleyTransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn metadata(&self) -> Option<Metadata> {
        self.0.metadata.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &ShelleyTransactionBody,
        witness_set: &ShelleyTransactionWitnessSet,
        metadata: Option<Metadata>,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyTransaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            metadata.map(Into::into),
        ))
    }
}

impl From<cml_multi_era::shelley::ShelleyTransaction> for ShelleyTransaction {
    fn from(native: cml_multi_era::shelley::ShelleyTransaction) -> Self {
        Self(native)
    }
}

impl From<ShelleyTransaction> for cml_multi_era::shelley::ShelleyTransaction {
    fn from(wasm: ShelleyTransaction) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::ShelleyTransaction> for ShelleyTransaction {
    fn as_ref(&self) -> &cml_multi_era::shelley::ShelleyTransaction {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTransactionBody(cml_multi_era::shelley::ShelleyTransactionBody);

#[wasm_bindgen]
impl ShelleyTransactionBody {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ShelleyTransactionBody, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ShelleyTransactionBody, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> ShelleyTransactionOutputList {
        self.0.outputs.clone().into()
    }

    pub fn fee(&self) -> Coin {
        self.0.fee
    }

    pub fn ttl(&self) -> u64 {
        self.0.ttl
    }

    pub fn set_certs(&mut self, certs: &CertificateList) {
        self.0.certs = Some(certs.clone().into())
    }

    pub fn certs(&self) -> Option<CertificateList> {
        self.0.certs.clone().map(std::convert::Into::into)
    }

    pub fn set_withdrawals(&mut self, withdrawals: &Withdrawals) {
        self.0.withdrawals = Some(withdrawals.clone().into())
    }

    pub fn withdrawals(&self) -> Option<Withdrawals> {
        self.0.withdrawals.clone().map(std::convert::Into::into)
    }

    pub fn set_update(&mut self, update: &ShelleyUpdate) {
        self.0.update = Some(update.clone().into())
    }

    pub fn update(&self) -> Option<ShelleyUpdate> {
        self.0.update.clone().map(std::convert::Into::into)
    }

    pub fn set_auxiliary_data_hash(&mut self, auxiliary_data_hash: &AuxiliaryDataHash) {
        self.0.auxiliary_data_hash = Some(auxiliary_data_hash.clone().into())
    }

    pub fn auxiliary_data_hash(&self) -> Option<AuxiliaryDataHash> {
        self.0
            .auxiliary_data_hash
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new(
        inputs: &TransactionInputList,
        outputs: &ShelleyTransactionOutputList,
        fee: Coin,
        ttl: u64,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyTransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
            ttl,
        ))
    }
}

impl From<cml_multi_era::shelley::ShelleyTransactionBody> for ShelleyTransactionBody {
    fn from(native: cml_multi_era::shelley::ShelleyTransactionBody) -> Self {
        Self(native)
    }
}

impl From<ShelleyTransactionBody> for cml_multi_era::shelley::ShelleyTransactionBody {
    fn from(wasm: ShelleyTransactionBody) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::ShelleyTransactionBody> for ShelleyTransactionBody {
    fn as_ref(&self) -> &cml_multi_era::shelley::ShelleyTransactionBody {
        &self.0
    }
}

pub type ShelleyTransactionIndex = u16;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTransactionOutput(cml_multi_era::shelley::ShelleyTransactionOutput);

#[wasm_bindgen]
impl ShelleyTransactionOutput {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ShelleyTransactionOutput, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ShelleyTransactionOutput, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> Coin {
        self.0.amount
    }

    pub fn new(address: &Address, amount: Coin) -> Self {
        Self(cml_multi_era::shelley::ShelleyTransactionOutput::new(
            address.clone().into(),
            amount,
        ))
    }
}

impl From<cml_multi_era::shelley::ShelleyTransactionOutput> for ShelleyTransactionOutput {
    fn from(native: cml_multi_era::shelley::ShelleyTransactionOutput) -> Self {
        Self(native)
    }
}

impl From<ShelleyTransactionOutput> for cml_multi_era::shelley::ShelleyTransactionOutput {
    fn from(wasm: ShelleyTransactionOutput) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::ShelleyTransactionOutput> for ShelleyTransactionOutput {
    fn as_ref(&self) -> &cml_multi_era::shelley::ShelleyTransactionOutput {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTransactionWitnessSet(cml_multi_era::shelley::ShelleyTransactionWitnessSet);

#[wasm_bindgen]
impl ShelleyTransactionWitnessSet {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ShelleyTransactionWitnessSet, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ShelleyTransactionWitnessSet, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn set_vkeywitnesses(&mut self, vkeywitnesses: &VkeywitnessList) {
        self.0.vkeywitnesses = Some(vkeywitnesses.clone().into())
    }

    pub fn vkeywitnesses(&self) -> Option<VkeywitnessList> {
        self.0.vkeywitnesses.clone().map(std::convert::Into::into)
    }

    pub fn set_native_scripts(&mut self, native_scripts: &MultisigScriptList) {
        self.0.native_scripts = Some(native_scripts.clone().into())
    }

    pub fn native_scripts(&self) -> Option<MultisigScriptList> {
        self.0.native_scripts.clone().map(std::convert::Into::into)
    }

    pub fn set_bootstrap_witnesses(&mut self, bootstrap_witnesses: &BootstrapWitnessList) {
        self.0.bootstrap_witnesses = Some(bootstrap_witnesses.clone().into())
    }

    pub fn bootstrap_witnesses(&self) -> Option<BootstrapWitnessList> {
        self.0
            .bootstrap_witnesses
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(cml_multi_era::shelley::ShelleyTransactionWitnessSet::new())
    }
}

impl From<cml_multi_era::shelley::ShelleyTransactionWitnessSet> for ShelleyTransactionWitnessSet {
    fn from(native: cml_multi_era::shelley::ShelleyTransactionWitnessSet) -> Self {
        Self(native)
    }
}

impl From<ShelleyTransactionWitnessSet> for cml_multi_era::shelley::ShelleyTransactionWitnessSet {
    fn from(wasm: ShelleyTransactionWitnessSet) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::ShelleyTransactionWitnessSet> for ShelleyTransactionWitnessSet {
    fn as_ref(&self) -> &cml_multi_era::shelley::ShelleyTransactionWitnessSet {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyUpdate(cml_multi_era::shelley::ShelleyUpdate);

#[wasm_bindgen]
impl ShelleyUpdate {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ShelleyUpdate, JsValue> {
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

    pub fn from_json(json: &str) -> Result<ShelleyUpdate, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn shelley_proposed_protocol_parameter_updates(
        &self,
    ) -> ShelleyProposedProtocolParameterUpdates {
        self.0
            .shelley_proposed_protocol_parameter_updates
            .clone()
            .into()
    }

    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    pub fn new(
        shelley_proposed_protocol_parameter_updates: &ShelleyProposedProtocolParameterUpdates,
        epoch: Epoch,
    ) -> Self {
        Self(cml_multi_era::shelley::ShelleyUpdate::new(
            shelley_proposed_protocol_parameter_updates.clone().into(),
            epoch,
        ))
    }
}

impl From<cml_multi_era::shelley::ShelleyUpdate> for ShelleyUpdate {
    fn from(native: cml_multi_era::shelley::ShelleyUpdate) -> Self {
        Self(native)
    }
}

impl From<ShelleyUpdate> for cml_multi_era::shelley::ShelleyUpdate {
    fn from(wasm: ShelleyUpdate) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::shelley::ShelleyUpdate> for ShelleyUpdate {
    fn as_ref(&self) -> &cml_multi_era::shelley::ShelleyUpdate {
        &self.0
    }
}
