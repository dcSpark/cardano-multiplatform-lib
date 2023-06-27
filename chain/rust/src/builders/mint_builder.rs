use crate::builders::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};

use super::witness_builder::{RequiredWitnessSet, NativeScriptWitnessInfo};

use cml_core::ordered_hash_map::OrderedHashMap;

use crate::{
    PolicyId,
    AssetName,
    NativeScript,
    transaction::RequiredSigners,
};

#[derive(Clone)]
pub struct MintBuilderResult {
    pub policy_id: PolicyId,
    pub assets: OrderedHashMap<AssetName, i64>,
    pub aggregate_witness: Option<InputAggregateWitnessData>,
    pub required_wits: RequiredWitnessSet,
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

    pub fn new_single_asset(asset: AssetName, amount: i64) -> Self {
        let mut assets = OrderedHashMap::new();
        assets.insert(asset, amount);
        Self {
            assets,
        }
    }

    pub fn native_script(self, native_script: NativeScript, witness_info: NativeScriptWitnessInfo) -> MintBuilderResult {
        let mut required_wits = RequiredWitnessSet::default();
        let script_hash = native_script.hash();
        required_wits.add_script_hash(script_hash.clone());
        
        MintBuilderResult {
            assets: self.assets,
            policy_id: script_hash,
            aggregate_witness: Some(InputAggregateWitnessData::NativeScript(native_script, witness_info)),
            required_wits,
        }
    }

    pub fn plutus_script(self, partial_witness: PartialPlutusWitness, required_signers: RequiredSigners) -> MintBuilderResult {
        let mut required_wits = RequiredWitnessSet::default();

        let script_hash = partial_witness.script.hash();
        required_signers.iter().for_each(|required_signer| required_wits.add_vkey_key_hash(required_signer.clone()));
        required_wits.add_script_hash(script_hash.clone());

        MintBuilderResult {
            assets: self.assets,
            policy_id: script_hash,
            aggregate_witness: Some(InputAggregateWitnessData::PlutusScript(partial_witness, required_signers, None)),
            required_wits,
        }
    }
}
