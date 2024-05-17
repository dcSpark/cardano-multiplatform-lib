use cml_chain::{assets::PositiveCoin, Coin};
use cml_chain_wasm::{
    address::Address,
    assets::{Mint, Value},
    block::{OperationalCert, ProtocolVersion},
    certs::{
        AuthCommitteeHotCert, PoolRegistration, PoolRetirement, RegCert, RegDrepCert,
        ResignCommitteeColdCert, StakeDelegation, StakeDeregistration, StakeRegDelegCert,
        StakeRegistration, StakeVoteDelegCert, StakeVoteRegDelegCert, UnregCert, UnregDrepCert,
        UpdateDrepCert, VoteDelegCert, VoteRegDelegCert,
    },
    crypto::{GenesisHash, Nonce, VRFCert, Vkey},
    governance::VotingProcedures,
    plutus::{CostModels, ExUnitPrices, ExUnits},
    DRepVotingThresholds, MapTransactionIndexToAuxiliaryData, NetworkId, PoolVotingThresholds,
    ProposalProcedureList, Rational, RequiredSigners, TransactionInputList,
    TransactionWitnessSetList, UnitInterval, Withdrawals,
};
use cml_core::{Epoch, TransactionIndex};
use cml_core_wasm::{impl_wasm_conversions, impl_wasm_json_api, impl_wasm_list, impl_wasm_map};
use cml_crypto_wasm::{
    AuxiliaryDataHash, BlockBodyHash, BlockHeaderHash, ScriptDataHash, TransactionHash, VRFVkey,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{
    allegra::MoveInstantaneousRewardsCert, shelley::GenesisKeyDelegation,
    shelley::ProtocolVersionStruct, GenesisHashList, MultiEraBlock, MultiEraTransactionBody,
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
pub struct MultiEraBlockHeader(cml_multi_era::utils::MultiEraBlockHeader);

impl_wasm_json_api!(MultiEraBlockHeader);

impl_wasm_conversions!(
    cml_multi_era::utils::MultiEraBlockHeader,
    MultiEraBlockHeader
);

#[wasm_bindgen]
impl MultiEraBlockHeader {
    pub fn block_number(&self) -> u64 {
        self.0.block_number()
    }

    pub fn slot(&self) -> u64 {
        self.0.slot()
    }

    pub fn prev_hash(&self) -> Option<BlockHeaderHash> {
        self.0.prev_hash().map(Into::into)
    }

    pub fn issuer_vkey(&self) -> Option<Vkey> {
        self.0.issuer_vkey().map(|vkey| vkey.clone().into())
    }

    pub fn vrf_vkey(&self) -> Option<VRFVkey> {
        self.0.vrf_vkey().map(|vkey| (*vkey).into())
    }

    pub fn nonce_vrf(&self) -> Option<VRFCert> {
        self.0.nonce_vrf().map(|vrf| vrf.clone().into())
    }

    pub fn leader_vrf(&self) -> Option<VRFCert> {
        self.0.leader_vrf().map(|vrf| vrf.clone().into())
    }

    pub fn vrf_result(&self) -> Option<VRFCert> {
        self.0.vrf_result().map(|res| res.clone().into())
    }

    pub fn block_body_size(&self) -> Option<u64> {
        self.0.block_body_size()
    }

    pub fn block_body_hash(&self) -> Option<BlockBodyHash> {
        self.0.block_body_hash().map(Into::into)
    }

    pub fn operational_cert(&self) -> Option<OperationalCert> {
        self.0.operational_cert().map(|cert| cert.clone().into())
    }

    pub fn protocol_version(&self) -> Option<ProtocolVersion> {
        self.0.protocol_version().map(|ver| ver.clone().into())
    }
}

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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiEraProtocolParamUpdate(cml_multi_era::utils::MultiEraProtocolParamUpdate);

impl_wasm_json_api!(MultiEraProtocolParamUpdate);

impl_wasm_conversions!(
    cml_multi_era::utils::MultiEraProtocolParamUpdate,
    MultiEraProtocolParamUpdate
);

#[wasm_bindgen]
impl MultiEraProtocolParamUpdate {
    pub fn minfee_a(&self) -> Option<u64> {
        self.0.minfee_a()
    }

    pub fn minfee_b(&self) -> Option<u64> {
        self.0.minfee_b()
    }

    pub fn max_block_body_size(&self) -> Option<u64> {
        self.0.max_block_body_size()
    }

    pub fn max_transaction_size(&self) -> Option<u64> {
        self.0.max_transaction_size()
    }

    pub fn max_block_header_size(&self) -> Option<u64> {
        self.0.max_block_header_size()
    }

    pub fn key_deposit(&self) -> Option<Coin> {
        self.0.key_deposit()
    }

    pub fn pool_deposit(&self) -> Option<Coin> {
        self.0.pool_deposit()
    }

    pub fn maximum_epoch(&self) -> Option<Epoch> {
        self.0.maximum_epoch()
    }

    pub fn n_opt(&self) -> Option<u64> {
        self.0.n_opt()
    }

    pub fn pool_pledge_influence(&self) -> Option<Rational> {
        self.0.pool_pledge_influence().map(|ppi| ppi.clone().into())
    }

    pub fn expansion_rate(&self) -> Option<UnitInterval> {
        self.0.expansion_rate().map(|er| er.clone().into())
    }

    pub fn treasury_growth_rate(&self) -> Option<UnitInterval> {
        self.0.treasury_growth_rate().map(|tgr| tgr.clone().into())
    }

    pub fn decentralization_constant(&self) -> Option<UnitInterval> {
        self.0
            .decentralization_constant()
            .map(|dc| dc.clone().into())
    }

    pub fn extra_entropy(&self) -> Option<Nonce> {
        self.0.extra_entropy().map(|ee| ee.clone().into())
    }

    pub fn protocol_version(&self) -> Option<ProtocolVersionStruct> {
        self.0.protocol_version().map(|pv| pv.clone().into())
    }

    pub fn min_utxo_value(&self) -> Option<Coin> {
        self.0.min_utxo_value()
    }

    pub fn min_pool_cost(&self) -> Option<Coin> {
        self.0.min_pool_cost()
    }

    pub fn ada_per_utxo_byte(&self) -> Option<Coin> {
        self.0.ada_per_utxo_byte()
    }

    pub fn cost_models_for_script_languages(&self) -> Option<CostModels> {
        self.0.cost_models_for_script_languages().map(Into::into)
    }

    pub fn execution_costs(&self) -> Option<ExUnitPrices> {
        self.0.execution_costs().map(|ec| ec.clone().into())
    }

    pub fn max_tx_ex_units(&self) -> Option<ExUnits> {
        self.0.max_tx_ex_units().map(|mteu| mteu.clone().into())
    }

    pub fn max_block_ex_units(&self) -> Option<ExUnits> {
        self.0.max_block_ex_units().map(|mbeu| mbeu.clone().into())
    }

    pub fn max_value_size(&self) -> Option<u64> {
        self.0.max_value_size()
    }

    pub fn collateral_percentage(&self) -> Option<u64> {
        self.0.collateral_percentage()
    }

    pub fn max_collateral_inputs(&self) -> Option<u64> {
        self.0.max_collateral_inputs()
    }

    pub fn pool_voting_thresholds(&self) -> Option<PoolVotingThresholds> {
        self.0
            .pool_voting_thresholds()
            .map(|pvt| pvt.clone().into())
    }

    pub fn d_rep_voting_thresholds(&self) -> Option<DRepVotingThresholds> {
        self.0
            .d_rep_voting_thresholds()
            .map(|drvt| drvt.clone().into())
    }

    pub fn min_committee_size(&self) -> Option<u64> {
        self.0.min_committee_size()
    }

    pub fn committee_term_limit(&self) -> Option<u64> {
        self.0.committee_term_limit()
    }

    pub fn governance_action_validity_period(&self) -> Option<Epoch> {
        self.0.governance_action_validity_period()
    }

    pub fn governance_action_deposit(&self) -> Option<Coin> {
        self.0.governance_action_deposit()
    }

    pub fn d_rep_deposit(&self) -> Option<Coin> {
        self.0.d_rep_deposit()
    }

    pub fn d_rep_inactivity_period(&self) -> Option<Epoch> {
        self.0.d_rep_inactivity_period()
    }
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

    pub fn update(&self) -> Option<MultiEraUpdate> {
        self.0.update().map(Into::into)
    }

    pub fn auxiliary_data_hash(&self) -> Option<AuxiliaryDataHash> {
        self.0.auxiliary_data_hash().map(|aux| (*aux).into())
    }

    pub fn validity_interval_start(&self) -> Option<u64> {
        self.0.validity_interval_start()
    }

    pub fn mint(&self) -> Option<Mint> {
        self.0.mint().map(|m| m.into_owned().into())
    }

    pub fn script_data_hash(&self) -> Option<ScriptDataHash> {
        self.0.script_data_hash().map(Into::into)
    }

    pub fn collateral_inputs(&self) -> Option<TransactionInputList> {
        self.0
            .collateral_inputs()
            .map(|inputs| inputs.to_vec().into())
    }

    pub fn required_signers(&self) -> Option<RequiredSigners> {
        self.0
            .required_signers()
            .map(|signers| signers.to_vec().into())
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
            .map(|inputs| inputs.to_vec().into())
    }

    pub fn voting_procedures(&self) -> Option<VotingProcedures> {
        self.0.voting_procedures().map(|vps| vps.clone().into())
    }

    pub fn proposal_procedures(&self) -> Option<ProposalProcedureList> {
        self.0.proposal_procedures().map(|pps| pps.to_vec().into())
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

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct MultiEraUpdate(cml_multi_era::utils::MultiEraUpdate);

impl_wasm_conversions!(cml_multi_era::utils::MultiEraUpdate, MultiEraUpdate);

#[wasm_bindgen]
impl MultiEraUpdate {
    pub fn epoch(&self) -> u64 {
        self.0.epoch
    }

    pub fn proposed_protocol_parameter_updates(
        &self,
    ) -> MapGenesisHashToMultiEraProtocolParamUpdate {
        self.0.proposed_protocol_parameter_updates.clone().into()
    }
}

impl_wasm_map!(
    cml_crypto::GenesisHash,
    cml_multi_era::utils::MultiEraProtocolParamUpdate,
    GenesisHash,
    MultiEraProtocolParamUpdate,
    GenesisHashList,
    MapGenesisHashToMultiEraProtocolParamUpdate
);
