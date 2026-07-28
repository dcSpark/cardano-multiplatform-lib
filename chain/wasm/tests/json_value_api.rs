//! `to_json_value()` must return exactly what `JSON.parse(to_json())` returns.
//!
//! That is the contract `cml_core_wasm::impl_wasm_json_api!` states in its own comment, and it
//! is what the published `<Class>JSON` TypeScript types claim: `json-ts-types.js` retypes
//! `to_json_value(): any` to the type generated from the JSON *schema*, i.e. from the document
//! `to_json()` produces. If the two routes disagree, the shipped type is a lie.
//!
//! The assertion is made in JS terms on purpose — `JSON.stringify` of both values — rather than
//! by deserializing the `JsValue` back into Rust. `cml-chain` enables
//! `serde_json/arbitrary_precision`, whose `Value` deserializer recognises its own
//! `$serde_json::private::Number` token struct and folds it back into a number, so a Rust-side
//! round-trip would silently repair the exact defect this file exists to catch. Both sides here
//! go through `JSON.stringify` of a JS value built from the same Rust serialization order, so
//! key order matches and only the shapes are under test.
//!
//! ABOVE 2^53 the two routes deliberately DIVERGE, and that is cddl-codegen's contract, not a gap
//! here: `to_json()` stays lossless, and `to_json_value()` fails loud rather than silently rounding
//! (`docs/docs/wasm_differences.mdx`, pinned upstream by `tests/wasm_json/roundtrip.mjs`). CML's
//! `impl_wasm_json_api!` body is a verbatim copy of the generator's emission, so that contract is
//! ours to keep too — `loud_failure_above_2_53_is_preserved` below pins it, and exists to stop
//! anyone "fixing" the agreement above by routing `to_json_value` through `JSON.parse(to_json())`.
//! Doing so would make every test above pass and silently trade the loud failure for rounding.
//!
//! wasm32 only: `to_json_value` returns a real `JsValue`, so there is nothing to assert natively.
//!
//!   cargo test --target wasm32-unknown-unknown -p cml-chain-wasm --test json_value_api
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::wasm_bindgen_test;

/// `JSON.stringify(value)`, i.e. what a JS consumer actually holds.
fn stringify(value: &wasm_bindgen::JsValue) -> String {
    js_sys::JSON::stringify(value)
        .expect("JSON.stringify failed")
        .into()
}

/// The agreement assertion, for values inside JavaScript's safe-integer range. Above it the
/// contract is divergence, not agreement — see the module header and the last test in this file.
fn assert_routes_agree(what: &str, json: String, json_value: wasm_bindgen::JsValue) {
    let via_value = stringify(&json_value);
    let via_string = stringify(&js_sys::JSON::parse(&json).expect("to_json() is not valid JSON"));
    assert_eq!(
        via_value, via_string,
        "{what}: to_json_value() and JSON.parse(to_json()) disagree"
    );
}

#[wasm_bindgen_test]
fn plutus_data_int_json_value_matches_to_json() {
    // 1903e8 = integer 1000. Small on purpose: this is not about magnitude.
    let datum = cml_chain_wasm::plutus::PlutusData::from_cbor_bytes(&[0x19, 0x03, 0xe8]).unwrap();
    assert_routes_agree(
        "PlutusData::Integer",
        datum.to_json().unwrap(),
        datum.to_json_value().unwrap(),
    );
}

#[wasm_bindgen_test]
fn constr_plutus_data_json_value_matches_to_json() {
    // The constructor arm reaches PlutusData through `fields`, so an empty list would not
    // exercise the same serialization path an inhabited one does.
    let mut fields = cml_chain_wasm::PlutusDataList::new();
    fields.add(&cml_chain_wasm::plutus::PlutusData::from_cbor_bytes(&[0x07]).unwrap());
    let constr = cml_chain_wasm::plutus::ConstrPlutusData::new(2, &fields);
    assert_routes_agree(
        "ConstrPlutusData",
        constr.to_json().unwrap(),
        constr.to_json_value().unwrap(),
    );
}

#[wasm_bindgen_test]
fn transaction_metadatum_json_value_matches_to_json() {
    let metadatum =
        cml_chain_wasm::auxdata::TransactionMetadatum::from_cbor_bytes(&[0x19, 0x03, 0xe8])
            .unwrap();
    assert_routes_agree(
        "TransactionMetadatum::Int",
        metadatum.to_json().unwrap(),
        metadatum.to_json_value().unwrap(),
    );
}

#[wasm_bindgen_test]
fn generated_type_json_value_matches_to_json() {
    // Control: a fully generated type, whose derived `Serialize` never builds a
    // `serde_json::Value`. It should pass both before and after the fix — if it ever fails, the
    // problem is broader than the hand-written datum encodings.
    let input = cml_chain_wasm::transaction::TransactionInput::new(
        &cml_crypto_wasm::TransactionHash::from_hex(&"0".repeat(64)).unwrap(),
        3,
    );
    assert_routes_agree(
        "TransactionInput",
        input.to_json().unwrap(),
        input.to_json_value().unwrap(),
    );
}

#[wasm_bindgen_test]
fn loud_failure_above_2_53_is_preserved() {
    // cddl-codegen's blessed contract for a u64 past JavaScript's safe-integer range: to_json()
    // stays lossless, to_json_value() THROWS rather than returning a silently rounded number.
    // Asserted here because CML's impl_wasm_json_api! carries a copy of the generator's emission,
    // so nothing else in this repo would notice the copy drifting.
    //
    // It also covers the hand-written datum encodings, and that is the point of putting it in this
    // file: they used to bypass the refusal entirely, handing back a lossless-but-unusable
    // `$serde_json::private::Number` object instead. Now that json_serialize::Value emits a real
    // u64, they are subject to the same contract as every generated type.
    let big = "9007199254740993"; // 2^53 + 1, the first u64 a JS number cannot hold exactly

    let input = cml_chain_wasm::transaction::TransactionInput::new(
        &cml_crypto_wasm::TransactionHash::from_hex(&"0".repeat(64)).unwrap(),
        9007199254740993,
    );
    assert!(
        input.to_json().unwrap().contains(big),
        "to_json() must keep a u64 > 2^53 at full precision"
    );
    assert!(
        input.to_json_value().is_err(),
        "to_json_value() must fail loud for a u64 > 2^53, not round silently"
    );

    let datum = cml_chain_wasm::plutus::PlutusData::new_integer(
        &cml_chain_wasm::utils::BigInteger::from_str(big).unwrap(),
    );
    assert!(
        datum.to_json().unwrap().contains(big),
        "to_json() must keep a plutus integer > 2^53 at full precision"
    );
    assert!(
        datum.to_json_value().is_err(),
        "a plutus integer > 2^53 must reach the same refusal as any other u64"
    );
}
