use super::{
    certificate_builder::CertificateBuilderResult, input_builder::InputBuilderResult,
    mint_builder::MintBuilderResult, proposal_builder::ProposalBuilderResult,
    vote_builder::VoteBuilderResult, withdrawal_builder::WithdrawalBuilderResult,
};
use crate::plutus::{ExUnits, LegacyRedeemer, PlutusData, RedeemerTag, Redeemers};
use cml_core_wasm::impl_wasm_conversions;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

#[wasm_bindgen]
#[derive(Clone, Copy, PartialOrd, Ord, Debug, PartialEq, Eq, Hash)]
pub struct RedeemerWitnessKey(cml_chain::builders::redeemer_builder::RedeemerWitnessKey);

impl_wasm_conversions!(
    cml_chain::builders::redeemer_builder::RedeemerWitnessKey,
    RedeemerWitnessKey
);

#[wasm_bindgen]
impl RedeemerWitnessKey {
    pub fn new(tag: RedeemerTag, index: u64) -> Self {
        cml_chain::builders::redeemer_builder::RedeemerWitnessKey::new(tag, index).into()
    }

    pub fn from_redeemer(redeemer: &LegacyRedeemer) -> Self {
        cml_chain::builders::redeemer_builder::RedeemerWitnessKey::from(redeemer.as_ref()).into()
    }
}

/// Redeemer without the tag of index
/// This allows builder code to return partial redeemers
/// and then later have them placed in the right context
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct UntaggedRedeemer(cml_chain::builders::redeemer_builder::UntaggedRedeemer);

impl_wasm_conversions!(
    cml_chain::builders::redeemer_builder::UntaggedRedeemer,
    UntaggedRedeemer
);

#[wasm_bindgen]
impl UntaggedRedeemer {
    pub fn new(data: &PlutusData, ex_units: &ExUnits) -> Self {
        cml_chain::builders::redeemer_builder::UntaggedRedeemer::new(
            data.clone().into(),
            ex_units.clone().into(),
        )
        .into()
    }
}

/// In order to calculate the index from the sorted set, "add_*" methods in this builder
/// must be called along with the "add_*" methods in transaction builder.
#[wasm_bindgen]
#[derive(Clone, Default, Debug)]
pub struct RedeemerSetBuilder(cml_chain::builders::redeemer_builder::RedeemerSetBuilder);

impl_wasm_conversions!(
    cml_chain::builders::redeemer_builder::RedeemerSetBuilder,
    RedeemerSetBuilder
);

#[wasm_bindgen]
impl RedeemerSetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// note: will override existing value if called twice with the same key
    pub fn update_ex_units(&mut self, key: &RedeemerWitnessKey, ex_units: &ExUnits) {
        self.0
            .update_ex_units((*key).into(), ex_units.clone().into());
    }

    pub fn add_spend(&mut self, result: &InputBuilderResult) {
        self.0.add_spend(result.as_ref());
    }

    pub fn add_mint(&mut self, result: &MintBuilderResult) {
        self.0.add_mint(result.as_ref());
    }

    pub fn add_reward(&mut self, result: &WithdrawalBuilderResult) {
        self.0.add_reward(result.as_ref());
    }

    pub fn add_cert(&mut self, result: &CertificateBuilderResult) {
        self.0.add_cert(result.as_ref());
    }

    pub fn add_proposal(&mut self, result: &ProposalBuilderResult) {
        self.0.add_proposal(result.as_ref());
    }

    pub fn add_vote(&mut self, result: &VoteBuilderResult) {
        self.0.add_vote(result.as_ref());
    }

    pub fn build(&self, default_to_dummy_exunits: bool) -> Result<Redeemers, JsError> {
        self.0
            .build(default_to_dummy_exunits)
            .map(Into::into)
            .map_err(Into::into)
    }
}
