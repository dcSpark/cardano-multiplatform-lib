use blockfrost::{BlockfrostAPI, BlockfrostError};
use cml_chain::{
    builders::tx_builder::{TransactionBuilderConfigBuilder, TxBuilderConfigField},
    fees::LinearFee,
    plutus::ExUnitPrices,
    Coin, SubCoin,
};
use std::str::FromStr;

use crate::{parse_cost_models, BlockfrostParamsParseError};

#[derive(Debug, thiserror::Error)]
pub enum BlockfrostTxBuilderConfigError {
    #[error("Parsing: {0}")]
    Parsing(#[from] BlockfrostParamsParseError),
    #[error("Blockfrost: {0}")]
    Blockfrost(#[from] BlockfrostError),
}

/**
 * Completely automated config creation via supplied blockfrost API.
 * Both calls blockfrost via the passed API to get enough information
 * and also parses the information to make a TransactionBuilderConfigBuilder
 * with all necessary protocol parameter information set
 */
pub async fn make_tx_builder_cfg(
    api: &BlockfrostAPI,
) -> Result<TransactionBuilderConfigBuilder, BlockfrostTxBuilderConfigError> {
    let params = api.epochs_latest_parameters().await?;
    let coins_per_utxo_byte = params
        .coins_per_utxo_word
        .ok_or(BlockfrostParamsParseError::MissingField(
            TxBuilderConfigField::CoinsPerUtxoBytes,
        ))
        .and_then(|c| {
            Coin::from_str(&c).map_err(|_| {
                BlockfrostParamsParseError::IncorrectFormat(TxBuilderConfigField::CoinsPerUtxoBytes)
            })
        })?;
    let pool_deposit = Coin::from_str(&params.pool_deposit).map_err(|_| {
        BlockfrostParamsParseError::IncorrectFormat(TxBuilderConfigField::PoolDeposit)
    })?;
    let key_deposit = Coin::from_str(&params.key_deposit).map_err(|_| {
        BlockfrostParamsParseError::IncorrectFormat(TxBuilderConfigField::KeyDeposit)
    })?;
    let max_value_size = params
        .max_val_size
        .ok_or(BlockfrostParamsParseError::MissingField(
            TxBuilderConfigField::MaxValueSize,
        ))
        .and_then(|c| {
            u32::from_str(&c).map_err(|_| {
                BlockfrostParamsParseError::IncorrectFormat(TxBuilderConfigField::MaxValueSize)
            })
        })?;
    let ex_unit_prices = match (params.price_mem, params.price_step) {
        (Some(price_mem), Some(price_step)) => Ok(ExUnitPrices::new(
            SubCoin::from_base10_f32(price_mem),
            SubCoin::from_base10_f32(price_step),
        )),
        _ => Err(BlockfrostParamsParseError::MissingField(
            TxBuilderConfigField::ExUnitPrices,
        )),
    }?;
    let fee_algo = LinearFee::new(params.min_fee_a as u64, params.min_fee_b as u64);
    let max_tx_size = params.max_tx_size as u32;
    let collateral_percentage =
        params
            .collateral_percent
            .ok_or(BlockfrostParamsParseError::MissingField(
                TxBuilderConfigField::CollateralPercentage,
            ))? as u32;
    let max_collateral_inputs =
        params
            .max_collateral_inputs
            .ok_or(BlockfrostParamsParseError::MissingField(
                TxBuilderConfigField::MaxCollateralInputs,
            ))? as u32;
    let mut config_builder = TransactionBuilderConfigBuilder::new()
        .fee_algo(fee_algo)
        .coins_per_utxo_byte(coins_per_utxo_byte)
        .pool_deposit(pool_deposit)
        .key_deposit(key_deposit)
        .max_value_size(max_value_size)
        .max_tx_size(max_tx_size)
        .ex_unit_prices(ex_unit_prices)
        .collateral_percentage(collateral_percentage)
        .max_collateral_inputs(max_collateral_inputs);
    if let Some(cost_models) = params.cost_models {
        config_builder = config_builder.cost_models(parse_cost_models(&cost_models)?);
    }
    Ok(config_builder)
}
