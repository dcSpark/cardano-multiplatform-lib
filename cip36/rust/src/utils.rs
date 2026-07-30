use alloc::boxed::Box;
use alloc::vec::Vec;
use cbor_event::{self, se::Serializer};

use crate::error::CIP36Error;

pub use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    ordered_hash_map::OrderedHashMap,
    serialization::{Deserialize, LenEncoding, Serialize, StringEncoding},
};

pub use cml_chain::{
    NetworkId,
    address::Address,
    auxdata::{Metadata, TransactionMetadatum},
};

use super::{
    CIP36DelegationDistribution, CIP36DeregistrationCbor, CIP36DeregistrationWitness,
    CIP36KeyDeregistration, CIP36KeyRegistration, CIP36RegistrationCbor, CIP36RegistrationWitness,
};

pub static KEY_REGISTRATION_LABEL: u64 = 61284;
pub static REGISTRATION_WITNESS_LABEL: u64 = 61285;
pub static DEREGISTRATION_WITNESS_LABEL: u64 = REGISTRATION_WITNESS_LABEL;
pub static KEY_DEREGISTRATION_LABEL: u64 = 61286;

// The generated Serialize/Deserialize for the *Cbor view types are the CBOR API: the CDDL rest
// row captures every non-CIP36 metadatum label (with encodings and wire position), so a parsed
// value round-trips the FULL metadata map byte-exactly. The to/from_metadata_bytes names are
// kept as thin aliases for API compatibility; the hand-written serializers they used to wrap
// (which silently projected down to only the CIP36 labels) are gone.

impl CIP36DeregistrationCbor {
    /// Add to an existing metadata (could be empty) the full CIP36 deregistration metadata,
    /// including any captured non-CIP36 metadatum labels (`rest`)
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), DeserializeError> {
        let dereg_metadatum =
            TransactionMetadatum::from_cbor_bytes(&self.key_deregistration.to_cbor_bytes())?;
        metadata.set(KEY_DEREGISTRATION_LABEL, dereg_metadatum);
        let witness_metadatum =
            TransactionMetadatum::from_cbor_bytes(&self.deregistration_witness.to_cbor_bytes())?;
        metadata.set(DEREGISTRATION_WITNESS_LABEL, witness_metadatum);
        for (label, datum) in self.rest.iter() {
            metadata.set(*label, datum.clone());
        }
        Ok(())
    }

    /// Serializes to bytes compatible with Metadata.
    /// If this was created from bytes or from a Metadata that was created from bytes, it will
    /// preserve the encodings of all captured labels, not just the CIP36 ones.
    /// Alias of `to_cbor_bytes()`.
    pub fn to_metadata_bytes(&self) -> Vec<u8> {
        self.to_cbor_bytes()
    }

    /// Create a CIP36 view from the bytes of a Metadata.
    /// Non-CIP36 metadatum labels are captured in `rest` and will round-trip.
    /// Alias of `from_cbor_bytes()`.
    pub fn from_metadata_bytes(metadata_cbor_bytes: &[u8]) -> Result<Self, DeserializeError> {
        Self::from_cbor_bytes(metadata_cbor_bytes)
    }
}

impl TryFrom<&Metadata> for CIP36DeregistrationCbor {
    type Error = DeserializeError;

    fn try_from(metadata: &Metadata) -> Result<Self, Self::Error> {
        use cml_core::error::Key;
        let dereg_metadatum = metadata.get(KEY_DEREGISTRATION_LABEL).ok_or_else(|| {
            DeserializeFailure::MandatoryFieldMissing(Key::Uint(KEY_DEREGISTRATION_LABEL))
        })?;
        let witness_metadatum = metadata.get(DEREGISTRATION_WITNESS_LABEL).ok_or_else(|| {
            DeserializeFailure::MandatoryFieldMissing(Key::Uint(DEREGISTRATION_WITNESS_LABEL))
        })?;
        let mut dereg_cbor = Self::new(
            CIP36KeyDeregistration::from_cbor_bytes(&dereg_metadatum.to_cbor_bytes())?,
            CIP36DeregistrationWitness::from_cbor_bytes(&witness_metadatum.to_cbor_bytes())?,
        );
        for (label, datum) in metadata.entries.iter() {
            if *label != KEY_DEREGISTRATION_LABEL && *label != DEREGISTRATION_WITNESS_LABEL {
                dereg_cbor.rest.insert(*label, datum.clone());
            }
        }
        Ok(dereg_cbor)
    }
}

impl TryInto<Metadata> for &CIP36DeregistrationCbor {
    type Error = DeserializeError;

    fn try_into(self) -> Result<Metadata, Self::Error> {
        let mut metadata = Metadata::new();
        self.add_to_metadata(&mut metadata)?;
        Ok(metadata)
    }
}

impl CIP36KeyDeregistration {
    /// Create bytes to sign to make a `CIP36DeregistrationWitness` from.
    ///
    /// # Arguments
    ///
    /// * `force_canonical` - Whether to encode the inner registration canonically. Should be true for hardware wallets and false otherwise.
    pub fn hash_to_sign(&self, force_canonical: bool) -> cbor_event::Result<Vec<u8>> {
        let mut buf = Serializer::new_vec();
        buf.write_map(cbor_event::Len::Len(1))?;
        buf.write_unsigned_integer(KEY_DEREGISTRATION_LABEL)?;
        self.serialize(&mut buf, force_canonical)?;
        let sign_data = buf.finalize();
        Ok(cml_crypto::blake2b256(&sign_data).to_vec())
    }
}

impl CIP36KeyRegistration {
    /// Create bytes to sign to make a `CIP36RegistrationWitness` from.
    ///
    /// # Arguments
    ///
    /// * `force_canonical` - Whether to encode the inner registration canonically. Should be true for hardware wallets and false otherwise.
    pub fn hash_to_sign(&self, force_canonical: bool) -> cbor_event::Result<Vec<u8>> {
        let mut buf = Serializer::new_vec();
        buf.write_map(cbor_event::Len::Len(1))?;
        buf.write_unsigned_integer(KEY_REGISTRATION_LABEL)?;
        self.serialize(&mut buf, force_canonical)?;
        let sign_data = buf.finalize();
        Ok(cml_crypto::blake2b256(&sign_data).to_vec())
    }
}

impl CIP36RegistrationCbor {
    /// Add to an existing metadata (could be empty) the full CIP36 registration metadata,
    /// including any captured non-CIP36 metadatum labels (`rest`)
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), DeserializeError> {
        self.verify()
            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)))?;
        let reg_metadatum =
            TransactionMetadatum::from_cbor_bytes(&self.key_registration.to_cbor_bytes())?;
        metadata.set(KEY_REGISTRATION_LABEL, reg_metadatum);
        let witness_metadatum =
            TransactionMetadatum::from_cbor_bytes(&self.registration_witness.to_cbor_bytes())?;
        metadata.set(REGISTRATION_WITNESS_LABEL, witness_metadatum);
        for (label, datum) in self.rest.iter() {
            metadata.set(*label, datum.clone());
        }
        Ok(())
    }

    /// Verifies invariants in CIP36.
    pub fn verify(&self) -> Result<(), CIP36Error> {
        if let CIP36DelegationDistribution::Weighted { weighted, .. } =
            &self.key_registration.delegation
        {
            if weighted.is_empty() {
                return Err(CIP36Error::EmptyDelegationArray);
            }
            // CIP-36: "The weights in the delegation array are not all zero"
            if weighted.iter().all(|d| d.weight == 0) {
                return Err(CIP36Error::DelegationWeightsZero);
            }
        }
        Ok(())
    }

    /// CIP36 invariants enforced during (de)serialization, called from
    /// cddl-codegen preserved blocks in the generated Serialize/Deserialize impls
    pub fn extra_validation(&self) -> Result<(), DeserializeError> {
        self.verify()
            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
    }

    /// Serializes to bytes compatible with Metadata.
    /// If this was created from bytes or from a Metadata that was created from bytes, it will
    /// preserve the encodings of all captured labels, not just the CIP36 ones.
    /// Alias of `to_cbor_bytes()`.
    pub fn to_metadata_bytes(&self) -> Vec<u8> {
        self.to_cbor_bytes()
    }

    /// Create a CIP36 view from the bytes of a Metadata.
    /// Non-CIP36 metadatum labels are captured in `rest` and will round-trip.
    /// Alias of `from_cbor_bytes()`.
    pub fn from_metadata_bytes(metadata_cbor_bytes: &[u8]) -> Result<Self, DeserializeError> {
        Self::from_cbor_bytes(metadata_cbor_bytes)
    }
}

impl TryFrom<&Metadata> for CIP36RegistrationCbor {
    type Error = DeserializeError;

    fn try_from(metadata: &Metadata) -> Result<Self, Self::Error> {
        use cml_core::error::Key;
        let reg_metadatum = metadata.get(KEY_REGISTRATION_LABEL).ok_or_else(|| {
            DeserializeFailure::MandatoryFieldMissing(Key::Uint(KEY_REGISTRATION_LABEL))
        })?;
        let witness_metadatum = metadata.get(REGISTRATION_WITNESS_LABEL).ok_or_else(|| {
            DeserializeFailure::MandatoryFieldMissing(Key::Uint(REGISTRATION_WITNESS_LABEL))
        })?;
        let mut reg_cbor = Self::new(
            CIP36KeyRegistration::from_cbor_bytes(&reg_metadatum.to_cbor_bytes())?,
            CIP36RegistrationWitness::from_cbor_bytes(&witness_metadatum.to_cbor_bytes())?,
        );
        for (label, datum) in metadata.entries.iter() {
            if *label != KEY_REGISTRATION_LABEL && *label != REGISTRATION_WITNESS_LABEL {
                reg_cbor.rest.insert(*label, datum.clone());
            }
        }
        Ok(reg_cbor)
    }
}

impl TryInto<Metadata> for &CIP36RegistrationCbor {
    type Error = DeserializeError;

    fn try_into(self) -> Result<Metadata, Self::Error> {
        let mut metadata = Metadata::new();
        self.add_to_metadata(&mut metadata)?;
        Ok(metadata)
    }
}
