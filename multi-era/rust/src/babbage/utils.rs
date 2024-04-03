use cml_chain::{
    auxdata::{AuxiliaryData, ConwayFormatAuxData},
    plutus::Redeemers,
    transaction::TransactionWitnessSet,
    Script,
};

use super::{
    BabbageAuxiliaryData, BabbageScript, BabbageTransactionBody, BabbageTransactionWitnessSet,
};

use cml_core::serialization::Serialize;
use cml_crypto::{blake2b256, TransactionHash};

impl BabbageTransactionBody {
    pub fn hash(&self) -> TransactionHash {
        blake2b256(&self.to_cbor_bytes()).into()
    }
}

impl From<BabbageScript> for Script {
    fn from(script: BabbageScript) -> Script {
        match script {
            BabbageScript::Native {
                script,
                len_encoding,
                tag_encoding,
            } => Script::Native {
                script,
                len_encoding,
                tag_encoding,
            },
            BabbageScript::PlutusV1 {
                script,
                len_encoding,
                tag_encoding,
            } => Script::PlutusV1 {
                script,
                len_encoding,
                tag_encoding,
            },
            BabbageScript::PlutusV2 {
                script,
                len_encoding,
                tag_encoding,
            } => Script::PlutusV2 {
                script,
                len_encoding,
                tag_encoding,
            },
        }
    }
}

impl From<BabbageAuxiliaryData> for AuxiliaryData {
    fn from(aux: BabbageAuxiliaryData) -> Self {
        match aux {
            BabbageAuxiliaryData::Shelley(md) => AuxiliaryData::new_shelley(md.clone()),
            BabbageAuxiliaryData::ShelleyMA(md) => AuxiliaryData::new_shelley_m_a(md.clone()),
            BabbageAuxiliaryData::Babbage(md) => AuxiliaryData::new_conway({
                let mut conway = ConwayFormatAuxData::new();
                conway.metadata = md.metadata.clone();
                conway.native_scripts = md.native_scripts.clone();
                conway.plutus_v1_scripts = md.plutus_v1_scripts.clone();
                conway.plutus_v2_scripts = md.plutus_v2_scripts.clone();
                conway
            }),
        }
    }
}

impl From<BabbageTransactionWitnessSet> for TransactionWitnessSet {
    fn from(wits: BabbageTransactionWitnessSet) -> Self {
        let mut new_wits = TransactionWitnessSet::new();
        new_wits.vkeywitnesses = wits.vkeywitnesses.map(Into::into);
        new_wits.native_scripts = wits.native_scripts.map(Into::into);
        new_wits.bootstrap_witnesses = wits.bootstrap_witnesses.map(Into::into);
        new_wits.redeemers = wits.redeemers.map(|rs| {
            Redeemers::new_arr_legacy_redeemer(rs.into_iter().map(Into::into).collect::<Vec<_>>())
        });
        new_wits.plutus_datums = wits.plutus_datums.map(Into::into);
        new_wits.plutus_v1_scripts = wits.plutus_v1_scripts.map(Into::into);
        new_wits.plutus_v2_scripts = wits.plutus_v2_scripts.map(Into::into);
        new_wits
    }
}
