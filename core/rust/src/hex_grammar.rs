//! Which hex grammar a read surface accepts, and why the answer is not the same everywhere.
//!
//! CML reads hex text at two kinds of place, and they get two different grammars. The deciding
//! question is **who owns the format**.
//!
//! # 1. CML owns the encoding → canonical
//!
//! Hashes, addresses, keys, signatures, the EmIP3 blob, Byron block CBOR. CML both writes and reads
//! these, and every writer emits bare lowercase. They read through
//! [`decode_canonical_hex`] (re-exported here from the generated runtime, which owns the grammar and
//! uses it for `RawBytesEncoding::from_raw_hex` and the emitted JSON bytes newtypes).
//!
//! Canonical means **bare, even-length, lowercase**. The property it buys is that the accepted
//! grammar and the emitted grammar are the same grammar: `from_hex(s)?.to_hex() == s` for every
//! accepted `s`, not merely `bytes(from_hex(s)) == bytes(s)`. Uppercase and `0x`-prefixed input are
//! second and third spellings of the same value, and admitting spellings CML would never itself
//! write is what makes "the encoding of this value" stop being a single thing.
//!
//! # 2. Someone else owns the format → [`decode_bare`]
//!
//! Bare, even-length, **any case**. Rejects a `0x`/`0X` prefix — the backing decoder strips one
//! silently, and none of these formats permit it — but does not impose CML's casing preference on a
//! grammar CML did not define:
//!
//! * **IPv6 hextets** (`certs/utils.rs`). RFC 4291 §2.2 writes them in uppercase; RFC 5952 prefers
//!   lowercase for output while requiring readers to accept both. Rejecting `2001:0DB8::…` would
//!   refuse a spelling the standard explicitly blesses.
//! * **Plutus datum JSON** (`json/plutus_datums.rs`) and **transaction metadata JSON**
//!   (`json/metadatums.rs`). These are cardano-cli interop (`--script-data-file` and friends); the
//!   documented examples on those very functions are uppercase (`{"bytes": "CAFEF00D"}`). Narrowing
//!   them would reject documents cardano-cli itself produces, and contradict CML's own published
//!   docs.
//!
//! `metadatums.rs`'s `hex_string_to_bytes` additionally REQUIRES a leading `0x` as CML's own JSON
//! convention; it strips that itself and hands the remainder here, so `0x0xdead` stays rejected.
//!
//! # Why this file exists at all
//!
//! Swapping the hex decoder (`hex` → `const-hex`, for its unconditional `core::error::Error` impl)
//! silently WIDENED every one of these surfaces: `const-hex` strips a leading `0x`. The decoded
//! bytes are identical either way, so no existing test could see it — a suite that only feeds
//! well-formed input cannot detect the accepted-input SET growing. Both grammars are therefore
//! stated explicitly here rather than inherited from whatever decoder happens to be underneath.

extern crate alloc;
use alloc::vec::Vec;

/// The canonical grammar — bare, even-length, lowercase — owned by the generated runtime.
///
/// Re-exported so this module is the single place that names both doors and the rule for choosing
/// between them, and so `#[macro_export]` macro bodies can reach it through a `$crate` path.
pub use crate::serialization::decode_canonical_hex;

/// Decode bare hex digits of **either case**, rejecting a leading `0x`/`0X`.
///
/// For formats CML does not define. Where CML owns the encoding, use [`decode_canonical_hex`]
/// instead — see the module docs.
///
/// The error for a prefixed string is exactly what the incumbent decoder answered before the
/// const-hex swap — an invalid character at index 1 — so callers' rendering is unchanged.
///
/// A bare `"0x"` is the edge that makes this load-bearing rather than cosmetic: the backing decoder
/// strips the prefix and returns an EMPTY byte string, which a variable-length caller would
/// otherwise accept as a legitimate empty value.
pub fn decode_bare(hex_str: &str) -> Result<Vec<u8>, hex::FromHexError> {
    reject_prefix(hex_str)?;
    hex::decode(hex_str)
}

/// The prefix check alone, for callers that decode through something other than [`decode_bare`].
pub fn reject_prefix(hex_str: &str) -> Result<(), hex::FromHexError> {
    let raw = hex_str.as_bytes();
    if raw.len() >= 2 && raw[0] == b'0' && (raw[1] == b'x' || raw[1] == b'X') {
        return Err(hex::FromHexError::InvalidHexCharacter {
            c: raw[1] as char,
            index: 1,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn bare_accepts_either_case() {
        assert_eq!(decode_bare("a1b2").unwrap(), vec![0xa1, 0xb2]);
        assert_eq!(decode_bare("A1B2").unwrap(), vec![0xa1, 0xb2]);
        assert_eq!(decode_bare("a1B2").unwrap(), vec![0xa1, 0xb2]);
        assert_eq!(decode_bare("").unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn canonical_rejects_uppercase() {
        // The whole reason the two doors exist.
        assert!(decode_canonical_hex("a1b2").is_ok());
        assert!(decode_canonical_hex("A1B2").is_err());
        assert!(decode_canonical_hex("a1B2").is_err());
    }

    #[test]
    fn both_doors_reject_a_prefix() {
        for s in ["0xa1b2", "0Xa1b2", "0x"] {
            assert!(decode_bare(s).is_err(), "decode_bare accepted {s:?}");
            assert!(
                decode_canonical_hex(s).is_err(),
                "decode_canonical_hex accepted {s:?}"
            );
        }
    }

    #[test]
    fn prefix_error_names_index_1() {
        // Pinned so a future decoder change is loud rather than silent.
        match decode_bare("0xa1b2") {
            Err(hex::FromHexError::InvalidHexCharacter { c, index }) => {
                assert_eq!((c, index), ('x', 1));
            }
            other => panic!("expected invalid character at index 1, got {other:?}"),
        }
    }

    #[test]
    fn bare_0x_is_not_an_empty_byte_string() {
        assert!(decode_bare("0x").is_err());
        assert!(decode_canonical_hex("0x").is_err());
    }

    #[test]
    fn malformed_input_still_reports_the_original_error() {
        assert!(matches!(
            decode_bare("a1b"),
            Err(hex::FromHexError::OddLength)
        ));
        assert!(matches!(
            decode_bare("zz"),
            Err(hex::FromHexError::InvalidHexCharacter { .. })
        ));
    }
}
