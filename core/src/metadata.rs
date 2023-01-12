use derivative::Derivative;
use crate::Int;
use crate::error::{DeserializeError, DeserializeFailure, Key};
use crate::ordered_hash_map::OrderedHashMap;
use crate::serialization::{
  Serialize,
  Deserialize,
  StringEncoding,
  LenEncoding,
};
use cbor_event::{de::Deserializer, se::Serializer};

use std::io::{BufRead, Write, Seek, SeekFrom};

pub type TransactionMetadatumLabel = u64;

pub type Metadata = OrderedHashMap<TransactionMetadatumLabel, TransactionMetadatum>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Derivative)]
#[derivative(Eq, PartialEq, Ord="feature_allow_slow_enum", PartialOrd="feature_allow_slow_enum", Hash)]
pub enum TransactionMetadatum {
    Map {
        entries: OrderedHashMap<TransactionMetadatum, TransactionMetadatum>,
        #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
        #[serde(skip)]
        entries_encoding: LenEncoding,
    },
    List {
        elements: Vec<TransactionMetadatum>,
        #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
        #[serde(skip)]
        elements_encoding: LenEncoding,
    },
    Int(Int),
    Bytes {
        bytes: Vec<u8>,
        #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
        #[serde(skip)]
        bytes_encoding: StringEncoding,
    },
    Text {
        text: String,
        #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
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
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>, force_canonical: bool) -> cbor_event::Result<&'se mut Serializer<W>> {
      match self {
          TransactionMetadatum::Map{ entries, entries_encoding } => {
              serializer.write_map_sz(entries_encoding.to_len_sz(entries.len() as u64, force_canonical))?;
              let mut key_order = entries.iter().map(|(k, v)| {
                  let mut buf = cbor_event::se::Serializer::new_vec();
                  k.serialize(&mut buf, force_canonical)?;
                  Ok((buf.finalize(), k, v))
              }).collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
              if force_canonical {
                  key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                      match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                          std::cmp::Ordering::Equal => lhs_bytes.cmp(&rhs_bytes),
                          diff_ord => diff_ord,
                      }
                  });
              }
              for (key_bytes, key, value) in key_order {
                  serializer.write_raw_bytes(&key_bytes)?;
                  value.serialize(serializer, force_canonical)?;
              }
              entries_encoding.end(serializer, force_canonical)
          },
          TransactionMetadatum::List{ elements, elements_encoding } => {
              serializer.write_array_sz(elements_encoding.to_len_sz(elements.len() as u64, force_canonical))?;
              for element in elements.iter() {
                  element.serialize(serializer, force_canonical)?;
              }
              elements_encoding.end(serializer, force_canonical)
          },
          TransactionMetadatum::Int(int) => {
              int.serialize(serializer, force_canonical)
          },
          TransactionMetadatum::Bytes{ bytes, bytes_encoding } => {
              serializer.write_bytes_sz(&bytes, bytes_encoding.to_str_len_sz(bytes.len() as u64, force_canonical))
          },
          TransactionMetadatum::Text{ text, text_encoding } => {
              serializer.write_text_sz(&text, text_encoding.to_str_len_sz(text.len() as u64, force_canonical))
          },
      }
  }
}

impl Deserialize for TransactionMetadatum {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      (|| -> Result<_, DeserializeError> {
          let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
          match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
              let mut map_transaction_metadatum_to_transaction_metadatum_table = OrderedHashMap::new();
              let map_transaction_metadatum_to_transaction_metadatum_len = raw.map_sz()?;
              let entries_encoding = map_transaction_metadatum_to_transaction_metadatum_len.into();
              while match map_transaction_metadatum_to_transaction_metadatum_len { cbor_event::LenSz::Len(n, _) => map_transaction_metadatum_to_transaction_metadatum_table.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                  if raw.cbor_type()? == cbor_event::Type::Special {
                      assert_eq!(raw.special()?, cbor_event::Special::Break);
                      break;
                  }
                  let map_transaction_metadatum_to_transaction_metadatum_key = TransactionMetadatum::deserialize(raw)?;
                  let map_transaction_metadatum_to_transaction_metadatum_value = TransactionMetadatum::deserialize(raw)?;
                  if map_transaction_metadatum_to_transaction_metadatum_table.insert(map_transaction_metadatum_to_transaction_metadatum_key.clone(), map_transaction_metadatum_to_transaction_metadatum_value).is_some() {
                      return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                  }
              }
              Ok((map_transaction_metadatum_to_transaction_metadatum_table, entries_encoding))
          })(raw)
          {
              Ok((entries, entries_encoding)) => return Ok(Self::Map {
                  entries,
                  entries_encoding,
              }),
              Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
          };
          match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
              let mut arr_transaction_metadatum_arr = Vec::new();
              let len = raw.array_sz()?;
              let elements_encoding = len.into();
              while match len { cbor_event::LenSz::Len(n, _) => arr_transaction_metadatum_arr.len() < n as usize, cbor_event::LenSz::Indefinite => true, } {
                  if raw.cbor_type()? == cbor_event::Type::Special {
                      assert_eq!(raw.special()?, cbor_event::Special::Break);
                      break;
                  }
                  arr_transaction_metadatum_arr.push(TransactionMetadatum::deserialize(raw)?);
              }
              Ok((arr_transaction_metadatum_arr, elements_encoding))
          })(raw)
          {
              Ok((elements, elements_encoding)) => return Ok(Self::List {
                  elements,
                  elements_encoding,
              }),
              Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
          };
          match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
              Ok(Int::deserialize(raw)?)
          })(raw)
          {
              Ok(int) => return Ok(Self::Int(int)),
              Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
          };
          match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
              Ok(raw.bytes_sz().map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?)
          })(raw)
          {
              Ok((bytes, bytes_encoding)) => return Ok(Self::Bytes {
                  bytes,
                  bytes_encoding,
              }),
              Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
          };
          match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
              Ok(raw.text_sz().map(|(s, enc)| (s, StringEncoding::from(enc)))?)
          })(raw)
          {
              Ok((text, text_encoding)) => return Ok(Self::Text {
                  text,
                  text_encoding,
              }),
              Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
          };
          Err(DeserializeError::new("TransactionMetadatum", DeserializeFailure::NoVariantMatched.into()))
      })().map_err(|e| e.annotate("TransactionMetadatum"))
  }
}