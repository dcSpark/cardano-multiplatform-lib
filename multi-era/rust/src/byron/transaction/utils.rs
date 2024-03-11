use super::ByronTx;
use cml_core::serialization::ToBytes;
use cml_crypto::{blake2b256, TransactionHash};

impl ByronTx {
    pub fn hash(&self) -> TransactionHash {
        blake2b256(&self.to_bytes()).into()
    }
}
