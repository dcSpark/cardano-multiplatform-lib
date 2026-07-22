// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::generated::assets::Coin;
use crate::generated::auxdata::Metadata;
use crate::generated::certs::StakeCredential;
use crate::generated::crypto::GenesisHash;
use crate::generated::transaction::AlonzoFormatTxOut;
use crate::generated::{DeltaCoin, TransactionIndex};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core_wasm::{impl_wasm_conversions, impl_wasm_list_needs_into};
use wasm_bindgen::prelude::wasm_bindgen;

impl_wasm_list_needs_into!(
    cml_chain::transaction::AlonzoFormatTxOut,
    AlonzoFormatTxOut,
    AlonzoFormatTxOutList,
    true,
    false
);

impl_wasm_list_needs_into!(
    cml_chain::crypto::GenesisHash,
    GenesisHash,
    GenesisHashList,
    true,
    true
);

/// Generated at the request of: cml-multi-era.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapStakeCredentialToCoin(
    pub(crate) OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::assets::Coin>,
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

/// Generated at the request of: cml-multi-era.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapStakeCredentialToDeltaCoin(
    pub(crate) OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::DeltaCoin>,
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

/// Generated at the request of: cml-multi-era.
#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapTransactionIndexToMetadata(
    pub(crate) OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::Metadata>,
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
    cml_chain::certs::StakeCredential,
    StakeCredential,
    StakeCredentialList,
    true,
    false
);
