# Generating from these specs

We generate using [cddl-codegen](https://github.com/dcSpark/cddl-codegen) using the following arguments:

For `chain`:

```
--input=specs/babbage --output=CML_CHAIN_DIR --preserve-encodings=true --canonical-form=true --json-serde-derives=true --json-schema-export=true
```

For `cip36`:

```
--input=specs/cip36.cddl --output=CML_CIP36_DIR --preserve-encodings=true --canonical-form=true --json-serde-derives=true --json-schema-export=true
```

For `cip25`:

```
--input=specs/cip25.cddl --output=CML_CIP25_DIR --json-serde-derives=true --json-schema-export=true
```

To run from the cddl-codegen directory this would be prefixed with `cargo run -- --input=specs/...`

# Generating CDDL instances

First you need to install `cddl`
```
sudo apt install ruby
sudo gem install cddl
sudo gem install cbor-diag
```

You can generate new tests with
1) `cddl specs/shelley.cddl generate 1 > test/name_here.diag`
2) `diag2cbor.rb test/name_here.diag > test/name_here.cbor`

You can combine these together with `cddl specs/shelley.cddl generate 1 | diag2cbor.rb > test/name_here.cbor`
