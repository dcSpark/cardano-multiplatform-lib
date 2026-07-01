use super::MaryTransactionBody;
use cml_core::serialization::Serialize;
use cml_crypto::{TransactionHash, blake2b256};

impl MaryTransactionBody {
    pub fn hash(&self) -> TransactionHash {
        blake2b256(&self.to_cbor_bytes()).into()
    }
}
