use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use cml_core_wasm::impl_wasm_conversions;

use crate::{
    governance::{GovActionId, Voter, VotingProcedure},
    plutus::PlutusData,
    transaction::NativeScript,
    RequiredSigners,
};

use super::witness_builder::{NativeScriptWitnessInfo, PartialPlutusWitness};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VoteBuilderResult(cml_chain::builders::vote_builder::VoteBuilderResult);

impl_wasm_conversions!(
    cml_chain::builders::vote_builder::VoteBuilderResult,
    VoteBuilderResult
);

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VoteBuilder(cml_chain::builders::vote_builder::VoteBuilder);

impl_wasm_conversions!(cml_chain::builders::vote_builder::VoteBuilder, VoteBuilder);

#[wasm_bindgen]
impl VoteBuilder {
    pub fn new() -> Self {
        Self(cml_chain::builders::vote_builder::VoteBuilder::new())
    }

    pub fn with_vote(
        &self,
        voter: &Voter,
        gov_action_id: &GovActionId,
        procedure: &VotingProcedure,
    ) -> Result<VoteBuilder, JsError> {
        self.0
            .clone()
            .with_vote(
                voter.clone().into(),
                gov_action_id.clone().into(),
                procedure.clone().into(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn with_native_script_vote(
        &self,
        voter: &Voter,
        gov_action_id: &GovActionId,
        procedure: &VotingProcedure,
        native_script: NativeScript,
        witness_info: NativeScriptWitnessInfo,
    ) -> Result<VoteBuilder, JsError> {
        self.0
            .clone()
            .with_native_script_vote(
                voter.clone().into(),
                gov_action_id.clone().into(),
                procedure.clone().into(),
                native_script.clone().into(),
                witness_info.clone().into(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn with_plutus_vote(
        &self,
        voter: &Voter,
        gov_action_id: &GovActionId,
        procedure: &VotingProcedure,
        partial_witness: &PartialPlutusWitness,
        required_signers: &RequiredSigners,
        datum: &PlutusData,
    ) -> Result<VoteBuilder, JsError> {
        self.0
            .clone()
            .with_plutus_vote(
                voter.clone().into(),
                gov_action_id.clone().into(),
                procedure.clone().into(),
                partial_witness.clone().into(),
                required_signers.clone().into(),
                datum.clone().into(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn with_plutus_vote_inline_datum(
        &self,
        voter: &Voter,
        gov_action_id: &GovActionId,
        procedure: &VotingProcedure,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
    ) -> Result<VoteBuilder, JsError> {
        self.0
            .clone()
            .with_plutus_vote_inline_datum(
                voter.clone().into(),
                gov_action_id.clone().into(),
                procedure.clone().into(),
                partial_witness.clone().into(),
                required_signers.clone().into(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn build(&self) -> VoteBuilderResult {
        self.0.clone().build().into()
    }
}
