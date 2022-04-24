use crate::*;
use crate::builders::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};

use super::witness_builder::{RequiredWitnessSet, NativeScriptWitnessInfo, PlutusScriptWitnessInfo};

#[wasm_bindgen]
#[derive(Clone)]
pub struct MintBuilderResult {
    policy_id: PolicyID,
    assets: MintAssets,
    aggregate_witness: Option<InputAggregateWitnessData>,
    required_wits: RequiredWitnessSet,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SingleMintBuilder {
    assets: MintAssets,
}

#[wasm_bindgen]
impl SingleMintBuilder {
    pub fn new(assets: &MintAssets,) -> Self {
        Self {
            assets: assets.clone(),
        }
    }

    pub fn skip_witness(&self, policy_id: &PolicyID) -> MintBuilderResult {
        MintBuilderResult {
            assets: self.assets.clone(),
            policy_id: policy_id.clone(),
            aggregate_witness: None,
            required_wits: RequiredWitnessSet::default(),
        }
    }

    pub fn native_script(&self, native_script: &NativeScript, witness_info: &NativeScriptWitnessInfo) -> Result<MintBuilderResult, JsError> {
        Ok(MintBuilderResult {
            assets: self.assets.clone(),
            policy_id: native_script.hash(ScriptHashNamespace::NativeScript),
            aggregate_witness: Some(InputAggregateWitnessData::NativeScript(native_script.clone(), witness_info.clone())),
            required_wits: RequiredWitnessSet::default(),
        })
    }

    pub fn plutus_script(&self, partial_witness: &PartialPlutusWitness, witness_info: &PlutusScriptWitnessInfo) -> Result<MintBuilderResult, JsError> {
        // TODO: Plutus V2
        let script_hash = partial_witness.script().hash(ScriptHashNamespace::PlutusV1);
        
        Ok(MintBuilderResult {
            assets: self.assets.clone(),
            policy_id: script_hash,
            aggregate_witness: Some(InputAggregateWitnessData::PlutusScriptNoDatum(partial_witness.clone(), witness_info.clone())),
            required_wits: RequiredWitnessSet::default(),
        })
    }
}
