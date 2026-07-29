use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use bech32::Hrp;
use core::error::Error as StdError;
use core::fmt;
use core::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

pub trait Bech32 {
    const BECH32_HRP: &'static str;

    fn try_from_bech32_str(bech32_str: &str) -> Result<Self>
    where
        Self: Sized;

    fn to_bech32_str(&self) -> String;
}

pub fn to_bech32_from_bytes<B: Bech32>(bytes: &[u8]) -> String {
    let hrp = Hrp::parse(B::BECH32_HRP).expect("statically-known HRP is valid");
    // Cardano uses the original Bech32 checksum (not Bech32m).
    bech32::encode::<bech32::Bech32>(hrp, bytes)
        .unwrap_or_else(|e| panic!("Failed to build bech32: {}", e))
}

pub fn try_from_bech32_to_bytes<B: Bech32>(bech32_str: &str) -> Result<Vec<u8>> {
    let (hrp, data) =
        bech32::decode(bech32_str).map_err(|e| Error::Bech32Malformed(e.to_string()))?;
    if hrp.as_str() != B::BECH32_HRP {
        return Err(Error::HrpInvalid {
            expected: B::BECH32_HRP,
            actual: hrp.as_str().to_owned(),
        });
    }
    Ok(data)
}

#[derive(Debug)]
pub enum Error {
    Bech32Malformed(String),
    HrpInvalid {
        expected: &'static str,
        actual: String,
    },
    DataInvalid(Box<dyn StdError + Send + Sync + 'static>),
}

impl Error {
    pub fn data_invalid(cause: impl StdError + Send + Sync + 'static) -> Self {
        Error::DataInvalid(Box::new(cause))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> StdResult<(), fmt::Error> {
        match self {
            Error::Bech32Malformed(cause) => {
                write!(f, "Failed to parse bech32, invalid data format: {cause}")
            }
            Error::HrpInvalid { expected, actual } => write!(
                f,
                "Parsed bech32 has invalid HRP prefix '{actual}', expected '{expected}'"
            ),
            Error::DataInvalid(_) => write!(f, "Failed to parse data decoded from bech32"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::DataInvalid(cause) => Some(&**cause),
            _ => None,
        }
    }
}
