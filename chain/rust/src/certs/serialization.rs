// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;

use cml_crypto::RawBytesEncoding;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl Serialize for AuthCommitteeHotCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for AuthCommitteeHotCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            14u64,
            fit_sz(
                14u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.index_0_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.committee_cold_credential
            .serialize(serializer, force_canonical)?;
        self.committee_hot_credential
            .serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for AuthCommitteeHotCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for AuthCommitteeHotCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let index_0_encoding = (|| -> Result<_, DeserializeError> {
                let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                if index_0_value != 14 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(14),
                    }
                    .into());
                }
                Ok(Some(index_0_encoding))
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let committee_cold_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("committee_cold_credential"))?;
            let committee_hot_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("committee_hot_credential"))?;
            Ok(AuthCommitteeHotCert {
                committee_cold_credential,
                committee_hot_credential,
                encodings: Some(AuthCommitteeHotCertEncoding {
                    len_encoding,
                    index_0_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("AuthCommitteeHotCert"))
    }
}

impl Serialize for Certificate {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Certificate::StakeRegistration(stake_registration) => {
                stake_registration.serialize(serializer, force_canonical)
            }
            Certificate::StakeDeregistration(stake_deregistration) => {
                stake_deregistration.serialize(serializer, force_canonical)
            }
            Certificate::StakeDelegation(stake_delegation) => {
                stake_delegation.serialize(serializer, force_canonical)
            }
            Certificate::PoolRegistration(pool_registration) => {
                pool_registration.serialize(serializer, force_canonical)
            }
            Certificate::PoolRetirement(pool_retirement) => {
                pool_retirement.serialize(serializer, force_canonical)
            }
            Certificate::RegCert(reg_cert) => reg_cert.serialize(serializer, force_canonical),
            Certificate::UnregCert(unreg_cert) => unreg_cert.serialize(serializer, force_canonical),
            Certificate::VoteDelegCert(vote_deleg_cert) => {
                vote_deleg_cert.serialize(serializer, force_canonical)
            }
            Certificate::StakeVoteDelegCert(stake_vote_deleg_cert) => {
                stake_vote_deleg_cert.serialize(serializer, force_canonical)
            }
            Certificate::StakeRegDelegCert(stake_reg_deleg_cert) => {
                stake_reg_deleg_cert.serialize(serializer, force_canonical)
            }
            Certificate::VoteRegDelegCert(vote_reg_deleg_cert) => {
                vote_reg_deleg_cert.serialize(serializer, force_canonical)
            }
            Certificate::StakeVoteRegDelegCert(stake_vote_reg_deleg_cert) => {
                stake_vote_reg_deleg_cert.serialize(serializer, force_canonical)
            }
            Certificate::AuthCommitteeHotCert(auth_committee_hot_cert) => {
                auth_committee_hot_cert.serialize(serializer, force_canonical)
            }
            Certificate::ResignCommitteeColdCert(resign_committee_cold_cert) => {
                resign_committee_cold_cert.serialize(serializer, force_canonical)
            }
            Certificate::RegDrepCert(reg_drep_cert) => {
                reg_drep_cert.serialize(serializer, force_canonical)
            }
            Certificate::UnregDrepCert(unreg_drep_cert) => {
                unreg_drep_cert.serialize(serializer, force_canonical)
            }
            Certificate::UpdateDrepCert(update_drep_cert) => {
                update_drep_cert.serialize(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for Certificate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let ret = StakeRegistration::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(stake_registration) => return Ok(Self::StakeRegistration(stake_registration)),
                Err(e) => {
                    errs.push(e.annotate("StakeRegistration"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let ret =
                    StakeDeregistration::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(stake_deregistration) => {
                    return Ok(Self::StakeDeregistration(stake_deregistration))
                }
                Err(e) => {
                    errs.push(e.annotate("StakeDeregistration"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = StakeDelegation::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(stake_delegation) => return Ok(Self::StakeDelegation(stake_delegation)),
                Err(e) => {
                    errs.push(e.annotate("StakeDelegation"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(10)?;
                read_len.finish()?;
                let ret = PoolRegistration::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(pool_registration) => return Ok(Self::PoolRegistration(pool_registration)),
                Err(e) => {
                    errs.push(e.annotate("PoolRegistration"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = PoolRetirement::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(pool_retirement) => return Ok(Self::PoolRetirement(pool_retirement)),
                Err(e) => {
                    errs.push(e.annotate("PoolRetirement"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = RegCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(reg_cert) => return Ok(Self::RegCert(reg_cert)),
                Err(e) => {
                    errs.push(e.annotate("RegCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = UnregCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(unreg_cert) => return Ok(Self::UnregCert(unreg_cert)),
                Err(e) => {
                    errs.push(e.annotate("UnregCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = VoteDelegCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(vote_deleg_cert) => return Ok(Self::VoteDelegCert(vote_deleg_cert)),
                Err(e) => {
                    errs.push(e.annotate("VoteDelegCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(4)?;
                read_len.finish()?;
                let ret =
                    StakeVoteDelegCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(stake_vote_deleg_cert) => {
                    return Ok(Self::StakeVoteDelegCert(stake_vote_deleg_cert))
                }
                Err(e) => {
                    errs.push(e.annotate("StakeVoteDelegCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(4)?;
                read_len.finish()?;
                let ret = StakeRegDelegCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(stake_reg_deleg_cert) => {
                    return Ok(Self::StakeRegDelegCert(stake_reg_deleg_cert))
                }
                Err(e) => {
                    errs.push(e.annotate("StakeRegDelegCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(4)?;
                read_len.finish()?;
                let ret = VoteRegDelegCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(vote_reg_deleg_cert) => return Ok(Self::VoteRegDelegCert(vote_reg_deleg_cert)),
                Err(e) => {
                    errs.push(e.annotate("VoteRegDelegCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(5)?;
                read_len.finish()?;
                let ret =
                    StakeVoteRegDelegCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(stake_vote_reg_deleg_cert) => {
                    return Ok(Self::StakeVoteRegDelegCert(stake_vote_reg_deleg_cert))
                }
                Err(e) => {
                    errs.push(e.annotate("StakeVoteRegDelegCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret =
                    AuthCommitteeHotCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(auth_committee_hot_cert) => {
                    return Ok(Self::AuthCommitteeHotCert(auth_committee_hot_cert))
                }
                Err(e) => {
                    errs.push(e.annotate("AuthCommitteeHotCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret =
                    ResignCommitteeColdCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(resign_committee_cold_cert) => {
                    return Ok(Self::ResignCommitteeColdCert(resign_committee_cold_cert))
                }
                Err(e) => {
                    errs.push(e.annotate("ResignCommitteeColdCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(4)?;
                read_len.finish()?;
                let ret = RegDrepCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(reg_drep_cert) => return Ok(Self::RegDrepCert(reg_drep_cert)),
                Err(e) => {
                    errs.push(e.annotate("RegDrepCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = UnregDrepCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(unreg_drep_cert) => return Ok(Self::UnregDrepCert(unreg_drep_cert)),
                Err(e) => {
                    errs.push(e.annotate("UnregDrepCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = UpdateDrepCert::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(update_drep_cert) => return Ok(Self::UpdateDrepCert(update_drep_cert)),
                Err(e) => {
                    errs.push(e.annotate("UpdateDrepCert"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "Certificate",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("Certificate"))
    }
}

impl Serialize for Credential {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Credential::PubKey {
                hash,
                len_encoding,
                tag_encoding,
                hash_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    0u64,
                    fit_sz(0u64, *tag_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    hash.to_raw_bytes(),
                    hash_encoding.to_str_len_sz(hash.to_raw_bytes().len() as u64, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            Credential::Script {
                hash,
                len_encoding,
                tag_encoding,
                hash_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    1u64,
                    fit_sz(1u64, *tag_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    hash.to_raw_bytes(),
                    hash_encoding.to_str_len_sz(hash.to_raw_bytes().len() as u64, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
        }
    }
}

impl Deserialize for Credential {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let tag_encoding = (|| -> Result<_, DeserializeError> {
                    let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                    if tag_value != 0 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(tag_value),
                            expected: Key::Uint(0),
                        }
                        .into());
                    }
                    Ok(Some(tag_encoding))
                })()
                .map_err(|e| e.annotate("tag"))?;
                let (hash, hash_encoding) = raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        Ed25519KeyHash::from_raw_bytes(&bytes)
                            .map(|bytes| (bytes, StringEncoding::from(enc)))
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("hash"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::PubKey {
                    hash,
                    len_encoding,
                    tag_encoding,
                    hash_encoding,
                })
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("PubKey"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let tag_encoding = (|| -> Result<_, DeserializeError> {
                    let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                    if tag_value != 1 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(tag_value),
                            expected: Key::Uint(1),
                        }
                        .into());
                    }
                    Ok(Some(tag_encoding))
                })()
                .map_err(|e| e.annotate("tag"))?;
                let (hash, hash_encoding) = raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        ScriptHash::from_raw_bytes(&bytes)
                            .map(|bytes| (bytes, StringEncoding::from(enc)))
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("hash"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::Script {
                    hash,
                    len_encoding,
                    tag_encoding,
                    hash_encoding,
                })
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("Script"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "Credential",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("Credential"))
    }
}

impl Serialize for DNSName {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_text_sz(
            &self.inner,
            self.encodings
                .as_ref()
                .map(|encs| encs.inner_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.inner.len() as u64, force_canonical),
        )
    }
}

impl Deserialize for DNSName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw
            .text_sz()
            .map(|(s, enc)| (s, StringEncoding::from(enc)))?;
        if inner.len() > 128 {
            return Err(DeserializeError::new(
                "DNSName",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as isize,
                    min: Some(0),
                    max: Some(128),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: Some(DNSNameEncoding { inner_encoding }),
        })
    }
}

impl Serialize for DRep {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            DRep::Key {
                pool,
                len_encoding,
                index_0_encoding,
                pool_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    0u64,
                    fit_sz(0u64, *index_0_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    pool.to_raw_bytes(),
                    pool_encoding.to_str_len_sz(pool.to_raw_bytes().len() as u64, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            DRep::Script {
                script_hash,
                len_encoding,
                index_0_encoding,
                script_hash_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    1u64,
                    fit_sz(1u64, *index_0_encoding, force_canonical),
                )?;
                serializer.write_bytes_sz(
                    script_hash.to_raw_bytes(),
                    script_hash_encoding
                        .to_str_len_sz(script_hash.to_raw_bytes().len() as u64, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            DRep::AlwaysAbstain {
                always_abstain_encoding,
                len_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(1, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    2u64,
                    fit_sz(2u64, *always_abstain_encoding, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            DRep::AlwaysNoConfidence {
                always_no_confidence_encoding,
                len_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(1, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    3u64,
                    fit_sz(3u64, *always_no_confidence_encoding, force_canonical),
                )?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
        }
    }
}

impl Deserialize for DRep {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let index_0_encoding = (|| -> Result<_, DeserializeError> {
                    let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                    if index_0_value != 0 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(index_0_value),
                            expected: Key::Uint(0),
                        }
                        .into());
                    }
                    Ok(Some(index_0_encoding))
                })()
                .map_err(|e| e.annotate("index_0"))?;
                let (pool, pool_encoding) = raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        Ed25519KeyHash::from_raw_bytes(&bytes)
                            .map(|bytes| (bytes, StringEncoding::from(enc)))
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("pool"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::Key {
                    pool,
                    len_encoding,
                    index_0_encoding,
                    pool_encoding,
                })
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("Key"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let index_0_encoding = (|| -> Result<_, DeserializeError> {
                    let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                    if index_0_value != 1 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(index_0_value),
                            expected: Key::Uint(1),
                        }
                        .into());
                    }
                    Ok(Some(index_0_encoding))
                })()
                .map_err(|e| e.annotate("index_0"))?;
                let (script_hash, script_hash_encoding) = raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        ScriptHash::from_raw_bytes(&bytes)
                            .map(|bytes| (bytes, StringEncoding::from(enc)))
                            .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                    })
                    .map_err(|e: DeserializeError| e.annotate("script_hash"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::Script {
                    script_hash,
                    len_encoding,
                    index_0_encoding,
                    script_hash_encoding,
                })
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("Script"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(1)?;
                read_len.finish()?;
                let (always_abstain_value, always_abstain_encoding) = raw.unsigned_integer_sz()?;
                if always_abstain_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(always_abstain_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                let ret = Ok(Some(always_abstain_encoding));
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(always_abstain_encoding) => {
                    return Ok(Self::AlwaysAbstain {
                        always_abstain_encoding,
                        len_encoding,
                    })
                }
                Err(e) => {
                    errs.push(e.annotate("AlwaysAbstain"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(1)?;
                read_len.finish()?;
                let (always_no_confidence_value, always_no_confidence_encoding) =
                    raw.unsigned_integer_sz()?;
                if always_no_confidence_value != 3 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(always_no_confidence_value),
                        expected: Key::Uint(3),
                    }
                    .into());
                }
                let ret = Ok(Some(always_no_confidence_encoding));
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(always_no_confidence_encoding) => {
                    return Ok(Self::AlwaysNoConfidence {
                        always_no_confidence_encoding,
                        len_encoding,
                    })
                }
                Err(e) => {
                    errs.push(e.annotate("AlwaysNoConfidence"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "DRep",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("DRep"))
    }
}

impl Serialize for Ipv4 {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(
            &self.inner,
            self.encodings
                .as_ref()
                .map(|encs| encs.inner_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.inner.len() as u64, force_canonical),
        )
    }
}

impl Deserialize for Ipv4 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw
            .bytes_sz()
            .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        if inner.len() != 4 {
            return Err(DeserializeError::new(
                "Ipv4",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as isize,
                    min: Some(4),
                    max: Some(4),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: Some(Ipv4Encoding { inner_encoding }),
        })
    }
}

impl Serialize for Ipv6 {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(
            &self.inner,
            self.encodings
                .as_ref()
                .map(|encs| encs.inner_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.inner.len() as u64, force_canonical),
        )
    }
}

impl Deserialize for Ipv6 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw
            .bytes_sz()
            .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        if inner.len() != 16 {
            return Err(DeserializeError::new(
                "Ipv6",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as isize,
                    min: Some(16),
                    max: Some(16),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: Some(Ipv6Encoding { inner_encoding }),
        })
    }
}

impl Serialize for MultiHostName {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(2, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for MultiHostName {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            2u64,
            fit_sz(
                2u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.dns_name.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for MultiHostName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for MultiHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let dns_name =
                DNSName::deserialize(raw).map_err(|e: DeserializeError| e.annotate("dns_name"))?;
            Ok(MultiHostName {
                dns_name,
                encodings: Some(MultiHostNameEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("MultiHostName"))
    }
}

impl Serialize for PoolMetadata {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(2, force_canonical),
        )?;
        self.url.serialize(serializer, force_canonical)?;
        serializer.write_bytes_sz(
            self.pool_metadata_hash.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.pool_metadata_hash_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.pool_metadata_hash.to_raw_bytes().len() as u64,
                    force_canonical,
                ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for PoolMetadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let url = Url::deserialize(raw).map_err(|e: DeserializeError| e.annotate("url"))?;
            let (pool_metadata_hash, pool_metadata_hash_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    PoolMetadataHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("pool_metadata_hash"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(PoolMetadata {
                url,
                pool_metadata_hash,
                encodings: Some(PoolMetadataEncoding {
                    len_encoding,
                    pool_metadata_hash_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("PoolMetadata"))
    }
}

impl Serialize for PoolParams {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(9, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for PoolParams {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes_sz(
            self.operator.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.operator_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.operator.to_raw_bytes().len() as u64, force_canonical),
        )?;
        serializer.write_bytes_sz(
            self.vrf_keyhash.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.vrf_keyhash_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(
                    self.vrf_keyhash.to_raw_bytes().len() as u64,
                    force_canonical,
                ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.pledge,
            fit_sz(
                self.pledge,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.pledge_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.cost,
            fit_sz(
                self.cost,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.cost_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.margin.serialize(serializer, force_canonical)?;
        self.reward_account.serialize(serializer, force_canonical)?;
        self.pool_owners.serialize(serializer, force_canonical)?;
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.relays_encoding)
                .unwrap_or_default()
                .to_len_sz(self.relays.len() as u64, force_canonical),
        )?;
        for element in self.relays.iter() {
            element.serialize(serializer, force_canonical)?;
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.relays_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        match &self.pool_metadata {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for PoolParams {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(9)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for PoolParams {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let (operator, operator_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Ed25519KeyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("operator"))?;
            let (vrf_keyhash, vrf_keyhash_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    VRFKeyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("vrf_keyhash"))?;
            let (pledge, pledge_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("pledge"))?;
            let (cost, cost_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("cost"))?;
            let margin = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("margin"))?;
            let reward_account = RewardAccount::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("reward_account"))?;
            let pool_owners = SetEd25519KeyHash::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("pool_owners"))?;
            let (relays, relays_encoding) = (|| -> Result<_, DeserializeError> {
                let mut relays_arr = Vec::new();
                let len = raw.array_sz()?;
                let relays_encoding = len.into();
                while match len {
                    cbor_event::LenSz::Len(n, _) => (relays_arr.len() as u64) < n,
                    cbor_event::LenSz::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    relays_arr.push(Relay::deserialize(raw)?);
                }
                Ok((relays_arr, relays_encoding))
            })()
            .map_err(|e| e.annotate("relays"))?;
            let pool_metadata = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(PoolMetadata::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("pool_metadata"))?;
            Ok(PoolParams {
                operator,
                vrf_keyhash,
                pledge,
                cost,
                margin,
                reward_account,
                pool_owners,
                relays,
                pool_metadata,
                encodings: Some(PoolParamsEncoding {
                    len_encoding,
                    operator_encoding,
                    vrf_keyhash_encoding,
                    pledge_encoding,
                    cost_encoding,
                    relays_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("PoolParams"))
    }
}

impl Serialize for PoolRegistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(10, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for PoolRegistration {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            3u64,
            fit_sz(
                3u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.pool_params
            .serialize_as_embedded_group(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for PoolRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(10)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for PoolRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 3 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(3),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let pool_params = PoolParams::deserialize_as_embedded_group(raw, read_len, len)
                .map_err(|e: DeserializeError| e.annotate("pool_params"))?;
            Ok(PoolRegistration {
                pool_params,
                encodings: Some(PoolRegistrationEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("PoolRegistration"))
    }
}

impl Serialize for PoolRetirement {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for PoolRetirement {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            4u64,
            fit_sz(
                4u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_bytes_sz(
            self.pool.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.pool_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.pool.to_raw_bytes().len() as u64, force_canonical),
        )?;
        serializer.write_unsigned_integer_sz(
            self.epoch,
            fit_sz(
                self.epoch,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.epoch_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for PoolRetirement {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for PoolRetirement {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 4 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(4),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let (pool, pool_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Ed25519KeyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("pool"))?;
            let (epoch, epoch_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("epoch"))?;
            Ok(PoolRetirement {
                pool,
                epoch,
                encodings: Some(PoolRetirementEncoding {
                    len_encoding,
                    tag_encoding,
                    pool_encoding,
                    epoch_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("PoolRetirement"))
    }
}

impl Serialize for RegCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for RegCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            7u64,
            fit_sz(
                7u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.stake_credential
            .serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(
            self.deposit,
            fit_sz(
                self.deposit,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.deposit_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for RegCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for RegCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 7 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(7),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let stake_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
            let (deposit, deposit_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("deposit"))?;
            Ok(RegCert {
                stake_credential,
                deposit,
                encodings: Some(RegCertEncoding {
                    len_encoding,
                    tag_encoding,
                    deposit_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("RegCert"))
    }
}

impl Serialize for RegDrepCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(4, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for RegDrepCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            16u64,
            fit_sz(
                16u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.index_0_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.drep_credential
            .serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(
            self.deposit,
            fit_sz(
                self.deposit,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.deposit_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        match &self.anchor {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for RegDrepCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for RegDrepCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let index_0_encoding = (|| -> Result<_, DeserializeError> {
                let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                if index_0_value != 16 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(16),
                    }
                    .into());
                }
                Ok(Some(index_0_encoding))
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let drep_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("drep_credential"))?;
            let (deposit, deposit_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("deposit"))?;
            let anchor = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(Anchor::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("anchor"))?;
            Ok(RegDrepCert {
                drep_credential,
                deposit,
                anchor,
                encodings: Some(RegDrepCertEncoding {
                    len_encoding,
                    index_0_encoding,
                    deposit_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("RegDrepCert"))
    }
}

impl Serialize for Relay {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Relay::SingleHostAddr(single_host_addr) => {
                single_host_addr.serialize(serializer, force_canonical)
            }
            Relay::SingleHostName(single_host_name) => {
                single_host_name.serialize(serializer, force_canonical)
            }
            Relay::MultiHostName(multi_host_name) => {
                multi_host_name.serialize(serializer, force_canonical)
            }
        }
    }
}

impl Deserialize for Relay {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(4)?;
                read_len.finish()?;
                let ret = SingleHostAddr::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(single_host_addr) => return Ok(Self::SingleHostAddr(single_host_addr)),
                Err(e) => {
                    errs.push(e.annotate("SingleHostAddr"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(3)?;
                read_len.finish()?;
                let ret = SingleHostName::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(single_host_name) => return Ok(Self::SingleHostName(single_host_name)),
                Err(e) => {
                    errs.push(e.annotate("SingleHostName"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let ret = MultiHostName::deserialize_as_embedded_group(raw, &mut read_len, len);
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                ret
            })(raw);
            match deser_variant {
                Ok(multi_host_name) => return Ok(Self::MultiHostName(multi_host_name)),
                Err(e) => {
                    errs.push(e.annotate("MultiHostName"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "Relay",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("Relay"))
    }
}

impl Serialize for ResignCommitteeColdCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for ResignCommitteeColdCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            15u64,
            fit_sz(
                15u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.index_0_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.committee_cold_credential
            .serialize(serializer, force_canonical)?;
        match &self.anchor {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ResignCommitteeColdCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for ResignCommitteeColdCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let index_0_encoding = (|| -> Result<_, DeserializeError> {
                let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                if index_0_value != 15 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(15),
                    }
                    .into());
                }
                Ok(Some(index_0_encoding))
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let committee_cold_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("committee_cold_credential"))?;
            let anchor = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(Anchor::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("anchor"))?;
            Ok(ResignCommitteeColdCert {
                committee_cold_credential,
                anchor,
                encodings: Some(ResignCommitteeColdCertEncoding {
                    len_encoding,
                    index_0_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ResignCommitteeColdCert"))
    }
}

impl Serialize for SingleHostAddr {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(4, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for SingleHostAddr {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            0u64,
            fit_sz(
                0u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        match &self.port {
            Some(x) => serializer.write_unsigned_integer_sz(
                *x as u64,
                fit_sz(
                    *x as u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.port_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        match &self.ipv4 {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        match &self.ipv6 {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for SingleHostAddr {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for SingleHostAddr {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let (port, port_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Result::<_, DeserializeError>::Ok(
                        raw.unsigned_integer_sz()
                            .map(|(x, enc)| (x as u16, Some(enc)))?,
                    )
                    .map(|(x, port_encoding)| (Some(x), port_encoding))?,
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        (None, None)
                    }
                })
            })()
            .map_err(|e| e.annotate("port"))?;
            let ipv4 = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(Ipv4::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("ipv4"))?;
            let ipv6 = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(Ipv6::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("ipv6"))?;
            Ok(SingleHostAddr {
                port,
                ipv4,
                ipv6,
                encodings: Some(SingleHostAddrEncoding {
                    len_encoding,
                    tag_encoding,
                    port_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("SingleHostAddr"))
    }
}

impl Serialize for SingleHostName {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for SingleHostName {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            1u64,
            fit_sz(
                1u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        match &self.port {
            Some(x) => serializer.write_unsigned_integer_sz(
                *x as u64,
                fit_sz(
                    *x as u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.port_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            ),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.dns_name.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for SingleHostName {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for SingleHostName {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let (port, port_encoding) = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Result::<_, DeserializeError>::Ok(
                        raw.unsigned_integer_sz()
                            .map(|(x, enc)| (x as u16, Some(enc)))?,
                    )
                    .map(|(x, port_encoding)| (Some(x), port_encoding))?,
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        (None, None)
                    }
                })
            })()
            .map_err(|e| e.annotate("port"))?;
            let dns_name =
                DNSName::deserialize(raw).map_err(|e: DeserializeError| e.annotate("dns_name"))?;
            Ok(SingleHostName {
                port,
                dns_name,
                encodings: Some(SingleHostNameEncoding {
                    len_encoding,
                    tag_encoding,
                    port_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("SingleHostName"))
    }
}

impl Serialize for StakeDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for StakeDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            2u64,
            fit_sz(
                2u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.stake_credential
            .serialize(serializer, force_canonical)?;
        serializer.write_bytes_sz(
            self.pool.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.pool_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.pool.to_raw_bytes().len() as u64, force_canonical),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for StakeDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for StakeDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let stake_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
            let (pool, pool_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Ed25519KeyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("pool"))?;
            Ok(StakeDelegation {
                stake_credential,
                pool,
                encodings: Some(StakeDelegationEncoding {
                    len_encoding,
                    tag_encoding,
                    pool_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("StakeDelegation"))
    }
}

impl Serialize for StakeDeregistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(2, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for StakeDeregistration {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            1u64,
            fit_sz(
                1u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.stake_credential
            .serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for StakeDeregistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for StakeDeregistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let stake_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
            Ok(StakeDeregistration {
                stake_credential,
                encodings: Some(StakeDeregistrationEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("StakeDeregistration"))
    }
}

impl Serialize for StakeRegDelegCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(4, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for StakeRegDelegCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            11u64,
            fit_sz(
                11u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.stake_credential
            .serialize(serializer, force_canonical)?;
        serializer.write_bytes_sz(
            self.pool.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.pool_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.pool.to_raw_bytes().len() as u64, force_canonical),
        )?;
        serializer.write_unsigned_integer_sz(
            self.deposit,
            fit_sz(
                self.deposit,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.deposit_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for StakeRegDelegCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for StakeRegDelegCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 11 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(11),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let stake_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
            let (pool, pool_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Ed25519KeyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("pool"))?;
            let (deposit, deposit_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("deposit"))?;
            Ok(StakeRegDelegCert {
                stake_credential,
                pool,
                deposit,
                encodings: Some(StakeRegDelegCertEncoding {
                    len_encoding,
                    tag_encoding,
                    pool_encoding,
                    deposit_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("StakeRegDelegCert"))
    }
}

impl Serialize for StakeRegistration {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(2, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for StakeRegistration {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            0u64,
            fit_sz(
                0u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.stake_credential
            .serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for StakeRegistration {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for StakeRegistration {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let stake_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
            Ok(StakeRegistration {
                stake_credential,
                encodings: Some(StakeRegistrationEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("StakeRegistration"))
    }
}

impl Serialize for StakeVoteDelegCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(4, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for StakeVoteDelegCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            10u64,
            fit_sz(
                10u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.stake_credential
            .serialize(serializer, force_canonical)?;
        serializer.write_bytes_sz(
            self.pool.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.pool_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.pool.to_raw_bytes().len() as u64, force_canonical),
        )?;
        self.d_rep.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for StakeVoteDelegCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for StakeVoteDelegCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 10 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(10),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let stake_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
            let (pool, pool_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Ed25519KeyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("pool"))?;
            let d_rep =
                DRep::deserialize(raw).map_err(|e: DeserializeError| e.annotate("d_rep"))?;
            Ok(StakeVoteDelegCert {
                stake_credential,
                pool,
                d_rep,
                encodings: Some(StakeVoteDelegCertEncoding {
                    len_encoding,
                    tag_encoding,
                    pool_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("StakeVoteDelegCert"))
    }
}

impl Serialize for StakeVoteRegDelegCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(5, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for StakeVoteRegDelegCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            13u64,
            fit_sz(
                13u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.stake_credential
            .serialize(serializer, force_canonical)?;
        serializer.write_bytes_sz(
            self.pool.to_raw_bytes(),
            self.encodings
                .as_ref()
                .map(|encs| encs.pool_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.pool.to_raw_bytes().len() as u64, force_canonical),
        )?;
        self.d_rep.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(
            self.deposit,
            fit_sz(
                self.deposit,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.deposit_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for StakeVoteRegDelegCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(5)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for StakeVoteRegDelegCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 13 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(13),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let stake_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
            let (pool, pool_encoding) = raw
                .bytes_sz()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|(bytes, enc)| {
                    Ed25519KeyHash::from_raw_bytes(&bytes)
                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("pool"))?;
            let d_rep =
                DRep::deserialize(raw).map_err(|e: DeserializeError| e.annotate("d_rep"))?;
            let (deposit, deposit_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("deposit"))?;
            Ok(StakeVoteRegDelegCert {
                stake_credential,
                pool,
                d_rep,
                deposit,
                encodings: Some(StakeVoteRegDelegCertEncoding {
                    len_encoding,
                    tag_encoding,
                    pool_encoding,
                    deposit_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("StakeVoteRegDelegCert"))
    }
}

impl Serialize for UnregCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for UnregCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            8u64,
            fit_sz(
                8u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.stake_credential
            .serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(
            self.deposit,
            fit_sz(
                self.deposit,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.deposit_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for UnregCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for UnregCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 8 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(8),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let stake_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
            let (deposit, deposit_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("deposit"))?;
            Ok(UnregCert {
                stake_credential,
                deposit,
                encodings: Some(UnregCertEncoding {
                    len_encoding,
                    tag_encoding,
                    deposit_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("UnregCert"))
    }
}

impl Serialize for UnregDrepCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for UnregDrepCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            17u64,
            fit_sz(
                17u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.index_0_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.drep_credential
            .serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(
            self.deposit,
            fit_sz(
                self.deposit,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.deposit_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for UnregDrepCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for UnregDrepCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let index_0_encoding = (|| -> Result<_, DeserializeError> {
                let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                if index_0_value != 17 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(17),
                    }
                    .into());
                }
                Ok(Some(index_0_encoding))
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let drep_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("drep_credential"))?;
            let (deposit, deposit_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("deposit"))?;
            Ok(UnregDrepCert {
                drep_credential,
                deposit,
                encodings: Some(UnregDrepCertEncoding {
                    len_encoding,
                    index_0_encoding,
                    deposit_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("UnregDrepCert"))
    }
}

impl Serialize for UpdateDrepCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for UpdateDrepCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            18u64,
            fit_sz(
                18u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.index_0_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.drep_credential
            .serialize(serializer, force_canonical)?;
        match &self.anchor {
            Some(x) => x.serialize(serializer, force_canonical),
            None => serializer.write_special(cbor_event::Special::Null),
        }?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for UpdateDrepCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for UpdateDrepCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let index_0_encoding = (|| -> Result<_, DeserializeError> {
                let (index_0_value, index_0_encoding) = raw.unsigned_integer_sz()?;
                if index_0_value != 18 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(18),
                    }
                    .into());
                }
                Ok(Some(index_0_encoding))
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let drep_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("drep_credential"))?;
            let anchor = (|| -> Result<_, DeserializeError> {
                Ok(match raw.cbor_type()? != cbor_event::Type::Special {
                    true => Some(Anchor::deserialize(raw)?),
                    false => {
                        if raw.special()? != cbor_event::Special::Null {
                            return Err(DeserializeFailure::ExpectedNull.into());
                        }
                        None
                    }
                })
            })()
            .map_err(|e| e.annotate("anchor"))?;
            Ok(UpdateDrepCert {
                drep_credential,
                anchor,
                encodings: Some(UpdateDrepCertEncoding {
                    len_encoding,
                    index_0_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("UpdateDrepCert"))
    }
}

impl Serialize for Url {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_text_sz(
            &self.inner,
            self.encodings
                .as_ref()
                .map(|encs| encs.inner_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.inner.len() as u64, force_canonical),
        )
    }
}

impl Deserialize for Url {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw
            .text_sz()
            .map(|(s, enc)| (s, StringEncoding::from(enc)))?;
        if inner.len() > 128 {
            return Err(DeserializeError::new(
                "Url",
                DeserializeFailure::RangeCheck {
                    found: inner.len() as isize,
                    min: Some(0),
                    max: Some(128),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: Some(UrlEncoding { inner_encoding }),
        })
    }
}

impl Serialize for VoteDelegCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(3, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for VoteDelegCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            9u64,
            fit_sz(
                9u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.stake_credential
            .serialize(serializer, force_canonical)?;
        self.d_rep.serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for VoteDelegCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for VoteDelegCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 9 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(9),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let stake_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
            let d_rep =
                DRep::deserialize(raw).map_err(|e: DeserializeError| e.annotate("d_rep"))?;
            Ok(VoteDelegCert {
                stake_credential,
                d_rep,
                encodings: Some(VoteDelegCertEncoding {
                    len_encoding,
                    tag_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("VoteDelegCert"))
    }
}

impl Serialize for VoteRegDelegCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(4, force_canonical),
        )?;
        self.serialize_as_embedded_group(serializer, force_canonical)
    }
}

impl SerializeEmbeddedGroup for VoteRegDelegCert {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            12u64,
            fit_sz(
                12u64,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.tag_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.stake_credential
            .serialize(serializer, force_canonical)?;
        self.d_rep.serialize(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(
            self.deposit,
            fit_sz(
                self.deposit,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.deposit_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for VoteRegDelegCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        let ret = Self::deserialize_as_embedded_group(raw, &mut read_len, len);
        match len {
            cbor_event::LenSz::Len(_, _) => (),
            cbor_event::LenSz::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        ret
    }
}

impl DeserializeEmbeddedGroup for VoteRegDelegCert {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        _read_len: &mut CBORReadLen,
        len: cbor_event::LenSz,
    ) -> Result<Self, DeserializeError> {
        let len_encoding = len.into();
        (|| -> Result<_, DeserializeError> {
            let tag_encoding = (|| -> Result<_, DeserializeError> {
                let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                if tag_value != 12 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(tag_value),
                        expected: Key::Uint(12),
                    }
                    .into());
                }
                Ok(Some(tag_encoding))
            })()
            .map_err(|e| e.annotate("tag"))?;
            let stake_credential = Credential::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("stake_credential"))?;
            let d_rep =
                DRep::deserialize(raw).map_err(|e: DeserializeError| e.annotate("d_rep"))?;
            let (deposit, deposit_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("deposit"))?;
            Ok(VoteRegDelegCert {
                stake_credential,
                d_rep,
                deposit,
                encodings: Some(VoteRegDelegCertEncoding {
                    len_encoding,
                    tag_encoding,
                    deposit_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("VoteRegDelegCert"))
    }
}
