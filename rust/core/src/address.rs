#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Address {
    #[serde(skip)]
    pub encodings: Option<AddressEncoding>,
}

impl Address {
    pub fn new() -> Self {
        Self {
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RewardAccount {
    #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
    #[serde(skip)]
    pub encodings: Option<RewardAccountEncoding>,
}

impl RewardAccount {
    pub fn new() -> Self {
        Self {
            encodings: None,
        }
    }
}

use super::*;