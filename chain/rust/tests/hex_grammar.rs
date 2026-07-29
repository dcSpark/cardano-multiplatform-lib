//! Pins which hex grammar each read surface accepts.
//!
//! CML uses two, chosen by who owns the format (see `cml_core::hex_grammar`):
//!
//! * **canonical** — bare, even-length, lowercase — where CML owns the encoding and its writer
//!   emits exactly that (hashes, addresses, keys, signatures);
//! * **bare, any case** — where the format belongs to someone else (RFC 4291 IPv6, cardano-cli
//!   JSON interop). A `0x` prefix is rejected on both.
//!
//! This file exists because swapping the hex decoder (`hex` → `const-hex`) silently widened every
//! surface at once, and no existing test could see it: the decoded bytes are identical either way,
//! so a suite that only feeds well-formed input cannot detect the accepted-input SET growing.

use cml_chain::address::Address;
use cml_chain::certs::Ipv6;
use cml_chain::json::plutus_datums::{
    CardanoNodePlutusDatumSchema, encode_json_str_to_plutus_datum,
};
use cml_crypto::{Ed25519KeyHash, RawBytesEncoding, TransactionHash};
use std::str::FromStr;

const HASH28: &str = "00112233445566778899aabbccddeeff00112233445566778899aabb";
const HASH32: &str = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
// mainnet enterprise address, key-hash payment credential
const ADDR: &str = "60112233445566778899aabbccddeeff00112233445566778899aabbcc";

// ---------------------------------------------------------------- CML-owned: canonical

#[test]
fn canonical_surfaces_round_trip_on_the_encoding() {
    // Not merely `bytes(from(s)) == bytes(s)` — the string itself comes back.
    assert_eq!(Ed25519KeyHash::from_hex(HASH28).unwrap().to_hex(), HASH28);
    assert_eq!(
        Ed25519KeyHash::from_raw_hex(HASH28).unwrap().to_raw_hex(),
        HASH28
    );
    assert_eq!(TransactionHash::from_hex(HASH32).unwrap().to_hex(), HASH32);
    assert_eq!(Address::from_hex(ADDR).unwrap().to_hex(), ADDR);
}

#[test]
fn canonical_surfaces_reject_uppercase() {
    // The narrowing adopted from upstream: uppercase is a SECOND encoding of the same value, and a
    // reader wider than its writer cannot offer the encoding-level round trip above.
    let up28 = HASH28.to_ascii_uppercase();
    let up32 = HASH32.to_ascii_uppercase();
    assert!(Ed25519KeyHash::from_hex(&up28).is_err());
    assert!(Ed25519KeyHash::from_raw_hex(&up28).is_err());
    assert!(TransactionHash::from_hex(&up32).is_err());
    assert!(Address::from_hex(&ADDR.to_ascii_uppercase()).is_err());
    // mixed case too
    assert!(Ed25519KeyHash::from_hex(&format!("A{}", &HASH28[1..])).is_err());
}

#[test]
fn canonical_surfaces_reject_prefixes() {
    assert!(Ed25519KeyHash::from_hex(&format!("0x{HASH28}")).is_err());
    assert!(Ed25519KeyHash::from_raw_hex(&format!("0x{HASH28}")).is_err());
    assert!(Ed25519KeyHash::from_hex(&format!("0X{HASH28}")).is_err());
    assert!(Address::from_hex(&format!("0x{ADDR}")).is_err());
}

#[test]
fn the_two_spellings_of_the_same_type_agree() {
    // The regression in its purest form. `from_hex` and `from_raw_hex` on the SAME type must answer
    // identically — they disagreed twice during this migration, once on `0x` and once on case.
    for s in [
        HASH28,
        &HASH28.to_ascii_uppercase(),
        &format!("A{}", &HASH28[1..]),
        &format!("0x{HASH28}"),
        &format!("0X{HASH28}"),
        "0x",
        "",
        "zz",
        "abc",
    ] {
        assert_eq!(
            Ed25519KeyHash::from_hex(s).is_ok(),
            Ed25519KeyHash::from_raw_hex(s).is_ok(),
            "from_hex and from_raw_hex disagree on {s:?}"
        );
    }
}

// ---------------------------------------------------------- foreign formats: bare, any case

#[test]
fn ipv6_accepts_either_case_but_not_a_prefix() {
    // RFC 4291 §2.2 writes hextets uppercase; RFC 5952 prefers lowercase output while requiring
    // readers to accept both. Narrowing this would refuse a spelling the standard blesses.
    assert!(Ipv6::from_str("2001:0db8:0000:0000:0000:ff00:0042:8329").is_ok());
    assert!(Ipv6::from_str("2001:0DB8:0000:0000:0000:FF00:0042:8329").is_ok());
    // ...but `0x12` is not a hextet, and the decoder would otherwise strip the prefix and take it.
    assert!(Ipv6::from_str("2001:0db8:0000:0000:0000:ff00:0x42:8329").is_err());
}

#[test]
fn plutus_datum_json_accepts_uppercase_as_cardano_cli_emits() {
    // The doc comment on this very function documents `{"bytes": "CAFEF00D"}`. This is
    // cardano-cli `--script-data-file` interop, not CML's own encoding.
    let up = r#"{"bytes": "CAFEF00D"}"#;
    let lo = r#"{"bytes": "cafef00d"}"#;
    let s = CardanoNodePlutusDatumSchema::DetailedSchema;
    assert!(encode_json_str_to_plutus_datum(up, s).is_ok());
    assert!(encode_json_str_to_plutus_datum(lo, s).is_ok());
    assert_eq!(
        encode_json_str_to_plutus_datum(up, s).unwrap(),
        encode_json_str_to_plutus_datum(lo, s).unwrap()
    );
}

#[test]
fn plutus_datum_json_rejects_both_prefix_spellings() {
    // The hand check in `plutus_datums.rs` is `starts_with("0x")`, which misses `0X` — the decoder
    // would then strip it and accept. Both must be refused.
    let s = CardanoNodePlutusDatumSchema::DetailedSchema;
    assert!(encode_json_str_to_plutus_datum(r#"{"bytes": "0xcafef00d"}"#, s).is_err());
    assert!(encode_json_str_to_plutus_datum(r#"{"bytes": "0Xcafef00d"}"#, s).is_err());
}
