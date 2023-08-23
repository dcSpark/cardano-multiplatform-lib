use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Crc32(cml_chain::byron::Crc32);

#[wasm_bindgen]
impl Crc32 {
    /// initialise a new CRC32 state
    #[inline]
    pub fn new() -> Self {
        Self(cml_chain::byron::Crc32::new())
    }

    /// update the CRC32 with the given bytes.
    ///
    /// beware that the order in which you update the Crc32
    /// matter
    pub fn update(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    /// finalize the CRC32, recovering the computed value
    pub fn finalize(&self) -> u32 {
        self.0.finalize()
    }
}

impl From<cml_chain::byron::Crc32> for Crc32 {
    fn from(native: cml_chain::byron::Crc32) -> Self {
        Self(native)
    }
}

impl From<Crc32> for cml_chain::byron::Crc32 {
    fn from(wasm: Crc32) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::byron::Crc32> for Crc32 {
    fn as_ref(&self) -> &cml_chain::byron::Crc32 {
        &self.0
    }
}
