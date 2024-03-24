# Multi-Era

This crate contains all the on-chain types for previous eras (Byron, Shelley, Alonzo, Babbage, etc). There are also wrappers around this era if you need era-agnostic types e.g. parsing all blocks from genesis. The wrappers support the current era as well.

## Parsing blocks across eras

`MultiEraBlock` can be used for this. Take care about the format you are giving it. Some tools (e.g. Pallas/Oura) won't give you the block format from the binary spec directly, but will instead have it wrapped in some network wrapper array containing the explicit era tag. If your CBOR looks like `[uint, <actual block here>]` (likely starting with `82` in hex e.g. `8201`, `8204`, `8207`, etc) then you should use `MultiEraBlock.from_explicit_network_cbor_bytes()` instead of `MultiEraBlock.from_cbor_bytes()`.