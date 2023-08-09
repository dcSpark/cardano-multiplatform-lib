use cml_core::ArithmeticError;

use crate::{Withdrawals, certs::Certificate, Coin, Value, transaction::TransactionBody};


pub fn internal_get_implicit_input(
    withdrawals: Option<&Withdrawals>,
    certs: Option<&[Certificate]>,
    pool_deposit: Coin, // // protocol parameter
    key_deposit: Coin, // protocol parameter
) -> Result<Value, ArithmeticError> {
    let withdrawal_sum = match withdrawals {
        None => 0,
        Some(w) => w
            .values()
            .try_fold(
                0u64,
                |acc, withdrawal_amt| acc.checked_add(*withdrawal_amt)
            )
            .ok_or(ArithmeticError::IntegerOverflow)?,
    };
    let certificate_refund = match certs {
        None => 0,
        Some(certs) => certs
            .iter()
            .try_fold(
                0u64,
                |acc, cert| match cert {
                    Certificate::PoolRetirement(_cert) => acc.checked_add(pool_deposit),
                    Certificate::StakeDeregistration(_cert) => acc.checked_add(key_deposit),
                    _ => Some(acc),
                }
            )
            .ok_or(ArithmeticError::IntegerOverflow)?
    };

    withdrawal_sum
        .checked_add(certificate_refund)
        .ok_or(ArithmeticError::IntegerOverflow)
        .map(Value::from)
}

pub fn internal_get_deposit(
    certs: Option<&[Certificate]>,
    pool_deposit: Coin, // // protocol parameter
    key_deposit: Coin, // protocol parameter
) -> Result<Coin, ArithmeticError> {
    let certificate_refund = match certs {
        None => 0,
        Some(certs) => certs
            .iter()
            .try_fold(
                0u64,
                |acc, cert| match cert {
                    Certificate::PoolRegistration(_cert) => acc.checked_add(pool_deposit),
                    Certificate::StakeRegistration(_cert) => acc.checked_add(key_deposit),
                    _ => Some(acc),
                }
            )
            .ok_or(ArithmeticError::IntegerOverflow)?
    };
    Ok(certificate_refund)
}


pub fn get_implicit_input(
    txbody: &TransactionBody,
    pool_deposit: Coin, // // protocol parameter
    key_deposit: Coin, // protocol parameter
) -> Result<Value, ArithmeticError> {
    internal_get_implicit_input(
        txbody.withdrawals.as_ref(),
        txbody.certs.as_deref(),
        pool_deposit,
        key_deposit,
    )
}

pub fn get_deposit(
    txbody: &TransactionBody,
    pool_deposit: Coin, // // protocol parameter
    key_deposit: Coin, // protocol parameter
) -> Result<Coin, ArithmeticError> {
    internal_get_deposit(
        txbody.certs.as_deref(),
        pool_deposit,
        key_deposit,
    )
}
