use crate::{
    crypto::hash::hash_plutus_data, governance::ProposalProcedure, plutus::PlutusData,
    transaction::NativeScript, RequiredSigners,
};

use super::{
    utils::required_wits_from_required_signers,
    witness_builder::{
        InputAggregateWitnessData, NativeScriptWitnessInfo, PartialPlutusWitness,
        RequiredWitnessSet,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum ProposalBuilderError {
    #[error("Proposal uses script. Call with_plutus_proposal() instead.")]
    ProposalIsScript,
    #[error("Proposal uses key hash. Call with_proposal() instead.")]
    ProposalIsKeyHash,
    #[error("Missing the following witnesses for the input: {0:?}")]
    MissingWitnesses(Box<RequiredWitnessSet>),
}

#[derive(Clone, Debug, Default)]
pub struct ProposalBuilderResult {
    pub proposals: Vec<ProposalProcedure>,
    pub required_wits: RequiredWitnessSet,
    pub aggregate_witnesses: Vec<InputAggregateWitnessData>,
}

#[derive(Clone, Debug)]
pub struct ProposalBuilder {
    result: ProposalBuilderResult,
}

impl Default for ProposalBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ProposalBuilder {
    pub fn new() -> Self {
        Self {
            result: ProposalBuilderResult::default(),
        }
    }

    pub fn with_proposal(
        mut self,
        proposal: ProposalProcedure,
    ) -> Result<Self, ProposalBuilderError> {
        if proposal.gov_action.script_hash().is_some() {
            return Err(ProposalBuilderError::ProposalIsScript);
        }

        self.result.proposals.push(proposal.clone());

        Ok(self)
    }

    pub fn with_native_script_proposal(
        mut self,
        proposal: ProposalProcedure,
        native_script: NativeScript,
        witness_info: NativeScriptWitnessInfo,
    ) -> Result<Self, ProposalBuilderError> {
        if let Some(script_hash) = proposal.gov_action.script_hash() {
            if *script_hash != native_script.hash() {
                let mut err_req_wits = RequiredWitnessSet::new();
                err_req_wits.add_script_hash(*script_hash);
                return Err(ProposalBuilderError::MissingWitnesses(Box::new(
                    err_req_wits,
                )));
            }
            self.result.required_wits.add_script_hash(*script_hash);
        } else {
            return Err(ProposalBuilderError::ProposalIsKeyHash);
        }

        self.result.proposals.push(proposal);

        self.result
            .aggregate_witnesses
            .push(InputAggregateWitnessData::NativeScript(
                native_script,
                witness_info,
            ));

        Ok(self)
    }

    pub fn with_plutus_proposal(
        self,
        proposal: ProposalProcedure,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
        datum: PlutusData,
    ) -> Result<Self, ProposalBuilderError> {
        self.with_plutus_proposal_impl(proposal, partial_witness, required_signers, Some(datum))
    }

    pub fn with_plutus_proposal_inline_datum(
        self,
        proposal: ProposalProcedure,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
    ) -> Result<Self, ProposalBuilderError> {
        self.with_plutus_proposal_impl(proposal, partial_witness, required_signers, None)
    }

    fn with_plutus_proposal_impl(
        mut self,
        proposal: ProposalProcedure,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
        datum: Option<PlutusData>,
    ) -> Result<Self, ProposalBuilderError> {
        let mut required_wits = required_wits_from_required_signers(&required_signers);
        if let Some(script_hash) = proposal.gov_action.script_hash() {
            required_wits.add_script_hash(*script_hash);
        } else {
            return Err(ProposalBuilderError::ProposalIsKeyHash);
        }

        let mut required_wits_left = required_wits.clone();

        // no way to know these at this time
        required_wits_left.vkeys.clear();

        let script_hash = partial_witness.script.hash();

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(&script_hash);
        if let Some(datum) = &datum {
            required_wits_left
                .plutus_data
                .remove(&hash_plutus_data(datum));
        }

        if required_wits_left.len() > 0 {
            return Err(ProposalBuilderError::MissingWitnesses(Box::new(
                required_wits_left,
            )));
        }

        self.result.proposals.push(proposal);

        self.result.required_wits.add_all(required_wits);

        self.result
            .aggregate_witnesses
            .push(InputAggregateWitnessData::PlutusScript(
                partial_witness,
                required_signers,
                datum,
            ));

        Ok(self)
    }

    pub fn build(self) -> ProposalBuilderResult {
        self.result
    }
}
