use crate::*;
use crate::builders::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};

use super::witness_builder::{RequiredWitnessSet, NativeScriptWitnessInfo, PlutusScriptWitness};

#[wasm_bindgen]
#[derive(Clone)]
pub struct MintBuilderResult {
    pub(crate) policy_id: PolicyID,
    pub(crate) assets: MintAssets,
    pub(crate) aggregate_witness: Option<InputAggregateWitnessData>,
    pub(crate) required_wits: RequiredWitnessSet,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SingleMintBuilder {
    assets: MintAssets,
}

#[wasm_bindgen]
impl SingleMintBuilder {
    pub fn new(assets: &MintAssets) -> Self {
        Self {
            assets: assets.clone(),
        }
    }

    pub fn native_script(&self, native_script: &NativeScript, witness_info: &NativeScriptWitnessInfo) -> MintBuilderResult {
        let mut required_wits = RequiredWitnessSet::default();
        required_wits.add_script_hash(&native_script.hash());
        
        MintBuilderResult {
            assets: self.assets.clone(),
            policy_id: native_script.hash(),
            aggregate_witness: Some(InputAggregateWitnessData::NativeScript(native_script.clone(), witness_info.clone())),
            required_wits,
        }
    }

    pub fn plutus_script(&self, partial_witness: &PartialPlutusWitness, required_signers: &RequiredSigners) -> MintBuilderResult {
        let mut required_wits = RequiredWitnessSet::default();

        let script_hash = partial_witness.script.hash();
        
        required_signers.0.iter().for_each(|required_signer| required_wits.add_vkey_key_hash(required_signer));
        required_wits.add_script_hash(&script_hash);

        MintBuilderResult {
            assets: self.assets.clone(),
            policy_id: script_hash,
            aggregate_witness: Some(InputAggregateWitnessData::PlutusScript(partial_witness.clone(), required_signers.clone(), None)),
            required_wits,
        }
    }
}
