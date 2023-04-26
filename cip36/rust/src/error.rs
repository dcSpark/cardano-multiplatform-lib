#[derive(Debug, thiserror::Error)]
pub enum CIP36Error {
    #[error("Empty delegation array")]
    EmptyDelegationArray,
    // TODO: can we check this somehow against anything? I don't believe so, so maybe remove this
    // #[error("Reward wrong network")]
    // RewardWrongNetwork,
    #[error("Invalid delegation weights")]
    DelegationWeightsZero,
}
