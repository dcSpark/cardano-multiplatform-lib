/// This is not for concrete implementations for on-chain things (see chain crate for that)
/// but instead for use there and in other CDDL-defined crates that have bytes that represent
/// crypto primitives.

// unfortunately concat_idents isn't in stable so we just take in two params
// as to not cause name collisions in the final WASM build in the chain wasm bindings
// TODO: embed the type into comments (looks like it's likely possible using another macro)
//       so that we have less generic documentation
#[macro_export]
macro_rules! impl_chain_crypto {
  ($name:ident, $primitive:ident, $wasm_mod:ident) => {
      /// On-chain cryptographic primitive
      #[wasm_bindgen]
      #[derive(Debug, Clone)]
      pub struct $name(core_crypto::chain::ChainCrypto<core_crypto::$primitive>);

      #[wasm_bindgen]
      impl $name {
          /// Get the underlying cryptographic primitive represented here
          pub fn primitive(&self) -> $wasm_mod::$primitive {
              self.0.primitive.clone().into()
          }

          /// Make a default-encoded on-chain cryptographic type based on the primitive
          pub fn new(primitive: &$wasm_mod::$primitive) -> Self {
              Self(core_crypto::chain::ChainCrypto::from(primitive.as_ref().clone()))
          }

          pub fn to_original_cbor_bytes(&self) -> Vec<u8> {
              self.0.to_original_cbor_bytes()
          }

          pub fn to_canonical_cbor_bytes(&self) -> Vec<u8> {
              self.0.to_canonical_cbor_bytes()
          }

          pub fn from_raw_bytes(bytes: &[u8]) -> Result<$name, JsError> {
              core_crypto::chain::ChainCrypto::<core_crypto::$primitive>::from_raw_bytes(bytes).map(Self).map_err(Into::into)
          }

          pub fn to_json(&self) -> Result<String, JsValue> {
              serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
          }
      
          pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
              JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
          }
      
          pub fn from_json(json: &str) -> Result<$name, JsValue> {
              serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
          }
      }

      // chain-crypto (rust) <-> chain-crypto (wasm)
      impl From<core_crypto::chain::ChainCrypto<core_crypto::$primitive>> for $name {
          fn from(inner: core_crypto::chain::ChainCrypto<core_crypto::$primitive>) -> Self {
              Self(inner)
          }
      }

      impl From<$name> for core_crypto::chain::ChainCrypto<core_crypto::$primitive> {
          fn from(wrapper: $name) -> core_crypto::chain::ChainCrypto<core_crypto::$primitive> {
              wrapper.0
          }
      }

      // crypto (wasm) <-> chain-crypto (wasm)
      impl From<$wasm_mod::$primitive> for $name {
          fn from(primitive: $wasm_mod::$primitive) -> Self {
              Self(core_crypto::$primitive::from(primitive).into())
          }
      }

      impl From<$name> for $wasm_mod::$primitive {
          fn from(wrapper: $name) -> $wasm_mod::$primitive {
              wrapper.0.primitive.into()
          }
      }
  };
}