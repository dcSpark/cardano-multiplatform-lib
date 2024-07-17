use crate::builders::witness_builder::{NativeScriptWitnessInfo, PartialPlutusWitness};

use cml_core_wasm::impl_wasm_conversions;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{AssetName, MapAssetNameToNonZeroInt64, NativeScript, RequiredSigners};

#[wasm_bindgen]
#[derive(Clone)]
pub struct MintBuilderResult(cml_chain::builders::mint_builder::MintBuilderResult);

impl_wasm_conversions!(
    cml_chain::builders::mint_builder::MintBuilderResult,
    MintBuilderResult
);

#[wasm_bindgen]
#[derive(Clone)]
pub struct SingleMintBuilder(cml_chain::builders::mint_builder::SingleMintBuilder);

impl_wasm_conversions!(
    cml_chain::builders::mint_builder::SingleMintBuilder,
    SingleMintBuilder
);

#[wasm_bindgen]
impl SingleMintBuilder {
    pub fn new(assets: &MapAssetNameToNonZeroInt64) -> Self {
        cml_chain::builders::mint_builder::SingleMintBuilder::new(assets.clone().into()).into()
    }

    pub fn new_single_asset(asset: &AssetName, amount: i64) -> Self {
        cml_chain::builders::mint_builder::SingleMintBuilder::new_single_asset(
            asset.clone().into(),
            amount,
        )
        .into()
    }

    pub fn native_script(
        self,
        native_script: &NativeScript,
        witness_info: &NativeScriptWitnessInfo,
    ) -> MintBuilderResult {
        self.0
            .native_script(native_script.clone().into(), witness_info.clone().into())
            .into()
    }

    pub fn plutus_script(
        self,
        partial_witness: &PartialPlutusWitness,
        required_signers: &RequiredSigners,
    ) -> MintBuilderResult {
        self.0
            .plutus_script(
                partial_witness.clone().into(),
                required_signers.clone().into(),
            )
            .into()
    }
}
