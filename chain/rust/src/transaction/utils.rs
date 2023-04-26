use std::collections::BTreeSet;

use crate::{
    address::Address,
    transaction::{DatumOption, TransactionOutput, ScriptRef},
    Value,
};
use cml_crypto::{DatumHash, Ed25519KeyHash};

use super::{NativeScript, ShelleyTxOut, AlonzoTxOut, BabbageTxOut};

impl TransactionOutput {
    pub fn address(&self) -> &Address {
        match self {
            Self::ShelleyTxOut(tx_out) => &tx_out.address,
            Self::AlonzoTxOut(tx_out) => &tx_out.address,
            Self::BabbageTxOut(tx_out) => &tx_out.address,
        }
    }

    pub fn amount(&self) -> &Value {
        match self {
            Self::ShelleyTxOut(tx_out) => &tx_out.amount,
            Self::AlonzoTxOut(tx_out) => &tx_out.amount,
            Self::BabbageTxOut(tx_out) => &tx_out.amount,
        }
    }

    pub fn set_amount(&mut self, amount: Value) {
        match self {
            Self::ShelleyTxOut(tx_out) => tx_out.amount = amount,
            Self::AlonzoTxOut(tx_out) => tx_out.amount = amount,
            Self::BabbageTxOut(tx_out) => tx_out.amount = amount,
        }
    }

    pub fn datum(&self) -> Option<DatumOption> {
        match self {
            Self::ShelleyTxOut(_) => None,
            Self::AlonzoTxOut(tx_out) => Some(DatumOption::new_hash(tx_out.datum_hash.clone())),
            Self::BabbageTxOut(tx_out) => tx_out.datum_option.clone(),
        }
    }

    /// Get the datum hash from a tx output if present as a hash.
    /// Returns None if there is no datum, or the datum is inlined.
    /// Use TransactionOutput::datum() for inlined datums.
    pub fn datum_hash(&self) -> Option<&DatumHash> {
        match self {
            Self::ShelleyTxOut(_) => None,
            Self::AlonzoTxOut(tx_out) => Some(&tx_out.datum_hash),
            Self::BabbageTxOut(tx_out) => match &tx_out.datum_option {
                Some(DatumOption::Hash { datum_hash, .. }) => Some(datum_hash),
                _ => None,
            }
        }
    }

    pub fn script_ref(&self) -> Option<&ScriptRef> {
        match self {
            Self::ShelleyTxOut(_) |
            Self::AlonzoTxOut(_) => None,
            Self::BabbageTxOut(tx_out) => tx_out.script_reference.as_ref(),
        }
    }
}

impl From<ShelleyTxOut> for TransactionOutput {
    fn from(tx_out: ShelleyTxOut) -> Self {
        Self::ShelleyTxOut(tx_out)
    }
}

impl From<AlonzoTxOut> for TransactionOutput {
    fn from(tx_out: AlonzoTxOut) -> Self {
        Self::AlonzoTxOut(tx_out)
    }
}

impl From<BabbageTxOut> for TransactionOutput {
    fn from(tx_out: BabbageTxOut) -> Self {
        Self::BabbageTxOut(tx_out)
    }
}

pub type RequiredSignersSet = BTreeSet<Ed25519KeyHash>;

impl From<&NativeScript> for RequiredSignersSet {
    fn from(script: &NativeScript) -> Self {
        fn from_scripts(scripts: &[NativeScript]) -> RequiredSignersSet {
            scripts.iter().fold(BTreeSet::new(), |mut set, s| {
                RequiredSignersSet::from(s).iter().for_each(|pk| {
                    set.insert(pk.clone());
                });
                set
            })
        }
        match script {
            NativeScript::ScriptPubkey(spk) => {
                let mut set = BTreeSet::new();
                set.insert(spk.ed25519_key_hash.clone());
                set
            },
            NativeScript::ScriptAll(all) => {
                from_scripts(&all.native_scripts)
            },
            NativeScript::ScriptAny(any) => {
                from_scripts(&any.native_scripts)
            },
            NativeScript::ScriptNOfK(ofk) => {
                from_scripts(&ofk.native_scripts)
            },
            _ => BTreeSet::new(),
        }
    }
}

impl NativeScript {
    /// Returns an array of unique Ed25519KeyHashes
    /// contained within this script recursively on any depth level.
    /// The order of the keys in the result is not determined in any way.
    pub fn get_required_signers(&self) -> Vec<Ed25519KeyHash> {
        RequiredSignersSet::from(self).iter().map(|k| { k.clone() }).collect()
    }
}