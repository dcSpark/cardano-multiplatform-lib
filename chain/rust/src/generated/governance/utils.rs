use cml_crypto::{Ed25519KeyHash, ScriptHash};

use super::{GovAction, Voter};

impl GovAction {
    pub fn script_hash(&self) -> Option<&ScriptHash> {
        match self {
            Self::ParameterChangeAction(action) => action.policy_hash.as_ref(),
            Self::HardForkInitiationAction(_action) => None,
            Self::TreasuryWithdrawalsAction(action) => action.policy_hash.as_ref(),
            Self::NoConfidence(_action) => None,
            // TODO: unsure if these count? they can be credentials but maybe it's not needed to sign
            Self::UpdateCommittee(_action) => None,
            // TODO: unsure if this counts?
            //Self::NewConstitution(action) => action.constitution.script_hash,
            Self::NewConstitution(_action) => None,
            Self::InfoAction { .. } => None,
        }
    }
}

impl Voter {
    pub fn key_hash(&self) -> Option<&Ed25519KeyHash> {
        match self {
            Self::ConstitutionalCommitteeHotKeyHash {
                ed25519_key_hash, ..
            } => Some(ed25519_key_hash),
            Self::ConstitutionalCommitteeHotScriptHash { .. } => None,
            Self::DRepKeyHash {
                ed25519_key_hash, ..
            } => Some(ed25519_key_hash),
            Self::DRepScriptHash { .. } => None,
            Self::StakingPoolKeyHash {
                ed25519_key_hash, ..
            } => Some(ed25519_key_hash),
        }
    }

    pub fn script_hash(&self) -> Option<&ScriptHash> {
        match self {
            Self::ConstitutionalCommitteeHotKeyHash { .. } => None,
            Self::ConstitutionalCommitteeHotScriptHash { script_hash, .. } => Some(script_hash),
            Self::DRepKeyHash { .. } => None,
            Self::DRepScriptHash { script_hash, .. } => Some(script_hash),
            Self::StakingPoolKeyHash { .. } => None,
        }
    }
}
