use super::error::{DeserializeError, DeserializeFailure};
use super::serialization::*;

macro_rules! any_cbor_recursion_guard {
    () => {};
}

// `AnyCbor` — a structured, self-describing CBOR value (the runtime lowering of CDDL `any`).
//
// PRESERVE variant (this file is concatenated only when `--preserve-encodings` is on). Every
// value carries the encoding detail needed to re-emit the ORIGINAL bytes exactly: integer/tag
// argument widths (`Sz`), string chunking (`StringEncoding`), and array/map length encoding
// (`LenEncoding`). Contract:
//   * `serialize` with `force_canonical = false` reproduces the deserialized bytes BYTE-EXACTLY;
//   * `serialize` with `force_canonical = true` produces RFC 8949 §4.2 deterministic encoding
//     (smallest widths, recursively length-first-then-bytewise map key ordering);
//   * `deserialize` accepts any single well-formed CBOR item and leaves the read cursor exactly
//     at the item's end (never over-reads trailing bytes — the cip36 skip-bug class).
//
// Equality/ordering/hashing are REPRESENTATIONAL: two values with equal CBOR content but
// different encoding (`0x01` vs `0x1801`, both the integer 1) compare UNEQUAL here, because a
// byte-preserving map key must not silently collide. This matches the in-tree precedent of
// `@used_as_key` preserve structs, whose derived `Eq`/`Hash` include their `encodings` field.
// (The non-preserve variant, `any_cbor_non_preserve.rs`, compares VALUE only.)
//
// Depth: `deserialize` recurses for nested arrays/maps/tags. It routes ALL recursion through the
// single `read` seam below, whose first line invokes `any_cbor_recursion_guard!()`. The includer
// supplies that macro: the static assembly in generated crates expands it to
// `DepthGuard::acquire(<baked limit>)?` under `--deserialize-depth-limit` (and to nothing without
// the flag, so no-flag crates carry no dead runtime code and keep byte-identical output); the
// property-harness shims supply their own definition to exercise the guard. The guard shares the
// generated composite deserializers' thread-local depth counter, so the whole nesting — struct
// and its `any` members alike — is bounded uniformly. What IS guaranteed here without the flag
// matches the rest of the crate: bounded allocation (never `Vec::with_capacity` from an
// attacker-claimed length), a dangling `Break` rejected as a value, and truncated/malformed input
// returning `Err`, never a panic.
#[derive(Clone, Debug)]
pub enum AnyCbor {
    UInt(u64, Option<cbor_event::Sz>),
    NInt(i128, Option<cbor_event::Sz>), // full CBOR nint domain (-2^64..=-1), from negative_integer_sz
    Bytes(Vec<u8>, StringEncoding),
    Text(String, StringEncoding),
    Array(Vec<AnyCbor>, LenEncoding),
    Map(Vec<(AnyCbor, AnyCbor)>, LenEncoding), // wire order AND duplicate keys preserved
    Tag(u64, Box<AnyCbor>, Option<cbor_event::Sz>),
    Special(AnySpecial),
}

#[derive(Clone, Debug)]
pub enum AnySpecial {
    Bool(bool),
    Null,
    Undefined,
    /// unassigned simple value: 0..=19 (single-byte) or 32..=255 (two-byte `0xf8` form); each
    /// has exactly one well-formed encoding, so no width slot is needed for byte-exactness.
    Unassigned(u8),
    /// `Sz::Two`/`Four`/`Eight` = the wire width (byte-exact replay via `write_float_sz`);
    /// `None` = emit the canonical smallest-width form (a value constructed in Rust).
    Float(f64, Option<cbor_event::Sz>),
}

/// Lightweight discriminant for `AnyCbor::kind()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnyCborKind {
    UInt,
    NInt,
    Bytes,
    Text,
    Array,
    Map,
    Tag,
    Bool,
    Null,
    Undefined,
    Unassigned,
    Float,
}

impl AnyCbor {
    /// The CBOR major-type/special discriminant of this value.
    pub fn kind(&self) -> AnyCborKind {
        match self {
            AnyCbor::UInt(..) => AnyCborKind::UInt,
            AnyCbor::NInt(..) => AnyCborKind::NInt,
            AnyCbor::Bytes(..) => AnyCborKind::Bytes,
            AnyCbor::Text(..) => AnyCborKind::Text,
            AnyCbor::Array(..) => AnyCborKind::Array,
            AnyCbor::Map(..) => AnyCborKind::Map,
            AnyCbor::Tag(..) => AnyCborKind::Tag,
            AnyCbor::Special(AnySpecial::Bool(_)) => AnyCborKind::Bool,
            AnyCbor::Special(AnySpecial::Null) => AnyCborKind::Null,
            AnyCbor::Special(AnySpecial::Undefined) => AnyCborKind::Undefined,
            AnyCbor::Special(AnySpecial::Unassigned(_)) => AnyCborKind::Unassigned,
            AnyCbor::Special(AnySpecial::Float(..)) => AnyCborKind::Float,
        }
    }

    pub fn as_uint(&self) -> Option<u64> {
        match self {
            AnyCbor::UInt(v, _) => Some(*v),
            _ => None,
        }
    }

    pub fn as_nint(&self) -> Option<i128> {
        match self {
            AnyCbor::NInt(v, _) => Some(*v),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            AnyCbor::Bytes(v, _) => Some(v),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            AnyCbor::Text(v, _) => Some(v),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[AnyCbor]> {
        match self {
            AnyCbor::Array(v, _) => Some(v),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&[(AnyCbor, AnyCbor)]> {
        match self {
            AnyCbor::Map(v, _) => Some(v),
            _ => None,
        }
    }

    pub fn as_tag(&self) -> Option<(u64, &AnyCbor)> {
        match self {
            AnyCbor::Tag(t, inner, _) => Some((*t, inner)),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            AnyCbor::Special(AnySpecial::Float(f, _)) => Some(*f),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AnyCbor::Special(AnySpecial::Bool(b)) => Some(*b),
            _ => None,
        }
    }

    /// The simple-value code of an `unassigned` special (0..=19 or 32..=255), else `None`. Needed by
    /// the JSON surface's reverse mapping (there is no other way to read the code back out).
    pub fn as_unassigned(&self) -> Option<u8> {
        match self {
            AnyCbor::Special(AnySpecial::Unassigned(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, AnyCbor::Special(AnySpecial::Null))
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, AnyCbor::Special(AnySpecial::Undefined))
    }

    // --- constructors (mode-paired with the non-preserve variant; SAME names/signatures). Each fills
    // the DEFAULT encoding (`None`/`Canonical`), i.e. the encoding a Rust-constructed value carries:
    // serialize (non-canonical) then emits the canonical smallest-width form, exactly like a value the
    // generated code built itself. These are the value-building surface the JSON deserialize path, the
    // wasm wrapper, and the emit-tests mint all construct through. ---

    pub fn new_uint(value: u64) -> Self {
        AnyCbor::UInt(value, None)
    }

    /// `value` must lie in the CBOR nint domain `-2^64..=-1`; out-of-domain is a debug-assert (no
    /// clamping — the caller is responsible, mirroring the crate-wide "no silent coercion" stance).
    pub fn new_nint(value: i128) -> Self {
        debug_assert!(
            (-(1i128 << 64)..=-1).contains(&value),
            "AnyCbor::new_nint: {value} outside the CBOR nint domain -2^64..=-1"
        );
        AnyCbor::NInt(value, None)
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        AnyCbor::Bytes(bytes, StringEncoding::Canonical)
    }

    pub fn new_text(text: String) -> Self {
        AnyCbor::Text(text, StringEncoding::Canonical)
    }

    pub fn new_array(elems: Vec<AnyCbor>) -> Self {
        AnyCbor::Array(elems, LenEncoding::Canonical)
    }

    pub fn new_map(pairs: Vec<(AnyCbor, AnyCbor)>) -> Self {
        AnyCbor::Map(pairs, LenEncoding::Canonical)
    }

    pub fn new_tag(tag: u64, inner: AnyCbor) -> Self {
        AnyCbor::Tag(tag, Box::new(inner), None)
    }

    pub fn new_bool(b: bool) -> Self {
        AnyCbor::Special(AnySpecial::Bool(b))
    }

    pub fn new_null() -> Self {
        AnyCbor::Special(AnySpecial::Null)
    }

    pub fn new_undefined() -> Self {
        AnyCbor::Special(AnySpecial::Undefined)
    }

    pub fn new_unassigned(code: u8) -> Self {
        AnyCbor::Special(AnySpecial::Unassigned(code))
    }

    /// Canonical-width float: stores `None` per the `Float` slot's documented contract (emit the
    /// canonical smallest-width form for a Rust-constructed value).
    pub fn new_float(f: f64) -> Self {
        AnyCbor::Special(AnySpecial::Float(f, None))
    }

    /// Byte-exact re-encoding (replays stored encodings). Never fails for a value produced by
    /// `deserialize`; the `Result` is for symmetry with the serializer.
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        let mut buf = cbor_event::se::Serializer::new_vec();
        self.serialize_ref(&mut buf, false).unwrap();
        buf.finalize()
    }

    /// RFC 8949 §4.2 deterministic re-encoding (smallest widths, recursively sorted map keys).
    pub fn to_canonical_cbor_bytes(&self) -> Vec<u8> {
        let mut buf = cbor_event::se::Serializer::new_vec();
        self.serialize_ref(&mut buf, true).unwrap();
        buf.finalize()
    }

    /// Value-level (encoding-independent) equality — the duplicate-key comparison an `any`-domain
    /// open struct-map rest row / table needs under `--preserve-encodings`, where the derived `Eq`
    /// is REPRESENTATIONAL (encoding fields participate, so two equal values with different wire
    /// widths compare `!=`). Two values are value-equal iff their CANONICAL encodings match:
    /// canonicalization is the encoding-independent normal form (RFC 7049 §3.9 minimal widths,
    /// recursively normalized map key order), so `0x01` and `0x1801` (both uint 1) are value-equal
    /// though representationally distinct. Accept/reject of a duplicate wire key must
    /// be a function of the wire VALUE, not of the key domain's spelling (`* uint => any` and
    /// `* any => any` reject the same duplicates), so the default-reject dup check compares this,
    /// separately from the container's representational keying.
    pub fn value_eq(&self, other: &Self) -> bool {
        self.to_canonical_cbor_bytes() == other.to_canonical_cbor_bytes()
    }

    /// The (de)serialization workhorse. The mode-appropriate trait impls
    /// (`any_cbor_preserve_non_force_canonical.rs` / `any_cbor_preserve_force_canonical.rs`)
    /// delegate here. Self-contained: computes every `Sz`/`LenSz`/`StringLenSz` from the stored
    /// encoding fields, so it does not depend on the `fit_sz`/`to_len_sz` fragment helpers whose
    /// arity differs between the two preserve assemblies.
    pub fn serialize_ref<'a>(
        &self,
        serializer: &'a mut cbor_event::se::Serializer,
        force_canonical: bool,
    ) -> cbor_event::Result<&'a mut cbor_event::se::Serializer> {
        match self {
            AnyCbor::UInt(v, sz) => {
                serializer.write_unsigned_integer_sz(*v, fit_int_sz(*v, sz, force_canonical))?;
            }
            AnyCbor::NInt(v, sz) => {
                // encoded argument = -(v) - 1 in the range 0..=2^64-1
                let arg = (-((*v) + 1)) as u64;
                serializer.write_negative_integer_sz(*v, fit_int_sz(arg, sz, force_canonical))?;
            }
            AnyCbor::Bytes(bytes, enc) => {
                serializer
                    .write_bytes_sz(bytes, str_len_sz(enc, bytes.len() as u64, force_canonical))?;
            }
            AnyCbor::Text(text, enc) => {
                serializer
                    .write_text_sz(text, str_len_sz(enc, text.len() as u64, force_canonical))?;
            }
            AnyCbor::Array(elems, enc) => {
                serializer.write_array_sz(len_sz(enc, elems.len() as u64, force_canonical))?;
                for elem in elems.iter() {
                    elem.serialize_ref(serializer, force_canonical)?;
                }
                len_end(serializer, enc, force_canonical)?;
            }
            AnyCbor::Map(pairs, enc) => {
                serializer.write_map_sz(len_sz(enc, pairs.len() as u64, force_canonical))?;
                if force_canonical {
                    // Sort by canonically-encoded key, length-first-then-bytewise. STABLE sort so
                    // equal-keyed (duplicate) entries keep first-appearance order.
                    let mut ordered: Vec<(Vec<u8>, &AnyCbor)> = pairs
                        .iter()
                        .map(|(k, v)| {
                            let mut key_buf = cbor_event::se::Serializer::new_vec();
                            k.serialize_ref(&mut key_buf, true)?;
                            Ok((key_buf.finalize(), v))
                        })
                        .collect::<cbor_event::Result<Vec<_>>>()?;
                    ordered.sort_by(|(lhs, _), (rhs, _)| cbor_canonical_key_cmp(lhs, rhs));
                    for (key_bytes, value) in ordered.iter() {
                        serializer.write_raw_bytes(key_bytes)?;
                        value.serialize_ref(serializer, true)?;
                    }
                } else {
                    for (key, value) in pairs.iter() {
                        key.serialize_ref(serializer, false)?;
                        value.serialize_ref(serializer, false)?;
                    }
                }
                len_end(serializer, enc, force_canonical)?;
            }
            AnyCbor::Tag(tag, inner, sz) => {
                serializer.write_tag_sz(*tag, fit_int_sz(*tag, sz, force_canonical))?;
                inner.serialize_ref(serializer, force_canonical)?;
            }
            AnyCbor::Special(special) => {
                serialize_special(serializer, special, force_canonical)?;
            }
        }
        Ok(serializer)
    }

    /// Recursion seam for deserialize. All nested reads go through here so a depth guard can be
    /// threaded at one place (see the file header).
    fn read(raw: &mut cbor_event::de::Deserializer) -> Result<Self, DeserializeError> {
        // Depth-guard hook: expands to a `DepthGuard::acquire(<baked limit>)?` RAII binding when the
        // generated crate is built with `--deserialize-depth-limit`, or to nothing otherwise. The
        // macro is supplied by the includer (the static assembly for generated crates; the test
        // shims for the property harness) so this one file serves every flag combination unsplit.
        any_cbor_recursion_guard!();
        match raw.cbor_type()? {
            cbor_event::Type::UnsignedInteger => {
                let (v, sz) = raw.unsigned_integer_sz()?;
                Ok(AnyCbor::UInt(v, Some(sz)))
            }
            cbor_event::Type::NegativeInteger => {
                let (v, sz) = raw.negative_integer_sz()?;
                Ok(AnyCbor::NInt(v, Some(sz)))
            }
            cbor_event::Type::Bytes => {
                let (bytes, len_sz) = raw.bytes_sz()?;
                Ok(AnyCbor::Bytes(bytes, len_sz.into()))
            }
            cbor_event::Type::Text => {
                let (text, len_sz) = raw.text_sz()?;
                Ok(AnyCbor::Text(text, len_sz.into()))
            }
            cbor_event::Type::Array => {
                let len = raw.array_sz()?;
                let mut elems = Vec::new(); // never with_capacity(claimed len)
                read_sequence(raw, len, |raw| {
                    elems.push(AnyCbor::read(raw)?);
                    Ok(())
                })?;
                Ok(AnyCbor::Array(elems, len.into()))
            }
            cbor_event::Type::Map => {
                let len = raw.map_sz()?;
                let mut pairs = Vec::new();
                read_sequence(raw, len, |raw| {
                    let key = AnyCbor::read(raw)?;
                    let value = AnyCbor::read(raw)?;
                    pairs.push((key, value));
                    Ok(())
                })?;
                Ok(AnyCbor::Map(pairs, len.into()))
            }
            cbor_event::Type::Tag => {
                let (tag, sz) = raw.tag_sz()?;
                let inner = AnyCbor::read(raw)?;
                Ok(AnyCbor::Tag(tag, Box::new(inner), Some(sz)))
            }
            cbor_event::Type::Special => {
                // Distinguish float (0x19/0x1a/0x1b) from other specials by the header low bits;
                // reject a dangling Break (0x1f) used as a value.
                let head = *raw
                    .as_slice()
                    .first()
                    .ok_or_else(|| DeserializeFailure::CBOR(cbor_event::Error::NotEnough(0, 1)))?;
                match head & 0b0001_1111 {
                    0x19..=0x1b => {
                        let (f, sz) = raw.float_sz()?;
                        Ok(AnyCbor::Special(AnySpecial::Float(f, Some(sz))))
                    }
                    0x1f => {
                        Err(DeserializeFailure::CBOR(cbor_event::Error::UnexpectedBreak).into())
                    }
                    _ => match raw.special()? {
                        cbor_event::Special::Bool(b) => Ok(AnyCbor::Special(AnySpecial::Bool(b))),
                        cbor_event::Special::Null => Ok(AnyCbor::Special(AnySpecial::Null)),
                        cbor_event::Special::Undefined => {
                            Ok(AnyCbor::Special(AnySpecial::Undefined))
                        }
                        cbor_event::Special::Unassigned(v) => {
                            Ok(AnyCbor::Special(AnySpecial::Unassigned(v)))
                        }
                        // floats were routed above; Break was rejected above
                        cbor_event::Special::Float(_) | cbor_event::Special::Break => {
                            Err(DeserializeFailure::CBOR(cbor_event::Error::UnexpectedBreak).into())
                        }
                    },
                }
            }
        }
    }
}

impl Deserialize for AnyCbor {
    fn deserialize(raw: &mut cbor_event::de::Deserializer) -> Result<Self, DeserializeError> {
        AnyCbor::read(raw)
    }
}

/// Drive a definite- or indefinite-length CBOR sequence, invoking `read_one` for each element
/// (one item for arrays, called twice per entry by the map caller). Definite lengths bound the
/// loop by COUNT only — allocation grows with actual items read, so a hostile huge claimed length
/// errors when the buffer is exhausted rather than pre-allocating.
fn read_sequence<F>(
    raw: &mut cbor_event::de::Deserializer,
    len: cbor_event::LenSz,
    mut read_one: F,
) -> Result<(), DeserializeError>
where
    F: FnMut(&mut cbor_event::de::Deserializer) -> Result<(), DeserializeError>,
{
    match len {
        cbor_event::LenSz::Len(n, _) => {
            for _ in 0..n {
                read_one(raw)?;
            }
        }
        cbor_event::LenSz::Indefinite => loop {
            if raw.cbor_type()? == cbor_event::Type::Special && raw.special_break()? {
                break;
            }
            read_one(raw)?;
        },
    }
    Ok(())
}

// --- serialize helpers (self-contained; mirror fit_sz/to_len_sz/to_str_len_sz without depending
//     on the arity-varying fragment helpers) ---

fn fit_int_sz(arg: u64, sz: &Option<cbor_event::Sz>, force_canonical: bool) -> cbor_event::Sz {
    match sz {
        Some(sz) if !force_canonical && arg <= sz_max(*sz) => *sz,
        _ => cbor_event::Sz::canonical(arg),
    }
}

fn len_sz(enc: &LenEncoding, len: u64, force_canonical: bool) -> cbor_event::LenSz {
    if force_canonical {
        return cbor_event::LenSz::Len(len, cbor_event::Sz::canonical(len));
    }
    match enc {
        LenEncoding::Canonical => cbor_event::LenSz::Len(len, cbor_event::Sz::canonical(len)),
        LenEncoding::Definite(sz) if sz_max(*sz) >= len => cbor_event::LenSz::Len(len, *sz),
        LenEncoding::Definite(_) => cbor_event::LenSz::Len(len, cbor_event::Sz::canonical(len)),
        LenEncoding::Indefinite => cbor_event::LenSz::Indefinite,
    }
}

fn len_end<'a>(
    serializer: &'a mut cbor_event::se::Serializer,
    enc: &LenEncoding,
    force_canonical: bool,
) -> cbor_event::Result<&'a mut cbor_event::se::Serializer> {
    if !force_canonical && *enc == LenEncoding::Indefinite {
        serializer.write_special(cbor_event::Special::Break)?;
    }
    Ok(serializer)
}

fn str_len_sz(enc: &StringEncoding, len: u64, force_canonical: bool) -> cbor_event::StringLenSz {
    if force_canonical {
        return cbor_event::StringLenSz::Len(cbor_event::Sz::canonical(len));
    }
    match enc {
        StringEncoding::Canonical => cbor_event::StringLenSz::Len(cbor_event::Sz::canonical(len)),
        StringEncoding::Definite(sz) if sz_max(*sz) >= len => cbor_event::StringLenSz::Len(*sz),
        StringEncoding::Definite(_) => cbor_event::StringLenSz::Len(cbor_event::Sz::canonical(len)),
        StringEncoding::Indefinite(lens) => cbor_event::StringLenSz::Indefinite(lens.clone()),
    }
}

fn serialize_special<'a>(
    serializer: &'a mut cbor_event::se::Serializer,
    special: &AnySpecial,
    force_canonical: bool,
) -> cbor_event::Result<&'a mut cbor_event::se::Serializer> {
    match special {
        AnySpecial::Bool(b) => serializer.write_special(cbor_event::Special::Bool(*b)),
        AnySpecial::Null => serializer.write_special(cbor_event::Special::Null),
        AnySpecial::Undefined => serializer.write_special(cbor_event::Special::Undefined),
        AnySpecial::Unassigned(v) => serializer.write_special(cbor_event::Special::Unassigned(*v)),
        AnySpecial::Float(f, sz) => {
            if force_canonical && f.is_nan() {
                // RFC 8949 §4.2.2: the canonical NaN is the zero-payload quiet NaN `f9 7e00`
                // (drop any payload). `cbor_event::se::smallest_float_sz` would instead shorten to
                // the narrowest width that preserves the PAYLOAD (per its own doc comment), so a
                // strictly-canonical writer must special-case NaN first.
                // (write_float_sz(f64::NAN, Two) narrows the standard quiet NaN to f16 0x7e00.)
                return serializer.write_float_sz(f64::NAN, cbor_event::Sz::Two);
            }
            // `smallest_float_sz` is the RFC 8949 §4.2.1 value-preserving smallest width (NaN
            // payload included). The NaN-drops-payload canonical case is handled above, so here it
            // only ever runs on non-NaN (force_canonical) or any value (non-canonical None).
            let width = match (force_canonical, sz) {
                (true, _) => cbor_event::se::smallest_float_sz(*f),
                (false, Some(sz)) => *sz, // replay stored width byte-exactly
                (false, None) => cbor_event::se::smallest_float_sz(*f), // payload-preserving smallest
            };
            serializer.write_float_sz(*f, width)
        }
    }
}

// --- representational Eq / Ord / Hash (encoding fields participate) ---

fn sz_rank(sz: cbor_event::Sz) -> u8 {
    match sz {
        cbor_event::Sz::Inline => 0,
        cbor_event::Sz::One => 1,
        cbor_event::Sz::Two => 2,
        cbor_event::Sz::Four => 3,
        cbor_event::Sz::Eight => 4,
    }
}

fn opt_sz_rank(sz: &Option<cbor_event::Sz>) -> (u8, u8) {
    match sz {
        None => (0, 0),
        Some(sz) => (1, sz_rank(*sz)),
    }
}

fn len_enc_rank(enc: &LenEncoding) -> (u8, u8) {
    match enc {
        LenEncoding::Canonical => (0, 0),
        LenEncoding::Definite(sz) => (1, sz_rank(*sz)),
        LenEncoding::Indefinite => (2, 0),
    }
}

fn str_enc_rank(enc: &StringEncoding) -> (u8, Vec<(u64, u8)>) {
    match enc {
        StringEncoding::Canonical => (0, Vec::new()),
        StringEncoding::Definite(sz) => (1, vec![(0, sz_rank(*sz))]),
        StringEncoding::Indefinite(lens) => {
            (2, lens.iter().map(|(l, sz)| (*l, sz_rank(*sz))).collect())
        }
    }
}

impl PartialEq for AnySpecial {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AnySpecial::Bool(a), AnySpecial::Bool(b)) => a == b,
            (AnySpecial::Null, AnySpecial::Null) => true,
            (AnySpecial::Undefined, AnySpecial::Undefined) => true,
            (AnySpecial::Unassigned(a), AnySpecial::Unassigned(b)) => a == b,
            (AnySpecial::Float(a, sa), AnySpecial::Float(b, sb)) => {
                a.to_bits() == b.to_bits() && sa == sb
            }
            _ => false,
        }
    }
}
impl Eq for AnySpecial {}

impl PartialEq for AnyCbor {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AnyCbor::UInt(a, sa), AnyCbor::UInt(b, sb)) => a == b && sa == sb,
            (AnyCbor::NInt(a, sa), AnyCbor::NInt(b, sb)) => a == b && sa == sb,
            (AnyCbor::Bytes(a, ea), AnyCbor::Bytes(b, eb)) => a == b && ea == eb,
            (AnyCbor::Text(a, ea), AnyCbor::Text(b, eb)) => a == b && ea == eb,
            (AnyCbor::Array(a, ea), AnyCbor::Array(b, eb)) => a == b && ea == eb,
            (AnyCbor::Map(a, ea), AnyCbor::Map(b, eb)) => a == b && ea == eb,
            (AnyCbor::Tag(a, ia, sa), AnyCbor::Tag(b, ib, sb)) => a == b && ia == ib && sa == sb,
            (AnyCbor::Special(a), AnyCbor::Special(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for AnyCbor {}

impl std::hash::Hash for AnySpecial {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            AnySpecial::Bool(b) => b.hash(state),
            AnySpecial::Null | AnySpecial::Undefined => {}
            AnySpecial::Unassigned(v) => v.hash(state),
            AnySpecial::Float(f, sz) => {
                f.to_bits().hash(state);
                opt_sz_rank(sz).hash(state);
            }
        }
    }
}

impl std::hash::Hash for AnyCbor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            AnyCbor::UInt(v, sz) => {
                v.hash(state);
                opt_sz_rank(sz).hash(state);
            }
            AnyCbor::NInt(v, sz) => {
                v.hash(state);
                opt_sz_rank(sz).hash(state);
            }
            AnyCbor::Bytes(v, enc) => {
                v.hash(state);
                str_enc_rank(enc).hash(state);
            }
            AnyCbor::Text(v, enc) => {
                v.hash(state);
                str_enc_rank(enc).hash(state);
            }
            AnyCbor::Array(v, enc) => {
                v.hash(state);
                len_enc_rank(enc).hash(state);
            }
            AnyCbor::Map(v, enc) => {
                v.hash(state);
                len_enc_rank(enc).hash(state);
            }
            AnyCbor::Tag(t, inner, sz) => {
                t.hash(state);
                inner.hash(state);
                opt_sz_rank(sz).hash(state);
            }
            AnyCbor::Special(s) => s.hash(state),
        }
    }
}

fn any_special_ord_rank(s: &AnySpecial) -> u8 {
    match s {
        AnySpecial::Bool(_) => 0,
        AnySpecial::Null => 1,
        AnySpecial::Undefined => 2,
        AnySpecial::Unassigned(_) => 3,
        AnySpecial::Float(..) => 4,
    }
}

impl Ord for AnySpecial {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        any_special_ord_rank(self)
            .cmp(&any_special_ord_rank(other))
            .then_with(|| match (self, other) {
                (AnySpecial::Bool(a), AnySpecial::Bool(b)) => a.cmp(b),
                (AnySpecial::Unassigned(a), AnySpecial::Unassigned(b)) => a.cmp(b),
                (AnySpecial::Float(a, sa), AnySpecial::Float(b, sb)) => a
                    .total_cmp(b)
                    .then_with(|| opt_sz_rank(sa).cmp(&opt_sz_rank(sb))),
                _ => std::cmp::Ordering::Equal,
            })
    }
}
impl PartialOrd for AnySpecial {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn any_cbor_ord_rank(v: &AnyCbor) -> u8 {
    match v {
        AnyCbor::UInt(..) => 0,
        AnyCbor::NInt(..) => 1,
        AnyCbor::Bytes(..) => 2,
        AnyCbor::Text(..) => 3,
        AnyCbor::Array(..) => 4,
        AnyCbor::Map(..) => 5,
        AnyCbor::Tag(..) => 6,
        AnyCbor::Special(..) => 7,
    }
}

impl Ord for AnyCbor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        any_cbor_ord_rank(self)
            .cmp(&any_cbor_ord_rank(other))
            .then_with(|| match (self, other) {
                (AnyCbor::UInt(a, sa), AnyCbor::UInt(b, sb)) => {
                    a.cmp(b).then_with(|| opt_sz_rank(sa).cmp(&opt_sz_rank(sb)))
                }
                (AnyCbor::NInt(a, sa), AnyCbor::NInt(b, sb)) => {
                    a.cmp(b).then_with(|| opt_sz_rank(sa).cmp(&opt_sz_rank(sb)))
                }
                (AnyCbor::Bytes(a, ea), AnyCbor::Bytes(b, eb)) => a
                    .cmp(b)
                    .then_with(|| str_enc_rank(ea).cmp(&str_enc_rank(eb))),
                (AnyCbor::Text(a, ea), AnyCbor::Text(b, eb)) => a
                    .cmp(b)
                    .then_with(|| str_enc_rank(ea).cmp(&str_enc_rank(eb))),
                (AnyCbor::Array(a, ea), AnyCbor::Array(b, eb)) => a
                    .cmp(b)
                    .then_with(|| len_enc_rank(ea).cmp(&len_enc_rank(eb))),
                (AnyCbor::Map(a, ea), AnyCbor::Map(b, eb)) => a
                    .cmp(b)
                    .then_with(|| len_enc_rank(ea).cmp(&len_enc_rank(eb))),
                (AnyCbor::Tag(a, ia, sa), AnyCbor::Tag(b, ib, sb)) => a
                    .cmp(b)
                    .then_with(|| ia.cmp(ib))
                    .then_with(|| opt_sz_rank(sa).cmp(&opt_sz_rank(sb))),
                (AnyCbor::Special(a), AnyCbor::Special(b)) => a.cmp(b),
                _ => std::cmp::Ordering::Equal,
            })
    }
}
impl PartialOrd for AnyCbor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
// `AnyCbor` serialize impl for the preserve + canonical assembly, where generated types implement
// the crate-local `Serialize` trait (the one taking `force_canonical: bool`). Delegates to the
// mode-independent workhorse, threading `force_canonical` through so `to_canonical_cbor_bytes`
// (the local trait's default method) produces deterministic encoding. (Paired with
// `any_cbor_preserve.rs`; the non-canonical assembly uses
// `any_cbor_preserve_non_force_canonical.rs` instead.)
impl Serialize for AnyCbor {
    fn serialize<'a>(
        &self,
        serializer: &'a mut cbor_event::se::Serializer,
        force_canonical: bool,
    ) -> cbor_event::Result<&'a mut cbor_event::se::Serializer> {
        self.serialize_ref(serializer, force_canonical)
    }
}
// Manual serde `Serialize`/`Deserialize` for `AnyCbor` — a JSON *representation of CBOR*, not
// "natural" JSON. Written ONCE against the mode-independent surface (`kind()`/`as_*` accessors +
// the `new_*` constructors), so this single fragment serves BOTH the preserve and non-preserve
// assemblies (mirroring how `ordered_hash_map_json.rs` is mode-agnostic). Encoding fields NEVER
// appear in JSON — JSON is the deliberately lossy side of the contract (a preserve value survives
// value-equal modulo encodings; a non-preserve value survives exactly for all finite floats).
//
// Rendering: every value is a single-key object whose key is the snake_case kind name:
//   uint       {"uint": 5}                 JSON number (u64 range, as serde_json emits crate-wide)
//   nint       {"nint": -3}                JSON number when the value fits i64,
//              {"nint": "-18446744073709551616"}  else a decimal string (the nint domain exceeds i64)
//   bytes      {"bytes": "a1b2"}           lowercase hex
//   text       {"text": "…"}
//   array      {"array": [ … ]}            recursive
//   map        {"map": [[K, V], …]}        array of pairs — wire order + duplicate keys preserved,
//                                          non-string keys representable
//   tag        {"tag": [11, V]}
//   bool       {"bool": true}
//   null       {"null": null}
//   undefined  {"undefined": null}
//   unassigned {"unassigned": 250}
//   float      {"float": 1.5}              finite floats as numbers;
//              {"float": "NaN"|"Infinity"|"-Infinity"}  non-finite as strings (serde_json cannot
//                                          represent them as numbers). NaN payload bits are NOT
//                                          round-tripped through JSON (the lossy-side charter).
//
// Map-key note (matches the crate-wide non-string-key-table posture): serde_json
// requires MAP keys to be strings. A `{* uint => any}` / `{* text => any}` table works (uint keys
// stringify, text keys verbatim); a `{* any => any}` (or any non-string-keyed) table serializes each
// key as an OBJECT, so `to_json` errors at runtime with "key must be a string" — exactly as a
// `{* bytes => uint}` table already does today. This is intentional consistency, not a new rule:
// generation accepts such tables and the runtime serde error is the honest signal. The demanded
// shape (`{* uint => any}` metadata tables) is unaffected.
impl serde::Serialize for AnyCbor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(1))?;
        match self.kind() {
            AnyCborKind::UInt => {
                map.serialize_entry("uint", &self.as_uint().unwrap())?;
            }
            AnyCborKind::NInt => {
                let v = self.as_nint().unwrap();
                match i64::try_from(v) {
                    Ok(n) => map.serialize_entry("nint", &n)?,
                    Err(_) => map.serialize_entry("nint", &v.to_string())?,
                }
            }
            AnyCborKind::Bytes => {
                map.serialize_entry("bytes", &any_cbor_hex_encode(self.as_bytes().unwrap()))?;
            }
            AnyCborKind::Text => {
                map.serialize_entry("text", self.as_text().unwrap())?;
            }
            AnyCborKind::Array => {
                // &[AnyCbor] → JSON array, each element recursing through this impl.
                map.serialize_entry("array", self.as_array().unwrap())?;
            }
            AnyCborKind::Map => {
                // &[(AnyCbor, AnyCbor)] → JSON array of 2-element arrays (each tuple is a seq).
                map.serialize_entry("map", self.as_map().unwrap())?;
            }
            AnyCborKind::Tag => {
                let (tag, inner) = self.as_tag().unwrap();
                // (u64, &AnyCbor) → JSON array [tag, value].
                map.serialize_entry("tag", &(tag, inner))?;
            }
            AnyCborKind::Bool => {
                map.serialize_entry("bool", &self.as_bool().unwrap())?;
            }
            AnyCborKind::Null => {
                map.serialize_entry("null", &())?;
            }
            AnyCborKind::Undefined => {
                map.serialize_entry("undefined", &())?;
            }
            AnyCborKind::Unassigned => {
                map.serialize_entry("unassigned", &self.as_unassigned().unwrap())?;
            }
            AnyCborKind::Float => {
                let f = self.as_float().unwrap();
                if f.is_finite() {
                    map.serialize_entry("float", &f)?;
                } else if f.is_nan() {
                    map.serialize_entry("float", "NaN")?;
                } else if f > 0.0 {
                    map.serialize_entry("float", "Infinity")?;
                } else {
                    map.serialize_entry("float", "-Infinity")?;
                }
            }
        }
        map.end()
    }
}

/// A JSON nint payload: a number (must fit `i64`) or a decimal string (any magnitude in the domain).
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum AnyCborJsonNint {
    Num(i64),
    Str(String),
}

/// A JSON float payload: a finite number, or one of the three non-finite string sentinels.
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum AnyCborJsonFloat {
    Num(f64),
    Str(String),
}

impl<'de> serde::Deserialize<'de> for AnyCbor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(AnyCborJsonVisitor)
    }
}

struct AnyCborJsonVisitor;

impl<'de> serde::de::Visitor<'de> for AnyCborJsonVisitor {
    type Value = AnyCbor;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a single-key CBOR-tagged JSON object (e.g. {\"uint\": 5})")
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, mut access: A) -> Result<AnyCbor, A::Error> {
        use serde::de::Error;
        let key: String = access
            .next_key()?
            .ok_or_else(|| A::Error::custom("expected a single-key AnyCbor object, got {}"))?;
        let value = match key.as_str() {
            "uint" => AnyCbor::new_uint(access.next_value()?),
            "nint" => {
                let raw: i128 = match access.next_value::<AnyCborJsonNint>()? {
                    AnyCborJsonNint::Num(n) => n as i128,
                    AnyCborJsonNint::Str(s) => s
                        .parse::<i128>()
                        .map_err(|e| A::Error::custom(format!("nint decimal string: {e}")))?,
                };
                if !(-(1i128 << 64)..=-1).contains(&raw) {
                    return Err(A::Error::custom(format!(
                        "nint {raw} outside the CBOR nint domain -2^64..=-1"
                    )));
                }
                AnyCbor::new_nint(raw)
            }
            "bytes" => {
                let hex: String = access.next_value()?;
                AnyCbor::new_bytes(any_cbor_hex_decode(&hex).map_err(A::Error::custom)?)
            }
            "text" => AnyCbor::new_text(access.next_value()?),
            "array" => AnyCbor::new_array(access.next_value()?),
            "map" => AnyCbor::new_map(access.next_value()?),
            "tag" => {
                let (tag, inner): (u64, AnyCbor) = access.next_value()?;
                AnyCbor::new_tag(tag, inner)
            }
            "bool" => AnyCbor::new_bool(access.next_value()?),
            "null" => {
                access.next_value::<()>()?;
                AnyCbor::new_null()
            }
            "undefined" => {
                access.next_value::<()>()?;
                AnyCbor::new_undefined()
            }
            "unassigned" => AnyCbor::new_unassigned(access.next_value()?),
            "float" => {
                let f = match access.next_value::<AnyCborJsonFloat>()? {
                    AnyCborJsonFloat::Num(n) => n,
                    AnyCborJsonFloat::Str(s) => match s.as_str() {
                        "NaN" => f64::NAN,
                        "Infinity" => f64::INFINITY,
                        "-Infinity" => f64::NEG_INFINITY,
                        other => {
                            return Err(A::Error::custom(format!(
                                "unrecognized float string {other:?} (want a number or \
                                 \"NaN\"/\"Infinity\"/\"-Infinity\")"
                            )));
                        }
                    },
                };
                AnyCbor::new_float(f)
            }
            other => {
                return Err(A::Error::custom(format!(
                    "unknown AnyCbor kind key {other:?}"
                )));
            }
        };
        if access.next_key::<String>()?.is_some() {
            return Err(A::Error::custom(
                "an AnyCbor object must have exactly one key",
            ));
        }
        Ok(value)
    }
}

// =================================================================================================
// Natural-fallible JSON — the PRIMARY surface every *generated* type uses for an `any`-typed value.
// This is a SEPARATE surface from the tagged codec above: the tagged impls stay `AnyCbor`'s own
// `Serialize`/`Deserialize` (the total value-level escape hatch and the `AnyCbor` wasm-wrapper
// codec); generated members / enum arms / newtype wrappers instead route
// through `natural_any_cbor` (the `#[serde(with = …)]` adapter below), which renders the CBOR value
// as the JSON value it *naturally is* — `{ "count": 3 }` rather than `{ "map": [[{"text":"count"},
// {"uint":3}]] }`. Only natural rendering composes with static typing on the read side and only
// natural rendering is what a human-authored `from_json` document looks like.
//
// `to_natural_json` implements RFC 8949 §6.1's INJECTIVE subset and STRICT-FAILS everywhere §6.1
// would substitute a value (bytes, tags, `undefined`, unassigned simples, non-finite floats,
// out-of-i64 nints, complex/colliding map keys). No substitutes, ever: our output feeds a symmetric
// `from_natural_json`, so a silent substitution is write-back corruption. Consequence, by design:
// `to_json` on a generated type that *contains* an `any` is FALLIBLE on data — loudly (the error
// names the offending node kind), with `AnyCbor`'s tagged codec as the value-level escape hatch.
//
// `from_natural_json` is TOTAL (every JSON value has a CBOR home) and implements RFC 8949 §6.2:
// lexically-integral numbers become uint/nint (else float), object keys follow the `any`-domain
// prefer-numeric rule (a text key `"12"` JSON-round-trips to uint `12` — the documented JSON-only
// ambiguity; CBOR is authoritative).

/// Error from [`to_natural_json`]: a CBOR node whose kind has no injective JSON image (RFC 8949
/// §6.1 strict-fail — no substitute values). The message names the offending kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnyToNaturalJsonError(pub String);

impl std::fmt::Display for AnyToNaturalJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "AnyCbor value has no natural JSON representation: {}",
            self.0
        )
    }
}

impl std::error::Error for AnyToNaturalJsonError {}

/// The natural-JSON string form of an `any` MAP KEY (used both for a key's object-property name and
/// for collision detection): text verbatim, uint/nint in decimal. Any other kind (bytes, array,
/// map, tag, float, bool, null, undefined, unassigned) has no key image → strict-fail.
pub fn any_cbor_natural_key_string(key: &AnyCbor) -> Result<String, AnyToNaturalJsonError> {
    match key.kind() {
        AnyCborKind::Text => Ok(key.as_text().unwrap().to_owned()),
        AnyCborKind::UInt => Ok(key.as_uint().unwrap().to_string()),
        AnyCborKind::NInt => Ok(key.as_nint().unwrap().to_string()),
        other => Err(AnyToNaturalJsonError(format!(
            "map key of kind {other:?} is not text/uint/nint"
        ))),
    }
}

/// Render an [`AnyCbor`] as the [`serde_json::Value`] it naturally is, or error naming the node kind
/// with no injective JSON image (RFC 8949 §6.1's strict-fail). Big uints (> 2^53) stay JSON numbers
/// (the documented I-JSON / RFC 7493 precision caveat, not a failure).
pub fn to_natural_json(value: &AnyCbor) -> Result<serde_json::Value, AnyToNaturalJsonError> {
    use serde_json::Value;
    match value.kind() {
        AnyCborKind::UInt => Ok(Value::from(value.as_uint().unwrap())),
        AnyCborKind::NInt => {
            // serde_json's number model bottoms out at i64; a nint below i64::MIN has no image.
            let n = value.as_nint().unwrap();
            match i64::try_from(n) {
                Ok(i) => Ok(Value::from(i)),
                Err(_) => Err(AnyToNaturalJsonError(format!(
                    "nint {n} is below i64::MIN (no JSON number image)"
                ))),
            }
        }
        AnyCborKind::Text => Ok(Value::from(value.as_text().unwrap().to_owned())),
        AnyCborKind::Bool => Ok(Value::from(value.as_bool().unwrap())),
        AnyCborKind::Null => Ok(Value::Null),
        AnyCborKind::Float => {
            let f = value.as_float().unwrap();
            // serde_json cannot hold non-finite numbers; NaN/±Inf strict-fail (JSON is lossy for
            // NaN payloads anyway; here the whole non-finite value has no number image).
            serde_json::Number::from_f64(f)
                .map(Value::Number)
                .ok_or_else(|| {
                    AnyToNaturalJsonError(format!("non-finite float {f} has no JSON number image"))
                })
        }
        AnyCborKind::Array => {
            let arr = value.as_array().unwrap();
            let mut out = Vec::with_capacity(arr.len());
            for elem in arr {
                out.push(to_natural_json(elem)?);
            }
            Ok(Value::Array(out))
        }
        AnyCborKind::Map => {
            let pairs = value.as_map().unwrap();
            let mut obj = serde_json::Map::new();
            // Determinism + collision detection: a `BTreeSet` of stringified keys. Two keys that
            // stringify identically (uint `12` + text `"12"`, or two equal keys) are a collision →
            // strict-fail (RFC 8949 §6.1's "danger of key collision"): our JSON feeds a symmetric read.
            let mut seen = std::collections::BTreeSet::new();
            for (key, val) in pairs {
                let key_string = any_cbor_natural_key_string(key)?;
                if !seen.insert(key_string.clone()) {
                    return Err(AnyToNaturalJsonError(format!(
                        "map key {key_string:?} stringifies identically to an earlier key"
                    )));
                }
                obj.insert(key_string, to_natural_json(val)?);
            }
            Ok(Value::Object(obj))
        }
        AnyCborKind::Bytes => Err(AnyToNaturalJsonError("bytes".into())),
        AnyCborKind::Tag => Err(AnyToNaturalJsonError("tag".into())),
        AnyCborKind::Undefined => Err(AnyToNaturalJsonError("undefined".into())),
        AnyCborKind::Unassigned => Err(AnyToNaturalJsonError("unassigned simple value".into())),
    }
}

/// The `any`-domain reading of a JSON object key (RFC 8949 §6.2 read convention): prefer the numeric reading for a
/// CANONICAL decimal spelling (round-trips through `to_string`), else text. So `"12"` → uint `12`,
/// `"-5"` → nint `-5`, but `"012"`/`"+5"`/`"5.0"`/`"abc"` → text. Total.
pub fn any_cbor_natural_key_from_string(key: &str) -> AnyCbor {
    // `.ok().filter(round-trips)` keeps this a single `if let` (no nested-if / no let-chain, so it
    // stays clippy-clean AND edition-agnostic in the generated crate). A canonical decimal spelling
    // round-trips through `to_string`; `"012"`/`"+3"`/`"-0"`/non-round-tripping forms fall to text.
    if let Some(u) = key.parse::<u64>().ok().filter(|u| u.to_string() == key) {
        return AnyCbor::new_uint(u);
    }
    // Keys are STRINGS, so a nint key has no `i64` ceiling (unlike a nint VALUE, bounded by
    // serde_json's number model): parse the whole CBOR nint domain (-2^64..=-1) as `i128`, matching
    // what `any_cbor_natural_key_string` writes. The domain check subsumes the sign check.
    if let Some(i) = key
        .parse::<i128>()
        .ok()
        .filter(|i| (-(1i128 << 64)..=-1).contains(i) && i.to_string() == key)
    {
        return AnyCbor::new_nint(i);
    }
    AnyCbor::new_text(key.to_owned())
}

/// The reverse of [`to_natural_json`]: every JSON value has a CBOR home (RFC 8949 §6.2).
/// TOTAL — never fails. Lexically-integral numbers become uint (non-negative) / nint (negative),
/// anything else (fractional / out-of-i64-magnitude) becomes a float; object keys use the
/// `any`-domain prefer-numeric rule.
pub fn from_natural_json(value: serde_json::Value) -> AnyCbor {
    use serde_json::Value;
    match value {
        Value::Null => AnyCbor::new_null(),
        Value::Bool(b) => AnyCbor::new_bool(b),
        Value::Number(n) => {
            if let Some(u) = n.as_u64() {
                AnyCbor::new_uint(u)
            } else if let Some(i) = n.as_i64() {
                // as_i64 with a non-u64 number is negative → in the nint domain.
                AnyCbor::new_nint(i as i128)
            } else {
                // Not lexically integral (has a fraction/exponent, or out of i64 magnitude).
                AnyCbor::new_float(n.as_f64().expect("serde_json number is u64/i64/f64"))
            }
        }
        Value::String(s) => AnyCbor::new_text(s),
        Value::Array(arr) => AnyCbor::new_array(arr.into_iter().map(from_natural_json).collect()),
        Value::Object(obj) => AnyCbor::new_map(
            obj.into_iter()
                .map(|(k, v)| (any_cbor_natural_key_from_string(&k), from_natural_json(v)))
                .collect(),
        ),
    }
}

/// `#[serde(with = "…::natural_any_cbor")]` adapter: the serde face a generated type puts on an
/// `any`-typed field/arm so it renders NATURALLY (not through `AnyCbor`'s tagged codec). Serialize
/// is fallible-on-data (RFC 8949 §6.1's failure set surfaces as a serde error naming the node kind);
/// deserialize is total (RFC 8949 §6.2).
pub mod natural_any_cbor {
    use super::{AnyCbor, from_natural_json, to_natural_json};

    pub fn serialize<S>(value: &AnyCbor, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let json = to_natural_json(value).map_err(serde::ser::Error::custom)?;
        serde::Serialize::serialize(&json, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<AnyCbor, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let json = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        Ok(from_natural_json(json))
    }
}

/// The `Option<AnyCbor>` companion to [`natural_any_cbor`], for an OPTIONAL `any` member (`? N: any`
/// → `Option<AnyCbor>`). A generated optional field pairs this with `#[serde(default)]` so a missing
/// key reads back as `None`, matching the derive's ordinary optional handling.
pub mod natural_any_cbor_opt {
    use super::{AnyCbor, from_natural_json, to_natural_json};

    pub fn serialize<S>(value: &Option<AnyCbor>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match value {
            Some(v) => {
                let json = to_natural_json(v).map_err(serde::ser::Error::custom)?;
                serializer.serialize_some(&json)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<AnyCbor>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let opt = <Option<serde_json::Value> as serde::Deserialize>::deserialize(deserializer)?;
        Ok(opt.map(from_natural_json))
    }
}

// A serde-only view of an `AnyCbor` that renders NATURALLY. Used purely to let serde's own
// container handling (`Vec`/`BTreeMap`/`OrderedHashMap` serde) walk the collection element-wise —
// this is serde composition, NOT a parallel JSON path (serde drives the container; the wrapper only
// swaps the per-element codec from tagged to natural).
pub struct NaturalAnyCborSer<'a>(pub &'a AnyCbor);

impl serde::Serialize for NaturalAnyCborSer<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        natural_any_cbor::serialize(self.0, serializer)
    }
}

pub struct NaturalAnyCborDe(pub AnyCbor);

impl<'de> serde::Deserialize<'de> for NaturalAnyCborDe {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        natural_any_cbor::deserialize(deserializer).map(NaturalAnyCborDe)
    }
}

/// `#[serde(with = …)]` adapter for a `Vec<AnyCbor>` member (homogeneous `[* any]` array as a
/// struct field), rendering each element naturally. Rides serde's own seq handling.
pub mod natural_any_cbor_seq {
    use super::{AnyCbor, NaturalAnyCborDe, NaturalAnyCborSer};

    pub fn serialize<S>(value: &[AnyCbor], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(value.iter().map(NaturalAnyCborSer))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<AnyCbor>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let elems = <Vec<NaturalAnyCborDe> as serde::Deserialize>::deserialize(deserializer)?;
        Ok(elems.into_iter().map(|e| e.0).collect())
    }
}

/// `#[serde(with = …)]` adapter for a `BTreeMap<K, AnyCbor>` member (a `{* K => any}` table with a
/// stringifiable key as a struct field, non-preserve). The KEY renders through its own serde (the
/// crate-wide table-key posture — a non-stringifiable `any` key still errors at runtime per RFC 8949 §6.1);
/// only the VALUE flips to natural. Generic over `K` so one module serves every key type.
pub mod natural_any_cbor_btreemap {
    use super::{AnyCbor, NaturalAnyCborDe, NaturalAnyCborSer};
    use std::collections::BTreeMap;

    pub fn serialize<K, S>(value: &BTreeMap<K, AnyCbor>, serializer: S) -> Result<S::Ok, S::Error>
    where
        K: serde::Serialize,
        S: serde::Serializer,
    {
        serializer.collect_map(value.iter().map(|(k, v)| (k, NaturalAnyCborSer(v))))
    }

    pub fn deserialize<'de, K, D>(deserializer: D) -> Result<BTreeMap<K, AnyCbor>, D::Error>
    where
        K: serde::Deserialize<'de> + Ord,
        D: serde::Deserializer<'de>,
    {
        let map = <BTreeMap<K, NaturalAnyCborDe> as serde::Deserialize>::deserialize(deserializer)?;
        Ok(map.into_iter().map(|(k, v)| (k, v.0)).collect())
    }
}

/// `#[serde(with = …)]` adapter for an OPTIONAL homogeneous-array member (`? N: [* any]` →
/// `Option<Vec<AnyCbor>>`), paired with `#[serde(default)]`. `None` → JSON null / missing.
pub mod natural_any_cbor_opt_seq {
    use super::{AnyCbor, NaturalAnyCborDe, NaturalAnyCborSer};

    pub fn serialize<S>(value: &Option<Vec<AnyCbor>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match value {
            Some(v) => {
                let wrapped: Vec<NaturalAnyCborSer> = v.iter().map(NaturalAnyCborSer).collect();
                serializer.serialize_some(&wrapped)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<AnyCbor>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let opt = <Option<Vec<NaturalAnyCborDe>> as serde::Deserialize>::deserialize(deserializer)?;
        Ok(opt.map(|v| v.into_iter().map(|e| e.0).collect()))
    }
}

/// `#[serde(with = …)]` adapter for an OPTIONAL non-preserve table member (`? N: {* K => any}` →
/// `Option<BTreeMap<K, AnyCbor>>`), paired with `#[serde(default)]`.
pub mod natural_any_cbor_opt_btreemap {
    use super::{AnyCbor, NaturalAnyCborDe, NaturalAnyCborSer};
    use std::collections::BTreeMap;

    pub fn serialize<K, S>(
        value: &Option<BTreeMap<K, AnyCbor>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        K: serde::Serialize + Ord,
        S: serde::Serializer,
    {
        match value {
            Some(m) => {
                let wrapped: BTreeMap<&K, NaturalAnyCborSer> =
                    m.iter().map(|(k, v)| (k, NaturalAnyCborSer(v))).collect();
                serializer.serialize_some(&wrapped)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, K, D>(deserializer: D) -> Result<Option<BTreeMap<K, AnyCbor>>, D::Error>
    where
        K: serde::Deserialize<'de> + Ord,
        D: serde::Deserializer<'de>,
    {
        let opt = <Option<BTreeMap<K, NaturalAnyCborDe>> as serde::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(opt.map(|m| m.into_iter().map(|(k, v)| (k, v.0)).collect()))
    }
}

/// Lowercase hex of a byte slice (self-contained — the `hex` crate is not always a dep here).
fn any_cbor_hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(char::from_digit((b >> 4) as u32, 16).unwrap());
        s.push(char::from_digit((b & 0x0f) as u32, 16).unwrap());
    }
    s
}

/// Parse lowercase/uppercase hex into bytes; errors on odd length or a non-hex nibble.
fn any_cbor_hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() & 1 == 1 {
        return Err(format!("odd-length hex string (len {})", s.len()));
    }
    let bytes = s.as_bytes();
    (0..bytes.len())
        .step_by(2)
        .map(|i| {
            let hi = (bytes[i] as char)
                .to_digit(16)
                .ok_or_else(|| format!("invalid hex nibble {:?}", bytes[i] as char))?;
            let lo = (bytes[i + 1] as char)
                .to_digit(16)
                .ok_or_else(|| format!("invalid hex nibble {:?}", bytes[i + 1] as char))?;
            Ok(((hi << 4) | lo) as u8)
        })
        .collect()
}
// Preserve-only natural-JSON companions for `any`-valued map MEMBERS. Assembled ONLY under
// `--preserve-encodings` + `--json-serde-derives`, appended right after `any_cbor_json.rs`, so both
// `OrderedHashMap` (always present under `--preserve-encodings`) and the `NaturalAnyCbor{Ser,De}`
// serde-only wrappers (from `any_cbor_json.rs`) are in scope. These are the preserve counterparts of
// `natural_any_cbor_btreemap` / `natural_any_cbor_opt_btreemap`: a `{* K => any}` table member under
// preserve is an `OrderedHashMap<K, AnyCbor>`. Key ordering mirrors `ordered_hash_map_json.rs`'s own
// sort-into-`BTreeMap` serde so a preserve map member and its non-preserve equivalent render the same
// JSON. Rides serde's own container handling — not a parallel JSON path.
use super::ordered_hash_map::OrderedHashMap;

/// `#[serde(with = …)]` adapter for a preserve `{* K => any}` member (`OrderedHashMap<K, AnyCbor>`).
pub mod natural_any_cbor_orderedmap {
    use super::OrderedHashMap;
    use super::{AnyCbor, NaturalAnyCborDe, NaturalAnyCborSer};
    use std::collections::BTreeMap;

    pub fn serialize<K, S>(
        value: &OrderedHashMap<K, AnyCbor>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        K: serde::Serialize + std::hash::Hash + Eq + Ord,
        S: serde::Serializer,
    {
        let sorted: BTreeMap<&K, NaturalAnyCborSer> = value
            .iter()
            .map(|(k, v)| (k, NaturalAnyCborSer(v)))
            .collect();
        serde::Serialize::serialize(&sorted, serializer)
    }

    pub fn deserialize<'de, K, D>(deserializer: D) -> Result<OrderedHashMap<K, AnyCbor>, D::Error>
    where
        K: serde::Deserialize<'de> + std::hash::Hash + Eq + Ord,
        D: serde::Deserializer<'de>,
    {
        let map = <BTreeMap<K, NaturalAnyCborDe> as serde::Deserialize>::deserialize(deserializer)?;
        Ok(map.into_iter().map(|(k, v)| (k, v.0)).collect())
    }
}

/// `#[serde(with = …)]` adapter for an OPTIONAL preserve table member
/// (`? N: {* K => any}` → `Option<OrderedHashMap<K, AnyCbor>>`), paired with `#[serde(default)]`.
pub mod natural_any_cbor_opt_orderedmap {
    use super::OrderedHashMap;
    use super::{AnyCbor, NaturalAnyCborDe, NaturalAnyCborSer};
    use std::collections::BTreeMap;

    pub fn serialize<K, S>(
        value: &Option<OrderedHashMap<K, AnyCbor>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        K: serde::Serialize + std::hash::Hash + Eq + Ord,
        S: serde::Serializer,
    {
        match value {
            Some(m) => {
                let sorted: BTreeMap<&K, NaturalAnyCborSer> =
                    m.iter().map(|(k, v)| (k, NaturalAnyCborSer(v))).collect();
                serializer.serialize_some(&sorted)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, K, D>(
        deserializer: D,
    ) -> Result<Option<OrderedHashMap<K, AnyCbor>>, D::Error>
    where
        K: serde::Deserialize<'de> + std::hash::Hash + Eq + Ord,
        D: serde::Deserializer<'de>,
    {
        let opt = <Option<BTreeMap<K, NaturalAnyCborDe>> as serde::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(opt.map(|m| m.into_iter().map(|(k, v)| (k, v.0)).collect()))
    }
}
// Manual `schemars::JsonSchema` for `AnyCbor`, matching the `any_cbor_json.rs` rendering: a
// self-referential `oneOf` over the twelve single-key tagged-object forms. Precedent for a manual
// impl on a runtime type: `ordered_hash_map_schemars.rs`. The schema flows into the json-gen crate
// and through `run-json2ts.js`; the json2ts acceptance of this self-referential schema is a named
// pre-ship gate.
//
// Routed through `schemars::json_schema!` (which uses schemars' own bundled serde_json), so this
// compiles under `--json-schema-export` even when `--json-serde-derives` is off (serde_json is only
// a direct dep under the serde flag).
impl schemars::JsonSchema for AnyCbor {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        "AnyCbor".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        // Self-reference for the recursive positions (array items, map pair elements, tag value).
        // `subschema_for::<AnyCbor>` registers AnyCbor in the generator's definitions and returns a
        // `$ref` schema. AnyCbor is mid-generation at this point, so the generator returns a ref
        // rather than inlining — `inline_schema` returning false guarantees the recursion terminates.
        let any = generator.subschema_for::<AnyCbor>().to_value();
        schemars::json_schema!({
            "oneOf": [
                { "type": "object", "additionalProperties": false, "required": ["uint"],
                  "properties": { "uint": { "type": "integer", "minimum": 0 } } },
                { "type": "object", "additionalProperties": false, "required": ["nint"],
                  "properties": { "nint": { "type": ["integer", "string"] } } },
                { "type": "object", "additionalProperties": false, "required": ["bytes"],
                  "properties": { "bytes": { "type": "string" } } },
                { "type": "object", "additionalProperties": false, "required": ["text"],
                  "properties": { "text": { "type": "string" } } },
                { "type": "object", "additionalProperties": false, "required": ["array"],
                  "properties": { "array": { "type": "array", "items": (any.clone()) } } },
                { "type": "object", "additionalProperties": false, "required": ["map"],
                  "properties": { "map": { "type": "array",
                      "items": { "type": "array", "minItems": 2, "maxItems": 2,
                          "prefixItems": [(any.clone()), (any.clone())] } } } },
                { "type": "object", "additionalProperties": false, "required": ["tag"],
                  "properties": { "tag": { "type": "array", "minItems": 2, "maxItems": 2,
                      "prefixItems": [ { "type": "integer", "minimum": 0 }, (any.clone()) ] } } },
                { "type": "object", "additionalProperties": false, "required": ["bool"],
                  "properties": { "bool": { "type": "boolean" } } },
                { "type": "object", "additionalProperties": false, "required": ["null"],
                  "properties": { "null": { "type": "null" } } },
                { "type": "object", "additionalProperties": false, "required": ["undefined"],
                  "properties": { "undefined": { "type": "null" } } },
                { "type": "object", "additionalProperties": false, "required": ["unassigned"],
                  "properties": { "unassigned": { "type": "integer", "minimum": 0, "maximum": 255 } } },
                { "type": "object", "additionalProperties": false, "required": ["float"],
                  "properties": { "float": { "type": ["number", "string"] } } }
            ]
        })
    }

    fn inline_schema() -> bool {
        false
    }
}

/// The `#[schemars(schema_with = "…::natural_any_cbor_schema")]` companion to the
/// `natural_any_cbor` serde adapter: the schema a GENERATED type puts on an `any`-typed field/arm
/// so it describes the NATURAL rendering. It is the permissive "any JSON value" schema (an empty
/// schema accepts everything) because the natural rendering of an arbitrary CBOR value can be any
/// JSON value — json2ts turns it into TS `unknown`. This is deliberately DISTINCT from `AnyCbor`'s
/// own tagged `oneOf` schema above, which describes the tagged value-codec surface (the `AnyCbor`
/// wasm wrapper's `to_json`), and is left as `AnyCbor`'s own schema.
pub fn natural_any_cbor_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({})
}

/// Companion to `natural_any_cbor_seq` (a `Vec<AnyCbor>` member, `[* any]`): an array whose items
/// are the permissive "any JSON value" schema.
pub fn natural_any_cbor_seq_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({ "type": "array", "items": {} })
}

/// Companion to `natural_any_cbor_btreemap` (a `{* K => any}` table member with a stringifiable
/// key): an object whose additional-property values are the permissive "any JSON value" schema.
pub fn natural_any_cbor_map_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({ "type": "object", "additionalProperties": {} })
}
