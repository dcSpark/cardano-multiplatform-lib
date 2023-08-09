// use cml_core_wasm::impl_wasm_list;

// // these weren't generated due to it being _CDDL_CODEGEN_EXTERN_TYPE_
// impl_wasm_list!(cml_chain::byron::AddressId, cml_chain_wasm::byron::AddressId, AddressIdList);
// impl_wasm_list!(, VssPubKeyList);

use wasm_bindgen::prelude::wasm_bindgen;

use cml_core_wasm::impl_wasm_conversions;
use cml_crypto_wasm::impl_hash_type_ext;

impl_hash_type_ext!(cml_multi_era::byron::Blake2b224, Blake2b224);
impl_hash_type_ext!(cml_multi_era::byron::Blake2b256, Blake2b256);

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct ByronAny(cml_multi_era::byron::ByronAny);

// more methods on ByronAny to inspect it aren't offered as we don't encounter anything
// useful on-chain for this. It's either not present or is an empty array

impl_wasm_conversions!(cml_multi_era::byron::ByronAny, ByronAny);

/// We use this instead of cml_core::impl_wasm_cbor_json_api due to not implementing
/// cml's Serialize. Byron just does cbor_event's due to not supporting preserve-encodings=true
/// All other methods are identical to cml_core's macro though.
#[macro_export]
macro_rules! impl_wasm_cbor_json_api_byron {
    ($wasm_name:ident) => {
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl $wasm_name {
            pub fn to_cbor_bytes(&self) -> Vec<u8> {
                cml_core::serialization::ToBytes::to_bytes(&self.0)
            }

            pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<$wasm_name, JsValue> {
                cml_chain::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
                    .map(Self)
                    .map_err(|e| {
                        JsValue::from_str(&format!(
                            concat!(stringify!($wasm_name), "::from_cbor_bytes: {}"),
                            e
                        ))
                    })
            }

            pub fn to_json(&self) -> Result<String, JsValue> {
                serde_json::to_string_pretty(&self.0).map_err(|e| {
                    JsValue::from_str(&format!(
                        concat!(stringify!($wasm_name), "::to_json: {}"),
                        e
                    ))
                })
            }

            pub fn to_js_value(&self) -> Result<JsValue, JsValue> {
                serde_wasm_bindgen::to_value(&self.0).map_err(|e| {
                    JsValue::from_str(&format!(
                        concat!(stringify!($wasm_name), "::to_js_value: {}"),
                        e
                    ))
                })
            }

            pub fn from_json(json: &str) -> Result<$wasm_name, JsValue> {
                serde_json::from_str(json).map(Self).map_err(|e| {
                    JsValue::from_str(&format!(
                        concat!(stringify!($wasm_name), "::from_json: {}"),
                        e
                    ))
                })
            }
        }
    };
}
