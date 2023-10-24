// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::shelley::{GenesisKeyDelegation, ShelleyHeader, ShelleyUpdate};
use crate::{
    AllegraCertificateList, AllegraTransactionBodyList, AllegraTransactionWitnessSetList,
    MapTransactionIndexToAllegraAuxiliaryData, ShelleyTransactionOutputList,
};
use cml_chain_wasm::assets::Coin;
use cml_chain_wasm::auxdata::{ShelleyFormatAuxData, ShelleyMaFormatAuxData};
use cml_chain_wasm::certs::{PoolParams, StakeCredential};
use cml_chain_wasm::certs::{
    PoolRegistration, PoolRetirement, StakeDelegation, StakeDeregistration, StakeRegistration,
};
use cml_chain_wasm::Epoch;
use cml_chain_wasm::{
    BootstrapWitnessList, NativeScriptList, TransactionInputList, VkeywitnessList,
};
use cml_chain_wasm::{MapStakeCredentialToDeltaCoin, Withdrawals};
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use cml_crypto_wasm::Ed25519KeyHash;
use cml_crypto_wasm::{AuxiliaryDataHash, GenesisDelegateHash, GenesisHash, VRFKeyHash};
use cml_multi_era::allegra::MIRPot;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraAuxiliaryData(cml_multi_era::allegra::AllegraAuxiliaryData);

impl_wasm_cbor_json_api!(AllegraAuxiliaryData);

impl_wasm_conversions!(
    cml_multi_era::allegra::AllegraAuxiliaryData,
    AllegraAuxiliaryData
);

#[wasm_bindgen]
impl AllegraAuxiliaryData {
    pub fn new_shelley(shelley: &ShelleyFormatAuxData) -> Self {
        Self(cml_multi_era::allegra::AllegraAuxiliaryData::new_shelley(
            shelley.clone().into(),
        ))
    }

    pub fn new_shelley_m_a(shelley_m_a: &ShelleyMaFormatAuxData) -> Self {
        Self(
            cml_multi_era::allegra::AllegraAuxiliaryData::new_shelley_m_a(
                shelley_m_a.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> AllegraAuxiliaryDataKind {
        match &self.0 {
            cml_multi_era::allegra::AllegraAuxiliaryData::Shelley(_) => {
                AllegraAuxiliaryDataKind::Shelley
            }
            cml_multi_era::allegra::AllegraAuxiliaryData::ShelleyMA(_) => {
                AllegraAuxiliaryDataKind::ShelleyMA
            }
        }
    }

    pub fn as_shelley(&self) -> Option<ShelleyFormatAuxData> {
        match &self.0 {
            cml_multi_era::allegra::AllegraAuxiliaryData::Shelley(shelley) => {
                Some(shelley.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_shelley_m_a(&self) -> Option<ShelleyMaFormatAuxData> {
        match &self.0 {
            cml_multi_era::allegra::AllegraAuxiliaryData::ShelleyMA(shelley_m_a) => {
                Some(shelley_m_a.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum AllegraAuxiliaryDataKind {
    Shelley,
    ShelleyMA,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraBlock(cml_multi_era::allegra::AllegraBlock);

impl_wasm_cbor_json_api!(AllegraBlock);

impl_wasm_conversions!(cml_multi_era::allegra::AllegraBlock, AllegraBlock);

#[wasm_bindgen]
impl AllegraBlock {
    pub fn header(&self) -> ShelleyHeader {
        self.0.header.clone().into()
    }

    pub fn transaction_bodies(&self) -> AllegraTransactionBodyList {
        self.0.transaction_bodies.clone().into()
    }

    pub fn transaction_witness_sets(&self) -> AllegraTransactionWitnessSetList {
        self.0.transaction_witness_sets.clone().into()
    }

    pub fn auxiliary_data_set(&self) -> MapTransactionIndexToAllegraAuxiliaryData {
        self.0.auxiliary_data_set.clone().into()
    }

    pub fn new(
        header: &ShelleyHeader,
        transaction_bodies: &AllegraTransactionBodyList,
        transaction_witness_sets: &AllegraTransactionWitnessSetList,
        auxiliary_data_set: &MapTransactionIndexToAllegraAuxiliaryData,
    ) -> Self {
        Self(cml_multi_era::allegra::AllegraBlock::new(
            header.clone().into(),
            transaction_bodies.clone().into(),
            transaction_witness_sets.clone().into(),
            auxiliary_data_set.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraCertificate(cml_multi_era::allegra::AllegraCertificate);

impl_wasm_cbor_json_api!(AllegraCertificate);

impl_wasm_conversions!(
    cml_multi_era::allegra::AllegraCertificate,
    AllegraCertificate
);

#[wasm_bindgen]
impl AllegraCertificate {
    pub fn new_stake_registration(stake_credential: &StakeCredential) -> Self {
        Self(
            cml_multi_era::allegra::AllegraCertificate::new_stake_registration(
                stake_credential.clone().into(),
            ),
        )
    }

    pub fn new_stake_deregistration(stake_credential: &StakeCredential) -> Self {
        Self(
            cml_multi_era::allegra::AllegraCertificate::new_stake_deregistration(
                stake_credential.clone().into(),
            ),
        )
    }

    pub fn new_stake_delegation(
        stake_credential: &StakeCredential,
        ed25519_key_hash: &Ed25519KeyHash,
    ) -> Self {
        Self(
            cml_multi_era::allegra::AllegraCertificate::new_stake_delegation(
                stake_credential.clone().into(),
                ed25519_key_hash.clone().into(),
            ),
        )
    }

    pub fn new_pool_registration(pool_params: &PoolParams) -> Self {
        Self(
            cml_multi_era::allegra::AllegraCertificate::new_pool_registration(
                pool_params.clone().into(),
            ),
        )
    }

    pub fn new_pool_retirement(ed25519_key_hash: &Ed25519KeyHash, epoch: Epoch) -> Self {
        Self(
            cml_multi_era::allegra::AllegraCertificate::new_pool_retirement(
                ed25519_key_hash.clone().into(),
                epoch,
            ),
        )
    }

    pub fn new_genesis_key_delegation(
        genesis_hash: &GenesisHash,
        genesis_delegate_hash: &GenesisDelegateHash,
        v_r_f_key_hash: &VRFKeyHash,
    ) -> Self {
        Self(
            cml_multi_era::allegra::AllegraCertificate::new_genesis_key_delegation(
                genesis_hash.clone().into(),
                genesis_delegate_hash.clone().into(),
                v_r_f_key_hash.clone().into(),
            ),
        )
    }

    pub fn new_move_instantaneous_rewards_cert(
        move_instantaneous_reward: &MoveInstantaneousReward,
    ) -> Self {
        Self(
            cml_multi_era::allegra::AllegraCertificate::new_move_instantaneous_rewards_cert(
                move_instantaneous_reward.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> AllegraCertificateKind {
        match &self.0 {
            cml_multi_era::allegra::AllegraCertificate::StakeRegistration(_) => {
                AllegraCertificateKind::StakeRegistration
            }
            cml_multi_era::allegra::AllegraCertificate::StakeDeregistration(_) => {
                AllegraCertificateKind::StakeDeregistration
            }
            cml_multi_era::allegra::AllegraCertificate::StakeDelegation(_) => {
                AllegraCertificateKind::StakeDelegation
            }
            cml_multi_era::allegra::AllegraCertificate::PoolRegistration(_) => {
                AllegraCertificateKind::PoolRegistration
            }
            cml_multi_era::allegra::AllegraCertificate::PoolRetirement(_) => {
                AllegraCertificateKind::PoolRetirement
            }
            cml_multi_era::allegra::AllegraCertificate::GenesisKeyDelegation(_) => {
                AllegraCertificateKind::GenesisKeyDelegation
            }
            cml_multi_era::allegra::AllegraCertificate::MoveInstantaneousRewardsCert(_) => {
                AllegraCertificateKind::MoveInstantaneousRewardsCert
            }
        }
    }

    pub fn as_stake_registration(&self) -> Option<StakeRegistration> {
        match &self.0 {
            cml_multi_era::allegra::AllegraCertificate::StakeRegistration(stake_registration) => {
                Some(stake_registration.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_deregistration(&self) -> Option<StakeDeregistration> {
        match &self.0 {
            cml_multi_era::allegra::AllegraCertificate::StakeDeregistration(
                stake_deregistration,
            ) => Some(stake_deregistration.clone().into()),
            _ => None,
        }
    }

    pub fn as_stake_delegation(&self) -> Option<StakeDelegation> {
        match &self.0 {
            cml_multi_era::allegra::AllegraCertificate::StakeDelegation(stake_delegation) => {
                Some(stake_delegation.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_pool_registration(&self) -> Option<PoolRegistration> {
        match &self.0 {
            cml_multi_era::allegra::AllegraCertificate::PoolRegistration(pool_registration) => {
                Some(pool_registration.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_pool_retirement(&self) -> Option<PoolRetirement> {
        match &self.0 {
            cml_multi_era::allegra::AllegraCertificate::PoolRetirement(pool_retirement) => {
                Some(pool_retirement.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_genesis_key_delegation(&self) -> Option<GenesisKeyDelegation> {
        match &self.0 {
            cml_multi_era::allegra::AllegraCertificate::GenesisKeyDelegation(
                genesis_key_delegation,
            ) => Some(genesis_key_delegation.clone().into()),
            _ => None,
        }
    }

    pub fn as_move_instantaneous_rewards_cert(&self) -> Option<MoveInstantaneousRewardsCert> {
        match &self.0 {
            cml_multi_era::allegra::AllegraCertificate::MoveInstantaneousRewardsCert(
                move_instantaneous_rewards_cert,
            ) => Some(move_instantaneous_rewards_cert.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum AllegraCertificateKind {
    StakeRegistration,
    StakeDeregistration,
    StakeDelegation,
    PoolRegistration,
    PoolRetirement,
    GenesisKeyDelegation,
    MoveInstantaneousRewardsCert,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraTransaction(cml_multi_era::allegra::AllegraTransaction);

impl_wasm_cbor_json_api!(AllegraTransaction);

impl_wasm_conversions!(
    cml_multi_era::allegra::AllegraTransaction,
    AllegraTransaction
);

#[wasm_bindgen]
impl AllegraTransaction {
    pub fn body(&self) -> AllegraTransactionBody {
        self.0.body.clone().into()
    }

    pub fn witness_set(&self) -> AllegraTransactionWitnessSet {
        self.0.witness_set.clone().into()
    }

    pub fn auxiliary_data(&self) -> Option<AllegraAuxiliaryData> {
        self.0.auxiliary_data.clone().map(std::convert::Into::into)
    }

    pub fn new(
        body: &AllegraTransactionBody,
        witness_set: &AllegraTransactionWitnessSet,
        auxiliary_data: Option<AllegraAuxiliaryData>,
    ) -> Self {
        Self(cml_multi_era::allegra::AllegraTransaction::new(
            body.clone().into(),
            witness_set.clone().into(),
            auxiliary_data.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraTransactionBody(cml_multi_era::allegra::AllegraTransactionBody);

impl_wasm_cbor_json_api!(AllegraTransactionBody);

impl_wasm_conversions!(
    cml_multi_era::allegra::AllegraTransactionBody,
    AllegraTransactionBody
);

#[wasm_bindgen]
impl AllegraTransactionBody {
    pub fn inputs(&self) -> TransactionInputList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> ShelleyTransactionOutputList {
        self.0.outputs.clone().into()
    }

    pub fn fee(&self) -> Coin {
        self.0.fee
    }

    pub fn set_ttl(&mut self, ttl: u64) {
        self.0.ttl = Some(ttl)
    }

    pub fn ttl(&self) -> Option<u64> {
        self.0.ttl
    }

    pub fn set_certs(&mut self, certs: &AllegraCertificateList) {
        self.0.certs = Some(certs.clone().into())
    }

    pub fn certs(&self) -> Option<AllegraCertificateList> {
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
        self.0.auxiliary_data_hash.map(std::convert::Into::into)
    }

    pub fn set_validity_interval_start(&mut self, validity_interval_start: u64) {
        self.0.validity_interval_start = Some(validity_interval_start)
    }

    pub fn validity_interval_start(&self) -> Option<u64> {
        self.0.validity_interval_start
    }

    pub fn new(
        inputs: &TransactionInputList,
        outputs: &ShelleyTransactionOutputList,
        fee: Coin,
    ) -> Self {
        Self(cml_multi_era::allegra::AllegraTransactionBody::new(
            inputs.clone().into(),
            outputs.clone().into(),
            fee,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraTransactionWitnessSet(cml_multi_era::allegra::AllegraTransactionWitnessSet);

impl_wasm_cbor_json_api!(AllegraTransactionWitnessSet);

impl_wasm_conversions!(
    cml_multi_era::allegra::AllegraTransactionWitnessSet,
    AllegraTransactionWitnessSet
);

#[wasm_bindgen]
impl AllegraTransactionWitnessSet {
    pub fn set_vkeywitnesses(&mut self, vkeywitnesses: &VkeywitnessList) {
        self.0.vkeywitnesses = Some(vkeywitnesses.clone().into())
    }

    pub fn vkeywitnesses(&self) -> Option<VkeywitnessList> {
        self.0.vkeywitnesses.clone().map(std::convert::Into::into)
    }

    pub fn set_native_scripts(&mut self, native_scripts: &NativeScriptList) {
        self.0.native_scripts = Some(native_scripts.clone().into())
    }

    pub fn native_scripts(&self) -> Option<NativeScriptList> {
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
        Self(cml_multi_era::allegra::AllegraTransactionWitnessSet::new())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MIRAction(cml_multi_era::allegra::MIRAction);

impl_wasm_cbor_json_api!(MIRAction);

impl_wasm_conversions!(cml_multi_era::allegra::MIRAction, MIRAction);

#[wasm_bindgen]
impl MIRAction {
    pub fn new_to_stake_credentials(to_stake_credentials: &MapStakeCredentialToDeltaCoin) -> Self {
        Self(cml_multi_era::allegra::MIRAction::new_to_stake_credentials(
            to_stake_credentials.clone().into(),
        ))
    }

    pub fn new_to_other_pot(to_other_pot: Coin) -> Self {
        Self(cml_multi_era::allegra::MIRAction::new_to_other_pot(
            to_other_pot,
        ))
    }

    pub fn kind(&self) -> MIRActionKind {
        match &self.0 {
            cml_multi_era::allegra::MIRAction::ToStakeCredentials { .. } => {
                MIRActionKind::ToStakeCredentials
            }
            cml_multi_era::allegra::MIRAction::ToOtherPot { .. } => MIRActionKind::ToOtherPot,
        }
    }

    pub fn as_to_stake_credentials(&self) -> Option<MapStakeCredentialToDeltaCoin> {
        match &self.0 {
            cml_multi_era::allegra::MIRAction::ToStakeCredentials {
                to_stake_credentials,
                ..
            } => Some(to_stake_credentials.clone().into()),
            _ => None,
        }
    }

    pub fn as_to_other_pot(&self) -> Option<Coin> {
        match &self.0 {
            cml_multi_era::allegra::MIRAction::ToOtherPot { to_other_pot, .. } => {
                Some(*to_other_pot)
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum MIRActionKind {
    ToStakeCredentials,
    ToOtherPot,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MoveInstantaneousReward(cml_multi_era::allegra::MoveInstantaneousReward);

impl_wasm_cbor_json_api!(MoveInstantaneousReward);

impl_wasm_conversions!(
    cml_multi_era::allegra::MoveInstantaneousReward,
    MoveInstantaneousReward
);

#[wasm_bindgen]
impl MoveInstantaneousReward {
    pub fn pot(&self) -> MIRPot {
        self.0.pot
    }

    pub fn action(&self) -> MIRAction {
        self.0.action.clone().into()
    }

    pub fn new(pot: MIRPot, action: &MIRAction) -> Self {
        Self(cml_multi_era::allegra::MoveInstantaneousReward::new(
            pot,
            action.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MoveInstantaneousRewardsCert(cml_multi_era::allegra::MoveInstantaneousRewardsCert);

impl_wasm_cbor_json_api!(MoveInstantaneousRewardsCert);

impl_wasm_conversions!(
    cml_multi_era::allegra::MoveInstantaneousRewardsCert,
    MoveInstantaneousRewardsCert
);

#[wasm_bindgen]
impl MoveInstantaneousRewardsCert {
    pub fn move_instantaneous_reward(&self) -> MoveInstantaneousReward {
        self.0.move_instantaneous_reward.clone().into()
    }

    pub fn new(move_instantaneous_reward: &MoveInstantaneousReward) -> Self {
        Self(cml_multi_era::allegra::MoveInstantaneousRewardsCert::new(
            move_instantaneous_reward.clone().into(),
        ))
    }
}
