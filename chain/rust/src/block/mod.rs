// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod cbor_encodings;
pub mod serialization;

use super::TransactionIndex;
use crate::auxdata::AuxiliaryData;
use crate::crypto::{
    BlockBodyHash, BlockHeaderHash, Ed25519Signature, KESSignature, KESVkey, VRFCert, VRFVkey, Vkey,
};
use crate::transaction::{TransactionBody, TransactionWitnessSet};
use cbor_encodings::{
    BlockEncoding, HeaderBodyEncoding, HeaderEncoding, OperationalCertEncoding,
    ProtocolVersionEncoding,
};
use cml_core::ordered_hash_map::OrderedHashMap;
use std::collections::BTreeMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Block {
    pub header: Header,
    pub transaction_bodies: Vec<TransactionBody>,
    pub transaction_witness_sets: Vec<TransactionWitnessSet>,
    pub auxiliary_data_set: OrderedHashMap<TransactionIndex, AuxiliaryData>,
    pub invalid_transactions: Vec<TransactionIndex>,
    #[serde(skip)]
    pub encodings: Option<BlockEncoding>,
}

impl Block {
    pub fn new(
        header: Header,
        transaction_bodies: Vec<TransactionBody>,
        transaction_witness_sets: Vec<TransactionWitnessSet>,
        auxiliary_data_set: OrderedHashMap<TransactionIndex, AuxiliaryData>,
        invalid_transactions: Vec<TransactionIndex>,
    ) -> Self {
        Self {
            header,
            transaction_bodies,
            transaction_witness_sets,
            auxiliary_data_set,
            invalid_transactions,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Header {
    pub header_body: HeaderBody,
    pub body_signature: KESSignature,
    #[serde(skip)]
    pub encodings: Option<HeaderEncoding>,
}

impl Header {
    pub fn new(header_body: HeaderBody, body_signature: KESSignature) -> Self {
        Self {
            header_body,
            body_signature,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct HeaderBody {
    pub block_number: u64,
    pub slot: u64,
    pub prev_hash: Option<BlockHeaderHash>,
    pub issuer_vkey: Vkey,
    pub vrf_vkey: VRFVkey,
    pub vrf_result: VRFCert,
    pub block_body_size: u64,
    pub block_body_hash: BlockBodyHash,
    pub operational_cert: OperationalCert,
    pub protocol_version: ProtocolVersion,
    #[serde(skip)]
    pub encodings: Option<HeaderBodyEncoding>,
}

impl HeaderBody {
    pub fn new(
        block_number: u64,
        slot: u64,
        prev_hash: Option<BlockHeaderHash>,
        issuer_vkey: Vkey,
        vrf_vkey: VRFVkey,
        vrf_result: VRFCert,
        block_body_size: u64,
        block_body_hash: BlockBodyHash,
        operational_cert: OperationalCert,
        protocol_version: ProtocolVersion,
    ) -> Self {
        Self {
            block_number,
            slot,
            prev_hash,
            issuer_vkey,
            vrf_vkey,
            vrf_result,
            block_body_size,
            block_body_hash,
            operational_cert,
            protocol_version,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct OperationalCert {
    pub hot_vkey: KESVkey,
    pub sequence_number: u64,
    pub kes_period: u64,
    pub sigma: Ed25519Signature,
    #[serde(skip)]
    pub encodings: Option<OperationalCertEncoding>,
}

impl OperationalCert {
    pub fn new(
        hot_vkey: KESVkey,
        sequence_number: u64,
        kes_period: u64,
        sigma: Ed25519Signature,
    ) -> Self {
        Self {
            hot_vkey,
            sequence_number,
            kes_period,
            sigma,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ProtocolVersion {
    pub major: u64,
    pub minor: u64,
    #[serde(skip)]
    pub encodings: Option<ProtocolVersionEncoding>,
}

impl ProtocolVersion {
    pub fn new(major: u64, minor: u64) -> Self {
        Self {
            major,
            minor,
            encodings: None,
        }
    }
}
