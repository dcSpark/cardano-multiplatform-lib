use std::collections::BTreeSet;

use crate::{
    address::Address,
    plutus::Language,
    transaction::{DatumOption, ScriptRef, TransactionOutput},
    Value,
};
use cml_crypto::{DatumHash, Ed25519KeyHash, TransactionHash};

use super::{
    AlonzoFormatTxOut, ConwayFormatTxOut, NativeScript, TransactionBody, TransactionWitnessSet,
};

impl TransactionBody {
    pub fn hash(&self) -> TransactionHash {
        crate::crypto::hash::hash_transaction(self)
    }
}

impl TransactionOutput {
    pub fn new(
        address: Address,
        amount: Value,
        datum_option: Option<DatumOption>,
        script_reference: Option<ScriptRef>,
    ) -> Self {
        match (datum_option, script_reference) {
            (None, None) => Self::AlonzoFormatTxOut(AlonzoFormatTxOut::new(address, amount)),
            (Some(DatumOption::Hash { datum_hash, .. }), None) => {
                let mut tx_out = AlonzoFormatTxOut::new(address, amount);
                tx_out.datum_hash = Some(datum_hash);
                Self::AlonzoFormatTxOut(tx_out)
            }
            (datum, script_ref) => {
                let mut tx_out = ConwayFormatTxOut::new(address, amount);
                tx_out.datum_option = datum;
                tx_out.script_reference = script_ref;
                Self::ConwayFormatTxOut(tx_out)
            }
        }
    }

    pub fn address(&self) -> &Address {
        match self {
            Self::AlonzoFormatTxOut(tx_out) => &tx_out.address,
            Self::ConwayFormatTxOut(tx_out) => &tx_out.address,
        }
    }

    pub fn amount(&self) -> &Value {
        match self {
            Self::AlonzoFormatTxOut(tx_out) => &tx_out.amount,
            Self::ConwayFormatTxOut(tx_out) => &tx_out.amount,
        }
    }

    pub fn set_amount(&mut self, amount: Value) {
        match self {
            Self::AlonzoFormatTxOut(tx_out) => tx_out.amount = amount,
            Self::ConwayFormatTxOut(tx_out) => tx_out.amount = amount,
        }
    }

    pub fn datum(&self) -> Option<DatumOption> {
        match self {
            Self::AlonzoFormatTxOut(tx_out) => tx_out
                .datum_hash
                .as_ref()
                .map(|hash| DatumOption::new_hash(*hash)),
            Self::ConwayFormatTxOut(tx_out) => tx_out.datum_option.clone(),
        }
    }

    /// Get the datum hash from a tx output if present as a hash.
    /// Returns None if there is no datum, or the datum is inlined.
    /// Use TransactionOutput::datum() for inlined datums.
    pub fn datum_hash(&self) -> Option<&DatumHash> {
        match self {
            Self::AlonzoFormatTxOut(tx_out) => tx_out.datum_hash.as_ref(),
            Self::ConwayFormatTxOut(tx_out) => match &tx_out.datum_option {
                Some(DatumOption::Hash { datum_hash, .. }) => Some(datum_hash),
                _ => None,
            },
        }
    }

    pub fn script_ref(&self) -> Option<&ScriptRef> {
        match self {
            Self::AlonzoFormatTxOut(_) => None,
            Self::ConwayFormatTxOut(tx_out) => tx_out.script_reference.as_ref(),
        }
    }
}

impl From<AlonzoFormatTxOut> for TransactionOutput {
    fn from(tx_out: AlonzoFormatTxOut) -> Self {
        Self::AlonzoFormatTxOut(tx_out)
    }
}

impl From<ConwayFormatTxOut> for TransactionOutput {
    fn from(tx_out: ConwayFormatTxOut) -> Self {
        Self::ConwayFormatTxOut(tx_out)
    }
}

pub type RequiredSignersSet = BTreeSet<Ed25519KeyHash>;

impl From<&NativeScript> for RequiredSignersSet {
    fn from(script: &NativeScript) -> Self {
        fn from_scripts(scripts: &[NativeScript]) -> RequiredSignersSet {
            scripts.iter().fold(BTreeSet::new(), |mut set, s| {
                RequiredSignersSet::from(s).iter().for_each(|pk| {
                    set.insert(*pk);
                });
                set
            })
        }
        match script {
            NativeScript::ScriptPubkey(spk) => {
                let mut set = BTreeSet::new();
                set.insert(spk.ed25519_key_hash);
                set
            }
            NativeScript::ScriptAll(all) => from_scripts(&all.native_scripts),
            NativeScript::ScriptAny(any) => from_scripts(&any.native_scripts),
            NativeScript::ScriptNOfK(ofk) => from_scripts(&ofk.native_scripts),
            _ => BTreeSet::new(),
        }
    }
}

impl NativeScript {
    /// Returns an array of unique Ed25519KeyHashes
    /// contained within this script recursively on any depth level.
    /// The order of the keys in the result is not determined in any way.
    pub fn get_required_signers(&self) -> Vec<Ed25519KeyHash> {
        RequiredSignersSet::from(self).iter().cloned().collect()
    }
}

impl TransactionWitnessSet {
    pub fn add_all_witnesses(&mut self, other: Self) {
        // TODO: should we do duplicate checking here?
        if let Some(other_vkeys) = other.vkeywitnesses {
            if let Some(vkeys) = &mut self.vkeywitnesses {
                vkeys.extend(Vec::from(other_vkeys));
            } else {
                self.vkeywitnesses = Some(other_vkeys);
            }
        }
        if let Some(other_native_scripts) = other.native_scripts {
            if let Some(scripts) = &mut self.native_scripts {
                scripts.extend(Vec::from(other_native_scripts));
            } else {
                self.native_scripts = Some(other_native_scripts);
            }
        }
        if let Some(other_bootstraps) = other.bootstrap_witnesses {
            if let Some(bootstraps) = &mut self.bootstrap_witnesses {
                bootstraps.extend(Vec::from(other_bootstraps));
            } else {
                self.bootstrap_witnesses = Some(other_bootstraps);
            }
        }
        if let Some(other_plutus_v1_scripts) = other.plutus_v1_scripts {
            if let Some(scripts) = &mut self.plutus_v1_scripts {
                scripts.extend(Vec::from(other_plutus_v1_scripts));
            } else {
                self.plutus_v1_scripts = Some(other_plutus_v1_scripts);
            }
        }
        if let Some(other_plutus_v2_scripts) = other.plutus_v2_scripts {
            if let Some(scripts) = &mut self.plutus_v2_scripts {
                scripts.extend(Vec::from(other_plutus_v2_scripts));
            } else {
                self.plutus_v2_scripts = Some(other_plutus_v2_scripts);
            }
        }
        if let Some(other_plutus_v3_scripts) = other.plutus_v3_scripts {
            if let Some(scripts) = &mut self.plutus_v3_scripts {
                scripts.extend(Vec::from(other_plutus_v3_scripts));
            } else {
                self.plutus_v3_scripts = Some(other_plutus_v3_scripts);
            }
        }
        if let Some(other_plutus_datums) = other.plutus_datums {
            if let Some(datums) = &mut self.plutus_datums {
                datums.extend(Vec::from(other_plutus_datums));
            } else {
                self.plutus_datums = Some(other_plutus_datums);
            }
        }
        if let Some(other_redeemers) = other.redeemers {
            if let Some(redeemers) = &mut self.redeemers {
                redeemers.extend(other_redeemers);
            } else {
                self.redeemers = Some(other_redeemers);
            }
        }
    }

    pub fn languages(&self) -> Vec<Language> {
        let mut used_langs = vec![];
        if self.plutus_v1_scripts.is_some() {
            used_langs.push(Language::PlutusV1);
        }
        if self.plutus_v2_scripts.is_some() {
            used_langs.push(Language::PlutusV2);
        }
        if self.plutus_v3_scripts.is_some() {
            used_langs.push(Language::PlutusV3);
        }
        used_langs
    }
}
