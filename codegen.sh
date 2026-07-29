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
# WHAT the flags are is no longer this script's business: the four main crates' whole flag
# set lives in the committed codegen.toml at the repo root (every key there is a
# cddl-codegen flag; the tool's docs/docs/config_file.mdx documents the merge rules and
# everything the multi-era<->chain `deps` edge derives). This script's job is WHICH
# cddl-codegen runs (the pin below), the per-machine --static-dir, the opt-in byron pass,
# and the fmt/clippy/review epilogue.
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
# core/rust/src's runtime files (error/serialization/ordered_hash_map/non_empty*/json_*) are ALSO
# tool-owned now, refreshed via codegen.toml's [runtime] export-static-crate (carried by chain's
# invocation, declared by flavor-from) — which additionally merges the runtime's required dep
# versions into core/rust/Cargo.toml so a codegen-side dep bump can't skew against core's manifest;
# CML's additions in the files ride along in cddl-codegen:insert/replace blocks,
# and hand-added deps/keys in the manifest pass through the merge untouched.
#
# Usage:   ./codegen.sh                        # regenerate all four config crates in place
#          ./codegen.sh chain                  # regenerate a single crate
#          ./codegen.sh --with-deps multi-era  # a crate plus the deps it reads, in dep order
#          ./codegen.sh --print-flags          # show what codegen.toml expands to; generate nothing
#          ./codegen.sh byron                  # the opt-in byron pass (never runs by default)
#          CDDL_CODEGEN_DIR=~/src/cddl-codegen ./codegen.sh   # use a local checkout, as it stands
#          CDDL_CODEGEN_REV=<sha> ./codegen.sh                # use a rev other than the pin
#          CDDL_CODEGEN_REV=<sha> CDDL_CODEGEN_DIR=~/src/cddl-codegen ./codegen.sh
#                                       # that commit FROM that checkout's object store, without
#                                       # touching the checkout (dirty tree / other agent = fine)
set -euo pipefail

# cddl-codegen commit the specs target. Override with CDDL_CODEGEN_DIR
# This rev is the no_std line: the OrderedHashMap linked-hash-map->hashlink backing swap, the
# no_std emission series, and cddl-codegen's fixes for CML's two rounds of filings
# (draft/no-std-migration/ + draft/REQUEST-2026-07-29-request-16-no-std-followups.md). Relative
# to the previous pin (2605a7d7) it adds:
#   - no_std-capable generated output: core::/alloc:: paths throughout, a `std` feature on each
#     crate, and a tool-owned no-std-check/ shim per output root.
#   - OrderedHashMap backed by hashlink, with the wrapper Entry that keeps linked-hash-map's
#     position-preserving or_insert/or_default semantics (a moved entry would rewrite the bytes
#     of a table read off the wire -- iteration order IS serialized key order).
#   - the fixes for CML's filings: unpreserved-comment false positives, Entry's missing
#     or_default/and_modify, manifest spec formatting, the nested-inline-module alloc-import gap,
#     runtime-edge `default-features = false` + computed `std` forwarding lists (needs
#     [runtime] lib-name in codegen.toml), and hex -> const-hex with FromHexErrorCore deleted.
#   - the CANONICAL hex grammar (355cef22..2cf061ee): all three emitted hex-reading surfaces
#     accept only what they emit -- bare, even-length, LOWERCASE. This is wire-facing and narrows
#     what your program accepts at runtime: uppercase input, previously normalized, is now
#     rejected. CML follows it wherever CML owns the encoding and deliberately does NOT where the
#     format belongs to someone else (RFC 4291 IPv6, cardano-cli JSON interop). The rule and both
#     doors live in core/rust/src/hex_grammar.rs; chain/rust/tests/hex_grammar.rs pins it.
# The previous pins' deliveries (request-12 through request-15, the --config series, the
# group-choice arm fix, the borrowed_key_types scope qualification) are all ancestors and hold.
# NOTE: this rev is on the LOCAL cddl-codegen checkout's master. Upstream has declined to push
# to the GitHub remote (request-13 §8) — treat this as a PERMANENT condition of consuming an
# unpushed rev, not something to wait out: regen requires CDDL_CODEGEN_DIR pointing at a local
# checkout that has the commit (the default clone-from-GitHub path cannot check it out). CI is
# unaffected; it never regenerates.
CDDL_CODEGEN_PINNED_REV="2cf061ee"
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

# run_config [CRATE...|--with-deps|--print-flags] — the four main crates (chain, cip25, cip36,
# multi-era), generated from the committed codegen.toml. Two command-line-only things remain:
#   --static-dir "$WORK/static": where the PINNED TOOL keeps its own hand-written runtime — a
#     per-machine path a committed config must not bake in, and the one generation flag
#     --config accepts beside it. (Passing it explicitly is what retires the old reliance on
#     `cd "$WORK"` making clap's default `static` resolve there; the cd stays for cargo.)
#   crate selection: positional crate names; --with-deps closes the selection over the
#     config's `deps` edges (transitively) and runs it in generation order.
# Generation order is the config's topological sort — dependencies FIRST (chain, cip25, cip36,
# multi-era; the REVERSE of this script's old hard-coded order) — followed by a convergence
# pass that re-runs chain when the run rewrote a sidecar chain had already read. Exit codes:
#   0  generated (a full run converges in one command);
#   1  the run itself failed (config refused, spec failed) — fix the input;
#   2  the run SUCCEEDED but the committed tree is inconsistent across a `deps` edge (e.g. a
#      multi-era-only regen added a borrow chain doesn't host): the message names the
#      dependency-alone regen that converges it. This is the old hand-rolled
#      check_convergence, generalized to every `deps` edge and moved into the tool.
run_config() {
  # env: empty for a CDDL_CODEGEN_DIR used as-is, the lean/shared cargo settings for trees this
  # script materialized (see CODEGEN_CARGO_ENV above). The ${a[@]+…} form is for `set -u`, which
  # otherwise trips on expanding an empty array.
  ( cd "$WORK" && env ${CODEGEN_CARGO_ENV[@]+"${CODEGEN_CARGO_ENV[@]}"} \
      cargo run --quiet -- --config "$REPO_ROOT/codegen.toml" --static-dir "$WORK/static" "$@" )
}

# gen <group-dir> <input> <extra cddl-codegen args...> — raw flag invocation, used ONLY by the
# opt-in byron pass below. --output is the crate-GROUP dir: cddl-codegen writes `<group>/rust/`,
# `<group>/wasm/` and `<group>/wasm/json-gen/` under it, matching CML's layout.
gen() {
  local out="$1" input="$2"; shift 2
  echo ">> generating $out"
  ( cd "$WORK" && env ${CODEGEN_CARGO_ENV[@]+"${CODEGEN_CARGO_ENV[@]}"} \
      cargo run --quiet -- --input="$input" --output="$REPO_ROOT/$out" "$@" )
}

# Raw flag bundles for the BYRON pass only. These are the same values codegen.toml states for
# the config crates (see its comments for what each flag is for); byron cannot be a crate table
# there — it generates a second spec set into the SAME multi-era crate, and two crate tables can
# share neither a lib-name nor an output — so it passes them as flags. If the config's shared
# keys ever change, `./codegen.sh --print-flags` is how you keep this hand-written set honest.
OVERRIDE=(--common-import-override=cml_core --no-synthesized-rust-collection-aliases=true)
WASM_MACROS=(--wasm true --rust-wasm-feature=used_from_wasm --wasm-cbor-json-api-macro=cml_core_wasm::impl_wasm_cbor_json_api --wasm-conversions-macro=cml_core_wasm::impl_wasm_conversions --wasm-list-macro=cml_core_wasm::impl_wasm_list_needs_into)
# byron declares its chain dep as a hand stub tree (specs/multiera-byron/_CDDL_CODEGEN_EXTERN_
# DEPS_DIR_/cml_chain — NOT migrated to --extern-import; separate decision) and must NOT pass
# --workspace-dep: that would rewrite borrowed_collections.rs from byron's specs alone,
# clobbering the main pass's request set (byron's chain-element wrappers are hand-written in
# multi-era/wasm/src/byron/ and don't overlap the borrowed names). Without the flag the sidecar
# file is left untouched. --extern-wrapper-index still index-defers ownerless shapes against
# chain's committed collections index.
EXTERN_WASM_BYRON=(--extern-wasm-crate=cml_chain=cml_chain_wasm --extern-wrapper-index=cml_chain="$REPO_ROOT/chain/wasm/src/generated/collections.rs")

ARGS=("$@")
# named <arg>: true only when <arg> was passed explicitly. Used for the opt-in byron pass
# (must NOT run as part of the default no-arg ./codegen.sh) and for --print-flags detection.
named() {
  local a; for a in ${ARGS[@]+"${ARGS[@]}"}; do [ "$a" = "$1" ] && return 0; done
  return 1
}

# In-place regen is only reviewable against a clean tree; warn (don't block) if it's dirty.
# core is included: the [runtime] export rewrites its tool-owned runtime files too.
if [ -n "$(git -C "$REPO_ROOT" status --porcelain -- chain multi-era cip25 cip36 core 2>/dev/null)" ]; then
  echo "WARNING: crate dirs have uncommitted changes — commit or stash first so the regen diff is clean." >&2
fi

# Everything that is not `byron` (crate names, --with-deps, --print-flags) flows through to the
# --config invocation; the config validates unknown crate names against its own table.
CONFIG_ARGS=()
for a in ${ARGS[@]+"${ARGS[@]}"}; do [ "$a" = byron ] || CONFIG_ARGS+=("$a"); done

GEN_STATUS=0
if [ ${#ARGS[@]} -eq 0 ] || [ ${#CONFIG_ARGS[@]} -gt 0 ]; then
  run_config ${CONFIG_ARGS[@]+"${CONFIG_ARGS[@]}"} || GEN_STATUS=$?
  # 2 is the committed-state verdict: the run did what it was asked and already printed the
  # dependency-alone regen that converges the tree. Continue so fmt/clippy still cover what WAS
  # regenerated; 2 is folded into the exit code at the end. Anything else nonzero is a failed
  # run — stop where the tool's own error left off.
  if [ "$GEN_STATUS" -ne 0 ] && [ "$GEN_STATUS" -ne 2 ]; then
    exit "$GEN_STATUS"
  fi
fi
# --print-flags generates nothing; skip the fmt/clippy/review epilogue.
if named --print-flags; then
  exit "$GEN_STATUS"
fi

# Byron is a legacy era (original cardano-sl format). It is NOT regenerated by default because a
# routine regen would silently break it — it needs special args AND a hand-edit cddl-codegen can't
# produce:
#   - Byron's CBOR encoding is deterministic, so it is generated WITHOUT --preserve-encodings /
#     --canonical-form (which is why the committed byron has no cbor_encodings.rs files). Hence the
#     reduced arg set below, not the config's `encoded` profile.
#   - BUT real Byron blocks encode the block body payloads (tx/dlg/ssc/upd) as INDEFINITE-length CBOR
#     arrays, while cddl-codegen emits DEFINITE-length. The committed byron serialization is therefore
#     hand-edited to indefinite (grep `Len::Indefinite` in byron/*/serialization.rs). That edit MUST be
#     re-applied after any byron regen or real Byron blocks stop round-tripping (the golden block tests
#     catch this). Re-applying it cleanly is the open work for "properly code-generating byron".
named byron && gen multi-era "$SPECS/multiera-byron" --lib-name=cml-multi-era --json-serde-derives=true --json-schema-export=true "${OVERRIDE[@]}" "${WASM_MACROS[@]}" "${EXTERN_WASM_BYRON[@]}"

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

if [ "$GEN_STATUS" -ne 0 ]; then
  # The committed-state verdict (exit 2): the tool already printed which dependency to regen
  # (typically `./codegen.sh chain`, with your CDDL_CODEGEN_* env repeated — a dependency-alone
  # regen is always safe).
  exit "$GEN_STATUS"
fi
exit "$CLIPPY_STATUS"
