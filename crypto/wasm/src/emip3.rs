use wasm_bindgen::prelude::{JsError, wasm_bindgen};

/// Encrypt using Emip3: https://github.com/Emurgo/EmIPs/blob/master/specs/emip-003.md
#[wasm_bindgen]
pub fn emip3_encrypt_with_password(
    password: &str,
    salt: &str,
    nonce: &str,
    data: &str,
) -> Result<String, JsError> {
    cml_crypto::emip3::emip3_encrypt_with_password(password, salt, nonce, data).map_err(Into::into)
}

/// Decrypt using Emip3: https://github.com/Emurgo/EmIPs/blob/master/specs/emip-003.md
#[wasm_bindgen]
pub fn emip3_decrypt_with_password(
    password: &str,
    data: &str,
) -> Result<String, JsError> {
    cml_crypto::emip3::emip3_decrypt_with_password(password, data).map_err(Into::into)
}