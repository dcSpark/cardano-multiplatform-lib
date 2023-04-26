use super::StakeCredential;
use cml_crypto::RawBytesEncoding;

impl StakeCredential {
    // we don't implement RawBytesEncoding as from_raw_bytes() would be unable to distinguish
    pub fn to_raw_bytes(&self) -> &[u8] {
        match self {
            Self::PubKey { hash, .. } => hash.to_raw_bytes(),
            Self::Script { hash, .. } => hash.to_raw_bytes(),
        }
    }
}
