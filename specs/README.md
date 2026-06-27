# Generating from these specs

**Run `./codegen.sh` from the repo root** — it is the single source of truth for the
[cddl-codegen](https://github.com/dcSpark/cddl-codegen) version and per-crate arguments.
It pins cddl-codegen to the exact commit that produced the committed tree, so a regen diff
reflects spec changes only. It regenerates **in place** over the crate source dirs; you then
review with `git diff` and revert any clobbered hand-written files (`builders/`, `utils.rs`,
`Cargo.toml`) with `git checkout`. Run it on a clean tree so the diff is only the regen.
See the header of `codegen.sh` for the full rationale and the pinned commit.

For reference, the inputs/args it uses (run from a cddl-codegen checkout as
`cargo run -- --input=specs/...`):

| Crate       | Input                  | Extra args |
|-------------|------------------------|------------|
| `chain`     | `specs/conway`         | `--preserve-encodings=true --canonical-form=true --json-serde-derives=true --json-schema-export=true` |
| `multi-era` | `specs/multiera` (+ `specs/multiera-byron`) | same as `chain` |
| `cip36`     | `specs/cip36.cddl`     | same as `chain` |
| `cip25`     | `specs/cip25.cddl`     | `--json-serde-derives=true --json-schema-export=true` (no preserve-encodings) |

> Note: `chain` was historically generated from `specs/babbage`; it now uses `specs/conway`.

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
