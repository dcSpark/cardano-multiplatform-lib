
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

fn supports_tagged_values(schema: MetadataJsonSchema) -> bool {
    match schema {
        MetadataJsonSchema::NoConversions |
        MetadataJsonSchema::BasicConversions => false,
        MetadataJsonSchema::DetailedSchema => true,
    }
}

fn hex_string_to_bytes(hex: &str) -> Option<Vec<u8>> {
    if hex.starts_with("0x") {
        hex::decode(&hex[2..]).ok()
    } else {
        None
    }
}

fn bytes_to_hex_string(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

// Converts JSON to Metadata according to MetadataJsonSchema
#[wasm_bindgen]
pub fn encode_json_str_to_metadatum(json: String, schema: MetadataJsonSchema) -> Result<TransactionMetadatum, JsError> {
    let value = json_serialize::Value::from_string(json)?;
    encode_json_value_to_metadatum(value, schema)
}

pub fn encode_json_value_to_metadatum(value: json_serialize::Value, schema: MetadataJsonSchema) -> Result<TransactionMetadatum, JsError> {
    fn encode_string(s: String, schema: MetadataJsonSchema) -> Result<TransactionMetadatum, JsError> {
        if schema == MetadataJsonSchema::BasicConversions {
            match hex_string_to_bytes(&s) {
                Some(bytes) => TransactionMetadatum::new_bytes(bytes),
                None => TransactionMetadatum::new_text(s),
            }
        } else {
            TransactionMetadatum::new_text(s)
        }
    }
    fn encode_array(json_arr: Vec<json_serialize::Value>, schema: MetadataJsonSchema) -> Result<TransactionMetadatum, JsError> {
        let mut arr = MetadataList::new();
        for value in json_arr {
            arr.add(&encode_json_value_to_metadatum(value, schema)?);
        }
        Ok(TransactionMetadatum::new_list(&arr))
    }
    match schema {
        MetadataJsonSchema::NoConversions |
        MetadataJsonSchema::BasicConversions => match value {
            json_serialize::Value::Null => Err(JsError::from_str("null not allowed in metadata")),
            json_serialize::Value::Bool(_) => Err(JsError::from_str("bools not allowed in metadata")),
            json_serialize::Value::Number(x) => Ok(TransactionMetadatum::new_int(&Int::from_str(x.to_str().as_str())?)),
            json_serialize::Value::String(s) => encode_string(s, schema),
            json_serialize::Value::Array(json_arr) => encode_array(json_arr, schema),
            json_serialize::Value::Object(json_obj) => {
                let mut map = MetadataMap::new();
                for (raw_key, value) in json_obj {
                    let key = if schema == MetadataJsonSchema::BasicConversions {
                        match raw_key.parse::<i128>() {
                            Ok(x) => TransactionMetadatum::new_int(&Int(x)),
                            Err(_) => encode_string(raw_key, schema)?,
                        }
                    } else {
                        TransactionMetadatum::new_text(raw_key)?
                    };
                    map.insert(&key, &encode_json_value_to_metadatum(value, schema)?);
                }
                Ok(TransactionMetadatum::new_map(&map))
            },
        },
        // we rely on tagged objects to control parsing here instead
        MetadataJsonSchema::DetailedSchema => match value {
            json_serialize::Value::Object(obj) if obj.len() == 1 => {
                let (k, v) = obj.into_iter().next().unwrap();
                fn tag_mismatch() -> JsError {
                    JsError::from_str("key does not match type")
                }
                match k.as_str() {
                    "int" => match v {
                        json_serialize::Value::Number(x) => Ok(TransactionMetadatum::new_int(&Int::from_str(x.to_str().as_str())?)),
                        _ => Err(tag_mismatch()),
                    },
                    "string" => match v {
                        json_serialize::Value::String(string) => {
                            encode_string(string, schema)
                        },
                        _ => Err(tag_mismatch()),
                    }
                    "bytes" => match v {
                        json_serialize::Value::String(string) => {
                            match hex::decode(string) {
                                Ok(bytes) => TransactionMetadatum::new_bytes(bytes),
                                Err(_) => Err(JsError::from_str("invalid hex string in tagged byte-object")),
                            }
                        },
                        _ => Err(tag_mismatch()),
                    },
                    "list" => match v {
                        json_serialize::Value::Array(array) => {
                            encode_array(array, schema)
                        },
                        _ => Err(tag_mismatch()),
                    }
                    "map" => {
                        let mut map = MetadataMap::new();
                        fn map_entry_err() -> JsError {
                            JsError::from_str("entry format in detailed schema map object not correct. Needs to be of form {\"k\": \"key\", \"v\": value}")
                        }
                        let array = match v {
                            json_serialize::Value::Array(array) => {
                                Ok(array)
                            },
                            _ => Err(tag_mismatch()),
                        }?;
                        for entry in array {
                            let entry_obj = match entry {
                                json_serialize::Value::Object(obj) => {
                                    Ok(obj)
                                },
                                _ => Err(map_entry_err()),
                            }?;
                            let raw_key = entry_obj
                                .get("k")
                                .ok_or_else(map_entry_err)?;
                            let value = entry_obj.get("v").ok_or_else(map_entry_err)?;
                            let key = encode_json_value_to_metadatum(raw_key.clone(), schema)?;
                            map.insert(&key, &encode_json_value_to_metadatum(value.clone(), schema)?);
                        }
                        Ok(TransactionMetadatum::new_map(&map))
                    },
                    invalid_key => Err(JsError::from_str(&format!("key '{}' in tagged object not valid", invalid_key))),
                }
            },
            _ => Err(JsError::from_str("DetailedSchema requires types to be tagged objects")),
        },
    }
}

// Converts Metadata to JSON according to MetadataJsonSchema
#[wasm_bindgen]
pub fn decode_metadatum_to_json_str(metadatum: &TransactionMetadatum, schema: MetadataJsonSchema) -> Result<String, JsError> {
    let value = decode_metadatum_to_json_value(metadatum, schema)?;
    value.to_string()
}

pub fn decode_metadatum_to_json_value(metadatum: &TransactionMetadatum, schema: MetadataJsonSchema) -> Result<json_serialize::Value, JsError> {
    use serde_json::Value;
    use std::convert::TryFrom;
    fn decode_key(key: &TransactionMetadatum, schema: MetadataJsonSchema) -> Result<String, JsError> {
        match &key.0 {
            TransactionMetadatumEnum::Text(s) => Ok(s.clone()),
            TransactionMetadatumEnum::Bytes(b) if schema != MetadataJsonSchema::NoConversions => Ok(bytes_to_hex_string(b.as_ref())),
            TransactionMetadatumEnum::Int(i) if schema != MetadataJsonSchema::NoConversions => {
                Ok(i.to_str())
            },
            TransactionMetadatumEnum::MetadataList(list) if schema == MetadataJsonSchema::DetailedSchema => decode_metadatum_to_json_str(&TransactionMetadatum::new_list(&list), schema),
            TransactionMetadatumEnum::MetadataMap(map) if schema == MetadataJsonSchema::DetailedSchema => decode_metadatum_to_json_str(&TransactionMetadatum::new_map(&map), schema),
            _ => Err(JsError::from_str(&format!("key type {:?} not allowed in JSON under specified schema", key.0))),
        }
    }
    let (type_key, value) = match &metadatum.0 {
        TransactionMetadatumEnum::MetadataMap(map) => match schema {
            MetadataJsonSchema::NoConversions |
            MetadataJsonSchema::BasicConversions => {
                // treats maps directly as JSON maps
                let mut json_map = BTreeMap::new();
                for (key, value) in map.0.iter() {
                    json_map.insert(
                        decode_key(key, schema)?,
                        decode_metadatum_to_json_value(value, schema)?
                    );
                }
                ("map", json_serialize::Value::from(json_map))
            },
            
            MetadataJsonSchema::DetailedSchema => ("map", json_serialize::Value::from(map.0.iter().map(|(key, value)| {
                // must encode maps as JSON lists of objects with k/v keys
                // also in these schemas we support more key types than strings
                let k = decode_metadatum_to_json_value(key, schema)?;
                let v = decode_metadatum_to_json_value(value, schema)?;
                let mut kv_obj = BTreeMap::new();
                kv_obj.insert(String::from("k"), json_serialize::Value::from(k));
                kv_obj.insert(String::from("v"), v);
                Ok(json_serialize::Value::from(kv_obj))
            }).collect::<Result<Vec<_>, JsError>>()?))
        },
        TransactionMetadatumEnum::MetadataList(arr) => {
            ("list", json_serialize::Value::from(arr.0.iter().map(|e| {
                decode_metadatum_to_json_value(e, schema)
            }).collect::<Result<Vec<_>, JsError>>()?))
        },
        TransactionMetadatumEnum::Int(x) => ("int", json_serialize::Value::Number(BigInt::from_str(&x.to_str())?)),
        TransactionMetadatumEnum::Bytes(bytes) => ("bytes", match schema {
            MetadataJsonSchema::NoConversions => Err(JsError::from_str("bytes not allowed in JSON in specified schema")),
            // 0x prefix
            MetadataJsonSchema::BasicConversions => Ok(json_serialize::Value::from(bytes_to_hex_string(bytes.as_ref()))),
            // no prefix
            MetadataJsonSchema::DetailedSchema => Ok(json_serialize::Value::from(hex::encode(bytes))),
        }?),
        TransactionMetadatumEnum::Text(s) => ("string", json_serialize::Value::from(s.clone())),
    };
    // potentially wrap value in a keyed map to represent more types
    if supports_tagged_values(schema) {
        let mut wrapper = BTreeMap::new();
        wrapper.insert(String::from(type_key), value);
        Ok(json_serialize::Value::from(wrapper))
    } else {
        Ok(value)
    }
}
