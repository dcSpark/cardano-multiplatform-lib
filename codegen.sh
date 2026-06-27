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
# cddl-codegen is pinned to the commit that produced the current tree so the diff reflects
# spec changes only, not codegen drift. Bump CDDL_CODEGEN_REV in its own commit when
# intentionally adopting a newer codegen (e.g. for a new era).
#
# WHAT STILL SHOWS IN THE DIFF AFTER A NO-OP REGEN (all genuine hand-editing — reconcile, don't fight):
#   - plutus/ and transaction/ modules: heavily customized by hand on top of generation
#   - serialization.rs (top-level): a hand-written shim re-exporting cml_core's serialization
#   - lib.rs: hand-maintained `pub mod` list (incl. builders/byron/genesis/json/utils — non-generated)
#   - Cargo.toml: hand-tuned deps (cddl-codegen emits a generic one)
#   - per-module utils.rs (hand-added) and a few `#[allow(clippy::...)]`
# The bulk of every generated module reproduces exactly; the above is the irreducible manual part.
#
# Usage:   ./codegen.sh                 # regenerate all crates in place
#          ./codegen.sh chain           # regenerate a single crate
#          CDDL_CODEGEN_DIR=~/src/cddl-codegen ./codegen.sh   # use a local checkout
set -euo pipefail

# cddl-codegen commit that generated the committed tree: 1ec516f, the last rev before #240 made
# @newtype getters opt-in (the committed tree still has the auto-generated get() methods).
CDDL_CODEGEN_REV="1ec516fe02960929cd3cadf3c2bad62360756648"

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
#   cip25 deliberately omits preserve-encodings/canonical-form (it never had them).
OVERRIDE=(--common-import-override=cml_core)
COMMON=(--preserve-encodings=true --canonical-form=true --json-serde-derives=true --json-schema-export=true "${OVERRIDE[@]}")

# want <crate>: true when no crate filter was given, or <crate> is one of the args.
ARGS=("$@")
want() {
  [ ${#ARGS[@]} -eq 0 ] && return 0
  local a; for a in "${ARGS[@]}"; do [ "$a" = "$1" ] && return 0; done
  return 1
}

# In-place regen is only reviewable against a clean tree; warn (don't block) if it's dirty.
if [ -n "$(git -C "$REPO_ROOT" status --porcelain -- chain multi-era cip25 cip36 2>/dev/null)" ]; then
  echo "WARNING: crate dirs have uncommitted changes — commit or stash first so the regen diff is clean." >&2
fi

# Crate              spec input                 args
#   chain      <- specs/conway        (Conway-era on-chain types; README.md historically said babbage)
#   multi-era  <- specs/multiera      (uses _CDDL_CODEGEN_EXTERN_DEPS_DIR_/cml_chain to reference chain)
#   multi-era byron module <- specs/multiera-byron  (writes the byron module into multi-era/rust;
#                                       a two-input crate — scrutinize this pass's diff first time)
#   cip36      <- specs/cip36.cddl
#   cip25      <- specs/cip25.cddl    (no preserve-encodings)
want chain     && gen chain     "$SPECS/conway"         "${COMMON[@]}"
want multi-era && gen multi-era "$SPECS/multiera"       "${COMMON[@]}"
want multi-era && gen multi-era "$SPECS/multiera-byron" "${COMMON[@]}"
want cip36     && gen cip36     "$SPECS/cip36.cddl"     "${COMMON[@]}"
want cip25     && gen cip25     "$SPECS/cip25.cddl"     --json-serde-derives=true --json-schema-export=true "${OVERRIDE[@]}"

cat <<'EOF'

Done — sources regenerated in place. Review with git:

  git status               # what changed, was added, or was clobbered
  git diff                 # inspect the regenerated output
  git checkout -- <file>   # revert a clobbered hand-written file (Cargo.toml, builders/, utils.rs)
  git checkout -p          # or pick hunks to keep/revert interactively

Then sanity-build:  cargo test -p cml-chain   (etc.)
EOF
