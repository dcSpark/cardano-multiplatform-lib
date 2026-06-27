# Golden block vectors

Real Cardano block CBOR used by `tests/golden_blocks.rs` to assert byte-identical
round-trips (decode with `MultiEraBlock::from_explicit_network_cbor_bytes` → re-encode with
`to_explicit_network_cbor_bytes` → must equal the input). This locks CML's decoding/encoding
fidelity against real on-chain data — the safety net for spec/codegen changes (e.g. a hard fork).

All files are in the network "explicit era tag" envelope `[era_tag, <block>]` (hex starts with
`82..`). Files are either raw CBOR or ASCII-hex text; the test auto-detects.

## Sources

| Dir               | Source                                                                 |
|-------------------|------------------------------------------------------------------------|
| `mainnet_blocks/` | **Dolos** `crates/cardano/test_data/mainnet/update_proposal_blocks/`. Real mainnet blocks, named by block hash. repo https://github.com/txpipe/dolos commit `ea7960a1c2e56c523fec7c4bab75f390ee443514` |
| `pallas/`         | **Pallas** `test_data/*.block` (its per-era vector set). repo https://github.com/txpipe/pallas rev `a7b5a86` (the rev Dolos pins) |

To refresh: re-copy from those paths at a newer commit and update the refs above. The `u5c*`
(utxorpc) fixtures in Pallas's `test_data/` are intentionally not vendored.

## Era coverage

All eras: **Byron EBB (0)** and **Byron main (1)**, **Shelley (2)**, **Allegra (3)**, **Mary (4)**,
**Alonzo (5)**, **Babbage (6)**, **Conway (7)** — from Pallas's per-era vectors plus the real
mainnet Byron/Mary/Alonzo/Babbage blocks.

## Known divergence

`pallas/conway8.block` is in `KNOWN_FAILURES` in `golden_blocks.rs`: one of its certificates
carries a 56-byte value where a 28-byte hash is required (stake-delegation tag `2`, pool-key-hash
position). This is **not a CML gap** — Pallas itself (@ `a7b5a86`) also fails to decode this block
(`InvalidCbor: Invalid hash size`), and Pallas's own decode test (`test_iteration`) does not cover
conway8. Its slot (~96.8M) predates mainnet Conway, so it's a malformed/early-Conway (SanchoNet)
artifact. CML correctly rejects it; the entry stays as a guard against ever accepting such a block.

## Adding more

1. Drop a `[era, block]`-enveloped CBOR (or hex) file into `mainnet_blocks/` or `pallas/`.
   - From Pallas: `test_data/*.block`.
   - From Dolos: `crates/cardano/test_data/…` (Dolos can also fetch specific blocks from a remote node).
2. Note its origin above if it's a new source.
3. Re-run `cargo test -p cml-multi-era --test golden_blocks`.
