use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use cml_core_wasm::impl_wasm_conversions;

use crate::{
    governance::ProposalProcedure, plutus::PlutusData, transaction::NativeScript, RequiredSigners,
};

use super::witness_builder::{NativeScriptWitnessInfo, PartialPlutusWitness};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct ProposalBuilderResult(cml_chain::builders::proposal_builder::ProposalBuilderResult);

impl_wasm_conversions!(
    cml_chain::builders::proposal_builder::ProposalBuilderResult,
    ProposalBuilderResult
);

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct ProposalBuilder(cml_chain::builders::proposal_builder::ProposalBuilder);

impl_wasm_conversions!(
    cml_chain::builders::proposal_builder::ProposalBuilder,
    ProposalBuilder
);

#[wasm_bindgen]
impl ProposalBuilder {
    pub fn new() -> Self {
        Self(cml_chain::builders::proposal_builder::ProposalBuilder::new())
    }

    pub fn with_proposal(&self, proposal: ProposalProcedure) -> Result<ProposalBuilder, JsError> {
        self.0
            .clone()
            .with_proposal(proposal.clone().into())
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn with_native_script_proposal(
        &self,
        proposal: ProposalProcedure,
        native_script: NativeScript,
        witness_info: NativeScriptWitnessInfo,
    ) -> Result<ProposalBuilder, JsError> {
        self.0
            .clone()
            .with_native_script_proposal(
                proposal.clone().into(),
                native_script.clone().into(),
                witness_info.clone().into(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn with_plutus_proposal(
        &self,
        proposal: &ProposalProcedure,
        partial_witness: &PartialPlutusWitness,
        required_signers: &RequiredSigners,
        datum: &PlutusData,
    ) -> Result<ProposalBuilder, JsError> {
        self.0
            .clone()
            .with_plutus_proposal(
                proposal.clone().into(),
                partial_witness.clone().into(),
                required_signers.clone().into(),
                datum.clone().into(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn with_plutus_proposal_inline_datum(
        &self,
        proposal: ProposalProcedure,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
    ) -> Result<ProposalBuilder, JsError> {
        self.0
            .clone()
            .with_plutus_proposal_inline_datum(
                proposal.clone().into(),
                partial_witness.clone().into(),
                required_signers.clone().into(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn build(&self) -> ProposalBuilderResult {
        self.0.clone().build().into()
    }
}
