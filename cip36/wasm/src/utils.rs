pub use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    ordered_hash_map::OrderedHashMap,
    serialization::{Deserialize, LenEncoding, Serialize, StringEncoding},
};

pub use cml_chain_wasm::auxdata::{Metadata, TransactionMetadatum};

pub use cml_chain_wasm::{address::Address, NetworkId};
use wasm_bindgen::JsError;

use std::convert::{TryFrom, TryInto};

use super::{
    CIP36DelegationDistribution, CIP36DeregistrationCbor, CIP36KeyDeregistration,
    CIP36KeyRegistration, CIP36Nonce, CIP36RegistrationCbor, CIP36StakeCredential,
};

impl CIP36DeregistrationCbor {
    /// Add to an existing metadata (could be empty) the full CIP36 deregistration metadata
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), JsError> {
        self.0
            .add_to_metadata(metadata.as_mut())
            .map_err(Into::into)
    }

    // these are not implementing Serialize/Deserialize as we do not keep track of the rest of the encoding metadata
    // so it would be disingenuous to implement them if users called to_cbor_bytes() and we skip the rest of
    // the metadata, as well as when creating from a Metadata object its outer encoding (e.g. map len, key encodings)
    // is not present as that is simply an OrderedHashMap<TransactionMetadatumLabel, TransactionMetadatum>

    /// Serializes to bytes compatable with Metadata, but containing ONLY the relevant fields for CIP36.
    /// If this was created from bytes or from a Metadata that was created from bytes, it will preserve
    /// the encodings but only from the metadatums themselves within the keys 61285 and 61286
    pub fn to_metadata_bytes(&self) -> Vec<u8> {
        self.0.to_metadata_bytes()
    }

    /// Create a CIP36 view from the bytes of a Metadata.
    /// The resulting CIP36DeregistrationCbor will contain ONLY the relevant fields for CIP36 from the Metadata
    pub fn from_metadata_bytes(metadata_cbor_bytes: &[u8]) -> Result<Self, DeserializeError> {
        cml_cip36::CIP36DeregistrationCbor::from_metadata_bytes(metadata_cbor_bytes)
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn try_from_metadata(metadata: &Metadata) -> Result<CIP36DeregistrationCbor, JsError> {
        cml_cip36::CIP36DeregistrationCbor::try_from(metadata.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn try_into_metadata(&self) -> Result<Metadata, JsError> {
        TryInto::<cml_chain::auxdata::Metadata>::try_into(&self.0)
            .map(Into::into)
            .map_err(Into::into)
    }
}

impl CIP36KeyDeregistration {
    /// Creates a new CIP36KeyDeregistration. You must then sign self.hash_to_sign() to make a `DeregistrationWitness`.
    ///
    /// # Arguments
    ///
    /// * `stake_credential` - stake address for the network that this transaction is submitted to (to point to the Ada that was being delegated).
    /// * `nonce` - Monotonically rising across all transactions with the same staking key. Recommended to just use the slot of this tx.
    pub fn new(stake_credential: &CIP36StakeCredential, nonce: CIP36Nonce) -> Self {
        Self(cml_cip36::CIP36KeyDeregistration::new(
            stake_credential.clone().into(),
            nonce,
        ))
    }

    /// Create bytes to sign to make a `DeregistrationWitness` from.
    ///
    /// # Arguments
    ///
    /// * `force_canonical` - Whether to encode the inner registration canonically. Should be true for hardware wallets and false otherwise.
    pub fn hash_to_sign(&self, force_canonical: bool) -> Vec<u8> {
        self.0.hash_to_sign(force_canonical).unwrap()
    }
}

impl CIP36KeyRegistration {
    /// Creates a new CIP36KeyRegistration. You must then sign self.hash_to_sign() to make a `RegistrationWitness`.
    ///
    /// # Arguments
    ///
    /// * `delegation` - Delegation
    /// * `stake_credential` - stake address for the network that this transaction is submitted to (to point to the Ada that is being delegated).
    /// * `payment_address` - Shelley oayment address discriminated for the same network this transaction is submitted to for receiving awairds.
    /// * `nonce` - Monotonically rising across all transactions with the same staking key. Recommended to just use the slot of this tx.
    pub fn new(
        delegation: &CIP36DelegationDistribution,
        stake_credential: &CIP36StakeCredential,
        payment_address: &Address,
        nonce: CIP36Nonce,
    ) -> Self {
        Self(cml_cip36::CIP36KeyRegistration::new(
            delegation.clone().into(),
            stake_credential.clone().into(),
            payment_address.clone().into(),
            nonce,
        ))
    }

    /// Create bytes to sign to make a `RegistrationWitness` from.
    ///
    /// # Arguments
    ///
    /// * `force_canonical` - Whether to encode the inner registration canonically. Should be true for hardware wallets and false otherwise.
    pub fn hash_to_sign(&self, force_canonical: bool) -> Vec<u8> {
        self.0.hash_to_sign(force_canonical).unwrap()
    }
}

impl CIP36RegistrationCbor {
    /// Add to an existing metadata (could be empty) the full CIP36 registration metadata
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), JsError> {
        self.0
            .add_to_metadata(metadata.as_mut())
            .map_err(Into::into)
    }

    /// Verifies invariants in CIP36.
    pub fn verify(&self) -> Result<(), JsError> {
        self.0.verify().map_err(Into::into)
    }

    // these are not implementing Serialize/Deserialize as we do not keep track of the rest of the encoding metadata
    // so it would be disingenuous to implement them if users called to_cbor_bytes() and we skip the rest of
    // the metadata, as well as when creating from a Metadata object its outer encoding (e.g. map len, key encodings)
    // is not present as that is simply an OrderedHashMap<TransactionMetadatumLabel, TransactionMetadatum>

    /// Serializes to bytes compatable with Metadata, but containing ONLY the relevant fields for CIP36.
    /// If this was created from bytes or from a Metadata that was created from bytes, it will preserve
    /// the encodings but only from the metadatums themselves within the keys 61284 and 61285
    pub fn to_metadata_bytes(&self) -> Vec<u8> {
        self.0.to_metadata_bytes()
    }

    /// Create a CIP36 view from the bytes of a Metadata.
    /// The resulting CIP36RegistrationCbor will contain ONLY the relevant fields for CIP36 from the Metadata
    pub fn from_metadata_bytes(metadata_cbor_bytes: &[u8]) -> Result<Self, JsError> {
        cml_cip36::CIP36RegistrationCbor::from_metadata_bytes(metadata_cbor_bytes)
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn try_from_metadata(metadata: &Metadata) -> Result<CIP36RegistrationCbor, JsError> {
        cml_cip36::CIP36RegistrationCbor::try_from(metadata.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn try_into_metadata(&self) -> Result<Metadata, JsError> {
        TryInto::<cml_chain::auxdata::Metadata>::try_into(&self.0)
            .map(Into::into)
            .map_err(Into::into)
    }
}
