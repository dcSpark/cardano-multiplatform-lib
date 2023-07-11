#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]
pub mod allegra;
pub mod alonzo;
pub mod mary;
pub mod shelley;
// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cml_chain_wasm::{
    block::Block,
    certs::{StakeCredential},
    transaction::{AlonzoTxOut, ShelleyTxOut},
    Coin, TransactionIndex, StakeCredentialList,
};
use cml_core::ordered_hash_map::OrderedHashMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use crate::{
    alonzo::{
        AlonzoAuxiliaryData,
        AlonzoBlock,
        AlonzoTransactionBody,
        AlonzoTransactionWitnessSet
    },
    allegra::{
        AllegraAuxiliaryData,
        AllegraBlock,
        AllegraTransactionBody,
        AllegraTransactionWitnessSet,
    },
    mary::{
        MaryBlock,
        MaryTransactionBody
    },
    shelley::{
        ShelleyBlock,
        ShelleyTransactionBody,
        ShelleyTransactionOutput,
        ShelleyTransactionWitnessSet,
        MultisigScript,
    }
};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraTransactionBodyList(Vec<cml_multi_era::allegra::AllegraTransactionBody>);

#[wasm_bindgen]
impl AllegraTransactionBodyList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AllegraTransactionBody {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &AllegraTransactionBody) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::allegra::AllegraTransactionBody>> for AllegraTransactionBodyList {
    fn from(native: Vec<cml_multi_era::allegra::AllegraTransactionBody>) -> Self {
        Self(native)
    }
}

impl From<AllegraTransactionBodyList> for Vec<cml_multi_era::allegra::AllegraTransactionBody> {
    fn from(wasm: AllegraTransactionBodyList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::allegra::AllegraTransactionBody>> for AllegraTransactionBodyList {
    fn as_ref(&self) -> &Vec<cml_multi_era::allegra::AllegraTransactionBody> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AllegraTransactionWitnessSetList(
    Vec<cml_multi_era::allegra::AllegraTransactionWitnessSet>,
);

#[wasm_bindgen]
impl AllegraTransactionWitnessSetList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AllegraTransactionWitnessSet {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &AllegraTransactionWitnessSet) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::allegra::AllegraTransactionWitnessSet>>
    for AllegraTransactionWitnessSetList
{
    fn from(native: Vec<cml_multi_era::allegra::AllegraTransactionWitnessSet>) -> Self {
        Self(native)
    }
}

impl From<AllegraTransactionWitnessSetList>
    for Vec<cml_multi_era::allegra::AllegraTransactionWitnessSet>
{
    fn from(wasm: AllegraTransactionWitnessSetList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::allegra::AllegraTransactionWitnessSet>>
    for AllegraTransactionWitnessSetList
{
    fn as_ref(&self) -> &Vec<cml_multi_era::allegra::AllegraTransactionWitnessSet> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoTransactionBodyList(Vec<cml_multi_era::alonzo::AlonzoTransactionBody>);

#[wasm_bindgen]
impl AlonzoTransactionBodyList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AlonzoTransactionBody {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &AlonzoTransactionBody) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::alonzo::AlonzoTransactionBody>> for AlonzoTransactionBodyList {
    fn from(native: Vec<cml_multi_era::alonzo::AlonzoTransactionBody>) -> Self {
        Self(native)
    }
}

impl From<AlonzoTransactionBodyList> for Vec<cml_multi_era::alonzo::AlonzoTransactionBody> {
    fn from(wasm: AlonzoTransactionBodyList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::alonzo::AlonzoTransactionBody>> for AlonzoTransactionBodyList {
    fn as_ref(&self) -> &Vec<cml_multi_era::alonzo::AlonzoTransactionBody> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoTransactionWitnessSetList(Vec<cml_multi_era::alonzo::AlonzoTransactionWitnessSet>);

#[wasm_bindgen]
impl AlonzoTransactionWitnessSetList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AlonzoTransactionWitnessSet {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &AlonzoTransactionWitnessSet) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::alonzo::AlonzoTransactionWitnessSet>>
    for AlonzoTransactionWitnessSetList
{
    fn from(native: Vec<cml_multi_era::alonzo::AlonzoTransactionWitnessSet>) -> Self {
        Self(native)
    }
}

impl From<AlonzoTransactionWitnessSetList>
    for Vec<cml_multi_era::alonzo::AlonzoTransactionWitnessSet>
{
    fn from(wasm: AlonzoTransactionWitnessSetList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::alonzo::AlonzoTransactionWitnessSet>>
    for AlonzoTransactionWitnessSetList
{
    fn as_ref(&self) -> &Vec<cml_multi_era::alonzo::AlonzoTransactionWitnessSet> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AlonzoTxOutList(Vec<cml_chain::transaction::AlonzoTxOut>);

#[wasm_bindgen]
impl AlonzoTxOutList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> AlonzoTxOut {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &AlonzoTxOut) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_chain::transaction::AlonzoTxOut>> for AlonzoTxOutList {
    fn from(native: Vec<cml_chain::transaction::AlonzoTxOut>) -> Self {
        Self(native)
    }
}

impl From<AlonzoTxOutList> for Vec<cml_chain::transaction::AlonzoTxOut> {
    fn from(wasm: AlonzoTxOutList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::transaction::AlonzoTxOut>> for AlonzoTxOutList {
    fn as_ref(&self) -> &Vec<cml_chain::transaction::AlonzoTxOut> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MapStakeCredentialToCoin(
    OrderedHashMap<
        cml_chain::certs::StakeCredential,
        cml_chain::assets::Coin,
    >,
);

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
        self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>().into()
    }
}

impl
    From<
        OrderedHashMap<
            cml_chain::certs::StakeCredential,
            cml_chain::assets::Coin,
        >,
    > for MapStakeCredentialToCoin
{
    fn from(
        native: OrderedHashMap<
            cml_chain::certs::StakeCredential,
            cml_chain::assets::Coin,
        >,
    ) -> Self {
        Self(native)
    }
}

impl From<MapStakeCredentialToCoin>
    for OrderedHashMap<
        cml_chain::certs::StakeCredential,
        cml_chain::assets::Coin,
    >
{
    fn from(wasm: MapStakeCredentialToCoin) -> Self {
        wasm.0
    }
}

impl
    AsRef<
        OrderedHashMap<
            cml_chain::certs::StakeCredential,
            cml_chain::assets::Coin,
        >,
    > for MapStakeCredentialToCoin
{
    fn as_ref(
        &self,
    ) -> &OrderedHashMap<
        cml_chain::certs::StakeCredential,
        cml_chain::assets::Coin,
    > {
        &self.0
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

impl
    From<
        OrderedHashMap<
            cml_chain::TransactionIndex,
            cml_multi_era::allegra::AllegraAuxiliaryData,
        >,
    > for MapTransactionIndexToAllegraAuxiliaryData
{
    fn from(
        native: OrderedHashMap<
            cml_chain::TransactionIndex,
            cml_multi_era::allegra::AllegraAuxiliaryData,
        >,
    ) -> Self {
        Self(native)
    }
}

impl From<MapTransactionIndexToAllegraAuxiliaryData>
    for OrderedHashMap<
        cml_chain::TransactionIndex,
        cml_multi_era::allegra::AllegraAuxiliaryData,
    >
{
    fn from(wasm: MapTransactionIndexToAllegraAuxiliaryData) -> Self {
        wasm.0
    }
}

impl
    AsRef<
        OrderedHashMap<
            cml_chain::TransactionIndex,
            cml_multi_era::allegra::AllegraAuxiliaryData,
        >,
    > for MapTransactionIndexToAllegraAuxiliaryData
{
    fn as_ref(
        &self,
    ) -> &OrderedHashMap<
        cml_chain::TransactionIndex,
        cml_multi_era::allegra::AllegraAuxiliaryData,
    > {
        &self.0
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

impl
    From<
        OrderedHashMap<
            cml_chain::TransactionIndex,
            cml_multi_era::alonzo::AlonzoAuxiliaryData,
        >,
    > for MapTransactionIndexToAlonzoAuxiliaryData
{
    fn from(
        native: OrderedHashMap<
            cml_chain::TransactionIndex,
            cml_multi_era::alonzo::AlonzoAuxiliaryData,
        >,
    ) -> Self {
        Self(native)
    }
}

impl From<MapTransactionIndexToAlonzoAuxiliaryData>
    for OrderedHashMap<
        cml_chain::TransactionIndex,
        cml_multi_era::alonzo::AlonzoAuxiliaryData,
    >
{
    fn from(wasm: MapTransactionIndexToAlonzoAuxiliaryData) -> Self {
        wasm.0
    }
}

impl
    AsRef<
        OrderedHashMap<
            cml_chain::TransactionIndex,
            cml_multi_era::alonzo::AlonzoAuxiliaryData,
        >,
    > for MapTransactionIndexToAlonzoAuxiliaryData
{
    fn as_ref(
        &self,
    ) -> &OrderedHashMap<
        cml_chain::TransactionIndex,
        cml_multi_era::alonzo::AlonzoAuxiliaryData,
    > {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MaryTransactionBodyList(Vec<cml_multi_era::mary::MaryTransactionBody>);

#[wasm_bindgen]
impl MaryTransactionBodyList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> MaryTransactionBody {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &MaryTransactionBody) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::mary::MaryTransactionBody>> for MaryTransactionBodyList {
    fn from(native: Vec<cml_multi_era::mary::MaryTransactionBody>) -> Self {
        Self(native)
    }
}

impl From<MaryTransactionBodyList> for Vec<cml_multi_era::mary::MaryTransactionBody> {
    fn from(wasm: MaryTransactionBodyList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::mary::MaryTransactionBody>> for MaryTransactionBodyList {
    fn as_ref(&self) -> &Vec<cml_multi_era::mary::MaryTransactionBody> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultisigScriptList(Vec<cml_multi_era::shelley::MultisigScript>);

#[wasm_bindgen]
impl MultisigScriptList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> MultisigScript {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &MultisigScript) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::shelley::MultisigScript>> for MultisigScriptList {
    fn from(native: Vec<cml_multi_era::shelley::MultisigScript>) -> Self {
        Self(native)
    }
}

impl From<MultisigScriptList> for Vec<cml_multi_era::shelley::MultisigScript> {
    fn from(wasm: MultisigScriptList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::shelley::MultisigScript>> for MultisigScriptList {
    fn as_ref(&self) -> &Vec<cml_multi_era::shelley::MultisigScript> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiEraBlock(cml_multi_era::MultiEraBlock);

#[wasm_bindgen]
impl MultiEraBlock {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_core::serialization::Serialize::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<MultiEraBlock, JsValue> {
        cml_core::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
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

    pub fn from_json(json: &str) -> Result<MultiEraBlock, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
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

    pub fn new_babbage(babbage: &Block) -> Self {
        Self(cml_multi_era::MultiEraBlock::new_babbage(
            babbage.clone().into(),
        ))
    }

    pub fn kind(&self) -> MultiEraBlockKind {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Shelley(_) => MultiEraBlockKind::Shelley,
            cml_multi_era::MultiEraBlock::Allegra(_) => MultiEraBlockKind::Allegra,
            cml_multi_era::MultiEraBlock::Mary(_) => MultiEraBlockKind::Mary,
            cml_multi_era::MultiEraBlock::Alonzo(_) => MultiEraBlockKind::Alonzo,
            cml_multi_era::MultiEraBlock::Babbage(_) => MultiEraBlockKind::Babbage,
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

    pub fn as_babbage(&self) -> Option<Block> {
        match &self.0 {
            cml_multi_era::MultiEraBlock::Babbage(babbage) => Some(babbage.clone().into()),
            _ => None,
        }
    }
}

impl From<cml_multi_era::MultiEraBlock> for MultiEraBlock {
    fn from(native: cml_multi_era::MultiEraBlock) -> Self {
        Self(native)
    }
}

impl From<MultiEraBlock> for cml_multi_era::MultiEraBlock {
    fn from(wasm: MultiEraBlock) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::MultiEraBlock> for MultiEraBlock {
    fn as_ref(&self) -> &cml_multi_era::MultiEraBlock {
        &self.0
    }
}

#[wasm_bindgen]
pub enum MultiEraBlockKind {
    Shelley,
    Allegra,
    Mary,
    Alonzo,
    Babbage,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTransactionBodyList(Vec<cml_multi_era::shelley::ShelleyTransactionBody>);

#[wasm_bindgen]
impl ShelleyTransactionBodyList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ShelleyTransactionBody {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ShelleyTransactionBody) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::shelley::ShelleyTransactionBody>> for ShelleyTransactionBodyList {
    fn from(native: Vec<cml_multi_era::shelley::ShelleyTransactionBody>) -> Self {
        Self(native)
    }
}

impl From<ShelleyTransactionBodyList> for Vec<cml_multi_era::shelley::ShelleyTransactionBody> {
    fn from(wasm: ShelleyTransactionBodyList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::shelley::ShelleyTransactionBody>> for ShelleyTransactionBodyList {
    fn as_ref(&self) -> &Vec<cml_multi_era::shelley::ShelleyTransactionBody> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTransactionOutputList(Vec<cml_multi_era::shelley::ShelleyTransactionOutput>);

#[wasm_bindgen]
impl ShelleyTransactionOutputList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ShelleyTransactionOutput {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ShelleyTransactionOutput) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::shelley::ShelleyTransactionOutput>> for ShelleyTransactionOutputList {
    fn from(native: Vec<cml_multi_era::shelley::ShelleyTransactionOutput>) -> Self {
        Self(native)
    }
}

impl From<ShelleyTransactionOutputList> for Vec<cml_multi_era::shelley::ShelleyTransactionOutput> {
    fn from(wasm: ShelleyTransactionOutputList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::shelley::ShelleyTransactionOutput>> for ShelleyTransactionOutputList {
    fn as_ref(&self) -> &Vec<cml_multi_era::shelley::ShelleyTransactionOutput> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTransactionWitnessSetList(
    Vec<cml_multi_era::shelley::ShelleyTransactionWitnessSet>,
);

#[wasm_bindgen]
impl ShelleyTransactionWitnessSetList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ShelleyTransactionWitnessSet {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ShelleyTransactionWitnessSet) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::shelley::ShelleyTransactionWitnessSet>>
    for ShelleyTransactionWitnessSetList
{
    fn from(native: Vec<cml_multi_era::shelley::ShelleyTransactionWitnessSet>) -> Self {
        Self(native)
    }
}

impl From<ShelleyTransactionWitnessSetList>
    for Vec<cml_multi_era::shelley::ShelleyTransactionWitnessSet>
{
    fn from(wasm: ShelleyTransactionWitnessSetList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::shelley::ShelleyTransactionWitnessSet>>
    for ShelleyTransactionWitnessSetList
{
    fn as_ref(&self) -> &Vec<cml_multi_era::shelley::ShelleyTransactionWitnessSet> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ShelleyTxOutList(Vec<cml_chain::transaction::ShelleyTxOut>);

#[wasm_bindgen]
impl ShelleyTxOutList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> ShelleyTxOut {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &ShelleyTxOut) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_chain::transaction::ShelleyTxOut>> for ShelleyTxOutList {
    fn from(native: Vec<cml_chain::transaction::ShelleyTxOut>) -> Self {
        Self(native)
    }
}

impl From<ShelleyTxOutList> for Vec<cml_chain::transaction::ShelleyTxOut> {
    fn from(wasm: ShelleyTxOutList) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_chain::transaction::ShelleyTxOut>> for ShelleyTxOutList {
    fn as_ref(&self) -> &Vec<cml_chain::transaction::ShelleyTxOut> {
        &self.0
    }
}
