use std::io::{BufRead, Seek, Write};

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

use schemars::JsonSchema;
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use wasm_bindgen::prelude::*;

use cbor_event::Special as CBORSpecial;
use cbor_event::Type as CBORType;

use crate::{to_from_bytes, error::{JsError, DeserializeError, DeserializeFailure}, to_from_json, MultiAsset, Assets};
use cbor_event::{self, de::Deserializer, se::{Serialize, Serializer}};
use super::binary::*;

// Generic u64 wrapper for platforms that don't support u64 or BigInt/etc
// This is an unsigned type - no negative numbers.
// Can be converted to/from plain rust 
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BigNum(u64);

to_from_bytes!(BigNum);

impl std::str::FromStr for BigNum {
    type Err = JsError;
    fn from_str(string: &str) -> Result<BigNum, JsError> {
        string.parse::<u64>()
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
            .map(BigNum)
    }
}

#[wasm_bindgen]
impl BigNum {
    // Create a BigNum from a standard rust string representation
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(string: &str) -> Result<BigNum, JsError> {
        // have to redefine so it's visible in WASM
        std::str::FromStr::from_str(string)
    }

    // String representation of the BigNum value for use from environments that don't support BigInt
    pub fn to_str(&self) -> String {
        format!("{}", self.0)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn checked_mul(&self, other: &BigNum) -> Result<BigNum, JsError> {
        match self.0.checked_mul(other.0) {
            Some(value) => Ok(BigNum(value)),
            None => Err(JsError::from_str("overflow")),
        }
    }

    pub fn checked_add(&self, other: &BigNum) -> Result<BigNum, JsError> {
        match self.0.checked_add(other.0) {
            Some(value) => Ok(BigNum(value)),
            None => Err(JsError::from_str("overflow")),
        }
    }

    pub fn checked_sub(&self, other: &BigNum) -> Result<BigNum, JsError> {
        match self.0.checked_sub(other.0) {
            Some(value) => Ok(BigNum(value)),
            None => Err(JsError::from_str("underflow")),
        }
    }

    /// returns 0 if it would otherwise underflow
    pub fn clamped_sub(&self, other: &BigNum) -> BigNum {
        match self.0.checked_sub(other.0) {
            Some(value) => BigNum(value),
            None => BigNum(0),
        }
    }

    pub fn checked_div(&self, other: &BigNum) -> Result<BigNum, JsError> {
        match self.0.checked_div(other.0) {
            Some(value) => Ok(BigNum(value)),
            None => Err(JsError::from_str("underflow")),
        }
    }

    pub fn checked_div_ceil(&self, other: &BigNum) -> Result<BigNum, JsError> {
        if other.0 <= 0 {
            return Err(JsError::from_str("underflow"));
        }
        match self
            .0
            .checked_add(other.0.checked_sub(1).unwrap())
            .unwrap()
            .checked_div(other.0)
        {
            Some(value) => Ok(BigNum(value)),
            None => Err(JsError::from_str("underflow")),
        }
    }

    pub fn compare(&self, rhs_value: &BigNum) -> i8 {
        match self.cmp(rhs_value) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

impl From<BigNum> for u64 {
    fn from(big_num: BigNum) -> Self {
        big_num.0
    }
}

impl From<u64> for BigNum {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl cbor_event::se::Serialize for BigNum {
  fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
      serializer.write_unsigned_integer(self.0)
  }
}

impl Deserialize for BigNum {
  fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
      match raw.unsigned_integer() {
          Ok(value) => Ok(Self(value)),
          Err(e) => Err(DeserializeError::new("BigNum", DeserializeFailure::CBOR(e))),
      }
  }
}

impl serde::Serialize for BigNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_str())
    }
}

impl <'de> serde::de::Deserialize<'de> for BigNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
    D: serde::de::Deserializer<'de> {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_e| serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"string rep of a number"))
    }
}

impl JsonSchema for BigNum {
    fn schema_name() -> String { String::from("BigNum") }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { String::json_schema(gen) }
    fn is_referenceable() -> bool { String::is_referenceable() }
}

// This is not idiomatic rust, we should favor the new From
// implementations. So that we can convert between these with .into()
pub fn to_bignum(val: u64) -> BigNum {
    BigNum(val)
}
pub fn from_bignum(val: &BigNum) -> u64 {
    val.0
}

// Specifies an amount of ADA in terms of lovelace
pub type Coin = BigNum;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, /*Hash,*/ Ord, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Value {
    pub (crate) coin: Coin,
    pub (crate) multiasset: Option<MultiAsset>,
}

to_from_bytes!(Value);

to_from_json!(Value);

#[wasm_bindgen]
impl Value {

    pub fn new(coin: &Coin) -> Value {
        Self {
            coin: *coin,
            multiasset: None,
        }
    }

    pub fn new_from_assets(multiasset: &MultiAsset) -> Value {
        match multiasset.0.is_empty() {
            true => Value::zero(),
            false => Self {
                coin: Coin::zero(),
                multiasset: Some(multiasset.clone()),
            }
        }
    }

    pub fn zero() -> Value {
        Value::new(&Coin::zero())
    }

    pub fn is_zero(&self) -> bool {
        self.coin.is_zero() && self.multiasset.as_ref().map(|m| m.len() == 0).unwrap_or(true)
    }

    pub fn coin(&self) -> Coin {
        self.coin
    }

    pub fn set_coin(&mut self, coin: &Coin) {
        self.coin = *coin;
    }

    pub fn multiasset(&self) -> Option<MultiAsset> {
        self.multiasset.clone()
    }

    pub fn set_multiasset(&mut self, multiasset: &MultiAsset) {
        self.multiasset = Some(multiasset.clone());
    }

    pub fn checked_add(&self, rhs: &Value) -> Result<Value, JsError> {
        use std::collections::btree_map::Entry;
        let coin = self.coin.checked_add(&rhs.coin)?;

        let multiasset = match (&self.multiasset, &rhs.multiasset) {
            (Some(lhs_multiasset), Some(rhs_multiasset)) => {
                let mut multiasset = MultiAsset::new();

                for ma in &[lhs_multiasset, rhs_multiasset] {
                    for (policy, assets) in &ma.0 {
                        for (asset_name, amount) in &assets.0 {
                            match multiasset.0.entry(policy.clone()) {
                                Entry::Occupied(mut assets) => {
                                    match assets.get_mut().0.entry(asset_name.clone()) {
                                        Entry::Occupied(mut assets) => {
                                            let current = assets.get_mut();
                                            *current = current.checked_add(amount)?;
                                        }
                                        Entry::Vacant(vacant_entry) => {
                                            vacant_entry.insert(*amount);
                                        }
                                    }
                                }
                                Entry::Vacant(entry) => {
                                    let mut assets = Assets::new();
                                    assets.0.insert(asset_name.clone(), *amount);
                                    entry.insert(assets);
                                }
                            }
                        }
                    }
                }

                Some(multiasset)
            },
            (None, None) => None, 
            (Some(ma), None) => Some(ma.clone()),
            (None, Some(ma)) => Some(ma.clone()),
        };

        Ok(Value {
            coin, 
            multiasset
        })
    }

    pub fn checked_sub(&self, rhs_value: &Value) -> Result<Value, JsError> {
        let coin = self.coin.checked_sub(&rhs_value.coin)?;
        let multiasset = match(&self.multiasset, &rhs_value.multiasset) {
            (Some(lhs_ma), Some(rhs_ma)) => {
                match lhs_ma.sub(rhs_ma).len() {
                    0 => None,
                    _ => Some(lhs_ma.sub(rhs_ma))
                }
            },
            (Some(lhs_ma), None) => Some(lhs_ma.clone()),
            (None, Some(_rhs_ma)) => None,
            (None, None) => None
        };

        Ok(Value { coin, multiasset })
    }

    pub fn clamped_sub(&self, rhs_value: &Value) -> Value {
        let coin = self.coin.clamped_sub(&rhs_value.coin);
        let multiasset = match(&self.multiasset, &rhs_value.multiasset) {
            (Some(lhs_ma), Some(rhs_ma)) => {
                match lhs_ma.sub(rhs_ma).len() {
                    0 => None,
                    _ => Some(lhs_ma.sub(rhs_ma))
                }
            },
            (Some(lhs_ma), None) => Some(lhs_ma.clone()),
            (None, Some(_rhs_ma)) => None,
            (None, None) => None
        };

        Value { coin, multiasset }
    }

    /// note: values are only partially comparable
    pub fn compare(&self, rhs_value: &Value) -> Option<i8> {
        match self.partial_cmp(rhs_value) {
            None => None,
            Some(std::cmp::Ordering::Equal) => Some(0),
            Some(std::cmp::Ordering::Less) => Some(-1),
            Some(std::cmp::Ordering::Greater) => Some(1),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;

        fn compare_assets(lhs: &Option<MultiAsset>, rhs: &Option<MultiAsset>) -> Option<std::cmp::Ordering> {
            match (lhs, rhs) {
                (None, None) => Some(Equal),
                (None, Some(rhs_assets)) => MultiAsset::new().partial_cmp(rhs_assets),
                (Some(lhs_assets), None) => lhs_assets.partial_cmp(&MultiAsset::new()),
                (Some(lhs_assets), Some(rhs_assets)) => lhs_assets.partial_cmp(rhs_assets),
            }
        }

        compare_assets(&self.multiasset(), &other.multiasset())
            .and_then(|assets_match| {
                let coin_cmp = self.coin.cmp(&other.coin);

                match (coin_cmp, assets_match) {
                    (coin_order, Equal) => Some(coin_order),
                    (Equal, Less) => Some(Less),
                    (Less, Less) => Some(Less),
                    (Equal, Greater) => Some(Greater),
                    (Greater, Greater) => Some(Greater),
                    (_, _) => None
                }
            })
    }
}

impl cbor_event::se::Serialize for Value {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match &self.multiasset {
            Some(multiasset) => {
                serializer.write_array(cbor_event::Len::Len(2))?;
                self.coin.serialize(serializer)?;
                multiasset.serialize(serializer)
            },
            None => self.coin.serialize(serializer)
        }
    }
}

impl Deserialize for Value {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::UnsignedInteger => Ok(Value::new(&Coin::deserialize(raw)?)),
                cbor_event::Type::Array => {
                    let len = raw.array()?;
                    let coin = (|| -> Result<_, DeserializeError> {
                        Ok(Coin::deserialize(raw)?)
                    })().map_err(|e| e.annotate("coin"))?;
                    let multiasset = (|| -> Result<_, DeserializeError> {
                        Ok(MultiAsset::deserialize(raw)?)
                    })().map_err(|e| e.annotate("multiasset"))?;
                    let ret = Ok(Self {
                        coin,
                        multiasset: Some(multiasset),
                    });
                    match len {
                        cbor_event::Len::Len(n) => match n {
                            2 => /* it's ok */(),
                            n => return Err(DeserializeFailure::DefiniteLenMismatch(n, Some(2)).into()),
                        },
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => /* it's ok */(),
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    ret
                },
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })().map_err(|e| e.annotate("Value"))
    }
}

impl std::str::FromStr for Int {
    type Err = JsError;
    fn from_str(string: &str) -> Result<Int, JsError> {
        let x = string.parse::<i128>()
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))?;
        if x.abs() > u64::MAX as i128 {
            return Err(JsError::from_str(&format!("{} out of bounds. Value (without sign) must fit within 4 bytes limit of {}", x, u64::MAX)));
        }
        Ok(Self(x))
    }
}

// CBOR has int = uint / nint
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int(pub (crate) i128);

to_from_bytes!(Int);

#[wasm_bindgen]
impl Int {
    pub fn new(x: &BigNum) -> Self {
        Self(x.0 as i128)
    }

    pub fn new_negative(x: &BigNum) -> Self {
        Self(-(x.0 as i128))
    }

    pub fn new_i32(x: i32) -> Self {
        Self(x as i128)
    }

    pub fn is_positive(&self) -> bool {
        self.0 >= 0
    }

    /// BigNum can only contain unsigned u64 values
    ///
    /// This function will return the BigNum representation
    /// only in case the underlying i128 value is positive.
    ///
    /// Otherwise nothing will be returned (undefined).
    pub fn as_positive(&self) -> Option<BigNum> {
        if self.is_positive() {
            Some(to_bignum(self.0 as u64))
        } else {
            None
        }
    }

    /// BigNum can only contain unsigned u64 values
    ///
    /// This function will return the *absolute* BigNum representation
    /// only in case the underlying i128 value is negative.
    ///
    /// Otherwise nothing will be returned (undefined).
    pub fn as_negative(&self) -> Option<BigNum> {
        if !self.is_positive() {
            Some(to_bignum((-self.0) as u64))
        } else {
            None
        }
    }

    /// !!! DEPRECATED !!!
    /// Returns an i32 value in case the underlying original i128 value is within the limits.
    /// Otherwise will just return an empty value (undefined).
    #[deprecated(
        since = "0.1.0",
        note = "Unsafe ignoring of possible boundary error and it's not clear from the function name. Use `as_i32_or_nothing`, `as_i32_or_fail`, or `to_str`"
    )]
    pub fn as_i32(&self) -> Option<i32> {
        self.as_i32_or_nothing()
    }

    /// Returns the underlying value converted to i32 if possible (within limits)
    /// Otherwise will just return an empty value (undefined).
    pub fn as_i32_or_nothing(&self) -> Option<i32> {
        use std::convert::TryFrom;
        i32::try_from(self.0).ok()
    }

    /// Returns the underlying value converted to i32 if possible (within limits)
    /// JsError in case of out of boundary overflow
    pub fn as_i32_or_fail(&self) -> Result<i32, JsError> {
        use std::convert::TryFrom;
        i32::try_from(self.0)
            .map_err(|e| JsError::from_str(&format!("{}", e)))
    }

    /// Returns string representation of the underlying i128 value directly.
    /// Might contain the minus sign (-) in case of negative value.
    pub fn to_str(&self) -> String {
        format!("{}", self.0)
    }

    // Create an Int from a standard rust string representation
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(string: &str) -> Result<Int, JsError> {
        // have to redefine so it's visible in WASM
        std::str::FromStr::from_str(string)
    }
}

impl cbor_event::se::Serialize for Int {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        if self.0 < 0 {
            serializer.write_negative_integer(self.0 as i64)
        } else {
            serializer.write_unsigned_integer(self.0 as u64)
        }
    }
}

impl Deserialize for Int {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::UnsignedInteger => Ok(Self(raw.unsigned_integer()? as i128)),
                cbor_event::Type::NegativeInteger => Ok(Self(read_nint(raw)?)),
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })().map_err(|e| e.annotate("Int"))
    }
}

/// TODO: this function can be removed in case `cbor_event` library ever gets a fix on their side
/// See https://github.com/Emurgo/cardano-serialization-lib/pull/392
fn read_nint<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result <i128, DeserializeError> {
    let found = raw.cbor_type()?;
    if found != cbor_event::Type::NegativeInteger {
        return Err(cbor_event::Error::Expected(cbor_event::Type::NegativeInteger, found).into());
    }
    let (len, len_sz) = raw.cbor_len()?;
    match len {
        cbor_event::Len::Indefinite => Err(cbor_event::Error::IndefiniteLenNotSupported(cbor_event::Type::NegativeInteger).into()),
        cbor_event::Len::Len(v) => {
            raw.advance(1 + len_sz)?;
            Ok(-(v as i128) - 1)
        }
    }
}

impl serde::Serialize for Int {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_str())
    }
}

impl <'de> serde::de::Deserialize<'de> for Int {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
    D: serde::de::Deserializer<'de> {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_e| serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"string rep of a number"))
    }
}

impl JsonSchema for Int {
    fn schema_name() -> String { String::from("Int") }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { String::json_schema(gen) }
    fn is_referenceable() -> bool { String::is_referenceable() }
}


#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BigInt(num_bigint::BigInt);

to_from_bytes!(BigInt);

impl serde::Serialize for BigInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_str())
    }
}

impl <'de> serde::de::Deserialize<'de> for BigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
    D: serde::de::Deserializer<'de> {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        BigInt::from_str(&s).map_err(|_e| serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"string rep of a big int"))
    }
}

impl JsonSchema for BigInt {
    fn schema_name() -> String { String::from("BigInt") }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { String::json_schema(gen) }
    fn is_referenceable() -> bool { String::is_referenceable() }
}

impl std::str::FromStr for BigInt {
    type Err = JsError;
    fn from_str(string: &str) -> Result<BigInt, JsError> {
        num_bigint::BigInt::from_str(string)
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
            .map(BigInt)
    }
}

#[wasm_bindgen]
impl BigInt {
    pub fn as_u64(&self) -> Option<BigNum> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        if sign == num_bigint::Sign::Minus {
            return None;
        }
        match u64_digits.len() {
            0 => Some(to_bignum(0)),
            1 => Some(to_bignum(*u64_digits.first().unwrap())),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<Int> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        let u64_digit = match u64_digits.len() {
            0 => Some(to_bignum(0)),
            1 => Some(to_bignum(*u64_digits.first().unwrap())),
            _ => None,
        }?;
        match sign {
            num_bigint::Sign::NoSign |
            num_bigint::Sign::Plus => Some(Int::new(&u64_digit)),
            num_bigint::Sign::Minus => Some(Int::new_negative(&u64_digit)),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(string: &str) -> Result<BigInt, JsError> {
        // have to redefine so it's visible in WASM
        std::str::FromStr::from_str(string)
    }

    pub fn to_str(&self) -> String {
        self.0.to_string()
    }
}

impl cbor_event::se::Serialize for BigInt {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        match u64_digits.len() {
            0 => serializer.write_unsigned_integer(0),
            // we use the uint/nint encodings to use a minimum of space
            1 => match sign {
                // uint
                num_bigint::Sign::Plus |
                num_bigint::Sign::NoSign => serializer.write_unsigned_integer(*u64_digits.first().unwrap()),
                // nint
                num_bigint::Sign::Minus => serializer.write_negative_integer(-(*u64_digits.first().unwrap() as i128) as i64),
            },
            _ => {
                // Small edge case: nint's minimum is -18446744073709551616 but in this bigint lib
                // that takes 2 u64 bytes so we put that as a special case here:
                if sign == num_bigint::Sign::Minus && u64_digits == vec![0, 1] {
                    serializer.write_negative_integer(-18446744073709551616i128 as i64)
                } else {
                let (sign, bytes) = self.0.to_bytes_be();
                    match sign {
                        // positive bigint
                        num_bigint::Sign::Plus |
                        num_bigint::Sign::NoSign => {
                            serializer.write_tag(2u64)?;
                            write_bounded_bytes(serializer, &bytes)
                        },
                        // negative bigint
                        num_bigint::Sign::Minus => {
                            serializer.write_tag(3u64)?;
                            use std::ops::Neg;
                            // CBOR RFC defines this as the bytes of -n -1
                            let adjusted = self.0.clone().neg().checked_sub(&num_bigint::BigInt::from(1u32)).unwrap().to_biguint().unwrap();
                            write_bounded_bytes(serializer, &adjusted.to_bytes_be())
                        },
                    }
                }
            },
        }
    }
}

impl Deserialize for BigInt {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                // bigint
                CBORType::Tag => {
                    let tag = raw.tag()?;
                    let bytes = read_bounded_bytes(raw)?;
                    match tag {
                        // positive bigint
                        2 => Ok(Self(num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes))),
                        // negative bigint
                        3 => {
                            // CBOR RFC defines this as the bytes of -n -1
                            let initial = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes);
                            use std::ops::Neg;
                            let adjusted = initial.checked_add(&num_bigint::BigInt::from(1u32)).unwrap().neg();
                            Ok(Self(adjusted))
                        },
                        _ => Err(DeserializeFailure::TagMismatch{ found: tag, expected: 2 }.into()),
                    }
                },
                // uint
                CBORType::UnsignedInteger => Ok(Self(num_bigint::BigInt::from(raw.unsigned_integer()?))),
                // nint
                CBORType::NegativeInteger => Ok(Self(num_bigint::BigInt::from(read_nint(raw)?))),
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })().map_err(|e| e.annotate("BigInt"))
    }
}

impl<T> std::convert::From<T> for BigInt where T: std::convert::Into<num_bigint::BigInt> {
    fn from(x: T) -> Self {
        Self(x.into())
    }
}
