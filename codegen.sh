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
# (e.g. for a new era). The pin can be overridden per-run from the environment
# (CDDL_CODEGEN_REV=<sha> ./codegen.sh). Combined with CDDL_CODEGEN_DIR it selects a commit from
# that checkout's object store — read-only, the checkout itself is never modified — instead of
# whatever commit it happens to sit on. An env override is a one-off for testing an unmerged or
# older codegen; the committed pin is what the tree is generated against, so anything you keep
# must be regenerated against a committed pin before landing.
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
#          CDDL_CODEGEN_DIR=~/src/cddl-codegen ./codegen.sh   # use a local checkout, as it stands
#          CDDL_CODEGEN_REV=<sha> ./codegen.sh                # use a rev other than the pin
#          CDDL_CODEGEN_REV=<sha> CDDL_CODEGEN_DIR=~/src/cddl-codegen ./codegen.sh
#                                       # that commit FROM that checkout's object store, without
#                                       # touching the checkout (dirty tree / other agent = fine)
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
# The 2026-07-19 upstream cycle (see draft/migrations/RESPONSE-2026-07-19-*.md) added MORE the
# bump rev must include:
#   - --rust-wasm-feature (feature-gates the c-style-enum #[wasm_bindgen] in the RUST crate via
#     cfg_attr, replacing the old noop_proc_macro import shim the committed generated code still
#     carries; the flag is passed below, so an older rev fails loudly on it) and the automatic
#     repair of the legacy `used_from_wasm = ["wasm-bindgen"]` feature list to ["dep:…"]
#   - banner-only extern_interface_check.rs / key_demand_assertions.rs (per-row `// <rule>`
#     markers removed — they self-perpetuated as unpreserved-comment traps after rule deletions;
#     our tree carries no trap blocks, so no one-time hand-cleanup is needed here)
#   - json-gen extern-row fixes (skips uncompilable rows for generic-extern bases and
#     workspace-dep-owned types; KEEPS rows for own-spec externs, which now contractually need
#     schemars::JsonSchema — all of CML's externs already impl it)
#   - borrowed_key_types.rs self-check emits the dep's SCOPED path (bare machine rows unchanged;
#     sidecar bytes only change if a map is keyed on a non-root dep type)
# Bump to a rev with all of the above once they are on the GitHub remote — nothing earlier; e.g.
# 2bff93f has --export-static-dir but not the alias fix, and 18fb7cc lacks the @custom_json fix.
# The 2026-07-20 cycle added two more requirements (both shipped upstream, see
# draft/feature-requests/RESPONSE-2026-07-20-request-0{7,8}.md in the cddl-codegen repo):
#   - the REQUEST-07 fix (generic-extern instance `Base<Args>` leaked into scope `use` lists;
#     previously blocked the chain regen entirely)
#   - the REQUEST-08 series (transparent tag-258 set idiom, branch feature/request-08-cbor-set):
#     specs/conway/lib.cddl now DEFINES set<T>/nonempty_set<T> as the `#6.258([* T]) / [* T]`
#     choice instead of externing them, and the hand NonemptySet/NonemptySetRawBytes impls
#     (chain/rust/src/utils.rs) plus the wasm alias/conversion shims were DELETED. A rev without
#     the request-08 collapse mis-models these rules as two-variant enums — do not regen chain
#     on anything older.
# As of 2026-07-19 the earlier cycle's fixes are NOT yet pushed (only e07c3a0 of the referenced
# commits is on the remote). Until everything above lands on the remote, regen only works via
# CDDL_CODEGEN_DIR pointing at a local checkout.
CDDL_CODEGEN_PINNED_REV="77237871a3d2585996b103fbcc03bd227606c445"
# A CDDL_CODEGEN_REV in the environment wins over the pin. Remember whether it was set explicitly:
# with CDDL_CODEGEN_DIR the two cases differ (HEAD as-is vs. move the checkout to that commit).
CDDL_CODEGEN_REV_EXPLICIT=0
[ -n "${CDDL_CODEGEN_REV:-}" ] && CDDL_CODEGEN_REV_EXPLICIT=1
CDDL_CODEGEN_REV="${CDDL_CODEGEN_REV:-$CDDL_CODEGEN_PINNED_REV}"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPECS="$REPO_ROOT/specs"

# Clone + check out the pinned cddl-codegen (cached between runs; override with CDDL_CODEGEN_DIR).
# Kept OUTSIDE the repo: a checkout under REPO_ROOT gets absorbed by this Cargo workspace and
# cargo refuses to build a nested package that isn't a member. Outside, it also builds with the
# system toolchain rather than this repo's pinned rust-toolchain.toml.
CACHE_ROOT="${XDG_CACHE_HOME:-$HOME/.cache}"
WORK="${CDDL_CODEGEN_DIR:-$CACHE_ROOT/cml-cddl-codegen}"

# Cargo env for trees THIS SCRIPT materializes (the cached clone, and extracted revs below).
# Deliberately NOT applied to a CDDL_CODEGEN_DIR used as-is: that tree is yours, its target/ is
# warm, and redirecting it would force a full rebuild in a directory you're working in.
#   CARGO_TARGET_DIR  — one target dir shared by every tree we build. Dependencies dominate it
#     and are identical across revs, so after the first build a second rev costs ~0 extra bytes
#     there (measured) instead of ~960MB of its own.
#   CARGO_PROFILE_DEV_DEBUG=0 / CARGO_INCREMENTAL=0 — this binary is built to be run once per
#     regen, never stepped through, and never rebuilt from a small edit. Dropping debug info and
#     the incremental cache took that shared dir from 957MB to 324MB for a byte-identical run.
# Net: the whole tool cache is ~324MB once, plus ~31MB of source per extra rev you pin.
CODEGEN_CARGO_ENV=()
CODEGEN_CARGO_LEAN=(
  CARGO_TARGET_DIR="$CACHE_ROOT/cml-cddl-codegen-target"
  CARGO_PROFILE_DEV_DEBUG=0
  CARGO_INCREMENTAL=0
)
if [ -z "${CDDL_CODEGEN_DIR:-}" ]; then
  mkdir -p "$(dirname "$WORK")"
  if [ ! -d "$WORK/.git" ]; then
    git clone https://github.com/dcSpark/cddl-codegen "$WORK"
  fi
  git -C "$WORK" fetch --quiet origin
  git -C "$WORK" checkout --quiet "$CDDL_CODEGEN_REV"
  CODEGEN_CARGO_ENV=("${CODEGEN_CARGO_LEAN[@]}")
elif [ "$CDDL_CODEGEN_REV_EXPLICIT" = 1 ]; then
  # Local checkout AND an explicit rev: generate from that exact COMMIT while leaving the checkout
  # completely alone — no checkout, no fetch, no stash, nothing written to it. `git archive` reads
  # the commit straight out of the object database, so the branch that checkout is on, its index,
  # and any uncommitted work (yours, or another agent's) are untouched and unobserved. That is the
  # point of this path: pinning the generation to a known commit must not cost you the working
  # tree you're developing the codegen in.
  #
  # The commit is materialized once per sha under the cache dir and reused, so repeat regens skip
  # the extract and (via the shared CARGO_TARGET_DIR above) most of the rebuild. `git archive`
  # writes only files TRACKED at that commit — no target/, no .claude/, no untracked test output —
  # so an extracted rev is ~31MB here regardless of how large the source checkout has grown.
  # These dirs are pure cache; delete any of them freely.
  SRC="$WORK"
  # rev-parse so a branch/tag name works too, and so the cache key is always the resolved sha
  # (a branch tip moves; the extracted tree must not be reused for a different commit).
  REV_SHA="$(git -C "$SRC" rev-parse --verify --quiet "${CDDL_CODEGEN_REV}^{commit}")" || {
    echo "ERROR: $SRC has no commit $CDDL_CODEGEN_REV." >&2
    echo "  Fetch it in that checkout first (git -C $SRC fetch), or name a rev it already has." >&2
    exit 1
  }
  WORK="$CACHE_ROOT/cml-cddl-codegen-revs/$REV_SHA"
  if [ -d "$WORK" ]; then
    echo ">> using cached tree for $REV_SHA (extracted from $SRC; that checkout untouched)" >&2
  else
    echo ">> extracting $REV_SHA from $SRC into $WORK (that checkout untouched)" >&2
    # .tmp + mv: an interrupted extract must not leave a half-tree that the next run trusts.
    rm -rf "$WORK.tmp"
    mkdir -p "$WORK.tmp"
    git -C "$SRC" archive "$REV_SHA" | tar -x -C "$WORK.tmp"
    mv "$WORK.tmp" "$WORK"
  fi
  CODEGEN_CARGO_ENV=("${CODEGEN_CARGO_LEAN[@]}")
fi
if [ "$CDDL_CODEGEN_REV_EXPLICIT" = 1 ] && [ "$CDDL_CODEGEN_REV" != "$CDDL_CODEGEN_PINNED_REV" ]; then
  echo "NOTE: generating with cddl-codegen $CDDL_CODEGEN_REV, NOT the pin ($CDDL_CODEGEN_PINNED_REV)." >&2
fi
# The env prefix that reproduces this run, for the re-run hints printed further down.
ENV_PREFIX=""
[ "$CDDL_CODEGEN_REV_EXPLICIT" = 1 ] && ENV_PREFIX="CDDL_CODEGEN_REV=$CDDL_CODEGEN_REV "
[ -n "${CDDL_CODEGEN_DIR:-}" ] && ENV_PREFIX="${ENV_PREFIX}CDDL_CODEGEN_DIR=$CDDL_CODEGEN_DIR "

# gen <group-dir> <input> <extra cddl-codegen args...>
# --output is the crate-GROUP dir (e.g. `chain`): cddl-codegen writes `<group>/rust/`,
# `<group>/wasm/` and `<group>/wasm/json-gen/` under it, matching CML's layout.
gen() {
  local out="$1" input="$2"; shift 2
  echo ">> generating $out"
  # env: empty for a CDDL_CODEGEN_DIR used as-is, the lean/shared cargo settings for trees this
  # script materialized (see CODEGEN_CARGO_ENV above). The ${a[@]+…} form is for `set -u`, which
  # otherwise trips on expanding an empty array.
  ( cd "$WORK" && env ${CODEGEN_CARGO_ENV[@]+"${CODEGEN_CARGO_ENV[@]}"} \
      cargo run --quiet -- --input="$input" --output="$REPO_ROOT/$out" "$@" )
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
#   --rust-wasm-feature=used_from_wasm: c-style enums are the one struct kind whose
#     #[wasm_bindgen] lands in the RUST crate; the codegen gates that attribute behind a cargo
#     feature (cfg_attr) and this flag names it. CML has always gated wasm-bindgen behind
#     `used_from_wasm` (the wasm crates' path deps already enable it), so pass that name instead
#     of the default "wasm". The regen also repairs the legacy `used_from_wasm = ["wasm-bindgen"]`
#     feature list to ["dep:wasm-bindgen"] and replaces the old noop_proc_macro import shim.
#   --no-synthesized-rust-collection-aliases=true suppresses the dead rust `pub type FooList =
#     Vec<Foo>;` aliases minted for generator-SYNTHESIZED collection wrappers (table keys-lists,
#     anonymous shapes). Rule-declared aliases are never touched. Generated code is structural, so
#     this is emission-only; it removes public rust API in EVERY crate (incl. chain's own
#     PolicyIdList etc.) — intentional, they were dead re-declarations.
OVERRIDE=(--common-import-override=cml_core --no-synthesized-rust-collection-aliases=true)
WASM_MACROS=(--wasm true --rust-wasm-feature=used_from_wasm --wasm-cbor-json-api-macro=cml_core_wasm::impl_wasm_cbor_json_api --wasm-conversions-macro=cml_core_wasm::impl_wasm_conversions --wasm-list-macro=cml_core_wasm::impl_wasm_list_needs_into)
CIP25_WASM_MACROS=(--wasm true --rust-wasm-feature=used_from_wasm --wasm-cbor-json-api-macro=cml_core_wasm::impl_wasm_cbor_json_api_cbor_event_serialize --wasm-conversions-macro=cml_core_wasm::impl_wasm_conversions --wasm-list-macro=cml_core_wasm::impl_wasm_list_needs_into)
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
#   cip36      <- specs/cip36   (extern-deps dirs attribute crypto/chain types to their crates)
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
    # --extern-wasm-crate=cml_core=cml_core_wasm: chain's spec references the built-in `int`
    # (DeltaCoin), whose wasm face must resolve through cml_core_wasm in wasm-bindgen-exported
    # signatures (otherwise `pub use cml_core::Int;` → E0277 on every DeltaCoin API). Accepting
    # the common-import-override crate as a mapping key landed upstream in 71e1d87 (cycle-2
    # response); this is the flag doc's own documented pairing with --common-import-override.
    gen chain "$SPECS/conway" --lib-name=cml-chain "${COMMON[@]}" "${WRAPPER_REQUESTS_CHAIN[@]}" \
      --extern-wasm-crate=cml_core=cml_core_wasm \
      --export-static-crate="$REPO_ROOT/core/rust"
  else
    # Only possible before the first multi-era regen under workspace mode. Without the sidecar,
    # chain would silently drop every hosted wrapper multi-era needs — refuse instead.
    echo "ERROR: $MULTIERA_SIDECAR missing — regenerate multi-era first (./codegen.sh multi-era)." >&2
    exit 1
  fi
fi
# cip36's crypto/chain types are declared in _CDDL_CODEGEN_EXTERN_DEPS_DIR_/{cml_crypto,cml_chain}
# (dep-owned, NOT bare own-spec externs), so the generator emits qualified dep paths in rust and —
# via the --extern-wasm-crate mappings — in wasm, and json-gen skips the dep-owned schema rows.
# See draft/remaining/ISSUE-crypto-extern-crate-attribution.md for the rationale (chain's crypto
# externs are the remaining unmigrated case of the same pattern).
want cip36     && gen cip36     "$SPECS/cip36" --lib-name=cml-cip36    "${COMMON[@]}" \
  --extern-wasm-crate=cml_crypto=cml_crypto_wasm --extern-wasm-crate=cml_chain=cml_chain_wasm
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
    echo "  fix (dep-alone regen is always safe): ${ENV_PREFIX}./codegen.sh chain" >&2
    return 1
  fi
}

echo "Running clippy on the regenerated code..." >&2
cargo fmt --all
# CHECK ONLY — deliberately NOT `clippy --fix`. Everything clippy has to say about a file under
# src/generated/ is about code the TOOL wrote: a fix applied here is reverted by the next regen
# (and, for the tool-owned cml-core runtime files, immediately), so --fix churned the diff forever
# and quietly hid lints that the codegen should stop emitting in the first place. Whatever fires
# is a cddl-codegen feature request, not a local edit — see the message below.
# The gate itself (deny level + the hand-code allow list) lives in ./clippy.sh, which CI runs too:
# ONE definition, no list to keep in sync across this script and the workflow.
CLIPPY_STATUS=0
"$REPO_ROOT/clippy.sh" || CLIPPY_STATUS=$?

# Status-captured (rather than fatal on the spot) so a stale-convergence run still reaches the
# clippy guidance below; both statuses are folded into the exit code at the end.
CONV_STATUS=0
check_convergence || CONV_STATUS=$?

cat <<'EOF'

Done — sources regenerated in place. Review with git:

  git status               # what changed, was added, or was clobbered
  git diff                 # inspect the regenerated output
  git checkout -- <file>   # revert a clobbered hand-written file (Cargo.toml, builders/, utils.rs)
  git checkout -p          # or pick hunks to keep/revert interactively

Then sanity-build:  cargo test -p cml-chain   (etc.)
EOF

if [ "$CLIPPY_STATUS" -ne 0 ]; then
  cat >&2 <<EOF

ERROR: clippy is unhappy with the regenerated code (exit $CLIPPY_STATUS).

Do NOT fix this by editing the output or by running \`cargo clippy --fix\`: files under
src/generated/ are rewritten wholesale by the next regen, so the fix disappears and the lint
comes back. File a feature request upstream instead, at

  https://github.com/dcSpark/cddl-codegen/issues

reporting it as a regression in the emitter — quote the lint name and one generated snippet that
trips it. The fix belongs in the emitter; the #![allow(...)] set at each generated module root is
for structural cases only (variant sizes, arg counts), not a dumping ground. Then bump
CDDL_CODEGEN_REV once the fix lands and regen. Do NOT widen ./clippy.sh's allow list to cover
generated code: that silences the lint for our hand-written code too, and — more to the point —
this check exists to DETECT that class of regression, so absorbing it locally turns the detector
off. Five such entries had already gone stale that way by 2026-07-25.

If the lint fires on a HAND-written file (builders/, utils.rs, address.rs, crypto/, core/ outside
the tool-owned runtime files), that one is ours: fix it at the site, or — if the lint is wrong
about our design rather than about one line — add it to the allow list in ./clippy.sh with a
comment naming the sites that justify it.
EOF
fi

[ "$CONV_STATUS" -ne 0 ] && exit "$CONV_STATUS"
exit "$CLIPPY_STATUS"
