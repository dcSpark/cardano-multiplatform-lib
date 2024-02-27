use cbor_event::{self, de::Deserializer, se::Serializer};

use crate::error::CIP36Error;

pub use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    ordered_hash_map::OrderedHashMap,
    serialization::{Deserialize, LenEncoding, Serialize, StringEncoding},
};

pub use cml_chain::{
    address::Address,
    auxdata::{Metadata, TransactionMetadatum},
    NetworkId,
};

use std::convert::From;

use super::{
    CIP36DelegationDistribution, CIP36DeregistrationCbor, CIP36DeregistrationWitness,
    CIP36KeyDeregistration, CIP36KeyRegistration, CIP36Nonce, CIP36RegistrationCbor,
    CIP36RegistrationWitness, CIP36StakeCredential,
};

use std::io::{BufRead, Write};

use cbor_event::Type as CBORType;

use cbor_event::Special as CBORSpecial;

pub static KEY_REGISTRATION_LABEL: u64 = 61284;
pub static REGISTRATION_WITNESS_LABEL: u64 = 61285;
pub static DEREGISTRATION_WITNESS_LABEL: u64 = REGISTRATION_WITNESS_LABEL;
pub static KEY_DEREGISTRATION_LABEL: u64 = 61286;

impl CIP36DeregistrationCbor {
    pub fn new(
        key_deregistration: CIP36KeyDeregistration,
        deregistration_witness: CIP36DeregistrationWitness,
    ) -> Self {
        Self {
            key_deregistration,
            deregistration_witness,
        }
    }

    /// Add to an existing metadata (could be empty) the full CIP36 deregistration metadata
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), DeserializeError> {
        let dereg_metadatum =
            TransactionMetadatum::from_cbor_bytes(&self.key_deregistration.to_cbor_bytes())?;
        metadata.set(KEY_DEREGISTRATION_LABEL, dereg_metadatum);
        let witness_metadatum =
            TransactionMetadatum::from_cbor_bytes(&self.deregistration_witness.to_cbor_bytes())?;
        metadata.set(DEREGISTRATION_WITNESS_LABEL, witness_metadatum);
        Ok(())
    }

    // these are not implementing Serialize/Deserialize as we do not keep track of the rest of the encoding metadata
    // so it would be disingenuous to implement them if users called to_cbor_bytes() and we skip the rest of
    // the metadata, as well as when creating from a Metadata object its outer encoding (e.g. map len, key encodings)
    // is not present as that is simply an OrderedHashMap<TransactionMetadatumLabel, TransactionMetadatum>

    /// Serializes to bytes compatable with Metadata, but containing ONLY the relevant fields for CIP36.
    /// If this was created from bytes or from a Metadata that was created from bytes, it will preserve
    /// the encodings but only from the metadatums themselves within the keys 61285 and 61286
    pub fn to_metadata_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf, false).unwrap();
        buf.finalize()
    }

    /// Create a CIP36 view from the bytes of a Metadata.
    /// The resulting CIP36DeregistrationCbor will contain ONLY the relevant fields for CIP36 from the Metadata
    pub fn from_metadata_bytes(metadata_cbor_bytes: &[u8]) -> Result<Self, DeserializeError> {
        let mut raw = Deserializer::from(std::io::Cursor::new(metadata_cbor_bytes));
        Self::deserialize(&mut raw)
    }

    /// Serializes as a Metadata structure containing ONLY the relevant fields for CIP36
    /// If this was created from bytes or from a Metadata that was created from bytes, it will preserve
    /// the encodings but only from the metadatums themselves within the keys 61285 and 61286
    /// * `force_canonical` - Whether to force canonical CBOR encodings. ONLY applies to the metadatums within labels 61285 and 61286
    pub fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(DEREGISTRATION_WITNESS_LABEL)?;
        self.deregistration_witness
            .serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer(KEY_DEREGISTRATION_LABEL)?;
        self.key_deregistration
            .serialize(serializer, force_canonical)
    }

    /// Deserializes a CIP36 view from either a Metadata or a CIP36DeregistrationCbor
    /// This contains ONLY the relevant fields for CIP36 if created from a Metadata
    pub fn deserialize<R: BufRead + std::io::Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<Self, DeserializeError> {
        use cml_core::{serialization::CBORReadLen, Key};

        let len = raw.map_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let mut deregistration_witness = None;
            let mut key_deregistration = None;
            let mut read = 0;
            while match len {
                cbor_event::LenSz::Len(n, _enc) => read < n,
                cbor_event::LenSz::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                        61285 => {
                            if deregistration_witness.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(
                                    DEREGISTRATION_WITNESS_LABEL,
                                ))
                                .into());
                            }
                            deregistration_witness =
                                Some(CIP36DeregistrationWitness::deserialize(raw).map_err(
                                    |e: DeserializeError| e.annotate("deregistration_witness"),
                                )?);
                        }
                        61286 => {
                            if key_deregistration.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(
                                    KEY_DEREGISTRATION_LABEL,
                                ))
                                .into());
                            }
                            key_deregistration =
                                Some(CIP36KeyDeregistration::deserialize(raw).map_err(
                                    |e: DeserializeError| e.annotate("key_deregistration"),
                                )?);
                        }
                        _unknown_key => (), /* ignore all other metadatum labels */
                    },
                    CBORType::Special => match len {
                        cbor_event::LenSz::Len(_, _) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => {
                        return Err(DeserializeFailure::UnexpectedKeyType(other_type).into())
                    }
                }
                read += 1;
            }
            let key_deregistration = match key_deregistration {
                Some(x) => x,
                None => {
                    return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(
                        KEY_DEREGISTRATION_LABEL,
                    ))
                    .into())
                }
            };
            let deregistration_witness = match deregistration_witness {
                Some(x) => x,
                None => {
                    return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(
                        DEREGISTRATION_WITNESS_LABEL,
                    ))
                    .into())
                }
            };
            read_len.finish()?;
            Ok(Self {
                key_deregistration,
                deregistration_witness,
            })
        })()
        .map_err(|e| e.annotate("CIP36DeregistrationCbor"))
    }
}

impl std::convert::TryFrom<&Metadata> for CIP36DeregistrationCbor {
    type Error = DeserializeError;

    fn try_from(metadata: &Metadata) -> Result<Self, Self::Error> {
        use cml_core::error::Key;
        let dereg_metadatum = metadata.get(KEY_DEREGISTRATION_LABEL).ok_or_else(|| {
            DeserializeFailure::MandatoryFieldMissing(Key::Uint(KEY_DEREGISTRATION_LABEL))
        })?;
        let witness_metadatum = metadata.get(DEREGISTRATION_WITNESS_LABEL).ok_or_else(|| {
            DeserializeFailure::MandatoryFieldMissing(Key::Uint(DEREGISTRATION_WITNESS_LABEL))
        })?;
        Ok(Self {
            key_deregistration: CIP36KeyDeregistration::from_cbor_bytes(
                &dereg_metadatum.to_cbor_bytes(),
            )?,
            deregistration_witness: CIP36DeregistrationWitness::from_cbor_bytes(
                &witness_metadatum.to_cbor_bytes(),
            )?,
        })
    }
}

impl std::convert::TryInto<Metadata> for &CIP36DeregistrationCbor {
    type Error = DeserializeError;

    fn try_into(self) -> Result<Metadata, Self::Error> {
        let mut metadata = Metadata::new();
        self.add_to_metadata(&mut metadata)?;
        Ok(metadata)
    }
}

impl CIP36KeyDeregistration {
    /// Creates a new CIP36KeyDeregistration. You must then sign self.hash_to_sign() to make a `CIP36DeregistrationWitness`.
    ///
    /// # Arguments
    ///
    /// * `stake_credential` - stake address for the network that this transaction is submitted to (to point to the Ada that was being delegated).
    /// * `nonce` - Monotonically rising across all transactions with the same staking key. Recommended to just use the slot of this tx.
    pub fn new(stake_credential: CIP36StakeCredential, nonce: CIP36Nonce) -> Self {
        Self {
            stake_credential,
            nonce,
            voting_purpose: 0,
            encodings: None,
        }
    }

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
    /// Creates a new CIP36KeyRegistration. You must then sign self.hash_to_sign() to make a `CIP36RegistrationWitness`.
    ///
    /// # Arguments
    ///
    /// * `delegation` - Delegation
    /// * `stake_credential` - stake address for the network that this transaction is submitted to (to point to the Ada that is being delegated).
    /// * `payment_address` - Shelley payment address discriminated for the same network this transaction is submitted to for receiving awairds.
    /// * `nonce` - Monotonically rising across all transactions with the same staking key. Recommended to just use the slot of this tx.
    pub fn new(
        delegation: CIP36DelegationDistribution,
        stake_credential: CIP36StakeCredential,
        payment_address: Address,
        nonce: CIP36Nonce,
    ) -> Self {
        Self {
            delegation,
            stake_credential,
            payment_address,
            nonce,
            voting_purpose: 0,
            encodings: None,
        }
    }

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
    pub fn new(
        key_registration: CIP36KeyRegistration,
        registration_witness: CIP36RegistrationWitness,
    ) -> Self {
        Self {
            key_registration,
            registration_witness,
        }
    }

    /// Add to an existing metadata (could be empty) the full CIP36 registration metadata
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), DeserializeError> {
        self.verify()
            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)))?;
        let reg_metadatum =
            TransactionMetadatum::from_cbor_bytes(&self.key_registration.to_cbor_bytes())?;
        metadata.set(KEY_REGISTRATION_LABEL, reg_metadatum);
        let witness_metadatum =
            TransactionMetadatum::from_cbor_bytes(&self.registration_witness.to_cbor_bytes())?;
        metadata.set(REGISTRATION_WITNESS_LABEL, witness_metadatum);
        Ok(())
    }

    /// Verifies invariants in CIP36.
    pub fn verify(&self) -> Result<(), CIP36Error> {
        if let CIP36DelegationDistribution::Weighted { delegations, .. } =
            &self.key_registration.delegation
        {
            if delegations.is_empty() {
                return Err(CIP36Error::EmptyDelegationArray);
            }
            if delegations.iter().any(|d| d.weight != 0) {
                return Err(CIP36Error::DelegationWeightsZero);
            }
        }
        Ok(())
    }

    // these are not implementing Serialize/Deserialize as we do not keep track of the rest of the encoding metadata
    // so it would be disingenuous to implement them if users called to_cbor_bytes() and we skip the rest of
    // the metadata, as well as when creating from a Metadata object its outer encoding (e.g. map len, key encodings)
    // is not present as that is simply an OrderedHashMap<TransactionMetadatumLabel, TransactionMetadatum>

    /// Serializes to bytes compatable with Metadata, but containing ONLY the relevant fields for CIP36.
    /// If this was created from bytes or from a Metadata that was created from bytes, it will preserve
    /// the encodings but only from the metadatums themselves within the keys 61284 and 61285
    pub fn to_metadata_bytes(&self) -> Vec<u8> {
        let mut buf = Serializer::new_vec();
        self.serialize(&mut buf, false).unwrap();
        buf.finalize()
    }

    /// Create a CIP36 view from the bytes of a Metadata.
    /// The resulting CIP36RegistrationCbor will contain ONLY the relevant fields for CIP36 from the Metadata
    pub fn from_metadata_bytes(metadata_cbor_bytes: &[u8]) -> Result<Self, DeserializeError> {
        let mut raw = Deserializer::from(std::io::Cursor::new(metadata_cbor_bytes));
        Self::deserialize(&mut raw)
    }

    /// Serializes as a Metadata structure containing ONLY the relevant fields for CIP36
    /// If this was created from bytes or from a Metadata that was created from bytes, it will preserve
    /// the encodings but only from the metadatums themselves within the keys 61284 and 61285
    /// * `force_canonical` - Whether to force canonical CBOR encodings. ONLY applies to the metadatums within labels 61285 and 61286
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.verify()
            .map_err(|e| cbor_event::Error::CustomError(e.to_string()))?;
        serializer.write_map(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(KEY_REGISTRATION_LABEL)?;
        self.key_registration
            .serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer(REGISTRATION_WITNESS_LABEL)?;
        self.registration_witness
            .serialize(serializer, force_canonical)
    }

    /// Deserializes a CIP36 view from either a Metadata or a CIP36RegistrationCbor
    /// This contains ONLY the relevant fields for CIP36 if created from a Metadata
    fn deserialize<R: BufRead + std::io::Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<Self, DeserializeError> {
        use cml_core::{error::Key, serialization::CBORReadLen};
        let len = raw.map_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let mut key_registration = None;
            let mut registration_witness = None;
            let mut read = 0;
            while match len {
                cbor_event::LenSz::Len(n, _) => read < n,
                cbor_event::LenSz::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                        61284 => {
                            if key_registration.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(
                                    KEY_REGISTRATION_LABEL,
                                ))
                                .into());
                            }
                            key_registration =
                                Some(CIP36KeyRegistration::deserialize(raw).map_err(
                                    |e: DeserializeError| e.annotate("key_registration"),
                                )?);
                        }
                        61285 => {
                            if registration_witness.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(
                                    REGISTRATION_WITNESS_LABEL,
                                ))
                                .into());
                            }
                            registration_witness =
                                Some(CIP36RegistrationWitness::deserialize(raw).map_err(
                                    |e: DeserializeError| e.annotate("registration_witness"),
                                )?);
                        }
                        _unknown_key => (), /* permissive of other metadatum labels */
                    },
                    CBORType::Text => {
                        return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into())
                    }
                    CBORType::Special => match len {
                        cbor_event::LenSz::Len(_, _) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => {
                        return Err(DeserializeFailure::UnexpectedKeyType(other_type).into())
                    }
                }
                read += 1;
            }
            let key_registration = match key_registration {
                Some(x) => x,
                None => {
                    return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(
                        KEY_REGISTRATION_LABEL,
                    ))
                    .into())
                }
            };
            let registration_witness = match registration_witness {
                Some(x) => x,
                None => {
                    return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(
                        REGISTRATION_WITNESS_LABEL,
                    ))
                    .into())
                }
            };
            read_len.finish()?;
            let reg_cbor = Self {
                key_registration,
                registration_witness,
            };
            reg_cbor
                .verify()
                .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)))?;
            Ok(reg_cbor)
        })()
        .map_err(|e| e.annotate("CIP36RegistrationCbor"))
    }
}

impl std::convert::TryFrom<&Metadata> for CIP36RegistrationCbor {
    type Error = DeserializeError;

    fn try_from(metadata: &Metadata) -> Result<Self, Self::Error> {
        use cml_core::error::Key;
        let reg_metadatum = metadata.get(KEY_REGISTRATION_LABEL).ok_or_else(|| {
            DeserializeFailure::MandatoryFieldMissing(Key::Uint(KEY_REGISTRATION_LABEL))
        })?;
        let witness_metadatum = metadata.get(REGISTRATION_WITNESS_LABEL).ok_or_else(|| {
            DeserializeFailure::MandatoryFieldMissing(Key::Uint(REGISTRATION_WITNESS_LABEL))
        })?;
        Ok(Self {
            key_registration: CIP36KeyRegistration::from_cbor_bytes(
                &reg_metadatum.to_cbor_bytes(),
            )?,
            registration_witness: CIP36RegistrationWitness::from_cbor_bytes(
                &witness_metadatum.to_cbor_bytes(),
            )?,
        })
    }
}

impl std::convert::TryInto<Metadata> for &CIP36RegistrationCbor {
    type Error = DeserializeError;

    fn try_into(self) -> Result<Metadata, Self::Error> {
        let mut metadata = Metadata::new();
        self.add_to_metadata(&mut metadata)?;
        Ok(metadata)
    }
}
