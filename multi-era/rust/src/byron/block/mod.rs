// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod serialization;

use crate::byron::delegation::{
    ByronDelegation, ByronDelegationSignature, LightWeightDelegationSignature,
};
use crate::byron::mpc::{Ssc, SscProof};
use crate::byron::transaction::{ByronAttributes, ByronTx, ByronTxProof, ByronTxWitness};
use crate::byron::update::{ByronBlockVersion, ByronSoftwareVersion, ByronUpdate};
use crate::byron::{Blake2b256, ByronBlockId, ByronPubKey, ByronSignature, ByronSlotId, EpochId};

use cml_chain::byron::StakeholderId;

use std::collections::BTreeMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BlockHeaderExtraData {
    pub block_version: ByronBlockVersion,
    pub software_version: ByronSoftwareVersion,
    pub byron_attributes: ByronAttributes,
    pub extra_proof: Blake2b256,
}

impl BlockHeaderExtraData {
    pub fn new(
        block_version: ByronBlockVersion,
        software_version: ByronSoftwareVersion,
        byron_attributes: ByronAttributes,
        extra_proof: Blake2b256,
    ) -> Self {
        Self {
            block_version,
            software_version,
            byron_attributes,
            extra_proof,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum ByronBlock {
    EpochBoundary(ByronEbBlock),
    Main(ByronMainBlock),
}

impl ByronBlock {
    pub fn new_epoch_boundary(epoch_boundary: ByronEbBlock) -> Self {
        Self::EpochBoundary(epoch_boundary)
    }

    pub fn new_main(main: ByronMainBlock) -> Self {
        Self::Main(main)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronBlockBody {
    pub tx_payload: TxPayload,
    pub ssc_payload: Ssc,
    pub dlg_payload: Vec<ByronDelegation>,
    pub upd_payload: ByronUpdate,
}

impl ByronBlockBody {
    pub fn new(
        tx_payload: TxPayload,
        ssc_payload: Ssc,
        dlg_payload: Vec<ByronDelegation>,
        upd_payload: ByronUpdate,
    ) -> Self {
        Self {
            tx_payload,
            ssc_payload,
            dlg_payload,
            upd_payload,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronBlockConsensusData {
    pub byron_slot_id: ByronSlotId,
    pub byron_pub_key: ByronPubKey,
    pub byron_difficulty: ByronDifficulty,
    pub byron_block_signature: ByronBlockSignature,
}

impl ByronBlockConsensusData {
    pub fn new(
        byron_slot_id: ByronSlotId,
        byron_pub_key: ByronPubKey,
        byron_difficulty: ByronDifficulty,
        byron_block_signature: ByronBlockSignature,
    ) -> Self {
        Self {
            byron_slot_id,
            byron_pub_key,
            byron_difficulty,
            byron_block_signature,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronBlockHeader {
    pub protocol_magic: u32,
    pub prev_block: ByronBlockId,
    pub body_proof: ByronBodyProof,
    pub consensus_data: ByronBlockConsensusData,
    pub extra_data: BlockHeaderExtraData,
}

impl ByronBlockHeader {
    pub fn new(
        protocol_magic: u32,
        prev_block: ByronBlockId,
        body_proof: ByronBodyProof,
        consensus_data: ByronBlockConsensusData,
        extra_data: BlockHeaderExtraData,
    ) -> Self {
        Self {
            protocol_magic,
            prev_block,
            body_proof,
            consensus_data,
            extra_data,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum ByronBlockSignature {
    Signature(ByronBlockSignatureNormal),
    ProxyLight(ByronBlockSignatureProxyLight),
    ProxyHeavy(ByronBlockSignatureProxyHeavy),
}

impl ByronBlockSignature {
    pub fn new_signature(signature: ByronBlockSignatureNormal) -> Self {
        Self::Signature(signature)
    }

    pub fn new_proxy_light(proxy_light: ByronBlockSignatureProxyLight) -> Self {
        Self::ProxyLight(proxy_light)
    }

    pub fn new_proxy_heavy(proxy_heavy: ByronBlockSignatureProxyHeavy) -> Self {
        Self::ProxyHeavy(proxy_heavy)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronBlockSignatureNormal {
    pub signature: ByronSignature,
}

impl ByronBlockSignatureNormal {
    pub fn new(signature: ByronSignature) -> Self {
        Self { signature }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronBlockSignatureProxyHeavy {
    pub signature: ByronDelegationSignature,
}

impl ByronBlockSignatureProxyHeavy {
    pub fn new(signature: ByronDelegationSignature) -> Self {
        Self { signature }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronBlockSignatureProxyLight {
    pub signature: LightWeightDelegationSignature,
}

impl ByronBlockSignatureProxyLight {
    pub fn new(signature: LightWeightDelegationSignature) -> Self {
        Self { signature }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronBodyProof {
    pub tx_proof: ByronTxProof,
    pub ssc_proof: SscProof,
    pub dlg_proof: Blake2b256,
    pub upd_proof: Blake2b256,
}

impl ByronBodyProof {
    pub fn new(
        tx_proof: ByronTxProof,
        ssc_proof: SscProof,
        dlg_proof: Blake2b256,
        upd_proof: Blake2b256,
    ) -> Self {
        Self {
            tx_proof,
            ssc_proof,
            dlg_proof,
            upd_proof,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronDifficulty {
    pub u64: u64,
}

impl ByronDifficulty {
    pub fn new(u64: u64) -> Self {
        Self { u64 }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronEbBlock {
    pub header: EbbHead,
    pub body: Vec<StakeholderId>,
    pub extra: Vec<ByronAttributes>,
}

impl ByronEbBlock {
    pub fn new(header: EbbHead, body: Vec<StakeholderId>, extra: Vec<ByronAttributes>) -> Self {
        Self {
            header,
            body,
            extra,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronMainBlock {
    pub header: ByronBlockHeader,
    pub body: ByronBlockBody,
    pub extra: Vec<ByronAttributes>,
}

impl ByronMainBlock {
    pub fn new(
        header: ByronBlockHeader,
        body: ByronBlockBody,
        extra: Vec<ByronAttributes>,
    ) -> Self {
        Self {
            header,
            body,
            extra,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct EbbConsensusData {
    pub epoch_id: EpochId,
    pub byron_difficulty: ByronDifficulty,
}

impl EbbConsensusData {
    pub fn new(epoch_id: EpochId, byron_difficulty: ByronDifficulty) -> Self {
        Self {
            epoch_id,
            byron_difficulty,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct EbbHead {
    pub protocol_magic: u32,
    pub prev_block: ByronBlockId,
    pub body_proof: Blake2b256,
    pub consensus_data: EbbConsensusData,
    pub extra_data: Vec<ByronAttributes>,
}

impl EbbHead {
    pub fn new(
        protocol_magic: u32,
        prev_block: ByronBlockId,
        body_proof: Blake2b256,
        consensus_data: EbbConsensusData,
        extra_data: Vec<ByronAttributes>,
    ) -> Self {
        Self {
            protocol_magic,
            prev_block,
            body_proof,
            consensus_data,
            extra_data,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct TxAux {
    pub byron_tx: ByronTx,
    pub byron_tx_witnesss: Vec<ByronTxWitness>,
}

impl TxAux {
    pub fn new(byron_tx: ByronTx, byron_tx_witnesss: Vec<ByronTxWitness>) -> Self {
        Self {
            byron_tx,
            byron_tx_witnesss,
        }
    }
}

pub type TxPayload = Vec<TxAux>;
