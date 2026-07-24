//! Characterization tests locking in the EXACT current serde_json output of the cml-cip36
//! generated types. These are a migration regression net: a pending cddl-codegen bump replaces
//! this crate's generated code and may change JSON generation. Any change to the wire-visible
//! JSON shape (enum tagging, field rendering, defaulted fields, etc.) must surface here as a
//! failing string-equality assertion, to be reviewed deliberately.
//!
//! Notable current behaviors captured below (see `// QUIRK:` tags):
//!   * `CIP36DelegationDistribution` is serialized with serde's DEFAULT external tagging, so the
//!     variant name (`Weighted` / `Legacy`) becomes a wrapping JSON object key, and the inner
//!     struct-variant field (`weighted` / `legacy`) is nested one level deeper.
//!   * `voting_purpose` has NO `skip_serializing_if`, so it is ALWAYS emitted, including when it
//!     defaulted to 0 (there is no "absent" JSON form).
//!   * `PublicKey` renders as a bech32 `ed25519_pk1…` string; `Ed25519Signature` renders as raw
//!     HEX (not bech32); `Address` renders as a bech32 string.
//!   * `NonEmptyVec` renders as a plain JSON array (no wrapper).
//!   * The per-field `encodings` / `*_encoding` members are `#[serde(skip)]`, so CBOR encoding
//!     metadata never leaks into JSON.

use cml_chain::address::Address;
use cml_cip36::*;
use cml_core::serialization::{Deserialize, RawBytesEncoding, Serialize};
use cml_crypto::{Ed25519Signature, PublicKey};

// ---------------------------------------------------------------------------
// Fixtures (byte vectors reused from the crate's `sign_data` unit test)
// ---------------------------------------------------------------------------

fn stake_cred() -> PublicKey {
    PublicKey::from_raw_bytes(&[
        227, 205, 36, 4, 200, 77, 230, 95, 150, 145, 143, 24, 213, 180, 69, 188, 185, 51, 167, 205,
        161, 142, 237, 237, 121, 69, 221, 25, 30, 67, 35, 105,
    ])
    .unwrap()
}

fn voting_key() -> PublicKey {
    PublicKey::from_raw_bytes(&[
        0, 54, 239, 62, 31, 13, 63, 89, 137, 226, 209, 85, 234, 84, 189, 178, 167, 44, 76, 69, 108,
        203, 149, 154, 244, 201, 72, 104, 244, 115, 245, 160,
    ])
    .unwrap()
}

// QUIRK: this fixture is actually a 29-byte reward/stake address (header 0xe0), so it renders as
// a `stake_test1…` bech32 string rather than an `addr…` one. Captured as-is from the unit test.
fn legacy_address() -> Address {
    Address::from_raw_bytes(&[
        224, 114, 182, 23, 101, 120, 129, 227, 10, 209, 124, 70, 228, 1, 12, 156, 179, 235, 178,
        68, 6, 83, 163, 77, 50, 33, 156, 131, 233,
    ])
    .unwrap()
}

fn new_address() -> Address {
    Address::from_raw_bytes(&[
        0, 71, 119, 86, 30, 125, 158, 193, 18, 236, 48, 117, 114, 250, 236, 26, 255, 97, 255, 12,
        254, 214, 141, 244, 205, 92, 132, 127, 24, 114, 182, 23, 101, 120, 129, 227, 10, 209, 124,
        70, 228, 1, 12, 156, 179, 235, 178, 68, 6, 83, 163, 77, 50, 33, 156, 131, 233,
    ])
    .unwrap()
}

// Official CIP-36 test-vector witness signature (61285 -> "1"), so the witness JSON snapshots are
// directly comparable with the other cml-cip36 test files that use the same vector.
fn sig() -> Ed25519Signature {
    Ed25519Signature::from_raw_bytes(
        &hex::decode(
            "cbb96ba1596fafc18eec84e306feea3067ba1c6ace95b11af820bcbd53837ef32bdcf28176749061e1f2a1300d4df98c80582722786e40cf330072d0b78a7408",
        )
        .unwrap(),
    )
    .unwrap()
}

fn weighted_reg() -> CIP36KeyRegistration {
    CIP36KeyRegistration::new(
        CIP36DelegationDistribution::new_weighted(cml_core::non_empty::NonEmptyVec::new(
            CIP36Delegation::new(voting_key(), 1),
        )),
        stake_cred(),
        new_address(),
        1234,
    )
}

fn legacy_reg() -> CIP36KeyRegistration {
    CIP36KeyRegistration::new(
        CIP36DelegationDistribution::new_legacy(voting_key()),
        stake_cred(),
        legacy_address(),
        1234,
    )
}

// ---------------------------------------------------------------------------
// 1. GOLDEN JSON snapshots (exact string equality)
// ---------------------------------------------------------------------------

// QUIRK: external tagging -> `"delegation": { "Weighted": { "weighted": [ … ] } }`.
// QUIRK: `voting_purpose` emitted even though it defaulted to 0.
const WEIGHTED_REG_JSON: &str = r#"{
  "delegation": {
    "Weighted": {
      "weighted": [
        {
          "voting_pub_key": "ed25519_pk1qqmw70slp5l4nz0z6927549ak2njcnz9dn9etxh5e9yx3arn7ksq2u27y7",
          "weight": 1
        }
      ]
    }
  },
  "stake_credential": "ed25519_pk1u0xjgpxgfhn9l9533uvdtdz9hjun8f7d5x8wmmteghw3j8jryd5s98t7t5",
  "payment_address": "addr_test1qprhw4s70k0vzyhvxp6h97hvrtlkrlcvlmtgmaxdtjz87xrjkctk27ypuv9dzlzxusqse89naweygpjn5dxnygvus05sdq9h07",
  "nonce": 1234,
  "voting_purpose": 0
}"#;

// QUIRK: external tagging -> `"delegation": { "Legacy": { "legacy": "ed25519_pk1…" } }`.
const LEGACY_REG_JSON: &str = r#"{
  "delegation": {
    "Legacy": {
      "legacy": "ed25519_pk1qqmw70slp5l4nz0z6927549ak2njcnz9dn9etxh5e9yx3arn7ksq2u27y7"
    }
  },
  "stake_credential": "ed25519_pk1u0xjgpxgfhn9l9533uvdtdz9hjun8f7d5x8wmmteghw3j8jryd5s98t7t5",
  "payment_address": "stake_test1upetv9m90zq7xzk303rwgqgvnje7hvjyqef6xnfjyxwg86gzpmj80",
  "nonce": 1234,
  "voting_purpose": 0
}"#;

const WEIGHTED_REG_VP5_JSON: &str = r#"{
  "delegation": {
    "Weighted": {
      "weighted": [
        {
          "voting_pub_key": "ed25519_pk1qqmw70slp5l4nz0z6927549ak2njcnz9dn9etxh5e9yx3arn7ksq2u27y7",
          "weight": 1
        }
      ]
    }
  },
  "stake_credential": "ed25519_pk1u0xjgpxgfhn9l9533uvdtdz9hjun8f7d5x8wmmteghw3j8jryd5s98t7t5",
  "payment_address": "addr_test1qprhw4s70k0vzyhvxp6h97hvrtlkrlcvlmtgmaxdtjz87xrjkctk27ypuv9dzlzxusqse89naweygpjn5dxnygvus05sdq9h07",
  "nonce": 1234,
  "voting_purpose": 5
}"#;

// QUIRK: `Ed25519Signature` serializes to raw HEX, not bech32.
const WITNESS_JSON: &str = r#"{
  "stake_witness": "cbb96ba1596fafc18eec84e306feea3067ba1c6ace95b11af820bcbd53837ef32bdcf28176749061e1f2a1300d4df98c80582722786e40cf330072d0b78a7408"
}"#;

const KEY_DEREG_JSON: &str = r#"{
  "stake_credential": "ed25519_pk1u0xjgpxgfhn9l9533uvdtdz9hjun8f7d5x8wmmteghw3j8jryd5s98t7t5",
  "nonce": 1234,
  "voting_purpose": 0
}"#;

const KEY_DEREG_VP7_JSON: &str = r#"{
  "stake_credential": "ed25519_pk1u0xjgpxgfhn9l9533uvdtdz9hjun8f7d5x8wmmteghw3j8jryd5s98t7t5",
  "nonce": 1234,
  "voting_purpose": 7
}"#;

const REG_CBOR_JSON: &str = r#"{
  "key_registration": {
    "delegation": {
      "Weighted": {
        "weighted": [
          {
            "voting_pub_key": "ed25519_pk1qqmw70slp5l4nz0z6927549ak2njcnz9dn9etxh5e9yx3arn7ksq2u27y7",
            "weight": 1
          }
        ]
      }
    },
    "stake_credential": "ed25519_pk1u0xjgpxgfhn9l9533uvdtdz9hjun8f7d5x8wmmteghw3j8jryd5s98t7t5",
    "payment_address": "addr_test1qprhw4s70k0vzyhvxp6h97hvrtlkrlcvlmtgmaxdtjz87xrjkctk27ypuv9dzlzxusqse89naweygpjn5dxnygvus05sdq9h07",
    "nonce": 1234,
    "voting_purpose": 0
  },
  "registration_witness": {
    "stake_witness": "cbb96ba1596fafc18eec84e306feea3067ba1c6ace95b11af820bcbd53837ef32bdcf28176749061e1f2a1300d4df98c80582722786e40cf330072d0b78a7408"
  }
}"#;

const DEREG_CBOR_JSON: &str = r#"{
  "key_deregistration": {
    "stake_credential": "ed25519_pk1u0xjgpxgfhn9l9533uvdtdz9hjun8f7d5x8wmmteghw3j8jryd5s98t7t5",
    "nonce": 1234,
    "voting_purpose": 0
  },
  "deregistration_witness": {
    "stake_witness": "cbb96ba1596fafc18eec84e306feea3067ba1c6ace95b11af820bcbd53837ef32bdcf28176749061e1f2a1300d4df98c80582722786e40cf330072d0b78a7408"
  }
}"#;

fn nonzero_reg() -> CIP36KeyRegistration {
    let mut r = weighted_reg();
    r.voting_purpose = 5;
    r
}

fn key_dereg() -> CIP36KeyDeregistration {
    CIP36KeyDeregistration::new(stake_cred(), 1234)
}

fn key_dereg_vp7() -> CIP36KeyDeregistration {
    let mut d = key_dereg();
    d.voting_purpose = 7;
    d
}

fn reg_cbor() -> CIP36RegistrationCbor {
    CIP36RegistrationCbor::new(weighted_reg(), CIP36RegistrationWitness::new(sig()))
}

fn dereg_cbor() -> CIP36DeregistrationCbor {
    CIP36DeregistrationCbor::new(key_dereg(), CIP36DeregistrationWitness::new(sig()))
}

#[test]
fn golden_weighted_registration() {
    assert_eq!(
        serde_json::to_string_pretty(&weighted_reg()).unwrap(),
        WEIGHTED_REG_JSON
    );
}

#[test]
fn golden_legacy_registration() {
    assert_eq!(
        serde_json::to_string_pretty(&legacy_reg()).unwrap(),
        LEGACY_REG_JSON
    );
}

#[test]
fn golden_registration_nonzero_voting_purpose() {
    assert_eq!(
        serde_json::to_string_pretty(&nonzero_reg()).unwrap(),
        WEIGHTED_REG_VP5_JSON
    );
}

#[test]
fn golden_registration_witness() {
    assert_eq!(
        serde_json::to_string_pretty(&CIP36RegistrationWitness::new(sig())).unwrap(),
        WITNESS_JSON
    );
}

#[test]
fn golden_key_deregistration_default_voting_purpose() {
    assert_eq!(
        serde_json::to_string_pretty(&key_dereg()).unwrap(),
        KEY_DEREG_JSON
    );
}

#[test]
fn golden_key_deregistration_nonzero_voting_purpose() {
    assert_eq!(
        serde_json::to_string_pretty(&key_dereg_vp7()).unwrap(),
        KEY_DEREG_VP7_JSON
    );
}

#[test]
fn golden_deregistration_witness() {
    assert_eq!(
        serde_json::to_string_pretty(&CIP36DeregistrationWitness::new(sig())).unwrap(),
        WITNESS_JSON
    );
}

#[test]
fn golden_registration_cbor() {
    assert_eq!(
        serde_json::to_string_pretty(&reg_cbor()).unwrap(),
        REG_CBOR_JSON
    );
}

#[test]
fn golden_deregistration_cbor() {
    assert_eq!(
        serde_json::to_string_pretty(&dereg_cbor()).unwrap(),
        DEREG_CBOR_JSON
    );
}

// ---------------------------------------------------------------------------
// 2. JSON round-trips (from_str -> re-serialize -> identical string)
//    plus CBOR-byte equality after a JSON round trip.
// ---------------------------------------------------------------------------

fn json_roundtrip_stable<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let json = serde_json::to_string_pretty(value).unwrap();
    let back: T = serde_json::from_str(&json).unwrap();
    let json2 = serde_json::to_string_pretty(&back).unwrap();
    assert_eq!(json, json2, "JSON round-trip changed the output string");
}

#[test]
fn json_roundtrip_weighted_registration() {
    json_roundtrip_stable(&weighted_reg());
}

#[test]
fn json_roundtrip_legacy_registration() {
    json_roundtrip_stable(&legacy_reg());
}

#[test]
fn json_roundtrip_key_deregistration() {
    json_roundtrip_stable(&key_dereg_vp7());
}

#[test]
fn json_roundtrip_registration_witness() {
    json_roundtrip_stable(&CIP36RegistrationWitness::new(sig()));
}

#[test]
fn json_roundtrip_registration_cbor() {
    json_roundtrip_stable(&reg_cbor());
}

#[test]
fn json_roundtrip_deregistration_cbor() {
    json_roundtrip_stable(&dereg_cbor());
}

// After a JSON round trip the reconstructed struct must still encode to the same CBOR bytes as
// the original (JSON carries no encoding metadata, so both take the default/canonical encoding).
#[test]
fn cbor_bytes_stable_across_json_roundtrip() {
    let original = weighted_reg();
    let original_cbor = original.to_cbor_bytes();
    let json = serde_json::to_string(&original).unwrap();
    let back: CIP36KeyRegistration = serde_json::from_str(&json).unwrap();
    assert_eq!(original_cbor, back.to_cbor_bytes());
}

#[test]
fn cbor_bytes_stable_across_json_roundtrip_deregistration() {
    let original = key_dereg_vp7();
    let original_cbor = original.to_cbor_bytes();
    let json = serde_json::to_string(&original).unwrap();
    let back: CIP36KeyDeregistration = serde_json::from_str(&json).unwrap();
    assert_eq!(original_cbor, back.to_cbor_bytes());
}

// ---------------------------------------------------------------------------
// 3. JSON of a struct DESERIALIZED FROM CBOR must equal the constructor-built JSON.
//    (CBOR encoding metadata must NOT leak into JSON.)
// ---------------------------------------------------------------------------

#[test]
fn json_from_cbor_deserialized_matches_constructor() {
    // Round-trip the key registration through its own CBOR bytes so the reconstructed struct
    // carries `encodings: Some(..)` rather than the constructor's `None`.
    let built = weighted_reg();
    let from_cbor = CIP36KeyRegistration::from_cbor_bytes(&built.to_cbor_bytes()).unwrap();
    assert_eq!(
        serde_json::to_string_pretty(&built).unwrap(),
        serde_json::to_string_pretty(&from_cbor).unwrap(),
        "CBOR encoding metadata leaked into JSON"
    );
    // And it still equals the golden snapshot.
    assert_eq!(
        serde_json::to_string_pretty(&from_cbor).unwrap(),
        WEIGHTED_REG_JSON
    );
}

#[test]
fn json_from_metadata_cbor_deserialized_matches_constructor() {
    // Build via the CIP36 metadata-bytes path (a full RegistrationCbor view) and confirm the
    // JSON is byte-identical to the constructor-built value's JSON.
    let built = reg_cbor();
    let metadata_bytes = built.to_metadata_bytes();
    let from_meta = CIP36RegistrationCbor::from_metadata_bytes(&metadata_bytes).unwrap();
    assert_eq!(
        serde_json::to_string_pretty(&built).unwrap(),
        serde_json::to_string_pretty(&from_meta).unwrap(),
        "metadata encoding leaked into JSON"
    );
    assert_eq!(
        serde_json::to_string_pretty(&from_meta).unwrap(),
        REG_CBOR_JSON
    );
}

// The official CIP-36 test-vector key_registration body carries `voting_purpose` as EXPLICIT wire
// bytes `0500` (map key 5 -> 0), not an omitted/defaulted field. Deserializing it and snapshotting
// the JSON proves an explicit-on-the-wire 0 renders identically to the constructor's implicit 0 —
// i.e. the JSON has no way to distinguish "explicit 0" from "defaulted 0".
#[test]
fn json_from_cbor_explicit_zero_voting_purpose() {
    // 61284 body from the vector (a5 .. 0500); the trailing `0500` is voting_purpose = 0 explicit.
    let cbor = hex::decode(
        "a501818258200036ef3e1f0d3f5989e2d155ea54bdb2a72c4c456ccb959af4c94868f473f5a001025820e3cd2404c84de65f96918f18d5b445bcb933a7cda18eeded7945dd191e432369035839004777561e7d9ec112ec307572faec1aff61ff0cfed68df4cd5c847f1872b617657881e30ad17c46e4010c9cb3ebb2440653a34d32219c83e9041904d20500",
    )
    .unwrap();
    let from_cbor = CIP36KeyRegistration::from_cbor_bytes(&cbor).unwrap();
    assert_eq!(from_cbor.voting_purpose, 0);
    assert_eq!(
        serde_json::to_string_pretty(&from_cbor).unwrap(),
        WEIGHTED_REG_JSON
    );
}

// ---------------------------------------------------------------------------
// 4. schemars JsonSchema snapshot (catches schema-generation changes on the codegen bump).
// ---------------------------------------------------------------------------

// QUIRK: `CIP36DelegationDistribution` schema uses `oneOf` with `additionalProperties:false`
// wrappers keyed by the variant name — the schema-side mirror of external tagging.
const KEY_REGISTRATION_SCHEMA_JSON: &str = r##"{"$schema":"https://json-schema.org/draft/2020-12/schema","title":"CIP36KeyRegistration","type":"object","properties":{"delegation":{"$ref":"#/$defs/CIP36DelegationDistribution"},"nonce":{"description":"Monotonically rising across all transactions with the same staking key. Recommended to just use the slot of this tx.","type":"integer","format":"uint64","minimum":0},"payment_address":{"type":"string"},"stake_credential":{"$ref":"#/$defs/PublicKey"},"voting_purpose":{"type":"integer","format":"uint64","minimum":0}},"required":["delegation","stake_credential","payment_address","nonce","voting_purpose"],"$defs":{"CIP36Delegation":{"description":"Weighted delegation input.\nThis is the proportion of weight to assign to this public key relative to the weights\nof all other Delegations where this is used.","type":"object","properties":{"voting_pub_key":{"$ref":"#/$defs/PublicKey"},"weight":{"type":"integer","format":"uint32","minimum":0}},"required":["voting_pub_key","weight"]},"CIP36DelegationDistribution":{"oneOf":[{"type":"object","properties":{"Weighted":{"type":"object","properties":{"weighted":{"type":"array","items":{"$ref":"#/$defs/CIP36Delegation"}}},"required":["weighted"]}},"additionalProperties":false,"required":["Weighted"]},{"type":"object","properties":{"Legacy":{"type":"object","properties":{"legacy":{"$ref":"#/$defs/PublicKey"}},"required":["legacy"]}},"additionalProperties":false,"required":["Legacy"]}]},"PublicKey":{"description":"ED25519 key used as public key","type":"string"}}}"##;

#[test]
fn golden_key_registration_schema() {
    let schema = schemars::schema_for!(CIP36KeyRegistration);
    assert_eq!(
        serde_json::to_string(&schema).unwrap(),
        KEY_REGISTRATION_SCHEMA_JSON
    );
}
