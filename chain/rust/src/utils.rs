use cbor_event::{de::Deserializer, se::Serializer};
use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    serialization::{fit_sz, sz_max, Deserialize, Serialize},
    Int,
};
use derivative::Derivative;
use std::io::{BufRead, Seek, Write};

const BOUNDED_BYTES_CHUNK_SIZE: usize = 64;

// to get around not having access from outside the library we just write the raw CBOR indefinite byte string code here
fn write_cbor_indefinite_byte_tag<'se, W: Write>(
    serializer: &'se mut Serializer<W>,
) -> cbor_event::Result<&'se mut Serializer<W>> {
    serializer.write_raw_bytes(&[0x5f])
}

use cml_core::serialization::StringEncoding;

fn valid_indefinite_string_encoding(chunks: &Vec<(u64, cbor_event::Sz)>, total_len: usize) -> bool {
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
        {
            ()
        }
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
pub struct BigInt {
    num: num_bigint::BigInt,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    encoding: Option<BigIntEncoding>,
}

impl serde::Serialize for BigInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for BigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use std::str::FromStr;
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        BigInt::from_str(&s).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&s),
                &"string rep of a big int",
            )
        })
    }
}

impl schemars::JsonSchema for BigInt {
    fn schema_name() -> String {
        String::from("BigInt")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.num.fmt(f)
    }
}

impl std::str::FromStr for BigInt {
    type Err = num_bigint::ParseBigIntError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        num_bigint::BigInt::from_str(string).map(|num| Self {
            num,
            encoding: None,
        })
    }
}

impl BigInt {
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

    /// Converts to an Int
    /// Returns None when the number is too big for an Int (outside +/- 64-bit unsigned)
    /// Retains encoding info if the original was encoded as an Int
    pub fn as_int(&self) -> Option<Int> {
        let (sign, u64_digits) = self.num.to_u64_digits();
        let u64_digit = match u64_digits.len() {
            0 => 0u64,
            1 => *u64_digits.first().unwrap(),
            _ => return None,
        };
        let encoding = match &self.encoding {
            Some(BigIntEncoding::Int(sz)) => Some(*sz),
            _ => None,
        };
        match sign {
            num_bigint::Sign::NoSign | num_bigint::Sign::Plus => Some(Int::Uint {
                value: u64_digit,
                encoding,
            }),
            num_bigint::Sign::Minus => Some(Int::Nint {
                value: u64_digit,
                encoding,
            }),
        }
    }
}

impl Serialize for BigInt {
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
            Some(BigIntEncoding::Int(sz)) if !force_canonical => {
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
                        valid_indefinite_string_encoding(&chunks, bytes.len())
                    }
                };
                if valid_non_canonical {
                    return write_self_as_bytes(serializer, &str_enc);
                }
            }
            _ =>
            /* always fallback to default */
            {
                ()
            }
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

impl Deserialize for BigInt {
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
        .map_err(|e| e.annotate("BigInt"))
    }
}

impl<T> std::convert::From<T> for BigInt
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
