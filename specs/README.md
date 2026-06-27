# Generating from these specs

**Run `./codegen.sh` from the repo root** to regenerate the cddl-codegen-derived Rust. It is
the single source of truth for the [cddl-codegen](https://github.com/dcSpark/cddl-codegen)
version and per-crate arguments, and regenerates **in place** so you review the result with
`git diff`. See the header of `codegen.sh` for the exact args, the pinned commit, and the full
rationale (including which files are hand-edited and expected to stay in the diff).

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
