use crate::builders::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};

use super::witness_builder::{RequiredWitnessSet, NativeScriptWitnessInfo, PlutusScriptWitness};

use cml_core::ordered_hash_map::OrderedHashMap;

use crate::{
    PolicyId,
    AssetName,
    NativeScript,
    transaction::RequiredSigners,
};

#[derive(Clone)]
pub struct MintBuilderResult {
    pub(crate) policy_id: PolicyId,
    pub(crate) assets: OrderedHashMap<AssetName, i64>,
    pub(crate) aggregate_witness: Option<InputAggregateWitnessData>,
    pub(crate) required_wits: RequiredWitnessSet,
}

#[derive(Clone)]
pub struct SingleMintBuilder {
    assets: OrderedHashMap<AssetName, i64>,
}

impl SingleMintBuilder {
    pub fn new(assets: OrderedHashMap<AssetName, i64>) -> Self {
        Self {
            assets,
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

    pub fn plutus_script(&self, partial_witness: &PartialPlutusWitness, required_signers: RequiredSigners) -> MintBuilderResult {
        let mut required_wits = RequiredWitnessSet::default();

        let script_hash = partial_witness.script.hash();
        todo!("the line below won't work until we regenerate RequiredSigners with https://github.com/dcSpark/cddl-codegen/issues/164 fixed");
        //required_signers.iter().for_each(|required_signer| required_wits.add_vkey_key_hash(required_signer));
        required_wits.add_script_hash(&script_hash);

        MintBuilderResult {
            assets: self.assets.clone(),
            policy_id: script_hash,
            aggregate_witness: Some(InputAggregateWitnessData::PlutusScript(partial_witness.clone(), required_signers.clone(), None)),
            required_wits,
        }
    }
}
