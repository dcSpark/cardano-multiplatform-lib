use cbor_event::{self, de::Deserializer, se::{Serialize, Serializer}};
use hex::FromHex;
use serde_json;
use std::{collections::HashMap, io::{BufRead, Seek, Write}};
use itertools::Itertools;
use std::cmp;
use std::ops::{Rem, Div, Sub};

use super::*;
use crate::error::{DeserializeError, DeserializeFailure};

// JsError can't be used by non-wasm targets so we use this macro to expose
// either a DeserializeError or a JsError error depending on if we're on a
// wasm or a non-wasm target where JsError is not available (it panics!).
// Note: wasm-bindgen doesn't support macros inside impls, so we have to wrap these
//       in their own impl and invoke the invoke the macro from global scope.
// TODO: possibly write s generic version of this for other usages (e.g. PrivateKey, etc)
#[macro_export]
macro_rules! from_bytes {
    // Custom from_bytes() code
    ($name:ident, $data: ident, $body:block) => {
        // wasm-exposed JsError return - JsError panics when used outside wasm
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        #[wasm_bindgen]
        impl $name {
            pub fn from_bytes($data: Vec<u8>) -> Result<$name, JsError> {
                Ok($body?)
            }
        }
        // non-wasm exposed DeserializeError return
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        impl $name {
            pub fn from_bytes($data: Vec<u8>) -> Result<$name, DeserializeError> $body
        }
    };
    // Uses Deserialize trait to auto-generate one
    ($name:ident) => {
        from_bytes!($name, bytes, {
            let mut raw = Deserializer::from(std::io::Cursor::new(bytes));
            Self::deserialize(&mut raw)
        });
    };
}

// There's no need to do wasm vs non-wasm as this call can't fail but
// this is here just to provide a default Serialize-based impl
// Note: Once again you can't use macros in impls with wasm-bindgen
//       so make sure you invoke this outside of one
#[macro_export]
macro_rules! to_bytes {
    ($name:ident) => {
        #[wasm_bindgen]
        impl $name {
            pub fn to_bytes(&self) -> Vec<u8> {
                let mut buf = Serializer::new_vec();
                self.serialize(&mut buf).unwrap();
                buf.finalize()
            }
        }
    }
}

#[macro_export]
macro_rules! to_from_bytes {
    ($name:ident) => {
        to_bytes!($name);
        from_bytes!($name);
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionUnspentOutput {
    input: TransactionInput,
    output: TransactionOutput
}

to_from_bytes!(TransactionUnspentOutput);

#[wasm_bindgen]
impl TransactionUnspentOutput {
    pub fn new(input: &TransactionInput, output: &TransactionOutput) -> TransactionUnspentOutput {
        Self {
            input: input.clone(),
            output: output.clone()
        }
    }

    pub fn input(&self) -> TransactionInput {
        self.input.clone()
    }

    pub fn output(&self) -> TransactionOutput {
        self.output.clone()
    }
}

impl cbor_event::se::Serialize for TransactionUnspentOutput {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        self.input.serialize(serializer)?;
        self.output.serialize(serializer)
    }
}

impl Deserialize for TransactionUnspentOutput {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::Array => {
                    let len = raw.array()?;
                    let input = (|| -> Result<_, DeserializeError> {
                        Ok(TransactionInput::deserialize(raw)?)
                    })().map_err(|e| e.annotate("input"))?;
                    let output = (|| -> Result<_, DeserializeError> {
                        Ok(TransactionOutput::deserialize(raw)?)
                    })().map_err(|e| e.annotate("output"))?;
                    let ret = Ok(Self {
                        input,
                        output
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
        })().map_err(|e| e.annotate("TransactionUnspentOutput"))
    }
}

// Generic u64 wrapper for platforms that don't support u64 or BigInt/etc
// This is an unsigned type - no negative numbers.
// Can be converted to/from plain rust 
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BigNum(u64);

to_from_bytes!(BigNum);

#[wasm_bindgen]
impl BigNum {
    // Create a BigNum from a standard rust string representation
    pub fn from_str(string: &str) -> Result<BigNum, JsError> {
        string.parse::<u64>()
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
            .map(BigNum)
    }

    // String representation of the BigNum value for use from environments that don't support BigInt
    pub fn to_str(&self) -> String {
        format!("{}", self.0)
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

    pub fn compare(&self, rhs_value: &BigNum) -> i8 {
        match self.cmp(&rhs_value) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
        }
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

pub fn to_bignum(val: u64) -> BigNum {
    BigNum(val)
}
pub fn from_bignum(val: &BigNum) -> u64 {
    val.0
}

// Specifies an amount of ADA in terms of lovelace
pub type Coin = BigNum;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, /*Hash,*/ Ord, PartialEq)]
pub struct Value {
    coin: Coin,
    multiasset: Option<MultiAsset>,
}

to_from_bytes!(Value);

#[wasm_bindgen]
impl Value {
    pub fn new(coin: &Coin) -> Value {
        Self {
            coin: coin.clone(),
            multiasset: None,
        }
    }

    pub fn coin(&self) -> Coin {
        self.coin
    }

    pub fn set_coin(&mut self, coin: &Coin) {
        self.coin = coin.clone();
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
                                            *current = current.checked_add(&amount)?;
                                        }
                                        Entry::Vacant(vacant_entry) => {
                                            vacant_entry.insert(amount.clone());
                                        }
                                    }
                                }
                                Entry::Vacant(entry) => {
                                    let mut assets = Assets::new();
                                    assets.0.insert(asset_name.clone(), amount.clone());
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
                Some(lhs_ma.sub(rhs_ma))
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
                Some(lhs_ma.sub(rhs_ma))
            },
            (Some(lhs_ma), None) => Some(lhs_ma.clone()),
            (None, Some(_rhs_ma)) => None,
            (None, None) => None
        };

        Value { coin, multiasset }
    }

    /// note: values are only partially comparable
    pub fn compare(&self, rhs_value: &Value) -> Option<i8> {
        match self.partial_cmp(&rhs_value) {
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
                (None, Some(rhs_assets)) => MultiAsset::new().partial_cmp(&rhs_assets),
                (Some(lhs_assets), None) => lhs_assets.partial_cmp(&MultiAsset::new()),
                (Some(lhs_assets), Some(rhs_assets)) => lhs_assets.partial_cmp(&rhs_assets),
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

// CBOR has int = uint / nint
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int(pub (crate) i128);

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
        return self.0 >= 0
    }

    pub fn as_positive(&self) -> Option<BigNum> {
        if self.is_positive() {
            Some(to_bignum(self.0 as u64))
        } else {
            None
        }
    }

    pub fn as_negative(&self) -> Option<BigNum> {
        if !self.is_positive() {
            Some(to_bignum((-self.0) as u64))
        } else {
            None
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        use std::convert::TryFrom;
        i32::try_from(self.0).ok()
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
                cbor_event::Type::NegativeInteger => Ok(Self(raw.negative_integer()? as i128)),
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })().map_err(|e| e.annotate("Int"))
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BigInt(num_bigint::BigInt);

to_from_bytes!(BigInt);

#[wasm_bindgen]
impl BigInt {
    pub fn as_u64(&self) -> Option<BigNum> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        if sign == num_bigint::Sign::Minus {
            return None;
        }
        match u64_digits.len() {
            1 => Some(to_bignum(*u64_digits.first().unwrap())),
            _ => None,
        }
    }

    pub fn from_str(text: &str) -> Result<BigInt, JsError> {
        use std::str::FromStr;
        num_bigint::BigInt::from_str(text)
            .map_err(|e| JsError::from_str(&format! {"{:?}", e}))
            .map(BigInt)
    }

    pub fn to_str(&self) -> String {
        self.0.to_string()
    }
}

impl cbor_event::se::Serialize for BigInt {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        let (sign, u64_digits) = self.0.to_u64_digits();
        // we use the uint/nint encodings to use a minimum of space
        if u64_digits.len() == 1 {
            match sign {
                // uint
                num_bigint::Sign::Plus |
                num_bigint::Sign::NoSign => serializer.write_unsigned_integer(*u64_digits.first().unwrap())?,
                // nint
                num_bigint::Sign::Minus => serializer.write_negative_integer(-(*u64_digits.first().unwrap() as i128) as i64)?,
            };
        } else {
            let (sign, bytes) = self.0.to_bytes_be();
            match sign {
                // positive bigint
                num_bigint::Sign::Plus |
                num_bigint::Sign::NoSign => {
                    serializer.write_tag(2u64)?;
                    serializer.write_bytes(bytes)?;
                },
                // negative bigint
                num_bigint::Sign::Minus => {
                    serializer.write_tag(3u64)?;
                    use std::ops::Neg;
                    // CBOR RFC defines this as the bytes of -n -1
                    let adjusted = self.0.clone().neg().checked_sub(&num_bigint::BigInt::from(1u32)).unwrap().to_biguint().unwrap();
                    serializer.write_bytes(adjusted.to_bytes_be())?;
                },
            }
        }
        Ok(serializer)
    }
}

impl Deserialize for BigInt {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                // bigint
                CBORType::Tag => {
                    let tag = raw.tag()?;
                    let bytes = raw.bytes()?;
                    if bytes.len() > 64 {
                        return Err(DeserializeFailure::OutOfRange{ found: bytes.len(), min: 0, max: 64}.into())
                    }
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
                        _ => return Err(DeserializeFailure::TagMismatch{ found: tag, expected: 2 }.into()),
                    }
                },
                // uint
                CBORType::UnsignedInteger => Ok(Self(num_bigint::BigInt::from(raw.unsigned_integer()?))),
                // nint
                CBORType::NegativeInteger => Ok(Self(num_bigint::BigInt::from(raw.negative_integer()?))),
                _ => return Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })().map_err(|e| e.annotate("BigInt"))
    }
}

// we use the cbor_event::Serialize trait directly

// This is only for use for plain cddl groups who need to be embedded within outer groups.
pub (crate) trait SerializeEmbeddedGroup {
    fn serialize_as_embedded_group<'a, W: Write + Sized>(
        &self,
        serializer: &'a mut Serializer<W>,
    ) -> cbor_event::Result<&'a mut Serializer<W>>;
}

// same as cbor_event::de::Deserialize but with our DeserializeError
pub trait Deserialize {
    fn deserialize<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
    ) -> Result<Self, DeserializeError> where Self: Sized;
}

// auto-implement for all cbor_event Deserialize implementors
impl<T: cbor_event::de::Deserialize> Deserialize for T {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<T, DeserializeError> {
        T::deserialize(raw).map_err(|e| DeserializeError::from(e))
    }
}

// This is only for use for plain cddl groups who need to be embedded within outer groups.
pub trait DeserializeEmbeddedGroup {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> where Self: Sized;
}

pub struct CBORReadLen {
    deser_len: cbor_event::Len,
    read: u64,
}

impl CBORReadLen {
    pub fn new(len: cbor_event::Len) -> Self {
        Self {
            deser_len: len,
            read: 0,
        }
    }

    // Marks {n} values as being read, and if we go past the available definite length
    // given by the CBOR, we return an error.
    pub fn read_elems(&mut self, count: usize) -> Result<(), DeserializeFailure> {
        match self.deser_len {
            cbor_event::Len::Len(n) => {
                self.read += count as u64;
                if self.read > n {
                    Err(DeserializeFailure::DefiniteLenMismatch(n, None))
                } else {
                    Ok(())
                }
            },
            cbor_event::Len::Indefinite => Ok(()),
        }
    }

    pub fn finish(&self) -> Result<(), DeserializeFailure> {
        match self.deser_len {
            cbor_event::Len::Len(n) => {
                if self.read == n {
                    Ok(())
                } else {
                    Err(DeserializeFailure::DefiniteLenMismatch(n, Some(self.read)))
                }
            },
            cbor_event::Len::Indefinite => Ok(()),
        }
    }
}

#[wasm_bindgen]
pub fn make_daedalus_bootstrap_witness(
    tx_body_hash: &TransactionHash,
    addr: &ByronAddress,
    key: &LegacyDaedalusPrivateKey,
) -> BootstrapWitness {
    let chain_code = key.chaincode();

    let pubkey = Bip32PublicKey::from_bytes(&key.0.to_public().as_ref()).unwrap();
    let vkey = Vkey::new(&pubkey.to_raw_key());
    let signature = Ed25519Signature::from_bytes(key.0.sign(&tx_body_hash.to_bytes()).as_ref().to_vec()).unwrap();

    BootstrapWitness::new(
        &vkey,
        &signature,
        chain_code,
        addr.attributes(),
    )
}

#[wasm_bindgen]
pub fn make_icarus_bootstrap_witness(
    tx_body_hash: &TransactionHash,
    addr: &ByronAddress,
    key: &Bip32PrivateKey,
) -> BootstrapWitness {
    let chain_code = key.chaincode();

    let raw_key = key.to_raw_key();
    let vkey = Vkey::new(&raw_key.to_public());
    let signature = raw_key.sign(&tx_body_hash.to_bytes());

    BootstrapWitness::new(
        &vkey,
        &signature,
        chain_code,
        addr.attributes(),
    )
}

#[wasm_bindgen]
pub fn make_vkey_witness(
    tx_body_hash: &TransactionHash,
    sk: &PrivateKey
) -> Vkeywitness {
    let sig = sk.sign(tx_body_hash.0.as_ref());
    Vkeywitness::new(&Vkey::new(&sk.to_public()), &sig)
}

#[wasm_bindgen]
pub fn hash_auxiliary_data(auxiliary_data: &AuxiliaryData) -> AuxiliaryDataHash {
  AuxiliaryDataHash::from(blake2b256(&auxiliary_data.to_bytes()))
}
#[wasm_bindgen]
pub fn hash_transaction(tx_body: &TransactionBody) -> TransactionHash {
    TransactionHash::from(crypto::blake2b256(tx_body.to_bytes().as_ref()))
}

// wasm-bindgen can't accept Option without clearing memory, so we avoid exposing this in WASM
pub fn internal_get_implicit_input(
    withdrawals: &Option<Withdrawals>,
    certs: &Option<Certificates>,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum, // protocol parameter
) -> Result<Value, JsError> {
    let withdrawal_sum = match &withdrawals {
        None => to_bignum(0),
        Some(x) => x.0
            .values()
            .try_fold(
                to_bignum(0),
                |acc, ref withdrawal_amt| acc.checked_add(&withdrawal_amt)
            )?,
    };
    let certificate_refund = match &certs {
        None => to_bignum(0),
        Some(certs) => certs.0
            .iter()
            .try_fold(
                to_bignum(0),
                |acc, ref cert| match &cert.0 {
                    CertificateEnum::PoolRetirement(_cert) => acc.checked_add(&pool_deposit),
                    CertificateEnum::StakeDeregistration(_cert) => acc.checked_add(&key_deposit),
                    _ => Ok(acc),
                }
            )?
    };

    Ok(Value::new(&withdrawal_sum.checked_add(&certificate_refund)?))
}
pub fn internal_get_deposit(
    certs: &Option<Certificates>,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum, // protocol parameter
) -> Result<Coin, JsError> {
    let certificate_refund = match &certs {
        None => to_bignum(0),
        Some(certs) => certs.0
            .iter()
            .try_fold(
                to_bignum(0),
                |acc, ref cert| match &cert.0 {
                    CertificateEnum::PoolRegistration(_cert) => acc.checked_add(&pool_deposit),
                    CertificateEnum::StakeRegistration(_cert) => acc.checked_add(&key_deposit),
                    _ => Ok(acc),
                }
            )?
    };
    Ok(certificate_refund)
}


#[wasm_bindgen]
pub fn get_implicit_input(
    txbody: &TransactionBody,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum, // protocol parameter
) -> Result<Value, JsError> {
    internal_get_implicit_input(
        &txbody.withdrawals,
        &txbody.certs,
        &pool_deposit,
        &key_deposit,
    )
}

#[wasm_bindgen]
pub fn get_deposit(
    txbody: &TransactionBody,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum, // protocol parameter
) -> Result<Coin, JsError> {
    internal_get_deposit(
        &txbody.certs,
        &pool_deposit,
        &key_deposit,
    )
}

struct OutputSizeConstants {
    k0: usize,
    k1: usize,
    k2: usize,
}

fn quot<T>(a: T, b: T) -> T
where T: Sub<Output=T> + Rem<Output=T> + Div<Output=T> + Copy + Clone + std::fmt::Display {
    (a - (a % b)) / b
}

fn bundle_size(
    assets: &Value,
    constants: &OutputSizeConstants,
) -> usize {
    // based on https://github.com/input-output-hk/cardano-ledger-specs/blob/master/doc/explanations/min-utxo.rst
    match &assets.multiasset {
        None => 1, // Haskell codebase considers these size 1
        Some (assets) => {
            let num_assets = assets.0
                .values()
                .fold(
                    0,
                    | acc, next| acc + next.len()
                );
            let sum_asset_name_lengths = assets.0
                .values()
                .flat_map(|assets| assets.0.keys())
                .unique_by(|asset| asset.name())
                .fold(
                    0,
                    | acc, next| acc + next.0.len()
                );
            let sum_policy_id_lengths = assets.0
                .keys()
                .fold(
                    0,
                    | acc, next| acc + next.0.len()
                );
            // converts bytes to 8-byte long words, rounding up
            fn roundup_bytes_to_words(b: usize) -> usize {
                quot(b + 7, 8)
            };
            constants.k0 + roundup_bytes_to_words(
                (num_assets * constants.k1) + sum_asset_name_lengths +
                (constants.k2 * sum_policy_id_lengths)
            )
        }
    }
}

#[wasm_bindgen]
pub fn min_ada_required(
    assets: &Value,
    minimum_utxo_val: &BigNum, // protocol parameter
) -> BigNum {
    // based on https://github.com/input-output-hk/cardano-ledger-specs/blob/master/doc/explanations/min-utxo.rst
    match &assets.multiasset {
        None => minimum_utxo_val.clone(),
        Some(_assets) => {
            // NOTE: should be 2, but a bug in Haskell set this to 0
            let coin_size: u64 = 0;
            let tx_out_len_no_val = 14;
            let tx_in_len = 7;
            let utxo_entry_size_without_val: u64 = 6 + tx_out_len_no_val + tx_in_len; // 27

            // NOTE: should be 29 but a bug in Haskell set this to 27
            let ada_only_utxo_size: u64 = utxo_entry_size_without_val + coin_size;

            let size = bundle_size(
                &assets,
                &OutputSizeConstants {
                    k0: 6,
                    k1: 12,
                    k2: 1,
                },
            );
            BigNum(cmp::max(
                minimum_utxo_val.0,
                quot(minimum_utxo_val.0, ada_only_utxo_size) * (utxo_entry_size_without_val + (size as u64))
            ))
        }
    }
}

/// Used to choosed the schema for a script JSON string
#[wasm_bindgen]
pub enum ScriptSchema {
    Wallet,
    Node,
}

/// Receives a script JSON string
/// and returns a NativeScript.
/// Cardano Wallet and Node styles are supported.
///
/// * wallet: https://github.com/input-output-hk/cardano-wallet/blob/master/specifications/api/swagger.yaml
/// * node: https://github.com/input-output-hk/cardano-node/blob/master/doc/reference/simple-scripts.md
///
/// self_xpub is expected to be a Bip32PublicKey as hex-encoded bytes
#[wasm_bindgen]
pub fn encode_json_str_to_native_script(
    json: &str,
    self_xpub: &str,
    schema: ScriptSchema,
) -> Result<NativeScript, JsError> {
    let value: serde_json::Value =
        serde_json::from_str(&json).map_err(|e| JsError::from_str(&e.to_string()))?;

    let native_script = match schema {
        ScriptSchema::Wallet => encode_wallet_value_to_native_script(value, self_address)?,
        ScriptSchema::Node => todo!(),
    };

    Ok(native_script)
}

fn encode_wallet_value_to_native_script(value: serde_json::Value, self_xpub: &str) -> Result<NativeScript, JsError> {
    match value {
        serde_json::Value::Object(map)
            if map.contains_key("cosigners") && map.contains_key("template") =>
        {
            let mut cosigners = HashMap::new();

            if let serde_json::Value::Object(cosigner_map) = map.get("cosigners").unwrap() {
                for (key, value) in cosigner_map.iter() {
                    if let serde_json::Value::String(xpub) = value {
                        if xpub == "self" {
                            cosigners.insert(key.to_owned(), self_xpub.to_owned());
                        } else {
                            cosigners.insert(key.to_owned(), xpub.to_owned());
                        }
                    } else {
                        return Err(JsError::from_str("cosigner value must be a string"));
                    }
                }
            } else {
                return Err(JsError::from_str("cosigners must be a map"));
            }

            let template = map.get("template").unwrap();

            let template_native_script = encode_template_to_native_script(template, &cosigners)?;

            Ok(template_native_script)
        }
        _ => Err(JsError::from_str(
            "top level must be an object. cosigners and template keys are required",
        )),
    }
}

fn encode_template_to_native_script(
    template: &serde_json::Value,
    cosigners: &HashMap<String, String>,
) -> Result<NativeScript, JsError> {
    match template {
        serde_json::Value::String(cosigner) => {
            if let Some(xpub) = cosigners.get(cosigner) {
                let bytes =
                    Vec::from_hex(xpub).map_err(|e| JsError::from_str(&e.to_string()))?;

                let public_key = Bip32PublicKey::from_bytes(&bytes)?;

                Ok(NativeScript::new_script_pubkey(&ScriptPubkey::new(
                    &public_key.to_raw_key().hash(),
                )))
            } else {
                Err(JsError::from_str(&format!("cosigner {} not found", cosigner)))
            }
        }
        serde_json::Value::Object(map) if map.contains_key("all") => {
            let mut all = NativeScripts::new();

            if let serde_json::Value::Array(array) = map.get("all").unwrap() {
                for val in array {
                    all.add(&encode_template_to_native_script(val, cosigners)?);
                }
            } else {
                return Err(JsError::from_str("all must be an array"));
            }

            Ok(NativeScript::new_script_all(&ScriptAll::new(&all)))
        }
        serde_json::Value::Object(map) if map.contains_key("any") => {
            let mut any = NativeScripts::new();

            if let serde_json::Value::Array(array) = map.get("any").unwrap() {
                for val in array {
                    any.add(&encode_template_to_native_script(val, cosigners)?);
                }
            } else {
                return Err(JsError::from_str("any must be an array"));
            }

            Ok(NativeScript::new_script_any(&ScriptAny::new(&any)))
        }
        serde_json::Value::Object(map) if map.contains_key("some") => {
            if let serde_json::Value::Object(some) = map.get("some").unwrap() {
                if some.contains_key("at_least") && some.contains_key("from") {
                    let n = if let serde_json::Value::Number(at_least) =
                        some.get("at_least").unwrap()
                    {
                        if let Some(n) = at_least.as_u64() {
                            n as u32
                        } else {
                            return Err(JsError::from_str("at_least must be an integer"));
                        }
                    } else {
                        return Err(JsError::from_str("at_least must be an integer"));
                    };

                    let mut from_scripts = NativeScripts::new();

                    if let serde_json::Value::Array(array) = some.get("from").unwrap() {
                        for val in array {
                            from_scripts
                                .add(&encode_template_to_native_script(val, cosigners)?);
                        }
                    } else {
                        return Err(JsError::from_str("from must be an array"));
                    }

                    Ok(NativeScript::new_script_n_of_k(&ScriptNOfK::new(
                        n,
                        &from_scripts,
                    )))
                } else {
                    Err(JsError::from_str("some must contain at_least and from"))
                }
            } else {
                Err(JsError::from_str("some must be an object"))
            }
        }
        serde_json::Value::Object(map) if map.contains_key("active_from") => {
            if let serde_json::Value::Number(active_from) = map.get("active_from").unwrap() {
                if let Some(n) = active_from.as_u64() {
                    let slot: u32 = n as u32;

                    let time_lock_start = TimelockStart::new(slot);

                    Ok(NativeScript::new_timelock_start(&time_lock_start))
                } else {
                    Err(JsError::from_str(
                        "active_from slot must be an integer greater than or equal to 0",
                    ))
                }
            } else {
                Err(JsError::from_str("active_from slot must be a number"))
            }
        }
        serde_json::Value::Object(map) if map.contains_key("active_until") => {
            if let serde_json::Value::Number(active_until) = map.get("active_until").unwrap() {
                if let Some(n) = active_until.as_u64() {
                    let slot: u32 = n as u32;

                    let time_lock_expiry = TimelockExpiry::new(slot);

                    Ok(NativeScript::new_timelock_expiry(&time_lock_expiry))
                } else {
                    Err(JsError::from_str(
                        "active_until slot must be an integer greater than or equal to 0",
                    ))
                }
            } else {
                Err(JsError::from_str("active_until slot must be a number"))
            }
        }
        _ => Err(JsError::from_str("invalid template format")),
    }
}

#[cfg(test)]
mod tests {
    use hex::FromHex;

    use super::*;

    // this is what is used in mainnet
    static MINIMUM_UTXO_VAL: u64 = 1_000_000;

    #[test]
    fn native_scripts_from_wallet_json() {
        let cosigner0_hex = "1423856bc91c49e928f6f30f4e8d665d53eb4ab6028bd0ac971809d514c92db11423856bc91c49e928f6f30f4e8d665d53eb4ab6028bd0ac971809d514c92db1";
        let cosigner1_hex = "a48d97f57ce49433f347d44ee07e54a100229b4f8e125d25f7bca9ad66d9707a25cd1331f46f7d6e279451637ca20802a25c441ba9436abf644fe5410d1080e3";
        let self_key_hex = "6ce83a12e9d4c783f54c0bb511303b37160a6e4f3f96b8e878a7c1f7751e18c4ccde3fb916d330d07f7bd51fb6bd99aa831d925008d3f7795033f48abd6df7f6";
        let native_script = encode_json_str_to_native_script(
            &format!(r#"
            {{
                "cosigners": {{
                    "cosigner#0": "{}",
                    "cosigner#1": "{}",
                    "cosigner#2": "self"
                }},
                "template": {{
                    "some": {{
                        "at_least": 2,
                        "from": [
                            {{
                                "all": [
                                    "cosigner#0",
                                    {{ "active_from": 120 }}
                                ]
                            }},
                            {{
                                "any": [
                                    "cosigner#1",
                                    {{ "active_until": 1000 }}
                                ]
                            }},
                            "cosigner#2"
                        ]
                    }}
                }}
            }}"#, cosigner0_hex, cosigner1_hex),
            self_key_hex,
            ScriptSchema::Wallet,
        );

        let n_of_k = native_script.unwrap().as_script_n_of_k().unwrap();
        let from = n_of_k.native_scripts();
        assert_eq!(n_of_k.n(), 2);
        assert_eq!(from.len(), 3);
        let all = from.get(0).as_script_all().unwrap().native_scripts();
        assert_eq!(all.len(), 2);
        let all_0 = all.get(0).as_script_pubkey().unwrap();
        assert_eq!(
            all_0.addr_keyhash(),
            Bip32PublicKey::from_bytes(&hex::decode(cosigner0_hex).unwrap()).unwrap().to_raw_key().hash()
        );
        let all_1 = all.get(1).as_timelock_start().unwrap();
        assert_eq!(all_1.slot(), 120);
        let any = from.get(1).as_script_any().unwrap().native_scripts();
        assert_eq!(all.len(), 2);
        let any_0 = any.get(0).as_script_pubkey().unwrap();
        assert_eq!(
            any_0.addr_keyhash(),
            Bip32PublicKey::from_bytes(&hex::decode(cosigner1_hex).unwrap()).unwrap().to_raw_key().hash()
        );
        let any_1 = any.get(1).as_timelock_expiry().unwrap();
        assert_eq!(any_1.slot(), 1000);
        let self_key = from.get(2).as_script_pubkey().unwrap();
        assert_eq!(
            self_key.addr_keyhash(),
            Bip32PublicKey::from_bytes(&hex::decode(self_key_hex).unwrap()).unwrap().to_raw_key().hash()
        );
    }

    #[test]
    fn no_token_minimum() {
        
        let assets = Value {
            coin: BigNum(0),
            multiasset: None,
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            MINIMUM_UTXO_VAL
        );
    }

    #[test]
    fn one_policy_one_smallest_name() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![]),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1407406),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1407406
        );
    }

    #[test]
    fn one_policy_one_small_name() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![1]),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1444443),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1444443
        );
    }

    #[test]
    fn one_policy_one_largest_name() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            // The largest asset names have length thirty-two
            &AssetName([1; 32].to_vec()),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1555554),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1555554
        );
    }

    #[test]
    fn one_policy_three_small_names() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![1]),
            &BigNum(1)
        );
        asset_list.insert(
            &AssetName(vec![2]),
            &BigNum(1)
        );
        asset_list.insert(
            &AssetName(vec![3]),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1555554),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1555554
        );
    }

    #[test]
    fn one_policy_three_largest_names() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            // The largest asset names have length thirty-two
            &AssetName([1; 32].to_vec()),
            &BigNum(1)
        );
        asset_list.insert(
            // The largest asset names have length thirty-two
            &AssetName([2; 32].to_vec()),
            &BigNum(1)
        );
        asset_list.insert(
            // The largest asset names have length thirty-two
            &AssetName([3; 32].to_vec()),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1962961),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1962961
        );
    }

    #[test]
    fn two_policies_one_smallest_name() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![]),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        token_bundle.insert(
            &PolicyID::from([1; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1592591),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1592591
        );
    }

    #[test]
    fn two_policies_two_small_names() {
        
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![]),
            &BigNum(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        token_bundle.insert(
            &PolicyID::from([1; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        let assets = Value {
            coin: BigNum(1592591),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            1592591
        );
    }

    #[test]
    fn three_policies_99_small_names() {
        
        let mut token_bundle = MultiAsset::new();
        fn add_policy(token_bundle: &mut MultiAsset, index: u8) -> () {
            let mut asset_list = Assets::new();

            for i in 0..33 {
                asset_list.insert(
                    &AssetName(vec![i]),
                    &BigNum(1)
                );
            }
            token_bundle.insert(
                &PolicyID::from([index; ScriptHash::BYTE_COUNT]),
                &asset_list
            );
        }
        add_policy(&mut token_bundle, 1);
        add_policy(&mut token_bundle, 2);
        add_policy(&mut token_bundle, 3);
        let assets = Value {
            coin: BigNum(7592585),
            multiasset: Some(token_bundle),
        };
        
        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            7296289
        );
    }
    
    #[test]
    fn proper_asset_size_calculation() {
        let assets = Value::from_bytes(
            Vec::from_hex("821a011cd498b82f581c4fd307695d244431ca93599be34d9aea403f24cf30f2eadcd7a178e2a1487377616e6b70696501581c50000001f7d74d2f95f82c4cfeb13f4119c350c2f91090fd69757fbda34f43617264616e6f526f6e696e313130014f43617264616e6f526f6e696e323732014f43617264616e6f526f6e696e34313401581c5090573eaf3ca637bce1fb23afc98a7ae1bc803df151a3c82e0921b5a14e4b6174654665617244656164313001581c513cdfefd0c39c79ff77b77f28aca06e74a6f3bd63a2599f8481db41a15454686546697273744b6579686f6c64657230383101581c52f02b66a66da2935d8b2c6424956a000b15e971004b1e32d88e4820a15254484548494748505249455354455353303101581c531052a4c667b84648c74f362c0c1878c3001b0f7696815752a4bed1a1554e46545530303132506c616e657450616c7a30353301581c558430d47055f97e15bfeb264a68c8e1c5acef1712bd36ab31ceb22ca14346414d01581c55dd4519ca2ce4071ba005eb209506d2140cccba49cdc3419dda695ca148417269657330303501581c56cffff175760004b9f2da7d2120341baee137f2131b09e7a0ad3eb4a1581b7766303030344368616f7469634d617468656d617469637330313801581c570f3e65dc46e8edf8ac288f13385297e6f749a8fd84064cdcda2839a1534e465455303031384e46544372617a6530393101581c597ecdd4be1ffdcd9426ca4cfaaf75285650351045e21bfc7b7f0ba8a1465275706565730a581c5b597e4f6560f9403dcba7618fef04df235bfcb72552176c0e8a599ba1444265657201581c5d58881d98f367befbce25ff966ee22b4de8f099da0e7f71c089e542a35543436f756e747269657342726f6e7a654d6564616c01464368696e613801464a6170616e3901581c5df412e3eb27ffa1665302f8bf97f74f89fa42215d3e5b09464c1c14a14b4c75636b794e654b6f696e190649581c61aaaaad465f0f8a28aec3461beaf38990f671f2e4bc979ddda194c4a1581b5468654d6973616476656e74757265736f66426f6763686932316101581c62ea7cb573306f6c272a2ff066679f2e4216270311d8e71b5f765251ac4a4164616c6f746c303134014a4164616c6f746c303434014a4164616c6f746c313439014a4164616c6f746c323337014a4164616c6f746c323430014a4164616c6f746c343432014a4164616c6f746c343836014a4164616c6f746c363036014a4164616c6f746c363530014a4164616c6f746c363638014a4164616c6f746c363932014a4164616c6f746c38333801581c6590687768bf097900b546a15efb0e413010cbefca2ade9f629c6d43a14c566f78656c697a656432303801581c66fb86c135d9d1350d29abff0b1c549a4b8204885227f76346601782a1475448454d4f4f4e01581c68fb69abc121dfb77be74e5d589876ea7fa271a70a905a4bbc580a60a1554e465455303030365375736869427974657330333101581c6ce4f2fd965a3f782baa0b9aa421c21299d434ad6c566ed077d6d663a1504475636343616d65466972737430363601581c73c590cc5d909ff19c2521bf14f673928a1fe3ff0e2c6eaa7bd36d5da14a45617450697a7a61303101581c76c2ddb32f3d974be983c39789313ab26e1791ef7a1bb09a55d2ade9a158186e65766572466f72676574446973636f726452617265303401581c77e4da914068a50d9d9420dbdda80817b55516ac6304273879318d5aa14f43756c746f664c75636b794e656b6f01581c78517792ad45d22f70a8ec4a1d30c458925938a8e7a5d9acb2449a63ad4f434d42594e41444150555a5a4c45310153434d42594e41444150555a5a4c45424b524437014c434d42594e50555a5a4c4531014d434d42594e50555a5a4c453130014d434d42594e50555a5a4c453131014c434d42594e50555a5a4c4532014c434d42594e50555a5a4c4533014c434d42594e50555a5a4c4534014c434d42594e50555a5a4c4535014c434d42594e50555a5a4c4536014c434d42594e50555a5a4c4537014c434d42594e50555a5a4c4538014c434d42594e50555a5a4c453901581c787622826f8ebd1baa04e5a9a76ae0eac7392f730544528738faafcba14f526f62696e5265645374616d70303901581c7b5fd95985e08b72a5c37b9d3c7d863bb8a6fed82ef4741594be357da1487377616e6b70696501581c7b9e74668dad56367f9314485a402c726d3bb8561834fdf253c65c4ba14f566973696f6e73426c61636b4f696c01581c7c306b00720b3a941ede9d6e1c469ec678cea1d1be8f70ff146dd6c8a1487377616e6b70696501581c84c0acb101c14416ad92859c429058871e201804468d5f353be31d71a64f50756e6b7374657241727432343038014f50756e6b7374657241727436393534014f50756e6b7374657241727437343733014f50756e6b7374657241727437353237014f50756e6b7374657241727437363831014f50756e6b737465724172743736393501581c8579617b51f533912b1652b7baa57a97335490848ec750741699db3ca34a535452454e4754483031014a544845444556494c3031014b5448454c4f56455253303101581c88814632e81b0d0d92d76bf7c7321351618a331558eb76f699f62580a1444f50323301581c89ee9ed7ce189c466be5937af4dc9103d71cc9ec150efdf158d4fe13a35043617264616e6f417065334431313830015043617264616e6f417065334432363032015043617264616e6f41706533443237383601581c89fa6dc66a24799ccaee43a3a16930bb045a8152fdf2a2642034774fa14f506c616e657450616c7a313134303801581c8acb8d48ccee9f22265bcf1f41b4bcffbcecf5a5a85c5e4e1bb7bad0a1464f726967696e01581c8bd876119ed2152848cc364db9fab76c5ed8d98fdf53c2157ffd4092a1487377616e6b70696501581c8f80ebfaf62a8c33ae2adf047572604c74db8bc1daba2b43f9a65635a45243617264616e6f57617272696f7235393535015243617264616e6f57617272696f7236313933015243617264616e6f57617272696f7238393131015243617264616e6f57617272696f723938383101581c8fea90d673cbdd5b3da3309fb7cf1dcbb1c485ba6a7e5148468351a2a14f457570686f726963446f736573333601581c900c8cf13faf04300988e173b2695b74423da2be1615e544ddf7f9d7a14d48617070795768616c6530303101581c91acca0a2614212d68a5ae7313c85962849994aab54e340d3a68aabba14653483435363101581c922cb8a086179b1b0464f1b80c1679a210d9ed9b1e1b8374f27496e6a1445250313401581c92e7d5f2ac1994d2916d547395ab8ce650bb7387a0f70a9efbd543a6a1534e46545530303131436f6e6a7572657231363401581c942f26b14e57ab29ceac5f5d0e1ce392001ac486a019ab272f37c9c5a3581a434d42594e784261636b776172647347656f6d6574727942303801581a434d42594e784261636b776172647347656f6d6574727945303301581a434d42594e784261636b776172647347656f6d6574727947303801581c949dbdd63f3157100e3b98dffb20259f5053bcddb1ce6fd253c6da85a54e466c6f77496e64696142756c6c30014e466c6f77496e646961466f757231014d466c6f77496e6469614f6e6532014f466c6f77496e646961546872656536014d466c6f77496e64696154776f3601581c97305ec3684b4e5ac2977d44ee05fd453b038ae882e665fd6499484ca45818456e70696d6f6e7943616e61646143656e746175723030380156456e70696d6f6e7943616e616461476f6c656d3031330156456e70696d6f6e7943616e616461486f7273653032330157456e70696d6f6e7943616e6164614b6e6967687430383701581ca8731ef90e36acc7083d9bd501f733cb610ad67e7143d891fd45ab89a14e53776565744b697473756e65303901581cabe233937c19a90f826d94f121029a2fccf2bc411b9c5456e2ba49dba155496e746f5468654c6f6f6b696e67476c617373303401581cd3bbe5b2a27a392fbac2471e0c42f54ddd55da1005f86e9c18c3e082a1581a526f79616c4d6348756d70696e4b69747469657342656163683201").unwrap()
        ).unwrap();

        assert_eq!(
            min_ada_required(&assets, &BigNum(MINIMUM_UTXO_VAL)).0,
            18666648
        );
    }

    #[test]
    fn subtract_values() {
        let policy1 = PolicyID::from([0; ScriptHash::BYTE_COUNT]);
        let policy2 = PolicyID::from([1; ScriptHash::BYTE_COUNT]);

        let asset1 = AssetName(vec![1]);
        let asset2 = AssetName(vec![2]);
        let asset3 = AssetName(vec![3]);
        let asset4 = AssetName(vec![4]);

        let mut token_bundle1 = MultiAsset::new();
        {
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            asset_list1.insert(
                &asset2,
                &BigNum(1)
            );
            asset_list1.insert(
                &asset3,
                &BigNum(1)
            );
            asset_list1.insert(
                &asset4,
                &BigNum(2)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );

            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy2,
                &asset_list2
            );
        }
        let assets1 = Value {
            coin: BigNum(1555554),
            multiasset: Some(token_bundle1),
        };

        let mut token_bundle2 = MultiAsset::new();
        {
            let mut asset_list2 = Assets::new();
            // more than asset1 bundle
            asset_list2.insert(
                &asset1,
                &BigNum(2)
            );
            // exactly equal to asset1 bundle
            asset_list2.insert(
                &asset2,
                &BigNum(1)
            );
            // skip asset 3
            // less than in asset1 bundle
            asset_list2.insert(
                &asset4,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );

            // this policy should be removed entirely
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy2,
                &asset_list2
            );
        }

        let assets2 = Value {
            coin: BigNum(2555554),
            multiasset: Some(token_bundle2),
        };

        let result = assets1.clamped_sub(&assets2);
        assert_eq!(
            result.coin().to_str(),
            "0"
        );
        assert_eq!(
            result.multiasset().unwrap().len(),
            1 // policy 2 was deleted successfully
        );
        let policy1_content = result.multiasset().unwrap().get(&policy1).unwrap();
        assert_eq!(
            policy1_content.len(),
            2
        );
        assert_eq!(
            policy1_content.get(&asset3).unwrap().to_str(),
            "1"
        );
        assert_eq!(
            policy1_content.get(&asset4).unwrap().to_str(),
            "1"
        );
    }

    #[test]
    fn compare_values() {
        let policy1 = PolicyID::from([0; ScriptHash::BYTE_COUNT]);

        let asset1 = AssetName(vec![1]);
        let asset2 = AssetName(vec![2]);

        // testing cases with no assets
        {
            let a = Value::new(&to_bignum(1));
            let b = Value::new(&to_bignum(1));
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Equal);
        }
        {
            let a = Value::new(&to_bignum(2));
            let b = Value::new(&to_bignum(1));
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let a = Value::new(&to_bignum(1));
            let b = Value::new(&to_bignum(2));
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        // testing case where one side has assets
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };
            let b = Value::new(&to_bignum(1));
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value::new(&to_bignum(1));
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        // testing case where both sides has assets
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Equal);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Greater);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b), None);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b).unwrap(), std::cmp::Ordering::Less);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(2),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset1,
                &BigNum(2)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b), None);
        }
        {
            let mut token_bundle1 = MultiAsset::new();
            let mut asset_list1 = Assets::new();
            asset_list1.insert(
                &asset1,
                &BigNum(1)
            );
            token_bundle1.insert(
                &policy1,
                &asset_list1
            );
            let a = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle1),
            };

            let mut token_bundle2 = MultiAsset::new();
            let mut asset_list2 = Assets::new();
            asset_list2.insert(
                &asset2,
                &BigNum(1)
            );
            token_bundle2.insert(
                &policy1,
                &asset_list2
            );
            let b = Value {
                coin: BigNum(1),
                multiasset: Some(token_bundle2),
            };
            assert_eq!(a.partial_cmp(&b), None);
        }
    }

    #[test]
    fn bigint_serialization() {
        let zero = BigInt::from_str("0").unwrap();
        let zero_rt = BigInt::from_bytes(zero.to_bytes()).unwrap();
        assert_eq!(zero.to_str(), zero_rt.to_str());

        let pos_small = BigInt::from_str("100").unwrap();
        let pos_small_rt = BigInt::from_bytes(pos_small.to_bytes()).unwrap();
        assert_eq!(pos_small.to_str(), pos_small_rt.to_str());

        let pos_big = BigInt::from_str("123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890").unwrap();
        let pos_big_rt = BigInt::from_bytes(pos_big.to_bytes()).unwrap();
        assert_eq!(pos_big.to_str(), pos_big_rt.to_str());

        let neg_small = BigInt::from_str("-100").unwrap();
        let neg_small_rt = BigInt::from_bytes(neg_small.to_bytes()).unwrap();
        assert_eq!(neg_small.to_str(), neg_small_rt.to_str());

        let neg_big = BigInt::from_str("-123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890").unwrap();
        let neg_big_rt = BigInt::from_bytes(neg_big.to_bytes()).unwrap();
        assert_eq!(neg_big.to_str(), neg_big_rt.to_str());

        // taken from CBOR RFC examples
        // negative big int
        assert_eq!(hex::decode("c349010000000000000000").unwrap(), BigInt::from_str("-18446744073709551617").unwrap().to_bytes());
        // positive big int
        assert_eq!(hex::decode("c249010000000000000000").unwrap(), BigInt::from_str("18446744073709551616").unwrap().to_bytes());
        // uint
        assert_eq!(hex::decode("1b000000e8d4a51000").unwrap(), BigInt::from_str("1000000000000").unwrap().to_bytes());
        // nint
        // we can't use this due to cbor_event actually not supporting the full NINT spectrum as it uses an i64 for some reason...
        //assert_eq!(hex::decode("3bffffffffffffffff").unwrap(), BigInt::from_str("-18446744073709551616").unwrap().to_bytes());
        // this one fits in an i64 though
        assert_eq!(hex::decode("3903e7").unwrap(), BigInt::from_str("-1000").unwrap().to_bytes());


        let x = BigInt::from_str("-18446744073709551617").unwrap();
        let x_rt = BigInt::from_bytes(x.to_bytes()).unwrap();
        assert_eq!(x.to_str(), x_rt.to_str());
    }
}
