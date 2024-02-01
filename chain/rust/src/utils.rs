use cbor_event::{de::Deserializer, se::Serializer};
use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    serialization::{fit_sz, sz_max, Deserialize, Serialize},
    Int,
};
use cml_crypto::ScriptHash;
use derivative::Derivative;
use std::io::{BufRead, Seek, Write};

use crate::{
    crypto::hash::{hash_script, ScriptHashNamespace},
    plutus::{Language, PlutusScript, PlutusV1Script, PlutusV2Script},
    NativeScript, Script,
};

impl Script {
    pub fn hash(&self) -> ScriptHash {
        match self {
            Self::Native { script, .. } => script.hash(),
            Self::PlutusV1 { script, .. } => script.hash(),
            Self::PlutusV2 { script, .. } => script.hash(),
            Self::PlutusV3 { script, .. } => script.hash(),
        }
    }

    // Returns which language the script is if it's a Plutus script
    // Returns None otherwise (i.e. NativeScript)
    pub fn language(&self) -> Option<Language> {
        match self {
            Self::Native { .. } => None,
            Self::PlutusV1 { .. } => Some(Language::PlutusV1),
            Self::PlutusV2 { .. } => Some(Language::PlutusV2),
            Self::PlutusV3 { .. } => Some(Language::PlutusV3),
        }
    }
}

impl NativeScript {
    pub fn hash(&self) -> ScriptHash {
        hash_script(ScriptHashNamespace::NativeScript, &self.to_cbor_bytes())
    }
}

impl From<NativeScript> for Script {
    fn from(script: NativeScript) -> Self {
        Self::new_native(script)
    }
}

impl From<PlutusV1Script> for Script {
    fn from(script: PlutusV1Script) -> Self {
        Self::new_plutus_v1(script)
    }
}

impl From<PlutusV2Script> for Script {
    fn from(script: PlutusV2Script) -> Self {
        Self::new_plutus_v2(script)
    }
}

impl From<PlutusScript> for Script {
    fn from(script: PlutusScript) -> Self {
        match script {
            PlutusScript::PlutusV1(v1) => Self::new_plutus_v1(v1),
            PlutusScript::PlutusV2(v2) => Self::new_plutus_v2(v2),
            PlutusScript::PlutusV3(v3) => Self::new_plutus_v3(v3),
        }
    }
}

const BOUNDED_BYTES_CHUNK_SIZE: usize = 64;

// to get around not having access from outside the library we just write the raw CBOR indefinite byte string code here
fn write_cbor_indefinite_byte_tag<W: Write>(
    serializer: &mut Serializer<W>,
) -> cbor_event::Result<&mut Serializer<W>> {
    serializer.write_raw_bytes(&[0x5f])
}

use cml_core::serialization::StringEncoding;

fn valid_indefinite_string_encoding(chunks: &[(u64, cbor_event::Sz)], total_len: usize) -> bool {
    let mut len_counter = 0;
    let valid_sz = chunks.iter().all(|(len, sz)| {
        len_counter += len;
        *len <= sz_max(*sz)
    });
    valid_sz && len_counter == total_len as u64
}

/// Write bounded bytes according to Cardano's special format:
/// bounded_bytes = bytes .size (0..64)
///  ; the real bounded_bytes does not have this limit. it instead has a different
///   ; limit which cannot be expressed in CDDL.
///   ; The limit is as follows:
///   ;  - bytes with a definite-length encoding are limited to size 0..64
///   ;  - for bytes with an indefinite-length CBOR encoding, each chunk is
///   ;    limited to size 0..64
///   ;  ( reminder: in CBOR, the indefinite-length encoding of bytestrings
///   ;    consists of a token #2.31 followed by a sequence of definite-length
///   ;    encoded bytestrings and a stop code )
pub fn write_bounded_bytes<'se, W: Write>(
    serializer: &'se mut Serializer<W>,
    bytes: &[u8],
    enc: &StringEncoding,
    force_canonical: bool,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    match enc {
        StringEncoding::Definite(sz) if !force_canonical => {
            if bytes.len() <= BOUNDED_BYTES_CHUNK_SIZE {
                let fit_sz = fit_sz(bytes.len() as u64, Some(*sz), force_canonical);
                return serializer.write_bytes_sz(bytes, cbor_event::StringLenSz::Len(fit_sz));
            }
        }
        StringEncoding::Indefinite(chunks) if !force_canonical => {
            if valid_indefinite_string_encoding(chunks, bytes.len()) {
                write_cbor_indefinite_byte_tag(serializer)?;
                let mut start = 0;
                for (len, sz) in chunks {
                    let end = start + *len as usize;
                    serializer
                        .write_bytes_sz(&bytes[start..end], cbor_event::StringLenSz::Len(*sz))?;
                    start = end;
                }
                return serializer.write_special(cbor_event::Special::Break);
            }
        }
        _ =>
            /* handled below */
            {}
    };
    // This is a fallback for when either it's canonical or the passed in encoding isn't
    // compatible with the passed in bytes (e.g. someone deserialized then modified the bytes)
    // If we truly need to encode canonical CBOR there's really no way to abide by both canonical
    // CBOR as well as following the Cardano format. So this is the best attempt at it while keeping
    // chunks when len > 64
    if bytes.len() <= BOUNDED_BYTES_CHUNK_SIZE {
        serializer.write_bytes(bytes)
    } else {
        write_cbor_indefinite_byte_tag(serializer)?;
        for chunk in bytes.chunks(BOUNDED_BYTES_CHUNK_SIZE) {
            serializer.write_bytes(chunk)?;
        }
        serializer.write_special(cbor_event::Special::Break)
    }
}

/// Read bounded bytes according to Cardano's special format:
/// bounded_bytes = bytes .size (0..64)
///  ; the real bounded_bytes does not have this limit. it instead has a different
///  ; limit which cannot be expressed in CDDL.
///  ; The limit is as follows:
///  ;  - bytes with a definite-length encoding are limited to size 0..64
///  ;  - for bytes with an indefinite-length CBOR encoding, each chunk is
///  ;    limited to size 0..64
///  ;  ( reminder: in CBOR, the indefinite-length encoding of bytestrings
///  ;    consists of a token #2.31 followed by a sequence of definite-length
///  ;    encoded bytestrings and a stop code )
pub fn read_bounded_bytes<R: BufRead + Seek>(
    raw: &mut Deserializer<R>,
) -> Result<(Vec<u8>, StringEncoding), DeserializeError> {
    let (bytes, bytes_enc) = raw.bytes_sz()?;
    match &bytes_enc {
        cbor_event::StringLenSz::Len(_sz) => {
            if bytes.len() > BOUNDED_BYTES_CHUNK_SIZE {
                return Err(DeserializeFailure::OutOfRange {
                    min: 0,
                    max: BOUNDED_BYTES_CHUNK_SIZE,
                    found: bytes.len(),
                }
                .into());
            }
        }
        cbor_event::StringLenSz::Indefinite(chunks) => {
            for (chunk_len, _chunk_len_sz) in chunks.iter() {
                if *chunk_len as usize > BOUNDED_BYTES_CHUNK_SIZE {
                    return Err(DeserializeFailure::OutOfRange {
                        min: 0,
                        max: BOUNDED_BYTES_CHUNK_SIZE,
                        found: *chunk_len as usize,
                    }
                    .into());
                }
            }
        }
    }
    Ok((bytes, bytes_enc.into()))
}

#[derive(Clone, Debug)]
enum BigIntEncoding {
    Int(cbor_event::Sz),
    Bytes(StringEncoding),
}

#[derive(Clone, Debug, Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BigInteger {
    num: num_bigint::BigInt,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    encoding: Option<BigIntEncoding>,
}

impl serde::Serialize for BigInteger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for BigInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use std::str::FromStr;
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        BigInteger::from_str(&s).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"string rep of a big int",
            )
        })
    }
}

impl schemars::JsonSchema for BigInteger {
    fn schema_name() -> String {
        String::from("BigInteger")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

impl std::fmt::Display for BigInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.num.fmt(f)
    }
}

impl std::str::FromStr for BigInteger {
    type Err = num_bigint::ParseBigIntError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        num_bigint::BigInt::from_str(string).map(|num| Self {
            num,
            encoding: None,
        })
    }
}

impl BigInteger {
    // can't be a trait due to being in other crate
    pub fn from_int(x: &Int) -> Self {
        Self {
            num: Into::<i128>::into(x).into(),
            encoding: x.encoding().map(BigIntEncoding::Int),
        }
    }

    /// Converts to a u64
    /// Returns None if the number was negative or too big for a u64
    pub fn as_u64(&self) -> Option<u64> {
        let (sign, u64_digits) = self.num.to_u64_digits();
        if sign == num_bigint::Sign::Minus {
            return None;
        }
        match u64_digits.len() {
            0 => Some(0),
            1 => Some(*u64_digits.first().unwrap()),
            _ => None,
        }
    }

    /// Converts to a u128
    /// Returns None if the number was negative or too big for a u128
    pub fn as_u128(&self) -> Option<u128> {
        let (sign, u32_digits) = self.num.to_u32_digits();
        if sign == num_bigint::Sign::Minus {
            return None;
        }
        match *u32_digits {
            [] => Some(0),
            [a] => Some(u128::from(a)),
            [a, b] => Some(u128::from(b) | (u128::from(a) << 32)),
            [a, b, c] => Some(u128::from(c) | (u128::from(b) << 32) | (u128::from(a) << 64)),
            [a, b, c, d] => Some(
                u128::from(d)
                    | (u128::from(c) << 32)
                    | (u128::from(b) << 64)
                    | (u128::from(a) << 96),
            ),
            _ => None,
        }
    }

    /// Converts to an Int
    /// Returns None when the number is too big for an Int (outside +/- 64-bit unsigned)
    /// Retains encoding info if the original was encoded as an Int
    pub fn as_int(&self) -> Option<Int> {
        let (sign, u64_digits) = self.num.to_u64_digits();
        // unsigned raw value that can fit in the up to 8 bytes of a CBOR uint or nint
        // negative values evaluate to -u64_value - 1
        let u64_value = match u64_digits.len() {
            0 => 0u64,
            1 => {
                if sign == num_bigint::Sign::Minus {
                    (*u64_digits.first().unwrap())
                        .checked_sub(1)
                        .expect("negative (non-zero) so can't underflow")
                } else {
                    *u64_digits.first().unwrap()
                }
            }
            // this could actually be -u64::MAX which in CBOR can be a single u64 as the sign
            // is encoded separately so values here start from -1 instead of 0.
            2 if sign == num_bigint::Sign::Minus && u64_digits[0] == 0 && u64_digits[1] == 1 => {
                u64::MAX
            }
            _ => return None,
        };
        let encoding = match &self.encoding {
            Some(BigIntEncoding::Int(sz)) => Some(*sz),
            _ => None,
        };
        match sign {
            num_bigint::Sign::NoSign | num_bigint::Sign::Plus => Some(Int::Uint {
                value: u64_value,
                encoding,
            }),
            num_bigint::Sign::Minus => Some(Int::Nint {
                value: u64_value,
                encoding,
            }),
        }
    }
}

impl Serialize for BigInteger {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        let write_self_as_bytes = |serializer: &'se mut Serializer<W>,
                                   enc: &StringEncoding|
         -> cbor_event::Result<&'se mut Serializer<W>> {
            let (sign, bytes) = self.num.to_bytes_be();
            match sign {
                // positive bigint
                num_bigint::Sign::Plus | num_bigint::Sign::NoSign => {
                    serializer.write_tag(2u64)?;
                    write_bounded_bytes(serializer, &bytes, enc, force_canonical)
                }
                // negative bigint
                num_bigint::Sign::Minus => {
                    serializer.write_tag(3u64)?;
                    use std::ops::Neg;
                    // CBOR RFC defines this as the bytes of -n -1
                    let adjusted = self
                        .num
                        .clone()
                        .neg()
                        .checked_sub(&num_bigint::BigInt::from(1u32))
                        .unwrap()
                        .to_biguint()
                        .unwrap();
                    write_bounded_bytes(serializer, &adjusted.to_bytes_be(), enc, force_canonical)
                }
            }
        };
        // use encoding if possible
        match &self.encoding {
            Some(BigIntEncoding::Int(_sz)) if !force_canonical => {
                // as_int() retains encoding info so we can direclty use Int::serialize()
                if let Some(int) = self.as_int() {
                    return int.serialize(serializer, force_canonical);
                }
            }
            Some(BigIntEncoding::Bytes(str_enc)) if !force_canonical => {
                let (_sign, bytes) = self.num.to_bytes_be();
                let valid_non_canonical = match str_enc {
                    StringEncoding::Canonical => false,
                    StringEncoding::Definite(sz) => bytes.len() <= sz_max(*sz) as usize,
                    StringEncoding::Indefinite(chunks) => {
                        valid_indefinite_string_encoding(chunks, bytes.len())
                    }
                };
                if valid_non_canonical {
                    return write_self_as_bytes(serializer, str_enc);
                }
            }
            _ =>
                /* always fallback to default */
                {}
        }
        // fallback for:
        // 1) canonical bytes needed
        // 2) no encoding specified (never deseiralized)
        // 3) deserialized but data changed and no longer compatible
        let (sign, u64_digits) = self.num.to_u64_digits();
        match u64_digits.len() {
            0 => serializer.write_unsigned_integer(0),
            // we use the uint/nint encodings to use a minimum of space
            1 => match sign {
                // uint
                num_bigint::Sign::Plus | num_bigint::Sign::NoSign => {
                    serializer.write_unsigned_integer(*u64_digits.first().unwrap())
                }
                // nint
                num_bigint::Sign::Minus => serializer
                    .write_negative_integer(-(*u64_digits.first().unwrap() as i128) as i64),
            },
            _ => {
                // Small edge case: nint's minimum is -18446744073709551616 but in this bigint lib
                // that takes 2 u64 bytes so we put that as a special case here:
                if sign == num_bigint::Sign::Minus && u64_digits == vec![0, 1] {
                    serializer.write_negative_integer(-18446744073709551616i128 as i64)
                } else {
                    write_self_as_bytes(serializer, &StringEncoding::Canonical)
                }
            }
        }
    }
}

impl Deserialize for BigInteger {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                // bigint
                cbor_event::Type::Tag => {
                    let tag = raw.tag()?;
                    let (bytes, bytes_enc) = read_bounded_bytes(raw)?;
                    match tag {
                        // positive bigint
                        2 => Ok(Self {
                            num: num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes),
                            encoding: Some(BigIntEncoding::Bytes(bytes_enc)),
                        }),
                        // negative bigint
                        3 => {
                            // CBOR RFC defines this as the bytes of -n -1
                            let initial =
                                num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes);
                            use std::ops::Neg;
                            let adjusted = initial
                                .checked_add(&num_bigint::BigInt::from(1u32))
                                .unwrap()
                                .neg();
                            Ok(Self {
                                num: adjusted,
                                encoding: Some(BigIntEncoding::Bytes(bytes_enc)),
                            })
                        }
                        _ => Err(DeserializeFailure::TagMismatch {
                            found: tag,
                            expected: 2,
                        }
                        .into()),
                    }
                }
                // uint
                cbor_event::Type::UnsignedInteger => {
                    let (num, num_enc) = raw.unsigned_integer_sz()?;
                    Ok(Self {
                        num: num_bigint::BigInt::from(num),
                        encoding: Some(BigIntEncoding::Int(num_enc)),
                    })
                }
                // nint
                cbor_event::Type::NegativeInteger => {
                    let (num, num_enc) = raw.negative_integer_sz()?;
                    Ok(Self {
                        num: num_bigint::BigInt::from(num),
                        encoding: Some(BigIntEncoding::Int(num_enc)),
                    })
                }
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
        .map_err(|e| e.annotate("BigInteger"))
    }
}

impl<T> std::convert::From<T> for BigInteger
where
    T: std::convert::Into<num_bigint::BigInt>,
{
    fn from(x: T) -> Self {
        Self {
            num: x.into(),
            encoding: None,
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct NetworkId {
    pub network: u64,
    #[serde(skip)]
    pub encoding: Option<cbor_event::Sz>,
}

impl NetworkId {
    pub fn new(network: u64) -> Self {
        Self {
            network,
            encoding: None,
        }
    }

    pub fn mainnet() -> Self {
        Self {
            network: 1,
            encoding: None,
        }
    }

    pub fn testnet() -> Self {
        Self {
            network: 0,
            encoding: None,
        }
    }
}

impl From<u64> for NetworkId {
    fn from(network: u64) -> Self {
        NetworkId::new(network)
    }
}

impl From<NetworkId> for u64 {
    fn from(id: NetworkId) -> u64 {
        id.network
    }
}

impl Serialize for NetworkId {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer_sz(
            self.network,
            fit_sz(self.network, self.encoding, force_canonical),
        )
    }
}

impl Deserialize for NetworkId {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (network, encoding) = raw.unsigned_integer_sz().map(|(x, enc)| (x, Some(enc)))?;
        Ok(Self { network, encoding })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn bigint_uint_u64_min() {
        let bytes = [0x00];
        let x = BigInteger::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(x.as_u64(), Some(u64::MIN));
        assert_eq!(x.as_int().unwrap().to_string(), x.to_string());
        assert_eq!(x.to_string(), "0");
    }

    #[test]
    fn bigint_uint_u64_max() {
        let bytes = [0x1B, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let x = BigInteger::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(x.as_u64(), Some(u64::MAX));
        assert_eq!(x.as_int().unwrap().to_string(), x.to_string());
        assert_eq!(x.to_string(), "18446744073709551615");
    }

    #[test]
    fn bigint_uint_u128_min() {
        let bytes = [0x00];
        let x = BigInteger::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(x.as_u128(), Some(u128::MIN));
        assert_eq!(x.to_string(), "0");
    }

    #[test]
    fn bigint_uint_u128_max() {
        let bytes = BigInteger::from_str(&u128::MAX.to_string())
            .unwrap()
            .to_cbor_bytes();
        let x = BigInteger::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(x.as_u128(), Some(u128::MAX));
        assert_eq!(x.to_string(), "340282366920938463463374607431768211455");
    }

    #[test]
    fn bigint_above_uint_min() {
        let bytes = [
            0xC2, 0x49, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let x = BigInteger::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(x.as_int(), None);
        assert_eq!(x.to_string(), "18446744073709551616");
    }

    #[test]
    fn bigint_nint_min() {
        let bytes = [0x3B, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let x = BigInteger::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(
            Into::<i128>::into(&x.as_int().unwrap()),
            -((u64::MAX as i128) + 1)
        );
        assert_eq!(x.as_int().unwrap().to_string(), x.to_string());
        assert_eq!(x.to_string(), "-18446744073709551616");
    }

    #[test]
    fn bigint_nint_max() {
        let bytes = [0x20];
        let x = BigInteger::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(x.as_u64(), None);
        assert_eq!(x.as_int().unwrap().to_string(), x.to_string());
        assert_eq!(x.to_string(), "-1");
    }

    #[test]
    fn bigint_below_nint_min() {
        let bytes = [
            0xC3, 0x49, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let x = BigInteger::from_cbor_bytes(&bytes).unwrap();
        assert_eq!(bytes, x.to_cbor_bytes().as_slice());
        assert_eq!(x.as_int(), None);
        assert_eq!(x.to_string(), "-18446744073709551617");
    }
}
