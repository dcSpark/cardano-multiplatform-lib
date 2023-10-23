// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::*;
use cbor_event;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;
use cml_crypto::RawBytesEncoding;
use std::io::{BufRead, Seek, SeekFrom, Write};

impl cbor_event::se::Serialize for Ssc {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Ssc::SscCommitmentsPayload(ssc_commitments_payload) => {
                ssc_commitments_payload.serialize(serializer)
            }
            Ssc::SscOpeningsPayload(ssc_openings_payload) => {
                ssc_openings_payload.serialize(serializer)
            }
            Ssc::SscSharesPayload(ssc_shares_payload) => ssc_shares_payload.serialize(serializer),
            Ssc::SscCertificatesPayload(ssc_certificates_payload) => {
                ssc_certificates_payload.serialize(serializer)
            }
        }
    }
}

impl Deserialize for Ssc {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> =
                SscCommitmentsPayload::deserialize(raw);
            match deser_variant {
                Ok(ssc_commitments_payload) => {
                    return Ok(Self::SscCommitmentsPayload(ssc_commitments_payload))
                }
                Err(e) => {
                    errs.push(e.annotate("SscCommitmentsPayload"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = SscOpeningsPayload::deserialize(raw);
            match deser_variant {
                Ok(ssc_openings_payload) => {
                    return Ok(Self::SscOpeningsPayload(ssc_openings_payload))
                }
                Err(e) => {
                    errs.push(e.annotate("SscOpeningsPayload"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = SscSharesPayload::deserialize(raw);
            match deser_variant {
                Ok(ssc_shares_payload) => return Ok(Self::SscSharesPayload(ssc_shares_payload)),
                Err(e) => {
                    errs.push(e.annotate("SscSharesPayload"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> =
                SscCertificatesPayload::deserialize(raw);
            match deser_variant {
                Ok(ssc_certificates_payload) => {
                    return Ok(Self::SscCertificatesPayload(ssc_certificates_payload))
                }
                Err(e) => {
                    errs.push(e.annotate("SscCertificatesPayload"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "Ssc",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("Ssc"))
    }
}

impl cbor_event::se::Serialize for SscCert {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        serializer.write_bytes(&self.vss_pub_key)?;
        serializer.write_unsigned_integer(self.epoch_id)?;
        serializer.write_bytes(&self.byron_pub_key)?;
        serializer.write_bytes(&self.byron_signature)?;
        Ok(serializer)
    }
}

impl Deserialize for SscCert {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let vss_pub_key = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("vss_pub_key"))?;
            let epoch_id = Ok(raw.unsigned_integer()?)
                .map_err(|e: DeserializeError| e.annotate("epoch_id"))?;
            let byron_pub_key = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("byron_pub_key"))?;
            let byron_signature = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("byron_signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SscCert {
                vss_pub_key,
                epoch_id,
                byron_pub_key,
                byron_signature,
            })
        })()
        .map_err(|e| e.annotate("SscCert"))
    }
}

impl cbor_event::se::Serialize for SscCertificatesPayload {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(3u64)?;
        serializer.write_tag(258u64)?;
        serializer.write_array(cbor_event::Len::Len(self.ssc_certs.len() as u64))?;
        for element in self.ssc_certs.iter() {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for SscCertificatesPayload {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 3 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(3),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let ssc_certs = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    258 => {
                        let mut ssc_certs_arr = Vec::new();
                        let len = raw.array()?;
                        while match len {
                            cbor_event::Len::Len(n) => (ssc_certs_arr.len() as u64) < n,
                            cbor_event::Len::Indefinite => true,
                        } {
                            if raw.cbor_type()? == cbor_event::Type::Special {
                                assert_eq!(raw.special()?, cbor_event::Special::Break);
                                break;
                            }
                            ssc_certs_arr.push(SscCert::deserialize(raw)?);
                        }
                        Ok(ssc_certs_arr)
                    }
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 258,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("ssc_certs"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SscCertificatesPayload { ssc_certs })
        })()
        .map_err(|e| e.annotate("SscCertificatesPayload"))
    }
}

impl cbor_event::se::Serialize for SscCertificatesProof {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(3u64)?;
        serializer.write_bytes(self.blake2b256.to_raw_bytes())?;
        Ok(serializer)
    }
}

impl Deserialize for SscCertificatesProof {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 3 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(3),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let blake2b256 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b256"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SscCertificatesProof { blake2b256 })
        })()
        .map_err(|e| e.annotate("SscCertificatesProof"))
    }
}

impl cbor_event::se::Serialize for SscCommitment {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_map(cbor_event::Len::Len(self.vss_shares.len() as u64))?;
        for (key, value) in self.vss_shares.iter() {
            serializer.write_bytes(key)?;
            value.serialize(serializer)?;
        }
        self.vss_proof.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for SscCommitment {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let vss_shares = (|| -> Result<_, DeserializeError> {
                let mut vss_shares_table = BTreeMap::new();
                let vss_shares_len = raw.map()?;
                while match vss_shares_len {
                    cbor_event::Len::Len(n) => (vss_shares_table.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let vss_shares_key = raw.bytes()? as Vec<u8>;
                    let vss_shares_value = VssEncryptedShare::deserialize(raw)?;
                    if vss_shares_table
                        .insert(vss_shares_key.clone(), vss_shares_value)
                        .is_some()
                    {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                }
                Ok(vss_shares_table)
            })()
            .map_err(|e| e.annotate("vss_shares"))?;
            let vss_proof = VssProof::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("vss_proof"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SscCommitment {
                vss_shares,
                vss_proof,
            })
        })()
        .map_err(|e| e.annotate("SscCommitment"))
    }
}

impl cbor_event::se::Serialize for SscCommitmentsPayload {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(0u64)?;
        serializer.write_tag(258u64)?;
        serializer.write_array(cbor_event::Len::Len(
            self.ssc_signed_commitments.len() as u64
        ))?;
        for element in self.ssc_signed_commitments.iter() {
            element.serialize(serializer)?;
        }
        serializer.write_tag(258u64)?;
        serializer.write_array(cbor_event::Len::Len(self.ssc_certs.len() as u64))?;
        for element in self.ssc_certs.iter() {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for SscCommitmentsPayload {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let ssc_signed_commitments = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    258 => {
                        let mut ssc_signed_commitments_arr = Vec::new();
                        let len = raw.array()?;
                        while match len {
                            cbor_event::Len::Len(n) => {
                                (ssc_signed_commitments_arr.len() as u64) < n
                            }
                            cbor_event::Len::Indefinite => true,
                        } {
                            if raw.cbor_type()? == cbor_event::Type::Special {
                                assert_eq!(raw.special()?, cbor_event::Special::Break);
                                break;
                            }
                            ssc_signed_commitments_arr.push(SscSignedCommitment::deserialize(raw)?);
                        }
                        Ok(ssc_signed_commitments_arr)
                    }
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 258,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("ssc_signed_commitments"))?;
            let ssc_certs = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    258 => {
                        let mut ssc_certs_arr = Vec::new();
                        let len = raw.array()?;
                        while match len {
                            cbor_event::Len::Len(n) => (ssc_certs_arr.len() as u64) < n,
                            cbor_event::Len::Indefinite => true,
                        } {
                            if raw.cbor_type()? == cbor_event::Type::Special {
                                assert_eq!(raw.special()?, cbor_event::Special::Break);
                                break;
                            }
                            ssc_certs_arr.push(SscCert::deserialize(raw)?);
                        }
                        Ok(ssc_certs_arr)
                    }
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 258,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("ssc_certs"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SscCommitmentsPayload {
                ssc_signed_commitments,
                ssc_certs,
            })
        })()
        .map_err(|e| e.annotate("SscCommitmentsPayload"))
    }
}

impl cbor_event::se::Serialize for SscCommitmentsProof {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(0u64)?;
        serializer.write_bytes(self.blake2b256.to_raw_bytes())?;
        serializer.write_bytes(self.blake2b2562.to_raw_bytes())?;
        Ok(serializer)
    }
}

impl Deserialize for SscCommitmentsProof {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let blake2b256 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b256"))?;
            let blake2b2562 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b2562"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SscCommitmentsProof {
                blake2b256,
                blake2b2562,
            })
        })()
        .map_err(|e| e.annotate("SscCommitmentsProof"))
    }
}

impl cbor_event::se::Serialize for SscOpeningsPayload {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(1u64)?;
        serializer.write_map(cbor_event::Len::Len(self.ssc_opens.len() as u64))?;
        for (key, value) in self.ssc_opens.iter() {
            serializer.write_bytes(key.to_raw_bytes())?;
            serializer.write_bytes(value)?;
        }
        serializer.write_tag(258u64)?;
        serializer.write_array(cbor_event::Len::Len(self.ssc_certs.len() as u64))?;
        for element in self.ssc_certs.iter() {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for SscOpeningsPayload {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let ssc_opens = (|| -> Result<_, DeserializeError> {
                let mut ssc_opens_table = BTreeMap::new();
                let ssc_opens_len = raw.map()?;
                while match ssc_opens_len {
                    cbor_event::Len::Len(n) => (ssc_opens_table.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let ssc_opens_key = raw
                        .bytes()
                        .map_err(Into::<DeserializeError>::into)
                        .and_then(|bytes| {
                            StakeholderId::from_raw_bytes(&bytes).map_err(|e| {
                                DeserializeFailure::InvalidStructure(Box::new(e)).into()
                            })
                        })?;
                    let ssc_opens_value = raw.bytes()? as Vec<u8>;
                    if ssc_opens_table
                        .insert(ssc_opens_key, ssc_opens_value)
                        .is_some()
                    {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                }
                Ok(ssc_opens_table)
            })()
            .map_err(|e| e.annotate("ssc_opens"))?;
            let ssc_certs = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    258 => {
                        let mut ssc_certs_arr = Vec::new();
                        let len = raw.array()?;
                        while match len {
                            cbor_event::Len::Len(n) => (ssc_certs_arr.len() as u64) < n,
                            cbor_event::Len::Indefinite => true,
                        } {
                            if raw.cbor_type()? == cbor_event::Type::Special {
                                assert_eq!(raw.special()?, cbor_event::Special::Break);
                                break;
                            }
                            ssc_certs_arr.push(SscCert::deserialize(raw)?);
                        }
                        Ok(ssc_certs_arr)
                    }
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 258,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("ssc_certs"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SscOpeningsPayload {
                ssc_opens,
                ssc_certs,
            })
        })()
        .map_err(|e| e.annotate("SscOpeningsPayload"))
    }
}

impl cbor_event::se::Serialize for SscOpeningsProof {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(1u64)?;
        serializer.write_bytes(self.blake2b256.to_raw_bytes())?;
        serializer.write_bytes(self.blake2b2562.to_raw_bytes())?;
        Ok(serializer)
    }
}

impl Deserialize for SscOpeningsProof {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 1 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(1),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let blake2b256 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b256"))?;
            let blake2b2562 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b2562"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SscOpeningsProof {
                blake2b256,
                blake2b2562,
            })
        })()
        .map_err(|e| e.annotate("SscOpeningsProof"))
    }
}

impl cbor_event::se::Serialize for SscProof {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            SscProof::SscCommitmentsProof(ssc_commitments_proof) => {
                ssc_commitments_proof.serialize(serializer)
            }
            SscProof::SscOpeningsProof(ssc_openings_proof) => {
                ssc_openings_proof.serialize(serializer)
            }
            SscProof::SscSharesProof(ssc_shares_proof) => ssc_shares_proof.serialize(serializer),
            SscProof::SscCertificatesProof(ssc_certificates_proof) => {
                ssc_certificates_proof.serialize(serializer)
            }
        }
    }
}

impl Deserialize for SscProof {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let deser_variant: Result<_, DeserializeError> = SscCommitmentsProof::deserialize(raw);
            match deser_variant {
                Ok(ssc_commitments_proof) => {
                    return Ok(Self::SscCommitmentsProof(ssc_commitments_proof))
                }
                Err(e) => {
                    errs.push(e.annotate("SscCommitmentsProof"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = SscOpeningsProof::deserialize(raw);
            match deser_variant {
                Ok(ssc_openings_proof) => return Ok(Self::SscOpeningsProof(ssc_openings_proof)),
                Err(e) => {
                    errs.push(e.annotate("SscOpeningsProof"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = SscSharesProof::deserialize(raw);
            match deser_variant {
                Ok(ssc_shares_proof) => return Ok(Self::SscSharesProof(ssc_shares_proof)),
                Err(e) => {
                    errs.push(e.annotate("SscSharesProof"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let deser_variant: Result<_, DeserializeError> = SscCertificatesProof::deserialize(raw);
            match deser_variant {
                Ok(ssc_certificates_proof) => {
                    return Ok(Self::SscCertificatesProof(ssc_certificates_proof))
                }
                Err(e) => {
                    errs.push(e.annotate("SscCertificatesProof"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "SscProof",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
            ))
        })()
        .map_err(|e| e.annotate("SscProof"))
    }
}

impl cbor_event::se::Serialize for SscSharesPayload {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(2u64)?;
        serializer.write_map(cbor_event::Len::Len(self.ssc_shares.len() as u64))?;
        for (key, value) in self.ssc_shares.iter() {
            serializer.write_bytes(key.to_raw_bytes())?;
            serializer.write_map(cbor_event::Len::Len(value.len() as u64))?;
            for (key, value) in value.iter() {
                serializer.write_bytes(key.to_raw_bytes())?;
                serializer.write_array(cbor_event::Len::Indefinite)?;
                for element in value.iter() {
                    serializer.write_bytes(element)?;
                }
                serializer.write_special(cbor_event::Special::Break)?;
            }
        }
        serializer.write_tag(258u64)?;
        serializer.write_array(cbor_event::Len::Len(self.ssc_certs.len() as u64))?;
        for element in self.ssc_certs.iter() {
            element.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for SscSharesPayload {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let ssc_shares = (|| -> Result<_, DeserializeError> {
                let mut ssc_shares_table = BTreeMap::new();
                let ssc_shares_len = raw.map()?;
                while match ssc_shares_len {
                    cbor_event::Len::Len(n) => (ssc_shares_table.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let ssc_shares_key = raw
                        .bytes()
                        .map_err(Into::<DeserializeError>::into)
                        .and_then(|bytes| {
                            AddressId::from_raw_bytes(&bytes).map_err(|e| {
                                DeserializeFailure::InvalidStructure(Box::new(e)).into()
                            })
                        })?;
                    let mut ssc_shares_value_table = BTreeMap::new();
                    let ssc_shares_value_len = raw.map()?;
                    while match ssc_shares_value_len {
                        cbor_event::Len::Len(n) => (ssc_shares_value_table.len() as u64) < n,
                        cbor_event::Len::Indefinite => true,
                    } {
                        if raw.cbor_type()? == cbor_event::Type::Special {
                            assert_eq!(raw.special()?, cbor_event::Special::Break);
                            break;
                        }
                        let ssc_shares_value_key = raw
                            .bytes()
                            .map_err(Into::<DeserializeError>::into)
                            .and_then(|bytes| {
                                AddressId::from_raw_bytes(&bytes).map_err(|e| {
                                    DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                })
                            })?;
                        let mut ssc_shares_value_value_arr = Vec::new();
                        let len = raw.array()?;
                        while match len {
                            cbor_event::Len::Len(n) => {
                                (ssc_shares_value_value_arr.len() as u64) < n
                            }
                            cbor_event::Len::Indefinite => true,
                        } {
                            if raw.cbor_type()? == cbor_event::Type::Special {
                                assert_eq!(raw.special()?, cbor_event::Special::Break);
                                break;
                            }
                            ssc_shares_value_value_arr.push(raw.bytes()? as Vec<u8>);
                        }
                        let ssc_shares_value_value = ssc_shares_value_value_arr;
                        if ssc_shares_value_table
                            .insert(ssc_shares_value_key, ssc_shares_value_value)
                            .is_some()
                        {
                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                                "some complicated/unsupported type",
                            )))
                            .into());
                        }
                    }
                    let ssc_shares_value = ssc_shares_value_table;
                    if ssc_shares_table
                        .insert(ssc_shares_key, ssc_shares_value)
                        .is_some()
                    {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                }
                Ok(ssc_shares_table)
            })()
            .map_err(|e| e.annotate("ssc_shares"))?;
            let ssc_certs = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    258 => {
                        let mut ssc_certs_arr = Vec::new();
                        let len = raw.array()?;
                        while match len {
                            cbor_event::Len::Len(n) => (ssc_certs_arr.len() as u64) < n,
                            cbor_event::Len::Indefinite => true,
                        } {
                            if raw.cbor_type()? == cbor_event::Type::Special {
                                assert_eq!(raw.special()?, cbor_event::Special::Break);
                                break;
                            }
                            ssc_certs_arr.push(SscCert::deserialize(raw)?);
                        }
                        Ok(ssc_certs_arr)
                    }
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 258,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("ssc_certs"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SscSharesPayload {
                ssc_shares,
                ssc_certs,
            })
        })()
        .map_err(|e| e.annotate("SscSharesPayload"))
    }
}

impl cbor_event::se::Serialize for SscSharesProof {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(2u64)?;
        serializer.write_bytes(self.blake2b256.to_raw_bytes())?;
        serializer.write_bytes(self.blake2b2562.to_raw_bytes())?;
        Ok(serializer)
    }
}

impl Deserialize for SscSharesProof {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 2 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(2),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let blake2b256 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b256"))?;
            let blake2b2562 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b2562"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SscSharesProof {
                blake2b256,
                blake2b2562,
            })
        })()
        .map_err(|e| e.annotate("SscSharesProof"))
    }
}

impl cbor_event::se::Serialize for SscSignedCommitment {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_bytes(&self.byron_pub_key)?;
        self.ssc_commitment.serialize(serializer)?;
        serializer.write_bytes(&self.byron_signature)?;
        Ok(serializer)
    }
}

impl Deserialize for SscSignedCommitment {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let byron_pub_key = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("byron_pub_key"))?;
            let ssc_commitment = SscCommitment::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("ssc_commitment"))?;
            let byron_signature = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("byron_signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SscSignedCommitment {
                byron_pub_key,
                ssc_commitment,
                byron_signature,
            })
        })()
        .map_err(|e| e.annotate("SscSignedCommitment"))
    }
}

impl cbor_event::se::Serialize for VssEncryptedShare {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        //serializer.write_array(cbor_event::Len::Len(1))?;
        serializer.write_array(cbor_event::Len::Indefinite)?;
        serializer.write_bytes(&self.index_0)?;
        serializer.write_special(cbor_event::Special::Break)?;
        Ok(serializer)
    }
}

impl Deserialize for VssEncryptedShare {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(1)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let index_0 =
                Ok(raw.bytes()? as Vec<u8>).map_err(|e: DeserializeError| e.annotate("index_0"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(VssEncryptedShare { index_0 })
        })()
        .map_err(|e| e.annotate("VssEncryptedShare"))
    }
}

impl cbor_event::se::Serialize for VssProof {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        serializer.write_bytes(&self.extra_gen)?;
        serializer.write_bytes(&self.proof)?;
        serializer.write_bytes(&self.parallel_proofs)?;
        //serializer.write_array(cbor_event::Len::Len(self.bytess.len() as u64))?;
        serializer.write_array(cbor_event::Len::Indefinite)?;
        for element in self.bytess.iter() {
            serializer.write_bytes(element)?;
        }
        serializer.write_special(cbor_event::Special::Break)?;
        Ok(serializer)
    }
}

impl Deserialize for VssProof {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let extra_gen = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("extra_gen"))?;
            let proof =
                Ok(raw.bytes()? as Vec<u8>).map_err(|e: DeserializeError| e.annotate("proof"))?;
            let parallel_proofs = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("parallel_proofs"))?;
            let bytess = (|| -> Result<_, DeserializeError> {
                let mut bytess_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (bytess_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    bytess_arr.push(raw.bytes()? as Vec<u8>);
                }
                Ok(bytess_arr)
            })()
            .map_err(|e| e.annotate("bytess"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(VssProof {
                extra_gen,
                proof,
                parallel_proofs,
                bytess,
            })
        })()
        .map_err(|e| e.annotate("VssProof"))
    }
}
