use cml_chain::{assets::PositiveCoin, Coin};
use cml_chain_wasm::{
    address::Address,
    assets::{Mint, Value},
    certs::{
        AuthCommitteeHotCert, PoolRegistration, PoolRetirement, RegCert, RegDrepCert,
        ResignCommitteeColdCert, StakeDelegation, StakeDeregistration, StakeRegDelegCert,
        StakeRegistration, StakeVoteDelegCert, StakeVoteRegDelegCert, UnregCert, UnregDrepCert,
        UpdateDrepCert, VoteDelegCert, VoteRegDelegCert,
    },
    governance::VotingProcedures,
    transaction::RequiredSigners,
    MapTransactionIndexToAuxiliaryData, NetworkId, ProposalProcedureList, TransactionInputList,
    TransactionWitnessSetList, Withdrawals,
};
use cml_core::TransactionIndex;
use cml_core_wasm::{impl_wasm_conversions, impl_wasm_json_api, impl_wasm_list};
use cml_crypto_wasm::{AuxiliaryDataHash, ScriptDataHash, TransactionHash};
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

use crate::{
    allegra::MoveInstantaneousRewardsCert, shelley::GenesisKeyDelegation, MultiEraBlock,
    MultiEraTransactionBody,
};

#[wasm_bindgen]
impl MultiEraBlock {
    /**
     * Parses a block given the network block format with explicit era tag
     *
     * Some tools (e.g. Pallas/Oura) won't give you the block format from the binary spec directly,
     * but will instead have it wrapped in some network wrapper array containing the explicit era tag.
     * If your CBOR looks like `[uint, <actual block here>]`
     * (likely starting with `82` in hex e.g. `8201`, `8204`, `8207`, etc)
     * then you should use this function instead of the regular from_cbor_bytes().
     */
    pub fn from_explicit_network_cbor_bytes(bytes: &[u8]) -> Result<MultiEraBlock, JsError> {
        cml_multi_era::MultiEraBlock::from_explicit_network_cbor_bytes(bytes)
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn transaction_bodies(&self) -> MultiEraTransactionBodyList {
        self.0.transaction_bodies().into()
    }

    pub fn transaction_witness_sets(&self) -> TransactionWitnessSetList {
        self.0.transaction_witness_sets().into()
    }

    pub fn auxiliary_data_set(&self) -> MapTransactionIndexToAuxiliaryData {
        self.0.auxiliary_data_set().into()
    }

    pub fn invalid_transactions(&self) -> Vec<TransactionIndex> {
        self.0.invalid_transactions()
    }
}

impl_wasm_list!(
    cml_multi_era::utils::MultiEraCertificate,
    MultiEraCertificate,
    MultiEraCertificateList
);

impl_wasm_list!(
    cml_multi_era::MultiEraTransactionBody,
    MultiEraTransactionBody,
    MultiEraTransactionBodyList
);

impl_wasm_list!(
    cml_multi_era::utils::MultiEraTransactionInput,
    MultiEraTransactionInput,
    MultiEraTransactionInputList
);

impl_wasm_list!(
    cml_multi_era::utils::MultiEraTransactionOutput,
    MultiEraTransactionOutput,
    MultiEraTransactionOutputList
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiEraCertificate(cml_multi_era::utils::MultiEraCertificate);

impl_wasm_json_api!(MultiEraCertificate);

impl_wasm_conversions!(
    cml_multi_era::utils::MultiEraCertificate,
    MultiEraCertificate
);

#[wasm_bindgen]
impl MultiEraCertificate {
    pub fn kind(&self) -> MultiEraCertificateKind {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::StakeRegistration(_) => {
                MultiEraCertificateKind::StakeRegistration
            }
            cml_multi_era::utils::MultiEraCertificate::StakeDeregistration(_) => {
                MultiEraCertificateKind::StakeDeregistration
            }
            cml_multi_era::utils::MultiEraCertificate::StakeDelegation(_) => {
                MultiEraCertificateKind::StakeDelegation
            }
            cml_multi_era::utils::MultiEraCertificate::PoolRegistration(_) => {
                MultiEraCertificateKind::PoolRegistration
            }
            cml_multi_era::utils::MultiEraCertificate::PoolRetirement(_) => {
                MultiEraCertificateKind::PoolRetirement
            }
            cml_multi_era::utils::MultiEraCertificate::GenesisKeyDelegation(_) => {
                MultiEraCertificateKind::GenesisKeyDelegation
            }
            cml_multi_era::utils::MultiEraCertificate::MoveInstantaneousRewardsCert(_) => {
                MultiEraCertificateKind::MoveInstantaneousRewardsCert
            }
            cml_multi_era::utils::MultiEraCertificate::RegCert(_) => {
                MultiEraCertificateKind::RegCert
            }
            cml_multi_era::utils::MultiEraCertificate::UnregCert(_) => {
                MultiEraCertificateKind::UnregCert
            }
            cml_multi_era::utils::MultiEraCertificate::VoteDelegCert(_) => {
                MultiEraCertificateKind::VoteDelegCert
            }
            cml_multi_era::utils::MultiEraCertificate::StakeVoteDelegCert(_) => {
                MultiEraCertificateKind::StakeVoteDelegCert
            }
            cml_multi_era::utils::MultiEraCertificate::StakeRegDelegCert(_) => {
                MultiEraCertificateKind::StakeRegDelegCert
            }
            cml_multi_era::utils::MultiEraCertificate::VoteRegDelegCert(_) => {
                MultiEraCertificateKind::VoteRegDelegCert
            }
            cml_multi_era::utils::MultiEraCertificate::StakeVoteRegDelegCert(_) => {
                MultiEraCertificateKind::StakeVoteRegDelegCert
            }
            cml_multi_era::utils::MultiEraCertificate::AuthCommitteeHotCert(_) => {
                MultiEraCertificateKind::AuthCommitteeHotCert
            }
            cml_multi_era::utils::MultiEraCertificate::ResignCommitteeColdCert(_) => {
                MultiEraCertificateKind::ResignCommitteeColdCert
            }
            cml_multi_era::utils::MultiEraCertificate::RegDrepCert(_) => {
                MultiEraCertificateKind::RegDrepCert
            }
            cml_multi_era::utils::MultiEraCertificate::UnregDrepCert(_) => {
                MultiEraCertificateKind::UnregDrepCert
            }
            cml_multi_era::utils::MultiEraCertificate::UpdateDrepCert(_) => {
                MultiEraCertificateKind::UpdateDrepCert
            }
        }
    }

    pub fn as_stake_registration(&self) -> Option<StakeRegistration> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::StakeRegistration(stake_registration) => {
                Some(stake_registration.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_deregistration(&self) -> Option<StakeDeregistration> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::StakeDeregistration(
                stake_deregistration,
            ) => Some(stake_deregistration.clone().into()),
            _ => None,
        }
    }

    pub fn as_stake_delegation(&self) -> Option<StakeDelegation> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::StakeDelegation(stake_delegation) => {
                Some(stake_delegation.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_pool_registration(&self) -> Option<PoolRegistration> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::PoolRegistration(pool_registration) => {
                Some(pool_registration.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_pool_retirement(&self) -> Option<PoolRetirement> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::PoolRetirement(pool_retirement) => {
                Some(pool_retirement.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_genesis_key_delegation(&self) -> Option<GenesisKeyDelegation> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::GenesisKeyDelegation(
                genesis_key_delegation,
            ) => Some(genesis_key_delegation.clone().into()),
            _ => None,
        }
    }

    pub fn as_move_instantaneous_rewards_cert(&self) -> Option<MoveInstantaneousRewardsCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::MoveInstantaneousRewardsCert(
                move_instantaneous_rewards_cert,
            ) => Some(move_instantaneous_rewards_cert.clone().into()),
            _ => None,
        }
    }

    pub fn as_reg_cert(&self) -> Option<RegCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::RegCert(reg_cert) => {
                Some(reg_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_unreg_cert(&self) -> Option<UnregCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::UnregCert(unreg_cert) => {
                Some(unreg_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_vote_deleg_cert(&self) -> Option<VoteDelegCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::VoteDelegCert(vote_deleg_cert) => {
                Some(vote_deleg_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_vote_deleg_cert(&self) -> Option<StakeVoteDelegCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::StakeVoteDelegCert(
                stake_vote_deleg_cert,
            ) => Some(stake_vote_deleg_cert.clone().into()),
            _ => None,
        }
    }

    pub fn as_stake_reg_deleg_cert(&self) -> Option<StakeRegDelegCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::StakeRegDelegCert(stake_reg_deleg_cert) => {
                Some(stake_reg_deleg_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_vote_reg_deleg_cert(&self) -> Option<VoteRegDelegCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::VoteRegDelegCert(vote_reg_deleg_cert) => {
                Some(vote_reg_deleg_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_vote_reg_deleg_cert(&self) -> Option<StakeVoteRegDelegCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::StakeVoteRegDelegCert(
                stake_vote_reg_deleg_cert,
            ) => Some(stake_vote_reg_deleg_cert.clone().into()),
            _ => None,
        }
    }

    pub fn as_auth_committee_hot_cert(&self) -> Option<AuthCommitteeHotCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::AuthCommitteeHotCert(
                auth_committee_hot_cert,
            ) => Some(auth_committee_hot_cert.clone().into()),
            _ => None,
        }
    }

    pub fn as_resign_committee_cold_cert(&self) -> Option<ResignCommitteeColdCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::ResignCommitteeColdCert(
                resign_committee_cold_cert,
            ) => Some(resign_committee_cold_cert.clone().into()),
            _ => None,
        }
    }

    pub fn as_reg_drep_cert(&self) -> Option<RegDrepCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::RegDrepCert(reg_drep_cert) => {
                Some(reg_drep_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_unreg_drep_cert(&self) -> Option<UnregDrepCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::UnregDrepCert(unreg_drep_cert) => {
                Some(unreg_drep_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_update_drep_cert(&self) -> Option<UpdateDrepCert> {
        match &self.0 {
            cml_multi_era::utils::MultiEraCertificate::UpdateDrepCert(update_drep_cert) => {
                Some(update_drep_cert.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum MultiEraCertificateKind {
    StakeRegistration,
    StakeDeregistration,
    StakeDelegation,
    PoolRegistration,
    PoolRetirement,
    GenesisKeyDelegation,
    MoveInstantaneousRewardsCert,
    RegCert,
    UnregCert,
    VoteDelegCert,
    StakeVoteDelegCert,
    StakeRegDelegCert,
    VoteRegDelegCert,
    StakeVoteRegDelegCert,
    AuthCommitteeHotCert,
    ResignCommitteeColdCert,
    RegDrepCert,
    UnregDrepCert,
    UpdateDrepCert,
}

#[wasm_bindgen]
impl MultiEraTransactionBody {
    pub fn inputs(&self) -> MultiEraTransactionInputList {
        self.0.inputs().into()
    }

    pub fn outputs(&self) -> MultiEraTransactionOutputList {
        self.0.outputs().into()
    }

    pub fn fee(&self) -> Option<Coin> {
        self.0.fee()
    }

    pub fn ttl(&self) -> Option<u64> {
        self.0.ttl()
    }

    pub fn certs(&self) -> Option<MultiEraCertificateList> {
        self.0.certs().map(Into::into)
    }

    pub fn withdrawals(&self) -> Option<Withdrawals> {
        self.0.withdrawals().map(|wd| wd.clone().into())
    }

    pub fn auxiliary_data_hash(&self) -> Option<AuxiliaryDataHash> {
        self.0.auxiliary_data_hash().map(|aux| (*aux).into())
    }

    pub fn validity_interval_start(&self) -> Option<u64> {
        self.0.validity_interval_start()
    }

    pub fn mint(&self) -> Option<Mint> {
        self.0.mint().map(|m| m.clone().into())
    }

    pub fn script_data_hash(&self) -> Option<ScriptDataHash> {
        self.0.script_data_hash().map(Into::into)
    }

    pub fn collateral_inputs(&self) -> Option<TransactionInputList> {
        self.0
            .collateral_inputs()
            .map(|inputs| inputs.clone().into())
    }

    pub fn required_signers(&self) -> Option<RequiredSigners> {
        self.0
            .required_signers()
            .map(|signers| signers.clone().into())
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        self.0.network_id().map(Into::into)
    }

    pub fn collateral_return(&self) -> Option<MultiEraTransactionOutput> {
        self.0.collateral_return().map(Into::into)
    }

    pub fn total_collateral(&self) -> Option<Coin> {
        self.0.total_collateral()
    }

    pub fn reference_inputs(&self) -> Option<TransactionInputList> {
        self.0
            .reference_inputs()
            .map(|inputs| inputs.clone().into())
    }

    pub fn voting_procedures(&self) -> Option<VotingProcedures> {
        self.0.voting_procedures().map(|vps| vps.clone().into())
    }

    pub fn proposal_procedures(&self) -> Option<ProposalProcedureList> {
        self.0.proposal_procedures().map(|pps| pps.clone().into())
    }

    pub fn current_treasury_value(&self) -> Option<Coin> {
        self.0.current_treasury_value()
    }

    pub fn donation(&self) -> Option<PositiveCoin> {
        self.0.donation()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct MultiEraTransactionInput(cml_multi_era::utils::MultiEraTransactionInput);

impl_wasm_conversions!(
    cml_multi_era::utils::MultiEraTransactionInput,
    MultiEraTransactionInput
);

#[wasm_bindgen]
impl MultiEraTransactionInput {
    /// Transaction hash this input was created in
    /// Will return None only for Byron Genesis inputs
    pub fn hash(&self) -> Option<TransactionHash> {
        self.0.hash().map(|h| (*h).into())
    }

    /// Transaction index into the tx that this input was created in
    /// Will return None for only Byron Genesis inputs
    pub fn index(&self) -> Option<u64> {
        self.0.index()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct MultiEraTransactionOutput(cml_multi_era::utils::MultiEraTransactionOutput);

impl_wasm_conversions!(
    cml_multi_era::utils::MultiEraTransactionOutput,
    MultiEraTransactionOutput
);

#[wasm_bindgen]
impl MultiEraTransactionOutput {
    pub fn address(&self) -> Address {
        self.0.address().into()
    }

    pub fn amount(&self) -> Value {
        self.0.amount().into()
    }
}
