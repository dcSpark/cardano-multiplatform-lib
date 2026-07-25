# Repository orientation

cardano-multiplatform-lib (CML) is a Rust library — compiled to native, WASM, and JS/TS — for
serializing, deserializing, and building Cardano data structures. This file is a map of how the
repo is built so you read the right files in the right way. It is orientation, not instructions.

## Read this first: most of the code is generated

The type and CBOR (de)serialization layer is **machine-generated from the CDDL specs in `specs/`**
by dcSpark's `cddl-codegen`. Generated files start with a `// This file was code-generated …`
header. This is the most important fact for reading the repo:

- **Generated files are not where hand-logic lives, and editing them directly is usually a
  mistake** — a regeneration reverts the change. If you need to change a *type* or its wire
  format, change the CDDL in `specs/` and regenerate; don't patch the generated `.rs`.
- When reading a generated file, treat the matching **`specs/**/*.cddl` as the source of truth**
  for the wire format; the generated Rust is a mechanical projection of it.

## Layout

Cargo workspace. Each crate has `rust/` (the library), `wasm/` (wasm-bindgen wrappers, also
generated), and often `wasm/json-gen/`:

- `core` — shared serialization scaffolding and primitives used by every other crate.
- `crypto` — hand-written cryptography (keys, hashes, bech32, BIP32). Correctness-critical.
- `chain` — current-era (Conway) on-chain types: tx, certs, governance, plutus, assets, address.
- `cip25` / `cip36` — metadata standards (NFT metadata, Catalyst voting).
- `multi-era` — types for every era (Byron … Conway) and multi-era block/tx parsing.

The top-level `rust/` directory is the legacy pre-split codebase, **excluded from the workspace**;
ignore it unless explicitly asked.

## Generated vs hand-written — and how to use each

The boundary is the **`src/generated/` directory** (thin-root layout): everything under it is
machine-owned and rewritten by a regen; everything outside it is hand territory. The crate root
`lib.rs` is seeded once by the generator and hand-owned after — it declares the hand modules AND
the **facade modules** that merge a scope's generated and hand items under one public path:

```rust
pub mod assets {                         // shadows the glob-imported generated `assets`
    pub use crate::generated::assets::*; // generated items
    pub mod utils;                       // hand file at src/assets/utils.rs
    pub use utils::*;                    // per-scope re-export policy, hand-decided
}
```

- **Hand-written (the real logic — read these to understand behavior):**
  `chain/rust/src/builders/` (transaction/cert/mint/etc. construction), the per-scope hand files
  at `src/<scope>/{utils,hash,metadata}.rs` (e.g. `chain/rust/src/assets/utils.rs`,
  `crypto/hash.rs`), each crate's `src/utils.rs`, `chain/*/src/address.rs`, the whole `crypto/`
  crate, `core/`, and each crate's `lib.rs` (module list + facades) and `Cargo.toml`.
- **Generated (read for shape; change via CDDL + regen):** everything under `**/src/generated/`
  and the wasm mirrors. Hand content inside a generated file must DECLARE itself, in one of three
  forms the regen re-applies automatically: `cddl-codegen:insert-start/replace-start` blocks for
  code, and `// cddl-codegen:keep` for a comment — either inline (`// cddl-codegen:keep <text>`) or
  as a bare marker on its own line claiming the contiguous comment run below it, which is the only
  form that can carry `///`/`//!` docs. **Never bare-edit a generated file**: an undeclared code
  edit is silently reverted, and an undeclared COMMENT now fails loudly — the regen replaces it
  with a `cddl-codegen:unpreserved-comment` `compile_error!` block carrying the original text, so
  the crate won't build until you either delete it (stale tool output) or re-add it with a marker.
  Prefer the CDDL `@doc` DSL over a `keep`-marked doc where the text suits both the rust and wasm
  faces; `cip25`/`cip36` keep theirs marked because their rust and wasm docs carry
  language-specific examples that one `@doc` cannot serve.
- To answer *"how is X encoded on-chain?"* → read `specs/**/*.cddl` plus the generated
  serialization. To answer *"how do I build/validate X?"* → read the hand-written `builders/`
  and the per-scope `utils.rs` files.

## Regeneration

`codegen.sh` is the single source of truth for how generated code is produced — the pinned
`cddl-codegen` version, exact per-crate args, and the in-place + `git diff` review workflow. Read
its header before doing anything generation-related; `specs/README.md` complements it. Byron is a
deliberately special, opt-in case (its rationale is documented in `codegen.sh`).

**A no-op regen is zero-diff** (verified property, chain + multi-era): facades, hand files, and
the insert/replace blocks all survive a `./codegen.sh` byte-for-byte. If a regen
shows unexpected diff, something may have regressed.
Note the regen may need one extra run to reach the fixed point after NEW blocks are hand-added
(the overlay normalizes their placement on the first pass).

**Workspace mode (cross-crate wrapper placement):** multi-era regenerates **before** chain —
reverse dependency order. multi-era emits a request sidecar
(`multi-era/wasm/src/generated/borrowed_collections.rs`) that chain's regen consumes
(`--wrapper-requests`) to host borrowed collection wrappers in its
`chain/wasm/src/generated/{requested_,}collections.rs` index. These generated files are
**cross-crate contracts**:
- never hand-edit them
- never keep/revert one side's regen diff without the other
note: a stranded half-pass surfaces as unresolved-import errors, but `codegen.sh` ends with a convergence check

## Verifying changes

`cargo test --workspace` is the gate. The `multi-era` golden tests round-trip real block CBOR
byte-for-byte, and the `chain` address tests assert known `addr1…` strings — so serialization or
dependency changes that alter behavior get caught. CI also runs `cargo fmt --check`, the clippy
gate, and an advisory `cargo deny`; mirror these locally. The clippy gate is defined once in
`./clippy.sh` and run by both CI and `codegen.sh` — never restate lint flags at a call site. It is
a two-owner gate: a lint in hand-written code is ours to fix at the site, while a lint under
`src/generated/` is an upstream cddl-codegen regression to report (the generator scopes its own
allows inside the generated module roots, so nothing should escape them). Never widen
`clippy.sh`'s allow list to cover generated code — that both blinds the same lint for hand-written
code and switches off the regression signal the check exists for. Read the script's header before
changing the policy.

## Conventions to check before changing them

- **Dependencies** (`CONTRIBUTING.md`): pinned to full `major.minor.patch`, root `Cargo.lock` is
  committed, and some deps are coupled to the codegen and shouldn't be bumped without a regen.
- **Toolchain** is pinned in `rust-toolchain.toml`.

## Domain note

This is Cardano: wire formats, era boundaries, and cryptography are exact and unforgiving. Prefer
the CDDL specs, the tests, and real on-chain vectors over recollection — verify rather than assume.
