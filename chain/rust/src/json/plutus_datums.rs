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
pub enum PlutusDatumSchema {
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
    ///    Integer's value is a JSON number e.g. {"int": 100}
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

#[wasm_bindgen]
pub fn encode_json_str_to_plutus_datum(json: &str, schema: PlutusDatumSchema) -> Result<PlutusData, JsError> {
    let value = json_serialize::Value::from_string(json.to_string())?;
    encode_json_value_to_plutus_datum(value, schema)
}

pub fn encode_json_value_to_plutus_datum(value: json_serialize::Value, schema: PlutusDatumSchema) -> Result<PlutusData, JsError> {
    fn encode_string(s: &str, schema: PlutusDatumSchema, is_key: bool) -> Result<PlutusData, JsError> {
        if schema == PlutusDatumSchema::BasicConversions {
            if s.starts_with("0x") {
                // this must be a valid hex bytestring after
                hex::decode(&s[2..])
                    .map(|bytes| PlutusData::new_bytes(bytes))
                    .map_err(|err| JsError::from_str(&format!("Error decoding {}: {}", s, err)))
            } else if is_key {
                // try as an integer
                BigInt::from_str(s)
                    .map(|x| PlutusData::new_integer(&x))
                // if not, we use the utf8 bytes of the string instead directly
                    .or_else(|_err| Ok(PlutusData::new_bytes(s.as_bytes().to_vec())))
            } else {
                // can only be UTF bytes if not in a key and not prefixed by 0x
                Ok(PlutusData::new_bytes(s.as_bytes().to_vec()))
            }
        } else {
            if s.starts_with("0x") {
                Err(JsError::from_str("Hex byte strings in detailed schema should NOT start with 0x and should just contain the hex characters"))
            } else {
                hex::decode(s)
                    .map(|bytes| PlutusData::new_bytes(bytes))
                    .map_err(|e| JsError::from_str(&e.to_string()))
            }
        }
    }
    fn encode_array(json_arr: Vec<json_serialize::Value>, schema: PlutusDatumSchema) -> Result<PlutusData, JsError> {
        let mut arr = PlutusList::new();
        for value in json_arr {
            arr.add(&encode_json_value_to_plutus_datum(value, schema)?);
        }
        Ok(PlutusData::new_list(&arr))
    }
    match schema {
        PlutusDatumSchema::BasicConversions => match value {
            json_serialize::Value::Null => Err(JsError::from_str("null not allowed in plutus datums")),
            json_serialize::Value::Bool(_) => Err(JsError::from_str("bools not allowed in plutus datums")),
            json_serialize::Value::Number(x) => Ok(PlutusData::new_integer(&x)),
            // no strings in plutus so it's all bytes (as hex or utf8 printable)
            json_serialize::Value::String(s) => encode_string(&s, schema, false),
            json_serialize::Value::Array(json_arr) => encode_array(json_arr, schema),
            json_serialize::Value::Object(json_obj) => {
                let mut map = PlutusMap::new();
                for (raw_key, raw_value) in json_obj {
                    let key = encode_string(&raw_key, schema, true)?;
                    let value = encode_json_value_to_plutus_datum(raw_value, schema)?;
                    map.insert(&key, &value);
                }
                Ok(PlutusData::new_map(&map))
            },
        },
        PlutusDatumSchema::DetailedSchema => match value {
            json_serialize::Value::Object(obj) => {
                if obj.len() == 1 {
                    // all variants except tagged constructors
                    let (k, v) = obj.into_iter().next().unwrap();
                    fn tag_mismatch() -> JsError {
                        JsError::from_str("key does not match type")
                    }
                    match k.as_str() {
                        "int" => match v {
                            json_serialize::Value::Number(x) => Ok(PlutusData::new_integer(&x)),
                            _ => Err(tag_mismatch()),
                        },
                        "bytes" => match v {
                            json_serialize::Value::String(s) => encode_string(&s, schema, false),
                            _ => Err(tag_mismatch()),
                        },
                        "list" => match v {
                            json_serialize::Value::Array(arr) => encode_array(arr, schema),
                            _ => Err(tag_mismatch()),
                        },
                        "map" => {
                            let mut map = PlutusMap::new();
                            fn map_entry_err() -> JsError {
                                JsError::from_str("entry format in detailed schema map object not correct. Needs to be of form {\"k\": {\"key_type\": key}, \"v\": {\"value_type\", value}}")
                            }
                            let array = match v {
                                json_serialize::Value::Array(array) => Ok(array),
                               _ => Err(tag_mismatch()),
                            }?;

                            for entry in array {

                                let entry_obj = match entry {
                                    json_serialize::Value::Object(obj) => Ok(obj),
                                    _ => Err(map_entry_err()),
                                }?;
                                let raw_key = entry_obj
                                    .get("k")
                                    .ok_or_else(map_entry_err)?;
                                let value = entry_obj
                                    .get("v")
                                    .ok_or_else(map_entry_err)?;
                                let key = encode_json_value_to_plutus_datum(raw_key.clone(), schema)?;
                                map.insert(&key, &encode_json_value_to_plutus_datum(value.clone(), schema)?);
                            }
                            Ok(PlutusData::new_map(&map))
                        },
                        invalid_key => Err(JsError::from_str(&format!("key '{}' in tagged object not valid", invalid_key))),
                    }
                } else {
                    // constructor with tagged variant
                    if obj.len() != 2 {
                        return Err(JsError::from_str("detailed schemas must either have only one of the following keys: \"int\", \"bytes\", \"list\" or \"map\", or both of these 2 keys: \"constructor\" + \"fields\""));
                    }
                    let variant: BigNum = obj
                        .get("constructor")
                        .and_then(|v|
                            match v {
                                json_serialize::Value::Number(number) => {
                                    number.as_u64()
                                }
                                _ => None
                            }
                        )
                        .ok_or_else(|| JsError::from_str("tagged constructors must contain an unsigned integer called \"constructor\""))?;
                    let fields_json = obj
                        .get("fields")
                        .and_then(|f| match f {
                            json_serialize::Value::Array(arr) => {
                                Some(arr)
                            }
                            _ => None
                        })
                        .ok_or_else(|| JsError::from_str("tagged constructors must contian a list called \"fields\""))?;
                    let mut fields = PlutusList::new();
                    for field_json in fields_json {
                        let field = encode_json_value_to_plutus_datum(field_json.clone(), schema)?;
                        fields.add(&field);
                    }
                    Ok(PlutusData::new_constr_plutus_data(&ConstrPlutusData::new(&variant, &fields)))
                }
            },
            _ => Err(JsError::from_str(&format!("DetailedSchema requires ALL JSON to be tagged objects, found: {:?}", value))),
        },
    }
}

#[wasm_bindgen]
pub fn decode_plutus_datum_to_json_str(datum: &PlutusData, schema: PlutusDatumSchema) -> Result<String, JsError> {
    let value = decode_plutus_datum_to_json_value(datum, schema)?;
    value.to_string()
}

pub fn decode_plutus_datum_to_json_value(datum: &PlutusData, schema: PlutusDatumSchema) -> Result<json_serialize::Value, JsError> {
    use serde_json::Value;
    use std::convert::TryFrom;
    let (type_tag, json_value) = match &datum.datum {
        PlutusDataEnum::ConstrPlutusData(constr) => {
            let mut obj = BTreeMap::new();
            obj.insert(
                String::from("constructor"),
                json_serialize::Value::from(from_bignum(&constr.alternative))
            );
            let mut fields = Vec::new();
            for field in constr.data.elems.iter() {
                fields.push(decode_plutus_datum_to_json_value(field, schema)?);
            }
            obj.insert(
                String::from("fields"),
                json_serialize::Value::from(fields)
            );
            (None, json_serialize::Value::from(obj))
        },
        PlutusDataEnum::Map(map) => match schema {
            PlutusDatumSchema::BasicConversions => (None, json_serialize::Value::from(map.0.iter().map(|(key, value)| {
                let json_key: String = match &key.datum {
                    PlutusDataEnum::ConstrPlutusData(_) => Err(JsError::from_str("plutus data constructors are not allowed as keys in this schema. Use DetailedSchema.")),
                    PlutusDataEnum::Map(_) => Err(JsError::from_str("plutus maps are not allowed as keys in this schema. Use DetailedSchema.")),
                    PlutusDataEnum::List(_) => Err(JsError::from_str("plutus lists are not allowed as keys in this schema. Use DetailedSchema.")),
                    PlutusDataEnum::Integer(x) => Ok(x.to_str()),
                    PlutusDataEnum::Bytes(bytes) => String::from_utf8(bytes.clone()).or_else(|_err| Ok(format!("0x{}", hex::encode(bytes))))
                }?;
                let json_value = decode_plutus_datum_to_json_value(value, schema)?;
                Ok((json_key, json_value))
            }).collect::<Result<BTreeMap<String, json_serialize::Value>, JsError>>()?)),
            PlutusDatumSchema::DetailedSchema => (Some("map"), json_serialize::Value::from(map.0.iter().map(|(key, value)| {
                let k = decode_plutus_datum_to_json_value(key, schema)?;
                let v = decode_plutus_datum_to_json_value(value, schema)?;
                let mut kv_obj = BTreeMap::new();
                kv_obj.insert(String::from("k"), k);
                kv_obj.insert(String::from("v"), v);
                Ok(json_serialize::Value::from(kv_obj))
            }).collect::<Result<Vec<_>, JsError>>()?)),
        },
        PlutusDataEnum::List(list) => {
            let mut elems = Vec::new();
            for elem in list.elems.iter() {
                elems.push(decode_plutus_datum_to_json_value(elem, schema)?);
            }
            (Some("list"), json_serialize::Value::from(elems))
        },
        PlutusDataEnum::Integer(bigint) => (
            Some("int"),
            json_serialize::Value::from(bigint.clone())
        ),
        PlutusDataEnum::Bytes(bytes) => (Some("bytes"), json_serialize::Value::from(match schema {
            PlutusDatumSchema::BasicConversions => {
                // cardano-cli converts to a string only if bytes are utf8 and all characters are printable
                String::from_utf8(bytes.clone())
                    .ok()
                    .filter(|utf8| utf8.chars().all(|c| !c.is_control()))
                // otherwise we hex-encode the bytes with a 0x prefix
                    .unwrap_or_else(|| format!("0x{}", hex::encode(bytes)))
            },
            PlutusDatumSchema::DetailedSchema => hex::encode(bytes),
        })),
    };
    if type_tag.is_none() || schema != PlutusDatumSchema::DetailedSchema {
        Ok(json_value)
    } else {
        let mut wrapper = BTreeMap::new();
        wrapper.insert(String::from(type_tag.unwrap()), json_value);
        Ok(json_serialize::Value::from(wrapper))
    }
}