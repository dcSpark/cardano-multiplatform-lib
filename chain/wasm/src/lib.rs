#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

use wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};

use cml_core::error::DeserializeError;
pub use cml_core_wasm::Int;

pub mod address;
pub mod auxdata;
pub mod block;
pub mod certs;
pub mod crypto;
pub mod plutus;
pub mod transaction;

use address::RewardAccount;
use auxdata::{AuxiliaryData, TransactionMetadatum};
use block::ProtocolVersion;
use certs::{Certificate, Relay, StakeCredential};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_crypto_wasm::{Ed25519KeyHash, GenesisHash, ScriptHash};
use crypto::{BootstrapWitness, Vkeywitness};
use plutus::{
    CostModels, ExUnitPrices, ExUnits, PlutusData, PlutusV1Script, PlutusV2Script, Redeemer,
};
use transaction::{
    NativeScript, TransactionBody, TransactionInput, TransactionOutput, TransactionWitnessSet,
};

pub mod utils;

//extern crate serde_wasm_bindgen;
// Code below here was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetName(cml_chain::AssetName);

#[wasm_bindgen]
impl AssetName {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<AssetName, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<AssetName, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn get(&self) -> Vec<u8> {
        self.0.inner.clone()
    }
}

impl From<cml_chain::AssetName> for AssetName {
    fn from(native: cml_chain::AssetName) -> Self {
        Self(native)
    }
}

impl From<AssetName> for cml_chain::AssetName {
    fn from(wasm: AssetName) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::AssetName> for AssetName {
    fn as_ref(&self) -> &cml_chain::AssetName {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AssetNameList(Vec<cml_chain::AssetName>);

#[wasm_bindgen]
impl AssetNameList {
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

impl From<Vec<cml_chain::AssetName>> for AssetNameList {
    fn from(native: Vec<cml_chain::AssetName>) -> Self {
        Self(native)
    }
}

impl From<AssetNameList> for Vec<cml_chain::AssetName> {
    fn from(wasm: AssetNameList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::AssetName>> for AssetNameList {
    fn as_ref(&self) -> &Vec<cml_chain::AssetName> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BootstrapWitnessList(Vec<cml_chain::crypto::BootstrapWitness>);

#[wasm_bindgen]
impl BootstrapWitnessList {
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

impl From<Vec<cml_chain::crypto::BootstrapWitness>> for BootstrapWitnessList {
    fn from(native: Vec<cml_chain::crypto::BootstrapWitness>) -> Self {
        Self(native)
    }
}

impl From<BootstrapWitnessList> for Vec<cml_chain::crypto::BootstrapWitness> {
    fn from(wasm: BootstrapWitnessList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::crypto::BootstrapWitness>> for BootstrapWitnessList {
    fn as_ref(&self) -> &Vec<cml_chain::crypto::BootstrapWitness> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct CertificateList(Vec<cml_chain::certs::Certificate>);

#[wasm_bindgen]
impl CertificateList {
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

impl From<Vec<cml_chain::certs::Certificate>> for CertificateList {
    fn from(native: Vec<cml_chain::certs::Certificate>) -> Self {
        Self(native)
    }
}

impl From<CertificateList> for Vec<cml_chain::certs::Certificate> {
    fn from(wasm: CertificateList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::certs::Certificate>> for CertificateList {
    fn as_ref(&self) -> &Vec<cml_chain::certs::Certificate> {
        &self.0
    }
}

pub type Coin = u64;

pub type DeltaCoin = Int;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Ed25519KeyHashList(Vec<cml_chain::crypto::Ed25519KeyHash>);

#[wasm_bindgen]
impl Ed25519KeyHashList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Ed25519KeyHash {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &Ed25519KeyHash) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_chain::crypto::Ed25519KeyHash>> for Ed25519KeyHashList {
    fn from(native: Vec<cml_chain::crypto::Ed25519KeyHash>) -> Self {
        Self(native)
    }
}

impl From<Ed25519KeyHashList> for Vec<cml_chain::crypto::Ed25519KeyHash> {
    fn from(wasm: Ed25519KeyHashList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::crypto::Ed25519KeyHash>> for Ed25519KeyHashList {
    fn as_ref(&self) -> &Vec<cml_chain::crypto::Ed25519KeyHash> {
        &self.0
    }
}

pub type Epoch = u64;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct GenesisHashList(Vec<cml_chain::crypto::GenesisHash>);

#[wasm_bindgen]
impl GenesisHashList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> GenesisHash {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &GenesisHash) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_chain::crypto::GenesisHash>> for GenesisHashList {
    fn from(native: Vec<cml_chain::crypto::GenesisHash>) -> Self {
        Self(native)
    }
}

impl From<GenesisHashList> for Vec<cml_chain::crypto::GenesisHash> {
    fn from(wasm: GenesisHashList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::crypto::GenesisHash>> for GenesisHashList {
    fn as_ref(&self) -> &Vec<cml_chain::crypto::GenesisHash> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct IntList(Vec<cml_chain::Int>);

#[wasm_bindgen]
impl IntList {
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

impl From<Vec<cml_chain::Int>> for IntList {
    fn from(native: Vec<cml_chain::Int>) -> Self {
        Self(native)
    }
}

impl From<IntList> for Vec<cml_chain::Int> {
    fn from(wasm: IntList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::Int>> for IntList {
    fn as_ref(&self) -> &Vec<cml_chain::Int> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapAssetNameToI64(OrderedHashMap<cml_chain::AssetName, i64>);

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
        AssetNameList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<OrderedHashMap<cml_chain::AssetName, i64>> for MapAssetNameToI64 {
    fn from(native: OrderedHashMap<cml_chain::AssetName, i64>) -> Self {
        Self(native)
    }
}

impl From<MapAssetNameToI64> for OrderedHashMap<cml_chain::AssetName, i64> {
    fn from(wasm: MapAssetNameToI64) -> Self {
        wasm.0
    }
}

impl AsRef<OrderedHashMap<cml_chain::AssetName, i64>> for MapAssetNameToI64 {
    fn as_ref(&self) -> &OrderedHashMap<cml_chain::AssetName, i64> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapAssetNameToU64(OrderedHashMap<cml_chain::AssetName, u64>);

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
        self.0.get(key.as_ref()).copied()
    }

    pub fn keys(&self) -> AssetNameList {
        AssetNameList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<OrderedHashMap<cml_chain::AssetName, u64>> for MapAssetNameToU64 {
    fn from(native: OrderedHashMap<cml_chain::AssetName, u64>) -> Self {
        Self(native)
    }
}

impl From<MapAssetNameToU64> for OrderedHashMap<cml_chain::AssetName, u64> {
    fn from(wasm: MapAssetNameToU64) -> Self {
        wasm.0
    }
}

impl AsRef<OrderedHashMap<cml_chain::AssetName, u64>> for MapAssetNameToU64 {
    fn as_ref(&self) -> &OrderedHashMap<cml_chain::AssetName, u64> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapPlutusDataToPlutusData(
    OrderedHashMap<cml_chain::plutus::PlutusData, cml_chain::plutus::PlutusData>,
);

#[wasm_bindgen]
impl MapPlutusDataToPlutusData {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &PlutusData, value: &PlutusData) -> Option<PlutusData> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &PlutusData) -> Option<PlutusData> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PlutusDataList {
        PlutusDataList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<OrderedHashMap<cml_chain::plutus::PlutusData, cml_chain::plutus::PlutusData>>
    for MapPlutusDataToPlutusData
{
    fn from(
        native: OrderedHashMap<cml_chain::plutus::PlutusData, cml_chain::plutus::PlutusData>,
    ) -> Self {
        Self(native)
    }
}

impl From<MapPlutusDataToPlutusData>
    for OrderedHashMap<cml_chain::plutus::PlutusData, cml_chain::plutus::PlutusData>
{
    fn from(wasm: MapPlutusDataToPlutusData) -> Self {
        wasm.0
    }
}

impl AsRef<OrderedHashMap<cml_chain::plutus::PlutusData, cml_chain::plutus::PlutusData>>
    for MapPlutusDataToPlutusData
{
    fn as_ref(
        &self,
    ) -> &OrderedHashMap<cml_chain::plutus::PlutusData, cml_chain::plutus::PlutusData> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapStakeCredentialToDeltaCoin(
    OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::DeltaCoin>,
);

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
        StakeCredentialList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::DeltaCoin>>
    for MapStakeCredentialToDeltaCoin
{
    fn from(
        native: OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::DeltaCoin>,
    ) -> Self {
        Self(native)
    }
}

impl From<MapStakeCredentialToDeltaCoin>
    for OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::DeltaCoin>
{
    fn from(wasm: MapStakeCredentialToDeltaCoin) -> Self {
        wasm.0
    }
}

impl AsRef<OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::DeltaCoin>>
    for MapStakeCredentialToDeltaCoin
{
    fn as_ref(&self) -> &OrderedHashMap<cml_chain::certs::StakeCredential, cml_chain::DeltaCoin> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapTransactionIndexToAuxiliaryData(
    OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::AuxiliaryData>,
);

#[wasm_bindgen]
impl MapTransactionIndexToAuxiliaryData {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: TransactionIndex,
        value: &AuxiliaryData,
    ) -> Option<AuxiliaryData> {
        self.0.insert(key, value.clone().into()).map(Into::into)
    }

    pub fn get(&self, key: TransactionIndex) -> Option<AuxiliaryData> {
        self.0.get(&key).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> Vec<TransactionIndex> {
        self.0.keys().copied().collect::<Vec<_>>()
    }
}

impl From<OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::AuxiliaryData>>
    for MapTransactionIndexToAuxiliaryData
{
    fn from(
        native: OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::AuxiliaryData>,
    ) -> Self {
        Self(native)
    }
}

impl From<MapTransactionIndexToAuxiliaryData>
    for OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::AuxiliaryData>
{
    fn from(wasm: MapTransactionIndexToAuxiliaryData) -> Self {
        wasm.0
    }
}

impl AsRef<OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::AuxiliaryData>>
    for MapTransactionIndexToAuxiliaryData
{
    fn as_ref(
        &self,
    ) -> &OrderedHashMap<cml_chain::TransactionIndex, cml_chain::auxdata::AuxiliaryData> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Mint(cml_chain::Mint);

#[wasm_bindgen]
impl Mint {
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
        PolicyIdList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<cml_chain::Mint> for Mint {
    fn from(native: cml_chain::Mint) -> Self {
        Self(native)
    }
}

impl From<Mint> for cml_chain::Mint {
    fn from(wasm: Mint) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::Mint> for Mint {
    fn as_ref(&self) -> &cml_chain::Mint {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Multiasset(cml_chain::Multiasset);

#[wasm_bindgen]
impl Multiasset {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &PolicyId,
        value: &MapAssetNameToU64,
    ) -> Option<MapAssetNameToU64> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &PolicyId) -> Option<MapAssetNameToU64> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> PolicyIdList {
        PolicyIdList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<cml_chain::Multiasset> for Multiasset {
    fn from(native: cml_chain::Multiasset) -> Self {
        Self(native)
    }
}

impl From<Multiasset> for cml_chain::Multiasset {
    fn from(wasm: Multiasset) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::Multiasset> for Multiasset {
    fn as_ref(&self) -> &cml_chain::Multiasset {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NativeScriptList(Vec<cml_chain::transaction::NativeScript>);

#[wasm_bindgen]
impl NativeScriptList {
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

impl From<Vec<cml_chain::transaction::NativeScript>> for NativeScriptList {
    fn from(native: Vec<cml_chain::transaction::NativeScript>) -> Self {
        Self(native)
    }
}

impl From<NativeScriptList> for Vec<cml_chain::transaction::NativeScript> {
    fn from(wasm: NativeScriptList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::transaction::NativeScript>> for NativeScriptList {
    fn as_ref(&self) -> &Vec<cml_chain::transaction::NativeScript> {
        &self.0
    }
}

pub type NetworkId = u8;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusDataList(Vec<cml_chain::plutus::PlutusData>);

#[wasm_bindgen]
impl PlutusDataList {
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

impl From<Vec<cml_chain::plutus::PlutusData>> for PlutusDataList {
    fn from(native: Vec<cml_chain::plutus::PlutusData>) -> Self {
        Self(native)
    }
}

impl From<PlutusDataList> for Vec<cml_chain::plutus::PlutusData> {
    fn from(wasm: PlutusDataList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::plutus::PlutusData>> for PlutusDataList {
    fn as_ref(&self) -> &Vec<cml_chain::plutus::PlutusData> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV1ScriptList(Vec<cml_chain::plutus::PlutusV1Script>);

#[wasm_bindgen]
impl PlutusV1ScriptList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV1Script {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusV1Script) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_chain::plutus::PlutusV1Script>> for PlutusV1ScriptList {
    fn from(native: Vec<cml_chain::plutus::PlutusV1Script>) -> Self {
        Self(native)
    }
}

impl From<PlutusV1ScriptList> for Vec<cml_chain::plutus::PlutusV1Script> {
    fn from(wasm: PlutusV1ScriptList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::plutus::PlutusV1Script>> for PlutusV1ScriptList {
    fn as_ref(&self) -> &Vec<cml_chain::plutus::PlutusV1Script> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PlutusV2ScriptList(Vec<cml_chain::plutus::PlutusV2Script>);

#[wasm_bindgen]
impl PlutusV2ScriptList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PlutusV2Script {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &PlutusV2Script) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_chain::plutus::PlutusV2Script>> for PlutusV2ScriptList {
    fn from(native: Vec<cml_chain::plutus::PlutusV2Script>) -> Self {
        Self(native)
    }
}

impl From<PlutusV2ScriptList> for Vec<cml_chain::plutus::PlutusV2Script> {
    fn from(wasm: PlutusV2ScriptList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::plutus::PlutusV2Script>> for PlutusV2ScriptList {
    fn as_ref(&self) -> &Vec<cml_chain::plutus::PlutusV2Script> {
        &self.0
    }
}

pub type PolicyId = ScriptHash;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PolicyIdList(Vec<cml_chain::PolicyId>);

#[wasm_bindgen]
impl PolicyIdList {
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

impl From<Vec<cml_chain::PolicyId>> for PolicyIdList {
    fn from(native: Vec<cml_chain::PolicyId>) -> Self {
        Self(native)
    }
}

impl From<PolicyIdList> for Vec<cml_chain::PolicyId> {
    fn from(wasm: PolicyIdList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::PolicyId>> for PolicyIdList {
    fn as_ref(&self) -> &Vec<cml_chain::PolicyId> {
        &self.0
    }
}

pub type Port = u16;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PositiveInterval(cml_chain::PositiveInterval);

#[wasm_bindgen]
impl PositiveInterval {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<PositiveInterval, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<PositiveInterval, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn strart(&self) -> u64 {
        self.0.strart
    }

    pub fn end(&self) -> u64 {
        self.0.end
    }

    pub fn new(strart: u64, end: u64) -> Self {
        Self(cml_chain::PositiveInterval::new(strart, end))
    }
}

impl From<cml_chain::PositiveInterval> for PositiveInterval {
    fn from(native: cml_chain::PositiveInterval) -> Self {
        Self(native)
    }
}

impl From<PositiveInterval> for cml_chain::PositiveInterval {
    fn from(wasm: PositiveInterval) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::PositiveInterval> for PositiveInterval {
    fn as_ref(&self) -> &cml_chain::PositiveInterval {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ProposedProtocolParameterUpdates(cml_chain::ProposedProtocolParameterUpdates);

#[wasm_bindgen]
impl ProposedProtocolParameterUpdates {
    pub fn new() -> Self {
        Self(OrderedHashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &GenesisHash,
        value: &ProtocolParamUpdate,
    ) -> Option<ProtocolParamUpdate> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &GenesisHash) -> Option<ProtocolParamUpdate> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> GenesisHashList {
        GenesisHashList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<cml_chain::ProposedProtocolParameterUpdates> for ProposedProtocolParameterUpdates {
    fn from(native: cml_chain::ProposedProtocolParameterUpdates) -> Self {
        Self(native)
    }
}

impl From<ProposedProtocolParameterUpdates> for cml_chain::ProposedProtocolParameterUpdates {
    fn from(wasm: ProposedProtocolParameterUpdates) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::ProposedProtocolParameterUpdates> for ProposedProtocolParameterUpdates {
    fn as_ref(&self) -> &cml_chain::ProposedProtocolParameterUpdates {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ProtocolParamUpdate(cml_chain::ProtocolParamUpdate);

#[wasm_bindgen]
impl ProtocolParamUpdate {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ProtocolParamUpdate, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ProtocolParamUpdate, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn set_minfee_a(&mut self, minfee_a: u64) {
        self.0.minfee_a = Some(minfee_a)
    }

    pub fn minfee_a(&self) -> Option<u64> {
        self.0.minfee_a
    }

    pub fn set_minfee_b(&mut self, minfee_b: u64) {
        self.0.minfee_b = Some(minfee_b)
    }

    pub fn minfee_b(&self) -> Option<u64> {
        self.0.minfee_b
    }

    pub fn set_max_block_body_size(&mut self, max_block_body_size: u64) {
        self.0.max_block_body_size = Some(max_block_body_size)
    }

    pub fn max_block_body_size(&self) -> Option<u64> {
        self.0.max_block_body_size
    }

    pub fn set_max_transaction_size(&mut self, max_transaction_size: u64) {
        self.0.max_transaction_size = Some(max_transaction_size)
    }

    pub fn max_transaction_size(&self) -> Option<u64> {
        self.0.max_transaction_size
    }

    pub fn set_max_block_header_size(&mut self, max_block_header_size: u64) {
        self.0.max_block_header_size = Some(max_block_header_size)
    }

    pub fn max_block_header_size(&self) -> Option<u64> {
        self.0.max_block_header_size
    }

    pub fn set_key_deposit(&mut self, key_deposit: Coin) {
        self.0.key_deposit = Some(key_deposit)
    }

    pub fn key_deposit(&self) -> Option<Coin> {
        self.0.key_deposit
    }

    pub fn set_pool_deposit(&mut self, pool_deposit: Coin) {
        self.0.pool_deposit = Some(pool_deposit)
    }

    pub fn pool_deposit(&self) -> Option<Coin> {
        self.0.pool_deposit
    }

    pub fn set_maximum_epoch(&mut self, maximum_epoch: Epoch) {
        self.0.maximum_epoch = Some(maximum_epoch)
    }

    pub fn maximum_epoch(&self) -> Option<Epoch> {
        self.0.maximum_epoch
    }

    pub fn set_n_opt(&mut self, n_opt: u64) {
        self.0.n_opt = Some(n_opt)
    }

    pub fn n_opt(&self) -> Option<u64> {
        self.0.n_opt
    }

    pub fn set_pool_pledge_influence(&mut self, pool_pledge_influence: &Rational) {
        self.0.pool_pledge_influence = Some(pool_pledge_influence.clone().into())
    }

    pub fn pool_pledge_influence(&self) -> Option<Rational> {
        self.0
            .pool_pledge_influence
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_expansion_rate(&mut self, expansion_rate: &UnitInterval) {
        self.0.expansion_rate = Some(expansion_rate.clone().into())
    }

    pub fn expansion_rate(&self) -> Option<UnitInterval> {
        self.0.expansion_rate.clone().map(std::convert::Into::into)
    }

    pub fn set_treasury_growth_rate(&mut self, treasury_growth_rate: &UnitInterval) {
        self.0.treasury_growth_rate = Some(treasury_growth_rate.clone().into())
    }

    pub fn treasury_growth_rate(&self) -> Option<UnitInterval> {
        self.0
            .treasury_growth_rate
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_protocol_version(&mut self, protocol_version: &ProtocolVersionStruct) {
        self.0.protocol_version = Some(protocol_version.clone().into())
    }

    pub fn protocol_version(&self) -> Option<ProtocolVersionStruct> {
        self.0
            .protocol_version
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_min_pool_cost(&mut self, min_pool_cost: Coin) {
        self.0.min_pool_cost = Some(min_pool_cost)
    }

    pub fn min_pool_cost(&self) -> Option<Coin> {
        self.0.min_pool_cost
    }

    pub fn set_ada_per_utxo_byte(&mut self, ada_per_utxo_byte: Coin) {
        self.0.ada_per_utxo_byte = Some(ada_per_utxo_byte)
    }

    pub fn ada_per_utxo_byte(&self) -> Option<Coin> {
        self.0.ada_per_utxo_byte
    }

    pub fn set_cost_models_for_script_languages(
        &mut self,
        cost_models_for_script_languages: &CostModels,
    ) {
        self.0.cost_models_for_script_languages =
            Some(cost_models_for_script_languages.clone().into())
    }

    pub fn cost_models_for_script_languages(&self) -> Option<CostModels> {
        self.0
            .cost_models_for_script_languages
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_execution_costs(&mut self, execution_costs: &ExUnitPrices) {
        self.0.execution_costs = Some(execution_costs.clone().into())
    }

    pub fn execution_costs(&self) -> Option<ExUnitPrices> {
        self.0.execution_costs.clone().map(std::convert::Into::into)
    }

    pub fn set_max_tx_ex_units(&mut self, max_tx_ex_units: &ExUnits) {
        self.0.max_tx_ex_units = Some(max_tx_ex_units.clone().into())
    }

    pub fn max_tx_ex_units(&self) -> Option<ExUnits> {
        self.0.max_tx_ex_units.clone().map(std::convert::Into::into)
    }

    pub fn set_max_block_ex_units(&mut self, max_block_ex_units: &ExUnits) {
        self.0.max_block_ex_units = Some(max_block_ex_units.clone().into())
    }

    pub fn max_block_ex_units(&self) -> Option<ExUnits> {
        self.0
            .max_block_ex_units
            .clone()
            .map(std::convert::Into::into)
    }

    pub fn set_max_value_size(&mut self, max_value_size: u64) {
        self.0.max_value_size = Some(max_value_size)
    }

    pub fn max_value_size(&self) -> Option<u64> {
        self.0.max_value_size
    }

    pub fn set_collateral_percentage(&mut self, collateral_percentage: u64) {
        self.0.collateral_percentage = Some(collateral_percentage)
    }

    pub fn collateral_percentage(&self) -> Option<u64> {
        self.0.collateral_percentage
    }

    pub fn set_max_collateral_inputs(&mut self, max_collateral_inputs: u64) {
        self.0.max_collateral_inputs = Some(max_collateral_inputs)
    }

    pub fn max_collateral_inputs(&self) -> Option<u64> {
        self.0.max_collateral_inputs
    }

    pub fn new() -> Self {
        Self(cml_chain::ProtocolParamUpdate::new())
    }
}

impl From<cml_chain::ProtocolParamUpdate> for ProtocolParamUpdate {
    fn from(native: cml_chain::ProtocolParamUpdate) -> Self {
        Self(native)
    }
}

impl From<ProtocolParamUpdate> for cml_chain::ProtocolParamUpdate {
    fn from(wasm: ProtocolParamUpdate) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::ProtocolParamUpdate> for ProtocolParamUpdate {
    fn as_ref(&self) -> &cml_chain::ProtocolParamUpdate {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ProtocolVersionStruct(cml_chain::ProtocolVersionStruct);

#[wasm_bindgen]
impl ProtocolVersionStruct {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<ProtocolVersionStruct, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ProtocolVersionStruct, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn protocol_version(&self) -> ProtocolVersion {
        self.0.protocol_version.clone().into()
    }

    pub fn new(protocol_version: &ProtocolVersion) -> Self {
        Self(cml_chain::ProtocolVersionStruct::new(
            protocol_version.clone().into(),
        ))
    }
}

impl From<cml_chain::ProtocolVersionStruct> for ProtocolVersionStruct {
    fn from(native: cml_chain::ProtocolVersionStruct) -> Self {
        Self(native)
    }
}

impl From<ProtocolVersionStruct> for cml_chain::ProtocolVersionStruct {
    fn from(wasm: ProtocolVersionStruct) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::ProtocolVersionStruct> for ProtocolVersionStruct {
    fn as_ref(&self) -> &cml_chain::ProtocolVersionStruct {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Rational(cml_chain::Rational);

#[wasm_bindgen]
impl Rational {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Rational, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Rational, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn numerator(&self) -> u64 {
        self.0.numerator
    }

    pub fn denominator(&self) -> u64 {
        self.0.denominator
    }

    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self(cml_chain::Rational::new(numerator, denominator))
    }
}

impl From<cml_chain::Rational> for Rational {
    fn from(native: cml_chain::Rational) -> Self {
        Self(native)
    }
}

impl From<Rational> for cml_chain::Rational {
    fn from(wasm: Rational) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::Rational> for Rational {
    fn as_ref(&self) -> &cml_chain::Rational {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RedeemerList(Vec<cml_chain::plutus::Redeemer>);

#[wasm_bindgen]
impl RedeemerList {
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

impl From<Vec<cml_chain::plutus::Redeemer>> for RedeemerList {
    fn from(native: Vec<cml_chain::plutus::Redeemer>) -> Self {
        Self(native)
    }
}

impl From<RedeemerList> for Vec<cml_chain::plutus::Redeemer> {
    fn from(wasm: RedeemerList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::plutus::Redeemer>> for RedeemerList {
    fn as_ref(&self) -> &Vec<cml_chain::plutus::Redeemer> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RelayList(Vec<cml_chain::certs::Relay>);

#[wasm_bindgen]
impl RelayList {
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

impl From<Vec<cml_chain::certs::Relay>> for RelayList {
    fn from(native: Vec<cml_chain::certs::Relay>) -> Self {
        Self(native)
    }
}

impl From<RelayList> for Vec<cml_chain::certs::Relay> {
    fn from(wasm: RelayList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::certs::Relay>> for RelayList {
    fn as_ref(&self) -> &Vec<cml_chain::certs::Relay> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RewardAccountList(Vec<cml_chain::address::RewardAccount>);

#[wasm_bindgen]
impl RewardAccountList {
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

impl From<Vec<cml_chain::address::RewardAccount>> for RewardAccountList {
    fn from(native: Vec<cml_chain::address::RewardAccount>) -> Self {
        Self(native)
    }
}

impl From<RewardAccountList> for Vec<cml_chain::address::RewardAccount> {
    fn from(wasm: RewardAccountList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::address::RewardAccount>> for RewardAccountList {
    fn as_ref(&self) -> &Vec<cml_chain::address::RewardAccount> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Script(cml_chain::Script);

#[wasm_bindgen]
impl Script {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Script, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Script, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_native(script: &NativeScript) -> Self {
        Self(cml_chain::Script::new_native(script.clone().into()))
    }

    pub fn new_plutus_v1(script: &PlutusV1Script) -> Self {
        Self(cml_chain::Script::new_plutus_v1(script.clone().into()))
    }

    pub fn new_plutus_v2(script: &PlutusV2Script) -> Self {
        Self(cml_chain::Script::new_plutus_v2(script.clone().into()))
    }

    pub fn kind(&self) -> ScriptKind {
        match &self.0 {
            cml_chain::Script::Native { .. } => ScriptKind::Native,
            cml_chain::Script::PlutusV1 { .. } => ScriptKind::PlutusV1,
            cml_chain::Script::PlutusV2 { .. } => ScriptKind::PlutusV2,
        }
    }

    pub fn as_native(&self) -> Option<NativeScript> {
        match &self.0 {
            cml_chain::Script::Native { script, .. } => Some(script.clone().into()),
            _ => None,
        }
    }

    pub fn as_plutus_v1(&self) -> Option<PlutusV1Script> {
        match &self.0 {
            cml_chain::Script::PlutusV1 { script, .. } => Some(script.clone().into()),
            _ => None,
        }
    }

    pub fn as_plutus_v2(&self) -> Option<PlutusV2Script> {
        match &self.0 {
            cml_chain::Script::PlutusV2 { script, .. } => Some(script.clone().into()),
            _ => None,
        }
    }
}

impl From<cml_chain::Script> for Script {
    fn from(native: cml_chain::Script) -> Self {
        Self(native)
    }
}

impl From<Script> for cml_chain::Script {
    fn from(wasm: Script) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::Script> for Script {
    fn as_ref(&self) -> &cml_chain::Script {
        &self.0
    }
}

#[wasm_bindgen]
pub enum ScriptKind {
    Native,
    PlutusV1,
    PlutusV2,
}

pub type Slot = u64;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeCredentialList(Vec<cml_chain::certs::StakeCredential>);

#[wasm_bindgen]
impl StakeCredentialList {
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

impl From<Vec<cml_chain::certs::StakeCredential>> for StakeCredentialList {
    fn from(native: Vec<cml_chain::certs::StakeCredential>) -> Self {
        Self(native)
    }
}

impl From<StakeCredentialList> for Vec<cml_chain::certs::StakeCredential> {
    fn from(wasm: StakeCredentialList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::certs::StakeCredential>> for StakeCredentialList {
    fn as_ref(&self) -> &Vec<cml_chain::certs::StakeCredential> {
        &self.0
    }
}

pub type SubCoin = PositiveInterval;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionBodyList(Vec<cml_chain::transaction::TransactionBody>);

#[wasm_bindgen]
impl TransactionBodyList {
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

impl From<Vec<cml_chain::transaction::TransactionBody>> for TransactionBodyList {
    fn from(native: Vec<cml_chain::transaction::TransactionBody>) -> Self {
        Self(native)
    }
}

impl From<TransactionBodyList> for Vec<cml_chain::transaction::TransactionBody> {
    fn from(wasm: TransactionBodyList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::transaction::TransactionBody>> for TransactionBodyList {
    fn as_ref(&self) -> &Vec<cml_chain::transaction::TransactionBody> {
        &self.0
    }
}

pub type TransactionIndex = u16;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionInputList(Vec<cml_chain::transaction::TransactionInput>);

#[wasm_bindgen]
impl TransactionInputList {
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

impl From<Vec<cml_chain::transaction::TransactionInput>> for TransactionInputList {
    fn from(native: Vec<cml_chain::transaction::TransactionInput>) -> Self {
        Self(native)
    }
}

impl From<TransactionInputList> for Vec<cml_chain::transaction::TransactionInput> {
    fn from(wasm: TransactionInputList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::transaction::TransactionInput>> for TransactionInputList {
    fn as_ref(&self) -> &Vec<cml_chain::transaction::TransactionInput> {
        &self.0
    }
}

pub type TransactionMetadatumLabel = u64;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionMetadatumList(Vec<cml_chain::auxdata::TransactionMetadatum>);

#[wasm_bindgen]
impl TransactionMetadatumList {
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

impl From<Vec<cml_chain::auxdata::TransactionMetadatum>> for TransactionMetadatumList {
    fn from(native: Vec<cml_chain::auxdata::TransactionMetadatum>) -> Self {
        Self(native)
    }
}

impl From<TransactionMetadatumList> for Vec<cml_chain::auxdata::TransactionMetadatum> {
    fn from(wasm: TransactionMetadatumList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::auxdata::TransactionMetadatum>> for TransactionMetadatumList {
    fn as_ref(&self) -> &Vec<cml_chain::auxdata::TransactionMetadatum> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionOutputList(Vec<cml_chain::transaction::TransactionOutput>);

#[wasm_bindgen]
impl TransactionOutputList {
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

impl From<Vec<cml_chain::transaction::TransactionOutput>> for TransactionOutputList {
    fn from(native: Vec<cml_chain::transaction::TransactionOutput>) -> Self {
        Self(native)
    }
}

impl From<TransactionOutputList> for Vec<cml_chain::transaction::TransactionOutput> {
    fn from(wasm: TransactionOutputList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::transaction::TransactionOutput>> for TransactionOutputList {
    fn as_ref(&self) -> &Vec<cml_chain::transaction::TransactionOutput> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TransactionWitnessSetList(Vec<cml_chain::transaction::TransactionWitnessSet>);

#[wasm_bindgen]
impl TransactionWitnessSetList {
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

impl From<Vec<cml_chain::transaction::TransactionWitnessSet>> for TransactionWitnessSetList {
    fn from(native: Vec<cml_chain::transaction::TransactionWitnessSet>) -> Self {
        Self(native)
    }
}

impl From<TransactionWitnessSetList> for Vec<cml_chain::transaction::TransactionWitnessSet> {
    fn from(wasm: TransactionWitnessSetList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::transaction::TransactionWitnessSet>> for TransactionWitnessSetList {
    fn as_ref(&self) -> &Vec<cml_chain::transaction::TransactionWitnessSet> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct UnitInterval(cml_chain::UnitInterval);

#[wasm_bindgen]
impl UnitInterval {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<UnitInterval, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<UnitInterval, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn start(&self) -> u64 {
        self.0.start
    }

    pub fn end(&self) -> u64 {
        self.0.end
    }

    pub fn new(start: u64, end: u64) -> Self {
        Self(cml_chain::UnitInterval::new(start, end))
    }
}

impl From<cml_chain::UnitInterval> for UnitInterval {
    fn from(native: cml_chain::UnitInterval) -> Self {
        Self(native)
    }
}

impl From<UnitInterval> for cml_chain::UnitInterval {
    fn from(wasm: UnitInterval) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::UnitInterval> for UnitInterval {
    fn as_ref(&self) -> &cml_chain::UnitInterval {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Update(cml_chain::Update);

#[wasm_bindgen]
impl Update {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Update, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Update, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn proposed_protocol_parameter_updates(&self) -> ProposedProtocolParameterUpdates {
        self.0.proposed_protocol_parameter_updates.clone().into()
    }

    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    pub fn new(
        proposed_protocol_parameter_updates: &ProposedProtocolParameterUpdates,
        epoch: Epoch,
    ) -> Self {
        Self(cml_chain::Update::new(
            proposed_protocol_parameter_updates.clone().into(),
            epoch,
        ))
    }
}

impl From<cml_chain::Update> for Update {
    fn from(native: cml_chain::Update) -> Self {
        Self(native)
    }
}

impl From<Update> for cml_chain::Update {
    fn from(wasm: Update) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::Update> for Update {
    fn as_ref(&self) -> &cml_chain::Update {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Value(cml_chain::Value);

#[wasm_bindgen]
impl Value {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_chain::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Value, JsValue> {
        cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Value, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn coin(&self) -> Coin {
        self.0.coin
    }

    pub fn multiasset(&self) -> Multiasset {
        self.0.multiasset.clone().into()
    }

    pub fn new(coin: Coin, multiasset: &Multiasset) -> Self {
        Self(cml_chain::Value::new(coin, multiasset.clone().into()))
    }
}

impl From<cml_chain::Value> for Value {
    fn from(native: cml_chain::Value) -> Self {
        Self(native)
    }
}

impl From<Value> for cml_chain::Value {
    fn from(wasm: Value) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::Value> for Value {
    fn as_ref(&self) -> &cml_chain::Value {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VkeywitnessList(Vec<cml_chain::crypto::Vkeywitness>);

#[wasm_bindgen]
impl VkeywitnessList {
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

impl From<Vec<cml_chain::crypto::Vkeywitness>> for VkeywitnessList {
    fn from(native: Vec<cml_chain::crypto::Vkeywitness>) -> Self {
        Self(native)
    }
}

impl From<VkeywitnessList> for Vec<cml_chain::crypto::Vkeywitness> {
    fn from(wasm: VkeywitnessList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::crypto::Vkeywitness>> for VkeywitnessList {
    fn as_ref(&self) -> &Vec<cml_chain::crypto::Vkeywitness> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Withdrawals(cml_chain::Withdrawals);

#[wasm_bindgen]
impl Withdrawals {
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
        RewardAccountList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<cml_chain::Withdrawals> for Withdrawals {
    fn from(native: cml_chain::Withdrawals) -> Self {
        Self(native)
    }
}

impl From<Withdrawals> for cml_chain::Withdrawals {
    fn from(wasm: Withdrawals) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::Withdrawals> for Withdrawals {
    fn as_ref(&self) -> &cml_chain::Withdrawals {
        &self.0
    }
}
