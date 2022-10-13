use wasm_bindgen::prelude::*;

use std::collections::BTreeMap;

use core::ordered_hash_map::OrderedHashMap;

use core::serialization::{LenEncoding, StringEncoding};

impl From<OrderedHashMap<core::TransactionMetadatumLabel, core::TransactionMetadatum>> for MapTransactionMetadatumLabelToTransactionMetadatum {
    fn from(native: OrderedHashMap<core::TransactionMetadatumLabel, core::TransactionMetadatum>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<OrderedHashMap<core::TransactionMetadatumLabel, core::TransactionMetadatum>> for MapTransactionMetadatumLabelToTransactionMetadatum {
    fn into(self) -> OrderedHashMap<core::TransactionMetadatumLabel, core::TransactionMetadatum> {
        self.0
    }
}

impl From<OrderedHashMap<core::TransactionMetadatum, core::TransactionMetadatum>> for MapTransactionMetadatumToTransactionMetadatum {
    fn from(native: OrderedHashMap<core::TransactionMetadatum, core::TransactionMetadatum>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<OrderedHashMap<core::TransactionMetadatum, core::TransactionMetadatum>> for MapTransactionMetadatumToTransactionMetadatum {
    fn into(self) -> OrderedHashMap<core::TransactionMetadatum, core::TransactionMetadatum> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct TransactionMetadatums(pub(crate) Vec<core::TransactionMetadatum>);

#[wasm_bindgen]

impl TransactionMetadatums {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionMetadatum {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionMetadatum) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::TransactionMetadatum>> for TransactionMetadatums {
    fn from(native: Vec<core::TransactionMetadatum>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::TransactionMetadatum>> for TransactionMetadatums {
    fn into(self) -> Vec<core::TransactionMetadatum> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct NativeScripts(pub(crate) Vec<core::NativeScript>);

#[wasm_bindgen]

impl NativeScripts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> NativeScript {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &NativeScript) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::NativeScript>> for NativeScripts {
    fn from(native: Vec<core::NativeScript>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::NativeScript>> for NativeScripts {
    fn into(self) -> Vec<core::NativeScript> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct PlutusV1Scripts(pub(crate) Vec<core::PlutusV1Script>);

#[wasm_bindgen]

impl PlutusV1Scripts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV1Script {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: PlutusV1Script) {
        self.0.push(elem);
    }
}

impl From<Vec<core::PlutusV1Script>> for PlutusV1Scripts {
    fn from(native: Vec<core::PlutusV1Script>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::PlutusV1Script>> for PlutusV1Scripts {
    fn into(self) -> Vec<core::PlutusV1Script> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct PlutusV2Scripts(pub(crate) Vec<core::PlutusV2Script>);

#[wasm_bindgen]

impl PlutusV2Scripts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV2Script {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: PlutusV2Script) {
        self.0.push(elem);
    }
}

impl From<Vec<core::PlutusV2Script>> for PlutusV2Scripts {
    fn from(native: Vec<core::PlutusV2Script>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::PlutusV2Script>> for PlutusV2Scripts {
    fn into(self) -> Vec<core::PlutusV2Script> {
        self.0
    }
}

impl From<OrderedHashMap<core::PolicyId, OrderedHashMap<core::AssetName, u64>>> for MapPolicyIdToMapAssetNameToU64 {
    fn from(native: OrderedHashMap<core::PolicyId, OrderedHashMap<core::AssetName, u64>>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<OrderedHashMap<core::PolicyId, OrderedHashMap<core::AssetName, u64>>> for MapPolicyIdToMapAssetNameToU64 {
    fn into(self) -> OrderedHashMap<core::PolicyId, OrderedHashMap<core::AssetName, u64>> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct PolicyIds(pub(crate) Vec<core::PolicyId>);

#[wasm_bindgen]

impl PolicyIds {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PolicyId {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PolicyId) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::PolicyId>> for PolicyIds {
    fn from(native: Vec<core::PolicyId>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::PolicyId>> for PolicyIds {
    fn into(self) -> Vec<core::PolicyId> {
        self.0
    }
}

impl From<OrderedHashMap<core::AssetName, u64>> for MapAssetNameToU64 {
    fn from(native: OrderedHashMap<core::AssetName, u64>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<OrderedHashMap<core::AssetName, u64>> for MapAssetNameToU64 {
    fn into(self) -> OrderedHashMap<core::AssetName, u64> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct AssetNames(pub(crate) Vec<core::AssetName>);

#[wasm_bindgen]

impl AssetNames {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AssetName {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &AssetName) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::AssetName>> for AssetNames {
    fn from(native: Vec<core::AssetName>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::AssetName>> for AssetNames {
    fn into(self) -> Vec<core::AssetName> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct TransactionBodys(pub(crate) Vec<core::TransactionBody>);

#[wasm_bindgen]

impl TransactionBodys {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionBody {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionBody) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::TransactionBody>> for TransactionBodys {
    fn from(native: Vec<core::TransactionBody>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::TransactionBody>> for TransactionBodys {
    fn into(self) -> Vec<core::TransactionBody> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct TransactionInputs(pub(crate) Vec<core::TransactionInput>);

#[wasm_bindgen]

impl TransactionInputs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionInput {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionInput) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::TransactionInput>> for TransactionInputs {
    fn from(native: Vec<core::TransactionInput>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::TransactionInput>> for TransactionInputs {
    fn into(self) -> Vec<core::TransactionInput> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct TransactionOutputs(pub(crate) Vec<core::TransactionOutput>);

#[wasm_bindgen]

impl TransactionOutputs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionOutput {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionOutput) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::TransactionOutput>> for TransactionOutputs {
    fn from(native: Vec<core::TransactionOutput>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::TransactionOutput>> for TransactionOutputs {
    fn into(self) -> Vec<core::TransactionOutput> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Certificates(pub(crate) Vec<core::Certificate>);

#[wasm_bindgen]

impl Certificates {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Certificate {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Certificate) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::Certificate>> for Certificates {
    fn from(native: Vec<core::Certificate>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::Certificate>> for Certificates {
    fn into(self) -> Vec<core::Certificate> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct AddrKeyhashs(pub(crate) Vec<core::AddrKeyhash>);

#[wasm_bindgen]

impl AddrKeyhashs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AddrKeyhash {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &AddrKeyhash) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::AddrKeyhash>> for AddrKeyhashs {
    fn from(native: Vec<core::AddrKeyhash>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::AddrKeyhash>> for AddrKeyhashs {
    fn into(self) -> Vec<core::AddrKeyhash> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Relays(pub(crate) Vec<core::Relay>);

#[wasm_bindgen]

impl Relays {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Relay {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Relay) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::Relay>> for Relays {
    fn from(native: Vec<core::Relay>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::Relay>> for Relays {
    fn into(self) -> Vec<core::Relay> {
        self.0
    }
}

impl From<OrderedHashMap<core::StakeCredential, core::DeltaCoin>> for MapStakeCredentialToDeltaCoin {
    fn from(native: OrderedHashMap<core::StakeCredential, core::DeltaCoin>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<OrderedHashMap<core::StakeCredential, core::DeltaCoin>> for MapStakeCredentialToDeltaCoin {
    fn into(self) -> OrderedHashMap<core::StakeCredential, core::DeltaCoin> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct StakeCredentials(pub(crate) Vec<core::StakeCredential>);

#[wasm_bindgen]

impl StakeCredentials {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> StakeCredential {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &StakeCredential) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::StakeCredential>> for StakeCredentials {
    fn from(native: Vec<core::StakeCredential>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::StakeCredential>> for StakeCredentials {
    fn into(self) -> Vec<core::StakeCredential> {
        self.0
    }
}

impl From<OrderedHashMap<core::RewardAccount, core::Coin>> for MapRewardAccountToCoin {
    fn from(native: OrderedHashMap<core::RewardAccount, core::Coin>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<OrderedHashMap<core::RewardAccount, core::Coin>> for MapRewardAccountToCoin {
    fn into(self) -> OrderedHashMap<core::RewardAccount, core::Coin> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct RewardAccounts(pub(crate) Vec<core::RewardAccount>);

#[wasm_bindgen]

impl RewardAccounts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> RewardAccount {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &RewardAccount) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::RewardAccount>> for RewardAccounts {
    fn from(native: Vec<core::RewardAccount>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::RewardAccount>> for RewardAccounts {
    fn into(self) -> Vec<core::RewardAccount> {
        self.0
    }
}

impl From<OrderedHashMap<core::Genesishash, core::ProtocolParamUpdate>> for MapGenesishashToProtocolParamUpdate {
    fn from(native: OrderedHashMap<core::Genesishash, core::ProtocolParamUpdate>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<OrderedHashMap<core::Genesishash, core::ProtocolParamUpdate>> for MapGenesishashToProtocolParamUpdate {
    fn into(self) -> OrderedHashMap<core::Genesishash, core::ProtocolParamUpdate> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Genesishashs(pub(crate) Vec<core::Genesishash>);

#[wasm_bindgen]

impl Genesishashs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Genesishash {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Genesishash) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::Genesishash>> for Genesishashs {
    fn from(native: Vec<core::Genesishash>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::Genesishash>> for Genesishashs {
    fn into(self) -> Vec<core::Genesishash> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Ints(pub(crate) Vec<core::Int>);

#[wasm_bindgen]

impl Ints {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Int {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Int) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::Int>> for Ints {
    fn from(native: Vec<core::Int>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::Int>> for Ints {
    fn into(self) -> Vec<core::Int> {
        self.0
    }
}

impl From<OrderedHashMap<core::PolicyId, OrderedHashMap<core::AssetName, core::Int64>>> for MapPolicyIdToMapAssetNameToInt64 {
    fn from(native: OrderedHashMap<core::PolicyId, OrderedHashMap<core::AssetName, core::Int64>>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<OrderedHashMap<core::PolicyId, OrderedHashMap<core::AssetName, core::Int64>>> for MapPolicyIdToMapAssetNameToInt64 {
    fn into(self) -> OrderedHashMap<core::PolicyId, OrderedHashMap<core::AssetName, core::Int64>> {
        self.0
    }
}

impl From<OrderedHashMap<core::AssetName, core::Int64>> for MapAssetNameToInt64 {
    fn from(native: OrderedHashMap<core::AssetName, core::Int64>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<OrderedHashMap<core::AssetName, core::Int64>> for MapAssetNameToInt64 {
    fn into(self) -> OrderedHashMap<core::AssetName, core::Int64> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct TransactionWitnessSets(pub(crate) Vec<core::TransactionWitnessSet>);

#[wasm_bindgen]

impl TransactionWitnessSets {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> TransactionWitnessSet {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &TransactionWitnessSet) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::TransactionWitnessSet>> for TransactionWitnessSets {
    fn from(native: Vec<core::TransactionWitnessSet>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::TransactionWitnessSet>> for TransactionWitnessSets {
    fn into(self) -> Vec<core::TransactionWitnessSet> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Vkeywitnesss(pub(crate) Vec<core::Vkeywitness>);

#[wasm_bindgen]

impl Vkeywitnesss {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Vkeywitness {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Vkeywitness) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::Vkeywitness>> for Vkeywitnesss {
    fn from(native: Vec<core::Vkeywitness>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::Vkeywitness>> for Vkeywitnesss {
    fn into(self) -> Vec<core::Vkeywitness> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct BootstrapWitnesss(pub(crate) Vec<core::BootstrapWitness>);

#[wasm_bindgen]

impl BootstrapWitnesss {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> BootstrapWitness {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &BootstrapWitness) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::BootstrapWitness>> for BootstrapWitnesss {
    fn from(native: Vec<core::BootstrapWitness>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::BootstrapWitness>> for BootstrapWitnesss {
    fn into(self) -> Vec<core::BootstrapWitness> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct PlutusDatas(pub(crate) Vec<core::PlutusData>);

#[wasm_bindgen]

impl PlutusDatas {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusData {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusData) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::PlutusData>> for PlutusDatas {
    fn from(native: Vec<core::PlutusData>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::PlutusData>> for PlutusDatas {
    fn into(self) -> Vec<core::PlutusData> {
        self.0
    }
}

impl From<OrderedHashMap<core::PlutusData, core::PlutusData>> for MapPlutusDataToPlutusData {
    fn from(native: OrderedHashMap<core::PlutusData, core::PlutusData>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<OrderedHashMap<core::PlutusData, core::PlutusData>> for MapPlutusDataToPlutusData {
    fn into(self) -> OrderedHashMap<core::PlutusData, core::PlutusData> {
        self.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Redeemers(pub(crate) Vec<core::Redeemer>);

#[wasm_bindgen]

impl Redeemers {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Redeemer {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Redeemer) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<core::Redeemer>> for Redeemers {
    fn from(native: Vec<core::Redeemer>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<Vec<core::Redeemer>> for Redeemers {
    fn into(self) -> Vec<core::Redeemer> {
        self.0
    }
}

impl From<OrderedHashMap<core::TransactionIndex, core::AuxiliaryData>> for MapTransactionIndexToAuxiliaryData {
    fn from(native: OrderedHashMap<core::TransactionIndex, core::AuxiliaryData>) -> Self {
        Self(native)
    }
}

impl std::convert::Into<OrderedHashMap<core::TransactionIndex, core::AuxiliaryData>> for MapTransactionIndexToAuxiliaryData {
    fn into(self) -> OrderedHashMap<core::TransactionIndex, core::AuxiliaryData> {
        self.0
    }
}

pub mod address;

pub use address::*;


pub mod block;

pub use block::*;


pub mod certs;

pub use certs::*;


pub mod crypto;

pub use crypto::*;


pub mod metadata;

pub use metadata::*;


pub mod plutus;

pub use plutus::*;


pub mod transaction;

pub use transaction::*;
pub type BoundedBytes = Vec<u8>;

pub type Coin = u64;

pub type DeltaCoin = Int;

pub type Epoch = u64;

pub type Int64 = i64;

pub type PolicyId = Hash28;

pub type Port = u16;

pub type SubCoin = PositiveInterval;

pub type TransactionIndex = u16;

pub type TransactionMetadatumLabel = u64;

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MapTransactionMetadatumLabelToTransactionMetadatum(pub(crate) OrderedHashMap<core::TransactionMetadatumLabel, core::TransactionMetadatum>);

#[wasm_bindgen]

impl MapTransactionMetadatumLabelToTransactionMetadatum {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: TransactionMetadatumLabel, value: &TransactionMetadatum) -> Option<TransactionMetadatum> {
        self.0.insert(key, value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: TransactionMetadatumLabel) -> Option<TransactionMetadatum> {
        self.0.get(&key).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> Vec<TransactionMetadatumLabel> {
        self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>()
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MapTransactionMetadatumToTransactionMetadatum(pub(crate) OrderedHashMap<core::TransactionMetadatum, core::TransactionMetadatum>);

#[wasm_bindgen]

impl MapTransactionMetadatumToTransactionMetadatum {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &TransactionMetadatum, value: &TransactionMetadatum) -> Option<TransactionMetadatum> {
        self.0.insert(key.clone().into(), value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: &TransactionMetadatum) -> Option<TransactionMetadatum> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> TransactionMetadatums {
        TransactionMetadatums(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MapPolicyIdToMapAssetNameToU64(pub(crate) OrderedHashMap<core::PolicyId, OrderedHashMap<core::AssetName, u64>>);

#[wasm_bindgen]

impl MapPolicyIdToMapAssetNameToU64 {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &PolicyId, value: &MapAssetNameToU64) -> Option<MapAssetNameToU64> {
        self.0.insert(key.clone().into(), value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: &PolicyId) -> Option<MapAssetNameToU64> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PolicyIds {
        PolicyIds(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MapAssetNameToU64(pub(crate) OrderedHashMap<core::AssetName, u64>);

#[wasm_bindgen]

impl MapAssetNameToU64 {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &AssetName, value: u64) -> Option<u64> {
        self.0.insert(key.clone().into(), value)
    }

    pub fn get(&self, key: &AssetName) -> Option<u64> {
        self.0.get(&key.0).copied()
    }

    pub fn keys(&self) -> AssetNames {
        AssetNames(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct AssetName(pub(crate) core::AssetName);

#[wasm_bindgen]

impl AssetName {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<AssetName, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<AssetName, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone().clone()
    }
}

impl From<core::AssetName> for AssetName {
    fn from(native: core::AssetName) -> Self {
        Self(native)
    }
}

impl From<AssetName> for core::AssetName {
    fn from(wasm: AssetName) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MapStakeCredentialToDeltaCoin(pub(crate) OrderedHashMap<core::StakeCredential, core::DeltaCoin>);

#[wasm_bindgen]

impl MapStakeCredentialToDeltaCoin {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &StakeCredential, value: &DeltaCoin) -> Option<DeltaCoin> {
        self.0.insert(key.clone().into(), value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: &StakeCredential) -> Option<DeltaCoin> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> StakeCredentials {
        StakeCredentials(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MapRewardAccountToCoin(pub(crate) OrderedHashMap<core::RewardAccount, core::Coin>);

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
        self.0.get(&key.0).copied()
    }

    pub fn keys(&self) -> RewardAccounts {
        RewardAccounts(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MapGenesishashToProtocolParamUpdate(pub(crate) OrderedHashMap<core::Genesishash, core::ProtocolParamUpdate>);

#[wasm_bindgen]

impl MapGenesishashToProtocolParamUpdate {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &Genesishash, value: &ProtocolParamUpdate) -> Option<ProtocolParamUpdate> {
        self.0.insert(key.clone().into(), value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: &Genesishash) -> Option<ProtocolParamUpdate> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> Genesishashs {
        Genesishashs(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MapPolicyIdToMapAssetNameToInt64(pub(crate) OrderedHashMap<core::PolicyId, OrderedHashMap<core::AssetName, core::Int64>>);

#[wasm_bindgen]

impl MapPolicyIdToMapAssetNameToInt64 {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &PolicyId, value: &MapAssetNameToInt64) -> Option<MapAssetNameToInt64> {
        self.0.insert(key.clone().into(), value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: &PolicyId) -> Option<MapAssetNameToInt64> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PolicyIds {
        PolicyIds(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MapAssetNameToInt64(pub(crate) OrderedHashMap<core::AssetName, core::Int64>);

#[wasm_bindgen]

impl MapAssetNameToInt64 {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &AssetName, value: Int64) -> Option<Int64> {
        self.0.insert(key.clone().into(), value)
    }

    pub fn get(&self, key: &AssetName) -> Option<Int64> {
        self.0.get(&key.0).copied()
    }

    pub fn keys(&self) -> AssetNames {
        AssetNames(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MapPlutusDataToPlutusData(pub(crate) OrderedHashMap<core::PlutusData, core::PlutusData>);

#[wasm_bindgen]

impl MapPlutusDataToPlutusData {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &PlutusData, value: &PlutusData) -> Option<PlutusData> {
        self.0.insert(key.clone().into(), value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: &PlutusData) -> Option<PlutusData> {
        self.0.get(&key.0).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PlutusDatas {
        PlutusDatas(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct MapTransactionIndexToAuxiliaryData(pub(crate) OrderedHashMap<core::TransactionIndex, core::AuxiliaryData>);

#[wasm_bindgen]

impl MapTransactionIndexToAuxiliaryData {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: TransactionIndex, value: &AuxiliaryData) -> Option<AuxiliaryData> {
        self.0.insert(key, value.clone().into()).map(|v| v.clone().into())
    }

    pub fn get(&self, key: TransactionIndex) -> Option<AuxiliaryData> {
        self.0.get(&key).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> Vec<TransactionIndex> {
        self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>()
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct BootstrapWitness(pub(crate) core::BootstrapWitness);

#[wasm_bindgen]

impl BootstrapWitness {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<BootstrapWitness, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<BootstrapWitness, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn public_key(&self) -> Vkey {
        self.0.public_key.clone().into()
    }

    pub fn signature(&self) -> Signature {
        self.0.signature.clone().into()
    }

    pub fn chain_code(&self) -> Vec<u8> {
        self.0.chain_code.clone()
    }

    pub fn attributes(&self) -> Vec<u8> {
        self.0.attributes.clone()
    }

    pub fn new(public_key: &Vkey, signature: &Signature, chain_code: Vec<u8>, attributes: Vec<u8>) -> Self {
        Self(core::BootstrapWitness::new(public_key.clone().into(), signature.clone().into(), chain_code, attributes))
    }
}

impl From<core::BootstrapWitness> for BootstrapWitness {
    fn from(native: core::BootstrapWitness) -> Self {
        Self(native)
    }
}

impl From<BootstrapWitness> for core::BootstrapWitness {
    fn from(wasm: BootstrapWitness) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct DatumOption0(pub(crate) core::DatumOption0);

#[wasm_bindgen]

impl DatumOption0 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<DatumOption0, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<DatumOption0, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn hash32(&self) -> Hash32 {
        self.0.hash32.clone().into()
    }

    pub fn new(hash32: &Hash32) -> Self {
        Self(core::DatumOption0::new(hash32.clone().into()))
    }
}

impl From<core::DatumOption0> for DatumOption0 {
    fn from(native: core::DatumOption0) -> Self {
        Self(native)
    }
}

impl From<DatumOption0> for core::DatumOption0 {
    fn from(wasm: DatumOption0) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct DatumOption1(pub(crate) core::DatumOption1);

#[wasm_bindgen]

impl DatumOption1 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<DatumOption1, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<DatumOption1, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn data(&self) -> Data {
        self.0.data.clone()
    }

    pub fn new(data: Data) -> Self {
        Self(core::DatumOption1::new(data.into()))
    }
}

impl From<core::DatumOption1> for DatumOption1 {
    fn from(native: core::DatumOption1) -> Self {
        Self(native)
    }
}

impl From<DatumOption1> for core::DatumOption1 {
    fn from(wasm: DatumOption1) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum I0OrI1Kind {
    I0,
    I1,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct I0OrI1(pub(crate) core::I0OrI1);

#[wasm_bindgen]

impl I0OrI1 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<I0OrI1, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<I0OrI1, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_i0() -> Self {
        Self(core::I0OrI1::new_i0())
    }

    pub fn new_i1() -> Self {
        Self(core::I0OrI1::new_i1())
    }

    pub fn kind(&self) -> I0OrI1Kind {
        match &self.0 {
            core::I0OrI1::I0{ .. } => I0OrI1Kind::I0,
            core::I0OrI1::I1{ .. } => I0OrI1Kind::I1,
        }
    }
}

impl From<core::I0OrI1> for I0OrI1 {
    fn from(native: core::I0OrI1) -> Self {
        Self(native)
    }
}

impl From<I0OrI1> for core::I0OrI1 {
    fn from(wasm: I0OrI1) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Int(pub(crate) core::Int);

#[wasm_bindgen]

impl Int {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Int, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Int, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new(x: i64) -> Self {
        if x >= 0 {
            Self(core::Int::new_uint(x as u64))
        }
        else {
            Self(core::Int::new_nint((x + 1).abs() as u64))
        }
    }

    pub fn to_str(&self) -> String {
        self.0.to_string()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(string: &str) -> Result<Int, JsValue> {
        // have to redefine so it's visible in WASM
        std::str::FromStr::from_str(string).map(Self).map_err(|e| JsValue::from_str(&format!("Int.from_str({}): {:?}", string, e)))
    }
}

impl From<core::Int> for Int {
    fn from(native: core::Int) -> Self {
        Self(native)
    }
}

impl From<Int> for core::Int {
    fn from(wasm: Int) -> Self {
        wasm.0
    }
}

type Mint = MapPolicyIdToMapAssetNameToInt64;

type Multiasset = MapPolicyIdToMapAssetNameToU64;

#[wasm_bindgen]

pub enum NetworkIdKind {
    I0,
    I1,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct NetworkId(pub(crate) core::NetworkId);

#[wasm_bindgen]

impl NetworkId {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<NetworkId, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<NetworkId, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_i0() -> Self {
        Self(core::NetworkId::new_i0())
    }

    pub fn new_i1() -> Self {
        Self(core::NetworkId::new_i1())
    }

    pub fn kind(&self) -> NetworkIdKind {
        match &self.0 {
            core::NetworkId::I0{ .. } => NetworkIdKind::I0,
            core::NetworkId::I1{ .. } => NetworkIdKind::I1,
        }
    }
}

impl From<core::NetworkId> for NetworkId {
    fn from(native: core::NetworkId) -> Self {
        Self(native)
    }
}

impl From<NetworkId> for core::NetworkId {
    fn from(wasm: NetworkId) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Nonce1(pub(crate) core::Nonce1);

#[wasm_bindgen]

impl Nonce1 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Nonce1, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Nonce1, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.bytes.clone()
    }

    pub fn new(bytes: Vec<u8>) -> Self {
        Self(core::Nonce1::new(bytes))
    }
}

impl From<core::Nonce1> for Nonce1 {
    fn from(native: core::Nonce1) -> Self {
        Self(native)
    }
}

impl From<Nonce1> for core::Nonce1 {
    fn from(wasm: Nonce1) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct PositiveInterval(pub(crate) core::PositiveInterval);

#[wasm_bindgen]

impl PositiveInterval {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<PositiveInterval, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<PositiveInterval, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new() -> Self {
        Self(core::PositiveInterval::new())
    }
}

impl From<core::PositiveInterval> for PositiveInterval {
    fn from(native: core::PositiveInterval) -> Self {
        Self(native)
    }
}

impl From<PositiveInterval> for core::PositiveInterval {
    fn from(wasm: PositiveInterval) -> Self {
        wasm.0
    }
}

type ProposedProtocolParameterUpdates = MapGenesishashToProtocolParamUpdate;

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ProtocolParamUpdate(pub(crate) core::ProtocolParamUpdate);

#[wasm_bindgen]

impl ProtocolParamUpdate {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ProtocolParamUpdate, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ProtocolParamUpdate, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn set_key_0(&mut self, key_0: u64) {
        self.0.key_0 = Some(key_0)
    }

    pub fn key_0(&self) -> Option<u64> {
        self.0.key_0
    }

    pub fn set_key_1(&mut self, key_1: u64) {
        self.0.key_1 = Some(key_1)
    }

    pub fn key_1(&self) -> Option<u64> {
        self.0.key_1
    }

    pub fn set_key_2(&mut self, key_2: u64) {
        self.0.key_2 = Some(key_2)
    }

    pub fn key_2(&self) -> Option<u64> {
        self.0.key_2
    }

    pub fn set_key_3(&mut self, key_3: u64) {
        self.0.key_3 = Some(key_3)
    }

    pub fn key_3(&self) -> Option<u64> {
        self.0.key_3
    }

    pub fn set_key_4(&mut self, key_4: u64) {
        self.0.key_4 = Some(key_4)
    }

    pub fn key_4(&self) -> Option<u64> {
        self.0.key_4
    }

    pub fn set_key_5(&mut self, key_5: Coin) {
        self.0.key_5 = Some(key_5)
    }

    pub fn key_5(&self) -> Option<Coin> {
        self.0.key_5
    }

    pub fn set_key_6(&mut self, key_6: Coin) {
        self.0.key_6 = Some(key_6)
    }

    pub fn key_6(&self) -> Option<Coin> {
        self.0.key_6
    }

    pub fn set_key_7(&mut self, key_7: Epoch) {
        self.0.key_7 = Some(key_7)
    }

    pub fn key_7(&self) -> Option<Epoch> {
        self.0.key_7
    }

    pub fn set_key_8(&mut self, key_8: u64) {
        self.0.key_8 = Some(key_8)
    }

    pub fn key_8(&self) -> Option<u64> {
        self.0.key_8
    }

    pub fn set_key_9(&mut self, key_9: &Rational) {
        self.0.key_9 = Some(key_9.clone().into())
    }

    pub fn key_9(&self) -> Option<Rational> {
        self.0.key_9.clone().map(std::convert::Into::into)
    }

    pub fn set_key_10(&mut self, key_10: &UnitInterval) {
        self.0.key_10 = Some(key_10.clone().into())
    }

    pub fn key_10(&self) -> Option<UnitInterval> {
        self.0.key_10.clone().map(std::convert::Into::into)
    }

    pub fn set_key_11(&mut self, key_11: &UnitInterval) {
        self.0.key_11 = Some(key_11.clone().into())
    }

    pub fn key_11(&self) -> Option<UnitInterval> {
        self.0.key_11.clone().map(std::convert::Into::into)
    }

    pub fn set_key_14(&mut self, key_14: &ProtocolVersionStruct) {
        self.0.key_14 = Some(key_14.clone().into())
    }

    pub fn key_14(&self) -> Option<ProtocolVersionStruct> {
        self.0.key_14.clone().map(std::convert::Into::into)
    }

    pub fn set_key_16(&mut self, key_16: Coin) {
        self.0.key_16 = Some(key_16)
    }

    pub fn key_16(&self) -> Option<Coin> {
        self.0.key_16
    }

    pub fn set_key_17(&mut self, key_17: Coin) {
        self.0.key_17 = Some(key_17)
    }

    pub fn key_17(&self) -> Option<Coin> {
        self.0.key_17
    }

    pub fn set_key_18(&mut self, key_18: &Costmdls) {
        self.0.key_18 = Some(key_18.clone().into())
    }

    pub fn key_18(&self) -> Option<Costmdls> {
        self.0.key_18.clone().map(std::convert::Into::into)
    }

    pub fn set_key_19(&mut self, key_19: &ExUnitPrices) {
        self.0.key_19 = Some(key_19.clone().into())
    }

    pub fn key_19(&self) -> Option<ExUnitPrices> {
        self.0.key_19.clone().map(std::convert::Into::into)
    }

    pub fn set_key_20(&mut self, key_20: &ExUnits) {
        self.0.key_20 = Some(key_20.clone().into())
    }

    pub fn key_20(&self) -> Option<ExUnits> {
        self.0.key_20.clone().map(std::convert::Into::into)
    }

    pub fn set_key_21(&mut self, key_21: &ExUnits) {
        self.0.key_21 = Some(key_21.clone().into())
    }

    pub fn key_21(&self) -> Option<ExUnits> {
        self.0.key_21.clone().map(std::convert::Into::into)
    }

    pub fn set_key_22(&mut self, key_22: u64) {
        self.0.key_22 = Some(key_22)
    }

    pub fn key_22(&self) -> Option<u64> {
        self.0.key_22
    }

    pub fn set_key_23(&mut self, key_23: u64) {
        self.0.key_23 = Some(key_23)
    }

    pub fn key_23(&self) -> Option<u64> {
        self.0.key_23
    }

    pub fn set_key_24(&mut self, key_24: u64) {
        self.0.key_24 = Some(key_24)
    }

    pub fn key_24(&self) -> Option<u64> {
        self.0.key_24
    }

    pub fn new() -> Self {
        Self(core::ProtocolParamUpdate::new())
    }
}

impl From<core::ProtocolParamUpdate> for ProtocolParamUpdate {
    fn from(native: core::ProtocolParamUpdate) -> Self {
        Self(native)
    }
}

impl From<ProtocolParamUpdate> for core::ProtocolParamUpdate {
    fn from(wasm: ProtocolParamUpdate) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ProtocolVersionStruct(pub(crate) core::ProtocolVersionStruct);

#[wasm_bindgen]

impl ProtocolVersionStruct {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ProtocolVersionStruct, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ProtocolVersionStruct, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.0.protocol_version.clone().into()
    }

    pub fn new(protocol_version: &ProtocolVersion) -> Self {
        Self(core::ProtocolVersionStruct::new(protocol_version.clone().into()))
    }
}

impl From<core::ProtocolVersionStruct> for ProtocolVersionStruct {
    fn from(native: core::ProtocolVersionStruct) -> Self {
        Self(native)
    }
}

impl From<ProtocolVersionStruct> for core::ProtocolVersionStruct {
    fn from(wasm: ProtocolVersionStruct) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Rational(pub(crate) core::Rational);

#[wasm_bindgen]

impl Rational {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Rational, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Rational, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn numerator(&self) -> u64 {
        self.0.numerator
    }

    pub fn denominator(&self) -> u64 {
        self.0.denominator
    }

    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self(core::Rational::new(numerator, denominator))
    }
}

impl From<core::Rational> for Rational {
    fn from(native: core::Rational) -> Self {
        Self(native)
    }
}

impl From<Rational> for core::Rational {
    fn from(wasm: Rational) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Script0(pub(crate) core::Script0);

#[wasm_bindgen]

impl Script0 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Script0, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Script0, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn native_script(&self) -> NativeScript {
        self.0.native_script.clone().into()
    }

    pub fn new(native_script: &NativeScript) -> Self {
        Self(core::Script0::new(native_script.clone().into()))
    }
}

impl From<core::Script0> for Script0 {
    fn from(native: core::Script0) -> Self {
        Self(native)
    }
}

impl From<Script0> for core::Script0 {
    fn from(wasm: Script0) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Script1(pub(crate) core::Script1);

#[wasm_bindgen]

impl Script1 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Script1, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Script1, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn plutus_v1_script(&self) -> PlutusV1Script {
        self.0.plutus_v1_script.clone()
    }

    pub fn new(plutus_v1_script: PlutusV1Script) -> Self {
        Self(core::Script1::new(plutus_v1_script))
    }
}

impl From<core::Script1> for Script1 {
    fn from(native: core::Script1) -> Self {
        Self(native)
    }
}

impl From<Script1> for core::Script1 {
    fn from(wasm: Script1) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Script2(pub(crate) core::Script2);

#[wasm_bindgen]

impl Script2 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Script2, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Script2, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn plutus_v2_script(&self) -> PlutusV2Script {
        self.0.plutus_v2_script.clone()
    }

    pub fn new(plutus_v2_script: PlutusV2Script) -> Self {
        Self(core::Script2::new(plutus_v2_script))
    }
}

impl From<core::Script2> for Script2 {
    fn from(native: core::Script2) -> Self {
        Self(native)
    }
}

impl From<Script2> for core::Script2 {
    fn from(wasm: Script2) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct StakeCredential0(pub(crate) core::StakeCredential0);

#[wasm_bindgen]

impl StakeCredential0 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<StakeCredential0, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<StakeCredential0, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn addr_keyhash(&self) -> AddrKeyhash {
        self.0.addr_keyhash.clone().into()
    }

    pub fn new(addr_keyhash: &AddrKeyhash) -> Self {
        Self(core::StakeCredential0::new(addr_keyhash.clone().into()))
    }
}

impl From<core::StakeCredential0> for StakeCredential0 {
    fn from(native: core::StakeCredential0) -> Self {
        Self(native)
    }
}

impl From<StakeCredential0> for core::StakeCredential0 {
    fn from(wasm: StakeCredential0) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct StakeCredential1(pub(crate) core::StakeCredential1);

#[wasm_bindgen]

impl StakeCredential1 {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<StakeCredential1, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<StakeCredential1, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn scripthash(&self) -> Scripthash {
        self.0.scripthash.clone().into()
    }

    pub fn new(scripthash: &Scripthash) -> Self {
        Self(core::StakeCredential1::new(scripthash.clone().into()))
    }
}

impl From<core::StakeCredential1> for StakeCredential1 {
    fn from(native: core::StakeCredential1) -> Self {
        Self(native)
    }
}

impl From<StakeCredential1> for core::StakeCredential1 {
    fn from(wasm: StakeCredential1) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct UnitInterval(pub(crate) core::UnitInterval);

#[wasm_bindgen]

impl UnitInterval {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<UnitInterval, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<UnitInterval, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_0(&self) -> u64 {
        self.0.index_0
    }

    pub fn index_1(&self) -> u64 {
        self.0.index_1
    }

    pub fn new(index_0: u64, index_1: u64) -> Self {
        Self(core::UnitInterval::new(index_0, index_1))
    }
}

impl From<core::UnitInterval> for UnitInterval {
    fn from(native: core::UnitInterval) -> Self {
        Self(native)
    }
}

impl From<UnitInterval> for core::UnitInterval {
    fn from(wasm: UnitInterval) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Update(pub(crate) core::Update);

#[wasm_bindgen]

impl Update {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Update, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Update, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn proposed_protocol_parameter_updates(&self) -> ProposedProtocolParameterUpdates {
        self.0.proposed_protocol_parameter_updates.clone().into()
    }

    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    pub fn new(proposed_protocol_parameter_updates: ProposedProtocolParameterUpdates, epoch: Epoch) -> Self {
        Self(core::Update::new(proposed_protocol_parameter_updates.clone().into(), epoch))
    }
}

impl From<core::Update> for Update {
    fn from(native: core::Update) -> Self {
        Self(native)
    }
}

impl From<Update> for core::Update {
    fn from(wasm: Update) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Value(pub(crate) core::Value);

#[wasm_bindgen]

impl Value {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Value, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Value, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn coin(&self) -> Coin {
        self.0.coin
    }

    pub fn multiasset(&self) -> Multiasset {
        self.0.multiasset.clone().into()
    }

    pub fn new(coin: Coin, multiasset: Multiasset) -> Self {
        Self(core::Value::new(coin, multiasset.clone().into()))
    }
}

impl From<core::Value> for Value {
    fn from(native: core::Value) -> Self {
        Self(native)
    }
}

impl From<Value> for core::Value {
    fn from(wasm: Value) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Vkeywitness(pub(crate) core::Vkeywitness);

#[wasm_bindgen]

impl Vkeywitness {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Vkeywitness, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Vkeywitness, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn vkey(&self) -> Vkey {
        self.0.vkey.clone().into()
    }

    pub fn signature(&self) -> Signature {
        self.0.signature.clone().into()
    }

    pub fn new(vkey: &Vkey, signature: &Signature) -> Self {
        Self(core::Vkeywitness::new(vkey.clone().into(), signature.clone().into()))
    }
}

impl From<core::Vkeywitness> for Vkeywitness {
    fn from(native: core::Vkeywitness) -> Self {
        Self(native)
    }
}

impl From<Vkeywitness> for core::Vkeywitness {
    fn from(wasm: Vkeywitness) -> Self {
        wasm.0
    }
}

type Withdrawals = MapRewardAccountToCoin;