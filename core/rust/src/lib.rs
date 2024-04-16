// This recently introduced lint does not play well with the derivative crate.
// We have both Ord and PartialOrd derive automatically by derivative's proc macros
// but clippy sees these as hand implementations.
// Putting this allow locally where it's found did not seem to supress it,
// likely due to the structure of how the proc macro derives the code.
// Doing what is suggested by this lint would just result in us actually doing
// hand implementations of the PartialOrd (an maybe PartialEq) when there's no need,
// possibly impacting PartialOrd performance on top of being unnecessary and occuring in generated code.
// Possibly the derivative crate could get updated to suppress this lint
// from within their proc macros itself. Issue: https://github.com/mcarton/rust-derivative/issues/115
#![allow(clippy::non_canonical_partial_ord_impl)]

pub use error::*;

pub mod error;
pub mod network;
pub mod ordered_hash_map;
pub mod serialization;

use crate::serialization::{fit_sz, Deserialize, Serialize};

extern crate derivative;
use derivative::Derivative;

use cbor_event::{de::Deserializer, se::Serializer};
use std::io::{BufRead, Seek, Write};

pub type Epoch = u64;

pub type Slot = u64;

pub type TransactionIndex = u16;

pub type CertificateIndex = u64;

#[derive(Clone, Debug, Derivative)]
#[derivative(
    Eq,
    PartialEq,
    Ord = "feature_allow_slow_enum",
    PartialOrd = "feature_allow_slow_enum",
    Hash
)]
pub enum Int {
    Uint {
        value: u64,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        encoding: Option<cbor_event::Sz>,
    },
    Nint {
        value: u64,
        #[derivative(
            PartialEq = "ignore",
            Ord = "ignore",
            PartialOrd = "ignore",
            Hash = "ignore"
        )]
        encoding: Option<cbor_event::Sz>,
    },
}

#[derive(Clone, Debug)]
pub enum IntError {
    Bounds(std::num::TryFromIntError),
    Parsing(std::num::ParseIntError),
}

impl Int {
    pub fn new_uint(value: u64) -> Self {
        Self::Uint {
            value,
            encoding: None,
        }
    }

    /// * `value` - Value as encoded in CBOR - note: a negative `x` here would be `|x + 1|` due to CBOR's `nint` encoding e.g. to represent -5, pass in 4.
    pub fn new_nint(value: u64) -> Self {
        Self::Nint {
            value,
            encoding: None,
        }
    }

    pub fn encoding(&self) -> &Option<cbor_event::Sz> {
        match self {
            Self::Uint { encoding, .. } => encoding,
            Self::Nint { encoding, .. } => encoding,
        }
    }
}

impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uint { value, .. } => write!(f, "{value}"),
            // need to cast to avoid potential overflow when value == u64::max
            Self::Nint { value, .. } => write!(f, "-{}", (*value as i128) + 1),
        }
    }
}

impl std::str::FromStr for Int {
    type Err = IntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::convert::TryFrom;
        let x = i128::from_str(s).map_err(IntError::Parsing)?;
        Self::try_from(x).map_err(IntError::Bounds)
    }
}

// serde has no proper support for the full spectrum
// of int values - notably i64 doesn't cover the lower half of negatives
// and i128 support is not fully supported by serde
impl serde::Serialize for Int {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for Int {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use std::str::FromStr;
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"invalid int (as string)",
            )
        })
    }
}

impl schemars::JsonSchema for Int {
    fn schema_name() -> String {
        String::from("Int")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

impl From<u64> for Int {
    fn from(x: u64) -> Self {
        Self::Uint {
            value: x,
            encoding: None,
        }
    }
}

impl From<i64> for Int {
    fn from(x: i64) -> Self {
        if x >= 0 {
            Self::Uint {
                value: x as u64,
                encoding: None,
            }
        } else {
            Self::Nint {
                value: (x + 1).unsigned_abs(),
                encoding: None,
            }
        }
    }
}

impl std::convert::TryFrom<i128> for Int {
    type Error = std::num::TryFromIntError;

    fn try_from(x: i128) -> Result<Self, Self::Error> {
        if x >= 0 {
            u64::try_from(x).map(|x| Self::Uint {
                value: x,
                encoding: None,
            })
        } else {
            u64::try_from((x + 1).abs()).map(|x| Self::Nint {
                value: x,
                encoding: None,
            })
        }
    }
}

impl From<&Int> for i128 {
    fn from(val: &Int) -> Self {
        match val {
            Int::Uint { value, .. } => (*value).into(),
            Int::Nint { value, .. } => -((*value as i128) + 1),
        }
    }
}

impl Serialize for Int {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Self::Uint { value, encoding } => serializer
                .write_unsigned_integer_sz(*value, fit_sz(*value, *encoding, force_canonical)),
            Self::Nint { value, encoding } => serializer.write_negative_integer_sz(
                -((*value as i128) + 1),
                fit_sz(*value, *encoding, force_canonical),
            ),
        }
    }
}

impl Deserialize for Int {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::UnsignedInteger => raw
                    .unsigned_integer_sz()
                    .map(|(x, enc)| Self::Uint {
                        value: x,
                        encoding: Some(enc),
                    })
                    .map_err(std::convert::Into::into),
                cbor_event::Type::NegativeInteger => raw
                    .negative_integer_sz()
                    .map(|(x, enc)| Self::Nint {
                        value: (-1 - x) as u64,
                        encoding: Some(enc),
                    })
                    .map_err(std::convert::Into::into),
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
        .map_err(|e| e.annotate("Int"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_uint_min() {
        let bytes = [0x00];
        let x = Int::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(Into::<i128>::into(&x), u64::MIN as i128);
        assert_eq!(x.to_string(), "0");
    }

    #[test]
    fn int_uint_max() {
        let bytes = [0x1B, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let x = Int::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(Into::<i128>::into(&x), u64::MAX as i128);
        assert_eq!(x.to_string(), "18446744073709551615");
    }

    #[test]
    fn int_nint_min() {
        let bytes = [0x3B, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let x = Int::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(Into::<i128>::into(&x), -((u64::MAX as i128) + 1));
        assert_eq!(x.to_string(), "-18446744073709551616");
    }

    #[test]
    fn int_nint_max() {
        let bytes = [0x20];
        let x = Int::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(Into::<i128>::into(&x), -1i128);
        assert_eq!(x.to_string(), "-1");
        let y = Int::from(-1i64);
        assert_eq!(x.to_canonical_cbor_bytes(), y.to_canonical_cbor_bytes());
    }
}
