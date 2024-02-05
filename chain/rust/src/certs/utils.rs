use std::{borrow::Cow, str::FromStr};

use super::{Ipv4, Ipv6, StakeCredential};
use cml_core::DeserializeError;
use cml_crypto::RawBytesEncoding;

impl StakeCredential {
    // we don't implement RawBytesEncoding as from_raw_bytes() would be unable to distinguish
    pub fn to_raw_bytes(&self) -> &[u8] {
        match self {
            Self::PubKey { hash, .. } => hash.to_raw_bytes(),
            Self::Script { hash, .. } => hash.to_raw_bytes(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IPStringParsingError {
    #[error("Invalid IPv4 Address String, expected period-separated bytes e.g. 0.0.0.0")]
    IPv4StringFormat,
    #[error("Invalid IPv6 Address String, expected colon-separated hextets e.g. 2001:0db8:0000:0000:0000:8a2e:0370:7334")]
    IPv6StringFormat,
    #[error("Deserializing from bytes: {0:?}")]
    DeserializeError(DeserializeError),
}

impl std::fmt::Display for Ipv4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(".")
        )
    }
}

impl FromStr for Ipv4 {
    type Err = IPStringParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split('.')
            .map(FromStr::from_str)
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_e| IPStringParsingError::IPv4StringFormat)
            .and_then(|bytes| Self::new(bytes).map_err(IPStringParsingError::DeserializeError))
    }
}

impl serde::Serialize for Ipv4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for Ipv4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_e| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"invalid ipv4 address")
        })
    }
}

impl schemars::JsonSchema for Ipv4 {
    fn schema_name() -> String {
        String::from("Ipv4")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

impl Ipv6 {
    const LEN: usize = 16;

    pub fn hextets(&self) -> Vec<u16> {
        let mut ret = Vec::with_capacity(Self::LEN / 2);
        for i in (0..self.inner.len()).step_by(2) {
            ret.push(((self.inner[i + 1] as u16) << 8) | (self.inner[i] as u16));
        }
        ret
    }
}

impl std::fmt::Display for Ipv6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Using the canonical format for IPV6 in RFC5952
        // 4.1) Leading zeros MUST be suppressed.
        // 4.2.1) :: MUST shorten as much as possible
        // 4.2.2) :: MUST NOT be used for a single 0 field
        // 4.2.3) :: Ties are broken by choosing the location first in the string
        // 4.3) Hex chars MUST be lowercase
        // NOTE: we do NOT support IPv4-Mapped IPv6 special text representations (Section 5)
        //       this is only RECOMMENDED, not required, and only when the format is known
        //       e.g. specific prefixes are used
        let mut best_gap_len = 0;
        let mut best_gap_start = 0;
        // usize::MAX is fine since we're max 16 here
        const UNDEF: usize = usize::MAX;
        let mut cur_gap_start = UNDEF;
        let hextets = self.hextets();
        for (i, hextet) in hextets.iter().enumerate() {
            if *hextet == 0 {
                if cur_gap_start == UNDEF {
                    cur_gap_start = i;
                }
            } else {
                if cur_gap_start != UNDEF && (i - cur_gap_start) > best_gap_len {
                    best_gap_len = i - cur_gap_start;
                    best_gap_start = cur_gap_start;
                }
                cur_gap_start = UNDEF;
            }
        }
        if cur_gap_start != UNDEF && (hextets.len() - cur_gap_start) > best_gap_len {
            best_gap_len = hextets.len() - cur_gap_start;
            best_gap_start = cur_gap_start;
        }
        fn ipv6_substr(hextet_substr: &[u16]) -> String {
            hextet_substr
                .iter()
                .map(|hextet| {
                    let trimmed = hex::encode(hextet.to_le_bytes())
                        .trim_start_matches('0')
                        .to_owned();
                    if trimmed.is_empty() {
                        "0".to_owned()
                    } else {
                        trimmed
                    }
                })
                .collect::<Vec<String>>()
                .join(":")
        }
        let canonical_str_rep = if best_gap_len > 1 {
            format!(
                "{}::{}",
                ipv6_substr(&hextets[..best_gap_start]),
                ipv6_substr(&hextets[(best_gap_start + best_gap_len)..])
            )
        } else {
            ipv6_substr(&hextets)
        };
        write!(f, "{}", canonical_str_rep)
    }
}

impl FromStr for Ipv6 {
    type Err = IPStringParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn ipv6_subbytes(substr: &str) -> Result<Vec<u8>, IPStringParsingError> {
            let mut bytes = Vec::new();
            for hextet_str in substr.split(':') {
                // hex::decode does not allow odd-length strings so pad it
                let padded_str = if hextet_str.len() % 2 == 0 {
                    Cow::Borrowed(hextet_str)
                } else {
                    Cow::Owned(format!("0{hextet_str}"))
                };
                let hextet_bytes = hex::decode(padded_str.as_bytes())
                    .map_err(|_e| IPStringParsingError::IPv6StringFormat)?;
                match hextet_bytes.len() {
                    0 => {
                        bytes.extend(&[0, 0]);
                    }
                    1 => {
                        bytes.push(0);
                        bytes.push(hextet_bytes[0]);
                    }
                    2 => {
                        bytes.extend(&hextet_bytes);
                    }
                    _ => return Err(IPStringParsingError::IPv6StringFormat),
                }
            }
            Ok(bytes)
        }
        let bytes = if let Some((left_str, right_str)) = s.split_once("::") {
            let mut bytes = ipv6_subbytes(left_str)?;
            let right_bytes = ipv6_subbytes(right_str)?;
            // pad middle with 0s
            bytes.resize(Self::LEN - right_bytes.len(), 0);
            bytes.extend(&right_bytes);
            bytes
        } else {
            ipv6_subbytes(s)?
        };
        Self::new(bytes).map_err(IPStringParsingError::DeserializeError)
    }
}

impl serde::Serialize for Ipv6 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for Ipv6 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_e| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"invalid ipv6 address")
        })
    }
}

impl schemars::JsonSchema for Ipv6 {
    fn schema_name() -> String {
        String::from("Ipv6")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4_json() {
        let json_str_1 = "\"0.0.0.0\"";
        let from_json_1: Ipv4 = serde_json::from_str(json_str_1).unwrap();
        let to_json_1 = serde_json::to_string_pretty(&from_json_1).unwrap();
        assert_eq!(json_str_1, to_json_1);
        let json_str_2 = "\"255.255.255.255\"";
        let from_json_2: Ipv4 = serde_json::from_str(json_str_2).unwrap();
        let to_json_2 = serde_json::to_string_pretty(&from_json_2).unwrap();
        assert_eq!(json_str_2, to_json_2);
    }

    fn ipv6_json_testcase(long_form_json: &str, canonical_form_json: &str) {
        let from_long: Ipv6 = serde_json::from_str(long_form_json).unwrap();
        let to_json_1 = serde_json::to_string_pretty(&from_long).unwrap();
        assert_eq!(canonical_form_json, to_json_1);
        let from_canonical: Ipv6 = serde_json::from_str(canonical_form_json).unwrap();
        let to_json_2 = serde_json::to_string_pretty(&from_canonical).unwrap();
        assert_eq!(canonical_form_json, to_json_2);
        assert_eq!(from_long.inner, from_canonical.inner);
    }

    #[test]
    fn ipv6_json() {
        // This tests that we abide by RFC5952 for IPV6 Canonical text form
        // part of the implementation relies on the hex crate's behavior but
        // that is checked as part of this test (e.g. that lowercase is used + omit leading 0s)
        ipv6_json_testcase(
            "\"2001:0db8:0000:0000:0000:ff00:0042:8329\"",
            "\"2001:db8::ff00:42:8329\"",
        );
        // ties broken by first one
        ipv6_json_testcase(
            "\"2001:0db8:0000:0000:1111:0000:0000:8329\"",
            "\"2001:db8::1111:0:0:8329\"",
        );
        // min run not first
        ipv6_json_testcase(
            "\"0001:0000:0002:0000:0000:0000:0003:0000\"",
            "\"1:0:2::3:0\"",
        );
        // ends in min run
        ipv6_json_testcase("\"000a:000b:0000:0000:0000:0000:0000:0000\"", "\"a:b::\"");
        // starts with min run
        ipv6_json_testcase(
            "\"0000:0000:0000:0000:0000:0000:abcd:0000\"",
            "\"::abcd:0\"",
        );
        // don't use runs for single 0 hextets
        ipv6_json_testcase(
            "\"0000:000a:0000:000b:0000:000c:0000:000d\"",
            "\"0:a:0:b:0:c:0:d\"",
        );
    }
}
