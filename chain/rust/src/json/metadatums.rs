#[cfg(not(feature = "used_from_wasm"))]
use noop_proc_macro::wasm_bindgen;
#[cfg(feature = "used_from_wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    auxdata::{MetadatumMap, TransactionMetadatum},
    json::json_serialize::{JsonParseError, Value as JSONValue},
    utils::BigInteger,
};

use cml_core::{DeserializeError, Int};

use std::collections::BTreeMap;
use std::convert::TryFrom;

#[wasm_bindgen]
#[derive(Copy, Clone, Eq, PartialEq)]
// Different schema methods for mapping between JSON and the metadata CBOR.
// This conversion should match TxMetadataJsonSchema in cardano-node defined (at time of writing) here:
// https://github.com/input-output-hk/cardano-node/blob/master/cardano-api/src/Cardano/Api/MetaData.hs
// but has 2 additional schemas for more or less conversionse
// Note: Byte/Strings (including keys) in any schema must be at most 64 bytes in length
pub enum MetadataJsonSchema {
    // Does zero implicit conversions.
    // Round-trip conversions are 100% consistent
    // Treats maps DIRECTLY as maps in JSON in a natural way e.g. {"key1": 47, "key2": [0, 1]]}
    // From JSON:
    // * null/true/false NOT supported.
    // * keys treated as strings only
    // To JSON
    // * Bytes, non-string keys NOT supported.
    // Stricter than any TxMetadataJsonSchema in cardano-node but more natural for JSON -> Metadata
    NoConversions,
    // Does some implicit conversions.
    // Round-trip conversions MD -> JSON -> MD is NOT consistent, but JSON -> MD -> JSON is.
    // Without using bytes
    // Maps are treated as an array of k-v pairs as such: [{"key1": 47}, {"key2": [0, 1]}, {"key3": "0xFFFF"}]
    // From JSON:
    // * null/true/false NOT supported.
    // * Strings parseable as bytes (0x starting hex) or integers are converted.
    // To JSON:
    // * Non-string keys partially supported (bytes as 0x starting hex string, integer converted to string).
    // * Bytes are converted to hex strings starting with 0x for both values and keys.
    // Corresponds to TxMetadataJsonSchema's TxMetadataJsonNoSchema in cardano-node
    BasicConversions,
    // Supports the annotated schema presented in cardano-node with tagged values e.g. {"int": 7}, {"list": [0, 1]}
    // Round-trip conversions are 100% consistent
    // Maps are treated as an array of k-v pairs as such: [{"key1": {"int": 47}}, {"key2": {"list": [0, 1]}}, {"key3": {"bytes": "0xFFFF"}}]
    // From JSON:
    // * null/true/false NOT supported.
    // * Strings parseable as bytes (hex WITHOUT 0x prefix) or integers converted.
    // To JSON:
    // * Non-string keys are supported. Any key parseable as JSON is encoded as metadata instead of a string
    // Corresponds to TxMetadataJsonSchema's TxMetadataJsonDetailedSchema in cardano-node
    DetailedSchema,
}

#[derive(Debug, thiserror::Error)]
pub enum MetadataJsonError {
    #[error("JSON Parsing: {0}")]
    JsonParse(#[from] JsonParseError),
    #[error("JSON printing: {0}")]
    JsonPrinting(#[from] serde_json::Error),
    #[error("null not allowed in metadatums")]
    NullFound,
    #[error("bools not allowed in metadatums")]
    BoolFound,
    #[error("DetailedSchema key {0} does not match type {1:?}")]
    DetailedKeyMismatch(String, JSONValue),
    #[error("entry format in detailed schema map object not correct. Needs to be of form {{\"k\": \"key\", \"v\": value}}")]
    InvalidMapEntry,
    #[error("key '{0}' in tagged object not valid")]
    InvalidTag(String),
    #[error(
        "DetailedSchema requires ALL JSON to be tagged objects, found: {:?}",
        0
    )]
    DetailedNonObject(JSONValue),
    #[error("Invalid hex string: {0}")]
    InvalidHex(#[from] hex::FromHexError),
    #[error("Bytes not allowed in BasicConversions schema")]
    BytesInNoConversions,
    #[error("Metadatum ints must fit in 8 bytes: {0}")]
    IntTooBig(BigInteger),
    #[error("key type {0:?} not allowed in JSON under specified schema")]
    InvalidKeyType(TransactionMetadatum),
    #[error("Metadatum structure error (e.g. too big for bounds): {0}")]
    InvalidStructure(#[from] DeserializeError),
}

fn supports_tagged_values(schema: MetadataJsonSchema) -> bool {
    match schema {
        MetadataJsonSchema::NoConversions | MetadataJsonSchema::BasicConversions => false,
        MetadataJsonSchema::DetailedSchema => true,
    }
}

fn hex_string_to_bytes(hex: &str) -> Option<Vec<u8>> {
    if let Some(stripped) = hex.strip_prefix("0x") {
        hex::decode(stripped).ok()
    } else {
        None
    }
}

fn bytes_to_hex_string(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

/// Converts JSON to Metadata according to MetadataJsonSchema
pub fn encode_json_str_to_metadatum(
    json: &str,
    schema: MetadataJsonSchema,
) -> Result<TransactionMetadatum, MetadataJsonError> {
    let value = JSONValue::from_string(json)?;
    encode_json_value_to_metadatum(value, schema)
}

pub fn encode_json_value_to_metadatum(
    value: JSONValue,
    schema: MetadataJsonSchema,
) -> Result<TransactionMetadatum, MetadataJsonError> {
    fn encode_string(
        s: String,
        schema: MetadataJsonSchema,
    ) -> Result<TransactionMetadatum, DeserializeError> {
        if schema == MetadataJsonSchema::BasicConversions {
            match hex_string_to_bytes(&s) {
                Some(bytes) => TransactionMetadatum::new_bytes(bytes),
                None => TransactionMetadatum::new_text(s),
            }
        } else {
            TransactionMetadatum::new_text(s)
        }
    }
    fn encode_array(
        json_arr: Vec<JSONValue>,
        schema: MetadataJsonSchema,
    ) -> Result<TransactionMetadatum, MetadataJsonError> {
        json_arr
            .into_iter()
            .map(|value| encode_json_value_to_metadatum(value, schema))
            .collect::<Result<Vec<_>, MetadataJsonError>>()
            .map(TransactionMetadatum::new_list)
    }
    match schema {
        MetadataJsonSchema::NoConversions | MetadataJsonSchema::BasicConversions => match value {
            JSONValue::Null => Err(MetadataJsonError::NullFound),
            JSONValue::Bool(_) => Err(MetadataJsonError::BoolFound),
            JSONValue::Number(x) => Ok(TransactionMetadatum::new_int(
                x.as_int().ok_or(MetadataJsonError::IntTooBig(x.clone()))?,
            )),
            JSONValue::String(s) => encode_string(s, schema).map_err(Into::into),
            JSONValue::Array(json_arr) => encode_array(json_arr, schema),
            JSONValue::Object(json_obj) => {
                let mut map = MetadatumMap::new();
                for (raw_key, value) in json_obj {
                    let key =
                        if schema == MetadataJsonSchema::BasicConversions {
                            match raw_key.parse::<i128>() {
                                Ok(x) => TransactionMetadatum::new_int(Int::try_from(x).map_err(
                                    |_e| MetadataJsonError::IntTooBig(BigInteger::from(x)),
                                )?),
                                Err(_) => encode_string(raw_key, schema)?,
                            }
                        } else {
                            TransactionMetadatum::new_text(raw_key)?
                        };
                    map.set(key, encode_json_value_to_metadatum(value, schema)?);
                }
                Ok(TransactionMetadatum::new_map(map))
            }
        },
        // we rely on tagged objects to control parsing here instead
        MetadataJsonSchema::DetailedSchema => match value {
            JSONValue::Object(obj) if obj.len() == 1 => {
                let (k, v) = obj.into_iter().next().unwrap();
                match k.as_str() {
                    "int" => match v {
                        JSONValue::Number(x) => Ok(TransactionMetadatum::new_int(
                            x.as_int().ok_or(MetadataJsonError::IntTooBig(x.clone()))?,
                        )),
                        _ => Err(MetadataJsonError::DetailedKeyMismatch(k, v)),
                    },
                    "string" => match v {
                        JSONValue::String(string) => {
                            encode_string(string, schema).map_err(Into::into)
                        }
                        _ => Err(MetadataJsonError::DetailedKeyMismatch(k, v)),
                    },
                    "bytes" => match v {
                        JSONValue::String(string) => hex::decode(string)
                            .map_err(Into::into)
                            .and_then(|b| TransactionMetadatum::new_bytes(b).map_err(Into::into)),
                        _ => Err(MetadataJsonError::DetailedKeyMismatch(k, v)),
                    },
                    "list" => match v {
                        JSONValue::Array(array) => encode_array(array, schema),
                        _ => Err(MetadataJsonError::DetailedKeyMismatch(k, v)),
                    },
                    "map" => {
                        let mut map = MetadatumMap::new();

                        let array = match v {
                            JSONValue::Array(array) => Ok(array),
                            _ => Err(MetadataJsonError::DetailedKeyMismatch(k, v)),
                        }?;
                        for entry in array {
                            let entry_obj = match entry {
                                JSONValue::Object(obj) => Ok(obj),
                                _ => Err(MetadataJsonError::InvalidMapEntry),
                            }?;
                            let raw_key = entry_obj
                                .get("k")
                                .ok_or(MetadataJsonError::InvalidMapEntry)?;
                            let value = entry_obj
                                .get("v")
                                .ok_or(MetadataJsonError::InvalidMapEntry)?;
                            let key = encode_json_value_to_metadatum(raw_key.clone(), schema)?;
                            map.set(key, encode_json_value_to_metadatum(value.clone(), schema)?);
                        }
                        Ok(TransactionMetadatum::new_map(map))
                    }
                    _invalid_key => Err(MetadataJsonError::InvalidTag(k)),
                }
            }
            _ => Err(MetadataJsonError::DetailedNonObject(value)),
        },
    }
}

/// Converts Metadata to JSON according to MetadataJsonSchema
pub fn decode_metadatum_to_json_str(
    metadatum: &TransactionMetadatum,
    schema: MetadataJsonSchema,
) -> Result<String, MetadataJsonError> {
    let value = decode_metadatum_to_json_value(metadatum, schema)?;
    value.to_string().map_err(Into::into)
}

pub fn decode_metadatum_to_json_value(
    metadatum: &TransactionMetadatum,
    schema: MetadataJsonSchema,
) -> Result<JSONValue, MetadataJsonError> {
    fn decode_key(
        key: &TransactionMetadatum,
        schema: MetadataJsonSchema,
    ) -> Result<String, MetadataJsonError> {
        match key {
            TransactionMetadatum::Text { text, .. } => Ok(text.clone()),
            TransactionMetadatum::Bytes { bytes, .. }
                if schema != MetadataJsonSchema::NoConversions =>
            {
                Ok(bytes_to_hex_string(bytes.as_ref()))
            }
            TransactionMetadatum::Int(i) if schema != MetadataJsonSchema::NoConversions => {
                Ok(i.to_string())
            }
            TransactionMetadatum::List { elements, .. }
                if schema == MetadataJsonSchema::DetailedSchema =>
            {
                decode_metadatum_to_json_str(
                    &TransactionMetadatum::new_list(elements.clone()),
                    schema,
                )
            }
            TransactionMetadatum::Map(map) if schema == MetadataJsonSchema::DetailedSchema => {
                decode_metadatum_to_json_str(&TransactionMetadatum::new_map(map.clone()), schema)
            }
            _ => Err(MetadataJsonError::InvalidKeyType(key.clone())),
        }
    }
    let (type_key, value) = match metadatum {
        TransactionMetadatum::Map(map) => match schema {
            MetadataJsonSchema::NoConversions | MetadataJsonSchema::BasicConversions => {
                // treats maps directly as JSON maps
                let mut json_map = BTreeMap::new();
                for (key, value) in map.entries.iter() {
                    json_map.insert(
                        decode_key(key, schema)?,
                        decode_metadatum_to_json_value(value, schema)?,
                    );
                }
                ("map", JSONValue::from(json_map))
            }

            MetadataJsonSchema::DetailedSchema => (
                "map",
                JSONValue::from(
                    map.entries
                        .iter()
                        .map(|(key, value)| {
                            // must encode maps as JSON lists of objects with k/v keys
                            // also in these schemas we support more key types than strings
                            let k = decode_metadatum_to_json_value(key, schema)?;
                            let v = decode_metadatum_to_json_value(value, schema)?;
                            let mut kv_obj = BTreeMap::new();
                            kv_obj.insert(String::from("k"), k);
                            kv_obj.insert(String::from("v"), v);
                            Ok(JSONValue::from(kv_obj))
                        })
                        .collect::<Result<Vec<_>, MetadataJsonError>>()?,
                ),
            ),
        },
        TransactionMetadatum::List { elements, .. } => (
            "list",
            JSONValue::from(
                elements
                    .iter()
                    .map(|e| decode_metadatum_to_json_value(e, schema))
                    .collect::<Result<Vec<_>, MetadataJsonError>>()?,
            ),
        ),
        TransactionMetadatum::Int(x) => ("int", JSONValue::Number(BigInteger::from_int(x))),
        TransactionMetadatum::Bytes { bytes, .. } => (
            "bytes",
            match schema {
                MetadataJsonSchema::NoConversions => Err(MetadataJsonError::BytesInNoConversions),
                // 0x prefix
                MetadataJsonSchema::BasicConversions => {
                    Ok(JSONValue::from(bytes_to_hex_string(bytes.as_ref())))
                }
                // no prefix
                MetadataJsonSchema::DetailedSchema => Ok(JSONValue::from(hex::encode(bytes))),
            }?,
        ),
        TransactionMetadatum::Text { text, .. } => ("string", JSONValue::from(text.clone())),
    };
    // potentially wrap value in a keyed map to represent more types
    if supports_tagged_values(schema) {
        let mut wrapper = BTreeMap::new();
        wrapper.insert(String::from(type_key), value);
        Ok(JSONValue::from(wrapper))
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_encoding_no_conversions() {
        let input_str = "{\"receiver_id\": \"SJKdj34k3jjKFDKfjFUDfdjkfd\",\"sender_id\": \"jkfdsufjdk34h3Sdfjdhfduf873\",\"comment\": \"happy birthday\",\"tags\": [0, 264, -1024, 32]}";
        let metadata = encode_json_str_to_metadatum(input_str, MetadataJsonSchema::NoConversions)
            .expect("encode failed");
        let map = metadata.as_map().unwrap();
        assert_eq!(
            map.get_str("receiver_id").unwrap().as_text().unwrap(),
            "SJKdj34k3jjKFDKfjFUDfdjkfd"
        );
        assert_eq!(
            map.get_str("sender_id").unwrap().as_text().unwrap(),
            "jkfdsufjdk34h3Sdfjdhfduf873"
        );
        assert_eq!(
            map.get_str("comment").unwrap().as_text().unwrap(),
            "happy birthday"
        );
        let tags = map.get_str("tags").unwrap().as_list().unwrap();
        let tags_i32 = tags
            .iter()
            .map(|md| md.as_int().unwrap().into())
            .collect::<Vec<i128>>();
        assert_eq!(tags_i32, vec![0, 264, -1024, 32]);
        let output_str = decode_metadatum_to_json_str(&metadata, MetadataJsonSchema::NoConversions)
            .expect("decode failed");
        let input_json: serde_json::Value = serde_json::from_str(input_str).unwrap();
        let output_json: serde_json::Value = serde_json::from_str(&output_str).unwrap();
        assert_eq!(input_json, output_json);
    }

    #[test]
    fn json_encoding_basic() {
        let input_str =
            "{\"0x8badf00d\": \"0xdeadbeef\",\"9\": 5,\"obj\": {\"a\":[{\"5\": 2},{}]}}";
        let metadata =
            encode_json_str_to_metadatum(input_str, MetadataJsonSchema::BasicConversions)
                .expect("encode failed");
        json_encoding_check_example_metadatum(&metadata);
        let output_str =
            decode_metadatum_to_json_str(&metadata, MetadataJsonSchema::BasicConversions)
                .expect("decode failed");
        let input_json: serde_json::Value = serde_json::from_str(input_str).unwrap();
        let output_json: serde_json::Value = serde_json::from_str(&output_str).unwrap();
        assert_eq!(input_json, output_json);
    }

    #[test]
    fn json_encoding_detailed() {
        let input_str = "{\"map\":[
            {
                \"k\":{\"bytes\":\"8badf00d\"},
                \"v\":{\"bytes\":\"deadbeef\"}
            },
            {
                \"k\":{\"int\":9},
                \"v\":{\"int\":5}
            },
            {
                \"k\":{\"string\":\"obj\"},
                \"v\":{\"map\":[
                    {
                        \"k\":{\"string\":\"a\"},
                        \"v\":{\"list\":[
                        {\"map\":[
                            {
                                \"k\":{\"int\":5},
                                \"v\":{\"int\":2}
                            }
                            ]},
                            {\"map\":[
                            ]}
                        ]}
                    }
                ]}
            }
        ]}";
        let metadata = encode_json_str_to_metadatum(input_str, MetadataJsonSchema::DetailedSchema)
            .expect("encode failed");
        json_encoding_check_example_metadatum(&metadata);
        let output_str =
            decode_metadatum_to_json_str(&metadata, MetadataJsonSchema::DetailedSchema)
                .expect("decode failed");
        let input_json: serde_json::Value = serde_json::from_str(input_str).unwrap();
        let output_json: serde_json::Value = serde_json::from_str(&output_str).unwrap();
        assert_eq!(input_json, output_json);
    }

    fn json_encoding_check_example_metadatum(metadata: &TransactionMetadatum) {
        let map = metadata.as_map().unwrap();
        assert_eq!(
            *map.get(&TransactionMetadatum::new_bytes(hex::decode("8badf00d").unwrap()).unwrap())
                .unwrap()
                .as_bytes()
                .unwrap(),
            hex::decode("deadbeef").unwrap()
        );
        assert_eq!(
            i128::from(
                map.get(&TransactionMetadatum::new_int(Int::from(9u64)))
                    .unwrap()
                    .as_int()
                    .unwrap()
            ),
            5
        );
        let inner_map = map.get_str("obj").unwrap().as_map().unwrap();
        let a = inner_map.get_str("a").unwrap().as_list().unwrap();
        let a1 = a[0].as_map().unwrap();
        assert_eq!(
            i128::from(
                a1.get(&TransactionMetadatum::new_int(Int::from(5u64)))
                    .unwrap()
                    .as_int()
                    .unwrap()
            ),
            2
        );
        let a2 = a[1].as_map().unwrap();
        assert_eq!(a2.len(), 0);
    }

    #[test]
    fn json_encoding_detailed_complex_key() {
        let input_str = "{\"map\":[
            {
            \"k\":{\"list\":[
                {\"map\": [
                    {
                        \"k\": {\"int\": 5},
                        \"v\": {\"int\": -7}
                    },
                    {
                        \"k\": {\"string\": \"hello\"},
                        \"v\": {\"string\": \"world\"}
                    }
                ]},
                {\"bytes\": \"ff00ff00\"}
            ]},
            \"v\":{\"int\":5}
            }
        ]}";
        let metadata = encode_json_str_to_metadatum(input_str, MetadataJsonSchema::DetailedSchema)
            .expect("encode failed");

        let map = metadata.as_map().unwrap();
        let (k, v) = map.entries.first().unwrap();
        assert_eq!(i128::from(v.as_int().unwrap()), 5i128);
        let key_list = k.as_list().unwrap();
        assert_eq!(key_list.len(), 2);
        let key_map = key_list[0].as_map().unwrap();
        assert_eq!(
            i128::from(
                key_map
                    .get(&TransactionMetadatum::new_int(Int::from(5u64)))
                    .unwrap()
                    .as_int()
                    .unwrap()
            ),
            -7i128
        );
        assert_eq!(
            key_map.get_str("hello").unwrap().as_text().unwrap(),
            "world"
        );
        let key_bytes = key_list[1].as_bytes().unwrap();
        assert_eq!(*key_bytes, hex::decode("ff00ff00").unwrap());

        let output_str =
            decode_metadatum_to_json_str(&metadata, MetadataJsonSchema::DetailedSchema)
                .expect("decode failed");
        let input_json: serde_json::Value = serde_json::from_str(input_str).unwrap();
        let output_json: serde_json::Value = serde_json::from_str(&output_str).unwrap();
        assert_eq!(input_json, output_json);
    }
}
