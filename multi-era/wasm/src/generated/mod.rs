// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
pub mod allegra;
pub mod alonzo;
pub mod babbage;
pub mod collections;
pub mod mary;
pub mod shelley;
pub use crate::Block;
pub use crate::ByronBlock;

use crate::{
    allegra::{
        AllegraAuxiliaryData, AllegraBlock, AllegraCertificate, AllegraTransactionBody,
        AllegraTransactionWitnessSet,
    },
    alonzo::{
        AlonzoAuxiliaryData, AlonzoBlock, AlonzoRedeemer, AlonzoTransactionBody,
        AlonzoTransactionWitnessSet,
    },
    babbage::{
        BabbageAuxiliaryData, BabbageBlock, BabbageTransactionBody, BabbageTransactionOutput,
        BabbageTransactionWitnessSet,
    },
    byron::transaction::ByronTx,
    mary::{MaryBlock, MaryTransactionBody, MaryTransactionOutput},
    shelley::{
        MultisigScript, ShelleyBlock, ShelleyCertificate, ShelleyRelay, ShelleyTransactionBody,
        ShelleyTransactionOutput, ShelleyTransactionWitnessSet,
    },
};
use cml_chain_wasm::{
    AssetNameList, Coin, DeltaCoin, PolicyId, PolicyIdList, RewardAccountList, TransactionIndex,
    address::RewardAccount,
    assets::AssetName,
    auxdata::Metadata,
    certs::StakeCredential,
    crypto::{BootstrapWitness, Ed25519KeyHash, Vkeywitness},
    transaction::{AlonzoFormatTxOut, TransactionBody, TransactionInput},
};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core_wasm::{
    impl_wasm_cbor_json_api, impl_wasm_conversions, impl_wasm_list, impl_wasm_list_needs_into,
    impl_wasm_map,
};
use cml_crypto_wasm::GenesisHash;
use wasm_bindgen::prelude::wasm_bindgen;

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
    cml_chain::transaction::AlonzoFormatTxOut,
    AlonzoFormatTxOut,
    AlonzoFormatTxOutList,
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

impl_wasm_list_needs_into!(
    cml_chain::crypto::BootstrapWitness,
    BootstrapWitness,
    BootstrapWitnessList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::crypto::Ed25519KeyHash,
    Ed25519KeyHash,
    Ed25519KeyHashList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::crypto::GenesisHash,
    GenesisHash,
    GenesisHashList,
    true,
    false
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapAssetNameToI64(OrderedHashMap<cml_chain::assets::AssetName, i64>);

impl_wasm_conversions!(OrderedHashMap<cml_chain::assets::AssetName, i64>, MapAssetNameToI64);

#[wasm_bindgen]
impl MapAssetNameToI64 {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &AssetName, value: i64) -> Option<i64> {
        self.0.insert(key.clone().into(), value)
    }

    pub fn get(&self, key: &AssetName) -> Option<i64> {
        self.0.get(key.as_ref()).copied()
    }

    pub fn keys(&self) -> AssetNameList {
        self.0.keys().cloned().collect::<Vec<_>>().into()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapStakeCredentialToCoin(
    OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::assets::Coin>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::assets::Coin>, MapStakeCredentialToCoin);

#[wasm_bindgen]
impl MapStakeCredentialToCoin {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &StakeCredential, value: Coin) -> Option<Coin> {
        self.0.insert(key.clone().into(), value)
    }

    pub fn get(&self, key: &StakeCredential) -> Option<Coin> {
        self.0.get(key.as_ref()).copied()
    }

    pub fn keys(&self) -> StakeCredentialList {
        StakeCredentialList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapStakeCredentialToDeltaCoin(
    OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::DeltaCoin>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::DeltaCoin>, MapStakeCredentialToDeltaCoin);

#[wasm_bindgen]
impl MapStakeCredentialToDeltaCoin {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &StakeCredential, value: &DeltaCoin) -> Option<DeltaCoin> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &StakeCredential) -> Option<DeltaCoin> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> StakeCredentialList {
        StakeCredentialList(self.0.keys().cloned().collect::<Vec<_>>())
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapTransactionIndexToAllegraAuxiliaryData(
    OrderedHashMap<
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
pub struct MapTransactionIndexToAlonzoAuxiliaryData(
    OrderedHashMap<
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
pub struct MapTransactionIndexToBabbageAuxiliaryData(
    OrderedHashMap<
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapTransactionIndexToMetadata(
    OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::Metadata>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::Metadata>, MapTransactionIndexToMetadata);

#[wasm_bindgen]
impl MapTransactionIndexToMetadata {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: TransactionIndex, value: &Metadata) -> Option<Metadata> {
        self.0.insert(key, value.clone().into()).map(Into::into)
    }

    pub fn get(&self, key: TransactionIndex) -> Option<Metadata> {
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
pub struct MultiEraBlock(cml_multi_era::MultiEraBlock);

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
pub struct MultiEraTransactionBody(cml_multi_era::MultiEraTransactionBody);

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

impl_wasm_list_needs_into!(
    cml_chain::certs::StakeCredential,
    StakeCredential,
    StakeCredentialList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::transaction::TransactionInput,
    TransactionInput,
    TransactionInputList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::crypto::Vkeywitness,
    Vkeywitness,
    VkeywitnessList,
    true,
    false
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapRewardAccountToCoin(
    OrderedHashMap<cml_chain::address::RewardAccount, cml_chain::assets::Coin>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::address::RewardAccount, cml_chain::assets::Coin>, MapRewardAccountToCoin);

#[wasm_bindgen]
impl MapRewardAccountToCoin {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &RewardAccount, value: Coin) -> Option<Coin> {
        self.0.insert(key.clone().into(), value)
    }

    pub fn get(&self, key: &RewardAccount) -> Option<Coin> {
        self.0.get(key.as_ref()).copied()
    }

    pub fn keys(&self) -> RewardAccountList {
        self.0.keys().cloned().collect::<Vec<_>>().into()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapPolicyIdToMapAssetNameToI64(
    OrderedHashMap<cml_chain::PolicyId, OrderedHashMap<cml_chain::assets::AssetName, i64>>,
);

impl_wasm_conversions!(OrderedHashMap<cml_chain::PolicyId, OrderedHashMap<cml_chain::assets::AssetName, i64>>, MapPolicyIdToMapAssetNameToI64);

#[wasm_bindgen]
impl MapPolicyIdToMapAssetNameToI64 {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &PolicyId,
        value: &MapAssetNameToI64,
    ) -> Option<MapAssetNameToI64> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &PolicyId) -> Option<MapAssetNameToI64> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PolicyIdList {
        self.0.keys().cloned().collect::<Vec<_>>().into()
    }
}
