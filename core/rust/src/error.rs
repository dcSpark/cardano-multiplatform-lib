#[derive(Debug)]
pub enum Key {
    Str(String),
    Uint(u64),
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Str(x) => write!(f, "\"{x}\""),
            Key::Uint(x) => write!(f, "{x}"),
        }
    }
}

#[derive(Debug)]
pub enum DeserializeFailure {
    BadAddressType(u8),
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
    /// Invalid internal structure imposed on top of the CBOR format
    InvalidStructure(Box<dyn std::error::Error>),
    MandatoryFieldMissing(Key),
    NoVariantMatched,
    NoVariantMatchedWithCauses(Vec<DeserializeError>),
    OutOfRange {
        min: usize,
        max: usize,
        found: usize,
    },
    RangeCheck {
        found: isize,
        min: Option<isize>,
        max: Option<isize>,
    },
    TagMismatch {
        found: u64,
        expected: u64,
    },
    UnknownKey(Key),
    UnexpectedKeyType(cbor_event::Type),
    VariableLenNatDecodeFailed,
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

    pub fn annotate<T: Into<String>>(self, location: T) -> Self {
        match self.location {
            Some(loc) => Self::new(format!("{}.{}", location.into(), loc), self.failure),
            None => Self::new(location, self.failure),
        }
    }

    fn fmt_indent(&self, f: &mut std::fmt::Formatter<'_>, indent: u32) -> std::fmt::Result {
        use std::fmt::Display;
        for _ in 0..indent {
            write!(f, "\t")?;
        }
        match &self.location {
            Some(loc) => write!(f, "Deserialization failed in {loc} because: "),
            None => write!(f, "Deserialization: "),
        }?;
        match &self.failure {
            DeserializeFailure::BadAddressType(header) => {
                write!(f, "Encountered unknown address header {header:#08b}")
            }
            DeserializeFailure::BreakInDefiniteLen => write!(
                f,
                "Encountered CBOR Break while reading definite length sequence"
            ),
            DeserializeFailure::CBOR(e) => e.fmt(f),
            DeserializeFailure::DefiniteLenMismatch(found, expected) => {
                write!(f, "Definite length mismatch: found {found}")?;
                if let Some(expected_elems) = expected {
                    write!(f, ", expected: {expected_elems}")?;
                }
                Ok(())
            }
            DeserializeFailure::DuplicateKey(key) => write!(f, "Duplicate key: {key}"),
            DeserializeFailure::EndingBreakMissing => write!(f, "Missing ending CBOR Break"),
            DeserializeFailure::ExpectedNull => write!(f, "Expected null, found other type"),
            DeserializeFailure::FixedValueMismatch { found, expected } => {
                write!(f, "Expected fixed value {expected} found {found}")
            }
            DeserializeFailure::InvalidStructure(e) => {
                write!(f, "Invalid internal structure: {e}")
            }
            DeserializeFailure::MandatoryFieldMissing(key) => {
                write!(f, "Mandatory field {key} not found")
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
            DeserializeFailure::OutOfRange { min, max, found } => {
                write!(f, "Out of range: {found} - must be in range {min} - {max}")
            }
            DeserializeFailure::RangeCheck { found, min, max } => match (min, max) {
                (Some(min), Some(max)) => write!(f, "{found} not in range {min} - {max}"),
                (Some(min), None) => write!(f, "{found} not at least {min}"),
                (None, Some(max)) => write!(f, "{found} not at most {max}"),
                (None, None) => write!(f, "invalid range (no min nor max specified)"),
            },
            DeserializeFailure::TagMismatch { found, expected } => {
                write!(f, "Expected tag {expected}, found {found}")
            }
            DeserializeFailure::UnknownKey(key) => write!(f, "Found unexpected key {key}"),
            DeserializeFailure::UnexpectedKeyType(ty) => {
                write!(f, "Found unexpected key of CBOR type {ty:?}")
            }
            DeserializeFailure::VariableLenNatDecodeFailed => {
                write!(f, "Variable length natural number decode failed")
            }
        }
    }
}

impl std::error::Error for DeserializeError {}

impl std::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl From<cbor_event::Error> for DeserializeError {
    fn from(err: cbor_event::Error) -> DeserializeError {
        DeserializeError {
            location: None,
            failure: DeserializeFailure::CBOR(err),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ArithmeticError {
    #[error("Integer overflow")]
    IntegerOverflow,
    #[error("Integer underflow")]
    IntegerUnderflow,
}
