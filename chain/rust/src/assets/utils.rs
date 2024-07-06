use crate::PolicyId;
use cbor_event::{de::Deserializer, se::Serializer};
use cml_core::{
    error::{DeserializeError, DeserializeFailure, Key},
    ordered_hash_map::OrderedHashMap,
    serialization::{fit_sz, CBORReadLen, Deserialize, LenEncoding, Serialize, StringEncoding},
    ArithmeticError,
};
use cml_crypto::{RawBytesEncoding, ScriptHash};
use std::io::{BufRead, Seek, Write};
use std::{
    cmp::PartialOrd,
    convert::{TryFrom, TryInto},
};

use std::collections::BTreeMap;

use super::AssetName;

pub type Coin = u64;

/// This should technically now allow 0 but this would make the API harder to use.
/// At least for now we'll keep it as Coin (aka u64) as an alias with this comment here.
/// Later on it could potentially be redone to its own struct.
pub type NonZeroInt64 = i64;

/// This should technically now allow 0 but this would make the API harder to use.
/// At least for now we'll keep it as Coin (aka u64) as an alias with this comment here.
/// Later on it could potentially be redone to its own struct.
pub type PositiveCoin = Coin;

#[derive(Debug, thiserror::Error)]
pub enum AssetArithmeticError {
    #[error("Arithmetic failed: {0}")]
    Arithmetic(#[from] ArithmeticError),
    #[error("Asset {0:?} doesn't exist")]
    AssetDoesntExist(AssetName),
    #[error("PolicyId {0:?} doesn't exist")]
    PolicyIdDoesntExist(PolicyId),
}

impl TryFrom<&str> for AssetName {
    type Error = DeserializeError;

    fn try_from(utf8_str: &str) -> Result<Self, Self::Error> {
        Self::new(utf8_str.as_bytes().to_vec())
    }
}

impl<'a> TryInto<&'a str> for &'a AssetName {
    type Error = std::str::Utf8Error;

    fn try_into(self) -> Result<&'a str, Self::Error> {
        std::str::from_utf8(self.to_raw_bytes())
    }
}

impl RawBytesEncoding for AssetName {
    fn to_raw_bytes(&self) -> &[u8] {
        self.inner.as_ref()
    }

    fn from_raw_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Self::new(bytes.to_vec())
    }
}

/// Bundle of assets within range of T, grouped by PolicyID then AssetName
#[derive(
    Clone, Default, PartialEq, Hash, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
pub struct AssetBundle<T>(OrderedHashMap<PolicyId, OrderedHashMap<AssetName, T>>);

impl<T: std::fmt::Debug> std::fmt::Debug for AssetBundle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut ds = f.debug_struct(if self.0.iter().any(|(_, a)| !a.is_empty()) {
            ""
        } else {
            "{}"
        });
        for (pid, assets) in self.0.iter() {
            let pid_hex = hex::encode(pid.to_raw_bytes());
            for (an, val) in assets.iter() {
                let an_hex = hex::encode(an.to_raw_bytes());
                let an_name = if an_hex.len() > 8 {
                    format!(
                        "{}..{}",
                        an_hex.get(0..3).unwrap(),
                        an_hex.get(an_hex.len() - 2..an_hex.len() - 1).unwrap()
                    )
                } else {
                    an_hex
                };
                ds.field(
                    &format!(
                        "{}..{}:{}",
                        pid_hex.get(0..3).unwrap(),
                        pid_hex.get(pid_hex.len() - 4..pid_hex.len() - 1).unwrap(),
                        an_name
                    ),
                    val,
                );
            }
        }
        ds.finish()
    }
}

impl<T: Default> AssetBundle<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> From<OrderedHashMap<PolicyId, OrderedHashMap<AssetName, T>>> for AssetBundle<T> {
    fn from(bundle: OrderedHashMap<PolicyId, OrderedHashMap<AssetName, T>>) -> Self {
        Self(bundle)
    }
}

impl<T> std::ops::Deref for AssetBundle<T> {
    type Target = OrderedHashMap<PolicyId, OrderedHashMap<AssetName, T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for AssetBundle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Subtraction over a numerical type where the type's minimum is returned if underflow would occur.
pub trait ClampedSub {
    fn clamped_sub(&self, rhs: &Self) -> Self;
}

impl<T: num::CheckedSub + num::Bounded + num::Zero + Ord> ClampedSub for T {
    fn clamped_sub(&self, rhs: &Self) -> Self {
        self.checked_sub(rhs).unwrap_or_else(|| {
            if *rhs < T::zero() {
                T::max_value()
            } else {
                T::min_value()
            }
        })
    }
}

impl<T> ClampedSub for AssetBundle<T>
where
    T: num::CheckedAdd + num::CheckedSub + num::Zero + num::Bounded + Copy + Clone + ClampedSub,
{
    fn clamped_sub(&self, rhs: &Self) -> Self {
        let mut bundle = self.clone();
        for (policy, rhs_assets) in rhs.iter() {
            for (asset_name, rhs_amount) in rhs_assets.iter() {
                match bundle.get_mut(policy) {
                    Some(lhs_assets) => match lhs_assets.get_mut(asset_name) {
                        Some(lhs_amount) => match lhs_amount.checked_sub(rhs_amount) {
                            Some(new_lhs_amount) => {
                                if new_lhs_amount.is_zero() {
                                    lhs_assets.remove(asset_name);
                                    if lhs_assets.is_empty() {
                                        bundle.remove(policy);
                                    }
                                } else {
                                    *lhs_amount = new_lhs_amount;
                                }
                            }
                            None => {
                                if T::min_value().is_zero() {
                                    // if underflow and unsigned, remove
                                    lhs_assets.remove(asset_name);
                                    if lhs_assets.is_empty() {
                                        bundle.remove(policy);
                                    }
                                } else {
                                    // otherwise keep it as minimum
                                    *lhs_amount = T::min_value();
                                }
                            }
                        },
                        None => {
                            // asset name is missing from left hand side
                            if !T::min_value().is_zero() {
                                bundle.set(
                                    *policy,
                                    asset_name.clone(),
                                    T::zero().clamped_sub(rhs_amount),
                                );
                            }
                        }
                    },
                    None => {
                        // policy id missing from left hand side
                        if !T::min_value().is_zero() {
                            bundle.set(
                                *policy,
                                asset_name.clone(),
                                T::zero().clamped_sub(rhs_amount),
                            );
                        }
                    }
                }
            }
        }
        bundle
    }
}

impl<T> AssetBundle<T>
where
    T: num::CheckedAdd + num::CheckedSub + num::Zero + num::Bounded + Copy + Clone,
{
    /// Set the value of policy_id:asset_name to value.
    /// Returns the previous value, or None if it didn't exist
    pub fn set(&mut self, policy_id: PolicyId, asset_name: AssetName, value: T) -> Option<T> {
        self.0
            .entry(policy_id)
            .or_default()
            .insert(asset_name, value)
    }

    /// Get the value of policy_id:asset_name if it exists.
    pub fn get(&self, policy_id: &PolicyId, asset_name: &AssetName) -> Option<T> {
        self.0
            .get(policy_id)
            .and_then(|assets| assets.get(asset_name))
            .copied()
    }

    /// Adds to bundles together, checking value bounds.
    /// Does not modify self, and instead returns the result.
    pub fn checked_add(&self, rhs: &Self) -> Result<Self, AssetArithmeticError> {
        use linked_hash_map::Entry;
        let mut bundle = self.0.clone();
        for (policy, assets) in rhs.0.iter() {
            for (asset_name, amount) in assets.iter() {
                match bundle.entry(*policy) {
                    Entry::Occupied(mut assets) => {
                        match assets.get_mut().entry(asset_name.clone()) {
                            Entry::Occupied(mut assets2) => {
                                let current = assets2.get_mut();
                                *current = current
                                    .checked_add(amount)
                                    .ok_or(ArithmeticError::IntegerOverflow)?;
                            }
                            Entry::Vacant(vacant_entry) => {
                                vacant_entry.insert(*amount);
                            }
                        }
                    }
                    Entry::Vacant(entry) => {
                        let mut assets = OrderedHashMap::new();
                        assets.insert(asset_name.clone(), *amount);
                        entry.insert(assets);
                    }
                }
            }
        }
        Ok(Self(bundle))
    }

    /// Subtracts rhs from this bundle.
    /// This does not modify self, and instead returns the result.
    /// Use clamped_sub (ClampedSub trait) if you need to only try to remove assets when they exist
    /// and ignore them when they don't.
    pub fn checked_sub(&self, rhs: &Self) -> Result<Self, AssetArithmeticError> {
        let mut bundle = self.0.clone();
        for (policy, rhs_assets) in rhs.iter() {
            for (asset_name, rhs_amount) in rhs_assets.iter() {
                match bundle.get_mut(policy) {
                    Some(lhs_assets) => match lhs_assets.get_mut(asset_name) {
                        Some(lhs_amount) => match lhs_amount.checked_sub(rhs_amount) {
                            Some(new_lhs_amount) => {
                                if new_lhs_amount.is_zero() {
                                    lhs_assets.remove(asset_name);
                                    if lhs_assets.is_empty() {
                                        bundle.remove(policy);
                                    }
                                } else {
                                    *lhs_amount = new_lhs_amount;
                                }
                            }
                            None => {
                                // underflow
                                return Err(ArithmeticError::IntegerUnderflow.into());
                            }
                        },
                        None => {
                            // asset name is missing from left hand side
                            return Err(AssetArithmeticError::AssetDoesntExist(asset_name.clone()));
                        }
                    },
                    None => {
                        // policy id missing from left hand side
                        return Err(AssetArithmeticError::PolicyIdDoesntExist(*policy));
                    }
                }
            }
        }
        Ok(Self(bundle))
    }
}

pub type Mint = AssetBundle<NonZeroInt64>;

pub type MultiAsset = AssetBundle<PositiveCoin>;

impl Mint {
    fn as_multiasset(&self, is_positive: bool) -> MultiAsset {
        self.0
            .iter()
            .fold(MultiAsset::default(), |mut acc, (policy, assets)| {
                let new_assets =
                    assets
                        .iter()
                        .fold(OrderedHashMap::new(), |mut acc, (asset, value)| {
                            if (*value >= 0) == is_positive {
                                acc.insert(asset.clone(), value.unsigned_abs());
                            }
                            acc
                        });
                if !assets.is_empty() {
                    acc.insert(*policy, new_assets);
                }
                acc
            })
    }

    /// Returns the multiasset where only positive (minting) entries are present
    pub fn as_positive_multiasset(&self) -> MultiAsset {
        self.as_multiasset(true)
    }

    /// Returns the multiasset where only negative (burning) entries are present
    pub fn as_negative_multiasset(&self) -> MultiAsset {
        self.as_multiasset(false)
    }
}

// note: we purposefully don't derive or implement Ord for Value to avoid potentially confusing
// orderings that don't obey Cardano semantics i.e. if x >= y then input x can cover cost y
// If you need to use Value or something in a tree map please consider using a hash map instead.
#[derive(
    Clone, Debug, derivative::Derivative, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[derivative(Hash, Eq, PartialEq, PartialOrd)]
pub struct Value {
    pub coin: PositiveCoin,
    pub multiasset: MultiAsset,
    #[serde(skip)]
    #[derivative(Hash = "ignore", PartialEq = "ignore", PartialOrd = "ignore")]
    pub encodings: Option<ValueEncoding>,
}

impl Value {
    pub fn new(coin: PositiveCoin, multiasset: MultiAsset) -> Self {
        Self {
            coin,
            multiasset,
            encodings: None,
        }
    }

    pub fn zero() -> Value {
        0u64.into()
    }

    pub fn is_zero(&self) -> bool {
        self.coin == 0 && !self.has_multiassets()
    }

    pub fn has_multiassets(&self) -> bool {
        self.multiasset
            .values()
            .any(|assets| assets.values().any(|amount| *amount != 0))
    }

    pub fn checked_add(&self, rhs: &Value) -> Result<Self, AssetArithmeticError> {
        let coin = self
            .coin
            .checked_add(rhs.coin)
            .ok_or(ArithmeticError::IntegerOverflow)?;
        let multiasset = self.multiasset.checked_add(&rhs.multiasset)?;
        Ok(Value {
            coin,
            multiasset,
            encodings: None,
        })
    }

    /// Subtract ADA and/or assets
    /// Removes an asset from the list if the result is 0 or less
    /// Does not modify this object, instead the result is returned
    /// None is returned if there would be integer underflow
    pub fn checked_sub(&self, rhs: &Value) -> Result<Self, AssetArithmeticError> {
        let coin = self
            .coin
            .checked_sub(rhs.coin)
            .ok_or(ArithmeticError::IntegerUnderflow)?;
        let multiasset = self.multiasset.checked_sub(&rhs.multiasset)?;
        Ok(Value {
            coin,
            multiasset,
            encodings: None,
        })
    }

    pub fn clamped_sub(&self, rhs: &Value) -> Value {
        let coin = self.coin.clamped_sub(&rhs.coin);
        let multiasset = self.multiasset.clamped_sub(&rhs.multiasset);
        Value {
            coin,
            multiasset,
            encodings: None,
        }
    }
}

// deriving PartialOrd doesn't work in a way that's useful , as the
// implementation of PartialOrd for BTreeMap compares keys by their order,
// i.e, is equivalent to comparing the iterators of (pid, Assets).
// that would mean that: v1 < v2 if the min_pid(v1) < min_pid(v2)
// this function instead compares amounts, assuming that if a pair (pid, aname)
// is not in the MultiAsset then it has an amount of 0
impl<T> PartialOrd for AssetBundle<T>
where
    T: num::CheckedAdd + num::CheckedSub + num::Zero + num::Bounded + Copy + Clone + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // idea: if (a-b) > 0 for some asset, then a > b for at least some asset
        fn is_all_zeros<T>(lhs: &AssetBundle<T>, rhs: &AssetBundle<T>) -> bool
        where
            T: num::CheckedAdd
                + num::CheckedSub
                + num::Zero
                + num::Bounded
                + Copy
                + Clone
                + PartialOrd,
        {
            for (pid, assets) in lhs.0.iter() {
                for (aname, amount) in assets.iter() {
                    match amount
                        .checked_sub(&rhs.get(pid, aname).unwrap_or(T::zero()))
                        .and_then(|o| o.partial_cmp(&T::zero()))
                    {
                        Some(std::cmp::Ordering::Equal) => (),
                        _ => return false,
                    }
                }
            }
            true
        }

        match (is_all_zeros(self, other), is_all_zeros(other, self)) {
            (true, true) => Some(std::cmp::Ordering::Equal),
            (true, false) => Some(std::cmp::Ordering::Less),
            (false, true) => Some(std::cmp::Ordering::Greater),
            (false, false) => None,
        }
    }
}

impl From<PositiveCoin> for Value {
    fn from(coin: PositiveCoin) -> Self {
        Self {
            coin,
            multiasset: AssetBundle::default(),
            encodings: None,
        }
    }
}

impl From<MultiAsset> for Value {
    fn from(multiasset: MultiAsset) -> Self {
        Self {
            coin: 0,
            multiasset,
            encodings: None,
        }
    }
}

impl Serialize for Value {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if self.multiasset.is_empty()
            && self
                .encodings
                .as_ref()
                .map(|encs| !encs.use_multiasset_format)
                .unwrap_or(true)
        {
            // coin-only format
            serializer.write_unsigned_integer_sz(
                self.coin,
                fit_sz(
                    self.coin,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.coin_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            )
        } else {
            // general format
            serializer.write_array_sz(
                self.encodings
                    .as_ref()
                    .map(|encs| encs.len_encoding)
                    .unwrap_or_default()
                    .to_len_sz(2, force_canonical),
            )?;
            serializer.write_unsigned_integer_sz(
                self.coin,
                fit_sz(
                    self.coin,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.coin_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            )?;
            serializer.write_map_sz(
                self.encodings
                    .as_ref()
                    .map(|encs| encs.multiasset_encoding)
                    .unwrap_or_default()
                    .to_len_sz(self.multiasset.len() as u64, force_canonical),
            )?;
            let mut key_order = self
                .multiasset
                .iter()
                .map(|(k, v)| {
                    let mut buf = cbor_event::se::Serializer::new_vec();
                    let multiasset_key_encoding = self
                        .encodings
                        .as_ref()
                        .and_then(|encs| encs.multiasset_key_encodings.get(k))
                        .cloned()
                        .unwrap_or_default();
                    buf.write_bytes_sz(
                        k.to_raw_bytes(),
                        multiasset_key_encoding
                            .to_str_len_sz(k.to_raw_bytes().len() as u64, force_canonical),
                    )?;
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
            for (key_bytes, key, value) in key_order {
                serializer.write_raw_bytes(&key_bytes)?;
                let (multiasset_value_encoding, multiasset_value_value_encodings) = self
                    .encodings
                    .as_ref()
                    .and_then(|encs| encs.multiasset_value_encodings.get(key))
                    .cloned()
                    .unwrap_or_else(|| (LenEncoding::default(), BTreeMap::new()));
                serializer.write_map_sz(
                    multiasset_value_encoding.to_len_sz(value.len() as u64, force_canonical),
                )?;
                let mut key_order = value
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
                for (key_bytes, key, value) in key_order {
                    serializer.write_raw_bytes(&key_bytes)?;
                    let multiasset_value_value_encoding = multiasset_value_value_encodings
                        .get(key)
                        .cloned()
                        .unwrap_or_default();
                    serializer.write_unsigned_integer_sz(
                        *value,
                        fit_sz(*value, multiasset_value_value_encoding, force_canonical),
                    )?;
                }
                multiasset_value_encoding.end(serializer, force_canonical)?;
            }
            self.encodings
                .as_ref()
                .map(|encs| encs.multiasset_encoding)
                .unwrap_or_default()
                .end(serializer, force_canonical)?;
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .end(serializer, force_canonical)
        }
    }
}

impl Deserialize for Value {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                // coin-only format
                cbor_event::Type::UnsignedInteger => {
                    let (coin, coin_encoding) = raw
                        .unsigned_integer_sz()
                        .map(|(x, enc)| (x, Some(enc)))
                        .map_err(Into::<DeserializeError>::into)
                        .map_err(|e: DeserializeError| e.annotate("coin"))?;
                    Ok(Value {
                        coin,
                        multiasset: AssetBundle::default(),
                        encodings: Some(ValueEncoding {
                            len_encoding: LenEncoding::default(),
                            coin_encoding,
                            multiasset_encoding: LenEncoding::default(),
                            multiasset_key_encodings: BTreeMap::default(),
                            multiasset_value_encodings: BTreeMap::default(),
                            use_multiasset_format: false,
                        }),
                    })
                }
                // general format
                cbor_event::Type::Array => {
                    let len = raw.array_sz()?;
                    let len_encoding: LenEncoding = len.into();
                    let mut read_len = CBORReadLen::new(len);
                    read_len.read_elems(2)?;
                    let (coin, coin_encoding) = raw
                        .unsigned_integer_sz()
                        .map(|(x, enc)| (x, Some(enc)))
                        .map_err(Into::<DeserializeError>::into)
                        .map_err(|e: DeserializeError| e.annotate("coin"))?;
                    let (
                        multiasset,
                        multiasset_encoding,
                        multiasset_key_encodings,
                        multiasset_value_encodings,
                    ) = (|| -> Result<_, DeserializeError> {
                        let mut multiasset_table = OrderedHashMap::new();
                        let multiasset_len = raw.map_sz()?;
                        let multiasset_encoding = multiasset_len.into();
                        let mut multiasset_key_encodings = BTreeMap::new();
                        let mut multiasset_value_encodings = BTreeMap::new();
                        while match multiasset_len {
                            cbor_event::LenSz::Len(n, _) => (multiasset_table.len() as u64) < n,
                            cbor_event::LenSz::Indefinite => true,
                        } {
                            if raw.cbor_type()? == cbor_event::Type::Special {
                                assert_eq!(raw.special()?, cbor_event::Special::Break);
                                break;
                            }
                            let (multiasset_key, multiasset_key_encoding) = raw
                                .bytes_sz()
                                .map_err(Into::<DeserializeError>::into)
                                .and_then(|(bytes, enc)| {
                                    ScriptHash::from_raw_bytes(&bytes)
                                        .map(|bytes| (bytes, StringEncoding::from(enc)))
                                        .map_err(|e| {
                                            DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                        })
                                })?;
                            let mut multiasset_value_table = OrderedHashMap::new();
                            let multiasset_value_len = raw.map_sz()?;
                            let multiasset_value_encoding = multiasset_value_len.into();
                            let mut multiasset_value_value_encodings = BTreeMap::new();
                            while match multiasset_value_len {
                                cbor_event::LenSz::Len(n, _) => {
                                    (multiasset_value_table.len() as u64) < n
                                }
                                cbor_event::LenSz::Indefinite => true,
                            } {
                                if raw.cbor_type()? == cbor_event::Type::Special {
                                    assert_eq!(raw.special()?, cbor_event::Special::Break);
                                    break;
                                }
                                let multiasset_value_key = AssetName::deserialize(raw)?;
                                let (multiasset_value_value, multiasset_value_value_encoding) =
                                    raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
                                if multiasset_value_table
                                    .insert(multiasset_value_key.clone(), multiasset_value_value)
                                    .is_some()
                                {
                                    return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                        String::from("some complicated/unsupported type"),
                                    ))
                                    .into());
                                }
                                multiasset_value_value_encodings
                                    .insert(multiasset_value_key, multiasset_value_value_encoding);
                            }
                            let (
                                multiasset_value,
                                multiasset_value_encoding,
                                multiasset_value_value_encodings,
                            ) = (
                                multiasset_value_table,
                                multiasset_value_encoding,
                                multiasset_value_value_encodings,
                            );
                            if multiasset_table
                                .insert(multiasset_key, multiasset_value)
                                .is_some()
                            {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    String::from("some complicated/unsupported type"),
                                ))
                                .into());
                            }
                            multiasset_key_encodings
                                .insert(multiasset_key, multiasset_key_encoding);
                            multiasset_value_encodings.insert(
                                multiasset_key,
                                (multiasset_value_encoding, multiasset_value_value_encodings),
                            );
                        }
                        Ok((
                            multiasset_table,
                            multiasset_encoding,
                            multiasset_key_encodings,
                            multiasset_value_encodings,
                        ))
                    })()
                    .map_err(|e| e.annotate("multiasset"))?;
                    match len {
                        cbor_event::LenSz::Len(_, _) => (),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            cbor_event::Special::Break => (),
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    Ok(Value {
                        coin,
                        multiasset: multiasset.into(),
                        encodings: Some(ValueEncoding {
                            len_encoding,
                            coin_encoding,
                            multiasset_encoding,
                            multiasset_key_encodings,
                            multiasset_value_encodings,
                            use_multiasset_format: true,
                        }),
                    })
                }
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
        .map_err(|e| e.annotate("Value"))
    }
}

#[derive(Clone, Debug, Default)]
pub struct ValueEncoding {
    pub len_encoding: LenEncoding,
    pub coin_encoding: Option<cbor_event::Sz>,
    pub multiasset_encoding: LenEncoding,
    pub multiasset_key_encodings: BTreeMap<ScriptHash, StringEncoding>,
    pub multiasset_value_encodings:
        BTreeMap<ScriptHash, (LenEncoding, BTreeMap<AssetName, Option<cbor_event::Sz>>)>,
    // the fields above are directly code-generated but we need to keep track of which variant
    // we created this from since you can have an empty multiasset map but still use the MA format
    pub use_multiasset_format: bool,
}
