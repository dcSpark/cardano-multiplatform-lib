// Open struct-map rest FLATTEN helpers. An open struct's
// captured unknown entries render at the SAME JSON object level as the declared fields (serde
// `flatten` merges the rest map into the parent object), and read back symmetrically (declared field
// names bind first, every other key lands in rest). The struct keeps its derive; the rest field
// carries `#[serde(flatten, serialize_with = …, deserialize_with = …)]` pointing at GENERATED
// per-struct wrappers that close over the declared JSON names (for the write-side collision check)
// and the key domain, delegating the mechanics to the two helpers below.
//
// These are `any`-free by construction (generic over the key type, the key-stringify closure's error,
// and the value view), so a fully-typed rest row (`* uint => text`, no `any` anywhere) uses them
// without pulling in the `AnyCbor` runtime. The `any`-domain wrappers supply the natural key/value
// views from `any_cbor.rs`; the typed-domain wrappers supply plain closures / the value's own serde.

/// Serialize an open struct-map's rest entries FLATTENED. `reserved` is the declared fields' JSON
/// names. Errors (no silent duplicate/shadow, per RFC 8949 §6.1's strict-fail): a rest key equal to a declared name (would shadow
/// it, and most JSON parsers are last-wins); two rest keys that stringify identically (this is how a
/// `@duplicates preserve` PairMap rest holding ACTUAL duplicate keys makes `to_json` fail —
/// duplicates stringify identically by definition); a key whose string form does not exist (a complex
/// `any` key — bytes/array/map/tag/float — surfaced as the `key_to_string` closure's `Err`). Collision
/// detection is over ALL keys before any byte is emitted (a `BTreeSet` — determinism).
pub fn serialize_flattened_rest<'a, S, K, W, E, I>(
    reserved: &[&str],
    key_to_string: impl Fn(&K) -> Result<String, E>,
    entries: I,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    W: serde::Serialize,
    E: std::fmt::Display,
    K: 'a,
    I: IntoIterator<Item = (&'a K, W)>,
{
    use serde::ser::SerializeMap;
    let mut pairs: Vec<(String, W)> = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for (k, w) in entries {
        let ks = key_to_string(k).map_err(serde::ser::Error::custom)?;
        if reserved.contains(&ks.as_str()) {
            return Err(serde::ser::Error::custom(format!(
                "open struct-map rest key {ks:?} stringifies to a declared field's JSON name \
                 (would shadow the declared field)"
            )));
        }
        if !seen.insert(ks.clone()) {
            return Err(serde::ser::Error::custom(format!(
                "two open struct-map rest keys stringify identically to {ks:?}"
            )));
        }
        pairs.push((ks, w));
    }
    let mut map = serializer.serialize_map(Some(pairs.len()))?;
    for (ks, w) in &pairs {
        map.serialize_entry(ks, w)?;
    }
    map.end()
}

/// Read the flattened rest entries serde's `flatten` buffering hands back as a MAP of string keys (it
/// never applies serde_json's numeric-key coercion, verified against current serde). Returns the raw
/// `(String, value)` pairs; the GENERATED wrapper then coerces each key to
/// the domain type per RFC 8949 §6.2 (typed domains parse per the domain type; `any` domains prefer the
/// numeric reading) and unwraps the value view. A duplicate declared field name in the input is
/// already rejected loudly upstream by serde's own flatten machinery (the read-side collision guard).
pub fn read_flattened_rest_pairs<'de, D, VDe>(
    deserializer: D,
) -> Result<Vec<(String, VDe)>, D::Error>
where
    D: serde::Deserializer<'de>,
    VDe: serde::Deserialize<'de>,
{
    struct Vis<VDe>(std::marker::PhantomData<VDe>);
    impl<'de, VDe: serde::Deserialize<'de>> serde::de::Visitor<'de> for Vis<VDe> {
        type Value = Vec<(String, VDe)>;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a map of open struct-map rest entries")
        }
        fn visit_map<M: serde::de::MapAccess<'de>>(
            self,
            mut access: M,
        ) -> Result<Self::Value, M::Error> {
            let mut out = Vec::new();
            while let Some((k, v)) = access.next_entry::<String, VDe>()? {
                out.push((k, v));
            }
            Ok(out)
        }
    }
    deserializer.deserialize_map(Vis(std::marker::PhantomData))
}
