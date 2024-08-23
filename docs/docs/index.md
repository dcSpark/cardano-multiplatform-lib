---
sidebar_label: "Introduction"
sidebar_position: 1
---


# Cardano Multiplatform Lib

This is a library, written in Rust, that can be deployed to multiple platforms (Rust crate, JS, Typescript, WASM, etc). It handles:
- Serialization & deserialization of core data structures
- Builders to streamline and verify the construction of transactions and related components
- Useful utility functions for dApps & wallets

##### NPM packages

- browser: [link](https://www.npmjs.com/package/@dcspark/cardano-multiplatform-lib-browser)
- nodejs: [link](https://www.npmjs.com/package/@dcspark/cardano-multiplatform-lib-nodejs)

There is also an outdated asm.js . It is strongly discouraged from using this as it is out of date and asm.js results in incredibly slow cryptographic operations.
- asm.js (strongly discouraged): [link](https://www.npmjs.com/package/@dcspark/cardano-multiplatform-lib-asmjs)

Note: If you are using WebPack, you must use version 5 or later for CML to work.

##### Rust crates

The rust crates are split up by functionality.

- core: [link](https://crates.io/crates/cml-core)
- crypto: [link](https://crates.io/crates/cml-crypto)
- chain: [link](https://crates.io/crates/cml-chain)
- multi-era: [link](https://crates.io/crates/cml-multi-era)
- cip25: [link](https://crates.io/crates/cml-cip25)
- cip36: [link](https://crates.io/crates/cml-cip36)

Most users will likely be using primarily `cml-chain` for general uses, `cml-multi-era` if they need historical (pre-babbage eras) chain-parsing and `cip25` or `cip36` if they need those specific metadata standards.

##### Mobile bindings

We recommend using Ionic + Capacitor or an equivalent setup to have the WASM bindings working in mobile


## Pre-requisite knowledge

This library assumes a certain amount of knowledge about how Cardano works (to avoid re-documenting the wheel).

You can find the specifications of Cardano's ledger [here](https://github.com/input-output-hk/cardano-ledger-specs) which we suggest consulting as you use this library. Notably, the `Shelley ledger formal specification` covers the core concepts. Make sure to check the specs for later eras as well when needed.


## Benefits of using this library

Serialization/deserialization code is automatically generated from
Cardano’s official specification, which guarantees it can easily stay up
to date! We do this using a tool managed by EMURGO & dcSpark called `cddl-codegen`
which can be re-used for other tasks such as automatically generate a
Rust library for Cardano metadata specifications!

The most important feature of this is that CML has been generated to allow all CBOR details to be preserved.
With CBOR many CBOR structures can have multiple ways to serialize to bytes from the same equivalent structure.
This causes issues especially when computing hashes and is a frequent problem with working across tools e.g. cardano-node-cli and cardano-serialization-lib encoding plutus datums differently. This makes CML much more compatible with all other libraries as it will remember all these specific CBOR encoding details. This is particularly important for use with dApps and wallets connecting to dApps.

It is also very easy to create scripts in Rust or WASM to share with
stake pools, or even embed inside an online tool! No more crazy
cardano-cli bash scripts!

Powerful and flexible enough to be used to power wallets and exchanges!
(Yes, it’s used in production!)

## A note on code examples

All code examples are using the WASM (typescript/javascript) API. If you are using CML from rust you will need to change the code to rust syntax e.g. `Foo.bar()` to `Foo::new()` etc. We've tried to keep the API as consistent as possible between the different bindings but some exceptions exist. The array/map wrappers (e.g. `FooList` / `MapFooToBar`) in WASM are simply `Vec<Foo>` and `OrderedHashMap<Foo, Bar>` respectively. There will be some changes relating to reference params/moving/etc as well.

You can find complete examples in the `/examples/` directory.

## Documentation

This library generates `Typescript` type definitions, so it’s often easiest to see what is possible by just looking at the types! These are found in the `.ts` file in the npm package roots.
If you are using rust the full API will be shown in the respective crates.io pages.