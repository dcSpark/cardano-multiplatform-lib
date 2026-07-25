//! Characterization tests for how the HAND-WRITTEN CIP36 outer deserializers
//! (`cip36/rust/src/utils.rs`) and the GENERATED inner deserializers
//! (`cip36/rust/src/generated/serialization.rs`) treat unknown / malformed keys
//! in a CIP-36 registration / deregistration metadata map.
//!
//! WHY THIS FILE EXISTS
//! --------------------
//! The crate is about to migrate to a new `cddl-codegen` that understands `any`
//! in CDDL and will REPLACE the hand-written `utils.rs` loops. That migration is
//! expected to change how unknown/malformed keys are handled. These tests pin
//! down EXACTLY what happens TODAY so any behavioral change surfaces as a
//! conscious red test during the migration.
//!
//! LABELS
//! ------
//! * every assertion is either a real `// contract:` (intended behavior that
//!   should survive migration) or a `// QUIRK:` / `// QUIRK-WATCH:` (accidental
//!   behavior of the current code that is LIKELY to change with the new codegen).
//!
//! THE CENTRAL BUG BEING CHARACTERIZED (outer loop in utils.rs)
//! -----------------------------------------------------------
//! The outer deserialize loop's `_unknown_key => ()` arm reads the unknown
//! integer LABEL but never consumes its VALUE, yet still does `read += 1`. So on
//! the next loop iteration the unknown key's VALUE is peeked/read AS IF IT WERE A
//! KEY. The observable outcome therefore depends entirely on the CBOR TYPE of the
//! unknown value, and the pair-count bookkeeping goes off by one. Every branch of
//! that divergence is pinned down below (see `scenario1_*`).
//!
//! Additional structural quirk: the outer loop calls `read_len.read_elems(2)`
//! ONCE up front (never per real pair) and `read_len.finish()` only on success.
//! Because of that, the outer definite map length must be EXACTLY 2 to succeed —
//! any other definite length fails with `DefiniteLenMismatch` regardless of what
//! the entries actually are.

use cbor_event::se::Serializer;
use cml_cip36::*;
use cml_core::error::DeserializeError;
use cml_core::serialization::{Deserialize, Serialize};
use cml_crypto::{Ed25519Signature, RawBytesEncoding};

// ---------------------------------------------------------------------------
// Fixtures (byte vectors reused from the crate's existing `sign_data` test).
// ---------------------------------------------------------------------------

fn stake_pub() -> cml_crypto::PublicKey {
    cml_crypto::PublicKey::from_raw_bytes(&[
        227, 205, 36, 4, 200, 77, 230, 95, 150, 145, 143, 24, 213, 180, 69, 188, 185, 51, 167, 205,
        161, 142, 237, 237, 121, 69, 221, 25, 30, 67, 35, 105,
    ])
    .unwrap()
}

fn address() -> cml_chain::address::Address {
    cml_chain::address::Address::from_raw_bytes(&[
        0, 71, 119, 86, 30, 125, 158, 193, 18, 236, 48, 117, 114, 250, 236, 26, 255, 97, 255, 12,
        254, 214, 141, 244, 205, 92, 132, 127, 24, 114, 182, 23, 101, 120, 129, 227, 10, 209, 124,
        70, 228, 1, 12, 156, 179, 235, 178, 68, 6, 83, 163, 77, 50, 33, 156, 131, 233,
    ])
    .unwrap()
}

fn voting_pub() -> CIP36VotingPubKey {
    CIP36VotingPubKey::from_raw_bytes(&[
        0, 54, 239, 62, 31, 13, 63, 89, 137, 226, 209, 85, 234, 84, 189, 178, 167, 44, 76, 69, 108,
        203, 149, 154, 244, 201, 72, 104, 244, 115, 245, 160,
    ])
    .unwrap()
}

/// A valid baseline registration. NOTE: weight is 1 (nonzero) on purpose —
/// `CIP36RegistrationCbor::deserialize` calls `verify()`, which errors iff all
/// delegation weights are zero. Keeping it nonzero isolates the unknown-key
/// behavior from the weights invariant.
fn baseline_reg() -> CIP36RegistrationCbor {
    let key_reg = CIP36KeyRegistration::new(
        CIP36DelegationDistribution::new_weighted(cml_core::non_empty::NonEmptyVec::new(
            CIP36Delegation::new(voting_pub(), 1),
        )),
        stake_pub(),
        address(),
        1234,
    );
    let witness =
        CIP36RegistrationWitness::new(Ed25519Signature::from_raw_bytes(&[0u8; 64]).unwrap());
    CIP36RegistrationCbor::new(key_reg, witness)
}

fn baseline_dereg() -> CIP36DeregistrationCbor {
    let key_dereg = CIP36KeyDeregistration::new(stake_pub(), 1234);
    let witness =
        CIP36DeregistrationWitness::new(Ed25519Signature::from_raw_bytes(&[0u8; 64]).unwrap());
    CIP36DeregistrationCbor::new(key_dereg, witness)
}

// The canonical inner metadatum bytes for each of the 4 CIP-36 inner maps.
// Empirically: reg key-registration = definite map of 5 (0xa5), witnesses =
// definite map of 1 (0xa1), key-deregistration = definite map of 2 (0xa2).
fn reg_key_bytes() -> Vec<u8> {
    baseline_reg().key_registration.to_cbor_bytes()
}
fn reg_witness_bytes() -> Vec<u8> {
    baseline_reg().registration_witness.to_cbor_bytes()
}
fn dereg_key_bytes() -> Vec<u8> {
    baseline_dereg().key_deregistration.to_cbor_bytes()
}
fn dereg_witness_bytes() -> Vec<u8> {
    baseline_dereg().deregistration_witness.to_cbor_bytes()
}

// ---------------------------------------------------------------------------
// Helpers for crafting mutated CBOR and asserting on the exact failure variant.
// ---------------------------------------------------------------------------

/// Assemble an outer metadata map. `len == Some(n)` => definite length n;
/// `len == None` => indefinite length terminated by a CBOR break. Each entry is
/// an unsigned-integer label followed by already-encoded raw value bytes.
fn assemble(len: Option<u64>, entries: &[(u64, Vec<u8>)]) -> Vec<u8> {
    let mut s = Serializer::new_vec();
    match len {
        Some(n) => s.write_map(cbor_event::Len::Len(n)).unwrap(),
        None => s.write_map(cbor_event::Len::Indefinite).unwrap(),
    };
    for (label, val) in entries {
        s.write_unsigned_integer(*label).unwrap();
        s.write_raw_bytes(val).unwrap();
    }
    if len.is_none() {
        s.write_special(cbor_event::Special::Break).unwrap();
    }
    s.finalize()
}

// Encoders for a single CBOR value of each major type, used as the (unconsumed)
// VALUE of an unknown key so we can observe how the loop mis-reads it as a key.
fn cbor_uint(n: u64) -> Vec<u8> {
    let mut s = Serializer::new_vec();
    s.write_unsigned_integer(n).unwrap();
    s.finalize()
}
fn cbor_text(t: &str) -> Vec<u8> {
    let mut s = Serializer::new_vec();
    s.write_text(t).unwrap();
    s.finalize()
}
fn cbor_map0() -> Vec<u8> {
    let mut s = Serializer::new_vec();
    s.write_map(cbor_event::Len::Len(0)).unwrap();
    s.finalize()
}
fn cbor_arr0() -> Vec<u8> {
    let mut s = Serializer::new_vec();
    s.write_array(cbor_event::Len::Len(0)).unwrap();
    s.finalize()
}
fn cbor_bytes() -> Vec<u8> {
    let mut s = Serializer::new_vec();
    s.write_bytes(vec![0xaa]).unwrap();
    s.finalize()
}

/// Debug string of the underlying `DeserializeFailure` variant (location
/// annotations stripped), for exact substring assertions.
fn fail_of<T>(r: Result<T, DeserializeError>) -> String {
    format!("{:?}", r.err().expect("expected an Err").failure())
}

// ===========================================================================
// SCENARIO 1: unknown integer label BEFORE the CIP-36 labels.
// unknown labels are CAPTURED into
// `rest` — key encoding, value, and wire position included — and round-trip
// byte-exactly. (avoids a skip arm never consumed
// the unknown VALUE, which would diverge by the value's CBOR type — e.g. an
// int-valued unknown label would silent eat a mandatory field.)
// ===========================================================================

#[test]
fn scenario1_unknown_label_before_cip36_labels() {
    // ---- REGISTRATION -----------------------------------------------------
    let reg_key = reg_key_bytes();
    let reg_wit = reg_witness_bytes();
    let reg = |val: Vec<u8>| {
        assemble(
            Some(3),
            &[
                (99, val),
                (61284, reg_key.clone()),
                (61285, reg_wit.clone()),
            ],
        )
    };
    for val in [
        cbor_uint(42),
        cbor_text("foo"),
        cbor_map0(),
        cbor_arr0(),
        cbor_bytes(),
    ] {
        let bytes = reg(val);
        let parsed = CIP36RegistrationCbor::from_metadata_bytes(&bytes)
            .expect("contract: unknown labels are captured, not rejected");
        assert_eq!(parsed.rest.len(), 1, "unknown entry lands in rest");
        assert_eq!(
            parsed.to_metadata_bytes(),
            bytes,
            "unknown entry round-trips byte-exactly (position + encodings)"
        );
    }

    // ---- DEREGISTRATION ---------------------------------------------------
    let dereg_key = dereg_key_bytes();
    let dereg_wit = dereg_witness_bytes();
    let dereg = |val: Vec<u8>| {
        assemble(
            Some(3),
            &[
                (99, val),
                (61285, dereg_wit.clone()),
                (61286, dereg_key.clone()),
            ],
        )
    };
    for val in [
        cbor_uint(42),
        cbor_text("foo"),
        cbor_map0(),
        cbor_arr0(),
        cbor_bytes(),
    ] {
        let bytes = dereg(val);
        let parsed = CIP36DeregistrationCbor::from_metadata_bytes(&bytes)
            .expect("contract: unknown labels are captured, not rejected");
        assert_eq!(parsed.rest.len(), 1, "unknown entry lands in rest");
        assert_eq!(parsed.to_metadata_bytes(), bytes);
    }
}

// ===========================================================================
// SCENARIO 2: unknown integer label AFTER the two CIP-36 pairs.
// captured and preserved for BOTH definite and
// indefinite outer maps. (avoids definite maps with extras failing
// if indefinite maps silently DROPPED the unknown entry on re-serialization.)
// ===========================================================================

#[test]
fn scenario2_unknown_label_after_cip36_labels() {
    let reg_key = reg_key_bytes();
    let reg_wit = reg_witness_bytes();
    let dereg_key = dereg_key_bytes();
    let dereg_wit = dereg_witness_bytes();

    for len in [Some(3), None] {
        let reg_bytes = assemble(
            len,
            &[
                (61284, reg_key.clone()),
                (61285, reg_wit.clone()),
                (99, cbor_uint(42)),
            ],
        );
        let parsed = CIP36RegistrationCbor::from_metadata_bytes(&reg_bytes)
            .expect("contract: trailing unknown label is captured (definite or indefinite)");
        assert_eq!(parsed.rest.len(), 1);
        assert_eq!(
            parsed.to_metadata_bytes(),
            reg_bytes,
            "trailing unknown entry (and the outer length encoding) must be preserved"
        );

        let dereg_bytes = assemble(
            len,
            &[
                (61285, dereg_wit.clone()),
                (61286, dereg_key.clone()),
                (99, cbor_uint(42)),
            ],
        );
        let parsed = CIP36DeregistrationCbor::from_metadata_bytes(&dereg_bytes)
            .expect("contract: trailing unknown label is captured (definite or indefinite)");
        assert_eq!(parsed.rest.len(), 1);
        assert_eq!(parsed.to_metadata_bytes(), dereg_bytes);
    }
}

// ===========================================================================
// SCENARIO 3: a genuine TEXT key in the outer map.
// The rest row's key domain is `uint` (transaction_metadatum_label), so text
// keys are rejected — with the SAME error for both types. (avoids
// loops diverging: registration UnknownKey(Str),
// deregistration UnexpectedKeyType(Text).)
// ===========================================================================

#[test]
fn scenario3_text_key_outer_asymmetry() {
    // Build outer maps whose first key is a text string. `assemble` only emits
    // integer labels, so craft these by hand.
    let text_key_reg = {
        let mut s = Serializer::new_vec();
        s.write_map(cbor_event::Len::Len(3)).unwrap();
        s.write_text("hello").unwrap();
        s.write_raw_bytes(&cbor_uint(1)).unwrap();
        s.write_unsigned_integer(61284).unwrap();
        s.write_raw_bytes(&reg_key_bytes()).unwrap();
        s.write_unsigned_integer(61285).unwrap();
        s.write_raw_bytes(&reg_witness_bytes()).unwrap();
        s.finalize()
    };
    let text_key_dereg = {
        let mut s = Serializer::new_vec();
        s.write_map(cbor_event::Len::Len(3)).unwrap();
        s.write_text("hello").unwrap();
        s.write_raw_bytes(&cbor_uint(1)).unwrap();
        s.write_unsigned_integer(61285).unwrap();
        s.write_raw_bytes(&dereg_witness_bytes()).unwrap();
        s.write_unsigned_integer(61286).unwrap();
        s.write_raw_bytes(&dereg_key_bytes()).unwrap();
        s.finalize()
    };

    // contract: unknown-KEY tolerance is exactly as wide as the rest row's key
    // domain (`uint`); a text key is an UnknownKey error either way.
    assert_eq!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&text_key_reg)),
        "UnknownKey(Str(\"hello\"))"
    );
    assert_eq!(
        fail_of(CIP36DeregistrationCbor::from_metadata_bytes(
            &text_key_dereg
        )),
        "UnknownKey(Str(\"hello\"))"
    );
}

// ===========================================================================
// SCENARIO 4: duplicate CIP-36 labels -> DuplicateKey.
// ===========================================================================

#[test]
fn scenario4_duplicate_keys() {
    let reg_key = reg_key_bytes();
    let reg_wit = reg_witness_bytes();
    let dereg_key = dereg_key_bytes();
    let dereg_wit = dereg_witness_bytes();

    // contract: duplicated 61284 is detected before any length reconciliation.
    assert_eq!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&assemble(
            Some(3),
            &[
                (61284, reg_key.clone()),
                (61284, reg_key.clone()),
                (61285, reg_wit.clone()),
            ],
        ))),
        "DuplicateKey(Uint(61284))"
    );
    // contract: duplicated 61285 (registration witness).
    assert_eq!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&assemble(
            Some(3),
            &[
                (61284, reg_key.clone()),
                (61285, reg_wit.clone()),
                (61285, reg_wit.clone()),
            ],
        ))),
        "DuplicateKey(Uint(61285))"
    );
    // contract: duplicated 61285 (deregistration witness).
    assert_eq!(
        fail_of(CIP36DeregistrationCbor::from_metadata_bytes(&assemble(
            Some(3),
            &[
                (61285, dereg_wit.clone()),
                (61285, dereg_wit.clone()),
                (61286, dereg_key.clone()),
            ],
        ))),
        "DuplicateKey(Uint(61285))"
    );
    // contract: duplicated 61286 (key deregistration).
    assert_eq!(
        fail_of(CIP36DeregistrationCbor::from_metadata_bytes(&assemble(
            Some(3),
            &[
                (61285, dereg_wit.clone()),
                (61286, dereg_key.clone()),
                (61286, dereg_key.clone()),
            ],
        ))),
        "DuplicateKey(Uint(61286))"
    );
}

// ===========================================================================
// SCENARIO 5: a missing mandatory CIP-36 label.
// The outcome depends on the outer map's declared length because of the up-front
// `read_elems(2)` bookkeeping.
// ===========================================================================

#[test]
fn scenario5_missing_mandatory_fields() {
    let reg_key = reg_key_bytes();
    let reg_wit = reg_witness_bytes();
    let dereg_key = dereg_key_bytes();
    let dereg_wit = dereg_witness_bytes();

    // QUIRK-WATCH: a DEFINITE map of length 1 (the natural "only one field
    // present" shape) never reaches the MandatoryFieldMissing check: the up-front
    // `read_elems(2)` immediately exceeds the declared length 1 and errors with
    // DefiniteLenMismatch(1, None). So "missing field" in a definite map is
    // reported as a LENGTH error, not a missing-field error.
    assert_eq!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&assemble(
            Some(1),
            &[(61284, reg_key.clone())],
        ))),
        "DefiniteLenMismatch(1, None)"
    );
    assert_eq!(
        fail_of(CIP36DeregistrationCbor::from_metadata_bytes(&assemble(
            Some(1),
            &[(61285, dereg_wit.clone())],
        ))),
        "DefiniteLenMismatch(1, None)"
    );

    // contract: to actually reach the MandatoryFieldMissing branch, use an
    // INDEFINITE outer map (no up-front length reconciliation). Then the correct
    // missing-label is reported.
    assert_eq!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&assemble(
            None,
            &[(61284, reg_key.clone())], // witness 61285 missing
        ))),
        "MandatoryFieldMissing(Uint(61285))"
    );
    assert_eq!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&assemble(
            None,
            &[(61285, reg_wit.clone())], // key_registration 61284 missing
        ))),
        "MandatoryFieldMissing(Uint(61284))"
    );
    assert_eq!(
        fail_of(CIP36DeregistrationCbor::from_metadata_bytes(&assemble(
            None,
            &[(61285, dereg_wit.clone())], // key_deregistration 61286 missing
        ))),
        "MandatoryFieldMissing(Uint(61286))"
    );
    assert_eq!(
        fail_of(CIP36DeregistrationCbor::from_metadata_bytes(&assemble(
            None,
            &[(61286, dereg_key.clone())], // witness 61285 missing
        ))),
        "MandatoryFieldMissing(Uint(61285))"
    );
}

// ===========================================================================
// SCENARIO 6: unknown keys INSIDE the inner maps, handled by the GENERATED
// deserializers. These are the assertions MOST likely to intentionally change
// with the `any`-supporting codegen, hence every one is QUIRK-WATCH.
// ===========================================================================

/// Take an inner metadatum that is a definite short-form map (`0xa0..=0xb7`),
/// bump its declared length by one, and append one extra `key => 0` pair.
fn inner_with_extra_uint_key(mut inner: Vec<u8>, key: u64) -> Vec<u8> {
    assert!(
        (0xa0..=0xb7).contains(&inner[0]),
        "expected a definite short-form map header, got {:#04x}",
        inner[0]
    );
    inner[0] += 1;
    inner.push(key as u8); // 0..=23 encode as a single byte
    inner.push(0x00); // value 0
    inner
}

#[test]
fn scenario6_unknown_keys_inside_inner_maps() {
    // key_registration (definite map of 5) with an extra unknown key 6.
    // QUIRK-WATCH: the generated deserializer hard-errors on the unknown integer
    // key. The `any`-supporting codegen is expected to instead tolerate/collect
    // it.
    let bad_key_reg = inner_with_extra_uint_key(reg_key_bytes(), 6);
    assert_eq!(
        fail_of(CIP36KeyRegistration::from_cbor_bytes(&bad_key_reg)),
        "UnknownKey(Uint(6))"
    );
    // ...and the same error surfaces through the outer `from_metadata_bytes`.
    // QUIRK-WATCH.
    assert_eq!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&assemble(
            Some(2),
            &[(61284, bad_key_reg.clone()), (61285, reg_witness_bytes()),],
        ))),
        "UnknownKey(Uint(6))"
    );

    // key_deregistration (definite map of 2) with an extra unknown key 4.
    // QUIRK-WATCH.
    let bad_key_dereg = inner_with_extra_uint_key(dereg_key_bytes(), 4);
    assert_eq!(
        fail_of(CIP36KeyDeregistration::from_cbor_bytes(&bad_key_dereg)),
        "UnknownKey(Uint(4))"
    );
    // ...and through the outer deregistration `from_metadata_bytes`. QUIRK-WATCH.
    assert_eq!(
        fail_of(CIP36DeregistrationCbor::from_metadata_bytes(&assemble(
            Some(2),
            &[
                (61285, dereg_witness_bytes()),
                (61286, bad_key_dereg.clone()),
            ],
        ))),
        "UnknownKey(Uint(4))"
    );

    // Witness map (definite map of 1) with an extra unknown key 2.
    // QUIRK-WATCH: unlike the key maps, the generated WITNESS deserializer calls
    // `read_len.finish()` UP FRONT (right after `read_elems(1)`), so a definite
    // map of length 2 is rejected as DefiniteLenMismatch BEFORE the loop ever
    // sees key 2 — the UnknownKey(Uint) path is unreachable for a definite map.
    let bad_witness_def = inner_with_extra_uint_key(reg_witness_bytes(), 2);
    assert_eq!(
        fail_of(CIP36RegistrationWitness::from_cbor_bytes(&bad_witness_def)),
        "DefiniteLenMismatch(2, Some(1))"
    );

    // The generated witness UnknownKey(Uint) path IS reachable via an INDEFINITE
    // inner map (no up-front finish). QUIRK-WATCH: this is the behavior expected
    // to change to tolerate the extra key.
    let bad_witness_indef = {
        let mut s = Serializer::new_vec();
        s.write_map(cbor_event::Len::Indefinite).unwrap();
        s.write_unsigned_integer(1).unwrap();
        s.write_bytes(vec![0u8; 64]).unwrap();
        s.write_unsigned_integer(2).unwrap();
        s.write_unsigned_integer(0).unwrap();
        s.write_special(cbor_event::Special::Break).unwrap();
        s.finalize()
    };
    assert_eq!(
        fail_of(CIP36RegistrationWitness::from_cbor_bytes(
            &bad_witness_indef
        )),
        "UnknownKey(Uint(2))"
    );
}

// ===========================================================================
// SCENARIO 7: outer length / key-type mismatches.
// ===========================================================================

#[test]
fn scenario7_length_and_key_type_mismatches() {
    let reg_key = reg_key_bytes();
    let reg_wit = reg_witness_bytes();

    // Declared length 3 but only 2 entries present: after consuming both real
    // pairs the loop (n=3) tries to read a 3rd key and runs off the end of the
    // buffer -> a raw cbor_event NotEnough error wrapped as CBOR(..).
    // QUIRK-WATCH: reported as a low-level CBOR read error rather than a
    // structural one.
    let short_buf = {
        let mut s = Serializer::new_vec();
        s.write_map(cbor_event::Len::Len(3)).unwrap();
        s.write_unsigned_integer(61284).unwrap();
        s.write_raw_bytes(&reg_key).unwrap();
        s.write_unsigned_integer(61285).unwrap();
        s.write_raw_bytes(&reg_wit).unwrap();
        s.finalize()
    };
    assert!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&short_buf))
            .starts_with("CBOR(NotEnough"),
        "expected a CBOR(NotEnough..) read error for declared-3/actual-2"
    );

    // contract: a CBOR break inside a definite-length outer map is rejected.
    let break_in_def = {
        let mut s = Serializer::new_vec();
        s.write_map(cbor_event::Len::Len(2)).unwrap();
        s.write_special(cbor_event::Special::Break).unwrap();
        s.finalize()
    };
    assert_eq!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&break_in_def)),
        "BreakInDefiniteLen"
    );

    // contract: a negative-integer key hits the `other_type` arm.
    let neg_key = {
        let mut s = Serializer::new_vec();
        s.write_map(cbor_event::Len::Len(3)).unwrap();
        s.write_negative_integer(-1).unwrap();
        s.write_raw_bytes(&cbor_uint(1)).unwrap();
        s.write_unsigned_integer(61284).unwrap();
        s.write_raw_bytes(&reg_key).unwrap();
        s.write_unsigned_integer(61285).unwrap();
        s.write_raw_bytes(&reg_wit).unwrap();
        s.finalize()
    };
    assert_eq!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&neg_key)),
        "UnexpectedKeyType(NegativeInteger)"
    );
    // Same for deregistration (both loops share the `other_type` arm here).
    let neg_key_dereg = {
        let mut s = Serializer::new_vec();
        s.write_map(cbor_event::Len::Len(3)).unwrap();
        s.write_negative_integer(-1).unwrap();
        s.write_raw_bytes(&cbor_uint(1)).unwrap();
        s.write_unsigned_integer(61285).unwrap();
        s.write_raw_bytes(&dereg_witness_bytes()).unwrap();
        s.write_unsigned_integer(61286).unwrap();
        s.write_raw_bytes(&dereg_key_bytes()).unwrap();
        s.finalize()
    };
    assert_eq!(
        fail_of(CIP36DeregistrationCbor::from_metadata_bytes(&neg_key_dereg)),
        "UnexpectedKeyType(NegativeInteger)"
    );

    // contract: a byte-string key hits the `other_type` arm.
    let bytes_key = {
        let mut s = Serializer::new_vec();
        s.write_map(cbor_event::Len::Len(3)).unwrap();
        s.write_bytes(vec![0x01]).unwrap();
        s.write_raw_bytes(&cbor_uint(1)).unwrap();
        s.write_unsigned_integer(61284).unwrap();
        s.write_raw_bytes(&reg_key).unwrap();
        s.write_unsigned_integer(61285).unwrap();
        s.write_raw_bytes(&reg_wit).unwrap();
        s.finalize()
    };
    assert_eq!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&bytes_key)),
        "UnexpectedKeyType(Bytes)"
    );
}

// ===========================================================================
// SCENARIO 8: trailing garbage after a complete, valid structure.
// ===========================================================================

#[test]
fn scenario8_trailing_garbage_is_rejected() {
    // contract: `from_metadata_bytes` (= the generated `from_cbor_bytes`) checks
    // that the whole input was consumed; extra bytes after a complete valid outer
    // map are rejected. (avoids the hand-written deserializer
    // silently ignored trailing bytes.)
    let mut bytes = baseline_reg().to_metadata_bytes();
    bytes.extend_from_slice(&[0xff, 0xff, 0xde, 0xad]);
    assert_eq!(
        fail_of(CIP36RegistrationCbor::from_metadata_bytes(&bytes)),
        "CBOR(TrailingData)"
    );

    // Same for deregistration.
    let mut dbytes = baseline_dereg().to_metadata_bytes();
    dbytes.extend_from_slice(&[0x01, 0x02, 0x03]);
    assert_eq!(
        fail_of(CIP36DeregistrationCbor::from_metadata_bytes(&dbytes)),
        "CBOR(TrailingData)"
    );
}
