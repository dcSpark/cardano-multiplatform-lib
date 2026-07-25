#!/usr/bin/env bash
#
# The clippy gate — THE definition of it. Both consumers invoke this script and pass no lint
# flags of their own:
#   - CI:         .github/workflows/pr-checks.yml (cargo-clippy step)
#   - codegen.sh: the post-regen check
# If you ever find yourself writing `--allow clippy::…` in either of those, it belongs here.
#
# WHY THE ALLOW LIST IS THIS SHORT (and should stay that way)
#
# Lints on cddl-codegen OUTPUT are not this file's business. The generator emits its own
# `#![allow(…)]` at each generated module root — the rust roots allow
# too_many_arguments/large_enum_variant/result_large_err, the wasm roots
# len_without_is_empty/too_many_arguments/new_without_default (see any `src/generated/mod.rs`).
# That scopes each exception to exactly the code that needs it and leaves the lint live for our
# hand-written code, which a repo-wide `--allow` here cannot do.
#
# Measured 2026-07-25 on toolchain 1.96.1, `--workspace --all-features --all-targets`, both on
# HEAD and on the in-progress regen tree: ZERO diagnostics under `**/src/generated/`. So a
# repo-wide allow for a generated-code lint currently buys nothing and costs coverage of hand
# code. That is why five entries this list used to carry — useless_conversion, manual_repeat_n,
# single_match, double_ended_iterator_last, derivable_impls — are gone: upstream fixed the
# emitter (draft/clippy-fixes/RESPONSE-2026-07-22.md) and nothing has tripped them since.
#
# So this gate has two owners: a lint in hand-written code is ours to fix at the site; a lint
# under src/generated/ is an upstream regression to REPORT, not to silence here — silencing it is
# what turns the signal off. See
# draft/REQUEST-2026-07-25-request-10-generated-code-clippy-ownership.md. If one ever has to be
# absorbed locally (upstream fix weeks out, tree must ship), add it with the upstream issue link
# in the comment and delete it when the fix lands — it is a workaround, not policy.
#
# Everything below is therefore a statement about OUR hand-written code.
#
# Usage:  ./clippy.sh                # the gate, exactly as CI runs it
#         ./clippy.sh -p cml-chain   # extra cargo-clippy args are forwarded
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"

LINTS=(
  --deny clippy::all

  # Deliberate architecture, not debt: TxBuilderError/WitnessBuilderError are large enums
  # returned by ~35 builder functions (chain/rust/src/builders/{tx,witness}_builder.rs).
  # Boxing them to satisfy the lint would change the signature of every builder call.
  --allow clippy::result_large_err

  # Same trade, other direction: hand-written era enums whose variants are inherently uneven
  # (multi-era/rust/src/utils.rs, multi-era/rust/src/byron/block/mod.rs). Boxing a variant
  # changes the public shape of the type.
  --allow clippy::large_enum_variant

  # Cosmetic debt, 3 sites: chain/rust/src/byron/base58.rs,
  # chain/rust/src/genesis/shelley/raw.rs, core/wasm/src/wasm_wrappers.rs.
  # Drop this entry once the blank line after those doc comments is removed.
  --allow clippy::empty_line_after_doc_comments
)

exec cargo clippy --workspace --all-features --all-targets "$@" -- "${LINTS[@]}"
