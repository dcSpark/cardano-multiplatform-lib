use crate::json::metadatums::{
    decode_metadatum_to_json_value, encode_json_value_to_metadatum, MetadataJsonSchema,
};
use cbor_event::{de::Deserializer, se::Serializer};
use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    serialization::{fit_sz, Deserialize, LenEncoding, Serialize, StringEncoding},
    Int,
};
use derivative::Derivative;

use std::io::{BufRead, Seek, Write};

pub type TransactionMetadatumLabel = u64;

pub const METADATA_MAX_LEN: usize = 64;

/// Collection of TransactionMetadatums indexed by TransactionMetadatumLabels
/// Handles the extremely rare edge-case of in previous generations allowing
/// duplicate metadatum labels.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Metadata {
    pub entries: Vec<(TransactionMetadatumLabel, TransactionMetadatum)>,
    #[serde(skip)]
    pub encodings: Option<MetadataEncoding>,
}

impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Replaces all metadatums of a given label, if any exist.
    pub fn set(&mut self, label: TransactionMetadatumLabel, datum: TransactionMetadatum) {
        self.entries.retain(|(l, _)| *l != label);
        self.entries.push((label, datum));
    }

    /// Gets the Metadatum corresponding to a given label, if it exists.
    /// Note: In the case of duplicate labels this only returns the first metadatum.
    /// This is an extremely rare occurence on-chain but can happen.
    pub fn get(&self, label: TransactionMetadatumLabel) -> Option<&TransactionMetadatum> {
        self.entries
            .iter()
            .find(|(l, _)| *l == label)
            .map(|(_, md)| md)
    }

    /// In the extremely unlikely situation there are duplicate labels, this gets all of a single label
    pub fn get_all(&self, label: TransactionMetadatumLabel) -> Option<Vec<&TransactionMetadatum>> {
        let matches = self
            .entries
            .iter()
            .filter_map(|(l, md)| if *l == label { Some(md) } else { None })
            .collect::<Vec<_>>();
        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MetadataEncoding {
    pub len_encoding: LenEncoding,
    pub label_encodings: Vec<cbor_event::Sz>,
}

impl Serialize for Metadata {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(self.entries.len() as u64, force_canonical),
        )?;
        let mut key_order = Vec::new();
        for (i, (label, datum)) in self.entries.iter().enumerate() {
            let mut buf = cbor_event::se::Serializer::new_vec();
            let metadata_key_encoding = self
                .encodings
                .as_ref()
                .and_then(|encs| encs.label_encodings.get(i))
                .cloned();
            buf.write_unsigned_integer_sz(
                *label,
                fit_sz(*label, metadata_key_encoding, force_canonical),
            )?;
            key_order.push((buf.finalize(), label, datum));
        }
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
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for Metadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut entries = Vec::new();
        let len = raw.map_sz()?;
        let len_encoding = len.into();
        let mut label_encodings = Vec::new();
        while match len {
            cbor_event::LenSz::Len(n, _) => (entries.len() as u64) < n,
            cbor_event::LenSz::Indefinite => true,
        } {
            if raw.cbor_type()? == cbor_event::Type::Special {
                assert_eq!(raw.special()?, cbor_event::Special::Break);
                break;
            }
            let (metadatum_label, label_encoding) = raw.unsigned_integer_sz()?;
            let metadatum = TransactionMetadatum::deserialize(raw)?;
            entries.push((metadatum_label, metadatum));
            label_encodings.push(label_encoding);
        }
        Ok(Self {
            entries,
            encodings: Some(MetadataEncoding {
                len_encoding,
                label_encodings,
            }),
        })
    }
}

/// Handles the extremely rare (2 total instances on mainnet) edge-case of in
/// previous generations allowing duplicate metadatum keys.
#[derive(Clone, Debug, Default, Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MetadatumMap {
    pub entries: Vec<(TransactionMetadatum, TransactionMetadatum)>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub entries_encoding: LenEncoding,
}

impl MetadatumMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Replaces all metadatums of a given key, if any exist.
    pub fn set(&mut self, key: TransactionMetadatum, datum: TransactionMetadatum) {
        self.entries.retain(|(k, _)| *k != key);
        self.entries.push((key, datum));
    }

    /// Gets the Metadatum corresponding to a given key, if it exists.
    /// Note: In the case of duplicate keys this only returns the first metadatum.
    /// This is an extremely rare occurence (2 total on mainnet) on-chain but can happen.
    pub fn get(&self, key: &TransactionMetadatum) -> Option<&TransactionMetadatum> {
        self.entries
            .iter()
            .find(|(k, _)| *k == *key)
            .map(|(_, md)| md)
    }

    /// In the extremely unlikely situation there are duplicate keys, this gets all of a single key
    pub fn get_all(&self, key: &TransactionMetadatum) -> Option<Vec<&TransactionMetadatum>> {
        let matches = self
            .entries
            .iter()
            .filter_map(|(k, md)| if *k == *key { Some(md) } else { None })
            .collect::<Vec<_>>();
        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }

    /// Gets the Metadatum by string only. Convenience functionality for get()
    pub fn get_str(&self, key: &str) -> Option<&TransactionMetadatum> {
        self.get(&TransactionMetadatum::new_text(key.to_owned()).ok()?)
    }
}

impl Serialize for MetadatumMap {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(
            self.entries_encoding
                .to_len_sz(self.entries.len() as u64, force_canonical),
        )?;
        let mut key_order = self
            .entries
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
        self.entries_encoding.end(serializer, force_canonical)
    }
}

impl Deserialize for MetadatumMap {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let mut entries = Vec::new();
        let entries_len = raw.map_sz()?;
        let entries_encoding = entries_len.into();
        while match entries_len {
            cbor_event::LenSz::Len(n, _) => (entries.len() as u64) < n,
            cbor_event::LenSz::Indefinite => true,
        } {
            if raw.cbor_type()? == cbor_event::Type::Special {
                assert_eq!(raw.special()?, cbor_event::Special::Break);
                break;
            }
            let key = TransactionMetadatum::deserialize(raw)?;
            let value = TransactionMetadatum::deserialize(raw)?;
            entries.push((key, value));
        }
        Ok(Self {
            entries,
            entries_encoding,
        })
    }
}

#[derive(Clone, Debug, Derivative)]
#[derivative(
    Eq,
    PartialEq,
    Ord = "feature_allow_slow_enum",
    PartialOrd = "feature_allow_slow_enum",
    Hash
)]
pub enum TransactionMetadatum {
    Map(MetadatumMap),
    List {
        elements: Vec<TransactionMetadatum>,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
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
        text_encoding: StringEncoding,
    },
}

impl TransactionMetadatum {
    pub fn new_map(map: MetadatumMap) -> Self {
        Self::Map(map)
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

    pub fn new_bytes(bytes: Vec<u8>) -> Result<Self, DeserializeError> {
        if bytes.len() > METADATA_MAX_LEN {
            return Err(DeserializeFailure::RangeCheck {
                found: bytes.len() as isize,
                min: None,
                max: Some(METADATA_MAX_LEN as isize),
            }
            .into());
        }
        Ok(Self::Bytes {
            bytes,
            bytes_encoding: StringEncoding::default(),
        })
    }

    pub fn new_text(text: String) -> Result<Self, DeserializeError> {
        if text.len() > METADATA_MAX_LEN {
            return Err(DeserializeFailure::RangeCheck {
                found: text.len() as isize,
                min: None,
                max: Some(METADATA_MAX_LEN as isize),
            }
            .into());
        }
        Ok(Self::Text {
            text,
            text_encoding: StringEncoding::default(),
        })
    }

    pub fn as_map(&self) -> Option<&MetadatumMap> {
        match self {
            Self::Map(map) => Some(map),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<TransactionMetadatum>> {
        match self {
            Self::List { elements, .. } => Some(elements),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<&Int> {
        match self {
            Self::Int(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&Vec<u8>> {
        match self {
            Self::Bytes { bytes, .. } => Some(bytes),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&String> {
        match self {
            Self::Text { text, .. } => Some(text),
            _ => None,
        }
    }
}

impl serde::Serialize for TransactionMetadatum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let json_value = decode_metadatum_to_json_value(self, MetadataJsonSchema::DetailedSchema)
            .expect("DetailedSchema can represent everything");
        serde_json::Value::from(json_value).serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for TransactionMetadatum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let serde_json_value =
            <serde_json::Value as serde::de::Deserialize>::deserialize(deserializer)?;
        let json_value = crate::json::json_serialize::Value::from(serde_json_value);
        encode_json_value_to_metadatum(json_value.clone(), MetadataJsonSchema::DetailedSchema)
            .map_err(|_e| {
                serde::de::Error::invalid_value(
                    (&json_value).into(),
                    &"invalid tx metadatum (cardano-node JSON format)",
                )
            })
    }
}

impl schemars::JsonSchema for TransactionMetadatum {
    fn schema_name() -> String {
        String::from("TransactionMetadatum")
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::from(schemars::schema::SchemaObject::new_ref(
            "TransactionMetadatum".to_owned(),
        ))
    }

    fn is_referenceable() -> bool {
        true
    }
}

impl Serialize for TransactionMetadatum {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            TransactionMetadatum::Map(map) => map.serialize(serializer, force_canonical),
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
                bytes,
                bytes_encoding.to_str_len_sz(bytes.len() as u64, force_canonical),
            ),
            TransactionMetadatum::Text {
                text,
                text_encoding,
            } => serializer.write_text_sz(
                text,
                text_encoding.to_str_len_sz(text.len() as u64, force_canonical),
            ),
        }
    }
}

impl Deserialize for TransactionMetadatum {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::Map => MetadatumMap::deserialize(raw).map(Self::Map),
                cbor_event::Type::Array => {
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
                    Ok(Self::List {
                        elements: elements_arr,
                        elements_encoding,
                    })
                }
                cbor_event::Type::UnsignedInteger | cbor_event::Type::NegativeInteger => {
                    Int::deserialize(raw).map(Self::Int)
                }
                cbor_event::Type::Bytes => raw
                    .bytes_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(bytes, enc)| {
                        if bytes.len() > METADATA_MAX_LEN {
                            Err(DeserializeFailure::RangeCheck {
                                found: bytes.len() as isize,
                                min: None,
                                max: Some(METADATA_MAX_LEN as isize),
                            }
                            .into())
                        } else {
                            Ok(Self::Bytes {
                                bytes,
                                bytes_encoding: StringEncoding::from(enc),
                            })
                        }
                    }),
                cbor_event::Type::Text => raw
                    .text_sz()
                    .map_err(Into::<DeserializeError>::into)
                    .and_then(|(text, enc)| {
                        if text.len() > METADATA_MAX_LEN {
                            Err(DeserializeFailure::RangeCheck {
                                found: text.len() as isize,
                                min: None,
                                max: Some(METADATA_MAX_LEN as isize),
                            }
                            .into())
                        } else {
                            Ok(Self::Text {
                                text,
                                text_encoding: StringEncoding::from(enc),
                            })
                        }
                    }),
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
        .map_err(|e| e.annotate("TransactionMetadatum"))
    }
}

/// encodes arbitrary bytes into chunks of 64 bytes (the limit for bytes) as a list to be valid Metadata
pub fn encode_arbitrary_bytes_as_metadatum(bytes: &[u8]) -> TransactionMetadatum {
    let mut list = Vec::new();
    for chunk in bytes.chunks(METADATA_MAX_LEN) {
        list.push(
            TransactionMetadatum::new_bytes(chunk.to_vec())
                .expect("this should never fail as we are already chunking it"),
        );
    }
    TransactionMetadatum::new_list(list)
}

/// decodes from chunks of bytes in a list to a byte vector if that is the metadata format, otherwise returns None
pub fn decode_arbitrary_bytes_from_metadatum(metadata: &TransactionMetadatum) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    for elem in metadata.as_list()? {
        bytes.extend(elem.as_bytes()?.iter());
    }
    Some(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_duplicate_labels() {
        let bytes_hex = "a219270fa16474657374747365636f6e64206d657461646174612066696c6519270fa16474657374736669727374206d657461646174612066696c65";
        let md = Metadata::from_cbor_bytes(&hex::decode(bytes_hex).unwrap()).unwrap();
        assert_eq!(bytes_hex, hex::encode(md.to_cbor_bytes()));
    }

    #[test]
    fn metdatum_duplicate_keys() {
        let bytes_hex = "a100a567536572766963656c4c4946542042616c6c6f7473685175657374696f6e6d536f6d65207175657374696f6e66417574686f7273736f6d652d677569642d686572652d736f6f6e64547970656653696e676c656743686f69636573a26643686f6963656b536f6d652043686f6963656643686f69636573536f6d6520416e6f746865722043686f696365";
        let md = Metadata::from_cbor_bytes(&hex::decode(bytes_hex).unwrap()).unwrap();
        assert_eq!(bytes_hex, hex::encode(md.to_cbor_bytes()));
    }

    #[test]
    fn binary_encoding() {
        let input_bytes = (0..1000).map(|x| x as u8).collect::<Vec<u8>>();
        let metadata = encode_arbitrary_bytes_as_metadatum(input_bytes.as_ref());
        let output_bytes = decode_arbitrary_bytes_from_metadatum(&metadata).expect("decode failed");
        assert_eq!(input_bytes, output_bytes);
    }

    #[test]
    fn metadatum_default_json() {
        let json_str = "{\"map\":[{\"k\":{\"list\":[{\"map\":[{\"k\":{\"int\":5},\"v\":{\"int\":-7}},{\"k\":{\"string\":\"hello\"},\"v\":{\"string\":\"world\"}}]},{\"bytes\":\"ff00ff00\"}]},\"v\":{\"int\":5}}]}";
        let metadatum: TransactionMetadatum = serde_json::from_str(json_str).unwrap();
        let roundtrip_str = serde_json::to_string(&metadatum).unwrap();
        assert_eq!(json_str, roundtrip_str);
    }
}
