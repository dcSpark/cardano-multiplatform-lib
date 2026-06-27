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

- **Hand-written (the real logic — read these to understand behavior):**
  `chain/rust/src/builders/` (transaction/cert/mint/etc. construction), the per-module
  `**/utils.rs`, the whole `crypto/` crate, `core/`'s serialization scaffolding, and each crate's
  `lib.rs` `pub mod` list and `Cargo.toml`. `chain/src/address.rs` and the `plutus`/`transaction`
  modules are generated *but heavily hand-edited* — read them as a mix.
- **Generated (read for shape; change via CDDL + regen):** the `**/{mod,serialization,
  cbor_encodings}.rs` files and the `wasm/` wrappers.
- To answer *"how is X encoded on-chain?"* → read `specs/**/*.cddl` plus the generated
  serialization. To answer *"how do I build/validate X?"* → read the hand-written `builders/`
  and `utils.rs`.

## Regeneration

`codegen.sh` is the single source of truth for how generated code is produced — the pinned
`cddl-codegen` version, exact per-crate args, and the in-place + `git diff` review workflow. Read
its header before doing anything generation-related; `specs/README.md` complements it. Byron is a
deliberately special, opt-in case (its rationale is documented in `codegen.sh`).

## Verifying changes

`cargo test --workspace` is the gate. The `multi-era` golden tests round-trip real block CBOR
byte-for-byte, and the `chain` address tests assert known `addr1…` strings — so serialization or
dependency changes that alter behavior get caught. CI also runs `cargo fmt --check`, `cargo clippy`
(with an allow-list for generated-code lints), and an advisory `cargo deny`; mirror these locally.

## Conventions to check before changing them

- **Dependencies** (`CONTRIBUTING.md`): pinned to full `major.minor.patch`, root `Cargo.lock` is
  committed, and some deps are coupled to the codegen and shouldn't be bumped without a regen.
- **Toolchain** is pinned in `rust-toolchain.toml`.

## Domain note

This is Cardano: wire formats, era boundaries, and cryptography are exact and unforgiving. Prefer
the CDDL specs, the tests, and real on-chain vectors over recollection — verify rather than assume.
