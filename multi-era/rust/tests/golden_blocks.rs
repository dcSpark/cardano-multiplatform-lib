//! Golden round-trip tests over real Cardano block CBOR.
//!
//! Each vector is a network "explicit era tag" block (`[era_tag, <block>]`). We decode it,
//! re-encode it, and assert the bytes are identical — exercising CML's preserve-encodings
//! fidelity against real on-chain data. See tests/golden_vectors/PROVENANCE.md for sources
//! and how to add more.

use cml_multi_era::MultiEraBlock;
use std::fs;
use std::path::{Path, PathBuf};

/// Vectors CML does not currently round-trip. Tracked here rather than dropped: the test
/// asserts each still fails, so if CML starts handling one it flips to a failure telling you
/// to remove it. Keep the reason precise.
const KNOWN_FAILURES: &[&str] = &[
    // conway8.block (Pallas): a certificate carries a 56-byte value where a 28-byte hash is
    // required (stake-delegation tag `2`, pool-key-hash position). NOT a CML gap — Pallas itself
    // (@ a7b5a86) also rejects this block with `InvalidCbor: Invalid hash size`, and its own
    // decode test doesn't cover conway8. The slot (~96.8M) predates mainnet Conway, so it's a
    // malformed/early-Conway (SanchoNet) artifact. CML rejecting it is correct; this entry stays
    // as a guard against CML ever accepting such a block.
    "conway8.block",
];

/// Vectors are stored as raw CBOR or as ASCII-hex text. Hex files are pure hex + whitespace;
/// raw CBOR blocks begin with 0x82 (not an ASCII hex digit), so this cleanly distinguishes them.
fn load(path: &Path) -> Vec<u8> {
    let raw = fs::read(path).unwrap();
    if !raw.is_empty()
        && raw
            .iter()
            .all(|b| b.is_ascii_hexdigit() || b.is_ascii_whitespace())
    {
        hex::decode(std::str::from_utf8(&raw).unwrap().trim()).unwrap()
    } else {
        raw
    }
}

fn vectors() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden_vectors");
    let mut out = Vec::new();
    for sub in ["mainnet_blocks", "pallas"] {
        for entry in fs::read_dir(root.join(sub)).unwrap() {
            let p = entry.unwrap().path();
            if p.is_file() && p.extension().map(|e| e != "md").unwrap_or(true) {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// Decode the `[era, block]` envelope and re-encode it; Ok(()) iff byte-identical.
fn round_trip(bytes: &[u8]) -> Result<(), String> {
    let block = MultiEraBlock::from_explicit_network_cbor_bytes(bytes)
        .map_err(|e| format!("decode failed: {e}"))?;
    let reencoded = block.to_explicit_network_cbor_bytes();
    if reencoded == bytes {
        Ok(())
    } else {
        Err(format!(
            "round-trip mismatch ({} in, {} out)",
            bytes.len(),
            reencoded.len()
        ))
    }
}

#[test]
fn golden_blocks_round_trip() {
    let vectors = vectors();
    assert!(!vectors.is_empty(), "no golden vectors found");

    let mut failures = Vec::new(); // a vector that should round-trip but didn't
    let mut now_passing = Vec::new(); // a KNOWN_FAILURES entry that now round-trips
    for path in &vectors {
        let fname = path.file_name().unwrap().to_str().unwrap();
        let known = KNOWN_FAILURES.contains(&fname);
        let name = path
            .strip_prefix(env!("CARGO_MANIFEST_DIR"))
            .unwrap_or(path)
            .display();
        match (round_trip(&load(path)), known) {
            (Ok(()), false) => {}
            (Ok(()), true) => now_passing.push(name.to_string()),
            (Err(_), true) => {}
            (Err(msg), false) => failures.push(format!("{name}: {msg}")),
        }
    }

    assert!(
        now_passing.is_empty(),
        "{} known-failing vector(s) now round-trip — remove them from KNOWN_FAILURES:\n{}",
        now_passing.len(),
        now_passing.join("\n")
    );
    assert!(
        failures.is_empty(),
        "{}/{} golden vectors failed ({} known-failing excluded):\n{}",
        failures.len(),
        vectors.len(),
        KNOWN_FAILURES.len(),
        failures.join("\n")
    );
}
