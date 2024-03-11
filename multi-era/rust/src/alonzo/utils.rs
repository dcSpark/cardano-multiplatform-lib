use cml_chain::{
    auxdata::{AuxiliaryData, ConwayFormatAuxData},
    transaction::TransactionWitnessSet,
};

use super::{AlonzoAuxiliaryData, AlonzoTransactionBody, AlonzoTransactionWitnessSet};

use cml_core::serialization::Serialize;
use cml_crypto::{blake2b256, TransactionHash};

impl AlonzoTransactionBody {
    pub fn hash(&self) -> TransactionHash {
        blake2b256(&self.to_cbor_bytes()).into()
    }
}

impl From<AlonzoAuxiliaryData> for AuxiliaryData {
    fn from(aux: AlonzoAuxiliaryData) -> Self {
        match aux {
            AlonzoAuxiliaryData::Shelley(md) => AuxiliaryData::new_shelley(md.clone()),
            AlonzoAuxiliaryData::ShelleyMA(md) => AuxiliaryData::new_shelley_m_a(md.clone()),
            AlonzoAuxiliaryData::Alonzo(md) => AuxiliaryData::new_conway({
                let mut conway = ConwayFormatAuxData::new();
                conway.metadata = md.metadata.clone();
                conway.native_scripts = md.native_scripts.clone();
                conway.plutus_v1_scripts = md.plutus_v1_scripts.clone();
                conway
            }),
        }
    }
}

impl From<AlonzoTransactionWitnessSet> for TransactionWitnessSet {
    fn from(wits: AlonzoTransactionWitnessSet) -> Self {
        let mut new_wits = TransactionWitnessSet::new();
        new_wits.vkeywitnesses = wits.vkeywitnesses;
        new_wits.native_scripts = wits.native_scripts;
        new_wits.bootstrap_witnesses = wits.bootstrap_witnesses;
        new_wits.redeemers = wits.redeemers;
        new_wits.plutus_datums = wits.plutus_datums;
        new_wits.plutus_v1_scripts = wits.plutus_v1_scripts;
        new_wits
    }
}
