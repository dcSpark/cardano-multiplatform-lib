use crate::{
    address::RewardAddress,
    byron::ByronAddress,
    crypto::{BootstrapWitness, Vkeywitness},
    plutus::{utils::PlutusScript, LegacyRedeemer, PlutusData},
    transaction::TransactionWitnessSet,
    Ed25519KeyHashList, LegacyRedeemerList, NativeScriptList, PlutusDataList, PlutusV1ScriptList,
    PlutusV2ScriptList, Script,
};
use cml_core_wasm::impl_wasm_conversions;
use cml_crypto_wasm::{DatumHash, Ed25519KeyHash, ScriptHash};
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use super::redeemer_builder::RedeemerWitnessKey;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct PlutusScriptWitness(cml_chain::builders::witness_builder::PlutusScriptWitness);

impl_wasm_conversions!(
    cml_chain::builders::witness_builder::PlutusScriptWitness,
    PlutusScriptWitness
);

#[wasm_bindgen]
impl PlutusScriptWitness {
    pub fn new_script(script: &PlutusScript) -> Self {
        cml_chain::builders::witness_builder::PlutusScriptWitness::from(script.as_ref().clone())
            .into()
    }

    pub fn new_ref(hash: &ScriptHash) -> Self {
        cml_chain::builders::witness_builder::PlutusScriptWitness::from(*hash.as_ref()).into()
    }

    // pub fn script(&self) -> Option<PlutusScript> {
    //     match self {
    //         Self::Ref(_) => None,
    //         Self::Script(script) => Some(script.clone()),
    //     }
    // }

    pub fn hash(&self) -> ScriptHash {
        self.0.hash().into()
    }
}

/// A partial Plutus witness
/// It contains all the information needed to witness the Plutus script execution
/// except for the redeemer tag and index
/// Note: no datum is attached because only input script types have datums
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PartialPlutusWitness(cml_chain::builders::witness_builder::PartialPlutusWitness);

impl_wasm_conversions!(
    cml_chain::builders::witness_builder::PartialPlutusWitness,
    PartialPlutusWitness
);

#[wasm_bindgen]
impl PartialPlutusWitness {
    pub fn new(script: &PlutusScriptWitness, data: &PlutusData) -> Self {
        cml_chain::builders::witness_builder::PartialPlutusWitness::new(
            script.clone().into(),
            data.clone().into(),
        )
        .into()
    }

    pub fn script(&self) -> PlutusScriptWitness {
        self.0.script.clone().into()
    }

    pub fn data(&self) -> PlutusData {
        self.0.redeemer.clone().into()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct InputAggregateWitnessData(
    cml_chain::builders::witness_builder::InputAggregateWitnessData,
);

impl_wasm_conversions!(
    cml_chain::builders::witness_builder::InputAggregateWitnessData,
    InputAggregateWitnessData
);

#[wasm_bindgen]
impl InputAggregateWitnessData {
    pub fn plutus_data(&self) -> Option<PlutusData> {
        self.0.redeemer_plutus_data().map(|d| d.clone().into())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct RequiredWitnessSet(cml_chain::builders::witness_builder::RequiredWitnessSet);

impl_wasm_conversions!(
    cml_chain::builders::witness_builder::RequiredWitnessSet,
    RequiredWitnessSet
);

#[wasm_bindgen]
impl RequiredWitnessSet {
    pub fn add_vkey_key_hash(&mut self, hash: &Ed25519KeyHash) {
        self.0.add_vkey_key_hash(hash.clone().into());
    }

    pub fn add_bootstrap(&mut self, address: &ByronAddress) {
        self.0.add_bootstrap(address.clone().into());
    }

    pub fn add_script_ref(&mut self, script_hash: &ScriptHash) {
        self.0.add_script_ref(script_hash.clone().into());
    }

    pub fn add_script_hash(&mut self, script_hash: &ScriptHash) {
        self.0.add_script_hash(script_hash.clone().into());
    }

    pub fn add_plutus_datum_hash(&mut self, plutus_datum: &DatumHash) {
        self.0.add_plutus_datum_hash(plutus_datum.clone().into());
    }

    pub fn add_redeemer_tag(&mut self, redeemer: &RedeemerWitnessKey) {
        self.0.add_redeemer_tag((*redeemer).into());
    }

    pub fn add_all(&mut self, requirements: &RequiredWitnessSet) {
        self.0.add_all(requirements.clone().into());
    }

    pub fn new() -> Self {
        cml_chain::builders::witness_builder::RequiredWitnessSet::new().into()
    }

    // comes from witsVKeyNeeded in the Ledger spec
    // this is here instead of withdrawal_builder.rs due to wasm restrictions on &mut params
    pub fn withdrawal_required_wits(&mut self, address: &RewardAddress) {
        cml_chain::builders::withdrawal_builder::withdrawal_required_wits(
            address.as_ref(),
            &mut self.0,
        );
    }
}

/// Builder de-duplicates witnesses as they are added
#[wasm_bindgen]
#[derive(Clone, Default, Debug)]
pub struct TransactionWitnessSetBuilder(
    cml_chain::builders::witness_builder::TransactionWitnessSetBuilder,
);

impl_wasm_conversions!(
    cml_chain::builders::witness_builder::TransactionWitnessSetBuilder,
    TransactionWitnessSetBuilder
);

#[wasm_bindgen]
impl TransactionWitnessSetBuilder {
    pub fn add_vkey(&mut self, vkey_witness: &Vkeywitness) {
        self.0.add_vkey(vkey_witness.clone().into());
    }

    pub fn add_bootstrap(&mut self, bootstrap: &BootstrapWitness) {
        self.0.add_bootstrap(bootstrap.clone().into())
    }

    pub fn add_script(&mut self, script: &Script) {
        self.0.add_script(script.clone().into());
    }

    pub fn get_native_script(&self) -> NativeScriptList {
        self.0.get_native_script().into()
    }

    pub fn get_plutus_v1_script(&self) -> PlutusV1ScriptList {
        self.0.get_plutus_v1_script().into()
    }

    pub fn get_plutus_v2_script(&self) -> PlutusV2ScriptList {
        self.0.get_plutus_v2_script().into()
    }

    pub fn add_plutus_datum(&mut self, plutus_datum: PlutusData) {
        self.0.add_plutus_datum(plutus_datum.into());
    }

    pub fn get_plutus_datum(&self) -> PlutusDataList {
        self.0.get_plutus_datum().into()
    }

    pub fn add_redeemer(&mut self, redeemer: &LegacyRedeemer) {
        self.0.add_redeemer(redeemer.clone().into());
    }

    pub fn get_redeemer(&self) -> LegacyRedeemerList {
        self.0.get_redeemer().into()
    }

    pub fn add_required_wits(&mut self, required_wits: &RequiredWitnessSet) {
        self.0.add_required_wits(required_wits.clone().into());
    }

    pub fn new() -> Self {
        cml_chain::builders::witness_builder::TransactionWitnessSetBuilder::new().into()
    }

    pub fn add_existing(&mut self, wit_set: &TransactionWitnessSet) {
        self.0.add_existing(wit_set.clone().into());
    }

    pub fn build(&self) -> TransactionWitnessSet {
        self.0.clone().build().into()
    }

    pub fn remaining_wits(&self) -> RequiredWitnessSet {
        self.0.remaining_wits().into()
    }

    pub fn try_build(&self) -> Result<TransactionWitnessSet, JsError> {
        self.0.try_build().map(Into::into).map_err(Into::into)
    }

    pub fn merge_fake_witness(&mut self, required_wits: &RequiredWitnessSet) {
        cml_chain::builders::witness_builder::merge_fake_witness(
            &mut self.0,
            required_wits.as_ref(),
        );
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct NativeScriptWitnessInfo(cml_chain::builders::witness_builder::NativeScriptWitnessInfo);

impl_wasm_conversions!(
    cml_chain::builders::witness_builder::NativeScriptWitnessInfo,
    NativeScriptWitnessInfo
);

#[wasm_bindgen]
impl NativeScriptWitnessInfo {
    /// Unsure which keys will sign, but you know the exact number to save on tx fee
    pub fn num_signatures(num: usize) -> Self {
        cml_chain::builders::witness_builder::NativeScriptWitnessInfo::num_signatures(num).into()
    }

    /// This native script will be witnessed by exactly these keys
    pub fn vkeys(vkeys: &Ed25519KeyHashList) -> Self {
        cml_chain::builders::witness_builder::NativeScriptWitnessInfo::vkeys(vkeys.clone().into())
            .into()
    }

    /// You don't know how many keys will sign, so the maximum possible case will be assumed
    pub fn assume_signature_count() -> Self {
        cml_chain::builders::witness_builder::NativeScriptWitnessInfo::assume_signature_count()
            .into()
    }
}
