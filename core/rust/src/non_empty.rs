use super::error::{DeserializeError, DeserializeFailure};

/// A `Vec<T>` guaranteed to hold at least one element — the restricted twin of `Vec<T>` for the
/// CDDL `[+ T]` occurrence (`+` / `1*`).
///
/// The non-emptiness is enforced ONCE, at the single `TryFrom<Vec<T>>` door, and thereafter cannot
/// be broken: there is no public field, no `&mut Vec<T>` and no length-shrinking access that could
/// drop below one element (removal is checked). Construction from a `Vec` that is empty fails with
/// the same `DeserializeFailure::RangeCheck { min: Some(1), max: None }` the CBOR decoder raises, so
/// the API door and the wire door report identical errors.
///
/// Staging an eventually-non-empty collection is done with a plain `Vec<T>` (`push`/`extend`/
/// `collect`) and a final `NonEmptyVec::try_from(vec)` — the loose `Vec` is the builder, so no
/// generated builder type is needed.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonEmptyVec<T>(Vec<T>);

impl<T> NonEmptyVec<T> {
    /// Construct from a single element — always valid (length 1).
    pub fn new(first: T) -> Self {
        Self(vec![first])
    }

    /// Append an element. Infallible: growing can never violate the `>= 1` lower bound.
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    /// Append the contents of an iterator. Infallible for the same reason as `push`.
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }

    /// Remove and return the last element, unless doing so would empty the collection (i.e. it is
    /// refused at length 1 so the non-emptiness invariant can never break).
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

    /// Always `false` — a `NonEmptyVec` holds at least one element by construction. Present so the
    /// `len`/`is_empty` pair stays consistent (and clippy-clean).
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

    /// Mutable iteration over elements. A slice iterator cannot change the length, so the invariant
    /// is preserved (the largest mutable view the invariant cannot see).
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }

    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    /// Mutable slice view. Slices cannot change length, so element edits stay within the invariant.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.0.as_mut_slice()
    }

    /// Escape hatch to the loose `Vec<T>` — mutate freely, then `try_into` back.
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = DeserializeError;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.is_empty() {
            Err(DeserializeFailure::RangeCheck {
                found: 0,
                min: Some(1),
                max: None,
            }
            .into())
        } else {
            Ok(Self(vec))
        }
    }
}

impl<T> From<NonEmptyVec<T>> for Vec<T> {
    fn from(v: NonEmptyVec<T>) -> Self {
        v.0
    }
}

impl<T> AsRef<[T]> for NonEmptyVec<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_slice()
    }
}

impl<T> std::ops::Index<usize> for NonEmptyVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> std::ops::IndexMut<usize> for NonEmptyVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<T: serde::Serialize> serde::Serialize for NonEmptyVec<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // a NonEmptyVec is a plain JSON array — serialize as the loose Vec would
        self.as_slice().serialize(serializer)
    }
}

impl<'de, T: serde::Deserialize<'de>> serde::de::Deserialize<'de> for NonEmptyVec<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        // never derive a structural Deserialize (it would bypass the invariant): deserialize the
        // loose Vec and route through the same TryFrom door as every other construction path.
        let vec = <Vec<T> as serde::de::Deserialize>::deserialize(deserializer)?;
        NonEmptyVec::try_from(vec).map_err(serde::de::Error::custom)
    }
}
impl<T: schemars::JsonSchema> schemars::JsonSchema for NonEmptyVec<T> {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        format!("NonEmptyVec<{}>", T::schema_name()).into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // shape matches the loose Vec (a JSON array); the >= 1 invariant is enforced at TryFrom
        Vec::<T>::json_schema(generator)
    }
    fn inline_schema() -> bool {
        Vec::<T>::inline_schema()
    }
}
