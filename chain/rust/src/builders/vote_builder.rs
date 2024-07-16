use crate::{
    crypto::hash::hash_plutus_data,
    governance::{GovActionId, Voter, VotingProcedure, VotingProcedures},
    plutus::PlutusData,
    transaction::NativeScript,
    RequiredSigners,
};

use super::{
    utils::required_wits_from_required_signers,
    witness_builder::{
        InputAggregateWitnessData, NativeScriptWitnessInfo, PartialPlutusWitness,
        RequiredWitnessSet,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum VoteBuilderError {
    #[error("Voter is script. Call with_plutus_vote() instead.")]
    VoterIsScript,
    #[error("Voter is key hash. Call with_vote() instead.")]
    VoterIsKeyHash,
    #[error("Vote already exists")]
    VoteAlreayExists,
    #[error("Missing the following witnesses for the input: {0:?}")]
    MissingWitnesses(Box<RequiredWitnessSet>),
}

#[derive(Clone, Debug, Default)]
pub struct VoteBuilderResult {
    pub votes: VotingProcedures,
    pub required_wits: RequiredWitnessSet,
    pub aggregate_witnesses: Vec<InputAggregateWitnessData>,
}

#[derive(Clone, Debug)]
pub struct VoteBuilder {
    result: VoteBuilderResult,
}

impl Default for VoteBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl VoteBuilder {
    pub fn new() -> Self {
        Self {
            result: VoteBuilderResult::default(),
        }
    }

    /// Add a vote using a voter with a key hash
    /// Will throw an error if the voter is script-hash based
    pub fn with_vote(
        mut self,
        voter: Voter,
        gov_action_id: GovActionId,
        procedure: VotingProcedure,
    ) -> Result<Self, VoteBuilderError> {
        if let Some(key_hash) = voter.key_hash() {
            self.result.required_wits.add_vkey_key_hash(*key_hash);
        } else {
            return Err(VoteBuilderError::VoterIsScript);
        }
        if self
            .result
            .votes
            .entry(voter)
            .or_default()
            .insert(gov_action_id, procedure)
            .is_some()
        {
            return Err(VoteBuilderError::VoteAlreayExists);
        }
        Ok(self)
    }

    pub fn with_native_script_vote(
        mut self,
        voter: Voter,
        gov_action_id: GovActionId,
        procedure: VotingProcedure,
        native_script: NativeScript,
        witness_info: NativeScriptWitnessInfo,
    ) -> Result<Self, VoteBuilderError> {
        if let Some(script_hash) = voter.script_hash() {
            if *script_hash != native_script.hash() {
                let mut err_req_wits = RequiredWitnessSet::new();
                err_req_wits.add_script_hash(*script_hash);
                return Err(VoteBuilderError::MissingWitnesses(Box::new(err_req_wits)));
            }
            self.result.required_wits.add_script_hash(*script_hash);
        } else {
            return Err(VoteBuilderError::VoterIsKeyHash);
        }

        if self
            .result
            .votes
            .entry(voter)
            .or_default()
            .insert(gov_action_id, procedure)
            .is_some()
        {
            return Err(VoteBuilderError::VoteAlreayExists);
        }

        self.result
            .aggregate_witnesses
            .push(InputAggregateWitnessData::NativeScript(
                native_script,
                witness_info,
            ));

        Ok(self)
    }

    pub fn with_plutus_vote(
        self,
        voter: Voter,
        gov_action_id: GovActionId,
        procedure: VotingProcedure,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
        datum: PlutusData,
    ) -> Result<Self, VoteBuilderError> {
        self.with_plutus_vote_impl(
            voter,
            gov_action_id,
            procedure,
            partial_witness,
            required_signers,
            Some(datum),
        )
    }

    pub fn with_plutus_vote_inline_datum(
        self,
        voter: Voter,
        gov_action_id: GovActionId,
        procedure: VotingProcedure,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
    ) -> Result<Self, VoteBuilderError> {
        self.with_plutus_vote_impl(
            voter,
            gov_action_id,
            procedure,
            partial_witness,
            required_signers,
            None,
        )
    }

    fn with_plutus_vote_impl(
        mut self,
        voter: Voter,
        gov_action_id: GovActionId,
        procedure: VotingProcedure,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
        datum: Option<PlutusData>,
    ) -> Result<Self, VoteBuilderError> {
        let mut required_wits = required_wits_from_required_signers(&required_signers);
        if let Some(script_hash) = voter.script_hash() {
            required_wits.add_script_hash(*script_hash);
        } else {
            return Err(VoteBuilderError::VoterIsKeyHash);
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
            return Err(VoteBuilderError::MissingWitnesses(Box::new(
                required_wits_left,
            )));
        }

        if self
            .result
            .votes
            .entry(voter)
            .or_default()
            .insert(gov_action_id, procedure)
            .is_some()
        {
            return Err(VoteBuilderError::VoteAlreayExists);
        }

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

    pub fn build(self) -> VoteBuilderResult {
        self.result
    }
}
