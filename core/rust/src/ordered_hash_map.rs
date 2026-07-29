extern crate alloc;
use alloc::format;
use core::hash::{BuildHasher, Hash};

/// The hash builder backing `OrderedHashMap`, chosen per build:
///
/// * with the `std` feature (the default), std's `RandomState` — randomly seeded SipHash, so keys
///   chosen by whoever wrote the CBOR can't drive a table into its worst case. This is the historic
///   choice and every std consumer keeps it.
/// * without it, the backing map crate's own default builder, which is what a crate with no `std`
///   to draw entropy from can have: faster, deterministically seeded, and correspondingly weaker
///   against an attacker who picks the keys.
///
/// The difference is a service-degradation axis and nothing more. A table's ITERATION order is its
/// insertion order under either builder, and iteration order is what the serializer writes, so the
/// bytes a given value produces are identical across the two.
///
/// It is a public alias rather than a written-out path because it appears in the signatures below —
/// anything naming those names this too, so the alias is the one place the choice lives, and
/// switching arms renames no public type.
#[cfg(feature = "std")]
pub type MapHashBuilder = std::collections::hash_map::RandomState;
#[cfg(not(feature = "std"))]
pub type MapHashBuilder = hashlink::DefaultHashBuilder;

/// The two halves of [`Entry`], re-exported so code matching on an entry can name them without
/// depending on the backing map crate itself.
pub use hashlink::linked_hash_map::{OccupiedEntry, VacantEntry};

#[derive(Clone, Debug, Hash, Ord, Eq, PartialEq, PartialOrd)]
pub struct OrderedHashMap<K, V>(hashlink::LinkedHashMap<K, V, MapHashBuilder>)
where
    K: Hash + Eq + Ord;

// An empty ordered map is always constructible, so `Default` must not require `K: Default` /
// `V: Default` the way `#[derive(Default)]` over a generic struct would. This mirrors
// `std::collections::BTreeMap`/`HashMap` (empty-map default, no element bounds) so a table whose
// KEY type isn't `Default` — e.g. a generated `@used_as_key` enum under `--preserve-encodings`,
// where tables become `OrderedHashMap` and enum keys don't derive `Default` — can still be
// `Default::default()`ed
impl<K, V> Default for OrderedHashMap<K, V>
where
    K: Hash + Eq + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> core::ops::Deref for OrderedHashMap<K, V>
where
    K: Hash + Eq + Ord,
{
    type Target = hashlink::LinkedHashMap<K, V, MapHashBuilder>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> core::ops::DerefMut for OrderedHashMap<K, V>
where
    K: Hash + Eq + Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A view into a single entry of an `OrderedHashMap`.
///
/// `OrderedHashMap::entry` is an INHERENT method, and an inherent method shadows `Deref`, so a
/// `map.entry(k)` call lands here however the receiver is spelled — this type's surface is the whole
/// surface, with no fallback to the backing map's own entry view. That surface is:
/// `or_insert`, `or_insert_with`, `or_default`, `and_modify`, `key`, and the two variant types
/// `OccupiedEntry`/`VacantEntry` re-exported above.
///
/// A table's iteration order IS its serialized key order, so under `--preserve-encodings` a moved
/// entry would silently rewrite the bytes of a value that was only read off the wire and
/// incremented. Reading or accumulating through an entry therefore leaves the order alone — which
/// is the reason this type exists: the backing crate's `or_insert`/`or_insert_with` move an occupied
/// entry to the back, refreshing it LRU-style. Only an explicit `insert` of a new value re-positions
/// a key.
pub enum Entry<'a, K, V, S = MapHashBuilder> {
    Occupied(OccupiedEntry<'a, K, V, S>),
    Vacant(VacantEntry<'a, K, V, S>),
}

impl<'a, K, V, S> Entry<'a, K, V, S> {
    /// The value for this entry, inserting `default` first if it is vacant. An occupied entry keeps
    /// its position; a vacant one is appended at the back, exactly as a fresh `insert` would be.
    pub fn or_insert(self, default: V) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// `or_insert` with the default computed only when the entry is vacant.
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    /// `or_insert` with `V::default()`. The backing crate has no `or_default` at all, so this is the
    /// only spelling of it a consumer has.
    pub fn or_default(self) -> &'a mut V
    where
        K: Hash,
        S: BuildHasher,
        V: Default,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(V::default()),
        }
    }

    /// In-place mutable access to an occupied entry before any potential insert. A vacant entry
    /// passes through untouched, and neither arm changes the insertion order.
    pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }

    /// The key this entry was looked up by, whether or not it is present.
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => entry.key(),
            Entry::Vacant(entry) => entry.key(),
        }
    }
}

impl<K, V> OrderedHashMap<K, V>
where
    K: Hash + Eq + Ord,
{
    pub fn new() -> Self {
        // `default()` rather than `new()`: it is the constructor BOTH arms of `MapHashBuilder`
        // offer, so the call needs no `cfg` of its own (only std's `RandomState` has an inherent
        // `new`). Both arms seed a builder exactly as their own `Default` does.
        Self(hashlink::LinkedHashMap::with_hasher(
            MapHashBuilder::default(),
        ))
    }

    /// Consume the wrapper, yielding the backing insertion-ordered map.
    pub fn take(self) -> hashlink::LinkedHashMap<K, V, MapHashBuilder> {
        self.0
    }

    /// Shadows the `Deref`'d entry view with the order-preserving [`Entry`] above — every other
    /// method reaches the backing map through `Deref` unchanged.
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, MapHashBuilder> {
        match self.0.entry(key) {
            hashlink::linked_hash_map::Entry::Occupied(entry) => Entry::Occupied(entry),
            hashlink::linked_hash_map::Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

impl<K, V> FromIterator<(K, V)> for OrderedHashMap<K, V>
where
    K: Hash + Eq + Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(hashlink::LinkedHashMap::from_iter(iter))
    }
}
impl<K, V> serde::Serialize for OrderedHashMap<K, V>
where
    K: Hash + Eq + Ord + serde::Serialize,
    V: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map = self.iter().collect::<alloc::collections::BTreeMap<_, _>>();
        map.serialize(serializer)
    }
}

impl<'de, K, V> serde::de::Deserialize<'de> for OrderedHashMap<K, V>
where
    K: Hash + Eq + Ord + serde::Deserialize<'de>,
    V: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let map = <alloc::collections::BTreeMap<_, _> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self(map.into_iter().collect()))
    }
}

impl<K, V> schemars::JsonSchema for OrderedHashMap<K, V>
where
    K: Hash + Eq + Ord + schemars::JsonSchema,
    V: schemars::JsonSchema,
{
    fn schema_name() -> alloc::borrow::Cow<'static, str> {
        format!("OrderedHashMap<{}, {}>", K::schema_name(), V::schema_name()).into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        alloc::collections::BTreeMap::<K, V>::json_schema(generator)
    }
    fn inline_schema() -> bool {
        alloc::collections::BTreeMap::<K, V>::inline_schema()
    }
}
