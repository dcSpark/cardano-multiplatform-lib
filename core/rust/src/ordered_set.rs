use super::error::{DeserializeError, DeserializeFailure, Key};

/// A `Vec<T>` guaranteed to hold no duplicate elements — the uniqueness twin of `Vec<T>` for a
/// `[* T]` collection rule carrying `@duplicates reject` (the tag-258 set idiom's strict flavor).
///
/// Insertion-ordered and NEVER sorted: `reject` only narrows which inputs are ACCEPTED, it must not
/// touch the bytes of what is accepted, so an accepted set re-emits byte-exactly in its original wire
/// order (the same reason the name follows the qualified-set family `IndexSet`/`LinkedHashSet`, not a
/// bare `Set` that would falsely claim std-set ordering — and would collide with the generated
/// `pub type Set<T> = …` alias the tag-258 idiom mints).
///
/// Uniqueness is enforced ONCE, at the single `TryFrom<Vec<T>>` door, and thereafter cannot be
/// broken: there is no public field, no `&mut Vec<T>`, and `push` is checked (an already-present
/// element is refused). Construction from a `Vec` that holds a duplicate fails with
/// `DeserializeFailure::DuplicateKey(Key::Uint(i))` where **`i` is the zero-based INDEX of the
/// duplicate element** — deterministic and actionable regardless of the element type — so the API
/// door and the wire door report identical errors.
///
/// Staging a not-yet-unique collection is done with a plain `Vec<T>` (`push`/`extend`/`collect`) and
/// a final `OrderedSet::try_from(vec)` — the loose `Vec` is the builder, so no builder type is needed.
//
// ponytail: duplicate detection is a linear `contains` scan (`T: PartialEq`, zero extra storage,
// O(n²) build). Real sets here are small (signers, certificates); a shadow hash/sorted set is a
// purely internal upgrade that would not change these public bounds if it is ever profiled to matter.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderedSet<T>(Vec<T>);

impl<T> OrderedSet<T> {
    /// An empty set — always valid (uniqueness is vacuous).
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Append an element, unless it is already present (which would break uniqueness). On refusal the
    /// `Err`'s `DuplicateKey` carries the INDEX the element would have occupied (the current length);
    /// the element itself is not returned and the collection is left unchanged.
    pub fn push(&mut self, value: T) -> Result<(), DeserializeError>
    where
        T: PartialEq,
    {
        if self.0.contains(&value) {
            return Err(DeserializeFailure::DuplicateKey(Key::Uint(self.0.len() as u64)).into());
        }
        self.0.push(value);
        Ok(())
    }

    /// Remove and return the last element (`None` if empty). Cannot create a duplicate, so unchecked.
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    /// Remove and return the element at `index`. Cannot create a duplicate, so unchecked.
    pub fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    /// Escape hatch to the loose `Vec<T>` — mutate freely, then `try_into` back through the door.
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<T> Default for OrderedSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Scan a `Vec` for the first duplicate; on success return the checked collection. Shared by both
/// twins' doors so the uniqueness scan and its `DuplicateKey(index)` error can never drift.
fn scan_unique<T: PartialEq>(vec: &[T]) -> Result<(), DeserializeError> {
    for (i, elem) in vec.iter().enumerate() {
        if vec[..i].contains(elem) {
            return Err(DeserializeFailure::DuplicateKey(Key::Uint(i as u64)).into());
        }
    }
    Ok(())
}

impl<T: PartialEq> TryFrom<Vec<T>> for OrderedSet<T> {
    type Error = DeserializeError;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        scan_unique(&vec)?;
        Ok(Self(vec))
    }
}

impl<T> From<OrderedSet<T>> for Vec<T> {
    fn from(v: OrderedSet<T>) -> Self {
        v.0
    }
}

impl<T> AsRef<[T]> for OrderedSet<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T> std::ops::Index<usize> for OrderedSet<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, T> IntoIterator for &'a OrderedSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// A `Vec<T>` guaranteed to hold at least one element AND no duplicates — the uniqueness twin of
/// `NonEmptyVec<T>` for a `[+ T]` collection rule carrying `@duplicates reject`.
///
/// Its single `TryFrom<Vec<T>>` door composes BOTH invariants: it fails with the same
/// `DeserializeFailure::RangeCheck { min: Some(1), .. }` as `NonEmptyVec` for an empty input, and the
/// same `DuplicateKey(Key::Uint(i))` (duplicate's INDEX) as `OrderedSet` for a repeated element.
/// Order-preserving and never sorted, for the same byte-exact-round-trip reason as `OrderedSet`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonEmptyOrderedSet<T>(Vec<T>);

impl<T> NonEmptyOrderedSet<T> {
    /// Construct from a single element — always valid (length 1, trivially unique).
    pub fn new(first: T) -> Self {
        Self(vec![first])
    }

    /// Append an element, unless it is already present (uniqueness). Growing can never violate the
    /// `>= 1` lower bound, so the only failure is a duplicate.
    pub fn push(&mut self, value: T) -> Result<(), DeserializeError>
    where
        T: PartialEq,
    {
        if self.0.contains(&value) {
            return Err(DeserializeFailure::DuplicateKey(Key::Uint(self.0.len() as u64)).into());
        }
        self.0.push(value);
        Ok(())
    }

    /// Remove and return the last element, unless doing so would empty the collection (refused at
    /// length 1 so the non-emptiness invariant can never break). Removal cannot create a duplicate.
    pub fn pop(&mut self) -> Result<T, DeserializeError> {
        if self.0.len() <= 1 {
            return Err(DeserializeFailure::RangeCheck {
                found: 0,
                min: Some(1),
                max: None,
            }
            .into());
        }
        Ok(self.0.pop().unwrap())
    }

    /// Remove and return the element at `index`, unless doing so would empty the collection.
    pub fn remove(&mut self, index: usize) -> Result<T, DeserializeError> {
        if self.0.len() <= 1 {
            return Err(DeserializeFailure::RangeCheck {
                found: 0,
                min: Some(1),
                max: None,
            }
            .into());
        }
        Ok(self.0.remove(index))
    }

    /// Number of elements. Never zero.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Always `false` — a `NonEmptyOrderedSet` holds at least one element by construction. Present so
    /// the `len`/`is_empty` pair stays consistent (and clippy-clean).
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    /// The first element — always present.
    pub fn first(&self) -> &T {
        &self.0[0]
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    /// Escape hatch to the loose `Vec<T>` — mutate freely, then `try_into` back through the door.
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<T: PartialEq> TryFrom<Vec<T>> for NonEmptyOrderedSet<T> {
    type Error = DeserializeError;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.is_empty() {
            return Err(DeserializeFailure::RangeCheck {
                found: 0,
                min: Some(1),
                max: None,
            }
            .into());
        }
        scan_unique(&vec)?;
        Ok(Self(vec))
    }
}

impl<T> From<NonEmptyOrderedSet<T>> for Vec<T> {
    fn from(v: NonEmptyOrderedSet<T>) -> Self {
        v.0
    }
}

impl<T> AsRef<[T]> for NonEmptyOrderedSet<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T> std::ops::Index<usize> for NonEmptyOrderedSet<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyOrderedSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<T: serde::Serialize> serde::Serialize for OrderedSet<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // an OrderedSet is a plain JSON array — serialize as the loose Vec would
        self.as_slice().serialize(serializer)
    }
}

impl<'de, T: serde::Deserialize<'de> + PartialEq> serde::de::Deserialize<'de> for OrderedSet<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        // never derive a structural Deserialize (it would bypass the uniqueness invariant):
        // deserialize the loose Vec and route through the same TryFrom door as every other path.
        let vec = <Vec<T> as serde::de::Deserialize>::deserialize(deserializer)?;
        OrderedSet::try_from(vec).map_err(serde::de::Error::custom)
    }
}

impl<T: serde::Serialize> serde::Serialize for NonEmptyOrderedSet<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_slice().serialize(serializer)
    }
}

impl<'de, T: serde::Deserialize<'de> + PartialEq> serde::de::Deserialize<'de>
    for NonEmptyOrderedSet<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let vec = <Vec<T> as serde::de::Deserialize>::deserialize(deserializer)?;
        NonEmptyOrderedSet::try_from(vec).map_err(serde::de::Error::custom)
    }
}
impl<T: schemars::JsonSchema> schemars::JsonSchema for OrderedSet<T> {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        format!("OrderedSet<{}>", T::schema_name()).into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // shape matches the loose Vec (a JSON array); the uniqueness invariant is enforced at TryFrom.
        // Deliberately NOT refined with `uniqueItems: true`: the sibling NonEmptyVec schemars impl sets
        // the convention of delegating to the Vec schema without invariant refinements (its `>= 1`
        // bound is likewise not surfaced as `minItems`) — the door is the single source of the
        // invariant, and json2ts emits the same array type either way.
        Vec::<T>::json_schema(generator)
    }
    fn inline_schema() -> bool {
        Vec::<T>::inline_schema()
    }
}

impl<T: schemars::JsonSchema> schemars::JsonSchema for NonEmptyOrderedSet<T> {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        format!("NonEmptyOrderedSet<{}>", T::schema_name()).into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // shape matches the loose Vec; uniqueness + non-emptiness are enforced at TryFrom.
        Vec::<T>::json_schema(generator)
    }
    fn inline_schema() -> bool {
        Vec::<T>::inline_schema()
    }
}
