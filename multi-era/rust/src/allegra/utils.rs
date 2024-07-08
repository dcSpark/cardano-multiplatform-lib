use cml_chain::{auxdata::AuxiliaryData, transaction::TransactionWitnessSet};

use super::{AllegraAuxiliaryData, AllegraTransactionBody, AllegraTransactionWitnessSet};

use cml_core::serialization::Serialize;
use cml_crypto::{blake2b256, TransactionHash};

impl AllegraTransactionBody {
    pub fn hash(&self) -> TransactionHash {
        blake2b256(&self.to_cbor_bytes()).into()
    }
}

impl From<AllegraAuxiliaryData> for AuxiliaryData {
    fn from(aux: AllegraAuxiliaryData) -> Self {
        match aux {
            AllegraAuxiliaryData::Shelley(md) => AuxiliaryData::new_shelley(md),
            AllegraAuxiliaryData::ShelleyMA(md) => AuxiliaryData::new_shelley_ma(md),
        }
    }
}

impl From<AllegraTransactionWitnessSet> for TransactionWitnessSet {
    fn from(wits: AllegraTransactionWitnessSet) -> Self {
        let mut new_wits = TransactionWitnessSet::new();
        new_wits.vkeywitnesses = wits.vkeywitnesses.map(Into::into);
        new_wits.native_scripts = wits.native_scripts.map(Into::into);
        new_wits.bootstrap_witnesses = wits.bootstrap_witnesses.map(Into::into);
        new_wits
    }
}
