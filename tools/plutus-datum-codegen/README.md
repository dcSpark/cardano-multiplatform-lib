# Plutus Datum Codegen

This is a tool that allows generating a rust/wasm library via cddl-codegen for parsing/creating Plutus datum specs.

The resulting tool will be directly integrated into CML and will have the benefits over direct `PlutusData` that the structure verification will be automatically done for you, as well as the interface matching the spec.

This tool will first verify that the input spec conforms to the plutus datum spec, and only if it does, will call cddl-codegen to generate it.

## How to use

```
cargo run -- --input=input.cddl --output=EXPORT --cddl-codegen=path/to/cddl-codegen --lib-name=plutus-datum-test
```

The input file must be a single file. Directories are not supported. If `--cddl-codegen` is a directory it will assume that this is the source code and `cargo run` will be run in that directory. If it is a file it will be assumed to be the compiled cddl-codegen binary, in which case `--static-dir` must be supplied e.g. the `/static/` dir of the `cddl-codegen` repo.

## Prelude

```cddl
bounded_bytes = bytes ; @custom_serialize cml_chain::utils::write_bounded_bytes @custom_deserialize cml_chain::utils::read_bounded_bytes @no_alias
```

The bytes serialization format for plutus datums is slightly different than arbitrary CBOR bytes. Please use the alias `bounded_bytes` that hooks into the serialization code in CML.

```cddl
utf8_text = text ; @custom_serialize crate::utils::serialize_utf8_bytes @custom_deserialize crate::utils::deserialize_utf8_bytes @no_alias

```

Plutus datums do not natively allow CBOR text, however, we provide a `utf8_bytes` alias that will be treated as `String` in the user-facing API with only the CBOR serialization logic converting to/from bytes using the utf8 byte representation. If this alias is used, the corresponding serialization functions will be exported into `utils.rs`.

## Example

```cddl
; tagged constructor (variant 2, concise fixed format)
; note that to the user this will be text, but on-chain it will be serialized to utf8 bytes
foo = #6.123([* utf8_text])

; regular array datum, but with specific struct structure forced on top
abc = [
  ; note that we MUST use bounded_bytes not regular bytes
  ; this is only an encoding detail (chunked into <=64-byte chunks)
  ; it is still regular bytes
  x: bounded_bytes,
  y: [* utf8_text],
]

; tagged constructor (arbitrary variant, generic format)
bar = #6.102([
  variant: uint,
  fields: [* abc]
])
```