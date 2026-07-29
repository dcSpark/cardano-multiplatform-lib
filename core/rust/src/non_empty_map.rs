extern crate alloc;
use super::error::{DeserializeError, DeserializeFailure};
use super::ordered_hash_map::OrderedHashMap;
use alloc::format;

/// A map guaranteed to hold at least one entry — the restricted twin of the loose table map for the
/// CDDL `{+ k => v}` occurrence (`+` / `1*`).
///
/// The non-emptiness is enforced ONCE, at the single `TryFrom<OrderedHashMap<K, V>>` door, and thereafter
/// cannot be broken: there is no public field, the inner map is never lent as `&mut`, and every
/// length-shrinking operation is checked (removal refuses at length 1). Value-level `&mut` access
/// (`get_mut` / `values_mut` / `iter_mut`) is unrestricted — the invariant is about the map's
/// LENGTH, which a `&mut V` cannot reach. Construction from a map that is empty
/// fails with the same `DeserializeFailure::RangeCheck { min: Some(1), max: None }` the CBOR decoder
/// raises, so the API door and the wire door report identical errors.
///
/// The inner map is this crate's table type (the flavor is fixed at generation time by
/// `--preserve-encodings`), so iteration stays deterministic (never a hash map with random order).
///
/// Staging an eventually-non-empty map is done with a plain map (`insert`/`extend`/`collect`) and a
/// final `NonEmptyMap::try_from(map)` — the loose map is the builder, so no generated builder type is
/// needed.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonEmptyMap<K: Ord + core::hash::Hash + Eq, V>(OrderedHashMap<K, V>);

impl<K: Ord + core::hash::Hash + Eq, V> NonEmptyMap<K, V> {
    /// Construct from a single entry — always valid (length 1).
    pub fn new(first_key: K, first_value: V) -> Self {
        let mut inner = OrderedHashMap::new();
        inner.insert(first_key, first_value);
        Self(inner)
    }

    /// Insert an entry, returning the displaced value if the key was already present. Infallible:
    /// growing (or replacing) can never violate the `>= 1` lower bound.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }

    /// Remove and return the value for `key`, unless doing so could empty the map (i.e. it is refused
    /// at length 1 so the non-emptiness invariant can never break).
    pub fn remove(&mut self, key: &K) -> Result<Option<V>, DeserializeError> {
        if self.0.len() <= 1 {
            return Err(DeserializeFailure::RangeCheck {
                found: 0,
                min: Some(1),
                max: None,
            }
            .into());
        }
        Ok(self.0.remove(key))
    }

    /// Number of entries. Never zero.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Always `false` — a `NonEmptyMap` holds at least one entry by construction. Present so the
    /// `len`/`is_empty` pair stays consistent (and clippy-clean).
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }

    /// Mutable access to a single value — e.g. updating a nested `NonEmptyMap` in place.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.0.iter_mut()
    }

    // Built on `iter_mut` so both inner map flavors need only that method.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.0.iter_mut().map(|(_, v)| v)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.0.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.0.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.0.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.0.values()
    }

    /// Escape hatch to the loose map — mutate freely, then `try_into` back.
    pub fn into_inner(self) -> OrderedHashMap<K, V> {
        self.0
    }
}

impl<K: Ord + core::hash::Hash + Eq, V> TryFrom<OrderedHashMap<K, V>> for NonEmptyMap<K, V> {
    type Error = DeserializeError;

    fn try_from(map: OrderedHashMap<K, V>) -> Result<Self, Self::Error> {
        if map.is_empty() {
            Err(DeserializeFailure::RangeCheck {
                found: 0,
                min: Some(1),
                max: None,
            }
            .into())
        } else {
            Ok(Self(map))
        }
    }
}

impl<K: Ord + core::hash::Hash + Eq, V> From<NonEmptyMap<K, V>> for OrderedHashMap<K, V> {
    fn from(m: NonEmptyMap<K, V>) -> Self {
        m.0
    }
}
impl<K: Ord + core::hash::Hash + Eq + serde::Serialize, V: serde::Serialize> serde::Serialize
    for NonEmptyMap<K, V>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // a NonEmptyMap is a plain JSON object — serialize as the loose map would
        self.0.serialize(serializer)
    }
}

impl<'de, K: Ord + core::hash::Hash + Eq + serde::Deserialize<'de>, V: serde::Deserialize<'de>>
    serde::de::Deserialize<'de> for NonEmptyMap<K, V>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        // never derive a structural Deserialize (it would bypass the invariant): deserialize the
        // loose map and route through the same TryFrom door as every other construction path.
        let map = <OrderedHashMap<K, V> as serde::de::Deserialize>::deserialize(deserializer)?;
        NonEmptyMap::try_from(map).map_err(serde::de::Error::custom)
    }
}
impl<K: Ord + core::hash::Hash + Eq + schemars::JsonSchema, V: schemars::JsonSchema>
    schemars::JsonSchema for NonEmptyMap<K, V>
{
    fn schema_name() -> alloc::borrow::Cow<'static, str> {
        format!("NonEmptyMap<{}, {}>", K::schema_name(), V::schema_name()).into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // shape matches the loose map (a JSON object); the >= 1 invariant is enforced at TryFrom
        OrderedHashMap::<K, V>::json_schema(generator)
    }
    fn inline_schema() -> bool {
        OrderedHashMap::<K, V>::inline_schema()
    }
}
