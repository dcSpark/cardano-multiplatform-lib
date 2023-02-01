use crate::error::{DeserializeError, DeserializeFailure, Key};
use crate::ordered_hash_map::OrderedHashMap;
use crate::serialization::{Deserialize, LenEncoding, Serialize, StringEncoding};
use crate::Int;
use cbor_event::{de::Deserializer, se::Serializer};
use derivative::Derivative;

use std::io::{BufRead, Seek, SeekFrom, Write};

pub type TransactionMetadatumLabel = u64;

pub type Metadata = OrderedHashMap<TransactionMetadatumLabel, TransactionMetadatum>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Derivative)]
#[derivative(
    Eq,
    PartialEq,
    Ord = "feature_allow_slow_enum",
    PartialOrd = "feature_allow_slow_enum",
    Hash
)]
pub enum TransactionMetadatum {
    Map {
        entries: OrderedHashMap<TransactionMetadatum, TransactionMetadatum>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        entries_encoding: LenEncoding,
    },
    List {
        elements: Vec<TransactionMetadatum>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        elements_encoding: LenEncoding,
    },
    Int(Int),
    Bytes {
        bytes: Vec<u8>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        bytes_encoding: StringEncoding,
    },
    Text {
        text: String,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        #[serde(skip)]
        text_encoding: StringEncoding,
    },
}

impl TransactionMetadatum {
    pub fn new_map(entries: OrderedHashMap<TransactionMetadatum, TransactionMetadatum>) -> Self {
        Self::Map {
            entries,
            entries_encoding: LenEncoding::default(),
        }
    }

    pub fn new_list(elements: Vec<TransactionMetadatum>) -> Self {
        Self::List {
            elements,
            elements_encoding: LenEncoding::default(),
        }
    }

    pub fn new_int(int: Int) -> Self {
        Self::Int(int)
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self::Bytes {
            bytes,
            bytes_encoding: StringEncoding::default(),
        }
    }

    pub fn new_text(text: String) -> Self {
        Self::Text {
            text,
            text_encoding: StringEncoding::default(),
        }
    }
}

impl Serialize for TransactionMetadatum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            TransactionMetadatum::Map {
                entries,
                entries_encoding,
            } => {
                serializer.write_map_sz(
                    entries_encoding.to_len_sz(entries.len() as u64, force_canonical),
                )?;
                let mut key_order = entries
                    .iter()
                    .map(|(k, v)| {
                        let mut buf = cbor_event::se::Serializer::new_vec();
                        k.serialize(&mut buf, force_canonical)?;
                        Ok((buf.finalize(), k, v))
                    })
                    .collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
                if force_canonical {
                    key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                        match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                            std::cmp::Ordering::Equal => lhs_bytes.cmp(rhs_bytes),
                            diff_ord => diff_ord,
                        }
                    });
                }
                for (key_bytes, _key, value) in key_order {
                    serializer.write_raw_bytes(&key_bytes)?;
                    value.serialize(serializer, force_canonical)?;
                }
                entries_encoding.end(serializer, force_canonical)
            }
            TransactionMetadatum::List {
                elements,
                elements_encoding,
            } => {
                serializer.write_array_sz(
                    elements_encoding.to_len_sz(elements.len() as u64, force_canonical),
                )?;
                for element in elements.iter() {
                    element.serialize(serializer, force_canonical)?;
                }
                elements_encoding.end(serializer, force_canonical)
            }
            TransactionMetadatum::Int(int) => int.serialize(serializer, force_canonical),
            TransactionMetadatum::Bytes {
                bytes,
                bytes_encoding,
            } => serializer.write_bytes_sz(
                &bytes,
                bytes_encoding.to_str_len_sz(bytes.len() as u64, force_canonical),
            ),
            TransactionMetadatum::Text {
                text,
                text_encoding,
            } => serializer.write_text_sz(
                &text,
                text_encoding.to_str_len_sz(text.len() as u64, force_canonical),
            ),
        }
    }
}

impl Deserialize for TransactionMetadatum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut entries_table = OrderedHashMap::new();
                let entries_len = raw.map_sz()?;
                let entries_encoding = entries_len.into();
                while match entries_len {
                    cbor_event::LenSz::Len(n, _) => (entries_table.len() as u64) < n,
                    cbor_event::LenSz::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let entries_key = TransactionMetadatum::deserialize(raw)?;
                    let entries_value = TransactionMetadatum::deserialize(raw)?;
                    if entries_table
                        .insert(entries_key.clone(), entries_value)
                        .is_some()
                    {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                }
                Ok((entries_table, entries_encoding))
            })(raw)
            {
                Ok((entries, entries_encoding)) => {
                    return Ok(Self::Map {
                        entries,
                        entries_encoding,
                    })
                }
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut elements_arr = Vec::new();
                let len = raw.array_sz()?;
                let elements_encoding = len.into();
                while match len {
                    cbor_event::LenSz::Len(n, _) => (elements_arr.len() as u64) < n,
                    cbor_event::LenSz::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    elements_arr.push(TransactionMetadatum::deserialize(raw)?);
                }
                Ok((elements_arr, elements_encoding))
            })(raw)
            {
                Ok((elements, elements_encoding)) => {
                    return Ok(Self::List {
                        elements,
                        elements_encoding,
                    })
                }
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            let deser_variant: Result<_, DeserializeError> = Int::deserialize(raw);
            match deser_variant {
                Ok(int) => return Ok(Self::Int(int)),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            let deser_variant: Result<_, DeserializeError> = raw
                .bytes_sz()
                .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))
                .map_err(Into::<DeserializeError>::into);
            match deser_variant {
                Ok((bytes, bytes_encoding)) => {
                    return Ok(Self::Bytes {
                        bytes,
                        bytes_encoding,
                    })
                }
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            let deser_variant: Result<_, DeserializeError> = raw
                .text_sz()
                .map(|(s, enc)| (s, StringEncoding::from(enc)))
                .map_err(Into::<DeserializeError>::into);
            match deser_variant {
                Ok((text, text_encoding)) => {
                    return Ok(Self::Text {
                        text,
                        text_encoding,
                    })
                }
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            Err(DeserializeError::new(
                "TransactionMetadatum",
                DeserializeFailure::NoVariantMatched,
            ))
        })()
        .map_err(|e| e.annotate("TransactionMetadatum"))
    }
}
