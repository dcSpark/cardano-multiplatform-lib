extern crate alloc;
use super::error::{DeserializeError, DeserializeFailure};
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// A `Vec<(K, V)>`-backed, entry-ordered, DUPLICATE-PERMITTING map — the `@duplicates preserve` twin
/// of the loose table map for a `{ * k => v }` rule that must accept AND re-emit duplicate keys.
///
/// A `BTreeMap`/`OrderedHashMap` is structurally incapable of holding two entries with the same key,
/// so a table whose wire data carries duplicate keys (pre-Conway Cardano `transaction_metadata` is
/// the driver — its auxiliary-data hash is computed over the ORIGINAL bytes, so a reader that
/// collapses or reorders duplicate keys fails hash verification) needs a vec-of-pairs instead: the
/// only shape faithful to both entry order and duplicates.
///
/// Unlike `OrderedSet`, a `PairMap` has NO invariant — any vec of pairs is valid — so it needs no
/// guarded door: `From<Vec<(K, V)>>` / `into_inner` convert both ways freely. The read surface is
/// map-flavored but honest about duplicates: `get` returns the FIRST match (linear scan), `get_all`
/// returns every match in entry order, iteration is in entry order.
///
/// `insert` APPENDS a new entry and NEVER replaces an existing key (that is the whole point — a
/// replacing insert would silently drop a duplicate). It always returns `None` (nothing is displaced
/// because nothing is overwritten); the `Option<V>` return exists only so the map read surface
/// matches the loose table's.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PairMap<K, V>(Vec<(K, V)>);

impl<K, V> PairMap<K, V> {
    /// An empty map — always valid.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Append an entry. NEVER replaces an existing key (duplicates are the point); always returns
    /// `None`. The `Option<V>` return mirrors the loose table's `insert` signature so generated wasm
    /// accessors stay uniform.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.push((key, value));
        None
    }

    /// Number of entries (counting duplicate keys separately).
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The value of the FIRST entry whose key equals `key` (linear scan, entry order). A
    /// duplicate-permitting map has no single "the" value for a repeated key; this returns the first,
    /// `get_all` returns them all.
    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: PartialEq,
    {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Every value whose key equals `key`, in entry order.
    pub fn get_all(&self, key: &K) -> Vec<&V>
    where
        K: PartialEq,
    {
        self.0
            .iter()
            .filter(|(k, _)| k == key)
            .map(|(_, v)| v)
            .collect()
    }

    pub fn contains_key(&self, key: &K) -> bool
    where
        K: PartialEq,
    {
        self.0.iter().any(|(k, _)| k == key)
    }

    /// Iterate entries in entry order as `(&K, &V)` — the same shape `BTreeMap::iter` yields, so the
    /// generated serialize loop is identical regardless of table flavor.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.0.iter().map(|(k, v)| (k, v))
    }

    /// Keys in entry order (duplicate keys appear once per entry).
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.0.iter().map(|(k, _)| k)
    }

    /// Values in entry order.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.0.iter().map(|(_, v)| v)
    }

    pub fn as_slice(&self) -> &[(K, V)] {
        self.0.as_slice()
    }

    /// Escape hatch to the loose `Vec<(K, V)>` — mutate freely, then `into` back (any vec is valid).
    pub fn into_inner(self) -> Vec<(K, V)> {
        self.0
    }
}

impl<K, V> Default for PairMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> From<Vec<(K, V)>> for PairMap<K, V> {
    fn from(entries: Vec<(K, V)>) -> Self {
        Self(entries)
    }
}

impl<K, V> From<PairMap<K, V>> for Vec<(K, V)> {
    fn from(m: PairMap<K, V>) -> Self {
        m.0
    }
}

impl<K, V> FromIterator<(K, V)> for PairMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

/// A `PairMap` guaranteed to hold at least one entry — the `@duplicates preserve` twin of
/// `NonEmptyMap` for a `{ + k => v }` rule. Composes the min-1 invariant with the vec-of-pairs
/// representation.
///
/// Non-emptiness is enforced ONCE, at the single `TryFrom<Vec<(K, V)>>` door, and thereafter cannot
/// be broken: there is no public field and no unguarded emptying operation. Construction from an
/// empty vec fails with the same `DeserializeFailure::RangeCheck { min: Some(1), max: None }` the
/// CBOR decoder raises for `{ + … }`, so the API door and the wire door report identical errors.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonEmptyPairMap<K, V>(Vec<(K, V)>);

impl<K, V> NonEmptyPairMap<K, V> {
    /// Construct from a single entry — always valid (length 1).
    pub fn new(first_key: K, first_value: V) -> Self {
        Self(vec![(first_key, first_value)])
    }

    /// Append an entry. Growing can never violate the `>= 1` lower bound, and (as with `PairMap`)
    /// this never replaces an existing key, always returning `None`.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.push((key, value));
        None
    }

    /// Number of entries. Never zero.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Always `false` — a `NonEmptyPairMap` holds at least one entry by construction. Present so the
    /// `len`/`is_empty` pair stays consistent (and clippy-clean).
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: PartialEq,
    {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn get_all(&self, key: &K) -> Vec<&V>
    where
        K: PartialEq,
    {
        self.0
            .iter()
            .filter(|(k, _)| k == key)
            .map(|(_, v)| v)
            .collect()
    }

    pub fn contains_key(&self, key: &K) -> bool
    where
        K: PartialEq,
    {
        self.0.iter().any(|(k, _)| k == key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.0.iter().map(|(k, v)| (k, v))
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.0.iter().map(|(k, _)| k)
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.0.iter().map(|(_, v)| v)
    }

    pub fn as_slice(&self) -> &[(K, V)] {
        self.0.as_slice()
    }

    /// Escape hatch to the loose `Vec<(K, V)>` — mutate freely, then `try_into` back through the door.
    pub fn into_inner(self) -> Vec<(K, V)> {
        self.0
    }
}

impl<K, V> TryFrom<Vec<(K, V)>> for NonEmptyPairMap<K, V> {
    type Error = DeserializeError;

    fn try_from(entries: Vec<(K, V)>) -> Result<Self, Self::Error> {
        if entries.is_empty() {
            Err(DeserializeFailure::RangeCheck {
                found: 0,
                min: Some(1),
                max: None,
            }
            .into())
        } else {
            Ok(Self(entries))
        }
    }
}

impl<K, V> TryFrom<PairMap<K, V>> for NonEmptyPairMap<K, V> {
    type Error = DeserializeError;

    /// The loose-to-restricted door the wasm `NonEmpty…PairMap` wrapper's `try_from` delegates to (the
    /// pair-map twin of `NonEmptyMap`'s `TryFrom<{table}>`): re-uses the single `Vec` door so the
    /// empty-map refusal error is identical to every other path.
    fn try_from(m: PairMap<K, V>) -> Result<Self, Self::Error> {
        Self::try_from(Vec::from(m))
    }
}

impl<K, V> From<NonEmptyPairMap<K, V>> for Vec<(K, V)> {
    fn from(m: NonEmptyPairMap<K, V>) -> Self {
        m.0
    }
}
impl<K: serde::Serialize, V: serde::Serialize> serde::Serialize for PairMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // A duplicate-permitting map CANNOT be a JSON object (object keys must be unique — a JSON
        // object silently collapses duplicates). Serialize as an ARRAY of `[k, v]` pairs instead,
        // which preserves both order and duplicates. serde renders a `[(K, V)]` slice as exactly
        // that: `[[k, v], [k, v], ...]`.
        self.as_slice().serialize(serializer)
    }
}

impl<'de, K: serde::Deserialize<'de>, V: serde::Deserialize<'de>> serde::de::Deserialize<'de>
    for PairMap<K, V>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        // the array-of-pairs shape from `Serialize` above
        let entries = <Vec<(K, V)> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(PairMap::from(entries))
    }
}

impl<K: serde::Serialize, V: serde::Serialize> serde::Serialize for NonEmptyPairMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_slice().serialize(serializer)
    }
}

impl<'de, K: serde::Deserialize<'de>, V: serde::Deserialize<'de>> serde::de::Deserialize<'de>
    for NonEmptyPairMap<K, V>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        // never derive a structural Deserialize (it would bypass the min-1 invariant): deserialize
        // the loose array-of-pairs and route through the same TryFrom door as every other path.
        let entries = <Vec<(K, V)> as serde::de::Deserialize>::deserialize(deserializer)?;
        NonEmptyPairMap::try_from(entries).map_err(serde::de::Error::custom)
    }
}
impl<K: schemars::JsonSchema, V: schemars::JsonSchema> schemars::JsonSchema for PairMap<K, V> {
    fn schema_name() -> alloc::borrow::Cow<'static, str> {
        format!("PairMap<{}, {}>", K::schema_name(), V::schema_name()).into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // shape matches the JSON representation (an array of `[k, v]` pairs), NOT an object: a
        // duplicate-permitting map has no object form. The `Vec<(K, V)>` schema is exactly that.
        Vec::<(K, V)>::json_schema(generator)
    }
    fn inline_schema() -> bool {
        Vec::<(K, V)>::inline_schema()
    }
}

impl<K: schemars::JsonSchema, V: schemars::JsonSchema> schemars::JsonSchema
    for NonEmptyPairMap<K, V>
{
    fn schema_name() -> alloc::borrow::Cow<'static, str> {
        format!(
            "NonEmptyPairMap<{}, {}>",
            K::schema_name(),
            V::schema_name()
        )
        .into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // shape matches the array-of-pairs JSON; the >= 1 invariant is enforced at TryFrom.
        Vec::<(K, V)>::json_schema(generator)
    }
    fn inline_schema() -> bool {
        Vec::<(K, V)>::inline_schema()
    }
}
