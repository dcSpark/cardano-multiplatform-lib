---
sidebar_label: "Introduction"
sidebar_position: 1
---


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


## Pre-requisite knowledge

This library assumes a certain amount of knowledge about how Cardano works (want to avoid re-documenting the wheel).

You can find the specifications of Cardano's ledger [here](https://github.com/input-output-hk/cardano-ledger-specs) which we suggest consulting as you use this library. Notably, the `Shelley ledger formal specification` covers the core concepts.


## Benefits of using this library

Serialization/deserialization code is automatically generated from
Cardano’s official specification, which guarantees it can easily stay up
to date! We do this using a tool managed by EMURGO & dcSpark called `cddl-codegen`_
which can be re-used for other tasks such as automatically generate a
Rust library for Cardano metadata specifications!

It is also very easy to create scripts in Rust or WASM to share with
stake pools, or even embed inside an online tool! No more crazy
cardano-cli bash scripts!

Powerful and flexible enough to be used to power wallets and exchanges!
(Yes, it’s used in production!)

## Documentation

This library generates both `Typescript`_ and `Flow`_ type definitions,
so it’s often easiest to see what is possible by just looking at the
types! You can find the Flow types `here`_

You can also look in the `example`_ folder to see how to use this
library from Typescript or just experiment with the library.