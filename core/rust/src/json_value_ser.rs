// An HONEST rendering of `serde_json::Value` / `serde_json::Number` in the serde data model.
//
// WHY THIS EXISTS. `serde_json::Number`'s own `Serialize` is not honest when the `arbitrary_precision`
// cargo feature is on anywhere in the build graph (cargo unifies features, so ONE crate turning it on
// turns it on for everybody). In that configuration a `Number` serializes as a one-field struct named
// `$serde_json::private::Number` whose field is the raw decimal string — a private token that ONLY
// serde_json's own serializer collapses back into a number. Every other serializer emits it verbatim,
// so a value that should be `1000` reaches the far side as
// `{"$serde_json::private::Number":"1000"}` — at EVERY magnitude, not just large ones.
// `serde_json::Value`'s `Serialize` delegates to `Number`'s for its number arm, so anything that
// hands a `serde_json::Value` to a non-serde_json serializer (`serde_wasm_bindgen`, `serde_cbor`,
// `ciborium`, `serde_yaml`, …) inherits the defect. `to_json()` (serde_json) looks right; the wasm
// `to_json_value()` and every other serializer do not.
//
// WHAT THIS GUARANTEES.
//   * WITHOUT `arbitrary_precision` (the default): byte-identical to `serde_json::Value`'s own
//     `Serialize`. An integer `Number` is internally a `u64`/`i64` and is emitted through the same
//     `serialize_u64`/`serialize_i64` calls; a float is internally an `f64` whose decimal spelling IS
//     serde_json's own print of it, so it reaches `serialize_f64`, exactly as before.
//   * WITH `arbitrary_precision`: every value whose decimal spelling IS a canonical integer in
//     `u64`/`i64`/`u128`/`i128` range is emitted as that integer, and every value whose spelling is
//     exactly what serde_json prints for the `f64` it parses back to is emitted as that `f64` — so
//     the far side sees a number. Only what the serde data model genuinely cannot hold LOSSLESSLY
//     keeps serde_json's own token rendering: a decimal carrying more precision than `f64`, an
//     exponent or non-canonical spelling (`1e+3`, `007`) whose bytes are not the number's own, and an
//     integer beyond ±2^127. Substituting an approximation there would truncate `to_json()`, which is
//     the trade this design refuses. Never worse than the status quo, strictly better everywhere a
//     lossless image exists.
//
// The round-trip guard on each arm (the chosen integer's `to_string()` must equal the `Number`'s own)
// is what makes the first guarantee hold for hand-built pathological spellings too: under
// `arbitrary_precision` a `Number` can hold `007`, `-0`, `+5` or `1e3`, all of which parse as integers
// but whose `to_json()` bytes are NOT the integer's. Those delegate, so `to_json()` stays exact.
//
// The 128-bit arms parse the decimal spelling rather than calling `Number::as_u128`/`as_i128`: those
// accessors postdate the `serde_json` version floor the generated manifest declares, and a runtime
// file must compile against the whole declared range. Parsing needs the spelling anyway for the
// round-trip guard, so it costs nothing extra.
//
// FOR HAND-WRITTEN `Serialize` IMPLS. A `_CDDL_CODEGEN_EXTERN_TYPE_` or `@custom_json` type whose
// `Serialize` builds a `serde_json::Value` (the natural shape for a hand-written JSON encoding) must
// route the final step through [`serialize_json_value`] / [`JsonValueSer`] instead of
// `serde::Serialize::serialize(&value, serializer)`, or it ships the token to every non-serde_json
// serializer. `serialize_json_value`'s signature is `#[serde(serialize_with = "…")]`-compatible.
//
// No let-chains / no edition-2024-only syntax: this file is compiled into consumer crates whose
// edition is theirs to choose, and into this repo's own in-bin runtime tests.

/// Serialize a [`serde_json::Number`] HONESTLY: as an integer when its decimal spelling is a
/// canonical integer that fits `u64`/`i64`/`u128`/`i128`, as an `f64` when its spelling is exactly
/// what serde_json prints for that `f64`, and otherwise by delegating to serde_json's own impl (the
/// only lossless carrier for a decimal the serde data model cannot hold). See this file's header for
/// the two cfg guarantees.
pub fn serialize_json_number<S>(
    number: &serde_json::Number,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // The `Number`'s own decimal spelling — computed ONCE and reused by every guard below. This is
    // exactly what `to_json()` writes, so "the chosen integer re-prints to this" is precisely the
    // condition under which swapping in an integer cannot change `to_json()` output.
    let spelling = number.to_string();
    // Widest-fits-LAST: prefer the narrowest serde integer method a value fits, so the emitted data
    // model matches what a non-`arbitrary_precision` build would have emitted for the same value.
    if let Some(u) = number.as_u64().filter(|u| u.to_string() == spelling) {
        return serializer.serialize_u64(u);
    }
    if let Some(i) = number.as_i64().filter(|i| i.to_string() == spelling) {
        return serializer.serialize_i64(i);
    }
    if let Some(u) = spelling
        .parse::<u128>()
        .ok()
        .filter(|u| u.to_string() == spelling)
    {
        return serializer.serialize_u128(u);
    }
    if let Some(i) = spelling
        .parse::<i128>()
        .ok()
        .filter(|i| i.to_string() == spelling)
    {
        return serializer.serialize_i128(i);
    }
    // Not an integer — but a float is honest too whenever the `Number`'s spelling is EXACTLY what
    // serde_json prints for the `f64` it parses back to. That is EVERY float a
    // `not(arbitrary_precision)` build can hold (its spelling IS that print), so this arm reproduces
    // the old `serialize_f64` delegate exactly rather than changing it; and under the feature it
    // covers every decimal an `f64` represents exactly, e.g. the `1.5` a CBOR float in an `any` member
    // becomes. The guard is what excludes the genuinely unrepresentable rest: a decimal carrying more
    // precision than `f64` (`0.1234567890123456789`) or an exponent spelling (`1e+3`) re-prints
    // differently, so it falls through and keeps its exact bytes.
    let spelling_is_this_f64 = |f: &f64| match serde_json::Number::from_f64(*f) {
        Some(reprinted) => reprinted.to_string() == spelling,
        None => false,
    };
    if let Some(f) = number.as_f64().filter(spelling_is_this_f64) {
        return serializer.serialize_f64(f);
    }
    // An arbitrary-precision decimal with no image anywhere in the serde data model. serde_json's own
    // token is its ONLY lossless carrier, so keeping it is what keeps `to_json()` exact — substituting
    // the nearest `f64` here would silently truncate the lossless surface to buy a prettier
    // `to_json_value()`, which is the trade this whole design refuses.
    serde::Serialize::serialize(number, serializer)
}

/// Serialize a [`serde_json::Value`] HONESTLY. Every non-number arm mirrors
/// `impl Serialize for serde_json::Value` exactly — `Null` as unit, `Bool`/`String` as themselves, an
/// array as a length-hinted seq, an object as a length-hinted map in the map's own iteration order —
/// so the only behavioural difference from the stock impl is the number arm, all the way down
/// (children are wrapped in [`JsonValueSer`] so nested numbers are honest too).
pub fn serialize_json_value<S>(value: &serde_json::Value, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::{SerializeMap, SerializeSeq};
    match value {
        serde_json::Value::Null => serializer.serialize_unit(),
        serde_json::Value::Bool(b) => serializer.serialize_bool(*b),
        serde_json::Value::Number(n) => serialize_json_number(n, serializer),
        serde_json::Value::String(s) => serializer.serialize_str(s),
        serde_json::Value::Array(elems) => {
            let mut seq = serializer.serialize_seq(Some(elems.len()))?;
            for elem in elems {
                seq.serialize_element(&JsonValueSer(elem))?;
            }
            seq.end()
        }
        serde_json::Value::Object(entries) => {
            let mut map = serializer.serialize_map(Some(entries.len()))?;
            for (key, val) in entries {
                map.serialize_entry(key, &JsonValueSer(val))?;
            }
            map.end()
        }
    }
}

/// A `Serialize`-implementing view of a [`serde_json::Value`] that renders it honestly, so one can
/// compose inside serde containers (a `Vec<JsonValueSer>`, a map value, a `serialize_some` payload)
/// wherever a bare `&serde_json::Value` would otherwise pull in the dishonest impl.
pub struct JsonValueSer<'a>(pub &'a serde_json::Value);

impl serde::Serialize for JsonValueSer<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize_json_value(self.0, serializer)
    }
}
