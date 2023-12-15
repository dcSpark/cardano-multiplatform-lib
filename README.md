# Cardano Multiplatform Lib

This is a library, written in Rust, that can be deployed to multiple platforms (Rust crate, JS, Typescript, WASM, etc). It handles:
- Serialization & deserialization of core data structures
- Useful utility functions for dApps & wallets

##### NPM packages

- browser: [link](https://www.npmjs.com/package/@dcspark/cardano-multiplatform-lib-browser)
- nodejs: [link](https://www.npmjs.com/package/@dcspark/cardano-multiplatform-lib-nodejs)
- asm.js (strongly discouraged): [link](https://www.npmjs.com/package/@dcspark/cardano-multiplatform-lib-asmjs)

##### Rust crates

- crates: [link](https://crates.io/crates/cardano-multiplatform-lib)

##### Mobile bindings

We recommend using Ionic + Capacitor or an equivalent setup to have the WASM bindings working in mobile

## Documentation

https://dcSpark.github.io/cardano-multiplatform-lib/

# Crate Architecture

For current users, the `rust/ `crate is the main version of CML and is the only one that should be used. There is a workspace in the root directory with crates like `core`, `wasm` etc, which are a part of a big refactor and will eventually replace the rust crate at some point in the future, but are still quite WIP for now. The rust crate when used for WASM builds via the npm scripts in the root repo dir will utilize the `rust/json-gen` crate here in the build scripts to generate typescript definitions for the JSON conversion. The `json-gen-split` crate is the equivalent for the new `core`/`wasm` crates and is not called anywhere from the build scripts, but will someday replace the `rust/json-gen` crate once the refactoring is completed.
