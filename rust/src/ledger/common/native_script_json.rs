#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use hex::FromHex;
use crate::{NativeScript, error::JsError, crypto::Bip32PublicKey, ScriptPubkey, NativeScripts, ScriptAll, ScriptAny, ScriptNOfK, TimelockStart, TimelockExpiry};


/// Used to choose the schema for a script JSON string
#[wasm_bindgen]
pub enum ScriptSchema {
    Wallet,
    Node,
}

/// Receives a script JSON string
/// and returns a NativeScript.
/// Cardano Wallet and Node styles are supported.
///
/// * wallet: https://github.com/input-output-hk/cardano-wallet/blob/master/specifications/api/swagger.yaml
/// * node: https://github.com/input-output-hk/cardano-node/blob/master/doc/reference/simple-scripts.md
///
/// self_xpub is expected to be a Bip32PublicKey as hex-encoded bytes
#[wasm_bindgen]
pub fn encode_json_str_to_native_script(
    json: &str,
    self_xpub: &str,
    schema: ScriptSchema,
) -> Result<NativeScript, JsError> {
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| JsError::from_str(&e.to_string()))?;

    let native_script = match schema {
        ScriptSchema::Wallet => encode_wallet_value_to_native_script(value, self_xpub)?,
        ScriptSchema::Node => todo!(),
    };

    Ok(native_script)
}

fn encode_wallet_value_to_native_script(value: serde_json::Value, self_xpub: &str) -> Result<NativeScript, JsError> {
    match value {
        serde_json::Value::Object(map)
            if map.contains_key("cosigners") && map.contains_key("template") =>
        {
            let mut cosigners = HashMap::new();

            if let serde_json::Value::Object(cosigner_map) = map.get("cosigners").unwrap() {
                for (key, value) in cosigner_map.iter() {
                    if let serde_json::Value::String(xpub) = value {
                        if xpub == "self" {
                            cosigners.insert(key.to_owned(), self_xpub.to_owned());
                        } else {
                            cosigners.insert(key.to_owned(), xpub.to_owned());
                        }
                    } else {
                        return Err(JsError::from_str("cosigner value must be a string"));
                    }
                }
            } else {
                return Err(JsError::from_str("cosigners must be a map"));
            }

            let template = map.get("template").unwrap();

            let template_native_script = encode_template_to_native_script(template, &cosigners)?;

            Ok(template_native_script)
        }
        _ => Err(JsError::from_str(
            "top level must be an object. cosigners and template keys are required",
        )),
    }
}

fn encode_template_to_native_script(
    template: &serde_json::Value,
    cosigners: &HashMap<String, String>,
) -> Result<NativeScript, JsError> {
    match template {
        serde_json::Value::String(cosigner) => {
            if let Some(xpub) = cosigners.get(cosigner) {
                let bytes =
                    Vec::from_hex(xpub).map_err(|e| JsError::from_str(&e.to_string()))?;

                let public_key = Bip32PublicKey::from_bytes(&bytes)?;

                Ok(NativeScript::new_script_pubkey(&ScriptPubkey::new(
                    &public_key.to_raw_key().hash(),
                )))
            } else {
                Err(JsError::from_str(&format!("cosigner {} not found", cosigner)))
            }
        }
        serde_json::Value::Object(map) if map.contains_key("all") => {
            let mut all = NativeScripts::new();

            if let serde_json::Value::Array(array) = map.get("all").unwrap() {
                for val in array {
                    all.add(&encode_template_to_native_script(val, cosigners)?);
                }
            } else {
                return Err(JsError::from_str("all must be an array"));
            }

            Ok(NativeScript::new_script_all(&ScriptAll::new(&all)))
        }
        serde_json::Value::Object(map) if map.contains_key("any") => {
            let mut any = NativeScripts::new();

            if let serde_json::Value::Array(array) = map.get("any").unwrap() {
                for val in array {
                    any.add(&encode_template_to_native_script(val, cosigners)?);
                }
            } else {
                return Err(JsError::from_str("any must be an array"));
            }

            Ok(NativeScript::new_script_any(&ScriptAny::new(&any)))
        }
        serde_json::Value::Object(map) if map.contains_key("some") => {
            if let serde_json::Value::Object(some) = map.get("some").unwrap() {
                if some.contains_key("at_least") && some.contains_key("from") {
                    let n = if let serde_json::Value::Number(at_least) =
                        some.get("at_least").unwrap()
                    {
                        if let Some(n) = at_least.as_u64() {
                            n as u32
                        } else {
                            return Err(JsError::from_str("at_least must be an integer"));
                        }
                    } else {
                        return Err(JsError::from_str("at_least must be an integer"));
                    };

                    let mut from_scripts = NativeScripts::new();

                    if let serde_json::Value::Array(array) = some.get("from").unwrap() {
                        for val in array {
                            from_scripts
                                .add(&encode_template_to_native_script(val, cosigners)?);
                        }
                    } else {
                        return Err(JsError::from_str("from must be an array"));
                    }

                    Ok(NativeScript::new_script_n_of_k(&ScriptNOfK::new(
                        n,
                        &from_scripts,
                    )))
                } else {
                    Err(JsError::from_str("some must contain at_least and from"))
                }
            } else {
                Err(JsError::from_str("some must be an object"))
            }
        }
        serde_json::Value::Object(map) if map.contains_key("active_from") => {
            if let serde_json::Value::Number(active_from) = map.get("active_from").unwrap() {
                if let Some(slot) = active_from.as_u64() {
                    let time_lock_start = TimelockStart::new(&slot.into());

                    Ok(NativeScript::new_timelock_start(&time_lock_start))
                } else {
                    Err(JsError::from_str(
                        "active_from slot must be an integer greater than or equal to 0",
                    ))
                }
            } else {
                Err(JsError::from_str("active_from slot must be a number"))
            }
        }
        serde_json::Value::Object(map) if map.contains_key("active_until") => {
            if let serde_json::Value::Number(active_until) = map.get("active_until").unwrap() {
                if let Some(slot) = active_until.as_u64() {
                    let time_lock_expiry = TimelockExpiry::new(&slot.into());

                    Ok(NativeScript::new_timelock_expiry(&time_lock_expiry))
                } else {
                    Err(JsError::from_str(
                        "active_until slot must be an integer greater than or equal to 0",
                    ))
                }
            } else {
                Err(JsError::from_str("active_until slot must be a number"))
            }
        }
        _ => Err(JsError::from_str("invalid template format")),
    }
}
