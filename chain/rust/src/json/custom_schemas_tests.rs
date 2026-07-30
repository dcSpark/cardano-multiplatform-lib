//! Round-trip validation of the hand-authored schemas in `src/json/custom_schemas/`.
//!
//! Those schemas exist because `PlutusData` and `TransactionMetadatum` publish a hand-written
//! serde encoding (the cardano-node formats), so nothing derivable describes them — and before
//! this test, nothing validated them either: two of `PlutusData`'s `oneOf` arms shipped wrong
//! for years because no sample ever exercised them. Every arm of both schemas is exercised
//! here against `serde_json::to_value` of real values, which is exactly the JSON the schema
//! claims to describe.
//!
//! The wasm `to_json_value()` route is a SECOND, independent serializer (serde-wasm-bindgen), so
//! it is not covered here and must not be assumed to agree — it did not, on every value, for as
//! long as an earlier version of this comment claimed it did. `chain/wasm/tests/json_value_api.rs`
//! is what holds the two routes together; this file validates the shape they are supposed to share.

use crate::auxdata::{MetadatumMap, TransactionMetadatum};
use crate::plutus::{ConstrPlutusData, PlutusData, PlutusMap};
use crate::utils::BigInteger;
use cml_core::Int;
use core::str::FromStr;

/// Wrap the schema files exactly the way the exported schema document embeds them: as `$defs`
/// entries named by their types, so the files' internal `#/$defs/<name>` references resolve
/// here the same way they do in the shipped document. All three files are always present,
/// mirroring the document (PlutusData's constructor arm references ConstrPlutusData).
fn validator(name: &str) -> jsonschema::Validator {
    let parse = |body: &str| -> serde_json::Value { serde_json::from_str(body).unwrap() };
    let wrapped = serde_json::json!({
        "$defs": {
            "PlutusData": parse(include_str!("custom_schemas/PlutusData.json")),
            "ConstrPlutusData": parse(include_str!("custom_schemas/ConstrPlutusData.json")),
            "TransactionMetadatum": parse(include_str!("custom_schemas/TransactionMetadatum.json")),
        },
        "$ref": format!("#/$defs/{name}"),
    });
    jsonschema::validator_for(&wrapped).unwrap()
}

fn assert_valid(validator: &jsonschema::Validator, value: &impl serde::Serialize, what: &str) {
    let json = serde_json::to_value(value).unwrap();
    if let Err(error) = validator.validate(&json) {
        panic!("{what}: schema rejects real serde output {json}: {error}");
    }
}

fn plutus_validator() -> jsonschema::Validator {
    validator("PlutusData")
}

fn metadatum_validator() -> jsonschema::Validator {
    validator("TransactionMetadatum")
}

fn constr_validator() -> jsonschema::Validator {
    validator("ConstrPlutusData")
}

#[test]
fn plutus_data_schema_accepts_every_arm() {
    let v = plutus_validator();

    let bytes = PlutusData::new_bytes(vec![0xCA, 0xFE, 0xF0, 0x0D]);
    let small_int = PlutusData::new_integer(BigInteger::from_str("5").unwrap());
    let negative_int = PlutusData::new_integer(BigInteger::from_str("-123456789").unwrap());
    // Past u64::MAX: the int arm must accept what BigInteger can hold, not what fits a word.
    let huge_int = PlutusData::new_integer(
        BigInteger::from_str("340282366920938463463374607431768211455").unwrap(),
    );
    let list = PlutusData::new_list(vec![bytes.clone(), small_int.clone()]);
    let mut map = PlutusMap::new();
    map.set(bytes.clone(), list.clone());
    map.set(small_int.clone(), bytes.clone());
    let map = PlutusData::new_map(map);
    let constr = PlutusData::new_constr_plutus_data(ConstrPlutusData::new(
        2,
        vec![small_int.clone(), list.clone(), map.clone()],
    ));
    let empty_constr = PlutusData::new_constr_plutus_data(ConstrPlutusData::new(0, vec![]));

    assert_valid(&v, &bytes, "bytes arm");
    assert_valid(&v, &small_int, "int arm");
    assert_valid(&v, &negative_int, "negative int arm");
    assert_valid(&v, &huge_int, "huge int arm");
    assert_valid(&v, &list, "list arm");
    assert_valid(&v, &map, "map arm");
    assert_valid(&v, &constr, "constructor arm");
    assert_valid(&v, &empty_constr, "empty constructor arm");
}

#[test]
fn plutus_data_schema_rejects_the_old_wrong_shapes() {
    let v = plutus_validator();
    // The two shapes the schema wrongly declared before it was repaired. If either validates,
    // the repair regressed.
    let int_as_string = serde_json::json!({ "int": "5" });
    assert!(
        !v.is_valid(&int_as_string),
        "int arm must be a JSON number, not a BigInteger string"
    );
    let fields_as_kv = serde_json::json!({
        "constructor": 0,
        "fields": [{ "k": { "int": 1 }, "v": { "int": 2 } }]
    });
    assert!(
        !v.is_valid(&fields_as_kv),
        "constructor fields are a plain PlutusData array, not map-style k/v pairs"
    );
}

#[test]
fn constr_plutus_data_serializes_as_detailed_schema() {
    // A type has ONE published JSON encoding: standalone ConstrPlutusData must serialize
    // exactly like the same value inside a PlutusData (the DetailedSchema form), and its
    // schema must accept it.
    let v = constr_validator();
    let constr = ConstrPlutusData::new(
        2,
        vec![
            PlutusData::new_integer(BigInteger::from_str("2").unwrap()),
            PlutusData::new_bytes(vec![0xCA, 0xFE]),
        ],
    );
    let standalone = serde_json::to_value(&constr).unwrap();
    let inside_plutus_data =
        serde_json::to_value(PlutusData::new_constr_plutus_data(constr.clone())).unwrap();
    assert_eq!(
        standalone, inside_plutus_data,
        "standalone ConstrPlutusData must serialize identically to the same value inside a PlutusData"
    );
    assert_valid(&v, &constr, "ConstrPlutusData");

    // The old derived shape must be rejected by both the schema and the deserializer.
    let derived_shape = serde_json::json!({ "alternative": 2, "fields": [] });
    assert!(
        !v.is_valid(&derived_shape),
        "the derived {{alternative, fields}} shape is not the published encoding"
    );
    assert!(serde_json::from_value::<ConstrPlutusData>(derived_shape).is_err());

    // And the published shape round-trips through the deserializer.
    let back: ConstrPlutusData = serde_json::from_value(standalone).unwrap();
    assert_eq!(back, constr);
}

/// The hand-authored schemas must carry the same integer bound metadata the generated rows do:
/// every spec-derived integer in the document is emitted with `format`/`minimum` (a `u64` field
/// is `{"type":"integer","format":"uint64","minimum":0}`), so a bare `{"type":"integer"}` in a
/// hand-authored file reads as "unbounded" to anything consuming the document — which is right
/// for exactly one of the three integers here and wrong for the other two.
///
/// It is not decoration. The bound is the only thing in the document that distinguishes
/// `PlutusData`'s `int` (a `BigInteger`, genuinely unbounded) from `TransactionMetadatum`'s
/// (a `cml_core::Int`, a CBOR int, -2^64 ..= 2^64-1) from `ConstrPlutusData`'s `constructor`
/// (a plain `u64`), and it is what a bound-aware consumer of the schema would have to read to
/// pick a type wider than a JS `number`. So each bound is asserted by a value that sits just
/// outside it.
#[test]
fn hand_authored_integer_bounds_match_their_rust_types() {
    // `constructor` is a u64. Negative and fractional are outside it.
    let constr = constr_validator();
    assert!(
        !constr.is_valid(&serde_json::json!({ "constructor": -1, "fields": [] })),
        "constructor is a u64: negative must not validate"
    );
    assert!(
        constr.is_valid(&serde_json::json!({ "constructor": 0, "fields": [] })),
        "constructor is a u64: zero must validate"
    );

    // `cml_core::Int` is a CBOR int: -2^64 ..= 2^64-1. Parsed from text rather than built with
    // json!, so the boundary values survive as exact integers.
    let parse = |text: &str| -> serde_json::Value { serde_json::from_str(text).unwrap() };
    let metadatum = metadatum_validator();
    for inside in [
        r#"{"int":18446744073709551615}"#,
        r#"{"int":-18446744073709551616}"#,
    ] {
        assert!(
            metadatum.is_valid(&parse(inside)),
            "metadatum int arm must accept the edge of the CBOR int range: {inside}"
        );
    }
    // The positive edge is exact — 2^64 does not fit a u64, so the validator compares it exactly.
    // The negative one is NOT: -2^64 does not fit an i64 either, so the comparison falls back to
    // f64, where the spacing at that magnitude is 4096 and everything within it of the bound
    // compares equal. -2^65 is the nearest round value comfortably outside that blur.
    //
    // That gap is worth stating rather than hiding: past 2^53 a JSON Schema bound is ADVISORY —
    // a double-based validator cannot enforce it exactly, for the same reason a TypeScript
    // `number` cannot hold the value. The bounds are still the right thing to publish (they are
    // read as decimal text by anything generating types from this document), they just are not a
    // runtime guarantee at this magnitude.
    for outside in [
        r#"{"int":18446744073709551616}"#,
        r#"{"int":-36893488147419103232}"#,
    ] {
        assert!(
            !metadatum.is_valid(&parse(outside)),
            "metadatum int arm must reject past the CBOR int range: {outside}"
        );
    }

    // PlutusData's int arm is a BigInteger and has no bound — the same value the metadatum arm
    // rejects belongs here. This is the assertion that keeps the absence of a bound on that arm
    // a deliberate statement rather than an oversight someone later "fixes".
    let plutus = plutus_validator();
    assert!(
        plutus.is_valid(&parse(r#"{"int":340282366920938463463374607431768211455}"#)),
        "PlutusData's int is a BigInteger: it must accept values no fixed-width type can hold"
    );
}

#[test]
fn transaction_metadatum_schema_accepts_every_arm() {
    let v = metadatum_validator();

    let bytes = TransactionMetadatum::new_bytes(vec![0xCA, 0xFE]).unwrap();
    let text = TransactionMetadatum::new_text("hello".to_owned()).unwrap();
    let uint = TransactionMetadatum::new_int(Int::new_uint(7));
    let nint = TransactionMetadatum::new_int(Int::new_nint(41));
    let list = TransactionMetadatum::new_list(vec![bytes.clone(), text.clone()]);
    let mut map = MetadatumMap::new();
    map.set(text.clone(), list.clone());
    map.set(uint.clone(), bytes.clone());
    let map = TransactionMetadatum::new_map(map);

    assert_valid(&v, &bytes, "bytes arm");
    assert_valid(&v, &text, "string arm");
    assert_valid(&v, &uint, "int arm (uint)");
    assert_valid(&v, &nint, "int arm (nint)");
    assert_valid(&v, &list, "list arm");
    assert_valid(&v, &map, "map arm");
}

#[test]
fn transaction_metadatum_schema_rejects_non_arm_shapes() {
    let v = metadatum_validator();
    assert!(
        !v.is_valid(&serde_json::json!({ "int": "5" })),
        "int arm must be a JSON number"
    );
    assert!(
        !v.is_valid(&serde_json::json!({ "constructor": 0, "fields": [] })),
        "metadata has no constructor arm"
    );
}
