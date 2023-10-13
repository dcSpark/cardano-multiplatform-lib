// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::{
    MapTransactionIndexToAuxiliaryData, TransactionBodyList, TransactionIndex,
    TransactionWitnessSetList,
};
use crate::crypto::{KESSignature, VRFCert, Vkey};
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use cml_crypto_wasm::{BlockBodyHash, BlockHeaderHash, Ed25519Signature, KESVkey, VRFVkey};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Block(cml_chain::block::Block);

impl_wasm_cbor_json_api!(Block);

impl_wasm_conversions!(cml_chain::block::Block, Block);

#[wasm_bindgen]
impl Block {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Header(cml_chain::block::Header);

impl_wasm_cbor_json_api!(Header);

impl_wasm_conversions!(cml_chain::block::Header, Header);

#[wasm_bindgen]
impl Header {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct HeaderBody(cml_chain::block::HeaderBody);

impl_wasm_cbor_json_api!(HeaderBody);

impl_wasm_conversions!(cml_chain::block::HeaderBody, HeaderBody);

#[wasm_bindgen]
impl HeaderBody {
    pub fn block_number(&self) -> u64 {
        self.0.block_number
    }

    pub fn slot(&self) -> u64 {
        self.0.slot
    }

    pub fn prev_hash(&self) -> Option<BlockHeaderHash> {
        self.0.prev_hash.map(std::convert::Into::into)
    }

    pub fn issuer_vkey(&self) -> Vkey {
        self.0.issuer_vkey.clone().into()
    }

    pub fn vrf_vkey(&self) -> VRFVkey {
        self.0.vrf_vkey.into()
    }

    pub fn vrf_result(&self) -> VRFCert {
        self.0.vrf_result.clone().into()
    }

    pub fn block_body_size(&self) -> u64 {
        self.0.block_body_size
    }

    pub fn block_body_hash(&self) -> BlockBodyHash {
        self.0.block_body_hash.into()
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct OperationalCert(cml_chain::block::OperationalCert);

impl_wasm_cbor_json_api!(OperationalCert);

impl_wasm_conversions!(cml_chain::block::OperationalCert, OperationalCert);

#[wasm_bindgen]
impl OperationalCert {
    pub fn hot_vkey(&self) -> KESVkey {
        self.0.hot_vkey.into()
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ProtocolVersion(cml_chain::block::ProtocolVersion);

impl_wasm_cbor_json_api!(ProtocolVersion);

impl_wasm_conversions!(cml_chain::block::ProtocolVersion, ProtocolVersion);

#[wasm_bindgen]
impl ProtocolVersion {
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
