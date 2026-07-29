# Generating from these specs

**Run `./codegen.sh` from the repo root** to regenerate the cddl-codegen-derived Rust. The
per-crate arguments live in the committed `codegen.toml` at the repo root (every key there is a
[cddl-codegen](https://github.com/dcSpark/cddl-codegen) flag; `./codegen.sh --print-flags` shows
the full expansion); the script owns the pinned tool version and the opt-in byron pass, and
regenerates **in place** so you review the result with `git diff`. See the headers of
`codegen.sh` and `codegen.toml` for the full rationale (including which files are hand-edited
and expected to stay in the diff).

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
