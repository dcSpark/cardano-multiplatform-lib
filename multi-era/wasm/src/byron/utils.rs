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
