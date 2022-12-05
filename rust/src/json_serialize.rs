use std::collections::BTreeMap;
use itertools::Itertools;
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Number};
use crate::{DeserializeError, JsError};
use crate::ledger::common::value::BigInt;

pub enum Value {
    Null,
    Bool(bool),
    Number(BigInt),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl Value {
    pub fn to_string(self) -> Result<String, JsError> {
        let result = match self {
            Value::Null => {
                serde_json::to_string(&serde_json::Value::Null).map_err(|err| JsError::from_str(format!("Can't convert null to string: {:?}", err).as_str()))?
            }
            Value::Bool(b) => {
                serde_json::to_string(&serde_json::Value::Bool(b)).map_err(|err| JsError::from_str(format!("Can't convert bool to string: {:?}", err).as_str()))?
            }
            Value::Number(bigint) => {
                bigint.to_str()
            }
            Value::String(str) => {
                serde_json::to_string(&serde_json::Value::String(str)).map_err(|err| JsError::from_str(format!("Can't convert string to string: {:?}", err).as_str()))?
            }
            Value::Array(arr) => {
                let mut arr_serialized = vec![String::new(); arr.len()];
                for (i, item) in arr.into_iter().enumerate() {
                    arr_serialized[i] = item.to_string()?;
                }
                format!("[{}]", arr_serialized.iter().join(","))
            }
            Value::Object(items) => {
                let mut items_serialized = vec![String::new(); items.len()];
                for (i, (key, value)) in items.into_iter().enumerate() {
                    items_serialized[i] = format!("\"{}\":{}", key, value.to_string()?);
                }
                format!("{{{}}}", items_serialized.iter().join(","))
            }
        };
        Ok(result)
    }
}

impl From<Vec<Value>> for Value {
    fn from(vec: Vec<Value>) -> Self {
        Value::Array(vec)
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(string)
    }
}

impl From<u64> for Value {
    fn from(number: u64) -> Self {
        Value::Number(BigInt::from(number))
    }
}

impl From<BigInt> for Value {
    fn from(number: BigInt) -> Self {
        Value::Number(number)
    }
}

impl From<BTreeMap<String, Value>> for Value {
    fn from(from: BTreeMap<String, Value>) -> Self {
        Value::Object(from)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::iter::FromIterator;
    use std::str::FromStr;
    use schemars::Map;
    use crate::json_serialize::Value;
    use crate::ledger::common::value::BigInt;

    #[test]
    fn run_primitives() {
        assert_eq!(Value::Null.to_string().unwrap(), serde_json::Value::Null.to_string());
        for b in vec![true, false].into_iter() {
            assert_eq!(Value::Bool(b).to_string().unwrap(), serde_json::Value::Bool(b).to_string());
        }
        // supported uints
        for integer in vec![0, u64::MAX].into_iter() {
            assert_eq!(Value::Number(BigInt::from(integer)).to_string().unwrap(), serde_json::Value::from(integer).to_string());
        }
        // supported ints
        for integer in vec![0, i64::MAX, i64::MIN].into_iter() {
            assert_eq!(Value::Number(BigInt::from(integer)).to_string().unwrap(), serde_json::Value::from(integer).to_string());
        }
        // unsupported ints
        assert_eq!(Value::Number(BigInt::from_str("980949788381070983313748912887").unwrap()).to_string().unwrap(), "980949788381070983313748912887");
        // string
        assert_eq!(Value::String(String::from("supported string")).to_string().unwrap(), serde_json::Value::from(String::from("supported string")).to_string());
    }

    #[test]
    fn run_array_supported_primitives() {
        let cml_arr = generate_array_of_primitives();
        let serde_arr = generate_array_of_primitives_serde_json();
        assert_eq!(
            Value::Array(cml_arr).to_string().unwrap(),
            serde_json::Value::Array(serde_arr).to_string()
        )
    }

    fn generate_array_of_primitives() -> Vec<Value> {
        let mut cml_arr = vec![Value::Null];
        cml_arr.extend(vec![true, false].into_iter().map(|b| Value::Bool(b)));
        cml_arr.extend(vec![0, u64::MAX].into_iter().map(|integer| Value::Number(BigInt::from(integer))));
        cml_arr.extend(vec![0, i64::MAX, i64::MIN].into_iter().map(|integer| Value::Number(BigInt::from(integer))));
        cml_arr.extend(vec!["supported_string", ""].into_iter().map(|str| Value::String(String::from(str))));
        cml_arr
    }

    fn generate_array_of_primitives_serde_json() -> Vec<serde_json::Value> {
        let mut serde_arr = vec![serde_json::Value::Null];
        serde_arr.extend(vec![true, false].into_iter().map(|b| serde_json::Value::Bool(b)));
        serde_arr.extend(vec![0, u64::MAX].into_iter().map(|integer| serde_json::Value::from(integer)));
        serde_arr.extend(vec![0, i64::MAX, i64::MIN].into_iter().map(|integer| serde_json::Value::from(integer)));
        serde_arr.extend(vec!["supported_string", ""].into_iter().map(|str| serde_json::Value::from(String::from(str))));
        serde_arr
    }

    fn generate_array_of_primitives_with_unsupported() -> Vec<Value> {
        let mut cml_arr = vec![Value::Null];
        cml_arr.extend(vec![true, false].into_iter().map(|b| Value::Bool(b)));
        cml_arr.extend(vec![0, u64::MAX].into_iter().map(|integer| Value::Number(BigInt::from(integer))));
        cml_arr.extend(vec![0, i64::MAX, i64::MIN].into_iter().map(|integer| Value::Number(BigInt::from(integer))));
        cml_arr.extend(vec![BigInt::from_str("980949788381070983313748912887").unwrap()].into_iter().map(|integer| Value::Number(integer)));
        cml_arr.extend(vec!["supported_string", ""].into_iter().map(|str| Value::String(String::from(str))));
        cml_arr
    }

    #[test]
    fn run_array_unsupported_primitives() {
        let cml_arr = generate_array_of_primitives_with_unsupported();
        assert_eq!(
            Value::Array(cml_arr).to_string().unwrap(),
            "[null,true,false,0,18446744073709551615,0,9223372036854775807,-9223372036854775808,980949788381070983313748912887,\"supported_string\",\"\"]"
        );
    }

    fn generate_map() -> BTreeMap<String, Value> {
        let mut index = 0;
        let mut cml_map = vec![(index.to_string(), Value::Null)];
        index += 1;

        cml_map.extend(vec![true, false].into_iter().map(|b| {
            let local = index;
            index += 1;
            (local.to_string(), Value::Bool(b))
        }));
        cml_map.extend(vec![0, u64::MAX].into_iter().map(|integer| {
            let local = index;
            index += 1;
            (local.to_string(), Value::Number(BigInt::from(integer)))
        }));
        cml_map.extend(vec![0, i64::MAX, i64::MIN].into_iter().map(|integer| {
            let local = index;
            index += 1;
            (local.to_string(), Value::Number(BigInt::from(integer)))
        }));
        cml_map.extend(vec!["supported_string", ""].into_iter().map(|str| {
            let local = index;
            index += 1;
            (local.to_string(), Value::String(String::from(str)))
        }
        ));
        cml_map.extend(vec![generate_array_of_primitives()].into_iter().map(|arr| {
            let local = index;
            index += 1;
            (local.to_string(), Value::Array(arr))
        }
        ));

        BTreeMap::from_iter(cml_map.into_iter())
    }

    fn generate_map_unsupported() -> BTreeMap<String, Value> {
        let mut map = generate_map();
        let mut index = map.keys().map(|key| u64::from_str(key).unwrap()).max().unwrap() + 1;

        map.insert(index.to_string(), Value::Number(BigInt::from_str("980949788381070983313748912887").unwrap()));
        index += 1;

        let arr = generate_array_of_primitives_with_unsupported();
        map.insert(index.to_string(), Value::Array(arr));
        index += 1;

        let arr = generate_map();
        map.insert(index.to_string(), Value::Object(arr));
        index += 1;

        map
    }

    fn generate_map_serde_json() -> serde_json::Value {
        let mut index = 0;
        let mut serde_map = vec![(index.to_string(), serde_json::Value::Null)];
        index += 1;

        serde_map.extend(vec![true, false].into_iter().map(|b| {
            let local = index;
            index += 1;
            (local.to_string(), serde_json::Value::Bool(b))
        }));
        serde_map.extend(vec![0, u64::MAX].into_iter().map(|integer| {
            let local = index;
            index += 1;
            (local.to_string(), serde_json::Value::from(integer))
        }));
        serde_map.extend(vec![0, i64::MAX, i64::MIN].into_iter().map(|integer| {
            let local = index;
            index += 1;
            (local.to_string(), serde_json::Value::from(integer))
        }));
        serde_map.extend(vec!["supported_string", ""].into_iter().map(|str| {
            let local = index;
            index += 1;
            (local.to_string(), serde_json::Value::from(str))
        }
        ));
        serde_map.extend(vec![generate_array_of_primitives_serde_json()].into_iter().map(|arr| {
            let local = index;
            index += 1;
            (local.to_string(), serde_json::Value::Array(arr))
        }
        ));

        serde_json::Value::Object(serde_json::Map::from_iter(serde_map.into_iter()))
    }

    #[test]
    fn run_map() {
        let cml_map = generate_map();
        let serde_map = generate_map_serde_json();

        assert_eq!(
            Value::Object(cml_map).to_string().unwrap(),
            serde_map.to_string()
        );
    }

    #[test]
    fn run_map_unsupported() {
        let cml_map = generate_map_unsupported();

        assert_eq!(
            Value::Object(cml_map).to_string().unwrap(),
            "{\"0\":null,\"1\":true,\"10\":[null,true,false,0,18446744073709551615,0,9223372036854775807,-9223372036854775808,\"supported_string\",\"\"],\"11\":980949788381070983313748912887,\"12\":[null,true,false,0,18446744073709551615,0,9223372036854775807,-9223372036854775808,980949788381070983313748912887,\"supported_string\",\"\"],\"13\":{\"0\":null,\"1\":true,\"10\":[null,true,false,0,18446744073709551615,0,9223372036854775807,-9223372036854775808,\"supported_string\",\"\"],\"2\":false,\"3\":0,\"4\":18446744073709551615,\"5\":0,\"6\":9223372036854775807,\"7\":-9223372036854775808,\"8\":\"supported_string\",\"9\":\"\"},\"2\":false,\"3\":0,\"4\":18446744073709551615,\"5\":0,\"6\":9223372036854775807,\"7\":-9223372036854775808,\"8\":\"supported_string\",\"9\":\"\"}"
        );
    }
}