use cml_chain::{
    auxdata::{AuxiliaryData, ConwayFormatAuxData},
    plutus::{LegacyRedeemer, RedeemerTag, Redeemers},
    transaction::TransactionWitnessSet,
};

use super::{
    AlonzoAuxiliaryData, AlonzoRedeemer, AlonzoRedeemerTag, AlonzoTransactionBody,
    AlonzoTransactionWitnessSet,
};

use cml_core::non_empty::NonEmptyVec;
use cml_core::serialization::Serialize;
use cml_crypto::{TransactionHash, blake2b256};

impl AlonzoTransactionBody {
    pub fn hash(&self) -> TransactionHash {
        blake2b256(&self.to_cbor_bytes()).into()
    }
}

impl From<AlonzoAuxiliaryData> for AuxiliaryData {
    fn from(aux: AlonzoAuxiliaryData) -> Self {
        match aux {
            AlonzoAuxiliaryData::Shelley(md) => AuxiliaryData::new_shelley(md.clone()),
            AlonzoAuxiliaryData::ShelleyMA(md) => AuxiliaryData::new_shelley_ma(md.clone()),
            AlonzoAuxiliaryData::Alonzo(md) => AuxiliaryData::new_conway({
                let mut conway = ConwayFormatAuxData::new();
                conway.metadata.clone_from(&md.metadata);
                conway.native_scripts.clone_from(&md.native_scripts);
                conway.plutus_v1_scripts.clone_from(&md.plutus_v1_scripts);
                conway
            }),
        }
    }
}

impl From<AlonzoTransactionWitnessSet> for TransactionWitnessSet {
    fn from(wits: AlonzoTransactionWitnessSet) -> Self {
        let mut new_wits = TransactionWitnessSet::new();
        new_wits.vkeywitnesses = wits.vkeywitnesses.map(Into::into);
        new_wits.native_scripts = wits.native_scripts.map(Into::into);
        new_wits.bootstrap_witnesses = wits.bootstrap_witnesses.map(Into::into);
        // Conway `Redeemers` cannot be empty; an empty older-era redeemer list maps to no redeemers.
        new_wits.redeemers = wits.redeemers.and_then(|r| {
            NonEmptyVec::try_from(r.into_iter().map(Into::into).collect::<Vec<_>>())
                .ok()
                .map(Redeemers::new_arr_legacy_redeemer)
        });
        new_wits.plutus_datums = wits.plutus_datums.map(Into::into);
        new_wits.plutus_v1_scripts = wits.plutus_v1_scripts.map(Into::into);
        new_wits
    }
}

impl From<AlonzoRedeemer> for LegacyRedeemer {
    fn from(redeemer: AlonzoRedeemer) -> Self {
        Self {
            tag: match redeemer.tag {
                AlonzoRedeemerTag::Cert => RedeemerTag::Cert,
                AlonzoRedeemerTag::Mint => RedeemerTag::Mint,
                AlonzoRedeemerTag::Reward => RedeemerTag::Reward,
                AlonzoRedeemerTag::Spend => RedeemerTag::Spend,
            },
            index: redeemer.index,
            data: redeemer.data,
            ex_units: redeemer.ex_units,
            encodings: None,
        }
    }
}
