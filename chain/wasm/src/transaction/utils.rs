use crate::{
    address::Address,
    transaction::{DatumOption, ScriptRef, TransactionOutput},
    utils::LanguageList,
    Ed25519KeyHashList, NativeScript, Value,
};
use cml_core::Slot;
use cml_crypto_wasm::{DatumHash, ScriptHash};
use wasm_bindgen::prelude::wasm_bindgen;

use super::TransactionWitnessSet;

#[wasm_bindgen]
impl TransactionOutput {
    pub fn new(
        address: &Address,
        amount: &Value,
        datum_option: Option<DatumOption>,
        script_reference: Option<ScriptRef>,
    ) -> Self {
        cml_chain::transaction::TransactionOutput::new(
            address.clone().into(),
            amount.clone().into(),
            datum_option.map(Into::into),
            script_reference.map(Into::into),
        )
        .into()
    }

    pub fn address(&self) -> Address {
        self.0.address().clone().into()
    }

    pub fn amount(&self) -> Value {
        self.0.amount().clone().into()
    }

    pub fn set_amount(&mut self, amount: &Value) {
        self.0.set_amount(amount.clone().into())
    }

    pub fn datum(&self) -> Option<DatumOption> {
        self.0.datum().map(Into::into)
    }

    /// Get the datum hash from a tx output if present as a hash.
    /// Returns None if there is no datum, or the datum is inlined.
    /// Use TransactionOutput::datum() for inlined datums.
    pub fn datum_hash(&self) -> Option<DatumHash> {
        self.0.datum_hash().cloned().map(Into::into)
    }

    pub fn script_ref(&self) -> Option<ScriptRef> {
        self.0.script_ref().cloned().map(Into::into)
    }
}

// TODO: anything here? pub type RequiredSignersSet = BTreeSet<Ed25519KeyHash>;

#[wasm_bindgen]
impl NativeScript {
    /// Returns an array of unique Ed25519KeyHashes
    /// contained within this script recursively on any depth level.
    /// The order of the keys in the result is not determined in any way.
    pub fn get_required_signers(&self) -> Ed25519KeyHashList {
        self.as_ref().get_required_signers().into()
    }

    pub fn hash(&self) -> ScriptHash {
        self.0.hash().into()
    }

    pub fn verify(
        &self,
        lower_bound: Option<Slot>,
        upper_bound: Option<Slot>,
        key_hashes: &Ed25519KeyHashList,
    ) -> bool {
        self.0.verify(lower_bound, upper_bound, key_hashes.as_ref())
    }
}

#[wasm_bindgen]
impl TransactionWitnessSet {
    pub fn add_all_witnesses(&mut self, other: &TransactionWitnessSet) {
        self.0.add_all_witnesses(other.clone().into());
    }

    pub fn languages(&self) -> LanguageList {
        self.0.languages().into()
    }
}
