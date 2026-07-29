extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
#[derive(Debug)]
pub enum Key {
    Str(String),
    Uint(u64),
    /// A negative integer key or fixed value. Separate from `Uint` (rather than widening it) so a
    /// mismatch names the value the CDDL AUTHORED (`-7`), not the CBOR nint wire representation
    /// (`-1-N`, i.e. `6`) that a u64-only `Key` would have forced the message to print.
    Nint(i128),
    Float(f64),
    Bool(bool),
}

impl core::fmt::Display for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Key::Str(x) => write!(f, "\"{}\"", x),
            Key::Uint(x) => write!(f, "{}", x),
            Key::Nint(x) => write!(f, "{}", x),
            Key::Float(x) => write!(f, "{}", x),
            Key::Bool(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug)]
pub enum DeserializeFailure {
    // cddl-codegen:insert-start
    BadAddressType(u8),
    OutOfRange {
        min: usize,
        max: usize,
        found: usize,
    },
    VariableLenNatDecodeFailed,
    // cddl-codegen:insert-end
    BreakInDefiniteLen,
    CBOR(cbor_event::Error),
    DefiniteLenMismatch(u64, Option<u64>),
    DuplicateKey(Key),
    EndingBreakMissing,
    ExpectedNull,
    FixedValueMismatch {
        found: Key,
        expected: Key,
    },
    /// The recursion depth guard (opt-in via `--deserialize-depth-limit`) rejected input nested
    /// deeper than the configured limit, instead of recursing further and overflowing the stack.
    DepthLimitExceeded {
        limit: usize,
    },
    /// Invalid internal structure imposed on top of the CBOR format
    InvalidStructure(Box<dyn core::error::Error>),
    MandatoryFieldMissing(Key),
    NoVariantMatched,
    NoVariantMatchedWithCauses(Vec<DeserializeError>),
    RangeCheck {
        found: i128,
        min: Option<i128>,
        max: Option<i128>,
    },
    /// A float value fell outside its CDDL window (`float64 .le 10.5`, `0.5..10.5`). Separate from
    /// `RangeCheck` because float bounds are f64-typed and carry per-side exclusivity (dense float
    /// space has no ±1 collapse). NaN always lands here — the emitted check is accept-form
    /// (`!(x >= min && x <= max)`), and every comparison against NaN is false.
    RangeCheckFloat {
        found: f64,
        min: Option<f64>,
        max: Option<f64>,
        min_inclusive: bool,
        max_inclusive: bool,
    },
    TagMismatch {
        found: u64,
        expected: u64,
    },
    UnknownKey(Key),
    UnexpectedKeyType(cbor_event::Type),
}

// we might want to add more info like which field,
#[derive(Debug)]
pub struct DeserializeError {
    location: Option<String>,
    failure: DeserializeFailure,
}

impl DeserializeError {
    pub fn new<T: Into<String>>(location: T, failure: DeserializeFailure) -> Self {
        Self {
            location: Some(location.into()),
            failure,
        }
    }

    /// The underlying failure reason. Lets callers (e.g. generated reject tests) assert the
    /// exact `DeserializeFailure` variant rather than just that deserialization errored.
    pub fn failure(&self) -> &DeserializeFailure {
        &self.failure
    }

    pub fn annotate<T: Into<String>>(self, location: T) -> Self {
        match self.location {
            Some(loc) => Self::new(format!("{}.{}", location.into(), loc), self.failure),
            None => Self::new(location, self.failure),
        }
    }

    fn fmt_indent(&self, f: &mut core::fmt::Formatter<'_>, indent: u32) -> core::fmt::Result {
        use core::fmt::Display;
        for _ in 0..indent {
            write!(f, "\t")?;
        }
        match &self.location {
            Some(loc) => write!(f, "Deserialization failed in {} because: ", loc),
            None => write!(f, "Deserialization: "),
        }?;
        match &self.failure {
            // cddl-codegen:insert-start
            DeserializeFailure::BadAddressType(header) => {
                write!(f, "Encountered unknown address header {header:#08b}")
            }
            DeserializeFailure::OutOfRange { min, max, found } => {
                write!(f, "Out of range: {found} - must be in range {min} - {max}")
            }
            DeserializeFailure::VariableLenNatDecodeFailed => {
                write!(f, "Variable length natural number decode failed")
            }
            // cddl-codegen:insert-end
            DeserializeFailure::BreakInDefiniteLen => write!(
                f,
                "Encountered CBOR Break while reading definite length sequence"
            ),
            DeserializeFailure::CBOR(e) => e.fmt(f),
            DeserializeFailure::DefiniteLenMismatch(found, expected) => {
                write!(f, "Definite length mismatch: found {}", found)?;
                if let Some(expected_elems) = expected {
                    write!(f, ", expected: {}", expected_elems)?;
                }
                Ok(())
            }
            DeserializeFailure::DuplicateKey(key) => write!(f, "Duplicate key: {}", key),
            DeserializeFailure::EndingBreakMissing => write!(f, "Missing ending CBOR Break"),
            DeserializeFailure::ExpectedNull => write!(f, "Expected null, found other type"),
            DeserializeFailure::FixedValueMismatch { found, expected } => {
                write!(f, "Expected fixed value {} found {}", expected, found)
            }
            DeserializeFailure::DepthLimitExceeded { limit } => write!(
                f,
                "Deserialization recursion depth exceeded the configured limit of {}",
                limit
            ),
            DeserializeFailure::InvalidStructure(e) => {
                write!(f, "Invalid internal structure: {}", e)
            }
            DeserializeFailure::MandatoryFieldMissing(key) => {
                write!(f, "Mandatory field {} not found", key)
            }
            DeserializeFailure::NoVariantMatched => write!(f, "No variant matched"),
            DeserializeFailure::NoVariantMatchedWithCauses(errs) => {
                writeln!(f, "No variant matched. Failures:")?;
                for e in errs {
                    e.fmt_indent(f, indent + 1)?;
                    writeln!(f)?;
                }
                Ok(())
            }
            DeserializeFailure::RangeCheck { found, min, max } => match (min, max) {
                (Some(min), Some(max)) => write!(f, "{} not in range {} - {}", found, min, max),
                (Some(min), None) => write!(f, "{} not at least {}", found, min),
                (None, Some(max)) => write!(f, "{} not at most {}", found, max),
                (None, None) => write!(f, "invalid range (no min nor max specified)"),
            },
            DeserializeFailure::RangeCheckFloat {
                found,
                min,
                max,
                min_inclusive,
                max_inclusive,
            } => {
                let lo = match min {
                    Some(min) => format!("{}{}", if *min_inclusive { ">=" } else { ">" }, min),
                    None => "-inf".to_owned(),
                };
                let hi = match max {
                    Some(max) => format!("{}{}", if *max_inclusive { "<=" } else { "<" }, max),
                    None => "+inf".to_owned(),
                };
                write!(f, "{} not in float range ({}, {})", found, lo, hi)
            }
            DeserializeFailure::TagMismatch { found, expected } => {
                write!(f, "Expected tag {}, found {}", expected, found)
            }
            DeserializeFailure::UnknownKey(key) => write!(f, "Found unexpected key {}", key),
            DeserializeFailure::UnexpectedKeyType(ty) => {
                write!(f, "Found unexpected key of CBOR type {:?}", ty)
            }
        }
    }
}

impl core::error::Error for DeserializeError {}

impl core::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.fmt_indent(f, 0)
    }
}

impl From<DeserializeFailure> for DeserializeError {
    fn from(failure: DeserializeFailure) -> DeserializeError {
        DeserializeError {
            location: None,
            failure,
        }
    }
}

// cddl-codegen:insert-start
#[derive(Debug, thiserror::Error)]
pub enum ArithmeticError {
    #[error("Integer overflow")]
    IntegerOverflow,
    #[error("Integer underflow")]
    IntegerUnderflow,
}
// cddl-codegen:insert-end
impl From<cbor_event::Error> for DeserializeError {
    fn from(err: cbor_event::Error) -> DeserializeError {
        DeserializeError {
            location: None,
            failure: DeserializeFailure::CBOR(err),
        }
    }
}
