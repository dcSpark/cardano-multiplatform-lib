#![allow(clippy::too_many_arguments)]

// This library was partially code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use cardano_multiplatform_lib_chain::TransactionMetadatum;
use cardano_multiplatform_lib_core as cml_core;
use cardano_multiplatform_lib_crypto as cml_crypto;

pub use cml_core::{
    ordered_hash_map::OrderedHashMap,
    error::{DeserializeError, DeserializeFailure},
    metadata::Metadata,
    serialization::{Serialize, Deserialize, StringEncoding, LenEncoding},
};

pub use cardano_multiplatform_lib_chain::{address::RewardAddress, NetworkId};

use cbor_event::{self, de::Deserializer, se::Serializer};

use std::io::{BufRead, Write};

use cbor_event::Type as CBORType;

use cbor_event::Special as CBORSpecial;

use serialization::*;

use std::collections::BTreeMap;

use std::convert::{From, TryFrom};

pub mod serialization;

use cbor_event::Sz;

pub mod cbor_encodings;

use cbor_encodings::*;

extern crate derivative;

use derivative::Derivative;

/// To avoid linking voting keys directly with Cardano spending keys,
/// the voting key derivation path must start with a specific segment:
/// m / 1694' / 1815' / account' / chain / address_index
pub type VotingPubKey = cml_crypto::chain::ChainCrypto<cml_crypto::PublicKey>;

pub type StakingPubKey = cml_crypto::chain::ChainCrypto<cml_crypto::PublicKey>;

pub type LegacyKeyRegistration = VotingPubKey;

/// The nonce is an unsigned integer that should be monotonically rising across all transactions with the same staking key.
/// The advised way to construct a nonce is to use the current slot number.
/// This is a simple way to keep the nonce increasing without having to access the previous transaction data.
pub type Nonce = u64;

pub type StakeCredential = StakingPubKey;

pub type StakeWitness = cml_crypto::chain::ChainCrypto<cml_crypto::Ed25519Signature>;

pub type VotingPurpose = u64;

pub type Weight = u32;

pub static KEY_REGISTRATION_LABEL: u64 = 61284;
pub static REGISTRATION_WITNESS_LABEL: u64 = 61285;
pub static DEREGISTRATION_WITNESS_LABEL: u64 = REGISTRATION_WITNESS_LABEL;
pub static KEY_DEREGISTRATION_LABEL: u64 = 61286;

#[derive(Debug, thiserror::Error)]
pub enum CIP36Error {
    #[error("Empty delegation array")]
    EmptyDelegationArray,
    // TODO: can we check this somehow against anything? I don't believe so, so maybe remove this
    // #[error("Reward wrong network")]
    // RewardWrongNetwork,
    #[error("Invalid delegation weights")]
    DelegationWeightsZero,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum DelegationDistribution {
    Weighted {
        delegations: Vec<Delegation>,
        #[serde(skip)]
        delegations_encoding: LenEncoding,
    },
    LegacyKeyRegistration(LegacyKeyRegistration),
}

impl DelegationDistribution {
    /// Create a new weighted delegation. Weights are relative to all others and will be rounded down.
    /// Leftover ADA will be delegated to the last item in the array.
    pub fn new_weighted(delegations: Vec<Delegation>) -> Self {
        Self::Weighted {
            delegations,
            delegations_encoding: LenEncoding::default(),
        }
    }

    /// Delegate to a single key i.e. CIP-15.
    pub fn new_legacy_key_registration(legacy_key_registration: LegacyKeyRegistration) -> Self {
        Self::LegacyKeyRegistration(legacy_key_registration)
    }
}

/// Weighted delegation input.
/// This is the proportion of weight to assign to this public key relative to the weights
/// of all other Delegations where this is used.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Delegation {
    pub voting_pub_key: VotingPubKey,
    pub weight: Weight,
    #[serde(skip)]
    pub encodings: Option<DelegationEncoding>,
}

impl Delegation {
    pub fn new(voting_pub_key: VotingPubKey, weight: Weight) -> Self {
        Self {
            voting_pub_key,
            weight,
            encodings: None,
        }
    }
}

/// This is the entire metadata schema for CIP-36 deregistration.
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadatum
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct DeregistrationCbor {
    pub key_deregistration: KeyDeregistration,
    pub deregistration_witness: DeregistrationWitness,
}

impl DeregistrationCbor {
    pub fn new(key_deregistration: KeyDeregistration, deregistration_witness: DeregistrationWitness) -> Self {
        Self {
            key_deregistration,
            deregistration_witness,
        }
    }

    /// Add to an existing metadata (could be empty) the full CIP36 deregistration metadata
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), DeserializeError> {
        let dereg_metadatum = TransactionMetadatum::from_cbor_bytes(&self.key_deregistration.to_original_cbor_bytes())?;
        metadata.insert(KEY_DEREGISTRATION_LABEL, dereg_metadatum);
        let witness_metadatum = TransactionMetadatum::from_cbor_bytes(&self.deregistration_witness.to_original_cbor_bytes())?;
        metadata.insert(DEREGISTRATION_WITNESS_LABEL, witness_metadatum);
        Ok(())
    }

    // these are not implementing Serialize/Deserialize as we do not keep track of the rest of the encoding metadata
    // so it would be disingenuous to implement them if users called to_original_cbor_bytes() and we skip the rest of
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
    /// The resulting DeregistrationCbor will contain ONLY the relevant fields for CIP36 from the Metadata
    pub fn from_metadata_bytes(&self, metadata_cbor_bytes: &[u8]) -> Result<Self, DeserializeError> {
        let mut raw = Deserializer::from(std::io::Cursor::new(metadata_cbor_bytes));
        Self::deserialize(&mut raw)
    }

    /// Serializes as a Metadata structure containing ONLY the relevant fields for CIP36
    /// If this was created from bytes or from a Metadata that was created from bytes, it will preserve
    /// the encodings but only from the metadatums themselves within the keys 61285 and 61286
    /// * `force_canonical` - Whether to force canonical CBOR encodings. ONLY applies to the metadatums within labels 61285 and 61286
    pub fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(DEREGISTRATION_WITNESS_LABEL)?;
        self.deregistration_witness.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer(KEY_DEREGISTRATION_LABEL)?;
        self.key_deregistration.serialize(serializer, force_canonical)
    }
    

    /// Deserializes a CIP36 view from either a Metadata or a DeregistrationCbor
    /// This contains ONLY the relevant fields for CIP36 if created from a Metadata
    pub fn deserialize<R: BufRead + std::io::Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        use cml_core::{Key, serialization::CBORReadLen};

        let len = raw.map_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let mut deregistration_witness = None;
            let mut key_deregistration = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _enc) => read < n, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                        61285 => {
                            if deregistration_witness.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(DEREGISTRATION_WITNESS_LABEL)).into());
                            }
                            deregistration_witness = Some(DeregistrationWitness::deserialize(raw).map_err(|e: DeserializeError| e.annotate("deregistration_witness"))?);
                        },
                        61286 => {
                            if key_deregistration.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(KEY_DEREGISTRATION_LABEL)).into());
                            }
                            key_deregistration = Some(KeyDeregistration::deserialize(raw).map_err(|e: DeserializeError| e.annotate("key_deregistration"))?);
                        },
                        _unknown_key => ()/* ignore all other metadatum labels */,
                    },
                    CBORType::Special => match len {
                        cbor_event::LenSz::Len(_, _) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
                }
                read += 1;
            }
            let key_deregistration = match key_deregistration {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(KEY_DEREGISTRATION_LABEL)).into()),
            };
            let deregistration_witness = match deregistration_witness {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(DEREGISTRATION_WITNESS_LABEL)).into()),
            };
            read_len.finish()?;
            Ok(Self {
                key_deregistration,
                deregistration_witness,
            })
        })().map_err(|e| e.annotate("DeregistrationCbor"))
    }
}

impl std::convert::TryFrom<&Metadata> for DeregistrationCbor {
    type Error = DeserializeError;

    fn try_from(metadata: &Metadata) -> Result<Self, Self::Error> {
        use cardano_multiplatform_lib_core::error::Key;
        let dereg_metadatum = metadata
            .get(&KEY_DEREGISTRATION_LABEL)
            .ok_or_else(|| DeserializeFailure::MandatoryFieldMissing(Key::Uint(KEY_DEREGISTRATION_LABEL)))?;
        let witness_metadatum = metadata
            .get(&DEREGISTRATION_WITNESS_LABEL)
            .ok_or_else(|| DeserializeFailure::MandatoryFieldMissing(Key::Uint(DEREGISTRATION_WITNESS_LABEL)))?;
        Ok(Self {
            key_deregistration: KeyDeregistration::from_cbor_bytes(&dereg_metadatum.to_original_cbor_bytes())?,
            deregistration_witness: DeregistrationWitness::from_cbor_bytes(&witness_metadatum.to_original_cbor_bytes())?,
        })
    }
}

impl std::convert::TryInto<Metadata> for &DeregistrationCbor {
    type Error = DeserializeError;

    fn try_into(self) -> Result<Metadata, Self::Error> {
        let mut metadata = Metadata::new();
        self.add_to_metadata(&mut metadata)?;
        Ok(metadata)
    }
}


#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct DeregistrationWitness {
    pub stake_witness: StakeWitness,
    #[serde(skip)]
    pub encodings: Option<DeregistrationWitnessEncoding>,
}

impl DeregistrationWitness {
    pub fn new(stake_witness: StakeWitness) -> Self {
        Self {
            stake_witness,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct KeyDeregistration {
    pub stake_credential: StakeCredential,
    pub nonce: Nonce,
    pub voting_purpose: VotingPurpose,
    #[serde(skip)]
    pub encodings: Option<KeyDeregistrationEncoding>,
}

impl KeyDeregistration {
    /// Creates a new KeyDeregistration. You must then sign self.hash_to_sign() to make a `DeregistrationWitness`.
    ///
    /// # Arguments
    ///
    /// * `stake_credential` - stake address for the network that this transaction is submitted to (to point to the Ada that was being delegated).
    /// * `nonce` - Monotonically rising across all transactions with the same staking key. Recommended to just use the slot of this tx.
    pub fn new(stake_credential: StakeCredential, nonce: Nonce) -> Self {
        Self {
            stake_credential,
            nonce,
            voting_purpose: 0,
            encodings: None,
        }
    }

    /// Create bytes to sign to make a `DeregistrationWitness` from.
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct KeyRegistration {
    pub delegation: DelegationDistribution,
    pub stake_credential: StakeCredential,
    pub reward_address: RewardAddress,
    pub nonce: Nonce,
    pub voting_purpose: VotingPurpose,
    #[serde(skip)]
    pub encodings: Option<KeyRegistrationEncoding>,
}

impl KeyRegistration {
    /// Creates a new KeyRegistration. You must then sign self.hash_to_sign() to make a `RegistrationWitness`.
    ///
    /// # Arguments
    ///
    /// * `delegation` - Delegation
    /// * `stake_credential` - stake address for the network that this transaction is submitted to (to point to the Ada that is being delegated).
    /// * `reward_address` - Shelley address discriminated for the same network this transaction is submitted to for receiving awairds.
    /// * `nonce` - Monotonically rising across all transactions with the same staking key. Recommended to just use the slot of this tx.
    pub fn new(delegation: DelegationDistribution, stake_credential: StakeCredential, reward_address: RewardAddress, nonce: Nonce) -> Self {
        Self {
            delegation,
            stake_credential,
            reward_address,
            nonce,
            voting_purpose: 0,
            encodings: None,
        }
    }

    /// Create bytes to sign to make a `RegistrationWitness` from.
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

/// This is the entire metadata schema for CIP-36 registration.
/// It can be parsed by passing in the CBOR bytes of the entire transaction metadatum
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct RegistrationCbor {
    pub key_registration: KeyRegistration,
    pub registration_witness: RegistrationWitness,
}

impl RegistrationCbor {
    pub fn new(key_registration: KeyRegistration, registration_witness: RegistrationWitness) -> Self {
        Self {
            key_registration,
            registration_witness,
        }
    }

    /// Add to an existing metadata (could be empty) the full CIP36 registration metadata
    pub fn add_to_metadata(&self, metadata: &mut Metadata) -> Result<(), DeserializeError> {
        self.verify().map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)))?;
        let reg_metadatum = TransactionMetadatum::from_cbor_bytes(&self.key_registration.to_original_cbor_bytes())?;
        metadata.insert(KEY_REGISTRATION_LABEL, reg_metadatum);
        let witness_metadatum = TransactionMetadatum::from_cbor_bytes(&self.registration_witness.to_original_cbor_bytes())?;
        metadata.insert(REGISTRATION_WITNESS_LABEL, witness_metadatum);
        Ok(())
    }

    /// Verifies invariants in CIP36.
    pub fn verify(&self) -> Result<(), CIP36Error> {
        if let DelegationDistribution::Weighted { delegations, .. } = &self.key_registration.delegation {
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
    // so it would be disingenuous to implement them if users called to_original_cbor_bytes() and we skip the rest of
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
    /// The resulting RegistrationCbor will contain ONLY the relevant fields for CIP36 from the Metadata
    pub fn from_metadata_bytes(&self, metadata_cbor_bytes: &[u8]) -> Result<Self, DeserializeError> {
        let mut raw = Deserializer::from(std::io::Cursor::new(metadata_cbor_bytes));
        Self::deserialize(&mut raw)
    }

    /// Serializes as a Metadata structure containing ONLY the relevant fields for CIP36
    /// If this was created from bytes or from a Metadata that was created from bytes, it will preserve
    /// the encodings but only from the metadatums themselves within the keys 61284 and 61285
    /// * `force_canonical` - Whether to force canonical CBOR encodings. ONLY applies to the metadatums within labels 61285 and 61286
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.verify().map_err(|e| cbor_event::Error::CustomError(e.to_string()))?;
        serializer.write_map(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(KEY_REGISTRATION_LABEL)?;
        self.key_registration.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer(REGISTRATION_WITNESS_LABEL)?;
        self.registration_witness.serialize(serializer, force_canonical)
    }

    /// Deserializes a CIP36 view from either a Metadata or a RegistrationCbor
    /// This contains ONLY the relevant fields for CIP36 if created from a Metadata
    fn deserialize<R: BufRead + std::io::Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        use cardano_multiplatform_lib_core::{error::Key, serialization::CBORReadLen};
        let len = raw.map_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        (|| -> Result<_, DeserializeError> {
            let mut key_registration = None;
            let mut registration_witness = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                        61284 =>  {
                            if key_registration.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(KEY_REGISTRATION_LABEL)).into());
                            }
                            key_registration = Some(KeyRegistration::deserialize(raw).map_err(|e: DeserializeError| e.annotate("key_registration"))?);
                        },
                        61285 =>  {
                            if registration_witness.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(REGISTRATION_WITNESS_LABEL)).into());
                            }
                            registration_witness = Some(RegistrationWitness::deserialize(raw).map_err(|e: DeserializeError| e.annotate("registration_witness"))?);
                        },
                        _unknown_key => ()/* permissive of other metadatum labels */,
                    },
                    CBORType::Text => return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into()),
                    CBORType::Special => match len {
                        cbor_event::LenSz::Len(_, _) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
                }
                read += 1;
            }
            let key_registration = match key_registration {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(KEY_REGISTRATION_LABEL)).into()),
            };
            let registration_witness = match registration_witness {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(REGISTRATION_WITNESS_LABEL)).into()),
            };
            read_len.finish()?;
            let reg_cbor = Self {
                key_registration,
                registration_witness,
            };
            reg_cbor.verify().map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)))?;
            Ok(reg_cbor)
        })().map_err(|e| e.annotate("RegistrationCbor"))
    }
}

impl std::convert::TryFrom<&Metadata> for RegistrationCbor {
    type Error = DeserializeError;

    fn try_from(metadata: &Metadata) -> Result<Self, Self::Error> {
        use cardano_multiplatform_lib_core::error::Key;
        let reg_metadatum = metadata
            .get(&KEY_REGISTRATION_LABEL)
            .ok_or_else(|| DeserializeFailure::MandatoryFieldMissing(Key::Uint(KEY_REGISTRATION_LABEL)))?;
        let witness_metadatum = metadata
            .get(&REGISTRATION_WITNESS_LABEL)
            .ok_or_else(|| DeserializeFailure::MandatoryFieldMissing(Key::Uint(REGISTRATION_WITNESS_LABEL)))?;
        Ok(Self {
            key_registration: KeyRegistration::from_cbor_bytes(&reg_metadatum.to_original_cbor_bytes())?,
            registration_witness: RegistrationWitness::from_cbor_bytes(&witness_metadatum.to_original_cbor_bytes())?,
        })
    }
}

impl std::convert::TryInto<Metadata> for &RegistrationCbor {
    type Error = DeserializeError;

    fn try_into(self) -> Result<Metadata, Self::Error> {
        let mut metadata = Metadata::new();
        self.add_to_metadata(&mut metadata)?;
        Ok(metadata)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct RegistrationWitness {
    pub stake_witness: StakeWitness,
    #[serde(skip)]
    pub encodings: Option<RegistrationWitnessEncoding>,
}

impl RegistrationWitness {
    pub fn new(stake_witness: StakeWitness) -> Self {
        Self {
            stake_witness,
            encodings: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use cardano_multiplatform_lib_chain::address::Address;
    use cml_crypto::*;

    use super::*;

    #[test]
    fn sign_data() {
        // TODO: test vectors don't fully specify how to arrive from these to the resulting delegations/stake creds/reward address
        // so we don't derive or anything and just put them straight into the structs but that would be a more complete test
        // and we might want to add deriving-related options (assuming these keys weren't in their already-derived state)
        // let payment_pub = PublicKey::from_raw_hex("3273a5316e4de228863bd7cf8dac90d57149e1a595f3dd131073b84e35546676").unwrap();
        // let staking_prv = Bip32PrivateKey::from_raw_hex("f5beaeff7932a4164d270afde7716067582412e8977e67986cd9b456fc082e3a").unwrap();
        // let staking_derived_prv = staking_prv
        //     .derive(1694)
        //     .derive(1815)
        //     .derive(0)
        //     .derive(0);
        // let catalyst_prv_key = hex::decode("4820f7ce221e177c8eae2b2ee5c1f1581a0d88ca5c14329d8f2389e77a465655c27662621bfb99cb9445bf8114cc2a630afd2dd53bc88c08c5f2aed8e9c7cb89").unwrap();
        let stake_cred = StakeCredential::from(PublicKey::from_raw_hex("86870efc99c453a873a16492ce87738ec79a0ebd064379a62e2c9cf4e119219e").unwrap());
        // let stake_cred = StakeCredential::from(staking_derived_prv.to_public().to_raw_key());
        let reward_address = RewardAddress::from_address(&Address::from_bech32("stake_test1uzhr5zn6akj2affzua8ylcm8t872spuf5cf6tzjrvnmwemcehgcjm").unwrap()).unwrap();
        let nonce = 1234;

        // legacy format
        let legacy_reg = KeyRegistration::new(
            DelegationDistribution::new_legacy_key_registration(LegacyKeyRegistration::from_raw_hex("a6a3c0447aeb9cc54cf6422ba32b294e5e1c3ef6d782f2acff4a70694c4d1663").unwrap()),
            stake_cred.clone(),
            reward_address.clone(),
            nonce);
        let legacy_sign_data_hash = legacy_reg.hash_to_sign(false).unwrap();
        assert_eq!("872bcb4a9e2b110a06fd5de04be5924b6c659c28a1665ecc75def13ebca6dfd8", hex::encode(legacy_sign_data_hash));

        // weighted
        let weighted_reg = KeyRegistration::new(
            DelegationDistribution::new_weighted(vec![
                Delegation::new(VotingPubKey::from_raw_hex("a6a3c0447aeb9cc54cf6422ba32b294e5e1c3ef6d782f2acff4a70694c4d1663").unwrap(), 1),
                Delegation::new(VotingPubKey::from_raw_hex("00588e8e1d18cba576a4d35758069fe94e53f638b6faf7c07b8abd2bc5c5cdee").unwrap(), 3),
            ]),
            stake_cred,
            reward_address,
            nonce);
        let weighted_sign_data_hash = weighted_reg.hash_to_sign(false).unwrap();
        // There are some issues with the CIP-36 test vector here. TODO: figure out whether it's the vector or the spec or us that's wrong.
        //assert_eq!("5bc0681f173efd76e1989037a3694b8a7abea22053f5940cbb5cfcdf721007d7", hex::encode(weighted_sign_data_hash));

        // TODO: deregistration test? there are no official test vectors in CIP36
    }
}