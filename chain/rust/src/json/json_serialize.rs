use std::collections::{BTreeMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::str::FromStr;

use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use unicode_segmentation::UnicodeSegmentation;

use crate::utils::BigInteger;

/**
 * Value replaces traditional serde_json::Value in some places.
 *
 * Main reason for custom type is the fact that serde_json::Value doesn't support big integers,
 * while we need them in metadata and plutus structs.
 *
 * If we move from integers to String we will no longer support the JSON format that cardano-node uses.
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(BigInteger),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum JsonToken {
    ArrayStart,
    ArrayEnd,

    ObjectStart,
    ObjectEnd,

    Colon,
    Comma,
    Quote,

    String { raw: String },

    LeftQuotedString { raw: String },

    ParsedValue { value: Value },
    // This is an object's key when [Quote, String, Quote, Colon] or [Quote, Quote, Colon] are parsed
    ParsedKey { key: String },
}

impl JsonToken {
    fn is_quote(&self) -> bool {
        matches!(self, JsonToken::Quote)
    }

    fn is_string(&self) -> bool {
        matches!(self, JsonToken::String { .. })
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = self
            .clone()
            .to_string()
            .map_err(|err| serde::ser::Error::custom(format!("{:?}", err)))?;

        serializer.serialize_str(string.as_str())
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Value::from_string(&s).map_err(|err| serde::de::Error::custom(format!("{:?}", err)))
    }
}

#[derive(Debug, Clone)]
pub enum JsonParseError {
    // general
    InvalidToken(JsonToken),
    InvalidParseResult(Vec<JsonToken>),

    // array and object
    InvalidTokenBeforeArrayOrObjectStart(JsonToken),

    // array
    NotAllowedInArray(JsonToken),
    NoArrayStartFound,
    ArrayCommaError,

    // object
    NotAllowedInObject(JsonToken),
    NoObjectStartFound,
    ObjectStructureError,
    NoValueForKey(JsonToken),
    NoKeyForValue(JsonToken),

    // quote
    InvalidTokenBeforeQuote(JsonToken),
    // colon
    InvalidTokenBeforeColon(Option<JsonToken>),
    // comma
    InvalidTokenBeforeComma(Option<JsonToken>),
    // string
    InvalidTokenBeforeString(JsonToken),
    InvalidRawString(String),
}

impl Display for JsonParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.serialize_str(format!("{:?}", self).as_str())
    }
}

impl std::error::Error for JsonParseError {}

fn tokenize_string(string: &str) -> Vec<JsonToken> {
    fn are_we_inside_string(tokens: &[JsonToken]) -> bool {
        if tokens.is_empty() {
            return false;
        }
        if tokens.len() == 1 {
            let is_last_token_quote = tokens.last().map(|t| t.is_quote()).unwrap_or(false);
            return is_last_token_quote;
        }
        let last_token = tokens.last();
        let token_before_last_token = tokens.get(tokens.len() - 2);
        let is_token_before_last_string_or_quote = token_before_last_token
            .map(|t| t.is_quote() || t.is_string())
            .unwrap_or(false);
        let is_last_token_quote = last_token.map(|t| t.is_quote()).unwrap_or(false);

        !is_token_before_last_string_or_quote && is_last_token_quote
        /*

        This works because of the following:
        We either have:
          - string without quote on right (which could be invalid as well)
          - string with quote on right

        If we had a string with quote on right before current position, the tokens will be:
        [.., String, Quote, <current pos>]
        or in case of empty string:
        [.., Quote, Quote, <current pos>].

        We never have 2 strings in a row [.., String, String, ..], so we won't face situation
        when we consider we're in / not in the string incorrectly

        If we had a string without quote on right than it's just current string and it's not pushed.
        This way we will have [.., Quote, <current pos>]

        if before quote there was another string - this is invalid json

        */
    }

    let mut tokens = Vec::<JsonToken>::new();
    let mut current_string: String = String::new();
    for char in string.graphemes(true) {
        let (reset_string, token) = match char {
            "\"" => {
                // if we have backslashed quotes in a string they're in the string already
                if !current_string.is_empty()
                    && current_string.graphemes(true).last().unwrap() == "\\"
                {
                    let graphemes_count = current_string.graphemes(true).count();
                    current_string = current_string
                        .graphemes(true)
                        .take(graphemes_count - 1)
                        .collect();
                    current_string += "\"";
                    (false, None)
                } else {
                    (true, Some(JsonToken::Quote))
                }
            }
            "{" | "}" | "[" | "]" | ":" | "," if are_we_inside_string(&tokens) => {
                current_string += char;
                (false, None)
            }
            "{" => (true, Some(JsonToken::ObjectStart)),
            "}" => (true, Some(JsonToken::ObjectEnd)),
            "[" => (true, Some(JsonToken::ArrayStart)),
            "]" => (true, Some(JsonToken::ArrayEnd)),
            ":" => (true, Some(JsonToken::Colon)),
            "," => (true, Some(JsonToken::Comma)),
            _ => {
                let splitted: Vec<&str> = char.split_whitespace().collect();
                let is_whitespace = splitted.is_empty()
                    || (splitted.len() == 1 && splitted.first().cloned().unwrap_or("").is_empty());
                if !is_whitespace || are_we_inside_string(&tokens) {
                    current_string += char;
                }
                (false, None)
            }
        };

        if reset_string && !current_string.is_empty() {
            tokens.push(JsonToken::String {
                raw: current_string.clone(),
            });
            current_string = String::new();
        }

        if let Some(token) = token {
            tokens.push(token);
        }
    }

    if !current_string.is_empty() {
        tokens.push(JsonToken::String {
            raw: current_string.clone(),
        });
    }

    tokens
}

fn parse_json(tokens: Vec<JsonToken>) -> Result<Value, JsonParseError> {
    let mut stack: VecDeque<JsonToken> = VecDeque::new();

    for token in tokens.into_iter() {
        match token {
            JsonToken::ArrayStart | JsonToken::ObjectStart => {
                handle_array_or_object_open(token, &mut stack)?; // done
            }
            JsonToken::ArrayEnd => {
                parse_array(&mut stack)?; // done
            }
            JsonToken::Colon => {
                handle_colon(&mut stack)?; // done
            }
            JsonToken::Comma => {
                handle_comma(&mut stack)?; // done
            }
            JsonToken::Quote => {
                handle_quote(&mut stack)?; // done
            }
            JsonToken::ObjectEnd => {
                parse_object(&mut stack)?;
            }
            JsonToken::String { raw } => {
                handle_string(raw, &mut stack)?;
            }
            JsonToken::ParsedKey { .. }
            | JsonToken::ParsedValue { .. }
            | JsonToken::LeftQuotedString { .. } => {
                return Err(JsonParseError::InvalidToken(token));
            }
        }
    }

    if stack.len() > 1 {
        return Err(JsonParseError::InvalidParseResult(Vec::from_iter(stack)));
    }

    match stack.pop_back() {
        None => Err(JsonParseError::InvalidParseResult(vec![])),
        Some(JsonToken::ParsedValue { value }) => Ok(value),
        Some(other) => Err(JsonParseError::InvalidParseResult(vec![other])),
    }
}

fn handle_array_or_object_open(
    token: JsonToken,
    stack: &mut VecDeque<JsonToken>,
) -> Result<(), JsonParseError> {
    match stack.back() {
        None
        | Some(JsonToken::ArrayStart)
        | Some(JsonToken::ParsedKey { .. })
        | Some(JsonToken::Comma) => {
            stack.push_back(token);
            Ok(())
        }
        back => Err(JsonParseError::InvalidTokenBeforeArrayOrObjectStart(
            back.cloned().unwrap(),
        )),
    }
}

fn handle_colon(stack: &mut VecDeque<JsonToken>) -> Result<(), JsonParseError> {
    let back = stack.pop_back();
    match &back {
        Some(JsonToken::ParsedValue {
            value: Value::String(string),
        }) => {
            stack.push_back(JsonToken::ParsedKey {
                key: string.clone(),
            });
            Ok(())
        }
        _ => Err(JsonParseError::InvalidTokenBeforeColon(back)),
    }
}

fn handle_comma(stack: &mut VecDeque<JsonToken>) -> Result<(), JsonParseError> {
    let back = stack.back();
    match back {
        Some(JsonToken::ParsedValue { .. }) => {
            stack.push_back(JsonToken::Comma);
            Ok(())
        }
        _ => Err(JsonParseError::InvalidTokenBeforeComma(back.cloned())),
    }
}

fn handle_quote(stack: &mut VecDeque<JsonToken>) -> Result<(), JsonParseError> {
    let back = stack.pop_back();
    match back {
        None => {
            stack.push_back(JsonToken::Quote);
            Ok(())
        }
        Some(JsonToken::ArrayStart)
        | Some(JsonToken::ObjectStart)
        | Some(JsonToken::Comma)
        | Some(JsonToken::ParsedKey { .. }) => {
            stack.push_back(back.unwrap());
            stack.push_back(JsonToken::Quote);
            Ok(())
        }
        Some(JsonToken::Quote) => {
            stack.push_back(JsonToken::ParsedValue {
                value: Value::String(String::new()),
            });
            Ok(())
        }
        Some(JsonToken::LeftQuotedString { raw }) => {
            stack.push_back(JsonToken::ParsedValue {
                value: Value::String(raw),
            });
            Ok(())
        }
        _ => Err(JsonParseError::InvalidTokenBeforeQuote(back.unwrap())),
    }
}

fn parse_raw_string(string: String) -> Result<Value, JsonParseError> {
    match string.as_str() {
        "null" => Ok(Value::Null),
        "false" => Ok(Value::Bool(false)),
        "true" => Ok(Value::Bool(true)),
        string => {
            let number = BigInteger::from_str(string);
            match number {
                Ok(number) => Ok(Value::Number(number)),
                Err(_) => Err(JsonParseError::InvalidRawString(String::from(string))),
            }
        }
    }
}

fn handle_string(string: String, stack: &mut VecDeque<JsonToken>) -> Result<(), JsonParseError> {
    let back = stack.pop_back();
    match &back {
        None => {
            let event = parse_raw_string(string)?;
            stack.push_back(JsonToken::ParsedValue { value: event });
            Ok(())
        }
        Some(JsonToken::Quote) => {
            stack.push_back(JsonToken::LeftQuotedString { raw: string });
            Ok(())
        }
        Some(JsonToken::ParsedKey { .. })
        | Some(JsonToken::Comma)
        | Some(JsonToken::ArrayStart) => {
            stack.push_back(back.unwrap());

            let event = parse_raw_string(string)?;
            stack.push_back(JsonToken::ParsedValue { value: event });
            Ok(())
        }
        _ => Err(JsonParseError::InvalidTokenBeforeString(back.unwrap())),
    }
}

fn parse_array(stack: &mut VecDeque<JsonToken>) -> Result<(), JsonParseError> {
    let mut array = Vec::<JsonToken>::new();
    let mut opening_brace_found = false;
    while !stack.is_empty() && !opening_brace_found {
        let current_token = stack.pop_back().unwrap();

        match &current_token {
            JsonToken::ArrayStart => {
                opening_brace_found = true;
                break;
            }
            JsonToken::ParsedValue { .. } | JsonToken::Comma => {
                array.push(current_token);
            }
            _ => {
                return Err(JsonParseError::NotAllowedInArray(current_token));
            }
        }
    }

    if !opening_brace_found {
        return Err(JsonParseError::NoArrayStartFound);
    }

    array.reverse();

    let mut result = Vec::<Value>::new();
    let total_tokens = array.len();
    for (number, token) in array.into_iter().enumerate() {
        match token {
            JsonToken::Comma => {
                if number % 2 != 1 || number + 1 == total_tokens {
                    return Err(JsonParseError::ArrayCommaError);
                }
            }
            JsonToken::ParsedValue { value } => {
                if number % 2 != 0 {
                    return Err(JsonParseError::ArrayCommaError);
                }
                result.push(value);
            }
            _ => return Err(JsonParseError::NotAllowedInArray(token)),
        }
    }

    stack.push_back(JsonToken::ParsedValue {
        value: Value::Array(result),
    });

    Ok(())
}

fn parse_object(stack: &mut VecDeque<JsonToken>) -> Result<(), JsonParseError> {
    let mut array = Vec::<JsonToken>::new();
    let mut opening_brace_found = false;
    while !stack.is_empty() && !opening_brace_found {
        let current_token = stack.pop_back().unwrap();

        match &current_token {
            JsonToken::ObjectStart => {
                opening_brace_found = true;
                break;
            }
            JsonToken::ParsedValue { .. } | JsonToken::Comma | JsonToken::ParsedKey { .. } => {
                array.push(current_token);
            }
            _ => {
                return Err(JsonParseError::NotAllowedInObject(current_token));
            }
        }
    }

    if !opening_brace_found {
        return Err(JsonParseError::NoObjectStartFound);
    }

    array.reverse();

    let mut result = BTreeMap::<String, Value>::new();
    let total_tokens = array.len();

    let mut current_key: Option<String> = None;

    for (number, token) in array.into_iter().enumerate() {
        match &token {
            JsonToken::ParsedKey { key } => {
                if number + 1 == total_tokens {
                    return Err(JsonParseError::NoValueForKey(token.clone()));
                }
                if number % 3 != 0 {
                    return Err(JsonParseError::ObjectStructureError);
                }
                current_key = Some(key.clone());
            }
            JsonToken::ParsedValue { value } => {
                let key = match current_key.clone() {
                    None => {
                        return Err(JsonParseError::NoKeyForValue(token.clone()));
                    }
                    Some(key) => {
                        current_key = None;
                        key
                    }
                };
                if number % 3 != 1 {
                    return Err(JsonParseError::ObjectStructureError);
                }
                result.insert(key, value.clone());
            }
            JsonToken::Comma => {
                if number % 3 != 2 || number + 1 == total_tokens {
                    return Err(JsonParseError::ObjectStructureError);
                }
            }
            _ => return Err(JsonParseError::NotAllowedInObject(token)),
        }
    }

    stack.push_back(JsonToken::ParsedValue {
        value: Value::Object(result),
    });

    Ok(())
}

impl Value {
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        match self {
            Value::Null => serde_json::to_string(&serde_json::Value::Null),
            Value::Bool(b) => serde_json::to_string(&serde_json::Value::Bool(*b)),
            Value::Number(bigint) => Ok(bigint.to_string()),
            Value::String(text) => serde_json::to_string(&serde_json::Value::String(text.clone())),
            Value::Array(arr) => {
                let mut arr_serialized = vec![String::new(); arr.len()];
                for (i, item) in arr.iter().enumerate() {
                    arr_serialized[i] = item.to_string()?;
                }
                Ok(format!("[{}]", arr_serialized.iter().join(",")))
            }
            Value::Object(items) => {
                let mut items_serialized = vec![String::new(); items.len()];
                for (i, (key, value)) in items.iter().enumerate() {
                    items_serialized[i] = format!("\"{}\":{}", key, value.to_string()?);
                }
                Ok(format!("{{{}}}", items_serialized.iter().join(",")))
            }
        }
    }

    pub fn from_string(from: &str) -> Result<Self, JsonParseError> {
        let tokens = tokenize_string(from);
        parse_json(tokens)
    }
}

impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(b) => serde_json::Value::Bool(b),
            Value::Null => serde_json::Value::Null,
            Value::Number(n) => {
                serde_json::Value::Number(serde_json::Number::from_str(&n.to_string()).unwrap())
            }
            Value::String(s) => serde_json::Value::String(s),
            Value::Array(arr) => {
                serde_json::Value::Array(arr.into_iter().map(|e| e.into()).collect())
            }
            Value::Object(obj) => {
                serde_json::Value::Object(obj.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
        }
    }
}

impl From<serde_json::Value> for Value {
    fn from(from: serde_json::Value) -> Self {
        match from {
            serde_json::Value::Bool(b) => Self::Bool(b),
            serde_json::Value::Null => Self::Null,
            serde_json::Value::Number(n) => {
                Self::Number(BigInteger::from_str(&n.to_string()).unwrap())
            }
            serde_json::Value::String(s) => Self::String(s),
            serde_json::Value::Array(arr) => Self::Array(arr.into_iter().map(Self::from).collect()),
            serde_json::Value::Object(obj) => {
                Self::Object(obj.into_iter().map(|(k, v)| (k, Self::from(v))).collect())
            }
        }
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
        Value::Number(BigInteger::from(number))
    }
}

impl From<BigInteger> for Value {
    fn from(number: BigInteger) -> Self {
        Value::Number(number)
    }
}

impl From<BTreeMap<String, Value>> for Value {
    fn from(from: BTreeMap<String, Value>) -> Self {
        Value::Object(from)
    }
}

impl<'a> From<&'a Value> for serde::de::Unexpected<'a> {
    fn from(from: &'a Value) -> Self {
        match from {
            Value::Array(_) => Self::Seq,
            Value::String(s) => Self::Str(s),
            Value::Bool(b) => Self::Bool(*b),
            Value::Null => Self::Unit,
            Value::Number(x) => {
                if let Some(as_u64) = x.as_u64() {
                    Self::Unsigned(as_u64)
                } else if let Some(as_i64) = x.as_int().and_then(|i| {
                    use std::convert::TryFrom;
                    i64::try_from(i128::from(&i)).ok()
                }) {
                    Self::Signed(as_i64)
                } else {
                    Self::Other("Large int")
                }
            }
            Value::Object(_) => Self::Map,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::iter::FromIterator;
    use std::str::FromStr;

    use super::{parse_json, tokenize_string, JsonToken, Value};
    use crate::utils::BigInteger;

    #[test]
    fn run_primitives() {
        assert_eq!(
            Value::Null.to_string().unwrap(),
            serde_json::Value::Null.to_string()
        );
        for b in vec![true, false].into_iter() {
            assert_eq!(
                Value::Bool(b).to_string().unwrap(),
                serde_json::Value::Bool(b).to_string()
            );
        }
        // supported uints
        for integer in vec![0, u64::MAX].into_iter() {
            assert_eq!(
                Value::Number(BigInteger::from(integer))
                    .to_string()
                    .unwrap(),
                serde_json::Value::from(integer).to_string()
            );
        }
        // supported ints
        for integer in vec![0, i64::MAX, i64::MIN].into_iter() {
            assert_eq!(
                Value::Number(BigInteger::from(integer))
                    .to_string()
                    .unwrap(),
                serde_json::Value::from(integer).to_string()
            );
        }
        // unsupported ints
        assert_eq!(
            Value::Number(BigInteger::from_str("980949788381070983313748912887").unwrap())
                .to_string()
                .unwrap(),
            "980949788381070983313748912887"
        );
        // string
        assert_eq!(
            Value::String(String::from("supported string"))
                .to_string()
                .unwrap(),
            serde_json::Value::from(String::from("supported string")).to_string()
        );
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
        cml_arr.extend(vec![true, false].into_iter().map(Value::Bool));
        cml_arr.extend(
            vec![0, u64::MAX]
                .into_iter()
                .map(|integer| Value::Number(BigInteger::from(integer))),
        );
        cml_arr.extend(
            vec![0, i64::MAX, i64::MIN]
                .into_iter()
                .map(|integer| Value::Number(BigInteger::from(integer))),
        );
        cml_arr.extend(
            vec!["supported_string", ""]
                .into_iter()
                .map(|str| Value::String(String::from(str))),
        );
        cml_arr
    }

    fn generate_array_of_primitives_serde_json() -> Vec<serde_json::Value> {
        let mut serde_arr = vec![serde_json::Value::Null];
        serde_arr.extend(vec![true, false].into_iter().map(serde_json::Value::Bool));
        serde_arr.extend(vec![0, u64::MAX].into_iter().map(serde_json::Value::from));
        serde_arr.extend(
            vec![0, i64::MAX, i64::MIN]
                .into_iter()
                .map(serde_json::Value::from),
        );
        serde_arr.extend(
            vec!["supported_string", ""]
                .into_iter()
                .map(|str| serde_json::Value::from(String::from(str))),
        );
        serde_arr
    }

    // serde_json::Value didn't support big integers like that
    fn generate_array_of_primitives_with_unsupported() -> Vec<Value> {
        let mut cml_arr = vec![Value::Null];
        cml_arr.extend(vec![true, false].into_iter().map(Value::Bool));
        cml_arr.extend(
            vec![0, u64::MAX]
                .into_iter()
                .map(|integer| Value::Number(BigInteger::from(integer))),
        );
        cml_arr.extend(
            vec![0, i64::MAX, i64::MIN]
                .into_iter()
                .map(|integer| Value::Number(BigInteger::from(integer))),
        );
        cml_arr.extend(
            vec![
                BigInteger::from_str("980949788381070983313748912887").unwrap(),
                BigInteger::from_str("-980949788381070983313748912887").unwrap(),
            ]
            .into_iter()
            .map(Value::Number),
        );
        cml_arr.extend(
            vec!["supported_string", ""]
                .into_iter()
                .map(|str| Value::String(String::from(str))),
        );
        cml_arr
    }

    #[test]
    fn run_array_unsupported_primitives() {
        let cml_arr = generate_array_of_primitives_with_unsupported();
        assert_eq!(
            Value::Array(cml_arr).to_string().unwrap(),
            "[null,true,false,0,18446744073709551615,0,9223372036854775807,-9223372036854775808,980949788381070983313748912887,-980949788381070983313748912887,\"supported_string\",\"\"]"
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
            (local.to_string(), Value::Number(BigInteger::from(integer)))
        }));
        cml_map.extend(vec![0, i64::MAX, i64::MIN].into_iter().map(|integer| {
            let local = index;
            index += 1;
            (local.to_string(), Value::Number(BigInteger::from(integer)))
        }));
        cml_map.extend(vec!["supported_string", ""].into_iter().map(|str| {
            let local = index;
            index += 1;
            (local.to_string(), Value::String(String::from(str)))
        }));
        cml_map.extend(vec![generate_array_of_primitives()].into_iter().map(|arr| {
            let local = index;
            index += 1;
            (local.to_string(), Value::Array(arr))
        }));

        BTreeMap::from_iter(cml_map)
    }

    fn generate_map_unsupported() -> BTreeMap<String, Value> {
        let mut map = generate_map();
        let mut index = map
            .keys()
            .map(|key| u64::from_str(key).unwrap())
            .max()
            .unwrap()
            + 1;

        map.insert(
            index.to_string(),
            Value::Number(BigInteger::from_str("980949788381070983313748912887").unwrap()),
        );
        index += 1;

        let arr = generate_array_of_primitives_with_unsupported();
        map.insert(index.to_string(), Value::Array(arr));
        index += 1;

        let arr = generate_map();
        map.insert(index.to_string(), Value::Object(arr));

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
        }));
        serde_map.extend(
            vec![generate_array_of_primitives_serde_json()]
                .into_iter()
                .map(|arr| {
                    let local = index;
                    index += 1;
                    (local.to_string(), serde_json::Value::Array(arr))
                }),
        );

        serde_json::Value::Object(serde_json::Map::from_iter(serde_map))
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
            "{\"0\":null,\"1\":true,\"10\":[null,true,false,0,18446744073709551615,0,9223372036854775807,-9223372036854775808,\"supported_string\",\"\"],\"11\":980949788381070983313748912887,\"12\":[null,true,false,0,18446744073709551615,0,9223372036854775807,-9223372036854775808,980949788381070983313748912887,-980949788381070983313748912887,\"supported_string\",\"\"],\"13\":{\"0\":null,\"1\":true,\"10\":[null,true,false,0,18446744073709551615,0,9223372036854775807,-9223372036854775808,\"supported_string\",\"\"],\"2\":false,\"3\":0,\"4\":18446744073709551615,\"5\":0,\"6\":9223372036854775807,\"7\":-9223372036854775808,\"8\":\"supported_string\",\"9\":\"\"},\"2\":false,\"3\":0,\"4\":18446744073709551615,\"5\":0,\"6\":9223372036854775807,\"7\":-9223372036854775808,\"8\":\"supported_string\",\"9\":\"\"}"
        );
    }

    fn easy_cases() -> Vec<(String, Vec<JsonToken>, Value)> {
        vec![
            (
                "false".to_string(),
                vec![JsonToken::String {
                    raw: "false".to_string(),
                }],
                Value::Bool(false),
            ),
            (
                "true".to_string(),
                vec![JsonToken::String {
                    raw: "true".to_string(),
                }],
                Value::Bool(true),
            ),
            (
                "null".to_string(),
                vec![JsonToken::String {
                    raw: "null".to_string(),
                }],
                Value::Null,
            ),
            (
                "\"string\"".to_string(),
                vec![
                    JsonToken::Quote,
                    JsonToken::String {
                        raw: "string".to_string(),
                    },
                    JsonToken::Quote,
                ],
                Value::String("string".to_string()),
            ),
            (
                "\"str\\\"ing\"".to_string(),
                vec![
                    JsonToken::Quote,
                    JsonToken::String {
                        raw: "str\"ing".to_string(),
                    },
                    JsonToken::Quote,
                ],
                Value::String("str\"ing".to_string()),
            ),
            (
                "\"\\\"\\\"\\\"\\\"\"".to_string(),
                vec![
                    JsonToken::Quote,
                    JsonToken::String {
                        raw: "\"\"\"\"".to_string(),
                    },
                    JsonToken::Quote,
                ],
                Value::String("\"\"\"\"".to_string()),
            ),
            (
                "\"\\\"\"".to_string(),
                vec![
                    JsonToken::Quote,
                    JsonToken::String {
                        raw: "\"".to_string(),
                    },
                    JsonToken::Quote,
                ],
                Value::String("\"".to_string()),
            ),
            (
                "\"\\\"\\\"\"".to_string(),
                vec![
                    JsonToken::Quote,
                    JsonToken::String {
                        raw: "\"\"".to_string(),
                    },
                    JsonToken::Quote,
                ],
                Value::String("\"\"".to_string()),
            ),
            (
                "\"y̆\\\"\"".to_string(),
                vec![
                    JsonToken::Quote,
                    JsonToken::String {
                        raw: "y̆\"".to_string(),
                    },
                    JsonToken::Quote,
                ],
                Value::String("y̆\"".to_string()),
            ),
            (
                "\"y̆\\\"y̆\\\"y̆\"".to_string(),
                vec![
                    JsonToken::Quote,
                    JsonToken::String {
                        raw: "y̆\"y̆\"y̆".to_string(),
                    },
                    JsonToken::Quote,
                ],
                Value::String("y̆\"y̆\"y̆".to_string()),
            ),
            (
                "\"y̆\\\"y̆y̆\\\"y̆\"".to_string(),
                vec![
                    JsonToken::Quote,
                    JsonToken::String {
                        raw: "y̆\"y̆y̆\"y̆".to_string(),
                    },
                    JsonToken::Quote,
                ],
                Value::String("y̆\"y̆y̆\"y̆".to_string()),
            ),
            (
                "1234".to_string(),
                vec![JsonToken::String {
                    raw: "1234".to_string(),
                }],
                Value::Number(BigInteger::from_str("1234").unwrap()),
            ),
            (
                "-1234".to_string(),
                vec![JsonToken::String {
                    raw: "-1234".to_string(),
                }],
                Value::Number(BigInteger::from_str("-1234").unwrap()),
            ),
            (
                "123456789876543212345678900000000000000000000".to_string(),
                vec![JsonToken::String {
                    raw: "123456789876543212345678900000000000000000000".to_string(),
                }],
                Value::Number(
                    BigInteger::from_str("123456789876543212345678900000000000000000000").unwrap(),
                ),
            ),
            (
                "-123456789876543212345678900000000000000000000".to_string(),
                vec![JsonToken::String {
                    raw: "-123456789876543212345678900000000000000000000".to_string(),
                }],
                Value::Number(
                    BigInteger::from_str("-123456789876543212345678900000000000000000000").unwrap(),
                ),
            ),
            (
                "0".to_string(),
                vec![JsonToken::String {
                    raw: "0".to_string(),
                }],
                Value::Number(BigInteger::from_str("0").unwrap()),
            ),
            (
                "-0".to_string(),
                vec![JsonToken::String {
                    raw: "-0".to_string(),
                }],
                Value::Number(BigInteger::from_str("-0").unwrap()),
            ),
        ]
    }

    fn run_cases(cases: Vec<(String, Vec<JsonToken>, Value)>) {
        for (case, correct_tokens, correct) in cases {
            let computed_tokens = tokenize_string(&case);
            assert_eq!(
                computed_tokens,
                correct_tokens,
                "Can't tokenize case: {}\n tokens: {}\n correct: {}\n",
                case,
                serde_json::to_string(&computed_tokens).unwrap(),
                serde_json::to_string(&correct_tokens).unwrap()
            );

            let parsed = parse_json(computed_tokens);
            assert!(
                parsed.is_ok(),
                "Can't parse case: {}\n error: {:?}\n correct: {:?}\n",
                case,
                parsed.err(),
                correct
            );
            assert_eq!(
                parsed.clone().unwrap(),
                correct,
                "Mismatch case: {}\n parsed: {:?}\n correct: {:?}\n",
                case,
                parsed,
                correct
            );
        }
    }

    #[test]
    fn deserialize_easy() {
        let cases = easy_cases();
        run_cases(cases);
    }

    fn generate_array(
        cases: Vec<(String, Vec<JsonToken>, Value)>,
    ) -> (String, Vec<JsonToken>, Value) {
        let mut test_string = String::from("[");
        let mut correct_tokens = vec![JsonToken::ArrayStart];
        let mut correct_value = vec![];

        let count = cases.len();
        for (number, (test, tokens, parsed)) in cases.into_iter().enumerate() {
            test_string += test.as_str();
            correct_tokens.extend(tokens);
            correct_value.push(parsed);
            if number + 1 != count {
                test_string += ",";
                correct_tokens.push(JsonToken::Comma);
            }
        }
        test_string += "]";
        correct_tokens.push(JsonToken::ArrayEnd);
        (test_string, correct_tokens, Value::Array(correct_value))
    }

    fn generate_arrays() -> Vec<(String, Vec<JsonToken>, Value)> {
        vec![
            generate_array(easy_cases()),
            generate_array(Vec::from_iter(
                vec![generate_array(easy_cases())]
                    .into_iter()
                    .chain(easy_cases().into_iter()),
            )),
            generate_array(Vec::from_iter(
                vec![generate_array(Vec::from_iter(
                    vec![generate_array(easy_cases())]
                        .into_iter()
                        .chain(easy_cases().into_iter()),
                ))]
                .into_iter()
                .chain(easy_cases().into_iter()),
            )),
        ]
    }

    #[test]
    fn deserialize_array() {
        let cases = generate_arrays();

        run_cases(cases);
    }

    fn generate_object(
        cases: Vec<(String, Vec<JsonToken>, Value)>,
    ) -> (String, Vec<JsonToken>, Value) {
        let mut test_string = String::from("{");
        let mut correct_tokens = vec![JsonToken::ObjectStart];
        let mut correct_value = BTreeMap::new();

        let count = cases.len();
        for (number, (test, tokens, parsed)) in cases.into_iter().enumerate() {
            test_string += &format!("\"{}\":", number);
            test_string += &test;
            correct_tokens.extend(vec![
                JsonToken::Quote,
                JsonToken::String {
                    raw: number.to_string(),
                },
                JsonToken::Quote,
                JsonToken::Colon,
            ]);
            correct_tokens.extend(tokens);
            correct_value.insert(number.to_string(), parsed);
            if number + 1 != count {
                test_string += ",";
                correct_tokens.push(JsonToken::Comma);
            }
        }
        test_string += "}";
        correct_tokens.push(JsonToken::ObjectEnd);
        (test_string, correct_tokens, Value::Object(correct_value))
    }

    fn generate_objects() -> Vec<(String, Vec<JsonToken>, Value)> {
        vec![
            generate_object(Vec::from_iter(
                vec![generate_array(easy_cases())]
                    .into_iter()
                    .chain(easy_cases().into_iter()),
            )),
            generate_object(Vec::from_iter(
                vec![generate_array(easy_cases())]
                    .into_iter()
                    .chain(easy_cases().into_iter())
                    .chain(
                        vec![generate_object(Vec::from_iter(
                            vec![generate_array(easy_cases())]
                                .into_iter()
                                .chain(easy_cases().into_iter()),
                        ))]
                        .into_iter(),
                    ),
            )),
            generate_object(Vec::from_iter(
                vec![generate_array(easy_cases())]
                    .into_iter()
                    .chain(easy_cases().into_iter())
                    .chain(
                        vec![generate_object(Vec::from_iter(
                            vec![generate_array(easy_cases())]
                                .into_iter()
                                .chain(easy_cases().into_iter())
                                .chain(generate_arrays().into_iter()),
                        ))]
                        .into_iter()
                        .chain(generate_arrays().into_iter()),
                    ),
            )),
        ]
    }

    #[test]
    fn deserialize_object() {
        let cases = generate_objects();

        run_cases(cases);
    }

    #[test]
    fn mix() {
        let cases = vec![
            generate_array(generate_objects()),
            generate_array(generate_arrays()),
            generate_object(generate_arrays()),
            generate_object(generate_objects()),
        ];

        run_cases(cases);
    }

    #[test]
    fn deserialize_errors() {
        let cases = vec![
            ",",
            "[],",
            "{},",
            "{,}",
            // commas
            "{\"1\":\"kek\",}",
            "{,\"1\":\"kek\"}",
            "{\"1\":\"kek\",\"1\":\"kek\",}",
            "{\"1\":\"kek\",,\"1\":\"kek\"}",
            "{\"1\",:\"kek\",\"1\":\"kek\"}",
            "{\"1\":,\"kek\",\"1\":\"kek\"}",
            "{\"1\",\"kek\",\"1\":\"kek\"}",
            "{\"1\":\"kek\",\"1\":\"kek\"},",
            ",{\"1\":\"kek\",\"1\":\"kek\"}",
            "{\"1\"\"kek\",\"1\":\"kek\"}",
            "{:\"kek\",\"1\":\"kek\"}",
            "{\"1:\"kek\",\"1\":\"kek\"}",
            "{1\":\"kek\",\"1\":\"kek\"}",
            "{1:\"kek\",\"1\":\"kek\"}",
            "{\"1\"kek\",\"1\":\"kek\"}",
            "{\"1kek\",\"1\":\"kek\"}",
            "[1,2,3,]",
            "[1,2,,3]",
            "[,1,2,3]",
            // array
            "[\"lel\":,2,3]",
            "[\"lel\":1,2,3]",
            "[1,2,3",
            "1,2,3]",
            "[",
            "]",
            "{",
            "}",
            "[[1,2,3]",
            "[1,2,3]]",
            "{\"\":1}}",
            "{{\"\":1}",
            "{\"\":[1,]}",
            "{\"\":[1,2}",
            "{\"\":[1,2,[1,]]}",
            "{\"\":[1,2,[1,[]]}",
            "{\"\":[1,2,[1,[]]]]}",
            // empty
            "[][]",
            "{}[]",
            "[]{}",
            "{}{}",
            "nul",
            "\"\"\"",
            "\"\\\"",
            "\\\"\\\"",
            "\\\"
            \\\"",
        ];
        for case in cases.into_iter() {
            let computed_tokens = tokenize_string(case);
            let parsed = parse_json(computed_tokens.clone());
            assert!(
                parsed.is_err(),
                "False parse case: {}\n result: {:?}\n",
                case,
                parsed.unwrap()
            );
        }
    }

    #[test]
    fn deserialize_ok() {
        let cases = vec![
            ("[]", Value::Array(vec![])),
            ("{}", Value::Object(BTreeMap::new())),
            ("\"{}[]:,\"", Value::String("{}[]:,".to_string())),
            ("null", Value::Null),
            ("null ", Value::Null),
            (" null ", Value::Null),
            (
                " \
            [\
             \"\
              \
             \"] \
            ",
                Value::Array(vec![Value::String(
                    "\
              \
             "
                    .to_string(),
                )]),
            ),
            (
                "  \
             [\"   \"    \
            ,   \"    \"   ] \
            ",
                Value::Array(vec![
                    Value::String("   ".to_string()),
                    Value::String("    ".to_string()),
                ]),
            ),
            (
                "{\"kek\":1}",
                Value::Object(BTreeMap::from_iter(vec![(
                    "kek".to_string(),
                    Value::Number(BigInteger::from(1)),
                )])),
            ),
            (
                "{\"kek\": 1}",
                Value::Object(BTreeMap::from_iter(vec![(
                    "kek".to_string(),
                    Value::Number(BigInteger::from(1)),
                )])),
            ),
            (
                "{\"kek\":false}",
                Value::Object(BTreeMap::from_iter(vec![(
                    "kek".to_string(),
                    Value::Bool(false),
                )])),
            ),
            (
                "{\"kek\":true}",
                Value::Object(BTreeMap::from_iter(vec![(
                    "kek".to_string(),
                    Value::Bool(true),
                )])),
            ),
            (
                "{\"kek\":null}",
                Value::Object(BTreeMap::from_iter(vec![("kek".to_string(), Value::Null)])),
            ),
            (
                "{\"kek\":{}}",
                Value::Object(BTreeMap::from_iter(vec![(
                    "kek".to_string(),
                    Value::Object(BTreeMap::new()),
                )])),
            ),
            (
                "{\"kek\":[]}",
                Value::Object(BTreeMap::from_iter(vec![(
                    "kek".to_string(),
                    Value::Array(vec![]),
                )])),
            ),
            (
                "{\"kek\":[ ]}",
                Value::Object(BTreeMap::from_iter(vec![(
                    "kek".to_string(),
                    Value::Array(vec![]),
                )])),
            ),
            (
                " {\"kek\": []}",
                Value::Object(BTreeMap::from_iter(vec![(
                    "kek".to_string(),
                    Value::Array(vec![]),
                )])),
            ),
            (
                "{\"kek\":[{\"\":[{}]}]}",
                Value::Object(BTreeMap::from_iter(vec![(
                    "kek".to_string(),
                    Value::Array(vec![Value::Object(BTreeMap::from_iter(vec![(
                        String::new(),
                        Value::Array(vec![Value::Object(BTreeMap::new())]),
                    )]))]),
                )])),
            ),
            (
                "{\"kek\":[{\"\":[1]}]}",
                Value::Object(BTreeMap::from_iter(vec![(
                    "kek".to_string(),
                    Value::Array(vec![Value::Object(BTreeMap::from_iter(vec![(
                        String::new(),
                        Value::Array(vec![Value::Number(BigInteger::from(1))]),
                    )]))]),
                )])),
            ),
            (
                "[{\"kek\":[{\"\":[1, \"{}[]:,\\\"{}[]:,\\\"\"\
            ]}]},{\"kek\":[{\"\":[1]}]}]",
                Value::Array(vec![
                    Value::Object(BTreeMap::from_iter(vec![(
                        "kek".to_string(),
                        Value::Array(vec![Value::Object(BTreeMap::from_iter(vec![(
                            String::new(),
                            Value::Array(vec![
                                Value::Number(BigInteger::from(1)),
                                Value::String("{}[]:,\"{}[]:,\"".to_string()),
                            ]),
                        )]))]),
                    )])),
                    Value::Object(BTreeMap::from_iter(vec![(
                        "kek".to_string(),
                        Value::Array(vec![Value::Object(BTreeMap::from_iter(vec![(
                            String::new(),
                            Value::Array(vec![Value::Number(BigInteger::from(1))]),
                        )]))]),
                    )])),
                ]),
            ),
            (
                "[{\"kek\":[{\"\":[1]}]},{\"kek\":[{\"\":[1]}]}]",
                Value::Array(vec![
                    Value::Object(BTreeMap::from_iter(vec![(
                        "kek".to_string(),
                        Value::Array(vec![Value::Object(BTreeMap::from_iter(vec![(
                            String::new(),
                            Value::Array(vec![Value::Number(BigInteger::from(1))]),
                        )]))]),
                    )])),
                    Value::Object(BTreeMap::from_iter(vec![(
                        "kek".to_string(),
                        Value::Array(vec![Value::Object(BTreeMap::from_iter(vec![(
                            String::new(),
                            Value::Array(vec![Value::Number(BigInteger::from(1))]),
                        )]))]),
                    )])),
                ]),
            ),
            (
                "[\
                {\
                    \"kek\": [\
                        {\"\":[1]}]},{\
            \"kek\":\
            [{\"\":[1]}]\
            }]",
                Value::Array(vec![
                    Value::Object(BTreeMap::from_iter(vec![(
                        "kek".to_string(),
                        Value::Array(vec![Value::Object(BTreeMap::from_iter(vec![(
                            String::new(),
                            Value::Array(vec![Value::Number(BigInteger::from(1))]),
                        )]))]),
                    )])),
                    Value::Object(BTreeMap::from_iter(vec![(
                        "kek".to_string(),
                        Value::Array(vec![Value::Object(BTreeMap::from_iter(vec![(
                            String::new(),
                            Value::Array(vec![Value::Number(BigInteger::from(1))]),
                        )]))]),
                    )])),
                ]),
            ),
        ];

        for (case, correct) in cases {
            let computed_tokens = tokenize_string(case);
            let parsed = parse_json(computed_tokens);
            assert!(
                parsed.is_ok(),
                "Can't parse case: {}\n error: {:?}\n correct: {:?}\n",
                case,
                parsed.err(),
                correct
            );
            assert_eq!(
                parsed.clone().unwrap(),
                correct,
                "Mismatch case: {}\n parsed: {:?}\n correct: {:?}\n",
                case,
                parsed,
                correct
            );
        }
    }
}
