// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod serialization;

use crate::byron::{ByronPubKey, ByronSignature, EpochId};
use cml_core::error::*;
use std::collections::BTreeMap;
use std::convert::TryFrom;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronDelegation {
    pub epoch: EpochId,
    pub issuer: ByronPubKey,
    pub delegate: ByronPubKey,
    pub certificate: ByronSignature,
}

impl ByronDelegation {
    pub fn new(
        epoch: EpochId,
        issuer: ByronPubKey,
        delegate: ByronPubKey,
        certificate: ByronSignature,
    ) -> Self {
        Self {
            epoch,
            issuer,
            delegate,
            certificate,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ByronDelegationSignature {
    pub byron_delegation: ByronDelegation,
    pub byron_signature: ByronSignature,
}

impl ByronDelegationSignature {
    pub fn new(byron_delegation: ByronDelegation, byron_signature: ByronSignature) -> Self {
        Self {
            byron_delegation,
            byron_signature,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct EpochRange {
    pub epoch_id: EpochId,
    pub epoch_id2: EpochId,
}

impl EpochRange {
    pub fn new(epoch_id: EpochId, epoch_id2: EpochId) -> Self {
        Self {
            epoch_id,
            epoch_id2,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct LightWeightDelegationSignature {
    pub light_weight_dlg: LightWeightDlg,
    pub byron_signature: ByronSignature,
}

impl LightWeightDelegationSignature {
    pub fn new(light_weight_dlg: LightWeightDlg, byron_signature: ByronSignature) -> Self {
        Self {
            light_weight_dlg,
            byron_signature,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct LightWeightDlg {
    pub epoch_range: EpochRange,
    pub issuer: ByronPubKey,
    pub delegate: ByronPubKey,
    pub certificate: ByronSignature,
}

impl LightWeightDlg {
    pub fn new(
        epoch_range: EpochRange,
        issuer: ByronPubKey,
        delegate: ByronPubKey,
        certificate: ByronSignature,
    ) -> Self {
        Self {
            epoch_range,
            issuer,
            delegate,
            certificate,
        }
    }
}
