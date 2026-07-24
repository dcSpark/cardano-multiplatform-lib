//! Characterization tests for `cml-cip36`.
//!
//! Purpose: lock in the *current* observable behavior of the hand-written +
//! generated CIP-36 code before the crate is migrated to a new cddl-codegen.
//! These tests capture:
//!   1. `verify()` validation invariants (CIP-36 delegation-weight rules)
//!   2. where `verify()` is enforced (add_to_metadata / from_metadata_bytes /
//!      to_metadata_bytes)
//!   3. the `? 5 : voting_purpose .default 0` default-field wire semantics
//!   4. `hash_to_sign` goldens (registration + deregistration, both canonical
//!      flags)
//!   5. the structural shape of the `hash_to_sign` preimage
//!   6. end-to-end signing/verification against the official CIP-36 vectors
//!
//! Goldens are literal hardcoded hex strings (never self-derived). The keys,
//! addresses, preimages, hashes and witness signatures come from the official
//! CIP-36 test vector. Assertions that encode a real contract are tagged
//! plainly; assertions that pin an *accidental* behavior that may legitimately
//! change under the migration are tagged `// QUIRK:` / `// QUIRK-WATCH:`.

use cbor_event::se::Serializer;
use cml_chain::address::Address;
use cml_chain::auxdata::Metadata;
use cml_cip36::error::CIP36Error;
use cml_cip36::*;
use cml_core::error::DeserializeFailure;
use cml_core::serialization::{Deserialize, RawBytesEncoding, Serialize};
use cml_crypto::{Ed25519Signature, PrivateKey, PublicKey};

// ---------------------------------------------------------------------------
// Official CIP-36 test-vector constants
// ---------------------------------------------------------------------------

const NONCE: u64 = 1234;

// Staking (a.k.a. stake credential) key pair. The witness signs the sign-hash
// with the staking key.
const STAKING_PRV_HEX: &str = "852fa5d17df3efdfdcd6dac53ec9fe5593f3c0bd7cadb3c2af76c7e15dfa8a5c";
const STAKING_PUB_HEX: &str = "e3cd2404c84de65f96918f18d5b445bcb933a7cda18eeded7945dd191e432369";
const VOTE_PUB_HEX: &str = "0036ef3e1f0d3f5989e2d155ea54bdb2a72c4c456ccb959af4c94868f473f5a0";

// Staking (reward) address — used in the LEGACY / CIP-15 registration for key 3.
const STAKING_ADDRESS_HEX: &str = "e072b617657881e30ad17c46e4010c9cb3ebb2440653a34d32219c83e9";
// Payment address — used in the CIP-36 registration for key 3.
const PAYMENT_ADDRESS_HEX: &str = "004777561e7d9ec112ec307572faec1aff61ff0cfed68df4cd5c847f1872b617657881e30ad17c46e4010c9cb3ebb2440653a34d32219c83e9";

// Official signed preimages: {61284: <key_registration>} = `a119ef64` ++ body.
const LEGACY_PREIMAGE: &str = "a119ef64a40158200036ef3e1f0d3f5989e2d155ea54bdb2a72c4c456ccb959af4c94868f473f5a0025820e3cd2404c84de65f96918f18d5b445bcb933a7cda18eeded7945dd191e43236903581de072b617657881e30ad17c46e4010c9cb3ebb2440653a34d32219c83e9041904d2";
const CIP36_PREIMAGE: &str = "a119ef64a501818258200036ef3e1f0d3f5989e2d155ea54bdb2a72c4c456ccb959af4c94868f473f5a001025820e3cd2404c84de65f96918f18d5b445bcb933a7cda18eeded7945dd191e432369035839004777561e7d9ec112ec307572faec1aff61ff0cfed68df4cd5c847f1872b617657881e30ad17c46e4010c9cb3ebb2440653a34d32219c83e9041904d20500";

// Official blake2b-256 sign-hashes.
const LEGACY_HASH: &str = "9946e71b5f6c16150cf431910a0f7dbb8084a992577847802e60d32becb3d6be";
const CIP36_HASH: &str = "3110fbad72589a80de7fc174310e92dac35bbfece1690c2dce53c2235a9776fa";

// Official witness signatures (staking key over the sign-hash).
const LEGACY_WITNESS_SIG: &str = "a9ec8735804c6c4c5c4a02e9589c65508ec7060063b2d7dbeba82d1cbfa1b8be6b457f95d4ead5e8b454b989624fa44e0b89a64d089fdc0a6a1268fef4876d0f";
const CIP36_WITNESS_SIG: &str = "cbb96ba1596fafc18eec84e306feea3067ba1c6ace95b11af820bcbd53837ef32bdcf28176749061e1f2a1300d4df98c80582722786e40cf330072d0b78a7408";

// ---------------------------------------------------------------------------
// Fixtures (built from the official vector keys/addresses)
// ---------------------------------------------------------------------------

fn stake_cred() -> PublicKey {
    PublicKey::from_raw_bytes(&hex::decode(STAKING_PUB_HEX).unwrap()).unwrap()
}

fn voting_key() -> CIP36VotingPubKey {
    CIP36VotingPubKey::from_raw_bytes(&hex::decode(VOTE_PUB_HEX).unwrap()).unwrap()
}

fn staking_address() -> Address {
    Address::from_raw_bytes(&hex::decode(STAKING_ADDRESS_HEX).unwrap()).unwrap()
}

fn payment_address() -> Address {
    Address::from_raw_bytes(&hex::decode(PAYMENT_ADDRESS_HEX).unwrap()).unwrap()
}

fn zero_witness() -> CIP36RegistrationWitness {
    CIP36RegistrationWitness::new(Ed25519Signature::from_raw_bytes(&[0u8; 64]).unwrap())
}

/// Legacy (CIP-15) registration — single legacy voting key, staking address in
/// key 3, no voting_purpose on the wire.
fn legacy_reg() -> CIP36KeyRegistration {
    CIP36KeyRegistration::new(
        CIP36DelegationDistribution::new_legacy(
            CIP36LegacyKeyRegistration::from_raw_bytes(&hex::decode(VOTE_PUB_HEX).unwrap())
                .unwrap(),
        ),
        stake_cred(),
        staking_address(),
        NONCE,
    )
}

/// Weighted (CIP-36) registration whose delegation array has the given weights,
/// payment address in key 3.
fn weighted_reg(weights: &[u32]) -> CIP36KeyRegistration {
    let mut v =
        cml_core::non_empty::NonEmptyVec::new(CIP36Delegation::new(voting_key(), weights[0]));
    for w in &weights[1..] {
        v.push(CIP36Delegation::new(voting_key(), *w));
    }
    CIP36KeyRegistration::new(
        CIP36DelegationDistribution::new_weighted(v),
        stake_cred(),
        payment_address(),
        NONCE,
    )
}

fn reg_cbor(weights: &[u32]) -> CIP36RegistrationCbor {
    CIP36RegistrationCbor::new(weighted_reg(weights), zero_witness())
}

fn legacy_reg_cbor() -> CIP36RegistrationCbor {
    CIP36RegistrationCbor::new(legacy_reg(), zero_witness())
}

fn dereg() -> CIP36KeyDeregistration {
    CIP36KeyDeregistration::new(stake_cred(), NONCE)
}

// ===========================================================================
// 1. verify() semantics
//
// CIP-36 requires:
//   * "The delegation array is not empty"
//   * "The weights in the delegation array are not all zero"
// The check errors with `DelegationWeightsZero` iff *all* weights are zero.
// (This locks in the CORRECTED semantics: a previous inverted check
// `any(|d| d.weight != 0)` rejected every valid weighted registration.)
// ===========================================================================

#[test]
fn verify_all_weights_zero_single_is_err() {
    // single [0] -> all weights zero -> reject
    let err = reg_cbor(&[0]).verify().unwrap_err();
    assert!(matches!(err, CIP36Error::DelegationWeightsZero));
}

#[test]
fn verify_all_weights_zero_multiple_is_err() {
    // multiple [0, 0] -> all weights zero -> reject
    let err = reg_cbor(&[0, 0]).verify().unwrap_err();
    assert!(matches!(err, CIP36Error::DelegationWeightsZero));
}

#[test]
fn verify_mixed_weights_is_ok() {
    // [0, 1] -> not all zero -> accept (the exact case the old inverted check broke)
    reg_cbor(&[0, 1]).verify().unwrap();
}

#[test]
fn verify_single_nonzero_weight_is_ok() {
    // single [1] -> not all zero -> accept
    reg_cbor(&[1]).verify().unwrap();
}

#[test]
fn verify_legacy_is_always_ok() {
    // The weight rules only apply to the weighted variant; legacy has no weights.
    legacy_reg_cbor().verify().unwrap();
}

// ===========================================================================
// 2. verify() enforcement points
// ===========================================================================

#[test]
fn add_to_metadata_rejects_all_zero_weights() {
    // add_to_metadata calls verify() and maps failure -> InvalidStructure.
    let mut metadata = Metadata::new();
    let err = reg_cbor(&[0]).add_to_metadata(&mut metadata).unwrap_err();
    assert!(matches!(
        err.failure(),
        DeserializeFailure::InvalidStructure(_)
    ));
}

#[test]
fn add_to_metadata_accepts_valid_weights() {
    let mut metadata = Metadata::new();
    reg_cbor(&[1]).add_to_metadata(&mut metadata).unwrap();
}

#[test]
fn from_metadata_bytes_rejects_all_zero_weights() {
    // Hand-craft a full CIP-36 metadata map {61284: key_reg, 61285: witness}
    // whose weighted delegation has an all-zero weight. We serialize the
    // key_registration DIRECTLY (the generated `Serialize` does NOT run
    // verify()), so we can produce bytes that the public API would refuse to
    // emit, then confirm from_metadata_bytes catches the invariant on read.
    let meta = handcrafted_metadata(&weighted_reg(&[0]));

    let err = CIP36RegistrationCbor::from_metadata_bytes(&meta).unwrap_err();
    // verify() failure surfaces as InvalidStructure...
    assert!(matches!(
        err.failure(),
        DeserializeFailure::InvalidStructure(_)
    ));
    // ...annotated with the top-level struct name.
    assert!(err.to_string().contains("CIP36RegistrationCbor"));
    assert!(err.to_string().contains("Invalid delegation weights"));
}

#[test]
fn from_metadata_bytes_accepts_valid_weights() {
    // Same construction but with a nonzero weight round-trips cleanly.
    let meta = handcrafted_metadata(&weighted_reg(&[1]));
    CIP36RegistrationCbor::from_metadata_bytes(&meta).unwrap();
}

#[test]
#[should_panic]
fn to_metadata_bytes_on_invalid_struct_panics() {
    // QUIRK: unwrap in to_metadata_bytes. to_metadata_bytes() calls
    // serialize(...).unwrap(). serialize() runs verify() and, on failure,
    // returns a cbor_event error -> the unwrap panics instead of returning a
    // Result. Callers holding an invalid struct therefore hit a panic, not an
    // error. Characterized here so the migration notices if this hard-fail path
    // changes.
    let _ = reg_cbor(&[0]).to_metadata_bytes();
}

/// Build the CBOR of a metadata map {61284: key_reg, 61285: witness} directly
/// via the generated `Serialize` (which does not run verify()).
fn handcrafted_metadata(key_reg: &CIP36KeyRegistration) -> Vec<u8> {
    let mut buf = Serializer::new_vec();
    buf.write_map(cbor_event::Len::Len(2)).unwrap();
    buf.write_unsigned_integer(61284).unwrap();
    key_reg.serialize(&mut buf, false).unwrap();
    buf.write_unsigned_integer(61285).unwrap();
    zero_witness().serialize(&mut buf, false).unwrap();
    buf.finalize()
}

// ===========================================================================
// 3. voting_purpose  `.default 0`  wire semantics
//
// The `? 5 : CIP36_voting_purpose .default 0` field is represented as a plain
// (non-Option) `u64` that defaults to 0 in the constructor. The generated code
// for CIP36KeyRegistration is hand-edited (cddl-codegen:replace blocks) so that
// for the WEIGHTED variant the key is emitted whenever `voting_purpose != 0` OR
// `voting_purpose_default_present` — and for a freshly-constructed struct
// (encodings == None) that flag defaults to `true` (`unwrap_or(true)`).
//
// Net observed behavior:
//   * fresh WEIGHTED reg  (voting_purpose == 0) -> key 5 IS written as `5: 0`
//   * fresh LEGACY   reg                        -> key 5 is NEVER written
//   * nonzero voting_purpose                    -> key 5 always written
//   * force_canonical does NOT drop the defaulted key 5 (weighted)
//
// The official CIP-36 vector agrees: its cip36 body ends in `...041904d20500`,
// i.e. `5: 0` is EXPLICIT on the wire even though it equals the `.default`.
// ===========================================================================

// Golden: fresh weighted reg, default voting_purpose == 0. Body of CIP36_PREIMAGE
// after the `a119ef64` map/label prefix. Trailing `0500` = key 5 -> value 0.
const WR_DEFAULT_CBOR: &str = "a501818258200036ef3e1f0d3f5989e2d155ea54bdb2a72c4c456ccb959af4c94868f473f5a001025820e3cd2404c84de65f96918f18d5b445bcb933a7cda18eeded7945dd191e432369035839004777561e7d9ec112ec307572faec1aff61ff0cfed68df4cd5c847f1872b617657881e30ad17c46e4010c9cb3ebb2440653a34d32219c83e9041904d20500";

// Golden: fresh weighted reg, voting_purpose == 5. Trailing `0505` = key 5 -> 5.
const WR_NONZERO_CBOR: &str = "a501818258200036ef3e1f0d3f5989e2d155ea54bdb2a72c4c456ccb959af4c94868f473f5a001025820e3cd2404c84de65f96918f18d5b445bcb933a7cda18eeded7945dd191e432369035839004777561e7d9ec112ec307572faec1aff61ff0cfed68df4cd5c847f1872b617657881e30ad17c46e4010c9cb3ebb2440653a34d32219c83e9041904d20505";

// Golden: fresh legacy reg. 4 map entries (header a4), ends at nonce `1904d2`,
// no key 5 at all.
const LR_DEFAULT_CBOR: &str = "a40158200036ef3e1f0d3f5989e2d155ea54bdb2a72c4c456ccb959af4c94868f473f5a0025820e3cd2404c84de65f96918f18d5b445bcb933a7cda18eeded7945dd191e43236903581de072b617657881e30ad17c46e4010c9cb3ebb2440653a34d32219c83e9041904d2";

#[test]
fn voting_purpose_default_present_on_fresh_weighted() {
    // (a) constructor default (voting_purpose == 0) -> key 5 present in output.
    let wr = weighted_reg(&[1]);
    assert_eq!(wr.voting_purpose, 0); // constructor default
    // QUIRK-WATCH: default-field behavior may change with codegen migration.
    // A defaulted optional field is emitted here (`5: 0`) rather than omitted;
    // a stock codegen would typically OMIT a value equal to its `.default`.
    // (This matches the official vector, which also encodes `5: 0` explicitly.)
    assert_eq!(hex::encode(wr.to_cbor_bytes()), WR_DEFAULT_CBOR);
    // The map header is `a5` (5 entries) and the encoding ends in `0500`.
    assert!(hex::encode(wr.to_cbor_bytes()).starts_with("a5"));
    assert!(hex::encode(wr.to_cbor_bytes()).ends_with("0500"));
}

#[test]
fn voting_purpose_canonical_keeps_defaulted_key_weighted() {
    // (b-canonical, fresh) QUIRK-WATCH: default-field behavior may change with
    // codegen migration. to_canonical_cbor_bytes does NOT drop the defaulted
    // key 5 for the weighted variant — canonical output is byte-identical to
    // the non-canonical output here.
    let wr = weighted_reg(&[1]);
    assert_eq!(hex::encode(wr.to_canonical_cbor_bytes()), WR_DEFAULT_CBOR);
}

#[test]
fn voting_purpose_roundtrip_preserves_explicit_default() {
    // (b) wire bytes WITH explicit `5: 0` -> deserialize.
    let bytes = hex::decode(WR_DEFAULT_CBOR).unwrap();
    let de = CIP36KeyRegistration::from_cbor_bytes(&bytes).unwrap();
    // accessor returns the defaulted value 0
    assert_eq!(de.voting_purpose, 0);
    // QUIRK-WATCH: default-field behavior may change with codegen migration.
    // re-serialize preserves key 5 (encoding preservation via
    // voting_purpose_default_present)...
    assert_eq!(hex::encode(de.to_cbor_bytes()), WR_DEFAULT_CBOR);
    // ...and to_canonical_cbor_bytes ALSO keeps it (does not drop the default).
    assert_eq!(hex::encode(de.to_canonical_cbor_bytes()), WR_DEFAULT_CBOR);
}

#[test]
fn voting_purpose_nonzero_always_present() {
    // (c) nonzero voting_purpose -> key 5 always present.
    let mut wr = weighted_reg(&[1]);
    wr.voting_purpose = 5;
    assert_eq!(hex::encode(wr.to_cbor_bytes()), WR_NONZERO_CBOR);
    assert_eq!(hex::encode(wr.to_canonical_cbor_bytes()), WR_NONZERO_CBOR);
}

#[test]
fn voting_purpose_omitted_for_legacy() {
    // Legacy variant never writes voting_purpose (hand-edit forces false).
    let lr = legacy_reg();
    assert_eq!(hex::encode(lr.to_cbor_bytes()), LR_DEFAULT_CBOR);
    // Only 4 map entries (`a4`), no key 5.
    assert!(hex::encode(lr.to_cbor_bytes()).starts_with("a4"));
}

// ===========================================================================
// 4. hash_to_sign goldens
// ===========================================================================

// Deregistration goldens are NEW (fills the "// TODO: deregistration test?"
// gap). Built from the same stake key + nonce 1234; both canonical flags agree.
const DEREG_HASH: &str = "fad05070aaa93d064dcf3c9b2f27e566e614648efcf691f1cbf75f0a9e65672c";
// Hash of a registration DESERIALIZED from a non-canonical (indefinite-length
// outer map) encoding, serialized non-canonically. Differs from CIP36_HASH.
const CIP36_HASH_NONCANONICAL: &str =
    "d51d877888a3f6d374a537ccf5daf9af3cc036fdc253f9dccaa8bd5481f4d913";

#[test]
fn hash_to_sign_registration_legacy() {
    let lr = legacy_reg();
    // Official vector, force_canonical == false.
    assert_eq!(hex::encode(lr.hash_to_sign(false).unwrap()), LEGACY_HASH);
    // NEW: force_canonical == true golden (equal for this fresh struct).
    assert_eq!(hex::encode(lr.hash_to_sign(true).unwrap()), LEGACY_HASH);
}

#[test]
fn hash_to_sign_registration_weighted() {
    let wr = weighted_reg(&[1]);
    // Official vector, force_canonical == false.
    assert_eq!(hex::encode(wr.hash_to_sign(false).unwrap()), CIP36_HASH);
    // NEW: force_canonical == true golden (equal for this fresh struct).
    assert_eq!(hex::encode(wr.hash_to_sign(true).unwrap()), CIP36_HASH);
}

#[test]
fn hash_to_sign_deregistration() {
    // NEW vectors: no official CIP-36 test vectors exist for deregistration.
    let dr = dereg();
    assert_eq!(hex::encode(dr.hash_to_sign(false).unwrap()), DEREG_HASH);
    assert_eq!(hex::encode(dr.hash_to_sign(true).unwrap()), DEREG_HASH);
}

#[test]
fn hash_to_sign_canonical_equal_for_fresh_struct() {
    // Characterized behavior: for a freshly-constructed struct the canonical and
    // non-canonical encodings are byte-identical, so their sign-hashes match.
    let wr = weighted_reg(&[1]);
    assert_eq!(
        wr.hash_to_sign(false).unwrap(),
        wr.hash_to_sign(true).unwrap()
    );
    let dr = dereg();
    assert_eq!(
        dr.hash_to_sign(false).unwrap(),
        dr.hash_to_sign(true).unwrap()
    );
}

#[test]
fn hash_to_sign_canonical_differs_for_noncanonical_deser() {
    // Build a non-canonical encoding of the weighted key_registration by
    // rewriting the outer map header from definite (`a5`) to indefinite (`bf`)
    // and appending a break (`ff`). The preserved encoding makes the
    // non-canonical hash differ from the canonical one (which normalizes it).
    let mut nc = weighted_reg(&[1]).to_cbor_bytes();
    assert_eq!(nc[0], 0xa5); // 5-entry definite map header
    nc[0] = 0xbf; // indefinite map
    nc.push(0xff); // break
    let de = CIP36KeyRegistration::from_cbor_bytes(&nc).unwrap();

    // non-canonical serialization preserves the indefinite map -> distinct hash
    assert_eq!(
        hex::encode(de.hash_to_sign(false).unwrap()),
        CIP36_HASH_NONCANONICAL
    );
    // canonical serialization normalizes back to the canonical weighted hash
    assert_eq!(hex::encode(de.hash_to_sign(true).unwrap()), CIP36_HASH);
    // and the two differ
    assert_ne!(
        de.hash_to_sign(false).unwrap(),
        de.hash_to_sign(true).unwrap()
    );
}

// ===========================================================================
// 5. hash_to_sign structural check
//
// The signed preimage is a single-entry map {label: body}, with label 61284 for
// registration and 61286 for deregistration, blake2b256-hashed. Confirm against
// the OFFICIAL published preimages, and reproduce the shape manually.
// ===========================================================================

#[test]
fn hash_to_sign_matches_official_preimage_legacy() {
    // blake2b256(official preimage) == official hash
    let preimage = hex::decode(LEGACY_PREIMAGE).unwrap();
    assert_eq!(hex::encode(cml_crypto::blake2b256(&preimage)), LEGACY_HASH);
    // and hash_to_sign on the vector-built struct reproduces the hash
    assert_eq!(
        hex::encode(legacy_reg().hash_to_sign(false).unwrap()),
        LEGACY_HASH
    );
}

#[test]
fn hash_to_sign_matches_official_preimage_cip36() {
    let preimage = hex::decode(CIP36_PREIMAGE).unwrap();
    assert_eq!(hex::encode(cml_crypto::blake2b256(&preimage)), CIP36_HASH);
    assert_eq!(
        hex::encode(weighted_reg(&[1]).hash_to_sign(false).unwrap()),
        CIP36_HASH
    );
}

#[test]
fn hash_to_sign_preimage_shape_registration() {
    // Manually reconstruct {61284: body} and confirm it matches both
    // hash_to_sign and the official preimage bytes.
    let wr = weighted_reg(&[1]);
    let mut buf = Serializer::new_vec();
    buf.write_map(cbor_event::Len::Len(1)).unwrap();
    buf.write_unsigned_integer(61284).unwrap(); // KEY_REGISTRATION_LABEL
    wr.serialize(&mut buf, false).unwrap();
    let preimage = buf.finalize();

    assert_eq!(hex::encode(&preimage), CIP36_PREIMAGE);
    assert_eq!(
        cml_crypto::blake2b256(&preimage).to_vec(),
        wr.hash_to_sign(false).unwrap()
    );
}

#[test]
fn hash_to_sign_preimage_shape_deregistration() {
    // Deregistration label is 61286; no official vector, so assert against the
    // manually-reconstructed preimage and the locked-in DEREG_HASH.
    let dr = dereg();
    let mut buf = Serializer::new_vec();
    buf.write_map(cbor_event::Len::Len(1)).unwrap();
    buf.write_unsigned_integer(61286).unwrap(); // KEY_DEREGISTRATION_LABEL
    dr.serialize(&mut buf, false).unwrap();
    let preimage = buf.finalize();

    assert_eq!(
        cml_crypto::blake2b256(&preimage).to_vec(),
        dr.hash_to_sign(false).unwrap()
    );
    assert_eq!(hex::encode(cml_crypto::blake2b256(&preimage)), DEREG_HASH);
}

// ===========================================================================
// 6. End-to-end signing / verification against the official witnesses
// ===========================================================================

#[test]
fn official_witness_verifies_legacy() {
    // (a) the staking public key verifies the official witness signature over
    // the sign-hash.
    let pub_key = PublicKey::from_raw_bytes(&hex::decode(STAKING_PUB_HEX).unwrap()).unwrap();
    let sig = Ed25519Signature::from_raw_bytes(&hex::decode(LEGACY_WITNESS_SIG).unwrap()).unwrap();
    let hash = legacy_reg().hash_to_sign(false).unwrap();
    assert!(pub_key.verify(&hash, &sig));
}

#[test]
fn official_witness_verifies_cip36() {
    let pub_key = PublicKey::from_raw_bytes(&hex::decode(STAKING_PUB_HEX).unwrap()).unwrap();
    let sig = Ed25519Signature::from_raw_bytes(&hex::decode(CIP36_WITNESS_SIG).unwrap()).unwrap();
    let hash = weighted_reg(&[1]).hash_to_sign(false).unwrap();
    assert!(pub_key.verify(&hash, &sig));
}

#[test]
fn signing_reproduces_official_witness() {
    // (b) Ed25519 is deterministic: signing the sign-hash with the staking
    // private key reproduces the official witness signatures exactly.
    let prv = PrivateKey::from_normal_bytes(&hex::decode(STAKING_PRV_HEX).unwrap()).unwrap();
    // sanity: this private key corresponds to the staking public key
    assert_eq!(hex::encode(prv.to_public().to_raw_bytes()), STAKING_PUB_HEX);

    let legacy_hash = legacy_reg().hash_to_sign(false).unwrap();
    assert_eq!(
        hex::encode(prv.sign(&legacy_hash).to_raw_bytes()),
        LEGACY_WITNESS_SIG
    );

    let cip36_hash = weighted_reg(&[1]).hash_to_sign(false).unwrap();
    assert_eq!(
        hex::encode(prv.sign(&cip36_hash).to_raw_bytes()),
        CIP36_WITNESS_SIG
    );
}
