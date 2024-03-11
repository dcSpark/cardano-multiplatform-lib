use cml_chain::transaction::{NativeScript, TransactionWitnessSet};

use super::{MultisigScript, ShelleyTransactionBody, ShelleyTransactionWitnessSet};

use cml_core::serialization::Serialize;
use cml_crypto::{blake2b256, TransactionHash};

impl ShelleyTransactionBody {
    pub fn hash(&self) -> TransactionHash {
        blake2b256(&self.to_cbor_bytes()).into()
    }
}

impl From<ShelleyTransactionWitnessSet> for TransactionWitnessSet {
    fn from(wits: ShelleyTransactionWitnessSet) -> Self {
        let mut new_wits = TransactionWitnessSet::new();
        new_wits.vkeywitnesses = wits.vkeywitnesses;
        new_wits.native_scripts = wits
            .native_scripts
            .map(|native_scripts| native_scripts.into_iter().map(NativeScript::from).collect());
        new_wits.bootstrap_witnesses = wits.bootstrap_witnesses;
        new_wits
    }
}

impl From<MultisigScript> for NativeScript {
    fn from(script: MultisigScript) -> Self {
        match script {
            MultisigScript::MultisigPubkey(key) => {
                NativeScript::new_script_pubkey(key.ed25519_key_hash)
            }
            MultisigScript::MultisigAll(all) => NativeScript::new_script_all(
                all.multisig_scripts
                    .into_iter()
                    .map(NativeScript::from)
                    .collect(),
            ),
            MultisigScript::MultisigAny(any) => NativeScript::new_script_any(
                any.multisig_scripts
                    .into_iter()
                    .map(NativeScript::from)
                    .collect(),
            ),
            MultisigScript::MultisigNOfK(nok) => NativeScript::new_script_n_of_k(
                nok.n,
                nok.multisig_scripts
                    .into_iter()
                    .map(NativeScript::from)
                    .collect(),
            ),
        }
    }
}
