// The `--json-schema-export` runtime helpers a generated `wasm/json-gen` crate imports from here:
// the row `Registrar` (which owns the published-name ledger and delegates each row to `add_schema`)
// and the document's reference-closure check `check_schema_ref_closure`. They live in the common
// runtime crate — ONE copy per workspace — and every json-gen crate pointed at that crate `use`s
// them, instead of each carrying its own emitted copy. The crate hosting this module never calls
// any of them itself.
//
// `add_schema` stays public beside the registrar: it is the one implementation of the guard, and a
// consumer whose layout the flags do not cover may have hand-written a row against it.
//
// The rest of the module serves the CONSUMER's own hand-written code rather than any emitted call:
// `custom_schema_impl!` (and the `custom_schema_body` / `retarget_defs_references` pair under it)
// writes the `schemars::JsonSchema` impl that `@custom_json` — and a hand-written extern with no
// derive — commits its author to. Nothing the tool emits calls those; the tool emits the
// registration ROW for such a type and the impl has to exist for that row to compile.
//
// Both checks live in the CONSUMER's own `cargo run` of their json-gen crate rather than in
// cddl-codegen's suite, for the same reason: we are the party that requires the hand-written
// `schemars::JsonSchema` impls they bite, and cddl-codegen's own suite asserting these properties
// over its own fixtures says nothing about a consumer's document.
//
// The panic wordings here are load-bearing. `snapshot_tests` pins the wiring, the
// `json-schema-name-merge` / `json-schema-name-stolen` / `json-schema-ref-dangling` fixtures assert
// on message fragments, and `docs/docs/command_line_flags.mdx` / `docs/docs/comment_dsl.mdx` quote
// them. Reword nothing.

/// `Cow`, re-exported so `custom_schema_impl!` can name it as `$crate::json_schema_gen::Cow`.
///
/// The macro expands in the INVOKING crate, which may be a consumer crate this tool never writes
/// to, so its `schema_name()` return type cannot be spelled `alloc::borrow::Cow`: `alloc` is a
/// sysroot crate that resolves only where an `extern crate alloc;` is in scope, and nothing here
/// can deliver that line into a hand-owned crate. Routing the type through `$crate` instead makes
/// the expansion depend only on this module being reachable — the same contract
/// `$crate::json_schema_gen::custom_schema_body` already relies on, so it adds no new requirement
/// on the invoking crate.
extern crate alloc;
pub use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// The prefix a HAND-AUTHORED schema body writes its internal references with. It is a fixed part
/// of the authoring convention (`{"$ref": "#/$defs/ConstrPlutusData"}`), NOT a claim about the
/// document the body ends up in: `retarget_defs_references` below rewrites exactly these onto
/// whatever namespace the generator actually uses, which is what keeps the hand-written file from
/// encoding a runtime fact.
const HAND_AUTHORED_DEFS_PREFIX: &str = "#/$defs/";

/// The ONE normalisation of a generator's `definitions_path` setting, read by every helper here
/// that has to name the reference namespace: the setting is spelled `/$defs` by schemars' own
/// defaults but a consumer's settings may carry the `#` and/or a trailing `/`, and the three
/// readers (the row guard's stolen-name check, the closure check, and the hand-authored-body
/// retarget) must agree on what "the namespace" is or they contradict each other on the same
/// document.
///
/// Returns the setting as a plain JSON POINTER into the finished document (`/$defs`), which is the
/// form the closure check needs for `serde_json::Value::pointer`.
fn definitions_pointer(definitions_path: &str) -> &str {
    let path = definitions_path
        .strip_prefix('#')
        .unwrap_or(definitions_path);
    path.strip_suffix('/').unwrap_or(path)
}

/// The same setting as the URI-fragment REFERENCE prefix every `$ref` into the definitions map
/// starts with (`#/$defs/`). Derived from `definitions_pointer` rather than normalised again, so
/// the pointer form and the reference form can never disagree.
fn definitions_ref_prefix(definitions_path: &str) -> String {
    format!("#{}/", definitions_pointer(definitions_path))
}

/// Rewrites a HAND-AUTHORED schema body's internal references onto the namespace `generator`
/// actually uses, recursively, everywhere in `body`.
///
/// Only a `$ref` already carrying the authoring convention's `#/$defs/` prefix is touched, and only
/// its prefix. Every other `$ref` is left exactly as written — a bare `"PlutusData"`, an
/// `http(s)://` URL, a pointer into some other document — because those are the shapes
/// `check_schema_ref_closure` exists to report, and silently rewriting one would turn a reported
/// failure into a differently-dangling reference.
///
/// This is why a hand-authored file may write `#/$defs/` and still not be encoding a runtime fact:
/// the prefix is the authoring convention, and the namespace the document ships with is read off
/// the generator here. It matters because `add_schemas` takes the generator as a parameter — a
/// consumer composing several crates' rows supplies their own, and `--json-schema-dep` threads it
/// across crates — so the document a hand-authored body lands in is not always the one
/// `export_schemas` builds with schemars' defaults.
pub fn retarget_defs_references(body: &mut serde_json::Value, definitions_path: &str) {
    retarget_with_prefix(body, definitions_ref_prefix(definitions_path).as_str());
}

fn retarget_with_prefix(value: &mut serde_json::Value, prefix: &str) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map.iter_mut() {
                if key == "$ref"
                    && let Some(name) = child
                        .as_str()
                        .and_then(|r| r.strip_prefix(HAND_AUTHORED_DEFS_PREFIX))
                {
                    *child = serde_json::Value::String(format!("{prefix}{name}"));
                } else {
                    retarget_with_prefix(child, prefix);
                }
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                retarget_with_prefix(item, prefix);
            }
        }
        _ => {}
    }
}

/// The whole `json_schema()` member `custom_schema_impl!` writes, as an ordinary function: parse the
/// hand-authored document, retarget its references onto `generator`'s namespace, and hand back a
/// `schemars::Schema`. `origin` names the file in the two failure messages and is otherwise unused.
///
/// A function rather than more macro body, for two reasons that both outlast the macro. It is the
/// part with behaviour, so it is directly testable and directly lintable, while what stays in the
/// macro is impl scaffolding a compiler checks on sight. And it is what a consumer whose type the
/// macro cannot serve — a manual `impl` for a shape outside the macro's two forms — calls to get the
/// same body without reimplementing the retarget.
pub fn custom_schema_body(
    generator: &schemars::SchemaGenerator,
    origin: &str,
    source: &str,
) -> schemars::Schema {
    let mut body: serde_json::Value = serde_json::from_str(source).unwrap_or_else(|e| {
        panic!("cddl-codegen custom_schema_impl!: {origin} is not valid JSON: {e}")
    });
    retarget_defs_references(&mut body, &generator.settings().definitions_path);
    schemars::Schema::try_from(body).unwrap_or_else(|e| {
        panic!("cddl-codegen custom_schema_impl!: {origin} is not a valid JSON schema: {e}")
    })
}

/// Writes the whole `schemars::JsonSchema` impl for a type whose schema BODY is a hand-authored
/// JSON file — the impl `@custom_json` commits a spec author to, and the one a hand-written
/// `_CDDL_CODEGEN_EXTERN_TYPE_` needs before it can carry a registration row.
///
/// ```ignore
/// // the published name is the type's own name, `PlutusData`
/// crate::custom_schema_impl!(PlutusData, "custom_schemas/PlutusData.json");
/// // …or state it, for a type that is not a bare ident here or that publishes under another name
/// crate::custom_schema_impl!(Ext<u64>, "custom_schemas/ExtU64.json", "ExtU64");
/// ```
///
/// `crate::` is the in-crate spelling — this file is a module of the rust crate you are writing in.
/// Under `--common-import-override` / `--export-static-crate` the macro lives in the common crate,
/// so it is `<common>::custom_schema_impl!(…)` instead, invoked from whichever crate defines the
/// type (fact 3).
///
/// Five facts about where it may be written, each of which is a compile error to get wrong:
///
/// 1. **It is exported at the CRATE ROOT of the crate hosting this module** (`#[macro_export]`
///    hoists it), so it is `<common>::custom_schema_impl!`, never
///    `<common>::json_schema_gen::custom_schema_impl!`. If that module is behind a `#[cfg]` of your
///    own, the macro is gated with it — it is defined in this file.
/// 2. **The expansion reaches back here as `$crate::json_schema_gen::…`**, and `$crate` is the crate
///    that DEFINES this macro — the one hosting this file, never the invoking one — so that crate
///    must have a module named `json_schema_gen` reachable from ITS root. Under
///    `--export-static-crate` you hand-declare `pub mod json_schema_gen;` in the target crate's root
///    (the tool's new-static-file notice names it). In-crate the tool declares the module inside
///    `src/generated/mod.rs`, so root reachability comes from the seed-once `src/lib.rs`'s
///    `pub use generated::*;` — a line you own after the first export. Narrowing that glob to a name
///    list makes every invocation an `E0433` for `json_schema_gen` in `$crate`, reported at the
///    expansion rather than at the edit that caused it.
/// 3. **The invocation must live in the crate that DEFINES the type.** `schemars::JsonSchema` is a
///    foreign trait, so the orphan rule allows the impl nowhere else. For a generated type carrying
///    `@custom_json` that means a hand-owned module of the GENERATED rust crate, declared from its
///    seed-once `src/lib.rs` — outside `src/generated/**`, which is clobbered every run.
/// 4. **`include_str!` resolves `$path` relative to the INVOKING file**, not to this one. Keep the
///    JSON inside the invoking crate's own directory: a published crate ships only its own files,
///    so a path reaching into a sibling crate compiles locally and breaks at `cargo publish`.
/// 5. **The invoking crate needs `schemars` reachable under that name** — and only that one. The
///    expansion writes `schemars::JsonSchema` / `SchemaGenerator` / `Schema` unqualified, but no
///    `serde_json`: the document is parsed inside `custom_schema_body`, compiled once in the crate
///    hosting this file, and the invocation hands it a `&'static str`. So a crate that invokes this
///    for a type of its own needs no JSON dependency of its own. Under `--json-schema-export` the
///    tool already asserts `schemars` on the generated rust crate, which is where fact 3 puts a
///    generated type's invocation.
///
/// `inline_schema()` returns `false`: that is what makes the type REFERABLE — schemars registers the
/// returned body as a definition and hands out a `$ref` to it — so one hand-authored file can point
/// at another's entry and each shape is declared once. It is also schemars' own default; stated
/// explicitly because it is the load-bearing half of the impl, not because it changes anything.
///
/// `schema_name()` is derived from the TYPE TOKEN in the two-argument form. That makes a
/// hand-authored type's published name follow the same rule as every generated sibling in the same
/// document (the schemars derive publishes the Rust type name) by construction, rather than by an
/// author keeping a string in sync with a type. Taking an `ident` there is what makes the derivation
/// exact — an ident stringifies to itself, with no path qualifier or generic-argument spelling
/// leaking into a published API name — and it costs nothing, because fact 3 already puts the
/// invocation in the crate that defines the type, where the type is nameable as a bare ident.
///
/// The three-argument form takes the name as an expression instead, for the two shapes the
/// derivation cannot serve: a type that is not a bare ident at the invocation, and a deliberate
/// published name that differs from the Rust one. Explicit rather than derived, because
/// `schema_name()` doubles as `schema_id()` by default and therefore as an IDENTITY: a generic whose
/// instantiations all report one constant name is exactly the silent MERGE the row ledger in
/// `add_schema` panics on, and the author is the only party who can vary it
/// (`format!("Base_{}", <T as schemars::JsonSchema>::schema_name())` — an expression, which is why
/// this argument is one).
/// The two arms deliberately do NOT delegate to one another: an arm written as
/// `$crate::custom_schema_impl!(…)` is an absolute path to a `macro_export` macro, which rustc
/// rejects outright in a crate that obtained this file by `include!` rather than as a real module
/// ("macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute
/// paths") — cddl-codegen's own harness is exactly such a crate. Since `custom_schema_body` already
/// holds everything with behaviour, the price of no delegation is one duplicated three-member
/// skeleton whose only difference is the `schema_name()` line.
#[macro_export]
macro_rules! custom_schema_impl {
    ($ty:ident, $path:literal) => {
        impl schemars::JsonSchema for $ty {
            fn schema_name() -> $crate::json_schema_gen::Cow<'static, str> {
                $crate::json_schema_gen::Cow::Borrowed(::core::stringify!($ty))
            }

            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                $crate::json_schema_gen::custom_schema_body(generator, $path, include_str!($path))
            }

            fn inline_schema() -> bool {
                false
            }
        }
    };
    ($ty:ty, $path:literal, $name:expr) => {
        impl schemars::JsonSchema for $ty {
            fn schema_name() -> $crate::json_schema_gen::Cow<'static, str> {
                ::core::convert::Into::into($name)
            }

            fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                $crate::json_schema_gen::custom_schema_body(generator, $path, include_str!($path))
            }

            fn inline_schema() -> bool {
                false
            }
        }
    };
}

/// The row helper every registration row reaches, through `Registrar::add` below.
/// `subschema_for::<T>()` registers T — and everything T references — into the generator's shared
/// `$defs` and returns a `$ref`, EXCEPT for a type whose `JsonSchema::inline_schema()` is true, where
/// it returns the schema itself and registers nothing (every `@newtype` wrapper over a primitive
/// emits exactly such an impl, since its JSON form IS the primitive's). Publishing the returned
/// schema under the type's own schema name in that case is what keeps EVERY exported row a `$defs`
/// entry.
///
/// The helper also carries the document's NAME-INJECTIVITY guard: a `$defs` key is the published API
/// name (`run-json2ts.js` suffixes it with `JSON` and json2ts emits it as the TypeScript type name),
/// so two Rust types claiming one `schemars::JsonSchema::schema_name()` either publish one type's
/// shape under the other's name (identical `schema_id`s — the default makes `schema_name` do double
/// duty as an identity, so schemars sees ONE type and emits ONE definition) or publish an
/// order-dependent `<name>2` (distinct ids — schemars' `{base}{i}` suffix loop). Both are silent, and
/// both make a published name a function of registration order rather than of the spec.
///
/// The ledger is keyed on the CALLING crate's own rows, never on `--json-schema-dep`: a dep registrar
/// call goes through the DEP's own `add_schemas`, which threads the dep's rows through this same
/// helper with a ledger of its own.
pub fn add_schema<T: schemars::JsonSchema>(
    generator: &mut schemars::SchemaGenerator,
    claimed: &mut alloc::collections::BTreeMap<String, &'static str>,
) {
    let name = <T as schemars::JsonSchema>::schema_name().into_owned();
    let rust = core::any::type_name::<T>();
    // A — the name ledger. Two rows claiming one published name is a silent MERGE when their
    // `schema_id`s also match (the id DEFAULTS to the name, so schemars sees one type, emits one
    // definition, and every reference to the loser resolves to the winner's shape) and an
    // order-dependent RENAME when the ids differ (schemars' `{base}{i}` suffix loop). The ledger is
    // the only check that can see the merge, since there both returned refs equal the shared name.
    // Keyed on `type_name` rather than on mere presence: two CDDL rules that alias the SAME rust
    // type (`a = ext<uint>` and `b = ext<uint>` both lower to a `pub type … = Ext<u64>;` alias) are
    // two rows for one type and must not trip the guard.
    if let Some(previous) = claimed
        .insert(name.clone(), rust)
        .filter(|previous| *previous != rust)
    {
        panic!(
            "cddl-codegen --json-schema-export: two distinct Rust types both publish the JSON schema name\n\"{name}\":\n  {previous}\n  {rust}\nA schema document can define only one type per name, so one of these is published under the other's name or under an order-dependent \"{name}2\" — decided by registration order, not by your spec. Give each type a `schemars::JsonSchema::schema_name()` that is unique within this crate; for a generic, vary it with the parameters (e.g. `format!(\"Base_{{}}\", T::schema_name())`). Note `schema_id()` DEFAULTS to `schema_name()`, so a hand-written impl that returns a constant name makes every instantiation the same type as far as `schemars` is concerned."
        );
    }
    let schema = generator.subschema_for::<T>();
    if <T as schemars::JsonSchema>::inline_schema() {
        // C — an inline-schema type registers nothing, so the row publishes the returned schema under
        // its own name. Inserting only when vacant would silently keep a DIFFERENT type's body that
        // already claimed the name (reachable when the claimant is a non-row, non-inline type).
        let value = schema.to_value();
        let existing = generator.definitions().get(name.as_str()).cloned();
        if let Some(existing) = existing {
            if existing != value {
                panic!(
                    "cddl-codegen --json-schema-export: {rust} publishes the inline JSON schema name \"{name}\", but the document already defines \"{name}\" with a different body. Give one of them a `schemars::JsonSchema::schema_name()` that is unique within this crate."
                );
            }
        } else {
            generator.definitions_mut().insert(name.clone(), value);
        }
    } else if let Some(reference) = schema.get("$ref").and_then(|r| r.as_str()) {
        // B — the row kept its OWN name. `subschema_for` returns `<definitions_path>/<assigned>`,
        // where `assigned` is this type's `schema_name()` unless another type claimed it first, in
        // which case schemars hands out `<name>2`. This is what sees a collision whose WINNER has no
        // row of its own (a type reached only through another row's schema), which the ledger cannot.
        // A returned schema with no `$ref` at all is not a naming decision, so it is skipped.
        let prefix = definitions_ref_prefix(&generator.settings().definitions_path);
        // schemars percent-/JSON-pointer-encodes a name whose bytes are not safe inside a URI
        // fragment (`OrderedHashMap<K, V>` becomes `OrderedHashMap%3CK,%20V%3E`) with an encoder that
        // is not part of its public API. Comparing only names no encoder can touch keeps this guard
        // from ever failing a build over a difference it merely cannot decode — the ledger above
        // still covers every collision between two ROWS whatever the spelling.
        let ref_safe_name = name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
        // `None` when the ref carries some other prefix (nothing to compare), when the name is one
        // the encoder may have touched, or when the row kept its own name.
        let stolen = reference
            .strip_prefix(prefix.as_str())
            .filter(|assigned| ref_safe_name && *assigned != name);
        if let Some(assigned) = stolen {
            panic!(
                "cddl-codegen --json-schema-export: {rust} publishes the JSON schema name \"{name}\", but the document assigned it \"{assigned}\" — another type claimed \"{name}\" first. The published name is then decided by registration order, so an unrelated spec edit can silently swap the two. Give one of them a `schemars::JsonSchema::schema_name()` that is unique within this crate."
            );
        }
    }
}

/// The registrar a generated `add_schemas` drives: `Registrar::new(generator)` then one
/// `reg.add::<T>();` per row. It OWNS the published-name ledger, so the row shape carries only the
/// type being registered — the bookkeeping the guard needs is the registrar's business, not
/// something a reader of a generated file (or of a migration diff) has to recognise as such.
///
/// It is a LOCAL of `add_schemas`, never that function's parameter, because
/// `pub fn add_schemas(generator: &mut schemars::SchemaGenerator)` is a published signature: the
/// tool emits `{lib}::add_schemas(generator);` calls into OTHER crates' json-gen crates
/// (`--json-schema-dep`), consumers hand-write one to compose crates the flag does not cover, and
/// the docs quote it. A registrar in the signature would break every one of those.
///
/// Consequence for the emitted body, and the reason the ordering is not incidental: the registrar
/// borrows the generator mutably for its whole life, so every `--json-schema-dep` call — which
/// passes the same `&mut` on to the dependency's own `add_schemas` — must be emitted BEFORE the
/// registrar is constructed.
///
/// The ledger is scoped to ONE `add_schemas`, i.e. to one crate's own rows; a dep's rows are
/// threaded through the dep's own registrar with a ledger of its own. That is the same scope the
/// ledger has always had — see `add_schema` above, which this delegates to so there stays exactly
/// one implementation of the guard.
pub struct Registrar<'a> {
    generator: &'a mut schemars::SchemaGenerator,
    claimed: alloc::collections::BTreeMap<String, &'static str>,
}

impl<'a> Registrar<'a> {
    pub fn new(generator: &'a mut schemars::SchemaGenerator) -> Self {
        Self {
            generator,
            claimed: alloc::collections::BTreeMap::new(),
        }
    }

    /// Register `T` as a published root, subject to the name-injectivity guard.
    pub fn add<T: schemars::JsonSchema>(&mut self) {
        add_schema::<T>(self.generator, &mut self.claimed);
    }
}

fn collect_schema_refs(value: &serde_json::Value, out: &mut alloc::collections::BTreeSet<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                if key == "$ref"
                    && let Some(reference) = child.as_str()
                {
                    out.insert(reference.to_owned());
                }
                collect_schema_refs(child, out);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_schema_refs(item, out);
            }
        }
        _ => {}
    }
}

fn decode_schema_ref_name(encoded: &str) -> String {
    // Inverse of schemars' `encode_ref_name`, which is `pub fn` inside a PRIVATE `mod encoding` — not
    // reachable from here, so the decode is written out. Two layers, undone in the order the encoder
    // applied them: percent-decode the URI-fragment layer first (RFC 3986), then the JSON-Pointer
    // escapes (RFC 6901), `~1` before `~0` — a name holding a literal `~1` encodes to `~01` and
    // decodes wrong in the other order.
    //
    // DEcoding is safe here even though the add_schema helper deliberately refuses to ENcode: that
    // guard skips names it cannot reconstruct because reproducing a private encoder risks a false
    // panic on a schemars bump. Decoding is a well-defined standard operation independent of which
    // characters the encoder chose to escape, so it carries no such risk. Do not "fix" the asymmetry.
    let bytes = encoded.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let mut escaped = false;
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(byte) = u8::from_str_radix(&encoded[i + 1..i + 3], 16)
        {
            decoded.push(byte);
            i += 3;
            escaped = true;
        }
        if !escaped {
            decoded.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&decoded)
        .replace("~1", "/")
        .replace("~0", "~")
}

/// The document's REFERENCE-CLOSURE check, run by the json-gen crate's `export_schemas()` before the
/// document is written. Every `$ref` in the finished document must be an internal pointer at one of
/// that same document's definitions; anything else — a bare relative name
/// (`Schema::new_ref("PlutusData")`, the shape a hand-written `JsonSchema` stub produces), a bare
/// `"#"`, an `http(s)://` URL, another document's path, or an internal pointer at a key nothing
/// defined — ships as a `.d.ts` that references a type it never declares (`TS2304`).
pub fn check_schema_ref_closure(document: &serde_json::Value, definitions_path: &str) {
    // The reference namespace is read off the generator's own settings, never hardcoded, through the
    // same `definitions_pointer` normalisation the row guard and the hand-authored-body retarget use.
    let prefix = definitions_ref_prefix(definitions_path);
    let definitions_path = definitions_pointer(definitions_path);
    // The definitions MAP is resolved through that same setting (as a JSON pointer into the finished
    // document) rather than assumed to be `$defs`, so the two halves of every comparison come from
    // one source. If it resolves to nothing, the reference namespace and the emitted document shape
    // have diverged — a schemars default change — and every reference would be a false positive.
    // Skipping is much cheaper than panicking in a build that is fine.
    let defs = match document
        .pointer(definitions_path)
        .and_then(|d| d.as_object())
    {
        Some(defs) => defs,
        None => return,
    };
    let mut references = alloc::collections::BTreeSet::new();
    collect_schema_refs(document, &mut references);
    // Sorted and deduplicated by the BTreeSet, so the same document always produces the same verdict
    // and the same message.
    let mut dangling = Vec::new();
    for reference in &references {
        match reference.strip_prefix(prefix.as_str()) {
            None => dangling.push(format!(
                "  {reference:?} — not an internal \"{prefix}<key>\" reference"
            )),
            Some(encoded) => {
                let key = decode_schema_ref_name(encoded);
                if !defs.contains_key(key.as_str()) {
                    dangling.push(format!(
                        "  {reference:?} — {key:?} is not defined in this document"
                    ));
                }
            }
        }
    }
    if !dangling.is_empty() {
        let dangling = dangling.join("\n");
        panic!(
            "cddl-codegen --json-schema-export: the exported JSON schema document holds references that do not resolve inside it:\n{dangling}\nThe document is self-contained by contract — `run-json2ts.js` compiles it in one pass with no external resolution — so each of these ships as a `.d.ts` that references a type it never declares (TS2304). A reference like this comes from a hand-written `schemars::JsonSchema` impl that returned a REFERENCE where a schema body was expected: return the real body, or give the referenced type a registration row of its own (a CDDL rule, or `--json-schema-root`) so this document defines it."
        );
    }
}
