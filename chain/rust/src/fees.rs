use crate::plutus::utils::compute_total_ex_units;
use crate::plutus::ExUnitPrices;
use crate::transaction::Transaction;
use crate::Coin;
use cml_core::{serialization::Serialize, ArithmeticError};
use num::{rational::BigRational, CheckedAdd, CheckedMul};
use std::convert::TryFrom;

/// Careful: although the linear fee is the same for Byron & Shelley
/// The value of the parameters and how fees are computed is not the same
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LinearFee {
    /// minfee_a
    pub coefficient: Coin,
    /// minfee_b
    pub constant: Coin,
    /// min_fee_ref_script_cost_per_byte
    pub ref_script_cost_per_byte: Coin,
}

impl LinearFee {
    /**
     * * `coefficient` - minfee_a from protocol params
     * * `constant` - minfee_b from protocol params
     * * `ref_script_cost_per_bytes` - min_fee_ref_script_cost_per_byte from protocol params. New in Conway
     */
    pub fn new(coefficient: Coin, constant: Coin, ref_script_cost_per_byte: Coin) -> Self {
        Self {
            constant,
            coefficient,
            ref_script_cost_per_byte,
        }
    }
}

/**
 * Min fee for JUST the script, NOT including ref inputs
 */
pub fn min_script_fee(
    tx: &Transaction,
    ex_unit_prices: &ExUnitPrices,
) -> Result<Coin, ArithmeticError> {
    if let Some(redeemers) = &tx.witness_set.redeemers {
        let total_ex_units = compute_total_ex_units(&redeemers.clone().to_flat_format())?;
        let script_fee = ((BigRational::new(total_ex_units.mem.into(), 1u64.into())
            * BigRational::new(
                ex_unit_prices.mem_price.numerator.into(),
                ex_unit_prices.mem_price.denominator.into(),
            ))
            + (BigRational::new(total_ex_units.steps.into(), 1u64.into())
                * BigRational::new(
                    ex_unit_prices.step_price.numerator.into(),
                    ex_unit_prices.step_price.denominator.into(),
                )))
        .ceil()
        .to_integer();
        u64::try_from(script_fee).map_err(|_| ArithmeticError::IntegerOverflow)
    } else {
        Ok(0)
    }
}

/**
 * Calculates the cost of all ref scripts
 * * `total_ref_script_size` - Total size (original, not hashes) of all ref scripts. Duplicate scripts are counted as many times as they occur
 */
pub fn min_ref_script_fee(
    linear_fee: &LinearFee,
    total_ref_script_size: u64,
) -> Result<Coin, ArithmeticError> {
    // based on:
    // https://github.com/IntersectMBO/cardano-ledger/blob/7e65f0365eef647b9415e3fe9b3c35561761a3d5/eras/conway/impl/src/Cardano/Ledger/Conway/Tx.hs#L84
    // https://github.com/IntersectMBO/cardano-ledger/blob/a34f878c56763d138d2203d8ba84b3af64d94fce/eras/conway/impl/src/Cardano/Ledger/Conway/UTxO.hs#L152

    if total_ref_script_size > 0 {
        let multiplier = BigRational::new(12u64.into(), 10u64.into());
        let size_increment = 25_600u64; // 25KiB
        let mut fee: BigRational = BigRational::from_integer(0.into());
        let mut fee_tier: BigRational =
            BigRational::from_integer(linear_fee.ref_script_cost_per_byte.into());
        let mut ref_scripts_size_left = total_ref_script_size;

        loop {
            fee = BigRational::from_integer(
                std::cmp::min(size_increment, ref_scripts_size_left).into(),
            )
            .checked_mul(&fee_tier)
            .and_then(|x| x.checked_add(&fee))
            .ok_or(ArithmeticError::IntegerOverflow)?;
            if ref_scripts_size_left <= size_increment {
                break;
            }
            ref_scripts_size_left -= size_increment;
            fee_tier = fee_tier
                .checked_mul(&multiplier)
                .ok_or(ArithmeticError::IntegerOverflow)?;
        }
        u64::try_from(fee.ceil().to_integer()).map_err(|_e| ArithmeticError::IntegerOverflow)
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
    ex_unit_prices: &ExUnitPrices,
    total_ref_script_size: u64,
) -> Result<Coin, ArithmeticError> {
    // TODO: the fee should be 0 if all inputs are genesis redeem addresses
    let base_fee = min_no_script_fee(tx, linear_fee)?;
    let script_fee = min_script_fee(tx, ex_unit_prices)?;
    let ref_scripts_fee = min_ref_script_fee(linear_fee, total_ref_script_size)?;
    base_fee
        .checked_add(script_fee)
        .and_then(|x| x.checked_add(ref_scripts_fee))
        .ok_or(ArithmeticError::IntegerOverflow)
}
