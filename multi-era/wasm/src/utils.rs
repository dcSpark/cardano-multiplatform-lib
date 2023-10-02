use wasm_bindgen::JsError;

use crate::MultiEraBlock;

impl MultiEraBlock {
    /**
     * Parses a block given the network block format with explicit era tag
     *
     * Some tools (e.g. Pallas/Oura) won't give you the block format from the binary spec directly,
     * but will instead have it wrapped in some network wrapper array containing the explicit era tag.
     * If your CBOR looks like `[uint, <actual block here>]`
     * (likely starting with `82` in hex e.g. `8201`, `8204`, `8207`, etc)
     * then you should use this function instead of the regular from_cbor_bytes().
     */
    pub fn from_explicit_network_cbor_bytes(bytes: &[u8]) -> Result<MultiEraBlock, JsError> {
        cml_multi_era::MultiEraBlock::from_explicit_network_cbor_bytes(bytes)
            .map(Into::into)
            .map_err(Into::into)
    }
}
