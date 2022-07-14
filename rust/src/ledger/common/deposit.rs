#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use wasm_bindgen::prelude::*;

use crate::{Withdrawals, Certificates, CertificateEnum, TransactionBody, error::JsError};

use super::value::{BigNum, Value, to_bignum, Coin};

// wasm-bindgen can't accept Option without clearing memory, so we avoid exposing this in WASM
pub fn internal_get_implicit_input(
    withdrawals: &Option<Withdrawals>,
    certs: &Option<Certificates>,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum, // protocol parameter
) -> Result<Value, JsError> {
    let withdrawal_sum = match &withdrawals {
        None => to_bignum(0),
        Some(x) => x.0
            .values()
            .try_fold(
                to_bignum(0),
                |acc, withdrawal_amt| acc.checked_add(withdrawal_amt)
            )?,
    };
    let certificate_refund = match &certs {
        None => to_bignum(0),
        Some(certs) => certs.0
            .iter()
            .try_fold(
                to_bignum(0),
                |acc, cert| match &cert.0 {
                    CertificateEnum::PoolRetirement(_cert) => acc.checked_add(pool_deposit),
                    CertificateEnum::StakeDeregistration(_cert) => acc.checked_add(key_deposit),
                    _ => Ok(acc),
                }
            )?
    };

    Ok(Value::new(&withdrawal_sum.checked_add(&certificate_refund)?))
}
pub fn internal_get_deposit(
    certs: &Option<Certificates>,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum, // protocol parameter
) -> Result<Coin, JsError> {
    let certificate_refund = match &certs {
        None => to_bignum(0),
        Some(certs) => certs.0
            .iter()
            .try_fold(
                to_bignum(0),
                |acc, cert| match &cert.0 {
                    CertificateEnum::PoolRegistration(_cert) => acc.checked_add(pool_deposit),
                    CertificateEnum::StakeRegistration(_cert) => acc.checked_add(key_deposit),
                    _ => Ok(acc),
                }
            )?
    };
    Ok(certificate_refund)
}


#[wasm_bindgen]
pub fn get_implicit_input(
    txbody: &TransactionBody,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum, // protocol parameter
) -> Result<Value, JsError> {
    internal_get_implicit_input(
        &txbody.withdrawals,
        &txbody.certs,
        pool_deposit,
        key_deposit,
    )
}

#[wasm_bindgen]
pub fn get_deposit(
    txbody: &TransactionBody,
    pool_deposit: &BigNum, // // protocol parameter
    key_deposit: &BigNum, // protocol parameter
) -> Result<Coin, JsError> {
    internal_get_deposit(
        &txbody.certs,
        pool_deposit,
        key_deposit,
    )
}
