// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod block;
pub mod delegation;
pub mod mpc;
pub mod transaction;
pub mod update;
#[macro_use]
pub mod utils;

use ::wasm_bindgen::prelude::wasm_bindgen;
use cml_chain_wasm::byron::ByronTxOut;
use cml_chain_wasm::utils::BigInteger;
use cml_core_wasm::{
    impl_wasm_cbor_json_api_cbor_event_serialize, impl_wasm_conversions, impl_wasm_list,
    impl_wasm_map_btree,
};
use cml_crypto_wasm::TransactionHash;
pub use utils::{Blake2b224, Blake2b256, ByronAny};

pub type SystemTag = String;

pub type ByronBlockId = Blake2b256;

pub type ByronPubKey = Vec<u8>;

pub type ByronSignature = Vec<u8>;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronSlotId(cml_multi_era::byron::ByronSlotId);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronSlotId);

impl_wasm_conversions!(cml_multi_era::byron::ByronSlotId, ByronSlotId);

#[wasm_bindgen]
impl ByronSlotId {
    pub fn epoch(&self) -> EpochId {
        self.0.epoch
    }

    pub fn slot(&self) -> u64 {
        self.0.slot
    }

    pub fn new(epoch: EpochId, slot: u64) -> Self {
        Self(cml_multi_era::byron::ByronSlotId::new(epoch, slot))
    }
}

pub type ByronTxId = TransactionHash;

pub type ByronUpdateId = Blake2b256;

pub type EpochId = u64;

// this is pasted from lib.rs due to the export structure being at cml-multi-era-wasm root:

use cml_chain_wasm::byron::{AddressId, StakeholderId};

use crate::byron::{
    delegation::ByronDelegation,
    transaction::{ByronAttributes, ByronTxIn, ByronTxWitness},
    update::{
        ByronTxFeePolicy, ByronUpdateData, ByronUpdateProposal, ByronUpdateVote, SoftForkRule,
    },
};

impl_wasm_list!(cml_chain::byron::AddressId, AddressId, AddressIdList);

impl_wasm_list!(cml_chain::utils::BigInteger, BigInteger, BigIntegerList);

impl_wasm_list!(cml_multi_era::byron::ByronAny, ByronAny, ByronAnyList);

impl_wasm_list!(
    cml_multi_era::byron::transaction::ByronAttributes,
    ByronAttributes,
    ByronAttributesList
);

impl_wasm_list!(
    cml_multi_era::byron::delegation::ByronDelegation,
    ByronDelegation,
    ByronDelegationList
);

impl_wasm_list!(
    cml_multi_era::byron::update::ByronTxFeePolicy,
    ByronTxFeePolicy,
    ByronTxFeePolicyList
);

impl_wasm_list!(
    cml_multi_era::byron::transaction::ByronTxIn,
    ByronTxIn,
    ByronTxInList
);

impl_wasm_list!(cml_chain::byron::ByronTxOut, ByronTxOut, ByronTxOutList);

impl_wasm_list!(
    cml_multi_era::byron::transaction::ByronTxWitness,
    ByronTxWitness,
    ByronTxWitnessList
);

impl_wasm_list!(
    cml_multi_era::byron::update::ByronUpdateProposal,
    ByronUpdateProposal,
    ByronUpdateProposalList
);

impl_wasm_list!(
    cml_multi_era::byron::update::ByronUpdateVote,
    ByronUpdateVote,
    ByronUpdateVoteList
);

impl_wasm_list!(Vec<u8>, Vec<u8>, BytesList, true, false);

impl_wasm_map_btree!(
    cml_multi_era::byron::update::SystemTag,
    cml_multi_era::byron::update::ByronUpdateData,
    SystemTag,
    ByronUpdateData,
    SystemTagList,
    MapSystemTagToByronUpdateData,
    true,
    false,
    false,
    false
);

impl_wasm_list!(
    cml_multi_era::byron::update::SoftForkRule,
    SoftForkRule,
    SoftForkRuleList
);

impl_wasm_list!(
    cml_chain::byron::StakeholderId,
    StakeholderId,
    StakeholderIdList
);

impl_wasm_list!(String, String, SystemTagList, true, false);

pub type VssDecryptedShareList = BytesList;

pub type VssPubKeyList = BytesList;
