#!/usr/bin/env bash
#
# Regenerate the cddl-codegen-derived Rust sources from the CDDL specs in specs/.
#
# DESIGN: regenerate IN PLACE, straight over the crate source dirs, and use `git diff`
# as the review-and-merge tool — the same way CML has always regenerated.
#
# Rationale: the tree is version-controlled, so git already is the diff/merge tool we'd
# otherwise have to rebuild. After a regen, `git status`/`git diff` show exactly what the
# new codegen changed, surface anything it clobbered (the hand-written builders/, utils.rs,
# or a tuned Cargo.toml), and let you keep or revert per file (`git checkout -p`,
# `git checkout -- <file>`). cddl-codegen only rewrites the files it generates; it does not
# wipe the output dir, so hand-added files like builders/ are left untouched. A scratch
# output dir was considered and rejected: it hides all of the above behind custom
# diff/copy tooling that in practice just copies everything back and then reads git diff
# anyway. So: regen, then read the diff. Run on a clean (committed/stashed) tree so the
# diff is only the regen.
#
# cddl-codegen is pinned (CDDL_CODEGEN_REV) so a regen reflects spec changes, not codegen
# drift. Bump CDDL_CODEGEN_REV in its own commit when intentionally adopting a newer codegen
# (e.g. for a new era).
#
# WHAT STILL SHOWS IN THE DIFF AFTER A NO-OP REGEN (all genuine hand-editing — reconcile, don't fight):
#   - plutus/ and transaction/ modules: heavily customized by hand on top of generation
#   - serialization.rs (top-level): a hand-written shim re-exporting cml_core's serialization
#   - lib.rs: hand-maintained `pub mod` list (incl. builders/byron/genesis/json/utils — non-generated)
#   - Cargo.toml: hand-tuned deps (cddl-codegen emits a generic one)
#   - per-module utils.rs (hand-added) and a few `#[allow(clippy::...)]`
# The bulk of every generated module reproduces exactly; the above is the irreducible manual part.
# core/rust/src's five runtime files (error/serialization/ordered_hash_map/non_empty*) are ALSO
# tool-owned now, refreshed via --export-static-crate on the chain invocation — which additionally
# merges the runtime's required dep versions into core/rust/Cargo.toml
# so a codegen-side dep bump can't skew against core's manifest; CML's additions in the files
# ride along in cddl-codegen:insert/replace blocks (see the chain gen call below),
# and hand-added deps/keys in the manifest pass through the merge untouched.
#
# Usage:   ./codegen.sh                 # regenerate all crates in place
#          ./codegen.sh chain           # regenerate a single crate
#          CDDL_CODEGEN_DIR=~/src/cddl-codegen ./codegen.sh   # use a local checkout
set -euo pipefail

# cddl-codegen commit the specs target. Override with CDDL_CODEGEN_DIR
# NOTE: this rev PREDATES the extern-wrapper-dedup feature (--no-synthesized-rust-collection-aliases,
# --extern-wrapper-index, generated collections.rs), workspace mode (--workspace-dep,
# --wrapper-requests, borrowed/requested_collections.rs), the cml-core runtime export
# (--export-static-dir), AND the wrapper-requests alias-element fix + hardening (without which the
# dep-side regen PANICS on this workspace's real sidecar — alias elements like stake_credential).
# It ALSO predates the @custom_json fix for sum-type/record encoding fields + record derives
# (c9c47b0), which specs/conway/plutus.cddl's plutus_data now relies on, AND the crate-shaped
# static export (--export-static-crate, which replaced --export-static-dir and also merges
# core/rust/Cargo.toml — the chain invocation below uses the NEW flag, so it needs a rev that
# has it; an older rev fails loudly on the unknown flag).
# Bump to a rev with all of the above once they are on the GitHub remote — nothing earlier; e.g.
# 2bff93f has --export-static-dir but not the alias fix, and 18fb7cc lacks the @custom_json fix.
# Until then regen only works via CDDL_CODEGEN_DIR pointing at a local checkout.
CDDL_CODEGEN_REV="77237871a3d2585996b103fbcc03bd227606c445"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPECS="$REPO_ROOT/specs"

# Clone + check out the pinned cddl-codegen (cached between runs; override with CDDL_CODEGEN_DIR).
# Kept OUTSIDE the repo: a checkout under REPO_ROOT gets absorbed by this Cargo workspace and
# cargo refuses to build a nested package that isn't a member. Outside, it also builds with the
# system toolchain rather than this repo's pinned rust-toolchain.toml.
WORK="${CDDL_CODEGEN_DIR:-${XDG_CACHE_HOME:-$HOME/.cache}/cml-cddl-codegen}"
if [ -z "${CDDL_CODEGEN_DIR:-}" ]; then
  mkdir -p "$(dirname "$WORK")"
  if [ ! -d "$WORK/.git" ]; then
    git clone https://github.com/dcSpark/cddl-codegen "$WORK"
  fi
  git -C "$WORK" fetch --quiet origin
  git -C "$WORK" checkout --quiet "$CDDL_CODEGEN_REV"
fi

# gen <group-dir> <input> <extra cddl-codegen args...>
# --output is the crate-GROUP dir (e.g. `chain`): cddl-codegen writes `<group>/rust/`,
# `<group>/wasm/` and `<group>/wasm/json-gen/` under it, matching CML's layout.
gen() {
  local out="$1" input="$2"; shift 2
  echo ">> generating $out"
  ( cd "$WORK" && cargo run --quiet -- --input="$input" --output="$REPO_ROOT/$out" "$@" )
}

# Shared flags.
#   --common-import-override=cml_core is THE flag that makes regen reproduce CML's layout: it
#     points the common scaffolding (error/serialization/ordered_hash_map) at the cml-core crate
#     AND suppresses generating those files locally (export_static_files() => false). Without it
#     cddl-codegen emits self-contained `crate::error` etc. and a wall of import diffs. Every CML
#     crate depends on cml-core, so all of them pass it.
#   --wasm enables wasm Cargo.toml features
#   --wasm-cbor-json-api-macro / --wasm-conversions-macro make the wasm bindings emit a macro CALL
#     (into cml_core_wasm) for each type's CBOR/JSON API and From/AsRef conversions, instead of
#     inlining those impls per-struct. The committed wasm uses the macro form; without these flags
#     every wasm struct gets a wall of inlined boilerplate. Shared by all crates (incl. cip25).
#   --lib-name=cml-<crate> (passed PER gen call below) sets the rust package name and the wasm
#     self-reference (cml_chain::... etc.); without it everything is the default `cddl-lib`/cddl_lib.
#   --wasm-list-macro collapses each Vec<T>-backed wasm list wrapper (struct + new/len/get/add +
#     conversions) into a single impl_wasm_list_needs_into!(rust, wasm, Name, needs_into, is_copy)
#     call. It supersedes --wasm-conversions-macro for list wrappers. The shim adapts the flag's
#     needs_into polarity to cml_core_wasm::impl_wasm_list (whose 4th arg is inverted).
#   --no-synthesized-rust-collection-aliases=true suppresses the dead rust `pub type FooList =
#     Vec<Foo>;` aliases minted for generator-SYNTHESIZED collection wrappers (table keys-lists,
#     anonymous shapes). Rule-declared aliases are never touched. Generated code is structural, so
#     this is emission-only; it removes public rust API in EVERY crate (incl. chain's own
#     PolicyIdList etc.) — intentional, they were dead re-declarations.
OVERRIDE=(--common-import-override=cml_core --no-synthesized-rust-collection-aliases=true)
WASM_MACROS=(--wasm true --wasm-cbor-json-api-macro=cml_core_wasm::impl_wasm_cbor_json_api --wasm-conversions-macro=cml_core_wasm::impl_wasm_conversions --wasm-list-macro=cml_core_wasm::impl_wasm_list_needs_into)
CIP25_WASM_MACROS=(--wasm true --wasm-cbor-json-api-macro=cml_core_wasm::impl_wasm_cbor_json_api_cbor_event_serialize --wasm-conversions-macro=cml_core_wasm::impl_wasm_conversions --wasm-list-macro=cml_core_wasm::impl_wasm_list_needs_into)
COMMON=(--preserve-encodings=true --canonical-form=true --json-serde-derives=true --json-schema-export=true "${OVERRIDE[@]}" "${WASM_MACROS[@]}")

# --extern-wasm-crate=<dep>=<dep>_wasm: multi-era references chain types as cross-crate extern deps
#   (specs/multiera{,-byron}/_CDDL_CODEGEN_EXTERN_DEPS_DIR_/cml_chain). In the wasm pass, boundary
#   types and their `use` imports must resolve through the dep's WASM crate (cml_chain_wasm), not its
#   rust crate; inner storage keeps the rust type. Only multi-era has a cml_chain extern dep — a
#   mapping that names no extern dep in the spec aborts codegen, so this is NOT in COMMON.
# Workspace mode (multi-era <-> chain). Three flags cooperate:
#   --workspace-dep=cml_chain: every wrapper whose element types are ALL chain's (transitively,
#     NonEmpty included) is BORROWED — multi-era imports chain's class unconditionally and records
#     the need in multi-era/wasm/src/generated/borrowed_collections.rs (the request sidecar).
#   --extern-wrapper-index (kept): covers OWNERLESS shapes (primitives-only, e.g. {* u64 => [* i64]})
#     which have no owning dep — those still index-defer against chain's committed collections.rs.
#     Mixed-element wrappers (chain key + multi-era value) always stay local.
#   --wrapper-requests=cml-multi-era=<sidecar> (on CHAIN's invocation): chain reads the committed
#     sidecar and hosts every requested wrapper its own spec doesn't produce, in
#     chain/wasm/src/generated/requested_collections.rs (indexed, attributed).
# ORDERING is REVERSE dependency order — consumers before deps: multi-era regens first (rewrites
# its sidecar), chain regens after (reads it). Single-crate runs read the other crate's committed
# state and are safe; the one loud case is a multi-era-only run that ADDS a borrow — multi-era
# won't build (unresolved import naming the wrapper) until chain regens too.
EXTERN_WASM_MULTIERA=(--extern-wasm-crate=cml_chain=cml_chain_wasm --extern-wrapper-index=cml_chain="$REPO_ROOT/chain/wasm/src/generated/collections.rs" --workspace-dep=cml_chain)
# The opt-in byron pass generates into the SAME multi-era crate from a different spec set. It must
# NOT pass --workspace-dep: it would rewrite borrowed_collections.rs from byron's specs alone,
# clobbering the main pass's request set (byron's chain-element wrappers are hand-written in
# multi-era/wasm/src/byron/ and don't overlap the borrowed names). Without the flag the sidecar
# file is left untouched.
EXTERN_WASM_BYRON=(--extern-wasm-crate=cml_chain=cml_chain_wasm --extern-wrapper-index=cml_chain="$REPO_ROOT/chain/wasm/src/generated/collections.rs")
MULTIERA_SIDECAR="$REPO_ROOT/multi-era/wasm/src/generated/borrowed_collections.rs"
WRAPPER_REQUESTS_CHAIN=(--wrapper-requests=cml-multi-era="$MULTIERA_SIDECAR")

ARGS=("$@")
# named <crate>: true only when <crate> was passed explicitly. Used for opt-in passes (byron)
# that must NOT run as part of the default no-arg `./codegen.sh`.
named() {
  local a; for a in "${ARGS[@]}"; do [ "$a" = "$1" ] && return 0; done
  return 1
}
# want <crate>: true when no crate filter was given, or <crate> is one of the args.
want() { [ ${#ARGS[@]} -eq 0 ] || named "$1"; }

# In-place regen is only reviewable against a clean tree; warn (don't block) if it's dirty.
if [ -n "$(git -C "$REPO_ROOT" status --porcelain -- chain multi-era cip25 cip36 2>/dev/null)" ]; then
  echo "WARNING: crate dirs have uncommitted changes — commit or stash first so the regen diff is clean." >&2
fi

# Crate              spec input                 args
#   multi-era  <- specs/multiera      (uses _CDDL_CODEGEN_EXTERN_DEPS_DIR_/cml_chain to reference chain)
#   chain      <- specs/conway        (Conway-era on-chain types; README.md historically said babbage)
#   cip36      <- specs/cip36.cddl
#   cip25      <- specs/cip25.cddl    (no preserve-encodings)
# multi-era BEFORE chain: workspace-mode reverse dependency order (see comment above).
want multi-era && gen multi-era "$SPECS/multiera" --lib-name=cml-multi-era "${COMMON[@]}" "${EXTERN_WASM_MULTIERA[@]}"
if want chain; then
  if [ -f "$MULTIERA_SIDECAR" ]; then
    # --export-static-crate: refresh cml-core's copy of the static runtime (error.rs,
    # serialization.rs prelude, ordered_hash_map.rs, non_empty*.rs → core/rust/src/) AND merge the
    # runtime's required dep versions into core/rust/Cargo.toml, so neither the files nor the
    # manifest can rot against the codegen rev. Passed on
    # the CHAIN invocation only — it carries the maximal flavor
    # (preserve-encodings + canonical + json-serde + json-schema) that the shared runtime must
    # serve; cip25's reduced flavor would export a non-preserve runtime and break the others.
    # CML-specific additions inside those five files (BadAddressType/OutOfRange/ArithmeticError,
    # ToBytes/FromBytes, len_to_len_sz, OrderedHashMap::take, the lenient from_cbor_bytes and
    # bool-not-blanket Deserialize impls) live in cddl-codegen:insert/replace blocks and are
    # re-applied by the preservation overlay on every regen; anything it can't re-place traps in
    # a loud compile_error!, never silently dropped. Hand-added manifest deps/keys pass through
    # the merge untouched. Everything else in core/ stays hand-owned.
    gen chain "$SPECS/conway" --lib-name=cml-chain "${COMMON[@]}" "${WRAPPER_REQUESTS_CHAIN[@]}" \
      --export-static-crate="$REPO_ROOT/core/rust"
  else
    # Only possible before the first multi-era regen under workspace mode. Without the sidecar,
    # chain would silently drop every hosted wrapper multi-era needs — refuse instead.
    echo "ERROR: $MULTIERA_SIDECAR missing — regenerate multi-era first (./codegen.sh multi-era)." >&2
    exit 1
  fi
fi
want cip36     && gen cip36     "$SPECS/cip36.cddl" --lib-name=cml-cip36    "${COMMON[@]}"
#    cip25 doesn't use COMMON as it deliberately omits preserve-encodings/canonical-form (it never had them).
want cip25     && gen cip25     "$SPECS/cip25.cddl" --lib-name=cml-cip25 --json-serde-derives=true --json-schema-export=true "${OVERRIDE[@]}" "${CIP25_WASM_MACROS[@]}"

# Byron is a legacy era (original cardano-sl format). It is NOT regenerated by default because a
# routine regen would silently break it — it needs special args AND a hand-edit cddl-codegen can't
# produce:
#   - Byron's CBOR encoding is deterministic, so it is generated WITHOUT --preserve-encodings /
#     --canonical-form (which is why the committed byron has no cbor_encodings.rs files). Hence the
#     reduced arg set below, not "${COMMON[@]}".
#   - BUT real Byron blocks encode the block body payloads (tx/dlg/ssc/upd) as INDEFINITE-length CBOR
#     arrays, while cddl-codegen emits DEFINITE-length. The committed byron serialization is therefore
#     hand-edited to indefinite (grep `Len::Indefinite` in byron/*/serialization.rs). That edit MUST be
#     re-applied after any byron regen or real Byron blocks stop round-tripping (the golden block tests
#     catch this). Re-applying it cleanly is the open work for "properly code-generating byron".
# The cml_chain extern dep is already correctly placed under
# specs/multiera-byron/_CDDL_CODEGEN_EXTERN_DEPS_DIR_/ so it is not generated as a local module.
named byron && gen multi-era "$SPECS/multiera-byron" --lib-name=cml-multi-era --json-serde-derives=true --json-schema-export=true "${OVERRIDE[@]}" "${WASM_MACROS[@]}" "${EXTERN_WASM_BYRON[@]}"

# Workspace convergence check: every wrapper multi-era's sidecar borrows from cml_chain must be
# listed in chain's collections index (satisfied either by chain's own spec rules or by hosting
# in requested_collections.rs). The ONLY ordering that can go stale is "multi-era regenerated
# after chain" — e.g. a single-crate multi-era run that added a borrow, or reverting chain's
# regen diff during reconciliation. Without this check that state surfaces later as confusing
# missing-type compile errors in cml-multi-era-wasm; with it, the fix is printed in one line.
# Runs on the COMMITTED/on-disk state, so it is also valid after partial runs.
check_convergence() {
  local index="$REPO_ROOT/chain/wasm/src/generated/collections.rs"
  [ -f "$MULTIERA_SIDECAR" ] && [ -f "$index" ] || return 0
  local missing="" name
  while IFS= read -r name; do
    grep -q "::${name};" "$index" || missing="$missing $name"
  done < <(sed -n 's/^ *use cml_chain_wasm::collections::\([A-Za-z0-9_]*\);$/\1/p' "$MULTIERA_SIDECAR")
  if [ -n "$missing" ]; then
    echo "ERROR: chain's collections index is STALE relative to multi-era's borrowed_collections.rs sidecar." >&2
    echo "  not yet provided by chain:$missing" >&2
    echo "  fix (dep-alone regen is always safe): ${CDDL_CODEGEN_DIR:+CDDL_CODEGEN_DIR=$CDDL_CODEGEN_DIR }./codegen.sh chain" >&2
    return 1
  fi
}

echo "Running clippy --fix on the regenerated code..." >&2
cargo fmt --all
# cml-core is excluded from clippy --fix: its runtime files are now tool-owned (see
# --export-static-dir above) and clippy's rewrites there (e.g. inline format args) would be
# reverted by the next regen, churning the diff forever. Regular clippy still checks cml-core.
cargo clippy --fix --allow-dirty --allow-staged --workspace --exclude cml-core --all-features --all-targets

check_convergence

cat <<'EOF'

Done — sources regenerated in place. Review with git:

  git status               # what changed, was added, or was clobbered
  git diff                 # inspect the regenerated output
  git checkout -- <file>   # revert a clobbered hand-written file (Cargo.toml, builders/, utils.rs)
  git checkout -p          # or pick hunks to keep/revert interactively

Then sanity-build:  cargo test -p cml-chain   (etc.)
EOF
