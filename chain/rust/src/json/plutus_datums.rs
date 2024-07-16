use crate::{
    json::json_serialize::{JsonParseError, Value as JSONValue},
    plutus::{ConstrPlutusData, PlutusData, PlutusMap},
    utils::BigInteger,
};
use std::collections::BTreeMap;
use std::str::FromStr;

#[cfg(not(feature = "used_from_wasm"))]
use noop_proc_macro::wasm_bindgen;
#[cfg(feature = "used_from_wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// JSON <-> PlutusData conversion schemas.
/// Follows ScriptDataJsonSchema in cardano-cli defined at:
/// https://github.com/input-output-hk/cardano-node/blob/master/cardano-api/src/Cardano/Api/ScriptData.hs#L254
///
/// All methods here have the following restrictions due to limitations on dependencies:
/// * JSON numbers above u64::MAX (positive) or below i64::MIN (negative) will throw errors
/// * Hex strings for bytes don't accept odd-length (half-byte) strings.
///      cardano-cli seems to support these however but it seems to be different than just 0-padding
///      on either side when tested so proceed with caution
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CardanoNodePlutusDatumSchema {
    /// ScriptDataJsonNoSchema in cardano-node.
    ///
    /// This is the format used by --script-data-value in cardano-cli
    /// This tries to accept most JSON but does not support the full spectrum of Plutus datums.
    /// From JSON:
    /// * null/true/false/floats NOT supported
    /// * strings starting with 0x are treated as hex bytes. All other strings are encoded as their utf8 bytes.
    /// To JSON:
    /// * ConstrPlutusData not supported in ANY FORM (neither keys nor values)
    /// * Lists not supported in keys
    /// * Maps not supported in keys
    ////
    BasicConversions,
    /// ScriptDataJsonDetailedSchema in cardano-node.
    ///
    /// This is the format used by --script-data-file in cardano-cli
    /// This covers almost all (only minor exceptions) Plutus datums, but the JSON must conform to a strict schema.
    /// The schema specifies that ALL keys and ALL values must be contained in a JSON map with 2 cases:
    /// 1. For ConstrPlutusData there must be two fields "constructor" contianing a number and "fields" containing its fields
    ///    e.g. { "constructor": 2, "fields": [{"int": 2}, {"list": [{"bytes": "CAFEF00D"}]}]}
    /// 2. For all other cases there must be only one field named "int", "bytes", "list" or "map"
    ///    BigInteger's value is a JSON number e.g. {"int": 100}
    ///    Bytes' value is a hex string representing the bytes WITHOUT any prefix e.g. {"bytes": "CAFEF00D"}
    ///    Lists' value is a JSON list of its elements encoded via the same schema e.g. {"list": [{"bytes": "CAFEF00D"}]}
    ///    Maps' value is a JSON list of objects, one for each key-value pair in the map, with keys "k" and "v"
    ///          respectively with their values being the plutus datum encoded via this same schema
    ///          e.g. {"map": [
    ///              {"k": {"int": 2}, "v": {"int": 5}},
    ///              {"k": {"map": [{"k": {"list": [{"int": 1}]}, "v": {"bytes": "FF03"}}]}, "v": {"list": []}}
    ///          ]}
    /// From JSON:
    /// * null/true/false/floats NOT supported
    /// * the JSON must conform to a very specific schema
    /// To JSON:
    /// * all Plutus datums should be fully supported outside of the integer range limitations outlined above.
    ////
    DetailedSchema,
}

#[derive(Debug, thiserror::Error)]
pub enum PlutusJsonError {
    #[error("JSON Parsing: {0}")]
    JsonParse(#[from] JsonParseError),
    #[error("JSON printing: {0}")]
    JsonPrinting(#[from] serde_json::Error),
    #[error("null not allowed in plutus datums")]
    NullFound,
    #[error("bools not allowed in plutus datums")]
    BoolFound,
    #[error(
        "DetailedSchema requires ALL JSON to be tagged objects, found: {:?}",
        0
    )]
    DetailedNonObject(JSONValue),
    #[error("Hex byte strings in detailed schema should NOT start with 0x and should just contain the hex characters")]
    DetailedHexWith0x,
    #[error("DetailedSchema key {0} does not match type {1:?}")]
    DetailedKeyMismatch(String, JSONValue),
    #[error("Invalid hex string: {0}")]
    InvalidHex(#[from] hex::FromHexError),
    #[error("entry format in detailed schema map object not correct. Needs to be of form {{\"k\": {{\"key_type\": key}}, \"v\": {{\"value_type\", value}}}}")]
    InvalidMapEntry,
    #[error("key '{0}' in tagged object not valid")]
    InvalidTag(String),
    #[error("Key requires DetailedSchema: {:?}", 0)]
    DetailedKeyInBasicSchema(PlutusData),
    #[error("detailed schemas must either have only one of the following keys: \"int\", \"bytes\", \"list\" or \"map\", or both of these 2 keys: \"constructor\" + \"fields\"")]
    InvalidTaggedConstructor,
}

pub fn encode_json_str_to_plutus_datum(
    json: &str,
    schema: CardanoNodePlutusDatumSchema,
) -> Result<PlutusData, PlutusJsonError> {
    let value = JSONValue::from_string(json)?;
    encode_json_value_to_plutus_datum(value, schema)
}

pub fn encode_json_value_to_plutus_datum(
    value: JSONValue,
    schema: CardanoNodePlutusDatumSchema,
) -> Result<PlutusData, PlutusJsonError> {
    fn encode_string(
        s: &str,
        schema: CardanoNodePlutusDatumSchema,
        is_key: bool,
    ) -> Result<PlutusData, PlutusJsonError> {
        if schema == CardanoNodePlutusDatumSchema::BasicConversions {
            if let Some(stripped) = s.strip_prefix("0x") {
                // this must be a valid hex bytestring after
                hex::decode(stripped)
                    .map(PlutusData::new_bytes)
                    .map_err(Into::into)
            } else if is_key {
                // try as an integer
                match BigInteger::from_str(s) {
                    Ok(x) => Ok(PlutusData::new_integer(x)),
                    // if not, we use the utf8 bytes of the string instead directly
                    Err(_err) => Ok(PlutusData::new_bytes(s.as_bytes().to_vec())),
                }
            } else {
                // can only be UTF bytes if not in a key and not prefixed by 0x
                Ok(PlutusData::new_bytes(s.as_bytes().to_vec()))
            }
        } else if s.starts_with("0x") {
            Err(PlutusJsonError::DetailedHexWith0x)
        } else {
            hex::decode(s)
                .map(PlutusData::new_bytes)
                .map_err(Into::into)
        }
    }
    fn encode_array(
        json_arr: Vec<JSONValue>,
        schema: CardanoNodePlutusDatumSchema,
    ) -> Result<PlutusData, PlutusJsonError> {
        let mut arr = Vec::new();
        for value in json_arr {
            arr.push(encode_json_value_to_plutus_datum(value, schema)?);
        }
        Ok(PlutusData::new_list(arr))
    }
    match schema {
        CardanoNodePlutusDatumSchema::BasicConversions => match value {
            JSONValue::Null => Err(PlutusJsonError::NullFound),
            JSONValue::Bool(_) => Err(PlutusJsonError::BoolFound),
            JSONValue::Number(x) => Ok(PlutusData::new_integer(x)),
            // no strings in plutus so it's all bytes (as hex or utf8 printable)
            JSONValue::String(s) => encode_string(&s, schema, false),
            JSONValue::Array(json_arr) => encode_array(json_arr, schema),
            JSONValue::Object(json_obj) => {
                let mut map = PlutusMap::new();
                for (raw_key, raw_value) in json_obj {
                    let key = encode_string(&raw_key, schema, true)?;
                    let value = encode_json_value_to_plutus_datum(raw_value, schema)?;
                    map.set(key, value);
                }
                Ok(PlutusData::new_map(map))
            }
        },
        CardanoNodePlutusDatumSchema::DetailedSchema => match value {
            JSONValue::Object(obj) => {
                if obj.len() == 1 {
                    // all variants except tagged constructors
                    let (k, v) = obj.into_iter().next().unwrap();
                    match k.as_str() {
                        "int" => match v {
                            JSONValue::Number(x) => Ok(PlutusData::new_integer(x)),
                            _ => Err(PlutusJsonError::DetailedKeyMismatch(k, v)),
                        },
                        "bytes" => match v {
                            JSONValue::String(s) => encode_string(&s, schema, false),
                            _ => Err(PlutusJsonError::DetailedKeyMismatch(k, v)),
                        },
                        "list" => match v {
                            JSONValue::Array(arr) => encode_array(arr, schema),
                            _ => Err(PlutusJsonError::DetailedKeyMismatch(k, v)),
                        },
                        "map" => {
                            let mut map = PlutusMap::new();
                            let array = match v {
                                JSONValue::Array(array) => Ok(array),
                                _ => Err(PlutusJsonError::DetailedKeyMismatch(k, v)),
                            }?;

                            for entry in array {
                                let entry_obj = match entry {
                                    JSONValue::Object(obj) => Ok(obj),
                                    _ => Err(PlutusJsonError::InvalidMapEntry),
                                }?;
                                let raw_key =
                                    entry_obj.get("k").ok_or(PlutusJsonError::InvalidMapEntry)?;
                                let value =
                                    entry_obj.get("v").ok_or(PlutusJsonError::InvalidMapEntry)?;
                                let key =
                                    encode_json_value_to_plutus_datum(raw_key.clone(), schema)?;
                                map.set(
                                    key,
                                    encode_json_value_to_plutus_datum(value.clone(), schema)?,
                                );
                            }
                            Ok(PlutusData::new_map(map))
                        }
                        _invalid_key => Err(PlutusJsonError::InvalidTag(k)),
                    }
                } else {
                    // constructor with tagged variant
                    let variant = obj.get("constructor").and_then(|v| match v {
                        JSONValue::Number(number) => number.as_u64(),
                        _ => None,
                    });
                    let fields_json = obj.get("fields").and_then(|f| match f {
                        JSONValue::Array(arr) => Some(arr),
                        _ => None,
                    });
                    match (obj.len(), variant, fields_json) {
                        (2, Some(variant), Some(fields_json)) => {
                            let mut fields = Vec::new();
                            for field_json in fields_json {
                                let field =
                                    encode_json_value_to_plutus_datum(field_json.clone(), schema)?;
                                fields.push(field);
                            }
                            Ok(PlutusData::new_constr_plutus_data(ConstrPlutusData::new(
                                variant, fields,
                            )))
                        }
                        _ => Err(PlutusJsonError::InvalidTaggedConstructor),
                    }
                }
            }
            _ => Err(PlutusJsonError::DetailedNonObject(value)),
        },
    }
}

pub fn decode_plutus_datum_to_json_str(
    datum: &PlutusData,
    schema: CardanoNodePlutusDatumSchema,
) -> Result<String, PlutusJsonError> {
    decode_plutus_datum_to_json_value(datum, schema).and_then(|v| v.to_string().map_err(Into::into))
}

pub fn decode_plutus_datum_to_json_value(
    datum: &PlutusData,
    schema: CardanoNodePlutusDatumSchema,
) -> Result<JSONValue, PlutusJsonError> {
    let (type_tag, json_value) = match datum {
        PlutusData::ConstrPlutusData(constr) => {
            let mut obj = BTreeMap::new();
            obj.insert(
                String::from("constructor"),
                JSONValue::from(constr.alternative),
            );
            let mut fields = Vec::new();
            for field in constr.fields.iter() {
                fields.push(decode_plutus_datum_to_json_value(field, schema)?);
            }
            obj.insert(String::from("fields"), JSONValue::from(fields));
            (None, JSONValue::from(obj))
        }
        PlutusData::Map(map) => match schema {
            CardanoNodePlutusDatumSchema::BasicConversions => (
                None,
                JSONValue::from(
                    map.entries
                        .iter()
                        .map(|(key, value)| {
                            let json_key: String = match key {
                                PlutusData::ConstrPlutusData(_)
                                | PlutusData::Map(_)
                                | PlutusData::List { .. } => {
                                    Err(PlutusJsonError::DetailedKeyInBasicSchema(key.clone()))
                                }
                                PlutusData::Integer(x) => Ok(x.to_string()),
                                PlutusData::Bytes { bytes, .. } => String::from_utf8(bytes.clone())
                                    .or_else(|_err| Ok(format!("0x{}", hex::encode(bytes)))),
                            }?;
                            let json_value = decode_plutus_datum_to_json_value(value, schema)?;
                            Ok((json_key, json_value))
                        })
                        .collect::<Result<BTreeMap<String, JSONValue>, PlutusJsonError>>()?,
                ),
            ),
            CardanoNodePlutusDatumSchema::DetailedSchema => (
                Some("map"),
                JSONValue::from(
                    map.entries
                        .iter()
                        .map(|(key, value)| {
                            let k = decode_plutus_datum_to_json_value(key, schema)?;
                            let v = decode_plutus_datum_to_json_value(value, schema)?;
                            let mut kv_obj = BTreeMap::new();
                            kv_obj.insert(String::from("k"), k);
                            kv_obj.insert(String::from("v"), v);
                            Ok(JSONValue::from(kv_obj))
                        })
                        .collect::<Result<Vec<_>, PlutusJsonError>>()?,
                ),
            ),
        },
        PlutusData::List { list, .. } => {
            let mut elems = Vec::new();
            for elem in list.iter() {
                elems.push(decode_plutus_datum_to_json_value(elem, schema)?);
            }
            (Some("list"), JSONValue::from(elems))
        }
        PlutusData::Integer(bigint) => (Some("int"), JSONValue::from(bigint.clone())),
        PlutusData::Bytes { bytes, .. } => (
            Some("bytes"),
            JSONValue::from(match schema {
                CardanoNodePlutusDatumSchema::BasicConversions => {
                    // cardano-cli converts to a string only if bytes are utf8 and all characters are printable
                    String::from_utf8(bytes.clone())
                        .ok()
                        .filter(|utf8| utf8.chars().all(|c| !c.is_control()))
                        // otherwise we hex-encode the bytes with a 0x prefix
                        .unwrap_or_else(|| format!("0x{}", hex::encode(bytes)))
                }
                CardanoNodePlutusDatumSchema::DetailedSchema => hex::encode(bytes),
            }),
        ),
    };
    match (type_tag, schema) {
        (Some(type_tag), CardanoNodePlutusDatumSchema::DetailedSchema) => {
            let mut wrapper = BTreeMap::new();
            wrapper.insert(String::from(type_tag), json_value);
            Ok(JSONValue::from(wrapper))
        }
        _ => Ok(json_value),
    }
}

#[cfg(test)]
mod tests {
    use crate::plutus::PlutusData;

    #[test]
    fn plutus_datum_json() {
        let json = "{\"map\":[{\"k\":{\"int\":100},\"v\":{\"list\":[{\"map\":[{\"k\":{\"bytes\":\"78\"},\"v\":{\"bytes\":\"30\"}},{\"k\":{\"bytes\":\"79\"},\"v\":{\"int\":1}}]}]}},{\"k\":{\"bytes\":\"666f6f\"},\"v\":{\"bytes\":\"0000baadf00d0000cafed00d0000deadbeef0000\"}}]}";
        // let datum = encode_json_str_to_plutus_datum(json, crate::json::plutus_datums::CardanoNodePlutusDatumSchema::DetailedSchema).unwrap();
        let datum: PlutusData = serde_json::from_str(json).unwrap();
        assert_eq!(json, serde_json::to_string(&datum).unwrap());
    }
}
