use cml_core::{ArithmeticError, serialization::Serialize};
use fraction::{Fraction, ToPrimitive};
use crate::Coin;
use crate::plutus::ExUnitPrices;
use crate::plutus::utils::compute_total_ex_units;
use crate::transaction::Transaction;

/// Careful: although the linear fee is the same for Byron & Shelley
/// The value of the parameters and how fees are computed is not the same
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LinearFee {
    pub constant: Coin,
    pub coefficient: Coin,
}

impl LinearFee {
    pub fn new(coefficient: Coin, constant: Coin) -> Self {
        Self {
            constant,
            coefficient,
        }
    }
}

pub fn min_script_fee(tx: &Transaction, ex_unit_prices: &ExUnitPrices) -> Result<Coin, ArithmeticError> {
    if let Some(redeemers) = &tx.witness_set.redeemers {
        let total_ex_units = compute_total_ex_units(redeemers)?;
        let script_fee = (
            (
                Fraction::new(total_ex_units.mem, 1u64)
                * Fraction::new(
                    ex_unit_prices.mem_price.numerator,
                    ex_unit_prices.mem_price.denominator
                )
            )
            +
            (
                Fraction::new(total_ex_units.steps, 1u64)
                * Fraction::new(
                    ex_unit_prices.step_price.numerator,
                    ex_unit_prices.step_price.denominator,
                )
            )
        ).ceil().to_u64().unwrap();
        Ok(script_fee)
    } else {
        Ok(0)
    }
}

pub fn min_no_script_fee(
    tx: &Transaction,
    linear_fee: &LinearFee,
) -> Result<Coin, ArithmeticError> {
    (tx.to_cbor_bytes().len() as u64)
        .checked_mul(linear_fee.coefficient)
        .and_then(|x| x.checked_add(linear_fee.constant))
        .ok_or(ArithmeticError::IntegerOverflow)
}

pub fn min_fee(
    tx: &Transaction,
    linear_fee: &LinearFee,
    ex_unit_prices: &ExUnitPrices
) -> Result<Coin, ArithmeticError> {
    // TODO: the fee should be 0 if all inputs are genesis redeem addresses
    min_no_script_fee(tx, linear_fee)?
        .checked_add(min_script_fee(tx, ex_unit_prices)?)
        .ok_or(ArithmeticError::IntegerOverflow)
}
