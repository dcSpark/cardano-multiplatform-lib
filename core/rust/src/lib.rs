pub use error::*;
pub mod error;

pub mod serialization;

pub mod metadata;

pub mod ordered_hash_map;

use crate::serialization::{fit_sz, Deserialize, Serialize};

extern crate derivative;
use derivative::Derivative;

use cbor_event::{de::Deserializer, se::Serializer};
use std::io::{BufRead, Seek, Write};

pub type Epoch = u64;

pub type Slot = u64;

pub type TransactionIndex = u16;

pub type CertificateIndex = u64;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Derivative)]
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
        #[serde(skip)]
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
        #[serde(skip)]
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
            Self::Uint { value, .. } => write!(f, "{}", value),
            Self::Nint { value, .. } => write!(f, "-{}", value + 1),
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

impl From<u64> for Int {
    fn from(x: u64) -> Self {
        Self::Uint { value: x, encoding: None }
    }
}

impl From<i64> for Int {
    fn from(x: i64) -> Self {
        if x >= 0 {
            Self::Uint { value: x as u64, encoding: None }
        } else {
            Self::Nint { value: (x + 1).abs() as u64, encoding: None }
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

impl Into<i128> for &Int {
    fn into(self) -> i128 {
        match self {
            Int::Uint { value, .. } => (*value).into(),
            Int::Nint { value, .. } => -((*value + 1) as i128),
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
