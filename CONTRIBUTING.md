# Contributing

## Dependency policy

This library handles keys, addresses, and on-chain serialization, so dependencies are managed
conservatively to avoid silent behavioral drift:

- **Pin full versions, including the patch**, in every `Cargo.toml` — e.g. `bech32 = "0.11.1"`,
  not `"0.11"` or `"2"`. A silent patch-level update could subtly change behavior, which is
  unacceptable here. The full version records and floors the intended patch, and — because a
  library's `Cargo.lock` does **not** constrain crates that depend on it — the version in
  `Cargo.toml` is the *only* lever that reaches downstream consumers. Do **not** "tidy" these back
  to bare `major.minor`.

- **The workspace root `Cargo.lock` is committed** (per-crate locks, from standalone non-workspace
  builds, are not). This freezes the resolved versions for *our* CI and local builds, so a fresh
  checkout doesn't re-resolve to newer patches. Caret semantics mean the full version in
  `Cargo.toml` alone still permits newer patches at resolve time — the committed lock is what
  actually freezes them.
