// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

pub type MapRewardAccountToCoin = cml_chain_wasm::Withdrawals;
pub type MapPolicyIdToMapAssetNameToI64 = cml_chain_wasm::assets::Mint;

pub mod allegra;
pub mod alonzo;
pub mod babbage;
mod borrowed_collections;
pub mod collections;
pub mod mary;
pub mod shelley;
pub use crate::Block;
pub use crate::ByronBlock;
pub use crate::ByronTx;

use allegra::{
    AllegraAuxiliaryData, AllegraBlock, AllegraCertificate, AllegraTransactionBody,
    AllegraTransactionWitnessSet,
};
use alonzo::{
    AlonzoAuxiliaryData, AlonzoBlock, AlonzoRedeemer, AlonzoTransactionBody,
    AlonzoTransactionWitnessSet,
};
use babbage::{
    BabbageAuxiliaryData, BabbageBlock, BabbageTransactionBody, BabbageTransactionOutput,
    BabbageTransactionWitnessSet,
};
use cml_chain_wasm::address::RewardAccount;
use cml_chain_wasm::assets::Coin;
use cml_chain_wasm::collections::{MapAssetNameToI64, PolicyIdList, RewardAccountList};
use cml_chain_wasm::crypto::GenesisHash;
use cml_chain_wasm::transaction::TransactionBody;
use cml_chain_wasm::{PolicyId, TransactionIndex};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions, impl_wasm_list_needs_into};
use mary::{MaryBlock, MaryTransactionBody, MaryTransactionOutput};
use shelley::{
    MultisigScript, ShelleyBlock, ShelleyCertificate, ShelleyRelay, ShelleyTransactionBody,
    ShelleyTransactionOutput, ShelleyTransactionWitnessSet,
};
use wasm_bindgen::prelude::{JsError, wasm_bindgen};

impl_wasm_list_needs_into!(
    cml_multi_era::allegra::AllegraCertificate,
    AllegraCertificate,
    AllegraCertificateList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::allegra::AllegraTransactionBody,
    AllegraTransactionBody,
    AllegraTransactionBodyList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::allegra::AllegraTransactionWitnessSet,
    AllegraTransactionWitnessSet,
    AllegraTransactionWitnessSetList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::alonzo::AlonzoRedeemer,
    AlonzoRedeemer,
    AlonzoRedeemerList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::alonzo::AlonzoTransactionBody,
    AlonzoTransactionBody,
    AlonzoTransactionBodyList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::alonzo::AlonzoTransactionWitnessSet,
    AlonzoTransactionWitnessSet,
    AlonzoTransactionWitnessSetList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::babbage::BabbageTransactionBody,
    BabbageTransactionBody,
    BabbageTransactionBodyList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::babbage::BabbageTransactionOutput,
    BabbageTransactionOutput,
    BabbageTransactionOutputList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::babbage::BabbageTransactionWitnessSet,
    BabbageTransactionWitnessSet,
    BabbageTransactionWitnessSetList,
    true,
    false
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
// rustfmt::skip: rustfmt breaks after the field vis leaving trailing whitespace and errors
// (rust-lang/rustfmt#5703, fix PR #5708 unmerged; present through 1.9.0-nightly 2026-07-17).
#[rustfmt::skip]
pub struct MapTransactionIndexToAllegraAuxiliaryData(
    pub(crate) OrderedHashMap<
        cml_chain::TransactionIndex,
        cml_multi_era::allegra::AllegraAuxiliaryData,
    >,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::TransactionIndex, cml_multi_era::allegra::AllegraAuxiliaryData>, MapTransactionIndexToAllegraAuxiliaryData);

#[wasm_bindgen]
impl MapTransactionIndexToAllegraAuxiliaryData {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: TransactionIndex,
        value: &AllegraAuxiliaryData,
    ) -> Option<AllegraAuxiliaryData> {
        self.0.insert(key, value.clone().into()).map(Into::into)
    }

    pub fn get(&self, key: TransactionIndex) -> Option<AllegraAuxiliaryData> {
        self.0.get(&key).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> Vec<TransactionIndex> {
        self.0.keys().copied().collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
// rustfmt::skip: rustfmt breaks after the field vis leaving trailing whitespace and errors
// (rust-lang/rustfmt#5703, fix PR #5708 unmerged; present through 1.9.0-nightly 2026-07-17).
#[rustfmt::skip]
pub struct MapTransactionIndexToAlonzoAuxiliaryData(
    pub(crate) OrderedHashMap<
        cml_chain::TransactionIndex,
        cml_multi_era::alonzo::AlonzoAuxiliaryData,
    >,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::TransactionIndex, cml_multi_era::alonzo::AlonzoAuxiliaryData>, MapTransactionIndexToAlonzoAuxiliaryData);

#[wasm_bindgen]
impl MapTransactionIndexToAlonzoAuxiliaryData {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: TransactionIndex,
        value: &AlonzoAuxiliaryData,
    ) -> Option<AlonzoAuxiliaryData> {
        self.0.insert(key, value.clone().into()).map(Into::into)
    }

    pub fn get(&self, key: TransactionIndex) -> Option<AlonzoAuxiliaryData> {
        self.0.get(&key).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> Vec<TransactionIndex> {
        self.0.keys().copied().collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
// rustfmt::skip: rustfmt breaks after the field vis leaving trailing whitespace and errors
// (rust-lang/rustfmt#5703, fix PR #5708 unmerged; present through 1.9.0-nightly 2026-07-17).
#[rustfmt::skip]
pub struct MapTransactionIndexToBabbageAuxiliaryData(
    pub(crate) OrderedHashMap<
        cml_chain::TransactionIndex,
        cml_multi_era::babbage::BabbageAuxiliaryData,
    >,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::TransactionIndex, cml_multi_era::babbage::BabbageAuxiliaryData>, MapTransactionIndexToBabbageAuxiliaryData);

#[wasm_bindgen]
impl MapTransactionIndexToBabbageAuxiliaryData {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: TransactionIndex,
        value: &BabbageAuxiliaryData,
    ) -> Option<BabbageAuxiliaryData> {
        self.0.insert(key, value.clone().into()).map(Into::into)
    }

    pub fn get(&self, key: TransactionIndex) -> Option<BabbageAuxiliaryData> {
        self.0.get(&key).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> Vec<TransactionIndex> {
        self.0.keys().copied().collect::<Vec<_>>()
    }
}

impl_wasm_list_needs_into!(
    cml_multi_era::mary::MaryTransactionBody,
    MaryTransactionBody,
    MaryTransactionBodyList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::mary::MaryTransactionOutput,
    MaryTransactionOutput,
    MaryTransactionOutputList,
    true,
    false
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiEraBlock(pub(crate) cml_multi_era::MultiEraBlock);

impl_wasm_cbor_json_api!(MultiEraBlock);

impl_wasm_conversions!(cml_multi_era::MultiEraBlock, MultiEraBlock);

#[wasm_bindgen]
impl MultiEraBlock {
    pub fn new_byron(byron: &ByronBlock) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_byron(
            byron.clone().into(),
        ))
    }

    pub fn new_shelley(shelley: &ShelleyBlock) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_shelley(
            shelley.clone().into(),
        ))
    }

    pub fn new_allegra(allegra: &AllegraBlock) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_allegra(
            allegra.clone().into(),
        ))
    }

    pub fn new_mary(mary: &MaryBlock) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_mary(mary.clone().into()))
    }

    pub fn new_alonzo(alonzo: &AlonzoBlock) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_alonzo(
            alonzo.clone().into(),
        ))
    }

    pub fn new_babbage(babbage: &BabbageBlock) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_babbage(
            babbage.clone().into(),
        ))
    }

    pub fn new_conway(conway: &Block) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_conway(
            conway.clone().into(),
        ))
    }

    pub fn kind(&self) -> MultiEraBlockKind {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Byron(_) => MultiEraBlockKind::Byron,
            cml_multi_era::MultiEraBlock::Shelley(_) => MultiEraBlockKind::Shelley,
            cml_multi_era::MultiEraBlock::Allegra(_) => MultiEraBlockKind::Allegra,
            cml_multi_era::MultiEraBlock::Mary(_) => MultiEraBlockKind::Mary,
            cml_multi_era::MultiEraBlock::Alonzo(_) => MultiEraBlockKind::Alonzo,
            cml_multi_era::MultiEraBlock::Babbage(_) => MultiEraBlockKind::Babbage,
            cml_multi_era::MultiEraBlock::Conway(_) => MultiEraBlockKind::Conway,
        }
    }

    pub fn as_byron(&self) -> Option<ByronBlock> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Byron(byron) => Some(byron.clone().into()),
            _ => None,
        }
    }

    pub fn as_shelley(&self) -> Option<ShelleyBlock> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Shelley(shelley) => Some(shelley.clone().into()),
            _ => None,
        }
    }

    pub fn as_allegra(&self) -> Option<AllegraBlock> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Allegra(allegra) => Some(allegra.clone().into()),
            _ => None,
        }
    }

    pub fn as_mary(&self) -> Option<MaryBlock> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Mary(mary) => Some(mary.clone().into()),
            _ => None,
        }
    }

    pub fn as_alonzo(&self) -> Option<AlonzoBlock> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Alonzo(alonzo) => Some(alonzo.clone().into()),
            _ => None,
        }
    }

    pub fn as_babbage(&self) -> Option<BabbageBlock> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Babbage(babbage) => Some(babbage.clone().into()),
            _ => None,
        }
    }

    pub fn as_conway(&self) -> Option<Block> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Conway(conway) => Some(conway.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum MultiEraBlockKind {
    Byron,
    Shelley,
    Allegra,
    Mary,
    Alonzo,
    Babbage,
    Conway,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiEraTransactionBody(pub(crate) cml_multi_era::MultiEraTransactionBody);

impl_wasm_cbor_json_api!(MultiEraTransactionBody);

impl_wasm_conversions!(
    cml_multi_era::MultiEraTransactionBody,
    MultiEraTransactionBody
);

#[wasm_bindgen]
impl MultiEraTransactionBody {
    pub fn new_byron(byron: &ByronTx) -> Self {
        Self(cml_multi_era::MultiEraTransactionBody::new_byron(
            byron.clone().into(),
        ))
    }

    pub fn new_shelley(shelley: &ShelleyTransactionBody) -> Self {
        Self(cml_multi_era::MultiEraTransactionBody::new_shelley(
            shelley.clone().into(),
        ))
    }

    pub fn new_allegra(allegra: &AllegraTransactionBody) -> Self {
        Self(cml_multi_era::MultiEraTransactionBody::new_allegra(
            allegra.clone().into(),
        ))
    }

    pub fn new_mary(mary: &MaryTransactionBody) -> Self {
        Self(cml_multi_era::MultiEraTransactionBody::new_mary(
            mary.clone().into(),
        ))
    }

    pub fn new_alonzo(alonzo: &AlonzoTransactionBody) -> Self {
        Self(cml_multi_era::MultiEraTransactionBody::new_alonzo(
            alonzo.clone().into(),
        ))
    }

    pub fn new_babbage(babbage: &BabbageTransactionBody) -> Self {
        Self(cml_multi_era::MultiEraTransactionBody::new_babbage(
            babbage.clone().into(),
        ))
    }

    pub fn new_conway(conway: &TransactionBody) -> Self {
        Self(cml_multi_era::MultiEraTransactionBody::new_conway(
            conway.clone().into(),
        ))
    }

    pub fn kind(&self) -> MultiEraTransactionBodyKind {
        match &self.0 {
            cml_multi_era::MultiEraTransactionBody::Byron(_) => MultiEraTransactionBodyKind::Byron,
            cml_multi_era::MultiEraTransactionBody::Shelley(_) => {
                MultiEraTransactionBodyKind::Shelley
            }
            cml_multi_era::MultiEraTransactionBody::Allegra(_) => {
                MultiEraTransactionBodyKind::Allegra
            }
            cml_multi_era::MultiEraTransactionBody::Mary(_) => MultiEraTransactionBodyKind::Mary,
            cml_multi_era::MultiEraTransactionBody::Alonzo(_) => {
                MultiEraTransactionBodyKind::Alonzo
            }
            cml_multi_era::MultiEraTransactionBody::Babbage(_) => {
                MultiEraTransactionBodyKind::Babbage
            }
            cml_multi_era::MultiEraTransactionBody::Conway(_) => {
                MultiEraTransactionBodyKind::Conway
            }
        }
    }

    pub fn as_byron(&self) -> Option<ByronTx> {
        match &self.0 {
            cml_multi_era::MultiEraTransactionBody::Byron(byron) => Some(byron.clone().into()),
            _ => None,
        }
    }

    pub fn as_shelley(&self) -> Option<ShelleyTransactionBody> {
        match &self.0 {
            cml_multi_era::MultiEraTransactionBody::Shelley(shelley) => {
                Some(shelley.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_allegra(&self) -> Option<AllegraTransactionBody> {
        match &self.0 {
            cml_multi_era::MultiEraTransactionBody::Allegra(allegra) => {
                Some(allegra.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_mary(&self) -> Option<MaryTransactionBody> {
        match &self.0 {
            cml_multi_era::MultiEraTransactionBody::Mary(mary) => Some(mary.clone().into()),
            _ => None,
        }
    }

    pub fn as_alonzo(&self) -> Option<AlonzoTransactionBody> {
        match &self.0 {
            cml_multi_era::MultiEraTransactionBody::Alonzo(alonzo) => Some(alonzo.clone().into()),
            _ => None,
        }
    }

    pub fn as_babbage(&self) -> Option<BabbageTransactionBody> {
        match &self.0 {
            cml_multi_era::MultiEraTransactionBody::Babbage(babbage) => {
                Some(babbage.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_conway(&self) -> Option<TransactionBody> {
        match &self.0 {
            cml_multi_era::MultiEraTransactionBody::Conway(conway) => Some(conway.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum MultiEraTransactionBodyKind {
    Byron,
    Shelley,
    Allegra,
    Mary,
    Alonzo,
    Babbage,
    Conway,
}

impl_wasm_list_needs_into!(
    cml_multi_era::shelley::MultisigScript,
    MultisigScript,
    MultisigScriptList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::shelley::ShelleyCertificate,
    ShelleyCertificate,
    ShelleyCertificateList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::shelley::ShelleyRelay,
    ShelleyRelay,
    ShelleyRelayList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::shelley::ShelleyTransactionBody,
    ShelleyTransactionBody,
    ShelleyTransactionBodyList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::shelley::ShelleyTransactionOutput,
    ShelleyTransactionOutput,
    ShelleyTransactionOutputList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_multi_era::shelley::ShelleyTransactionWitnessSet,
    ShelleyTransactionWitnessSet,
    ShelleyTransactionWitnessSetList,
    true,
    false
);
