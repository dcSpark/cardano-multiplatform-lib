use super::certificate_builder::*;
use super::input_builder::InputBuilderResult;
use super::mint_builder::MintBuilderResult;
use super::output_builder::{OutputBuilderError, SingleOutputBuilderResult};
use super::redeemer_builder::RedeemerBuilderError;
use super::redeemer_builder::RedeemerSetBuilder;
use super::redeemer_builder::RedeemerWitnessKey;
use super::withdrawal_builder::WithdrawalBuilderResult;
use super::witness_builder::merge_fake_witness;
use super::witness_builder::PlutusScriptWitness;
use super::witness_builder::RequiredWitnessSet;
use super::witness_builder::TransactionWitnessSetBuilder;
use super::witness_builder::{InputAggregateWitnessData, WitnessBuilderError};
use crate::address::Address;
use crate::assets::MultiAsset;
use crate::assets::{AssetArithmeticError, Mint};
use crate::auxdata::AuxiliaryData;
use crate::builders::output_builder::TransactionOutputBuilder;
use crate::certs::Certificate;
use crate::crypto::hash::{calc_script_data_hash, hash_auxiliary_data, ScriptDataHashError};
use crate::crypto::{BootstrapWitness, Vkeywitness};
use crate::deposit::{internal_get_deposit, internal_get_implicit_input};
use crate::fees::LinearFee;
use crate::min_ada::min_ada_required;
use crate::plutus::PlutusData;
use crate::plutus::{CostModels, ExUnits, Language, Redeemer};
use crate::transaction::{
    DatumOption, ScriptRef, Transaction, TransactionBody, TransactionInput, TransactionOutput,
    TransactionWitnessSet,
};
use crate::{assets::AssetName, Coin, ExUnitPrices, NetworkId, PolicyId, Value, Withdrawals};
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::{ArithmeticError, Slot};
use cml_crypto::{Ed25519KeyHash, ScriptDataHash, ScriptHash, Serialize};
use fraction::Zero;
use rand::Rng;
use std::collections::{BTreeSet, HashMap};
use std::convert::TryInto;
use std::ops::DerefMut;

// for enums:
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug)]
pub struct TransactionUnspentOutput {
    pub input: TransactionInput,
    pub output: TransactionOutput,
}

impl TransactionUnspentOutput {
    pub fn new(input: TransactionInput, output: TransactionOutput) -> Self {
        Self { input, output }
    }
}

#[derive(Clone, Default, Debug)]
struct WitnessBuilders {
    pub witness_set_builder: TransactionWitnessSetBuilder,
    pub fake_required_witnesses: RequiredWitnessSet,
    pub redeemer_set_builder: RedeemerSetBuilder,
}
impl WitnessBuilders {
    fn merge_data(
        &self,
        include_fake: bool,
    ) -> Result<TransactionWitnessSetBuilder, WitnessBuilderError> {
        // add redeemers
        let redeemers = self.redeemer_set_builder.build(true)?;
        let mut witness_set_clone = self.witness_set_builder.clone();
        redeemers
            .into_iter()
            .for_each(|r| witness_set_clone.add_redeemer(r));

        if include_fake {
            merge_fake_witness(&mut witness_set_clone, &self.fake_required_witnesses);
            let own_requirements = witness_set_clone.required_wits.clone();
            merge_fake_witness(&mut witness_set_clone, &own_requirements);
        }

        Ok(witness_set_clone)
    }

    /// build with fake witnesses to estimate tx fee cost
    pub fn build_fake(&self) -> Result<TransactionWitnessSet, WitnessBuilderError> {
        self.merge_data(true).map(|wit_builder| wit_builder.build())
    }

    /// build without including fake witnesses used for fee estimation. Allows missing witnesses
    pub fn build_unchecked(&self) -> Result<TransactionWitnessSetBuilder, WitnessBuilderError> {
        self.merge_data(false)
    }
}

// tx_body must be the result of building from tx_builder
// constructs the rest of the Transaction using fake witness data of the correct length
// for use in calculating the size of the final Transaction
fn fake_full_tx(
    tx_builder: &TransactionBuilder,
    body: TransactionBody,
) -> Result<Transaction, TxBuilderError> {
    Ok(Transaction::new(
        body,
        tx_builder.witness_builders.build_fake()?,
        true,
        tx_builder.auxiliary_data.clone(),
    ))
}

#[derive(Debug, Copy, Clone)]
pub enum TxBuilderConfigField {
    FeeAlgo,
    PoolDeposit,
    KeyDeposit,
    MaxValueSize,
    MaxTxSize,
    CoinsPerUtxoBytes,
    ExUnitPrices,
    CollateralPercentage,
    MaxCollateralInputs,
}

#[derive(Debug, thiserror::Error)]
pub enum TxBuilderError {
    #[error("Witness build failed: {0}")]
    WitnessBuildFailed(#[from] WitnessBuilderError),
    #[error("Redeem build failed: {0}")]
    RedeemerBuildFailed(#[from] RedeemerBuilderError),
    #[error("Output build failed: {0}")]
    OutputBuildFailed(#[from] OutputBuilderError),
    #[error("Arithmetic: {0}")]
    Arithmetic(#[from] ArithmeticError),
    #[error("Asset arithmetic: {0}")]
    AssetArithmetic(#[from] AssetArithmeticError),
    #[error("Uninitialized field: {0:?}")]
    UninitializedField(TxBuilderConfigField),
    #[error(
        "Multiasset values not supported by RandomImprove. Please use RandomImproveMultiAsset"
    )]
    RandomImproveCantContainMultiasset,
    #[error("UTxO Balance Insufficient")]
    UTxOBalanceInsufficient,
    #[error("NFTs too large for change output")]
    NFTTooLargeForChange,
    #[error("Collateral can only be payment keys (scripts not allowed)")]
    CollateralMustBePayment,
    #[error("Max collateral input count {0} exceeded")]
    MaxCollateralInputExceeded(u32),
    #[error("Max value size of {0} exceeded. Found: {1}")]
    MaxValueSizeExceeded(u32, usize),
    #[error("Max transaction size of {0} exceeded. Found: {1}")]
    MaxTxSizeExceeded(u32, usize),
    #[error("Value {0} less tan the minimum UTXO value {1}")]
    ValueBelowMinUTXOValue(u64, u64),
    #[error("Fee not specified")]
    FeeNotSpecified,
    #[error("Reference Script hash {0} not found in reference script witness set {1:?}")]
    RefScriptNotFound(ScriptHash, BTreeSet<ScriptHash>),
    #[error("Not enough ADA leftover to include non-ADA assets in a change address")]
    InsufficientADAForAssets,
    #[error("Missing input or output (possibly some native asset)")]
    MissingInputOrOutput,
    #[error("Cannot use collateral return without also having collateral input")]
    CollateralReturnRequiresCollateralInput,
    #[error("ScriptDatumHash failed: {0}")]
    ScriptDatumHashFailed(#[from] ScriptDataHashError),
    #[error("Duplicate Mint Asset: {0:?}:{1:?}")]
    DuplicateMint(PolicyId, AssetName),
}

fn min_fee(tx_builder: &TransactionBuilder) -> Result<Coin, TxBuilderError> {
    let full_tx = fake_full_tx(tx_builder, tx_builder.build_body()?)?;
    // we can't know the of scripts yet as they can't be calculated until we build the tx
    crate::fees::min_no_script_fee(&full_tx, &tx_builder.config.fee_algo).map_err(Into::into)
}

fn min_fee_with_exunits(tx_builder: &TransactionBuilder) -> Result<Coin, TxBuilderError> {
    let full_tx = fake_full_tx(tx_builder, tx_builder.build_body()?)?;
    // we can't know the of scripts yet as they can't be calculated until we build the tx
    crate::fees::min_fee(
        &full_tx,
        &tx_builder.config.fee_algo,
        &tx_builder.config.ex_unit_prices,
    )
    .map_err(Into::into)
}

#[wasm_bindgen]
pub enum CoinSelectionStrategyCIP2 {
    /// Performs CIP2's Largest First ada-only selection. Will error if outputs contain non-ADA assets.
    LargestFirst,
    /// Performs CIP2's Random Improve ada-only selection. Will error if outputs contain non-ADA assets.
    RandomImprove,
    /// Same as LargestFirst, but before adding ADA, will insert by largest-first for each asset type.
    LargestFirstMultiAsset,
    /// Same as RandomImprove, but before adding ADA, will insert by random-improve for each asset type.
    RandomImproveMultiAsset,
}

#[derive(Clone, Debug)]
pub struct TransactionBuilderConfig {
    fee_algo: LinearFee,
    pool_deposit: u64,            // protocol parameter
    key_deposit: u64,             // protocol parameter
    max_value_size: u32,          // protocol parameter
    max_tx_size: u32,             // protocol parameter
    coins_per_utxo_byte: Coin,    // protocol parameter
    ex_unit_prices: ExUnitPrices, // protocol parameter
    cost_models: CostModels,      // protocol parameter
    _collateral_percentage: u32,  // protocol parameter
    max_collateral_inputs: u32,   // protocol parameter
    prefer_pure_change: bool,
}

#[derive(Clone, Debug, Default)]
pub struct TransactionBuilderConfigBuilder {
    fee_algo: Option<LinearFee>,
    pool_deposit: Option<u64>,            // protocol parameter
    key_deposit: Option<u64>,             // protocol parameter
    max_value_size: Option<u32>,          // protocol parameter
    max_tx_size: Option<u32>,             // protocol parameter
    coins_per_utxo_byte: Option<Coin>,    // protocol parameter
    ex_unit_prices: Option<ExUnitPrices>, // protocol parameter
    cost_models: Option<CostModels>,      // protocol parameter
    collateral_percentage: Option<u32>,   // protocol parameter
    max_collateral_inputs: Option<u32>,   // protocol parameter
    prefer_pure_change: bool,
}

impl TransactionBuilderConfigBuilder {
    pub fn new() -> Self {
        // we have to provide new to expose it to WASM builds
        Self::default()
    }

    pub fn fee_algo(mut self, fee_algo: LinearFee) -> Self {
        self.fee_algo = Some(fee_algo);
        self
    }

    pub fn coins_per_utxo_byte(mut self, coins_per_utxo_byte: Coin) -> Self {
        self.coins_per_utxo_byte = Some(coins_per_utxo_byte);
        self
    }

    pub fn pool_deposit(mut self, pool_deposit: u64) -> Self {
        self.pool_deposit = Some(pool_deposit);
        self
    }

    pub fn key_deposit(mut self, key_deposit: u64) -> Self {
        self.key_deposit = Some(key_deposit);
        self
    }

    pub fn max_value_size(mut self, max_value_size: u32) -> Self {
        self.max_value_size = Some(max_value_size);
        self
    }

    pub fn max_tx_size(mut self, max_tx_size: u32) -> Self {
        self.max_tx_size = Some(max_tx_size);
        self
    }

    pub fn prefer_pure_change(mut self, prefer_pure_change: bool) -> Self {
        self.prefer_pure_change = prefer_pure_change;
        self
    }

    pub fn ex_unit_prices(mut self, ex_unit_prices: ExUnitPrices) -> Self {
        self.ex_unit_prices = Some(ex_unit_prices);
        self
    }

    pub fn cost_models(mut self, cost_models: CostModels) -> Self {
        self.cost_models = Some(cost_models);
        self
    }

    pub fn collateral_percentage(mut self, collateral_percentage: u32) -> Self {
        self.collateral_percentage = Some(collateral_percentage);
        self
    }

    pub fn max_collateral_inputs(mut self, max_collateral_inputs: u32) -> Self {
        self.max_collateral_inputs = Some(max_collateral_inputs);
        self
    }

    pub fn build(self) -> Result<TransactionBuilderConfig, TxBuilderError> {
        Ok(TransactionBuilderConfig {
            fee_algo: self.fee_algo.ok_or(TxBuilderError::UninitializedField(
                TxBuilderConfigField::FeeAlgo,
            ))?,
            pool_deposit: self.pool_deposit.ok_or(TxBuilderError::UninitializedField(
                TxBuilderConfigField::PoolDeposit,
            ))?,
            key_deposit: self.key_deposit.ok_or(TxBuilderError::UninitializedField(
                TxBuilderConfigField::KeyDeposit,
            ))?,
            max_value_size: self
                .max_value_size
                .ok_or(TxBuilderError::UninitializedField(
                    TxBuilderConfigField::MaxValueSize,
                ))?,
            max_tx_size: self.max_tx_size.ok_or(TxBuilderError::UninitializedField(
                TxBuilderConfigField::MaxTxSize,
            ))?,
            coins_per_utxo_byte: self.coins_per_utxo_byte.ok_or(
                TxBuilderError::UninitializedField(TxBuilderConfigField::CoinsPerUtxoBytes),
            )?,
            ex_unit_prices: self
                .ex_unit_prices
                .ok_or(TxBuilderError::UninitializedField(
                    TxBuilderConfigField::ExUnitPrices,
                ))?,
            cost_models: if self.cost_models.is_some() {
                self.cost_models.unwrap()
            } else {
                CostModels::new()
            },
            _collateral_percentage: self.collateral_percentage.ok_or(
                TxBuilderError::UninitializedField(TxBuilderConfigField::CollateralPercentage),
            )?,
            max_collateral_inputs: self.max_collateral_inputs.ok_or(
                TxBuilderError::UninitializedField(TxBuilderConfigField::MaxCollateralInputs),
            )?,
            prefer_pure_change: self.prefer_pure_change,
        })
    }
}

#[derive(Clone, Debug)]
pub struct TransactionBuilder {
    config: TransactionBuilderConfig,
    inputs: Vec<TransactionUnspentOutput>,
    outputs: Vec<TransactionOutput>,
    fee: Option<Coin>,
    ttl: Option<Slot>, // absolute slot number
    certs: Option<Vec<Certificate>>,
    withdrawals: Option<Withdrawals>,
    auxiliary_data: Option<AuxiliaryData>,
    validity_start_interval: Option<Slot>,
    mint: Option<Mint>,
    collateral: Option<Vec<TransactionUnspentOutput>>,
    required_signers: Option<BTreeSet<Ed25519KeyHash>>,
    network_id: Option<NetworkId>,
    witness_builders: WitnessBuilders,
    utxos: Vec<InputBuilderResult>,
    collateral_return: Option<TransactionOutput>,
    reference_inputs: Option<Vec<TransactionUnspentOutput>>,
}

impl TransactionBuilder {
    /// This automatically selects and adds inputs from {inputs} consisting of just enough to cover
    /// the outputs that have already been added.
    /// This should be called after adding all certs/outputs/etc and will be an error otherwise.
    /// Uses CIP2: https://github.com/cardano-foundation/CIPs/blob/master/CIP-0002/CIP-0002.md
    /// Adding a change output must be called after via TransactionBuilder::add_change_if_needed()
    /// This function, diverging from CIP2, takes into account fees and will attempt to add additional
    /// inputs to cover the minimum fees. This does not, however, set the txbuilder's fee.
    pub fn select_utxos(
        &mut self,
        strategy: CoinSelectionStrategyCIP2,
    ) -> Result<(), TxBuilderError> {
        let available_inputs = self.utxos.clone();
        let mut input_total = self.get_total_input()?;
        let mut output_total = self
            .get_total_output()?
            .checked_add(&Value::from(self.min_fee(false)?))?;
        match strategy {
            CoinSelectionStrategyCIP2::LargestFirst => {
                self.cip2_largest_first_by(
                    &available_inputs,
                    &mut (0..available_inputs.len()).collect(),
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin),
                )?;
            }
            CoinSelectionStrategyCIP2::RandomImprove => {
                if self
                    .outputs
                    .iter()
                    .any(|output| output.amount().has_multiassets())
                {
                    return Err(TxBuilderError::RandomImproveCantContainMultiasset);
                }
                let mut rng = rand::thread_rng();
                let mut available_indices =
                    (0..available_inputs.len()).collect::<BTreeSet<usize>>();
                self.cip2_random_improve_by(
                    &available_inputs,
                    &mut available_indices,
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin),
                    &mut rng,
                )?;
                // Phase 3: add extra inputs needed for fees (not covered by CIP-2)
                // We do this at the end because this new inputs won't be associated with
                // a specific output, so the improvement algorithm we do above does not apply here.
                while input_total.coin < output_total.coin {
                    if available_indices.is_empty() {
                        return Err(TxBuilderError::UTxOBalanceInsufficient);
                    }
                    let i = *available_indices
                        .iter()
                        .nth(rng.gen_range(0..available_indices.len()))
                        .unwrap();
                    available_indices.remove(&i);
                    let input = &available_inputs[i];
                    let input_fee = self.fee_for_input(input)?;
                    self.add_input(input.clone()).unwrap();
                    input_total = input_total.checked_add(input.utxo_info.amount())?;
                    output_total = output_total.checked_add(&Value::from(input_fee))?;
                }
            }
            CoinSelectionStrategyCIP2::LargestFirstMultiAsset => {
                // indices into {available_inputs} for inputs that contain {policy_id}:{asset_name}
                let mut available_indices = (0..available_inputs.len()).collect::<Vec<usize>>();
                // run largest-fist by each asset type
                for (policy_id, assets) in output_total.multiasset.clone().iter() {
                    for (asset_name, _) in assets.iter() {
                        self.cip2_largest_first_by(
                            &available_inputs,
                            &mut available_indices,
                            &mut input_total,
                            &mut output_total,
                            |value| value.multiasset.get(policy_id, asset_name),
                        )?;
                    }
                }
                // add in remaining ADA
                self.cip2_largest_first_by(
                    &available_inputs,
                    &mut available_indices,
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin),
                )?;
            }
            CoinSelectionStrategyCIP2::RandomImproveMultiAsset => {
                let mut rng = rand::thread_rng();
                let mut available_indices =
                    (0..available_inputs.len()).collect::<BTreeSet<usize>>();
                // run random-improve by each asset type
                for (policy_id, assets) in output_total.multiasset.clone().iter() {
                    for (asset_name, _) in assets.iter() {
                        self.cip2_random_improve_by(
                            &available_inputs,
                            &mut available_indices,
                            &mut input_total,
                            &mut output_total,
                            |value| value.multiasset.get(policy_id, asset_name),
                            &mut rng,
                        )?;
                    }
                }
                // add in remaining ADA
                self.cip2_random_improve_by(
                    &available_inputs,
                    &mut available_indices,
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin),
                    &mut rng,
                )?;
                // Phase 3: add extra inputs needed for fees (not covered by CIP-2)
                // We do this at the end because this new inputs won't be associated with
                // a specific output, so the improvement algorithm we do above does not apply here.
                while input_total.coin < output_total.coin {
                    if available_indices.is_empty() {
                        return Err(TxBuilderError::UTxOBalanceInsufficient);
                    }
                    let i = *available_indices
                        .iter()
                        .nth(rng.gen_range(0..available_indices.len()))
                        .unwrap();
                    available_indices.remove(&i);
                    let input = &available_inputs[i];
                    let input_fee = self.fee_for_input(input)?;
                    self.add_input(input.clone()).unwrap();
                    input_total = input_total.checked_add(input.utxo_info.amount())?;
                    output_total = output_total.checked_add(&Value::from(input_fee))?;
                }
            }
        }

        Ok(())
    }

    fn cip2_largest_first_by<F>(
        &mut self,
        available_inputs: &[InputBuilderResult],
        available_indices: &mut Vec<usize>,
        input_total: &mut Value,
        output_total: &mut Value,
        by: F,
    ) -> Result<(), TxBuilderError>
    where
        F: Fn(&Value) -> Option<u64>,
    {
        let mut relevant_indices = available_indices.clone();
        relevant_indices.retain(|i| by(available_inputs[*i].utxo_info.amount()).is_some());
        // ordered in ascending order by predicate {by}
        relevant_indices
            .sort_by_key(|i| by(available_inputs[*i].utxo_info.amount()).expect("filtered above"));

        // iterate in decreasing order for predicate {by}
        for i in relevant_indices.iter().rev() {
            if by(input_total).unwrap_or_else(u64::zero)
                >= by(output_total).expect("do not call on asset types that aren't in the output")
            {
                break;
            }
            let input = &available_inputs[*i];
            // differing from CIP2, we include the needed fees in the targets instead of just output values
            let input_fee = self.fee_for_input(input)?;
            self.add_input(input.clone()).unwrap();
            *input_total = input_total.checked_add(input.utxo_info.amount())?;
            *output_total = output_total.checked_add(&Value::from(input_fee))?;
            available_indices.swap_remove(available_indices.iter().position(|j| i == j).unwrap());
        }

        if by(input_total).unwrap_or_else(u64::zero)
            < by(output_total).expect("do not call on asset types that aren't in the output")
        {
            return Err(TxBuilderError::UTxOBalanceInsufficient);
        }

        Ok(())
    }

    fn cip2_random_improve_by<F, R: Rng + ?Sized>(
        &mut self,
        available_inputs: &[InputBuilderResult],
        available_indices: &mut BTreeSet<usize>,
        input_total: &mut Value,
        output_total: &mut Value,
        by: F,
        rng: &mut R,
    ) -> Result<(), TxBuilderError>
    where
        F: Fn(&Value) -> Option<u64>,
    {
        // Phase 1: Random Selection
        let mut relevant_indices = available_indices
            .iter()
            .filter(|i| by(available_inputs[**i].utxo_info.amount()).is_some())
            .cloned()
            .collect::<Vec<usize>>();
        let mut associated_indices: HashMap<TransactionOutput, Vec<usize>> = HashMap::new();
        let mut outputs = self
            .outputs
            .iter()
            .filter(|output| by(output.amount()).is_some())
            .cloned()
            .collect::<Vec<TransactionOutput>>();
        outputs.sort_by_key(|output| by(output.amount()).expect("filtered above"));
        for output in outputs.iter().rev() {
            // TODO: how should we adapt this to inputs being associated when running for other assets?
            // if we do these two phases for each asset and don't take into account the other runs for other assets
            // then we over-add (and potentially fail if we don't have plenty of inputs)
            // On the other hand, the improvement phase it difficult to determine if a change is an improvement
            // if we're trying to improve for multiple assets at a time without knowing how important each input is
            // e.g. maybe we have lots of asset A but not much of B
            // For now I will just have this be entirely separate per-asset but we might want to in a later commit
            // consider the improvements separately and have it take some kind of dot product / distance for assets
            // during the improvement phase and have the improvement phase target multiple asset types at once.
            // One issue with that is how to scale in between different assets. We could maybe normalize them by
            // dividing each asset type by the sum of the required asset type in all outputs.
            // Another possibility for adapting this to multiassets is when associating an input x for asset type a
            // we try and subtract all other assets b != a from the outputs we're trying to cover.
            // It might make sense to diverge further and not consider it per-output and to instead just match against
            // the sum of all outputs as one single value.
            let mut added = u64::zero();
            let needed = by(output.amount()).unwrap();
            while added < needed {
                if relevant_indices.is_empty() {
                    return Err(TxBuilderError::UTxOBalanceInsufficient);
                }
                let random_index = rng.gen_range(0..relevant_indices.len());
                let i = relevant_indices.swap_remove(random_index);
                available_indices.remove(&i);
                let input = &available_inputs[i];
                added = added
                    .checked_add(
                        by(input.utxo_info.amount())
                            .expect("do not call on asset types that aren't in the output"),
                    )
                    .ok_or(ArithmeticError::IntegerOverflow)?;
                associated_indices
                    .entry(output.clone())
                    .or_default()
                    .push(i);
            }
        }
        if !relevant_indices.is_empty() {
            // Phase 2: Improvement
            for output in outputs.iter_mut() {
                let associated = associated_indices.get_mut(output).unwrap();
                for i in associated.iter_mut() {
                    let random_index = rng.gen_range(0..relevant_indices.len());
                    let j: &mut usize = relevant_indices.get_mut(random_index).unwrap();
                    let should_improve = {
                        let input = &available_inputs[*i];
                        let new_input = &available_inputs[*j];
                        let cur = input.utxo_info.amount().coin;
                        let new = new_input.utxo_info.amount().coin;
                        let min = output.amount().coin;
                        let ideal = 2 * min;
                        let max = 3 * min;
                        let move_closer = (ideal as i128 - new as i128).abs()
                            < (ideal as i128 - cur as i128).abs();
                        let not_exceed_max = new < max;

                        move_closer && not_exceed_max
                    };
                    if should_improve {
                        available_indices.insert(*i);
                        available_indices.remove(j);
                        std::mem::swap(i, j);
                    }
                }
            }
        }

        // after finalizing the improvement we need to actually add these results to the builder
        for output in outputs.iter() {
            for i in associated_indices.get(output).unwrap().iter() {
                let input = &available_inputs[*i];
                let input_fee = self.fee_for_input(input)?;
                self.add_input(input.clone()).unwrap();
                *input_total = input_total.checked_add(input.utxo_info.amount())?;
                *output_total = output_total.checked_add(&Value::from(input_fee))?;
            }
        }

        Ok(())
    }

    pub fn add_input(&mut self, result: InputBuilderResult) -> Result<(), TxBuilderError> {
        if let Some(script_ref) = result.utxo_info.script_ref() {
            self.witness_builders
                .witness_set_builder
                .required_wits
                .script_refs
                .insert(script_ref.hash());
        }
        self.witness_builders
            .redeemer_set_builder
            .add_spend(&result);
        self.witness_builders
            .witness_set_builder
            .add_required_wits(result.required_wits);
        self.inputs.push(TransactionUnspentOutput {
            input: result.input,
            output: result.utxo_info,
        });
        if let Some(data) = result.aggregate_witness {
            self.witness_builders
                .witness_set_builder
                .add_input_aggregate_real_witness_data(&data);
            self.witness_builders
                .fake_required_witnesses
                .add_input_aggregate_fake_witness_data(&data);
            if let InputAggregateWitnessData::PlutusScript(script_witness, required_signers, _) =
                data
            {
                required_signers
                    .into_iter()
                    .for_each(|signer| self.add_required_signer(signer));

                match &script_witness.script {
                    PlutusScriptWitness::Ref(ref_script) => {
                        if self
                            .witness_builders
                            .witness_set_builder
                            .required_wits
                            .script_refs
                            .get(ref_script)
                            .is_none()
                        {
                            Err(TxBuilderError::RefScriptNotFound(
                                *ref_script,
                                self.witness_builders
                                    .witness_set_builder
                                    .required_wits
                                    .script_refs
                                    .clone(),
                            ))
                        } else {
                            Ok(())
                        }
                    }
                    _ => Ok(()),
                }
                .unwrap();
            }
        }
        Ok(())
    }

    pub fn add_utxo(&mut self, result: InputBuilderResult) {
        self.utxos.push(result);
    }

    /// calculates how much the fee would increase if you added a given output
    pub fn fee_for_input(&self, result: &InputBuilderResult) -> Result<Coin, TxBuilderError> {
        let mut self_copy = self.clone();

        // we need some value for these for it to be a a valid transaction
        // but since we're only calculating the difference between the fee of two transactions
        // it doesn't matter what these are set as, since it cancels out
        self_copy.set_fee(0);

        let fee_before = min_fee(&self_copy)?;

        self_copy.add_input(result.clone()).unwrap();
        let fee_after = min_fee(&self_copy)?;
        fee_after
            .checked_sub(fee_before)
            .ok_or_else(|| ArithmeticError::IntegerOverflow.into())
    }

    pub fn add_reference_input(&mut self, utxo: TransactionUnspentOutput) {
        let reference_inputs = match self.reference_inputs.as_mut() {
            None => {
                self.reference_inputs = Some(Vec::<TransactionUnspentOutput>::new());
                self.reference_inputs.as_mut().unwrap()
            }
            Some(inputs) => inputs,
        };

        if let Some(script_ref) = utxo.output.script_ref() {
            self.witness_builders
                .witness_set_builder
                .required_wits
                .script_refs
                .insert(script_ref.hash());
        }

        reference_inputs.push(utxo);
    }

    /// Add explicit output via a TransactionOutput object
    pub fn add_output(
        &mut self,
        builder_result: SingleOutputBuilderResult,
    ) -> Result<(), TxBuilderError> {
        let output = builder_result.output;
        let value_size = output.amount().to_cbor_bytes().len();
        if value_size > self.config.max_value_size as usize {
            return Err(TxBuilderError::MaxValueSizeExceeded(
                self.config.max_value_size,
                value_size,
            ));
        }
        let min_ada = min_ada_required(&output, self.config.coins_per_utxo_byte)?;
        if output.amount().coin < min_ada {
            Err(TxBuilderError::ValueBelowMinUTXOValue(
                output.amount().coin,
                min_ada,
            ))
        } else {
            if let Some(datum) = builder_result.communication_datum {
                self.witness_builders
                    .witness_set_builder
                    .add_plutus_datum(datum);
            }
            self.outputs.push(output);
            Ok(())
        }
    }

    /// calculates how much the fee would increase if you added a given output
    pub fn fee_for_output(
        &self,
        builder: &SingleOutputBuilderResult,
    ) -> Result<Coin, TxBuilderError> {
        let mut self_copy = self.clone();

        // we need some value for these for it to be a a valid transaction
        // but since we're only calculating the different between the fee of two transactions
        // it doesn't matter what these are set as, since it cancels out
        self_copy.set_fee(0);

        let fee_before = min_fee(&self_copy)?;

        self_copy.add_output(builder.clone())?;
        let fee_after = min_fee(&self_copy)?;
        fee_after
            .checked_sub(fee_before)
            .ok_or_else(|| ArithmeticError::IntegerOverflow.into())
    }

    pub fn set_fee(&mut self, fee: Coin) {
        self.fee = Some(fee)
    }

    pub fn set_ttl(&mut self, ttl: Slot) {
        self.ttl = Some(ttl)
    }

    pub fn set_validity_start_interval(&mut self, validity_start_interval: Slot) {
        self.validity_start_interval = Some(validity_start_interval)
    }

    pub fn add_cert(&mut self, result: CertificateBuilderResult) {
        self.witness_builders.redeemer_set_builder.add_cert(&result);
        if self.certs.is_none() {
            self.certs = Some(Vec::new());
        }
        self.certs.as_mut().unwrap().push(result.cert);
        if let Some(data) = result.aggregate_witness {
            self.witness_builders
                .witness_set_builder
                .add_input_aggregate_real_witness_data(&data);
            self.witness_builders
                .fake_required_witnesses
                .add_input_aggregate_fake_witness_data(&data);
            if let InputAggregateWitnessData::PlutusScript(_, required_signers, _) = data {
                required_signers
                    .into_iter()
                    .for_each(|signer| self.add_required_signer(signer));
            }
        }
        self.witness_builders
            .witness_set_builder
            .add_required_wits(result.required_wits);
    }

    pub fn get_withdrawals(&self) -> Option<Withdrawals> {
        self.withdrawals.clone()
    }

    pub fn add_withdrawal(&mut self, result: WithdrawalBuilderResult) {
        self.witness_builders
            .redeemer_set_builder
            .add_reward(&result);
        if self.withdrawals.is_none() {
            self.withdrawals = Some(OrderedHashMap::default());
        }
        self.withdrawals
            .as_mut()
            .unwrap()
            .insert(result.address, result.amount);
        if let Some(data) = result.aggregate_witness {
            self.witness_builders
                .witness_set_builder
                .add_input_aggregate_real_witness_data(&data);
            self.witness_builders
                .fake_required_witnesses
                .add_input_aggregate_fake_witness_data(&data);
            if let InputAggregateWitnessData::PlutusScript(_, required_signers, _) = data {
                required_signers
                    .into_iter()
                    .for_each(|signer| self.add_required_signer(signer));
            }
        }
        self.witness_builders
            .witness_set_builder
            .add_required_wits(result.required_wits);
    }

    pub fn get_auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.auxiliary_data.clone()
    }

    pub fn set_auxiliary_data(&mut self, new_aux_data: AuxiliaryData) {
        self.auxiliary_data = Some(new_aux_data)
    }

    pub fn add_auxiliary_data(&mut self, new_aux_data: AuxiliaryData) {
        match self.auxiliary_data.as_mut() {
            Some(data) => {
                data.add(new_aux_data);
            }
            None => {
                self.auxiliary_data = Some(new_aux_data);
            }
        }
    }

    pub fn add_mint(&mut self, result: MintBuilderResult) -> Result<(), TxBuilderError> {
        self.witness_builders.redeemer_set_builder.add_mint(&result);
        self.witness_builders
            .witness_set_builder
            .add_required_wits(result.required_wits.clone());
        let mut mint = self.mint.take().unwrap_or_default();
        let combined_assets = mint.deref_mut().entry(result.policy_id).or_default();
        for (asset_name, asset_value) in result.assets.iter() {
            if combined_assets
                .insert(asset_name.clone(), *asset_value)
                .is_some()
            {
                return Err(TxBuilderError::DuplicateMint(
                    result.policy_id,
                    asset_name.clone(),
                ));
            }
        }
        self.mint = Some(mint);
        if let Some(data) = result.aggregate_witness {
            self.witness_builders
                .witness_set_builder
                .add_input_aggregate_real_witness_data(&data);
            self.witness_builders
                .fake_required_witnesses
                .add_input_aggregate_fake_witness_data(&data);
            if let InputAggregateWitnessData::PlutusScript(_, required_signers, _) = data {
                required_signers
                    .into_iter()
                    .for_each(|signer| self.add_required_signer(signer));
            }
        }
        Ok(())
    }

    /// Returns a copy of the current mint state in the builder
    pub fn get_mint(&self) -> Option<Mint> {
        self.mint.clone()
    }

    pub fn new(cfg: TransactionBuilderConfig) -> Self {
        Self {
            config: cfg,
            inputs: Vec::new(),
            outputs: Vec::new(),
            fee: None,
            ttl: None,
            certs: None,
            withdrawals: None,
            auxiliary_data: None,
            validity_start_interval: None,
            mint: None,
            collateral: None,
            required_signers: None,
            network_id: None,
            witness_builders: WitnessBuilders::default(),
            utxos: Vec::new(),
            collateral_return: None,
            reference_inputs: None,
        }
    }

    pub fn add_collateral(&mut self, result: InputBuilderResult) -> Result<(), TxBuilderError> {
        if result.aggregate_witness.is_some() {
            return Err(TxBuilderError::CollateralMustBePayment);
        }
        let new_input = TransactionUnspentOutput {
            input: result.input,
            output: result.utxo_info,
        };
        match &mut self.collateral {
            None => self.collateral = Some(vec![new_input]),
            Some(collateral) => {
                if self.config.max_collateral_inputs <= collateral.len().try_into().unwrap() {
                    return Err(TxBuilderError::MaxCollateralInputExceeded(
                        self.config.max_collateral_inputs,
                    ));
                }
                collateral.push(new_input);
            }
        }

        // note: collateral doesn't get counted for ref scripts

        if let Some(data) = result.aggregate_witness {
            self.witness_builders
                .witness_set_builder
                .add_input_aggregate_real_witness_data(&data);
            self.witness_builders
                .fake_required_witnesses
                .add_input_aggregate_fake_witness_data(&data);
            if let InputAggregateWitnessData::PlutusScript(_, required_signers, _) = data {
                required_signers
                    .iter()
                    .for_each(|signer| self.add_required_signer(*signer));
            }
        }
        self.witness_builders
            .witness_set_builder
            .add_required_wits(result.required_wits);

        Ok(())
    }

    pub fn add_required_signer(&mut self, hash: Ed25519KeyHash) {
        let mut set = RequiredWitnessSet::new();
        set.add_vkey_key_hash(hash);
        self.witness_builders
            .witness_set_builder
            .add_required_wits(set);

        match &mut self.required_signers {
            None => {
                let mut required_signers = BTreeSet::new();
                required_signers.insert(hash);
                self.required_signers = Some(required_signers);
            }
            Some(required_signers) => {
                required_signers.insert(hash);
            }
        }
    }

    pub fn set_network_id(&mut self, network_id: NetworkId) {
        self.network_id = Some(network_id)
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        self.network_id
    }

    /// does not include refunds or withdrawals
    pub fn get_explicit_input(&self) -> Result<Value, TxBuilderError> {
        self.inputs
            .iter()
            .try_fold(Value::zero(), |acc, tx_builder_input| {
                acc.checked_add(tx_builder_input.output.amount())
            })
            .map_err(Into::into)
    }

    /// withdrawals and refunds
    pub fn get_implicit_input(&self) -> Result<Value, TxBuilderError> {
        internal_get_implicit_input(
            self.withdrawals.as_ref(),
            self.certs.as_deref(),
            self.config.pool_deposit,
            self.config.key_deposit,
        )
        .map_err(Into::into)
    }

    /// Returns mint as tuple of (mint_value, burn_value) or two zero values
    fn get_mint_as_values(&self) -> (Value, Value) {
        self.mint
            .as_ref()
            .map(|m| {
                (
                    Value::from(m.as_positive_multiasset()),
                    Value::from(m.as_negative_multiasset()),
                )
            })
            .unwrap_or((Value::zero(), Value::zero()))
    }

    /// Return explicit input plus implicit input plus mint
    pub fn get_total_input(&self) -> Result<Value, TxBuilderError> {
        let (mint_value, _) = self.get_mint_as_values();
        self.get_explicit_input()?
            .checked_add(&self.get_implicit_input()?)
            .and_then(|x| x.checked_add(&mint_value))
            .map_err(Into::into)
    }

    /// Return explicit output plus implicit output plus burn (does not consider fee directly)
    pub fn get_total_output(&self) -> Result<Value, TxBuilderError> {
        let (_, burn_value) = self.get_mint_as_values();
        self.get_explicit_output()?
            .checked_add(&Value::from(self.get_deposit()?))
            .and_then(|x| x.checked_add(&burn_value))
            .map_err(Into::into)
    }

    /// does not include fee
    pub fn get_explicit_output(&self) -> Result<Value, TxBuilderError> {
        self.outputs
            .iter()
            .try_fold(Value::from(0), |acc, output| {
                acc.checked_add(output.amount())
            })
            .map_err(Into::into)
    }

    pub fn get_deposit(&self) -> Result<Coin, TxBuilderError> {
        internal_get_deposit(
            self.certs.as_deref(),
            self.config.pool_deposit,
            self.config.key_deposit,
        )
        .map_err(Into::into)
    }

    pub fn get_fee_if_set(&self) -> Option<Coin> {
        self.fee
    }

    pub fn set_collateral_return(&mut self, output: TransactionOutput) {
        self.collateral_return = Some(output);
    }

    fn calc_collateral_total(&self) -> Result<Option<Coin>, TxBuilderError> {
        match self.collateral_return.as_ref() {
            None => Ok(None),
            Some(coll_ret) => {
                let input_sum = match self.collateral.as_ref() {
                    Some(collateral) => collateral.iter().try_fold(Coin::zero(), |acc, next| {
                        acc.checked_add(next.output.amount().coin)
                            .ok_or(ArithmeticError::IntegerOverflow)
                    }),
                    None => return Err(TxBuilderError::CollateralReturnRequiresCollateralInput),
                }?;

                let coll_tot = input_sum
                    .checked_sub(coll_ret.amount().coin)
                    .ok_or(ArithmeticError::IntegerOverflow)?;
                Ok(Some(coll_tot))
            }
        }
    }

    fn build_and_size(&self) -> Result<(TransactionBody, usize), TxBuilderError> {
        let fee = self.fee.ok_or(TxBuilderError::FeeNotSpecified)?;

        let redeemers = self.witness_builders.redeemer_set_builder.build(true)?;
        let has_dummy_exunit = redeemers
            .iter()
            .any(|redeemer| redeemer.ex_units == ExUnits::dummy());

        let script_data_hash = match self.witness_builders.redeemer_set_builder.is_empty() {
            true => None,
            // dummy exunits use large values
            // to avoid users accidentally spending all their ADA in tx fees,
            // we make that dummy exunits set a dummy script_data_hash to ensure the tx fails if submitted to a node
            false => match has_dummy_exunit {
                true => Some(ScriptDataHash::from([0u8; ScriptDataHash::BYTE_COUNT])),
                false => {
                    let scripts = self.witness_builders.witness_set_builder.scripts.values();
                    let mut languages =
                        scripts.fold(BTreeSet::<Language>::new(), |mut langs, script| {
                            if let Some(lang) = script.language() {
                                langs.insert(lang);
                            }
                            langs
                        });
                    if let Some(reference_inputs) = self.reference_inputs.as_ref() {
                        reference_inputs
                            .iter()
                            .fold(&mut languages, |langs, input| {
                                if let Some(script_ref) = &input.output.script_ref() {
                                    if let Some(lang) = script_ref.language() {
                                        langs.insert(lang);
                                    }
                                }
                                langs
                            });
                    };
                    self.inputs
                        .clone()
                        .iter()
                        .fold(&mut languages, |langs, input| {
                            if let Some(script_ref) = &input.output.script_ref() {
                                if let Some(lang) = script_ref.language() {
                                    langs.insert(lang);
                                }
                            }
                            langs
                        });
                    calc_script_data_hash(
                        &redeemers,
                        &self.witness_builders.witness_set_builder.get_plutus_datum(),
                        &self.config.cost_models,
                        &languages.iter().copied().collect::<Vec<_>>(),
                        None,
                    )?
                }
            },
        };
        let mut built = TransactionBody {
            inputs: self
                .inputs
                .iter()
                .map(|tx_builder_input| tx_builder_input.input.clone())
                .collect(),
            outputs: self.outputs.clone(),
            fee,
            ttl: self.ttl,
            certs: self.certs.clone(),
            withdrawals: self.withdrawals.clone(),
            auxiliary_data_hash: self.auxiliary_data.as_ref().map(hash_auxiliary_data),
            validity_interval_start: self.validity_start_interval,
            mint: self.mint.clone(),
            script_data_hash,
            collateral_inputs: self
                .collateral
                .as_ref()
                .map(|collateral| collateral.iter().map(|c| c.input.clone()).collect()),
            required_signers: self
                .required_signers
                .as_ref()
                .map(|set| set.iter().cloned().collect()),
            network_id: self.network_id,
            collateral_return: self.collateral_return.clone(),
            total_collateral: self.calc_collateral_total()?,
            reference_inputs: self
                .reference_inputs
                .as_ref()
                .map(|inputs| inputs.iter().map(|utxo| utxo.input.clone()).collect()),
            voting_procedures: None,
            proposal_procedures: None,
            current_treasury_value: None,
            donation: None,
            encodings: None,
        };

        // indices for redeemers in smart contract txs require fields to be sorted
        {
            // We sort inputs and withdrawals only since certs remain in the order given and
            // mint is sorted as items are added (by the nature of BTreeMaps)
            built
                .inputs
                .sort_by(|a, b| match a.transaction_id.cmp(&b.transaction_id) {
                    std::cmp::Ordering::Equal => a.index.cmp(&b.index),
                    rest => rest,
                });

            if let Some(withdrawals) = built.withdrawals {
                let mut sorted_keys = withdrawals.keys().collect::<Vec<_>>();
                sorted_keys.sort();

                let mut sorted_linked_hashmap = Withdrawals::new();
                sorted_linked_hashmap =
                    sorted_keys
                        .iter()
                        .fold(sorted_linked_hashmap, |mut accum, key| {
                            accum.insert((*key).clone(), *withdrawals.get(key).unwrap());
                            accum
                        });
                built.withdrawals = Some(sorted_linked_hashmap)
            };
        }

        // we must build a tx with fake data (of correct size) to check the final Transaction size
        let full_tx = fake_full_tx(self, built)?;
        let full_tx_size = full_tx.to_cbor_bytes().len();
        Ok((full_tx.body, full_tx_size))
    }

    pub fn full_size(&self) -> Result<usize, TxBuilderError> {
        self.build_and_size().map(|r| r.1)
    }

    pub fn output_sizes(&self) -> Vec<usize> {
        self.outputs
            .iter()
            .map(|o| o.to_cbor_bytes().len())
            .collect()
    }

    /// Returns object the body of the new transaction
    fn build_body(&self) -> Result<TransactionBody, TxBuilderError> {
        let (body, full_tx_size) = self.build_and_size()?;
        if full_tx_size > self.config.max_tx_size as usize {
            Err(TxBuilderError::MaxTxSizeExceeded(
                self.config.max_tx_size,
                full_tx_size,
            ))
        } else {
            Ok(body)
        }
    }

    // TODO: switch from ChangeSelectionAlgo to ChangeSelectionBuilder
    /// Builds the transaction and moves to the next step redeemer units can be added and a draft tx can
    /// be evaluated
    /// NOTE: is_valid set to true
    pub fn build_for_evaluation(
        &self,
        algo: ChangeSelectionAlgo,
        change_address: &Address,
    ) -> Result<TxRedeemerBuilder, TxBuilderError> {
        // First we finish change selection

        let mut tx = self.clone();
        choose_change_selection_algo(algo)(&mut tx, change_address, false)?;

        Ok(TxRedeemerBuilder {
            draft_body: tx.build_body()?,
            witness_builders: tx.witness_builders.clone(),
            auxiliary_data: tx.auxiliary_data.clone(),
        })
    }

    // TODO: switch from ChangeSelectionAlgo to ChangeSelectionBuilder
    /// Builds the transaction and moves to the next step where any real witness can be added
    /// NOTE: is_valid set to true
    pub fn build(
        &mut self,
        algo: ChangeSelectionAlgo,
        change_address: &Address,
    ) -> Result<SignedTxBuilder, TxBuilderError> {
        // First we finish change selection
        choose_change_selection_algo(algo)(self, change_address, true)?;

        Ok(SignedTxBuilder {
            body: self.build_body()?,
            // Side note: redeemer indices are calculated every time witness builder is built
            witness_set: self.witness_builders.build_unchecked()?,
            is_valid: true,
            auxiliary_data: self.auxiliary_data.clone(),
        })
    }

    /// used to override the exunit values initially provided when adding inputs
    pub fn set_exunits(&mut self, redeemer: RedeemerWitnessKey, ex_units: ExUnits) {
        self.witness_builders
            .redeemer_set_builder
            .update_ex_units(redeemer, ex_units);
    }

    /// warning: sum of all parts of a transaction must equal 0. You cannot just set the fee to the min value and forget about it
    /// warning: min_fee may be slightly larger than the actual minimum fee (ex: a few lovelaces)
    /// this is done to simplify the library code, but can be fixed later
    pub fn min_fee(&self, script_calulation: bool) -> Result<Coin, TxBuilderError> {
        if !script_calulation {
            let mut self_copy = self.clone();
            self_copy.fee = Some(u64::MAX);
            min_fee(&self_copy)
        } else {
            let mut self_copy = self.clone();
            self_copy.fee = Some(u64::MAX);
            min_fee_with_exunits(&self_copy)
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxRedeemerBuilder {
    draft_body: TransactionBody,
    witness_builders: WitnessBuilders,
    auxiliary_data: Option<AuxiliaryData>,
}

impl TxRedeemerBuilder {
    /// Builds the transaction and moves to the next step where any real witness can be added
    /// NOTE: is_valid set to true
    /// Will NOT require you to have set required signers & witnesses
    pub fn build(&self) -> Result<Vec<Redeemer>, RedeemerBuilderError> {
        self.witness_builders.redeemer_set_builder.build(true)
    }

    /// used to override the exunit values initially provided when adding inputs
    pub fn set_exunits(&mut self, redeemer: RedeemerWitnessKey, ex_units: ExUnits) {
        self.witness_builders
            .redeemer_set_builder
            .update_ex_units(redeemer, ex_units);
    }

    /// Transaction body with a dummy values for redeemers & script_data_hash
    /// Used for calculating exunits or required signers
    pub fn draft_body(&self) -> TransactionBody {
        self.draft_body.clone()
    }

    pub fn auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.auxiliary_data.clone()
    }

    /// Transaction body with a dummy values for redeemers & script_data_hash and padded with dummy witnesses
    /// Used for calculating exunits
    /// note: is_valid set to true
    pub fn draft_tx(&self) -> Result<Transaction, WitnessBuilderError> {
        Ok(Transaction::new(
            self.draft_body.clone(),
            // Side note: redeemer indices are calculated every time witness builder is built
            self.witness_builders.build_fake()?,
            true,
            self.auxiliary_data.clone(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct SignedTxBuilder {
    body: TransactionBody,
    witness_set: TransactionWitnessSetBuilder,
    is_valid: bool,
    auxiliary_data: Option<AuxiliaryData>,
}

impl SignedTxBuilder {
    pub fn new_with_data(
        body: TransactionBody,
        witness_set: TransactionWitnessSetBuilder,
        is_valid: bool,
        auxiliary_data: AuxiliaryData,
    ) -> SignedTxBuilder {
        SignedTxBuilder {
            body,
            witness_set,
            is_valid,
            auxiliary_data: Some(auxiliary_data),
        }
    }

    pub fn new_without_data(
        body: TransactionBody,
        witness_set: TransactionWitnessSetBuilder,
        is_valid: bool,
    ) -> SignedTxBuilder {
        SignedTxBuilder {
            body,
            witness_set,
            is_valid,
            auxiliary_data: None,
        }
    }

    pub fn build_checked(self) -> Result<Transaction, WitnessBuilderError> {
        Ok(Transaction::new(
            self.body,
            self.witness_set.try_build()?,
            self.is_valid,
            self.auxiliary_data,
        ))
    }

    pub fn build_unchecked(self) -> Transaction {
        Transaction::new(
            self.body,
            self.witness_set.build(),
            self.is_valid,
            self.auxiliary_data,
        )
    }

    // Note: we only allow adding vkey & bootstraps at this stage
    // This is because other witness kinds increase the tx size
    // so they should have been added during the TransactionBuilder step
    //
    // However, if you manually set the fee during the TransactionBuilder step
    // to allow adding some extra witnesses later,
    // use `build_unchecked`
    //
    // Note: can't easily check inside the `add_vkey` or `add_bootstrap` functions if the user added a wrong witness
    // This is because scripts may require keys that weren't known exactly during the tx building phase

    pub fn add_vkey(&mut self, vkey: Vkeywitness) {
        self.witness_set.add_vkey(vkey);
    }

    pub fn add_bootstrap(&mut self, bootstrap: BootstrapWitness) {
        self.witness_set.add_bootstrap(bootstrap);
    }

    pub fn body(&self) -> TransactionBody {
        self.body.clone()
    }

    pub fn witness_set(&self) -> TransactionWitnessSetBuilder {
        self.witness_set.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.auxiliary_data.clone()
    }
}

#[wasm_bindgen]
pub enum ChangeSelectionAlgo {
    Default,
}

pub fn choose_change_selection_algo(
    algo: ChangeSelectionAlgo,
) -> fn(&mut TransactionBuilder, &Address, include_exunits: bool) -> Result<bool, TxBuilderError> {
    match algo {
        ChangeSelectionAlgo::Default => add_change_if_needed,
    }
}

/// Warning: this function will mutate the /fee/ field
/// Make sure to call this function last after setting all other tx-body properties
/// Editing inputs, outputs, mint, etc. after change been calculated
/// might cause a mismatch in calculated fee versus the required fee
pub fn add_change_if_needed(
    builder: &mut TransactionBuilder,
    address: &Address,
    include_exunits: bool,
) -> Result<bool, TxBuilderError> {
    let fee = match &builder.fee {
        None => builder.min_fee(include_exunits),
        // generating the change output involves changing the fee
        Some(_x) => return Ok(false),
    }?;

    // note: can't add datum / script_ref to change
    // because we don't know how many change outputs will need to be created
    let datum = None;
    let script_ref = None;
    let communication_datum = None;

    let input_total = builder.get_total_input()?;

    let output_total = builder.get_total_output()?;

    use std::cmp::Ordering;
    match &input_total.partial_cmp(&output_total.checked_add(&Value::from(fee))?) {
        Some(Ordering::Equal) => {
            // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
            builder.set_fee(input_total.checked_sub(&output_total)?.coin);
            Ok(false)
        }
        Some(Ordering::Less) => Err(TxBuilderError::UTxOBalanceInsufficient),
        Some(Ordering::Greater) => {
            let change_estimator = input_total.checked_sub(&output_total)?;
            if change_estimator.has_multiassets() {
                fn will_adding_asset_make_output_overflow(
                    output: &TransactionOutput,
                    current_assets: &OrderedHashMap<AssetName, u64>,
                    asset_to_add: (PolicyId, AssetName, u64),
                    max_value_size: u32,
                    coins_per_utxo_byte: Coin,
                ) -> bool {
                    let (policy, asset_name, value) = asset_to_add;
                    let mut current_assets_clone = current_assets.clone();
                    current_assets_clone.insert(asset_name, value);
                    let mut amount_clone = output.amount().clone();
                    let mut val = Value::from(Coin::zero());
                    val.multiasset.insert(policy, current_assets_clone);
                    amount_clone = amount_clone.checked_add(&val).unwrap();

                    let mut output_clone = output.clone();
                    output_clone.set_amount(val);

                    // calculate minADA for more precise max value size
                    let min_ada = min_ada_required(&output_clone, coins_per_utxo_byte).unwrap();
                    amount_clone.coin = min_ada;

                    amount_clone.to_cbor_bytes().len() > max_value_size as usize
                }
                fn pack_nfts_for_change(
                    max_value_size: u32,
                    coins_per_utxo_byte: Coin,
                    change_address: &Address,
                    change_estimator: &Value,
                    datum: Option<DatumOption>,
                    script_ref: &Option<ScriptRef>,
                    _communication_datum: &Option<PlutusData>,
                ) -> Result<Vec<MultiAsset>, TxBuilderError> {
                    // we insert the entire available ADA temporarily here since that could potentially impact the size
                    // as it could be 1, 2 3 or 4 bytes for Coin.
                    let mut change_assets: Vec<MultiAsset> = Vec::new();

                    let mut base_coin = Value::from(change_estimator.coin);
                    let mut output = TransactionOutput::new(
                        change_address.clone(),
                        base_coin.clone(),
                        datum.clone(),
                        script_ref.clone(),
                    );
                    // If this becomes slow on large TXs we can optimize it like the following
                    // to avoid cloning + reserializing the entire output.
                    // This would probably be more relevant if we use a smarter packing algorithm
                    // which might need to compare more size differences than greedy
                    //let mut bytes_used = output.to_bytes().len();

                    // a greedy packing is done here to avoid an exponential bin-packing
                    // which in most cases likely shouldn't be the difference between
                    // having an extra change output or not unless there are gigantic
                    // differences in NFT policy sizes
                    for (policy, assets) in change_estimator.multiasset.iter() {
                        // for simplicity we also don't split assets within a single policy since
                        // you would need to have a very high amount of assets (which add 1-36 bytes each)
                        // in a single policy to make a difference. In the future if this becomes an issue
                        // we can change that here.

                        // this is the other part of the optimization but we need to take into account
                        // the difference between CBOR encoding which can change which happens in two places:
                        // a) length within assets of one policy id
                        // b) length of the entire multiasset
                        // so for simplicity we will just do it the safe, naive way unless
                        // performance becomes an issue.
                        //let extra_bytes = policy.to_bytes().len() + assets.to_bytes().len() + 2 + cbor_len_diff;
                        //if bytes_used + extra_bytes <= max_value_size as usize {
                        let mut old_amount = output.amount().clone();
                        let mut val = Value::from(Coin::zero());
                        let mut next_nft = MultiAsset::default();

                        let mut rebuilt_assets = OrderedHashMap::new();
                        for (asset_name, value) in assets.iter() {
                            if will_adding_asset_make_output_overflow(
                                &output,
                                &rebuilt_assets,
                                (*policy, asset_name.clone(), *value),
                                max_value_size,
                                coins_per_utxo_byte,
                            ) {
                                // if we got here, this means we will run into a overflow error,
                                // so we want to split into multiple outputs, for that we...

                                // 1. insert the current assets as they are, as this won't overflow
                                next_nft.insert(*policy, rebuilt_assets);
                                val.multiasset = next_nft;
                                output.set_amount(output.amount().checked_add(&val)?);
                                change_assets.push(output.amount().multiasset.clone());

                                // 2. create a new output with the base coin value as zero
                                base_coin = Value::from(Coin::zero());
                                output = TransactionOutput::new(
                                    change_address.clone(),
                                    base_coin.clone(),
                                    datum.clone(),
                                    script_ref.clone(),
                                );

                                // 3. continue building the new output from the asset we stopped
                                old_amount = output.amount().clone();
                                val = Value::from(Coin::zero());
                                next_nft = MultiAsset::default();

                                rebuilt_assets = OrderedHashMap::new();
                            }

                            rebuilt_assets.insert(asset_name.clone(), *value);
                        }

                        next_nft.insert(*policy, rebuilt_assets);
                        val.multiasset = next_nft;
                        output.set_amount(output.amount().checked_add(&val)?);

                        // calculate minADA for more precise max value size
                        let mut output_copy = output.clone();
                        output_copy.set_amount(val);
                        let min_ada = min_ada_required(&output_copy, coins_per_utxo_byte).unwrap();

                        let mut amount_clone = output.amount().clone();
                        amount_clone.coin = min_ada;
                        if amount_clone.to_cbor_bytes().len() > max_value_size as usize {
                            output.set_amount(old_amount);
                            break;
                        }
                    }
                    change_assets.push(output.amount().multiasset.clone());
                    Ok(change_assets)
                }
                let mut change_left = input_total.checked_sub(&output_total)?;
                let mut new_fee = fee;
                // we might need multiple change outputs for cases where the change has many asset types
                // which surpass the max UTXO size limit
                let minimum_utxo_val = min_ada_required(
                    &TransactionOutput::new(
                        address.clone(),
                        // This is value is taken from the old code's code for min_pure_ada():
                        // arbitrary value that happens to give the right number of bytes at the CBOR level
                        Value::from(1000000),
                        datum.clone(),
                        script_ref.clone(),
                    ),
                    builder.config.coins_per_utxo_byte,
                )?;
                while let Some(Ordering::Greater) =
                    change_left.multiasset.partial_cmp(&MultiAsset::default())
                {
                    let nft_changes = pack_nfts_for_change(
                        builder.config.max_value_size,
                        builder.config.coins_per_utxo_byte,
                        address,
                        &change_left,
                        datum.clone(),
                        &script_ref,
                        &communication_datum,
                    )?;
                    if nft_changes.is_empty() {
                        // this likely should never happen
                        return Err(TxBuilderError::NFTTooLargeForChange);
                    }
                    // we only add the minimum needed (for now) to cover this output
                    for nft_change in nft_changes {
                        let change_output = (TransactionOutputBuilder {
                            address: Some(address.clone()),
                            datum: datum.clone(),
                            communication_datum: communication_datum.clone(),
                            script_ref: script_ref.clone(),
                        })
                        .next()?
                        .with_asset_and_min_required_coin(
                            nft_change,
                            builder.config.coins_per_utxo_byte,
                        )?
                        .build()?;

                        // increase fee
                        let fee_for_change = builder.fee_for_output(&change_output)?;
                        new_fee = new_fee
                            .checked_add(fee_for_change)
                            .ok_or(ArithmeticError::IntegerOverflow)?;
                        let change_ada_plus_fee = change_output
                            .output
                            .amount()
                            .coin
                            .checked_add(new_fee)
                            .ok_or(ArithmeticError::IntegerOverflow)?;
                        if change_left.coin < change_ada_plus_fee {
                            return Err(TxBuilderError::InsufficientADAForAssets);
                        }
                        change_left = change_left.checked_sub(change_output.output.amount())?;
                        builder.add_output(change_output)?;
                    }
                }
                change_left = change_left.checked_sub(&Value::from(new_fee))?;
                // add potentially a separate pure ADA change output
                let left_above_minimum = change_left.coin > minimum_utxo_val;
                if builder.config.prefer_pure_change && left_above_minimum {
                    let pure_output = SingleOutputBuilderResult::new(TransactionOutput::new(
                        address.clone(),
                        change_left.clone(),
                        datum.clone(),
                        script_ref.clone(),
                    ));
                    let additional_fee = builder.fee_for_output(&pure_output)?;
                    let potential_pure_value =
                        change_left.checked_sub(&Value::from(additional_fee))?;
                    let potential_pure_above_minimum = potential_pure_value.coin > minimum_utxo_val;
                    if potential_pure_above_minimum {
                        new_fee = new_fee
                            .checked_add(additional_fee)
                            .ok_or(ArithmeticError::IntegerOverflow)?;
                        change_left = Value::zero();
                        let change_output = SingleOutputBuilderResult::new(TransactionOutput::new(
                            address.clone(),
                            potential_pure_value,
                            datum,
                            script_ref,
                        ));
                        builder.add_output(change_output)?;
                    }
                }
                builder.set_fee(new_fee);
                // add in the rest of the ADA
                if !change_left.is_zero() {
                    let last_with_remaining = builder
                        .outputs
                        .last()
                        .unwrap()
                        .amount()
                        .checked_add(&change_left)?;
                    builder
                        .outputs
                        .last_mut()
                        .unwrap()
                        .set_amount(last_with_remaining);
                }
                Ok(true)
            } else {
                let min_ada = min_ada_required(
                    &TransactionOutput::new(
                        address.clone(),
                        change_estimator.clone(),
                        datum.clone(),
                        script_ref.clone(),
                    ),
                    builder.config.coins_per_utxo_byte,
                )?;
                // no-asset case so we have no problem burning the rest if there is no other option
                fn burn_extra(
                    builder: &mut TransactionBuilder,
                    burn_amount: u64,
                ) -> Result<bool, TxBuilderError> {
                    // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
                    builder.set_fee(burn_amount);
                    Ok(false) // not enough input to covert the extra fee from adding an output so we just burn whatever is left
                }
                match change_estimator.coin >= min_ada {
                    false => burn_extra(builder, change_estimator.coin),
                    true => {
                        // check how much the fee would increase if we added a change output
                        let fee_for_change = builder.fee_for_output(
                            &SingleOutputBuilderResult::new(TransactionOutput::new(
                                address.clone(),
                                change_estimator.clone(),
                                datum.clone(),
                                script_ref.clone(),
                            )),
                        )?;

                        let new_fee = fee
                            .checked_add(fee_for_change)
                            .ok_or(ArithmeticError::IntegerOverflow)?;
                        match change_estimator.coin
                            >= min_ada
                                .checked_add(new_fee)
                                .ok_or(ArithmeticError::IntegerOverflow)?
                        {
                            false => burn_extra(builder, change_estimator.coin),
                            true => {
                                // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
                                builder.set_fee(new_fee);

                                let change_output =
                                    SingleOutputBuilderResult::new(TransactionOutput::new(
                                        address.clone(),
                                        change_estimator.checked_sub(&Value::from(new_fee))?,
                                        datum,
                                        script_ref,
                                    ));

                                builder.add_output(change_output)?;

                                Ok(true)
                            }
                        }
                    }
                }
            }
        }
        None => Err(TxBuilderError::MissingInputOrOutput),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::ops::Deref;

    use cml_core::metadata::{
        Metadata, MetadatumMap, TransactionMetadatum, TransactionMetadatumLabel,
    };
    use cml_core::Int;
    use cml_crypto::{
        Bip32PrivateKey, Bip32PublicKey, DatumHash, Deserialize, PrivateKey, RawBytesEncoding,
        TransactionHash,
    };

    use crate::address::{Address, BaseAddress, EnterpriseAddress, Pointer, PointerAddress};
    use crate::builders::witness_builder::{PartialPlutusWitness, PlutusScriptWitness};
    use crate::builders::{
        input_builder::SingleInputBuilder, mint_builder::SingleMintBuilder,
        witness_builder::NativeScriptWitnessInfo,
    };
    use crate::byron::{AddressContent, ByronAddress};
    use crate::certs::StakeCredential;
    use crate::crypto::hash::hash_transaction;
    use crate::crypto::utils::make_vkey_witness;
    use crate::genesis::network_info::{plutus_alonzo_cost_models, NetworkInfo};
    use crate::plutus::{PlutusScript, PlutusV1Script, PlutusV2Script, RedeemerTag};
    use crate::transaction::NativeScript;
    use crate::{Script, SubCoin};

    use super::*;
    use crate::builders::output_builder::TransactionOutputBuilder;

    const MAX_VALUE_SIZE: u32 = 4000;
    const MAX_TX_SIZE: u32 = 8000; // might be out of date but suffices for our tests
                                   // this is what is used in mainnet
    static COINS_PER_UTXO_BYTE: u64 = 4310;

    impl TransactionBuilder {
        fn add_change_if_needed_for_tests(
            &mut self,
            change_address: &Address,
        ) -> Result<bool, TxBuilderError> {
            choose_change_selection_algo(ChangeSelectionAlgo::Default)(self, change_address, false)
        }
    }

    fn genesis_id() -> TransactionHash {
        TransactionHash::from([0u8; TransactionHash::BYTE_COUNT])
    }

    fn root_key_15() -> Bip32PrivateKey {
        // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
        let entropy = [
            0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22,
            0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12,
        ];
        Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
    }

    fn fake_key_hash(x: u8) -> Ed25519KeyHash {
        Ed25519KeyHash::from_raw_bytes(&[
            x, 239, 181, 120, 142, 135, 19, 200, 68, 223, 211, 43, 46, 145, 222, 30, 48, 159, 239,
            255, 213, 85, 248, 39, 204, 158, 225, 100,
        ])
        .unwrap()
    }

    fn harden(index: u32) -> u32 {
        index | 0x80_00_00_00
    }

    fn byron_address() -> Address {
        ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3")
            .unwrap()
            .to_address()
    }

    fn create_linear_fee(coefficient: u64, constant: u64) -> LinearFee {
        LinearFee::new(coefficient, constant)
    }

    fn create_default_linear_fee() -> LinearFee {
        create_linear_fee(500, 2)
    }

    fn create_tx_builder_full(
        linear_fee: LinearFee,
        pool_deposit: u64,
        key_deposit: u64,
        max_val_size: u32,
        coins_per_utxo_byte: u64,
    ) -> TransactionBuilder {
        let cfg = TransactionBuilderConfigBuilder::default()
            .fee_algo(linear_fee)
            .pool_deposit(pool_deposit)
            .key_deposit(key_deposit)
            .max_value_size(max_val_size)
            .max_tx_size(MAX_TX_SIZE)
            .coins_per_utxo_byte(coins_per_utxo_byte)
            .ex_unit_prices(ExUnitPrices::new(
                SubCoin::new(577, 10000),
                SubCoin::new(721, 10000000),
            ))
            .collateral_percentage(150)
            .max_collateral_inputs(3)
            .cost_models(plutus_alonzo_cost_models())
            .build()
            .unwrap();
        TransactionBuilder::new(cfg)
    }

    fn create_tx_builder(
        linear_fee: LinearFee,
        coins_per_utxo_byte: u64,
        pool_deposit: u64,
        key_deposit: u64,
    ) -> TransactionBuilder {
        create_tx_builder_full(
            linear_fee,
            pool_deposit,
            key_deposit,
            MAX_VALUE_SIZE,
            coins_per_utxo_byte,
        )
    }

    fn create_realistic_tx_builder() -> TransactionBuilder {
        create_tx_builder(
            create_linear_fee(44, 155381),
            COINS_PER_UTXO_BYTE,
            500000000,
            2000000,
        )
    }

    fn create_tx_builder_with_fee_and_val_size(
        linear_fee: LinearFee,
        max_val_size: u32,
    ) -> TransactionBuilder {
        create_tx_builder_full(linear_fee, 1, 1, max_val_size, 1)
    }

    fn create_tx_builder_with_fee(linear_fee: LinearFee) -> TransactionBuilder {
        create_tx_builder(linear_fee, 1, 1, 1)
    }

    fn create_tx_builder_with_fee_and_pure_change(linear_fee: LinearFee) -> TransactionBuilder {
        TransactionBuilder::new(
            TransactionBuilderConfigBuilder::default()
                .fee_algo(linear_fee)
                .pool_deposit(1)
                .key_deposit(1)
                .max_value_size(MAX_VALUE_SIZE)
                .max_tx_size(MAX_TX_SIZE)
                .coins_per_utxo_byte(1)
                .ex_unit_prices(ExUnitPrices::new(SubCoin::new(0, 0), SubCoin::new(0, 0)))
                .collateral_percentage(150)
                .max_collateral_inputs(3)
                .prefer_pure_change(true)
                .build()
                .unwrap(),
        )
    }

    fn create_tx_builder_with_key_deposit(deposit: u64) -> TransactionBuilder {
        create_tx_builder(create_default_linear_fee(), 1, 1, deposit)
    }

    fn create_default_tx_builder() -> TransactionBuilder {
        create_tx_builder_with_fee(create_default_linear_fee())
    }

    fn create_account() -> (
        (Bip32PublicKey, StakeCredential),
        (Bip32PublicKey, StakeCredential),
        Address,
    ) {
        let spend = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();

        let spend_cred = StakeCredential::new_pub_key(spend.to_raw_key().hash());
        let stake_cred = StakeCredential::new_pub_key(stake.to_raw_key().hash());
        let address = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            spend_cred.clone(),
            stake_cred.clone(),
        )
        .to_address();

        ((spend, spend_cred), (stake, stake_cred), address)
    }

    #[test]
    fn build_tx_with_change() {
        let mut tx_builder = create_default_tx_builder();
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (_, stake_cred), addr_net_0) = create_account();

        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_net_0.clone(), Value::from(1_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(222)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();
        let added_change = tx_builder.add_change_if_needed_for_tests(&change_addr);
        assert!(added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 2);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::from(tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );
        assert_eq!(tx_builder.full_size().unwrap(), 285);
        assert_eq!(tx_builder.output_sizes(), vec![62, 65]);
        let _final_tx = tx_builder.build(ChangeSelectionAlgo::Default, &change_addr);
        // just test that it doesn't throw
    }

    #[test]
    fn build_tx_without_change() {
        let mut tx_builder = create_default_tx_builder();
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (_, stake_cred), addr_net_0) = create_account();
        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_net_0.clone(), Value::from(1_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(880_000)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();
        let added_change = tx_builder.add_change_if_needed_for_tests(&change_addr);
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::from(tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );
        let _final_tx = tx_builder.build(ChangeSelectionAlgo::Default, &change_addr);
        // just test that it doesn't throw
    }

    #[test]
    fn build_tx_with_certs() {
        let mut tx_builder = create_tx_builder_with_key_deposit(1_000_000);
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (stake, stake_cred), addr_net_0) = create_account();

        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_net_0, Value::from(5_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();
        tx_builder.set_ttl(1000);

        let cert =
            SingleCertificateBuilder::new(Certificate::new_stake_registration(stake_cred.clone()))
                .payment_key()
                .unwrap();
        tx_builder.add_cert(cert);

        let cert = SingleCertificateBuilder::new(Certificate::new_stake_delegation(
            stake_cred.clone(),
            stake.to_raw_key().hash(), // in reality, this should be the pool owner's key, not ours
        ))
        .payment_key()
        .unwrap();
        tx_builder.add_cert(cert);

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();
        tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert_eq!(tx_builder.min_fee(false).unwrap(), 214002);
        assert_eq!(tx_builder.get_fee_if_set().unwrap(), 214002);
        assert_eq!(tx_builder.get_deposit().unwrap(), 1000000);
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::from(tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
                .checked_add(&Value::from(tx_builder.get_deposit().unwrap()))
                .unwrap()
        );
        let _final_tx = tx_builder.build(ChangeSelectionAlgo::Default, &change_addr);
        // just test that it doesn't throw
    }

    #[test]
    fn build_tx_exact_amount() {
        // transactions where sum(input) == sum(output) exact should pass
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(0, 0));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (_, stake_cred), addr_net_0) = create_account();

        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_net_0.clone(), Value::from(222), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(222)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(0);

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();
        let added_change = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(!added_change);
        let final_tx = tx_builder.build_body().unwrap();
        assert_eq!(final_tx.outputs.len(), 1);
    }

    #[test]
    fn build_tx_exact_change() {
        // transactions where we have exactly enough ADA to add change should pass
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(0, 0));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (_, stake_cred), addr_net_0) = create_account();

        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_net_0.clone(), Value::from(542), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(222)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(0);

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();
        let added_change = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(added_change);
        let final_tx = tx_builder.build_body().unwrap();
        assert_eq!(final_tx.outputs.len(), 2);
        assert_eq!(final_tx.outputs[1].amount().coin, 320);
    }

    #[test]
    #[should_panic]
    fn build_tx_insufficient_deposit() {
        // transactions should fail with insufficient fees if a deposit is required
        let mut tx_builder = create_tx_builder_with_key_deposit(5);
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (_, stake_cred), addr_net_0) = create_account();

        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_net_0.clone(), Value::from(5), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(5)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(0);

        // add a cert which requires a deposit
        let cert =
            SingleCertificateBuilder::new(Certificate::new_stake_registration(stake_cred.clone()))
                .payment_key()
                .unwrap();
        tx_builder.add_cert(cert);

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();

        tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
    }

    #[test]
    fn build_tx_with_inputs() {
        let mut tx_builder = create_default_tx_builder();
        let ((spend, spend_cred), (_, stake_cred), _) = create_account();

        let input = {
            let address =
                EnterpriseAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone())
                    .to_address();
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, Value::from(1_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        assert_eq!(tx_builder.fee_for_input(&input).unwrap(), 69500);
        tx_builder.add_input(input).unwrap();

        let input = {
            let address = BaseAddress::new(
                NetworkInfo::testnet().network_id(),
                spend_cred.clone(),
                stake_cred,
            )
            .to_address();
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, Value::from(1_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        let input = {
            let address = PointerAddress::new(
                NetworkInfo::testnet().network_id(),
                spend_cred,
                Pointer::new(0, 0, 0),
            )
            .to_address();
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, Value::from(1_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        let input = {
            let address =
                AddressContent::icarus_from_key(spend, NetworkInfo::testnet().protocol_magic())
                    .to_address()
                    .to_address();
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, Value::from(1_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        assert_eq!(tx_builder.inputs.len(), 4);
    }

    #[test]
    fn build_tx_with_mint_all_sent() {
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(0, 1));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let ((_, spend_cred), (_, stake_cred), _) = create_account();

        let input = {
            let address =
                EnterpriseAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone())
                    .to_address();
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, Value::from(764), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            spend_cred,
            stake_cred.clone(),
        )
        .to_address();

        let (min_script, policy_id) = mint_script_and_policy(0);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let amount = 1234u64;

        let result = SingleMintBuilder::new_single_asset(name.clone(), amount as i64)
            .native_script(
                min_script,
                NativeScriptWitnessInfo::assume_signature_count(),
            );

        tx_builder.add_mint(result).unwrap();

        let mut mass = MultiAsset::new();
        mass.set(policy_id, name, amount);

        // One coin and the minted asset goes into the output
        let mut output_amount = Value::from(264);
        output_amount.multiasset = mass;

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();

        let added_change = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(added_change);
        assert_eq!(tx_builder.outputs.len(), 2);

        // Change must be one remaining coin because fee is one constant coin
        let change = tx_builder.outputs[1].amount();
        assert_eq!(change.coin, 499);
        assert!(!change.has_multiassets());
    }

    #[test]
    fn build_tx_with_mint_in_change() {
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(0, 1));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let ((_spend, spend_cred), (_, stake_cred), _) = create_account();

        let input = {
            let address =
                EnterpriseAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone())
                    .to_address();
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, Value::from(564), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            spend_cred,
            stake_cred.clone(),
        )
        .to_address();

        let (min_script, policy_id) = mint_script_and_policy(0);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let amount_minted = 1000i64;
        let amount_sent = 500u64;

        let result = SingleMintBuilder::new_single_asset(name.clone(), amount_minted)
            .native_script(
                min_script,
                NativeScriptWitnessInfo::assume_signature_count(),
            );

        tx_builder.add_mint(result).unwrap();

        let mut mass = MultiAsset::new();
        mass.set(policy_id, name.clone(), amount_sent);

        // One coin and the minted asset goes into the output
        let mut output_amount = Value::from(264);
        output_amount.multiasset = mass;

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();

        let added_change = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(added_change);
        assert_eq!(tx_builder.outputs.len(), 2);

        // Change must be one remaining coin because fee is one constant coin
        let change = tx_builder.outputs[1].amount();
        assert_eq!(change.coin, 299);
        assert!(change.has_multiassets());

        assert_eq!(
            change.multiasset.get(&policy_id, &name).unwrap() as i128,
            amount_minted.checked_sub(amount_sent as i64).unwrap() as i128,
        );
    }

    #[test]
    fn build_tx_with_native_assets_change() {
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(0, 1));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (_, stake_cred), addr_net_0) = create_account();

        let policy_id = PolicyId::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let ma_input1 = 100;
        let ma_input2 = 200;
        let ma_output1 = 60;

        let multiassets = [ma_input1, ma_input2, ma_output1]
            .iter()
            .map(|input| {
                let mut multiasset = MultiAsset::new();
                multiasset.set(policy_id, name.clone(), *input);
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets.iter().zip([500u64, 500].iter().cloned()) {
            let mut input_amount = Value::from(ada);
            input_amount.multiasset = multiasset.clone();

            let input = {
                SingleInputBuilder::new(
                    TransactionInput::new(genesis_id(), 0),
                    TransactionOutput::new(addr_net_0.clone(), input_amount, None, None),
                )
                .payment_key()
                .unwrap()
            };
            tx_builder.add_input(input).unwrap();
        }

        let mut output_amount = Value::from(263);
        output_amount.multiasset = multiassets[2].clone();

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();

        let added_change = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(added_change);
        let final_tx = tx_builder.build_body().unwrap();
        assert_eq!(final_tx.outputs.len(), 2);
        assert_eq!(
            final_tx.outputs[1]
                .amount()
                .multiasset
                .get(&policy_id, &name)
                .unwrap(),
            ma_input1 + ma_input2 - ma_output1
        );
        assert_eq!(final_tx.outputs[1].amount().coin, 736);
    }

    #[test]
    fn build_tx_with_native_assets_change_and_purification() {
        let coin_per_utxo_byte = 1;
        // Prefer pure change!
        let mut tx_builder = create_tx_builder_with_fee_and_pure_change(create_linear_fee(0, 1));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (_, stake_cred), addr_net_0) = create_account();

        let policy_id = &PolicyId::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let ma_input1 = 100;
        let ma_input2 = 200;
        let ma_output1 = 60;

        let multiassets = [ma_input1, ma_input2, ma_output1]
            .iter()
            .map(|input| {
                let mut multiasset = MultiAsset::new();
                multiasset.set(*policy_id, name.clone(), *input);
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets.iter().zip([500u64, 500].iter().cloned()) {
            let mut input_amount = Value::from(ada);
            input_amount.multiasset = multiasset.clone();

            let input = {
                SingleInputBuilder::new(
                    TransactionInput::new(genesis_id(), 0),
                    TransactionOutput::new(addr_net_0.clone(), input_amount, None, None),
                )
                .payment_key()
                .unwrap()
            };
            tx_builder.add_input(input).unwrap();
        }

        let mut output_amount = Value::from(263);
        output_amount.multiasset = multiassets[2].clone();

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();

        let added_change = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(added_change);
        let final_tx = tx_builder.build_body().unwrap();
        assert_eq!(final_tx.outputs.len(), 3);
        assert_eq!(final_tx.outputs[0].amount().coin, 263);
        assert_eq!(
            final_tx.outputs[1]
                .amount()
                .multiasset
                .get(policy_id, &name)
                .unwrap(),
            ma_input1 + ma_input2 - ma_output1
        );
        // The first change output that contains all the tokens contain minimum required Coin
        let min_coin_for_dirty_change =
            min_ada_required(&final_tx.outputs[1], coin_per_utxo_byte).unwrap();
        assert_eq!(final_tx.outputs[1].amount().coin, min_coin_for_dirty_change);
        assert_eq!(final_tx.outputs[2].amount().coin, 473);
        assert!(!final_tx.outputs[2].amount().has_multiassets());
    }

    #[test]
    fn build_tx_with_native_assets_change_and_no_purification_cuz_not_enough_pure_coin() {
        // Prefer pure change!
        let mut tx_builder = create_tx_builder_with_fee_and_pure_change(create_linear_fee(1, 1));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (_, stake_cred), addr_net_0) = create_account();

        let policy_id = &PolicyId::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let ma_input1 = 100;
        let ma_input2 = 200;
        let ma_output1 = 60;

        let multiassets = [ma_input1, ma_input2, ma_output1]
            .iter()
            .map(|input| {
                let mut multiasset = MultiAsset::new();
                multiasset.set(*policy_id, name.clone(), *input);
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets.iter().zip([500u64, 500].iter().cloned()) {
            let mut input_amount = Value::from(ada);
            input_amount.multiasset = multiasset.clone();

            let input = {
                SingleInputBuilder::new(
                    TransactionInput::new(genesis_id(), 0),
                    TransactionOutput::new(addr_net_0.clone(), input_amount, None, None),
                )
                .payment_key()
                .unwrap()
            };
            tx_builder.add_input(input).unwrap();
        }

        let output_amount = Value::new(263, multiassets[2].clone());

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();

        let added_change = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(added_change);
        let final_tx = tx_builder.build_body().unwrap();
        assert_eq!(final_tx.outputs.len(), 2);
        assert_eq!(final_tx.outputs[0].amount().coin, 263);
        assert_eq!(
            final_tx.outputs[1]
                .amount()
                .multiasset
                .get(policy_id, &name)
                .unwrap(),
            ma_input1 + ma_input2 - ma_output1
        );
        // The single change output contains more Coin then minimal utxo value
        // But not enough to cover the additional fee for a separate output
        assert_eq!(final_tx.outputs[1].amount().coin, 336);
    }

    #[test]
    #[should_panic]
    fn build_tx_leftover_assets() {
        let mut tx_builder = create_default_tx_builder();
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (_, stake_cred), addr_net_0) = create_account();

        // add an input that contains an asset not present in the output
        let policy_id = PolicyId::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let input_amount = {
            let mut input_multiasset = MultiAsset::new();
            input_multiasset.set(policy_id, name, 100);
            Value::new(1_000_000, input_multiasset)
        };
        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_net_0.clone(), input_amount, None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(880_000)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();
        let added_change = tx_builder.add_change_if_needed_for_tests(&change_addr);
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::from(tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );
        let _final_tx = tx_builder.build_body(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_burn_less_than_min_ada() {
        // with this mainnet value we should end up with a final min_ada_required of just under 1_000_000
        let mut tx_builder = create_realistic_tx_builder();

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap();
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(output_addr.to_address())
                    .next()
                    .unwrap()
                    .with_value(Value::from(2_000_000))
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let input = {
            let address = ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address();
            let builder = SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, Value::from(2_400_000), None, None),
            );
            builder.payment_key().unwrap()
        };
        tx_builder.add_input(input).unwrap();

        tx_builder.set_ttl(1);

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap();
        let added_change = tx_builder.add_change_if_needed_for_tests(&change_addr.to_address());
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::from(tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );
        let _final_tx = tx_builder.build_body(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_burn_empty_assets() {
        let mut tx_builder = create_realistic_tx_builder();

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap();
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(output_addr.to_address())
                    .next()
                    .unwrap()
                    .with_value(Value::from(2_000_000))
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let input_value = Value::from(2_400_000);
        let input = {
            let address = ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address();
            let builder = SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, input_value, None, None),
            );
            builder.payment_key().unwrap()
        };
        tx_builder.add_input(input).unwrap();

        tx_builder.set_ttl(1);

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap();
        let added_change = tx_builder.add_change_if_needed_for_tests(&change_addr.to_address());
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap()
                .coin,
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::from(tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
                .coin
        );
        let _final_tx = tx_builder.build_body(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_no_useless_multiasset() {
        let mut tx_builder = create_realistic_tx_builder();

        let policy_id = PolicyId::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        // add an output that uses up all the token but leaves ADA
        let input_amount = {
            let mut input_multiasset = MultiAsset::new();
            input_multiasset.set(policy_id, name.clone(), 100);
            Value::new(5_000_000, input_multiasset)
        };

        let input = {
            let address = ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address();
            let builder = SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, input_amount, None, None),
            );
            builder.payment_key().unwrap()
        };
        tx_builder.add_input(input).unwrap();

        // add an input that contains an asset & ADA
        let output_amount = {
            let mut output_multiasset = MultiAsset::new();
            output_multiasset.set(policy_id, name, 100);
            Value::new(2_000_000, output_multiasset)
        };

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap();
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(output_addr.to_address())
                    .next()
                    .unwrap()
                    .with_value(output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        tx_builder.set_ttl(1);

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap();
        let added_change = tx_builder.add_change_if_needed_for_tests(&change_addr.to_address());
        assert!(added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 2);
        let final_tx = tx_builder.build_body().unwrap();
        let change_output = &final_tx.outputs[1];

        // since all tokens got sent in the output
        // the change should be only ADA and not have any multiasset struct in it
        assert!(!change_output.amount().has_multiassets());
    }

    fn create_multiasset() -> (MultiAsset, [ScriptHash; 3], [AssetName; 3]) {
        let policy_ids = [
            PolicyId::from([0u8; 28]),
            PolicyId::from([1u8; 28]),
            PolicyId::from([2u8; 28]),
        ];
        let names = [
            AssetName::new(vec![99u8; 32]).unwrap(),
            AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
            AssetName::new(vec![4u8, 5, 6, 7, 8, 9]).unwrap(),
        ];
        let multiasset = policy_ids.iter().zip(names.iter()).fold(
            MultiAsset::new(),
            |mut acc, (policy_id, name)| {
                acc.set(*policy_id, name.clone(), 500);
                acc
            },
        );
        (multiasset, policy_ids, names)
    }

    #[test]
    fn build_tx_add_change_split_nfts() {
        let max_value_size = 100; // super low max output size to test with fewer assets
        let mut tx_builder =
            create_tx_builder_with_fee_and_val_size(create_linear_fee(0, 1), max_value_size);

        let (multiasset, policy_ids, names) = create_multiasset();

        let mut input_value = Value::from(1000);
        input_value.multiasset = multiasset;

        let input = {
            let address = ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address();
            let builder = SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, input_value, None, None),
            );
            builder.payment_key().unwrap()
        };
        tx_builder.add_input(input).unwrap();

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap()
        .to_address();
        let output_amount = Value::from(208);

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(output_addr)
                    .next()
                    .unwrap()
                    .with_value(output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();

        let added_change = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(added_change);
        let final_tx = tx_builder.build_body().unwrap();
        assert_eq!(final_tx.outputs.len(), 3);
        for (policy_id, asset_name) in policy_ids.iter().zip(names.iter()) {
            assert!(final_tx.outputs.iter().any(|output| output
                .amount()
                .multiasset
                .iter()
                .any(|(pid, a)| pid == policy_id && a.iter().any(|(name, _)| name == asset_name))));
        }
        for output in final_tx.outputs.iter() {
            assert!(output.amount().to_cbor_bytes().len() <= max_value_size as usize);
        }
    }

    #[test]
    fn build_tx_too_big_output() {
        let mut tx_builder = create_tx_builder_with_fee_and_val_size(create_linear_fee(0, 1), 10);

        let input = {
            let address = ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address();
            let builder = SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, Value::from(500), None, None),
            );
            builder.payment_key().unwrap()
        };
        tx_builder.add_input(input).unwrap();

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap()
        .to_address();
        let mut output_amount = Value::from(50);
        output_amount.multiasset = create_multiasset().0;

        assert!(tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(output_addr)
                    .next()
                    .unwrap()
                    .with_value(output_amount)
                    .build()
                    .unwrap()
            )
            .is_err());
    }

    #[test]
    fn build_tx_add_change_nfts_not_enough_ada() {
        let mut tx_builder = create_tx_builder_with_fee_and_val_size(
            create_linear_fee(0, 1),
            150, // super low max output size to test with fewer assets
        );

        let policy_ids = [
            PolicyId::from([0u8; 28]),
            PolicyId::from([1u8; 28]),
            PolicyId::from([2u8; 28]),
        ];
        let names = [
            AssetName::new(vec![99u8; 32]).unwrap(),
            AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
            AssetName::new(vec![4u8, 5, 6, 7, 8, 9]).unwrap(),
        ];

        let multiasset = policy_ids.iter().zip(names.iter()).fold(
            MultiAsset::new(),
            |mut acc, (policy_id, name)| {
                acc.set(*policy_id, name.clone(), 500);
                acc
            },
        );

        let mut input_value = Value::from(58);
        input_value.multiasset = multiasset;

        let input = {
            let address = ByronAddress::from_base58(
                "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
            )
            .unwrap()
            .to_address();
            let builder = SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, input_value, None, None),
            );
            builder.payment_key().unwrap()
        };
        tx_builder.add_input(input).unwrap();

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap()
        .to_address();
        let output_amount = Value::from(208);

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(output_addr)
                    .next()
                    .unwrap()
                    .with_value(output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();

        assert!(tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .is_err())
    }

    fn make_input(input_hash_byte: u8, value: Value) -> InputBuilderResult {
        let (_, _, address) = create_account();
        SingleInputBuilder::new(
            TransactionInput::new(TransactionHash::from([input_hash_byte; 32]), 0),
            TransactionOutputBuilder::new()
                .with_address(address)
                .next()
                .unwrap()
                .with_value(value)
                .build()
                .unwrap()
                .output,
        )
        .payment_key()
        .unwrap()
    }

    #[test]
    fn tx_builder_cip2_largest_first_increasing_fees() {
        // we have a = 1 to test increasing fees when more inputs are added
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(1, 0));
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(
                        Address::from_bech32(
                            "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                        )
                        .unwrap(),
                    )
                    .next()
                    .unwrap()
                    .with_value(10000)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.add_utxo(make_input(0u8, Value::from(1500)));
        tx_builder.add_utxo(make_input(1u8, Value::from(2000)));
        tx_builder.add_utxo(make_input(2u8, Value::from(8000)));
        tx_builder.add_utxo(make_input(3u8, Value::from(4000)));
        tx_builder.add_utxo(make_input(4u8, Value::from(1000)));
        tx_builder
            .select_utxos(CoinSelectionStrategyCIP2::LargestFirst)
            .unwrap();
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
        let change_added = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(change_added);
        let tx = tx_builder.build_body().unwrap();
        // change needed
        assert_eq!(2, tx.outputs.len());
        assert_eq!(2, tx.inputs.len());
        // confirm order of only what is necessary
        assert_eq!(2u8, tx.inputs[0].transaction_id.to_raw_bytes()[0]);
        assert_eq!(3u8, tx.inputs[1].transaction_id.to_raw_bytes()[0]);
    }

    #[test]
    fn tx_builder_cip2_largest_first_static_fees() {
        // we have a = 0 so we know adding inputs/outputs doesn't change the fee so we can analyze more
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(0, 0));
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(
                        Address::from_bech32(
                            "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                        )
                        .unwrap(),
                    )
                    .next()
                    .unwrap()
                    .with_value(1200)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.add_utxo(make_input(0u8, Value::from(150)));
        tx_builder.add_utxo(make_input(1u8, Value::from(200)));
        tx_builder.add_utxo(make_input(2u8, Value::from(800)));
        tx_builder.add_utxo(make_input(3u8, Value::from(400)));
        tx_builder.add_utxo(make_input(4u8, Value::from(100)));
        tx_builder
            .select_utxos(CoinSelectionStrategyCIP2::LargestFirst)
            .unwrap();
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
        let change_added = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(!change_added);
        let tx = tx_builder.build_body().unwrap();
        // change not needed - should be exact
        assert_eq!(1, tx.outputs.len());
        assert_eq!(2, tx.inputs.len());
        // confirm order of only what is necessary
        assert_eq!(2u8, tx.inputs[0].transaction_id.to_raw_bytes()[0]);
        assert_eq!(3u8, tx.inputs[1].transaction_id.to_raw_bytes()[0]);
    }

    #[test]
    fn tx_builder_cip2_largest_first_multiasset() {
        // we have a = 0 so we know adding inputs/outputs doesn't change the fee so we can analyze more
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(0, 0));
        let pid1 = PolicyId::from([1u8; 28]);
        let pid2 = PolicyId::from([2u8; 28]);
        let asset_name1 = AssetName::new(vec![1u8; 8]).unwrap();
        let asset_name2 = AssetName::new(vec![2u8; 11]).unwrap();
        let asset_name3 = AssetName::new(vec![3u8; 9]).unwrap();

        let mut output_value = Value::from(415);
        let mut output_ma = MultiAsset::new();
        output_ma.set(pid1, asset_name1.clone(), 5);
        output_ma.set(pid1, asset_name2.clone(), 1);
        output_ma.set(pid2, asset_name2.clone(), 2);
        output_ma.set(pid2, asset_name3.clone(), 4);
        output_value.multiasset = output_ma;
        tx_builder
            .add_output(SingleOutputBuilderResult::new(TransactionOutput::new(
                Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z")
                    .unwrap(),
                output_value.clone(),
                None,
                None,
            )))
            .unwrap();

        // should not be taken
        tx_builder.add_utxo(make_input(0u8, Value::from(150)));

        // should not be taken
        let mut ma1 = MultiAsset::new();
        ma1.set(pid1, asset_name1.clone(), 10);
        ma1.set(pid1, asset_name2.clone(), 1);
        ma1.set(pid2, asset_name2.clone(), 2);
        let input1 = make_input(1u8, Value::new(200, ma1));
        tx_builder.add_utxo(input1);

        // taken first to satisfy pid1:asset_name1 (but also satisfies pid2:asset_name3)
        let mut ma2 = MultiAsset::new();
        ma2.set(pid1, asset_name1.clone(), 20);
        ma2.set(pid2, asset_name3.clone(), 4);
        let input2 = make_input(2u8, Value::new(10, ma2));
        tx_builder.add_utxo(input2.clone());

        // taken second to satisfy pid1:asset_name2 (but also satisfies pid2:asset_name1)

        let mut ma3 = MultiAsset::new();
        ma3.set(pid2, asset_name1.clone(), 5);
        ma3.set(pid1, asset_name2.clone(), 15);
        let input3 = make_input(3u8, Value::new(50, ma3));
        tx_builder.add_utxo(input3.clone());

        // should not be taken either

        let mut ma4 = MultiAsset::new();
        ma4.set(pid1, asset_name1.clone(), 10);
        ma4.set(pid1, asset_name2.clone(), 10);
        let input4 = make_input(4u8, Value::new(10, ma4));
        tx_builder.add_utxo(input4);

        // taken third to satisfy pid2:asset_name_2
        let mut ma5 = MultiAsset::new();
        ma5.set(pid1, asset_name2.clone(), 10);
        ma5.set(pid2, asset_name2.clone(), 3);
        let input5 = make_input(5u8, Value::new(10, ma5));
        tx_builder.add_utxo(input5.clone());

        // should be taken to get enough ADA
        let input6 = make_input(6u8, Value::from(700));
        tx_builder.add_utxo(input6.clone());

        // should not be taken
        tx_builder.add_utxo(make_input(7u8, Value::from(100)));
        tx_builder
            .select_utxos(CoinSelectionStrategyCIP2::LargestFirstMultiAsset)
            .unwrap();
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
        let change_added = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(change_added);
        let tx = tx_builder.build_body().unwrap();

        assert_eq!(2, tx.outputs.len());
        assert_eq!(4, tx.inputs.len());
        // check order expected per-asset
        assert_eq!(2u8, tx.inputs[0].transaction_id.to_raw_bytes()[0]);
        assert_eq!(3u8, tx.inputs[1].transaction_id.to_raw_bytes()[0]);
        assert_eq!(5u8, tx.inputs[2].transaction_id.to_raw_bytes()[0]);
        assert_eq!(6u8, tx.inputs[3].transaction_id.to_raw_bytes()[0]);

        let change = tx.outputs[1].amount();
        assert_eq!(change.coin, 355);
        let change_ma = &change.multiasset;
        assert_eq!(15, change_ma.get(&pid1, &asset_name1).unwrap());
        assert_eq!(24, change_ma.get(&pid1, &asset_name2).unwrap());
        assert_eq!(1, change_ma.get(&pid2, &asset_name2).unwrap());
        assert_eq!(0, change_ma.get(&pid2, &asset_name3).unwrap_or_default());
        let expected_input = input2
            .utxo_info
            .amount()
            .checked_add(input3.utxo_info.amount())
            .unwrap()
            .checked_add(input5.utxo_info.amount())
            .unwrap()
            .checked_add(input6.utxo_info.amount())
            .unwrap();
        let expected_change = expected_input.checked_sub(&output_value).unwrap();
        assert_eq!(expected_change, *change);
    }

    #[test]
    #[flaky_test::flaky_test]
    fn tx_builder_cip2_random_improve_multiasset() {
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(0, 0));
        let pid1 = PolicyId::from([1u8; 28]);
        let pid2 = PolicyId::from([2u8; 28]);
        let asset_name1 = AssetName::new(vec![1u8; 8]).unwrap();
        let asset_name2 = AssetName::new(vec![2u8; 11]).unwrap();
        let asset_name3 = AssetName::new(vec![3u8; 9]).unwrap();

        let mut output_ma = MultiAsset::new();
        output_ma.set(pid1, asset_name1.clone(), 5);
        output_ma.set(pid1, asset_name2.clone(), 1);
        output_ma.set(pid2, asset_name2.clone(), 2);
        output_ma.set(pid2, asset_name3.clone(), 4);
        let output_value = Value::new(415, output_ma);
        tx_builder
            .add_output(SingleOutputBuilderResult::new(TransactionOutput::new(
                Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z")
                    .unwrap(),
                output_value.clone(),
                None,
                None,
            )))
            .unwrap();

        tx_builder.add_utxo(make_input(0u8, Value::from(150)));

        let mut ma1 = MultiAsset::new();
        ma1.set(pid1, asset_name1.clone(), 10);
        ma1.set(pid1, asset_name2.clone(), 1);
        ma1.set(pid2, asset_name2.clone(), 2);
        let input1 = make_input(1u8, Value::new(200, ma1));
        tx_builder.add_utxo(input1);

        let mut ma2 = MultiAsset::new();
        ma2.set(pid1, asset_name1.clone(), 20);
        ma2.set(pid2, asset_name3.clone(), 4);
        let input2 = make_input(2u8, Value::new(10, ma2));
        tx_builder.add_utxo(input2);

        let mut ma3 = MultiAsset::new();
        ma3.set(pid2, asset_name1.clone(), 5);
        ma3.set(pid1, asset_name2.clone(), 15);
        let input3 = make_input(3u8, Value::new(50, ma3));
        tx_builder.add_utxo(input3);

        let mut ma4 = MultiAsset::new();
        ma4.set(pid1, asset_name1, 10);
        ma4.set(pid1, asset_name2.clone(), 10);
        let input4 = make_input(4u8, Value::new(10, ma4));
        tx_builder.add_utxo(input4);

        let mut ma5 = MultiAsset::new();
        ma5.set(pid1, asset_name2.clone(), 10);
        ma5.set(pid2, asset_name2.clone(), 3);
        let input5 = make_input(5u8, Value::new(10, ma5));
        tx_builder.add_utxo(input5);

        let input6 = make_input(6u8, Value::from(400));
        tx_builder.add_utxo(input6);
        tx_builder.add_utxo(make_input(7u8, Value::from(100)));

        let mut ma8 = MultiAsset::new();
        ma8.set(pid2, asset_name2, 10);
        let input8 = make_input(8u8, Value::new(10, ma8));
        tx_builder.add_utxo(input8);

        let mut ma9 = MultiAsset::new();
        ma9.set(pid2, asset_name3, 10);
        let input9 = make_input(9u8, Value::new(10, ma9));
        tx_builder.add_utxo(input9);

        tx_builder
            .select_utxos(CoinSelectionStrategyCIP2::RandomImproveMultiAsset)
            .unwrap();
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
        let change_added = tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert!(change_added);
        let tx = tx_builder.build_body().unwrap();

        assert_eq!(2, tx.outputs.len());

        let input_total = tx_builder.get_explicit_input().unwrap();
        assert!(input_total >= output_value);
    }

    #[test]
    #[flaky_test::flaky_test]
    fn tx_builder_cip2_random_improve() {
        // we have a = 1 to test increasing fees when more inputs are added
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(1, 0));
        const COST: u64 = 10000;
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(
                        Address::from_bech32(
                            "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                        )
                        .unwrap(),
                    )
                    .next()
                    .unwrap()
                    .with_value(COST)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.utxos.push(make_input(0u8, Value::from(1500)));
        tx_builder.utxos.push(make_input(1u8, Value::from(2000)));
        tx_builder.utxos.push(make_input(2u8, Value::from(8000)));
        tx_builder.utxos.push(make_input(3u8, Value::from(4000)));
        tx_builder.utxos.push(make_input(4u8, Value::from(1000)));
        tx_builder.utxos.push(make_input(5u8, Value::from(2000)));
        tx_builder.utxos.push(make_input(6u8, Value::from(1500)));
        let add_inputs_res = tx_builder.select_utxos(CoinSelectionStrategyCIP2::RandomImprove);
        assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
        let add_change_res = tx_builder.add_change_if_needed_for_tests(&change_addr);
        assert!(add_change_res.is_ok(), "{:?}", add_change_res.err());
        let tx_build_res = tx_builder.build_body();
        assert!(tx_build_res.is_ok(), "{:?}", tx_build_res.err());
        let tx = tx_build_res.unwrap();
        // we need to look up the values to ensure there's enough
        let mut input_values = BTreeMap::new();
        for utxo in tx_builder.utxos.iter() {
            input_values.insert(utxo.input.transaction_id, utxo.utxo_info.amount().clone());
        }
        let mut encountered = std::collections::HashSet::new();
        let mut input_total = Value::from(Coin::zero());
        for input in tx.inputs.iter() {
            let txid = &input.transaction_id;
            if !encountered.insert(*txid) {
                panic!("Input {:?} duplicated", txid);
            }
            let value = input_values.get(txid).unwrap();
            input_total = input_total.checked_add(value).unwrap();
        }
        assert!(
            input_total
                >= Value::from(
                    tx_builder
                        .min_fee(false)
                        .unwrap()
                        .checked_add(COST)
                        .unwrap()
                )
        );
    }

    #[test]
    #[flaky_test::flaky_test]
    fn tx_builder_cip2_random_improve_exclude_used_indices() {
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(44, 155381));
        const COST: u64 = 1000000;
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(
                        Address::from_bech32(
                            "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                        )
                        .unwrap(),
                    )
                    .next()
                    .unwrap()
                    .with_value(COST)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.add_utxo(make_input(0u8, Value::from(1000000)));
        tx_builder.add_utxo(make_input(1u8, Value::from(10000000)));
        let mut input_total = tx_builder.get_total_input().unwrap();
        let mut output_total = tx_builder
            .get_explicit_output()
            .unwrap()
            .checked_add(&Value::from(tx_builder.get_deposit().unwrap()))
            .unwrap()
            .checked_add(&Value::from(tx_builder.min_fee(false).unwrap()))
            .unwrap();
        let available_inputs = tx_builder.utxos.clone();
        let mut available_indices: BTreeSet<usize> = (0..available_inputs.len()).collect();
        assert!(available_indices.len() == 2);
        use rand::SeedableRng;
        let mut rng = rand_chacha::ChaChaRng::seed_from_u64(1);
        tx_builder
            .cip2_random_improve_by(
                &available_inputs,
                &mut available_indices,
                &mut input_total,
                &mut output_total,
                |value| Some(value.coin),
                &mut rng,
            )
            .unwrap();
        assert!(!available_indices.contains(&0));
        assert!(available_indices.contains(&1));
        assert!(available_indices.len() < 2);
    }

    #[test]
    #[flaky_test::flaky_test]
    fn tx_builder_cip2_random_improve_when_using_all_available_inputs() {
        // we have a = 1 to test increasing fees when more inputs are added
        let linear_fee = LinearFee::new(1, 0);
        let cfg = TransactionBuilderConfigBuilder::default()
            .fee_algo(linear_fee)
            .pool_deposit(0)
            .key_deposit(0)
            .max_value_size(9999)
            .max_tx_size(9999)
            .coins_per_utxo_byte(Coin::zero())
            .ex_unit_prices(ExUnitPrices::new(
                SubCoin::new(u64::zero(), u64::zero()),
                SubCoin::new(u64::zero(), u64::zero()),
            ))
            .collateral_percentage(150)
            .max_collateral_inputs(3)
            .build()
            .unwrap();
        let mut tx_builder = TransactionBuilder::new(cfg);
        const COST: u64 = 1000;
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(
                        Address::from_bech32(
                            "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                        )
                        .unwrap(),
                    )
                    .next()
                    .unwrap()
                    .with_value(COST)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.add_utxo(make_input(1u8, Value::from(800)));
        tx_builder.add_utxo(make_input(2u8, Value::from(800)));
        let add_inputs_res = tx_builder.select_utxos(CoinSelectionStrategyCIP2::RandomImprove);
        assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
    }

    #[test]
    #[flaky_test::flaky_test]
    fn tx_builder_cip2_random_improve_adds_enough_for_fees() {
        // we have a = 1 to test increasing fees when more inputs are added
        let linear_fee = LinearFee::new(1, 0);
        let cfg = TransactionBuilderConfigBuilder::default()
            .fee_algo(linear_fee)
            .pool_deposit(0)
            .key_deposit(0)
            .max_value_size(9999)
            .max_tx_size(9999)
            .coins_per_utxo_byte(Coin::zero())
            .ex_unit_prices(ExUnitPrices::new(SubCoin::new(0, 0), SubCoin::new(0, 0)))
            .collateral_percentage(150)
            .max_collateral_inputs(3)
            .build()
            .unwrap();
        let mut tx_builder = TransactionBuilder::new(cfg);
        const COST: u64 = 100;
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(
                        Address::from_bech32(
                            "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z",
                        )
                        .unwrap(),
                    )
                    .next()
                    .unwrap()
                    .with_value(COST)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(tx_builder.min_fee(false).unwrap(), 53);
        tx_builder.add_utxo(make_input(1u8, Value::from(150)));
        tx_builder.add_utxo(make_input(2u8, Value::from(150)));
        tx_builder.add_utxo(make_input(3u8, Value::from(150)));
        let add_inputs_res = tx_builder.select_utxos(CoinSelectionStrategyCIP2::RandomImprove);
        assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
        assert_eq!(tx_builder.min_fee(false).unwrap(), 264);
        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();
        let add_change_res = tx_builder.add_change_if_needed_for_tests(&change_addr);
        assert!(add_change_res.is_ok(), "{:?}", add_change_res.err());
    }

    #[test]
    fn build_tx_pay_to_multisig() {
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(10, 2));
        let (_, _, addr_net_0) = create_account();

        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_net_0.clone(), Value::from(1_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(999_000)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);
        tx_builder.set_fee(1_000);

        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::from(tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );

        let final_tx = tx_builder.build_body().unwrap();
        let deser_t = TransactionBody::from_cbor_bytes(&final_tx.to_cbor_bytes()).unwrap();

        assert_eq!(deser_t.to_cbor_bytes(), final_tx.to_cbor_bytes());
    }

    #[test]
    fn build_tx_multisig_spend_1on1_unsigned() {
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(10, 2));

        let ((spend, _), (_, stake_cred), addr_multisig) = create_account();
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let addr_output =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();

        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_multisig, Value::from(1_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_output)
                    .next()
                    .unwrap()
                    .with_value(999_000)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);
        tx_builder.set_fee(1_000);

        let mut auxiliary_data = AuxiliaryData::new();
        let mut pubkey_native_scripts = Vec::new();
        let mut oneof_native_scripts = Vec::new();

        let spending_hash = spend.to_raw_key().hash();
        pubkey_native_scripts.push(NativeScript::new_script_pubkey(spending_hash));
        oneof_native_scripts.push(NativeScript::new_script_n_of_k(1, pubkey_native_scripts));
        auxiliary_data.add_native_scripts(oneof_native_scripts);
        tx_builder.add_auxiliary_data(auxiliary_data.clone());

        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::from(tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );

        let final_tx = tx_builder.build_body().unwrap();
        let deser_t = TransactionBody::from_cbor_bytes(&final_tx.to_cbor_bytes()).unwrap();

        assert_eq!(deser_t.to_cbor_bytes(), final_tx.to_cbor_bytes());
        assert_eq!(
            deser_t.auxiliary_data_hash.unwrap(),
            hash_auxiliary_data(&auxiliary_data)
        );
    }

    #[test]
    fn build_tx_multisig_1on1_signed() {
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(10, 2));
        let spend = root_key_15()
            .derive(harden(1854)) //multisig
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_15()
            .derive(harden(1854)) //multisig
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();

        let spend_cred = StakeCredential::new_pub_key(spend.to_raw_key().hash());
        let stake_cred = StakeCredential::new_pub_key(stake.to_raw_key().hash());
        let addr_net_0 =
            BaseAddress::new(NetworkInfo::testnet().network_id(), spend_cred, stake_cred)
                .to_address();
        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_net_0.clone(), Value::from(1_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(999_000)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);
        tx_builder.set_fee(1_000);

        let mut auxiliary_data = AuxiliaryData::new();
        let mut pubkey_native_scripts = Vec::new();
        let mut oneof_native_scripts = Vec::new();

        let spending_hash = spend.to_raw_key().hash();
        pubkey_native_scripts.push(NativeScript::new_script_pubkey(spending_hash));
        oneof_native_scripts.push(NativeScript::new_script_n_of_k(1, pubkey_native_scripts));
        auxiliary_data.add_native_scripts(oneof_native_scripts);
        tx_builder.add_auxiliary_data(auxiliary_data.clone());

        let body = tx_builder.build_body().unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder
                .get_explicit_input()
                .unwrap()
                .checked_add(&tx_builder.get_implicit_input().unwrap())
                .unwrap(),
            tx_builder
                .get_explicit_output()
                .unwrap()
                .checked_add(&Value::from(tx_builder.get_fee_if_set().unwrap()))
                .unwrap()
        );

        let mut witness_set = TransactionWitnessSet::new();

        witness_set.vkeywitnesses = Some(vec![make_vkey_witness(
            &hash_transaction(&body),
            &PrivateKey::from_normal_bytes(
                &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
                    .unwrap(),
            )
            .unwrap(),
        )]);

        let final_tx = Transaction::new(body, witness_set, true, None);
        let deser_t = Transaction::from_cbor_bytes(&final_tx.to_cbor_bytes()).unwrap();
        assert_eq!(deser_t.to_cbor_bytes(), final_tx.to_cbor_bytes());
        assert_eq!(
            deser_t.body.auxiliary_data_hash.unwrap(),
            hash_auxiliary_data(&auxiliary_data)
        );
    }

    #[test]
    fn add_change_splits_change_into_multiple_outputs_when_nfts_overflow_output_size() {
        let linear_fee = LinearFee::new(0, 1);
        let max_value_size = 100; // super low max output size to test with fewer assets
        let mut tx_builder = TransactionBuilder::new(
            TransactionBuilderConfigBuilder::default()
                .fee_algo(linear_fee)
                .pool_deposit(0)
                .key_deposit(0)
                .max_value_size(max_value_size)
                .max_tx_size(MAX_TX_SIZE)
                .coins_per_utxo_byte(1)
                .ex_unit_prices(ExUnitPrices::new(SubCoin::new(0, 0), SubCoin::new(0, 0)))
                .collateral_percentage(150)
                .max_collateral_inputs(3)
                .prefer_pure_change(true)
                .build()
                .unwrap(),
        );

        let policy_id = PolicyId::from([0u8; 28]);
        let names = [
            AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
            AssetName::new(vec![4u8, 5, 6, 7]).unwrap(),
            AssetName::new(vec![5u8, 5, 6, 7]).unwrap(),
            AssetName::new(vec![6u8, 5, 6, 7]).unwrap(),
            AssetName::new(vec![99u8; 32]).unwrap(),
        ];
        let mut multiasset = MultiAsset::new();
        for name in names.iter() {
            multiasset.set(policy_id, name.clone(), 500);
        }

        let input_value = Value::new(1300, multiasset);

        let input = {
            let builder = SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(
                    ByronAddress::from_base58(
                        "Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3",
                    )
                    .unwrap()
                    .to_address(),
                    input_value,
                    None,
                    None,
                ),
            );
            builder.payment_key().unwrap()
        };
        tx_builder.add_input(input).unwrap();

        let output_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b",
        )
        .unwrap()
        .to_address();
        let output_amount = Value::from(208);

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(output_addr)
                    .next()
                    .unwrap()
                    .with_value(output_amount)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let change_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
        )
        .unwrap()
        .to_address();

        let add_change_result = tx_builder.add_change_if_needed_for_tests(&change_addr);
        assert!(add_change_result.is_ok());
        assert_eq!(tx_builder.outputs.len(), 4);

        let change1 = &tx_builder.outputs[1];
        let change2 = &tx_builder.outputs[2];
        let change3 = &tx_builder.outputs[3];

        assert_eq!(*change1.address(), change_addr);
        assert_eq!(change1.address(), change2.address());
        assert_eq!(change1.address(), change3.address());

        assert_eq!(change1.amount().coin, 274);
        assert_eq!(change2.amount().coin, 279);
        assert_eq!(change3.amount().coin, 538);

        assert!(change1.amount().has_multiassets());
        assert!(change2.amount().has_multiassets());
        assert!(!change3.amount().has_multiassets()); // purified

        let masset1 = &change1.amount().multiasset;
        let masset2 = &change2.amount().multiasset;

        assert_eq!(masset1.keys().len(), 1);
        assert_eq!(
            masset1.keys().collect::<Vec<_>>(),
            masset2.keys().collect::<Vec<_>>()
        );

        let asset1 = masset1.deref().get(&policy_id).unwrap();
        let asset2 = masset2.deref().get(&policy_id).unwrap();
        assert_eq!(asset1.len(), 4);
        assert_eq!(asset2.len(), 1);

        names.iter().for_each(|name| {
            let v1 = asset1.get(name);
            let v2 = asset2.get(name);
            assert_ne!(v1.is_some(), v2.is_some());
            assert_eq!(*v1.or(v2).unwrap(), 500);
        });
    }

    fn create_metadatum() -> TransactionMetadatum {
        let mut entries = MetadatumMap::new();
        entries.set(
            TransactionMetadatum::new_text("qwe".into()),
            TransactionMetadatum::new_int(123i64.into()),
        );
        TransactionMetadatum::new_map(entries)
    }

    fn create_general_metadata(metadatum_key: TransactionMetadatumLabel) -> Metadata {
        let mut metadata = Metadata::new();
        metadata.set(metadatum_key, create_metadatum());
        metadata
    }

    fn create_aux_with_metadata(metadatum_key: TransactionMetadatumLabel) -> AuxiliaryData {
        let metadata = create_general_metadata(metadatum_key);

        let mut aux = AuxiliaryData::new_shelley(metadata);

        aux.add_native_scripts(vec![NativeScript::new_script_invalid_before(123)]);

        aux
    }

    fn assert_json_metadatum(dat: &TransactionMetadatum) {
        match dat {
            TransactionMetadatum::Map(map) => {
                assert_eq!(map.len(), 1);
                let key = TransactionMetadatum::new_text(String::from("qwe"));
                let val = map.get(&key).unwrap();
                match val {
                    TransactionMetadatum::Int(x) => assert_eq!(*x, 123u64.into()),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn set_metadata_with_empty_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num = 42;
        {
            let mut aux_data = AuxiliaryData::new();
            aux_data.metadata_mut().set(num, create_metadatum());
            tx_builder.add_auxiliary_data(aux_data);
        }

        assert!(tx_builder.auxiliary_data.is_some());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_v1_scripts().is_none());
        assert!(aux.plutus_v2_scripts().is_none());

        let met = aux.metadata().unwrap();

        assert_eq!(met.len(), 1);
        assert_json_metadatum(met.get(num).unwrap());
    }

    #[test]
    fn set_metadata_with_existing_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num1 = 42;
        tx_builder.add_auxiliary_data(create_aux_with_metadata(num1));

        let num2 = 84;
        {
            let mut aux_data = AuxiliaryData::new();
            aux_data.metadata_mut().set(num2, create_metadatum());
            tx_builder.set_auxiliary_data(aux_data);
        }

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_v1_scripts().is_none());
        assert!(aux.plutus_v2_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.len(), 1);
        assert!(met.get(num1).is_none());
        assert_json_metadatum(met.get(num2).unwrap());
    }

    #[test]
    fn add_metadatum_with_empty_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num = 42;
        {
            let mut aux_data = AuxiliaryData::new();
            aux_data.metadata_mut().set(num, create_metadatum());
            tx_builder.add_auxiliary_data(aux_data);
        }

        assert!(tx_builder.auxiliary_data.is_some());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_v1_scripts().is_none());
        assert!(aux.plutus_v2_scripts().is_none());

        let met = aux.metadata().unwrap();

        assert_eq!(met.len(), 1);
        assert_json_metadatum(met.get(num).unwrap());
    }

    #[test]
    fn add_metadatum_with_existing_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num1 = 42;
        tx_builder.add_auxiliary_data(create_aux_with_metadata(num1));

        let num2 = 84;
        tx_builder.add_auxiliary_data(create_aux_with_metadata(num2));

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_some());
        assert!(aux.plutus_v1_scripts().is_none());
        assert!(aux.plutus_v2_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.len(), 2);
        assert_json_metadatum(met.get(num1).unwrap());
        assert_json_metadatum(met.get(num2).unwrap());
    }

    #[test]
    fn add_json_metadatum_with_empty_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num = 42;
        tx_builder.add_auxiliary_data(AuxiliaryData::new_shelley(create_general_metadata(num)));

        assert!(tx_builder.auxiliary_data.is_some());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_v1_scripts().is_none());
        assert!(aux.plutus_v2_scripts().is_none());

        let met = aux.metadata().unwrap();

        assert_eq!(met.len(), 1);
        assert_json_metadatum(met.get(num).unwrap());
    }

    #[test]
    fn add_json_metadatum_with_existing_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num1 = 42;
        tx_builder.add_auxiliary_data(create_aux_with_metadata(num1));

        let num2 = 84;
        tx_builder.add_auxiliary_data(create_aux_with_metadata(num2));

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_some());
        assert!(aux.plutus_v1_scripts().is_none());
        assert!(aux.plutus_v2_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.len(), 2);
        assert_json_metadatum(met.get(num1).unwrap());
        assert_json_metadatum(met.get(num2).unwrap());
    }

    #[test]
    fn add_metadata_with_empty_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let key = 42;
        let value = TransactionMetadatum::new_text("Hello World".to_string());
        {
            let mut aux_data = AuxiliaryData::new();
            aux_data.metadata_mut().set(key, value.clone());
            tx_builder.add_auxiliary_data(aux_data);
        }

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_v1_scripts().is_none());
        assert!(aux.plutus_v2_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.len(), 1);
        assert_eq!(*met.get(key).unwrap(), value);
    }

    #[test]
    fn add_json_metadata_with_empty_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let key = 42;
        tx_builder.add_auxiliary_data(AuxiliaryData::new_shelley(create_general_metadata(key)));

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_v1_scripts().is_none());
        assert!(aux.plutus_v2_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.len(), 1);
        assert_json_metadatum(met.get(key).unwrap());
    }

    #[test]
    fn add_metadata_with_existing_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let key1 = 42;
        tx_builder.add_auxiliary_data(create_aux_with_metadata(key1));

        let key2 = 84;
        let val2 = TransactionMetadatum::new_text("Hello World".to_string());
        {
            let mut aux_data = AuxiliaryData::new();
            aux_data.metadata_mut().set(key2, val2.clone());
            tx_builder.add_auxiliary_data(aux_data);
        }

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_some());
        assert!(aux.plutus_v1_scripts().is_none());
        assert!(aux.plutus_v2_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.entries.len(), 2);
        assert_json_metadatum(met.get(key1).unwrap());
        assert_eq!(*met.get(key2).unwrap(), val2);
    }

    #[test]
    fn add_json_metadata_with_existing_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let key1 = 42;
        tx_builder.add_auxiliary_data(create_aux_with_metadata(key1));

        let key2 = 84;
        tx_builder.add_auxiliary_data(create_aux_with_metadata(key2));

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_some());
        assert!(aux.plutus_v1_scripts().is_none());
        assert!(aux.plutus_v2_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.entries.len(), 2);
        assert_json_metadatum(met.get(key1).unwrap());
        assert_json_metadatum(met.get(key2).unwrap());
    }

    fn create_asset_name() -> AssetName {
        AssetName::new(vec![0u8, 1, 2, 3]).unwrap()
    }

    fn create_mint_asset_builder() -> SingleMintBuilder {
        SingleMintBuilder::new_single_asset(create_asset_name(), 1234)
    }

    fn create_multiasset_one_asset(policy_id: &PolicyId) -> MultiAsset {
        let mut mint = MultiAsset::default();
        mint.set(*policy_id, create_asset_name(), 1234);
        mint
    }

    fn assert_mint_asset(mint: &Mint, policy_id: &PolicyId) {
        let result_asset = mint.deref().get(policy_id).unwrap();
        assert_eq!(result_asset.len(), 1);
        assert_eq!(
            *result_asset.deref().get(&create_asset_name()).unwrap(),
            1234
        );
    }

    fn mint_script_and_policy_and_hash(x: u8) -> (NativeScript, PolicyId, Ed25519KeyHash) {
        let hash = fake_key_hash(x);
        let mint_script = NativeScript::new_script_pubkey(hash);
        let policy_id = mint_script.hash();
        (mint_script, policy_id, hash)
    }

    fn mint_script_and_policy(x: u8) -> (NativeScript, PolicyId) {
        let (m, p, _) = mint_script_and_policy_and_hash(x);
        (m, p)
    }

    #[test]
    fn set_mint_asset_with_empty_mint() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script, policy_id) = mint_script_and_policy(0);

        let result = create_mint_asset_builder().native_script(
            mint_script,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        assert!(tx_builder.mint.is_some());

        let mint = tx_builder.mint.unwrap();

        assert_eq!(mint.len(), 1);
        assert_mint_asset(&mint, &policy_id);
    }

    #[test]
    fn set_mint_asset_with_existing_mint() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script1, policy_id1) = mint_script_and_policy(0);
        let (mint_script2, policy_id2) = mint_script_and_policy(1);

        let result = create_mint_asset_builder().native_script(
            mint_script1,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        let result = create_mint_asset_builder().native_script(
            mint_script2,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        assert!(tx_builder.mint.is_some());

        let mint = tx_builder.mint.unwrap();

        assert_eq!(mint.len(), 2);
        assert_mint_asset(&mint, &policy_id1);
        assert_mint_asset(&mint, &policy_id2);
    }

    #[test]
    fn add_mint_asset_with_empty_mint() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script, policy_id) = mint_script_and_policy(0);

        let result = create_mint_asset_builder().native_script(
            mint_script,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        assert!(tx_builder.mint.is_some());

        let mint = tx_builder.mint.unwrap();

        assert_eq!(mint.len(), 1);
        assert_mint_asset(&mint, &policy_id);
    }

    #[test]
    fn add_mint_asset_with_existing_mint() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script1, policy_id1) = mint_script_and_policy(0);
        let (mint_script2, policy_id2) = mint_script_and_policy(1);

        let result = create_mint_asset_builder().native_script(
            mint_script1,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        let result = create_mint_asset_builder().native_script(
            mint_script2,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        assert!(tx_builder.mint.is_some());

        let mint = tx_builder.mint.unwrap();

        assert_eq!(mint.len(), 2);
        assert_mint_asset(&mint, &policy_id1);
        assert_mint_asset(&mint, &policy_id2);
    }

    #[test]
    fn add_mint_same_policy() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script1, policy_id1) = mint_script_and_policy(1);
        let (mint_script2, policy_id2) = mint_script_and_policy(2);
        let (mint_script3, policy_id3) = mint_script_and_policy(3);

        let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let name2 = AssetName::new(vec![1u8, 1, 2, 3]).unwrap();
        let name3 = AssetName::new(vec![2u8, 1, 2, 3]).unwrap();
        let name4 = AssetName::new(vec![3u8, 1, 2, 3]).unwrap();
        let amount = 1234;

        // One input from an unrelated address
        let input = {
            let ((_spend, _), _, address) = create_account();
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, Value::from(10_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        // One input from a related address
        let input = {
            let cred = StakeCredential::new_script(policy_id1);
            let address = BaseAddress::new(NetworkInfo::testnet().network_id(), cred.clone(), cred)
                .to_address();
            let builder = SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(address, Value::from(10_000_000), None, None),
            );
            builder
                .native_script(
                    mint_script1.clone(),
                    NativeScriptWitnessInfo::assume_signature_count(),
                )
                .unwrap()
        };
        tx_builder.add_input(input).unwrap();

        let original_tx_fee = tx_builder.min_fee(false).unwrap();
        assert_eq!(original_tx_fee, 164502);

        let result = SingleMintBuilder::new_single_asset(name1, amount).native_script(
            mint_script1,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        let result = SingleMintBuilder::new_single_asset(name2, amount).native_script(
            mint_script2,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        let result = SingleMintBuilder::new_single_asset(name3, amount).native_script(
            mint_script3.clone(),
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        let result = SingleMintBuilder::new_single_asset(name4, amount).native_script(
            mint_script3,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        let mint = tx_builder.get_mint().unwrap();

        assert_eq!(mint.len(), 3);
        assert_eq!(mint.deref().get(&policy_id1).unwrap().len(), 1);
        assert_eq!(mint.deref().get(&policy_id2).unwrap().len(), 1);
        assert_eq!(mint.deref().get(&policy_id3).unwrap().len(), 2);

        let mint_scripts = tx_builder.witness_builders.build_fake().unwrap();

        assert_eq!(mint_scripts.native_scripts.unwrap().len(), 3);
        assert_eq!(mint_scripts.vkeywitnesses.unwrap().len(), 6);
        assert!(mint_scripts.bootstrap_witnesses.is_none());
        assert!(mint_scripts.plutus_datums.is_none());
        assert!(mint_scripts.plutus_v1_scripts.is_none());
        assert!(mint_scripts.redeemers.is_none());
    }

    #[test]
    fn add_output_amount() {
        let mut tx_builder = create_default_tx_builder();

        let policy_id1 = PolicyId::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);
        let value = Value::new(249, multiasset);

        let address = byron_address();
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(address.clone())
                    .next()
                    .unwrap()
                    .with_value(value.clone())
                    .build()
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = &tx_builder.outputs[0];

        assert_eq!(out.address().to_raw_bytes(), address.to_raw_bytes());
        assert_eq!(*out.amount(), value);
    }

    #[test]
    fn add_output_coin() {
        let mut tx_builder = create_default_tx_builder();

        let address = byron_address();
        let coin = 208;
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(address.clone())
                    .next()
                    .unwrap()
                    .with_value(coin)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = &tx_builder.outputs[0];

        assert_eq!(out.address().to_raw_bytes(), address.to_raw_bytes());
        assert_eq!(out.amount().coin, coin);
        assert!(!out.amount().has_multiassets());
    }

    #[test]
    fn add_output_coin_and_multiasset() {
        let mut tx_builder = create_default_tx_builder();

        let policy_id1 = PolicyId::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);

        let address = byron_address();
        let coin = 249;

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(address.clone())
                    .next()
                    .unwrap()
                    .with_value(Value::new(coin, multiasset.clone()))
                    .build()
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = &tx_builder.outputs[0];

        assert_eq!(out.address().to_raw_bytes(), address.to_raw_bytes());
        assert_eq!(out.amount().coin, coin);
        assert_eq!(out.amount().multiasset, multiasset);
    }

    #[test]
    fn add_output_asset_and_min_required_coin() {
        let mut tx_builder = create_realistic_tx_builder();

        let policy_id1 = PolicyId::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);

        let address = byron_address();
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(address.clone())
                    .next()
                    .unwrap()
                    .with_asset_and_min_required_coin(
                        multiasset.clone(),
                        tx_builder.config.coins_per_utxo_byte,
                    )
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = &tx_builder.outputs[0];

        assert_eq!(out.address().to_raw_bytes(), address.to_raw_bytes());
        assert_eq!(out.amount().multiasset, multiasset);
        assert_eq!(out.amount().coin, 1086120);
    }

    #[test]
    fn add_mint_asset_and_output() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script0, policy_id0) = mint_script_and_policy(0);
        let (mint_script1, policy_id1) = mint_script_and_policy(1);

        let name = create_asset_name();
        let amount = 1234;

        let address = byron_address();
        let coin = 249;

        let result = SingleMintBuilder::new_single_asset(name.clone(), amount).native_script(
            mint_script0,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        let multiasset = {
            let mut multiasset = MultiAsset::new();
            multiasset.set(policy_id1, name.clone(), 1234);
            multiasset
        };

        let output = TransactionOutputBuilder::new()
            .with_address(address.clone())
            .next()
            .unwrap()
            .with_value(Value::new(coin, multiasset))
            .build()
            .unwrap();

        tx_builder.add_output(output).unwrap();

        let result = SingleMintBuilder::new_single_asset(name.clone(), amount).native_script(
            mint_script1,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        assert!(tx_builder.mint.is_some());

        let mint = tx_builder.mint.as_ref().unwrap();

        // Mint contains two entries
        assert_eq!(mint.len(), 2);
        assert_mint_asset(mint, &policy_id0);
        assert_mint_asset(mint, &policy_id1);

        // One new output is created
        assert_eq!(tx_builder.outputs.len(), 1);
        let out = &tx_builder.outputs[0];

        assert_eq!(out.address().to_raw_bytes(), address.to_raw_bytes());
        assert_eq!(out.amount().coin, coin);

        let multiasset = &out.amount().multiasset;

        // Only second mint entry was added to the output
        assert_eq!(multiasset.len(), 1);
        assert!(multiasset.deref().get(&policy_id0).is_none());
        assert!(multiasset.deref().get(&policy_id1).is_some());

        let asset = multiasset.deref().get(&policy_id1).unwrap();
        assert_eq!(asset.len(), 1);
        assert_eq!(*asset.get(&name).unwrap(), 1234);
    }

    #[test]
    fn add_mint_asset_and_min_required_coin() {
        let mut tx_builder = create_realistic_tx_builder();

        let (mint_script0, policy_id0) = mint_script_and_policy(0);
        let (mint_script1, policy_id1) = mint_script_and_policy(1);

        let name = create_asset_name();
        let amount = 1234;

        let address = byron_address();

        let result = SingleMintBuilder::new_single_asset(name.clone(), amount).native_script(
            mint_script0,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        let multiasset = {
            let mut multiasset = MultiAsset::new();
            multiasset.set(policy_id1, name.clone(), 1234);
            multiasset
        };

        let output = TransactionOutputBuilder::new()
            .with_address(address.clone())
            .next()
            .unwrap()
            .with_asset_and_min_required_coin(multiasset, tx_builder.config.coins_per_utxo_byte)
            .unwrap()
            .build()
            .unwrap();

        tx_builder.add_output(output).unwrap();

        let result = SingleMintBuilder::new_single_asset(name.clone(), amount).native_script(
            mint_script1,
            NativeScriptWitnessInfo::assume_signature_count(),
        );

        tx_builder.add_mint(result).unwrap();

        assert!(tx_builder.mint.is_some());

        let mint = tx_builder.mint.as_ref().unwrap();

        // Mint contains two entries
        assert_eq!(mint.len(), 2);
        assert_mint_asset(mint, &policy_id0);
        assert_mint_asset(mint, &policy_id1);

        // One new output is created
        assert_eq!(tx_builder.outputs.len(), 1);
        let out = &tx_builder.outputs[0];

        assert_eq!(out.address().to_raw_bytes(), address.to_raw_bytes());
        assert_eq!(out.amount().coin, 1086120);

        let multiasset = &out.amount().multiasset;

        // Only second mint entry was added to the output
        assert_eq!(multiasset.len(), 1);
        assert!(multiasset.deref().get(&policy_id0).is_none());
        assert!(multiasset.deref().get(&policy_id1).is_some());

        let asset = multiasset.deref().get(&policy_id1).unwrap();
        assert_eq!(asset.len(), 1);
        assert_eq!(*asset.get(&name).unwrap(), 1234);
    }

    #[test]
    fn total_input_with_mint_and_burn() {
        let mut tx_builder = create_tx_builder_with_fee(create_linear_fee(0, 1));
        let (_, (_stake, _), addr_test_0) = create_account();

        let (mint_script1, policy_id1) = mint_script_and_policy(0);
        let (mint_script2, policy_id2) = mint_script_and_policy(1);

        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let ma_input1 = 100;
        let ma_input2 = 200;
        let ma_output1 = 60;

        let multiassets = [ma_input1, ma_input2, ma_output1]
            .iter()
            .map(|input| {
                let mut multiasset = MultiAsset::new();
                multiasset.set(policy_id1, name.clone(), *input);
                multiasset.set(policy_id2, name.clone(), *input);
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets.iter().zip([100u64, 100, 100].iter().cloned()) {
            let mut input_amount = Value::from(ada);
            input_amount.multiasset = multiasset.clone();

            let input = {
                SingleInputBuilder::new(
                    TransactionInput::new(genesis_id(), 0),
                    TransactionOutput::new(addr_test_0.clone(), input_amount, None, None),
                )
                .payment_key()
                .unwrap()
            };
            tx_builder.add_input(input).unwrap();
        }

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(byron_address())
                    .next()
                    .unwrap()
                    .with_value(208)
                    .build()
                    .unwrap(),
            )
            .unwrap();

        let total_input_before_mint = tx_builder.get_total_input().unwrap();
        let total_output_before_mint = tx_builder.get_total_output().unwrap();

        assert_eq!(total_input_before_mint.coin, 300);
        assert_eq!(total_output_before_mint.coin, 208);
        let ma1_input = &total_input_before_mint.multiasset;
        let ma1_output = &total_output_before_mint;
        assert_eq!(ma1_input.get(&policy_id1, &name).unwrap(), 360);
        assert_eq!(ma1_input.get(&policy_id2, &name).unwrap(), 360);
        assert!(!ma1_output.has_multiassets());

        // Adding mint
        let result = SingleMintBuilder::new_single_asset(name.clone(), 40).native_script(
            mint_script1,
            NativeScriptWitnessInfo::assume_signature_count(),
        );
        tx_builder.add_mint(result).unwrap();

        // Adding burn
        let result = SingleMintBuilder::new_single_asset(name.clone(), -40).native_script(
            mint_script2,
            NativeScriptWitnessInfo::assume_signature_count(),
        );
        tx_builder.add_mint(result).unwrap();

        let total_input_after_mint = tx_builder.get_total_input().unwrap();
        let total_output_after_mint = tx_builder.get_total_output().unwrap();

        assert_eq!(total_input_after_mint.coin, 300);
        assert_eq!(total_output_before_mint.coin, 208);
        let ma2_input = total_input_after_mint.multiasset;
        let ma2_output = total_output_after_mint.multiasset;
        assert_eq!(ma2_input.get(&policy_id1, &name).unwrap(), 400);
        assert_eq!(ma2_input.get(&policy_id2, &name).unwrap(), 360);
        assert_eq!(ma2_output.get(&policy_id2, &name).unwrap(), 40);
    }

    #[test]
    fn test_contract() {
        let mut tx_builder = create_realistic_tx_builder();

        // let tx = Transaction::from_bytes(
        //     hex::decode("84a70081825820473899cb48414442ea107735f7fc3e020f0293122e9d05e4be6f03ffafde5a0c00018283581d71aba3c2914116298a146af57d8156b1583f183fc05c0aa48ee95bec71821a001c41caa1581c6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d52a14f537061636542756442696433303533015820f7f2f57c58b5e4872201ab678928b0d63935e82d022d385e1bad5bfe347e89d8825839015627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9a013112333b21ec5063ae54f31b0ea883635b64530b70785a49c95041a040228dd021a000db2d907582029ed935cc80249c4de9f3e96fdcea6b7da123a543bbe75fffe9e2c66119e426d0b582039249ec62e53b77ff197bf6821548157b14d56ef63ec3a0b233180e3ae4241740d81825820a90a895d07049afc725a0d6a38c6b82218b8d1de60e7bd70ecdd58f1d9e1218b000e81581c5627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9a40081825820c9b539dea76713f036285a9c89d164ad929597367a5572c9911832f12fffe0235840bb7d26b65a15f9aa917663178d27e2f16a59bbd4aafe067090dcb60826a585d2b81bf6f25136f5c74fdf78fefcd1928ac6e03d28d13da10de1c03b185e697301038159194059193d010000332332233223232333332222233332222332232333222323332223233333333222222223233322232333322223232332232333222323332223232332233223232333332222233223322332233223322332222323223223232533530343330093333573466e1d401920042304e3055357426aae7940208cccd5cd19b875007480088c140c158d5d09aab9e500923333573466e1d40212000204f235058353059335738921035054310005a49926499263333573466e1d40112006205223333573466e1d40152004205523333573466e1d40192002205323333573466e1d401d2000205623505935305a3357389201035054310005b4992649926498cccd5cd19b8735573aa004900011980619191919191919191919191999ab9a3370e6aae75402920002333333333301a335028232323333573466e1cd55cea8012400046604060766ae854008c0b4d5d09aba25002235066353067335738921035054310006849926135573ca00226ea8004d5d0a80519a8140149aba150093335502f75ca05c6ae854020ccd540bdd728171aba1500733502804435742a00c66a05066aa0aa09aeb4d5d0a8029919191999ab9a3370e6aae754009200023350223232323333573466e1cd55cea80124000466a05466a086eb4d5d0a80118241aba135744a00446a0d46a60d666ae712401035054310006c49926135573ca00226ea8004d5d0a8011919191999ab9a3370e6aae7540092000233502833504375a6ae854008c120d5d09aba2500223506a35306b3357389201035054310006c49926135573ca00226ea8004d5d09aba250022350663530673357389201035054310006849926135573ca00226ea8004d5d0a80219a8143ae35742a00666a05066aa0aaeb88004d5d0a801181d1aba135744a00446a0c46a60c666ae71241035054310006449926135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135573ca00226ea8004d5d0a8011919191999ab9a3370ea00290031180f981e1aba135573ca00646666ae68cdc3a801240084603c608c6ae84d55cf280211999ab9a3370ea00690011180f18189aba135573ca00a46666ae68cdc3a80224000460426eb8d5d09aab9e500623505d35305e3357389201035054310005f49926499264984d55cea80089baa001357426ae8940088d4158d4c15ccd5ce2490350543100058499261057135055353056335738920103505435000574984d55cf280089baa001135573a6ea80044d55cea80089baa0012212330010030022001222222222212333333333300100b00a00900800700600500400300220012212330010030022001122123300100300212001122123300100300212001122123300100300212001212222300400521222230030052122223002005212222300100520011232230023758002640026aa080446666aae7c004940388cd4034c010d5d080118019aba200203f23232323333573466e1cd55cea801a4000466600e6464646666ae68cdc39aab9d5002480008cc034c0c4d5d0a80119a8098169aba135744a00446a0846a608666ae712401035054310004449926135573ca00226ea8004d5d0a801999aa805bae500a35742a00466a01eeb8d5d09aba2500223503e35303f335738921035054310004049926135744a00226aae7940044dd50009110919980080200180110009109198008018011000899aa800bae75a224464460046eac004c8004d540e888c8cccd55cf80112804919a80419aa81718031aab9d5002300535573ca00460086ae8800c0e84d5d08008891001091091198008020018900089119191999ab9a3370ea002900011a80418029aba135573ca00646666ae68cdc3a801240044a01046a06a6a606c66ae7124010350543100037499264984d55cea80089baa001121223002003112200112001232323333573466e1cd55cea8012400046600c600e6ae854008dd69aba135744a00446a05e6a606066ae71241035054310003149926135573ca00226ea80048848cc00400c00880048c8cccd5cd19b8735573aa002900011bae357426aae7940088d40acd4c0b0cd5ce2481035054310002d499261375400224464646666ae68cdc3a800a40084a00e46666ae68cdc3a8012400446a014600c6ae84d55cf280211999ab9a3370ea00690001280511a8171a981799ab9c490103505431000304992649926135573aa00226ea8004484888c00c0104488800844888004480048c8cccd5cd19b8750014800880188cccd5cd19b8750024800080188d4098d4c09ccd5ce2490350543100028499264984d55ce9baa0011220021220012001232323232323333573466e1d4005200c200b23333573466e1d4009200a200d23333573466e1d400d200823300b375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c46601a6eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc048c050d5d0a8049bae357426ae8940248cccd5cd19b875006480088c050c054d5d09aab9e500b23333573466e1d401d2000230133016357426aae7940308d40acd4c0b0cd5ce2481035054310002d49926499264992649926135573aa00826aae79400c4d55cf280109aab9e500113754002424444444600e01044244444446600c012010424444444600a010244444440082444444400644244444446600401201044244444446600201201040024646464646666ae68cdc3a800a400446660106eb4d5d0a8021bad35742a0066eb4d5d09aba2500323333573466e1d400920002300a300b357426aae7940188d4070d4c074cd5ce249035054310001e499264984d55cea80189aba25001135573ca00226ea80048488c00800c888488ccc00401401000c80048c8c8cccd5cd19b875001480088c018dd71aba135573ca00646666ae68cdc3a80124000460106eb8d5d09aab9e500423501635301733573892010350543100018499264984d55cea80089baa001212230020032122300100320011122232323333573466e1cd55cea80124000466aa010600c6ae854008c014d5d09aba25002235013353014335738921035054310001549926135573ca00226ea8004448848cc00400c00844800484888c00c01084888c00801048880048004488880104888800c488880084888800480048c8c8c8cccd5cd19b8735573aa006900011999111998068018010009bae35742a0066eb8d5d0a8011bad357426ae8940088d4018d4c01ccd5ce2481035054310000849926135744a00226aae7940044dd5000893090009000911091998008020018011000889191800800911980198010010009991999111919191991199991111991199911191919191919991119911919191919199999111119191919191999111999111999999991111111199119999911111991191919191991199119911919999111199119911991199119911919191919191991199119191919191919191919999111199119191191919191919111191919191919192999a983d80510a9999a9831805099835199a8342839183f8009a9aa83d280311000998351991199ab9a3371200400211202110026603860bea00460506a604802444444444400260bea00a660d46601aa00a60c4002a66a610a026603aa010603e002210e0226605260be66026a010603e00260bea0042c2660d46603aa010603e002660d4666a0d0a0e46a6aa0f4a00c440020fa6601aa00a60c40022660d4666a0d0a0e46a6aa0f4a00c440020fa660d46601aa00a60c4002660d46601866026a010603e00260c4002660086a05460bea004a00642a6666a60c60142c2660d46601866026a010a00660c4002660d46605260420026046024660086042002603e00226603aa010603e0022c2a6666a60c40122a66a6108026644666ae68cdc4801000843808440098082800a40042a66a6a0ec605401026102022c442a66a6a0f000226106022c46442a66a6a0f600226a6aa0fc6a6aa0fca0044400444a666a61040200242660e26602800660d2002660e2660606a06260cc0066054032660e2666a0de0ca605000290011a9aa840809a9aa84080a80291000912999a98428080090b10b0999a83883399814980d2805a4004603400442c2660e0666a0dc0c86604c602ea0109001180b8011a9aa840009a9aa84000a80211000912999a98420080090998399980b001983580099839998191a8199834001981600d999a8388339815000a400442c2c4426110022c266aa0fa601200660120022a66a6a0ec605401026104022c4646442a66a6a0f40022a666a60fe6a6aa0faa0064400242660dc66022a00660cc002660dc6605a6a05c60c6a006604e02c666a0d80c4604a002900110b0b1109844008b09a9aa83da80091001098038008b0b0b0a99a9a8369a9816003911a981800111111111111982300500590980e9a981e000910008b0a99a9a83a191a98170009111111111001a802898390b110a99a9a83b0008801110983b0b1191919191299a98438099815803241012179fa042660d86605660c26602aa014a0226054a004660d86605660c26602aa0146a6aa0f8a020440046054a0066605660c26602aa014002605466044660446604400ca004a0066a6aaa050a0084440022660d86605660c26602aa014a0226054a00a6605660c26602aa01400260546604400ca00a26a6aaa04ca00444400626a6aaa04aa0024440042666aaa04a660e40046a6aaa048a01c444002660e40046a6aa0f0a01844002660e40046a60440204444444440062660e20026a6aaa046a01a44400426a6aa0eaa002440042a66a6a0e2604a006260e02c442a66a6a0e60022600600444260e82c46a60766a60720024440064466a60ae0044c4a66a6a0d86a607800844400242a66a6a0da646a605e0024444444444a66a6a0f0666aa609824002a09e46a6aa1080200244a66a612202666ae68cdc7801007849808490089a83e8018a83e001109a83d9a9aa84200800910008a83ca80311919190a99a9a8389999999aba400423333573466e1d40092004233335573ea0084a0ea46666aae7cd5d128029299a9a83a98389aba150062135078308501001150762507607307223333573466e1d400d2002233335573ea00a4a0ec46666aae7cd5d128031299a9a83b18391aba150072135079308701001150772507707407323333573466e1d40112000233335573ea00c46a0f0108024a0ee0e84a0ec9324c93128399283992839928398381099aa83f18108050008b09aab9d5002135573ca00226ea800458584d4c0980048800888cc07cccc158008d4c068020888888888024ccd417dc51a980d004111111111003800a4004446603c6660aa004602e00e666a0bce28d4c06401c8888888880180052002135301600422222222200413535550175001222003135301400222222222200523322300200132233200132001333550023233503b22333503a0030010023503700133503a22230033002001200122337000029001000a400060662400266466aa603a2400244a66a60f06006004266a0d60040022002a0d446a6aaa02e002444660bc666a0b8042602c00c006666a0b80a400290011919a800a834a835091199aa829911a9aa83700111199aa82b911a9aa83900111299a983f999ab9a3370e002900004080840008801899805199aaa81080300100080180180080080191199aa980d890009119aa98060900091a9aa8360009119aa83780119aa98078900091a9aa8378009119aa839001199a9aa80700091980a24000002446602a004002466028002900000099aa98060900091a9aa8360009119aa837801199a9aa805800919aa98080900091a9aa8380009119aa8398011aa80900080091199aaa805011801000919aa98080900091a9aa8380009119aa8398011aa808000800999aaa80280f001000a8341a980f8011111111111199aa981289000911a981d0011111a981f8019119a982d8011299a984300999ab9a3371e0260021100210e02266a0f200a00e200e400ea0e4012222444666aa603624002a0ce66aa60142400246a6aa0d40024466aa0da0046aa018002666aa603624002446a6aa0d600444a66a60f0666aa606c240026466a07844666a6a016006440040040026a6a0120024400266a01244a66a60f400420f820020f246a6aa0dc002446601400400a00c2006266a0d6008006a0d000266aa60142400246a6aa0d4002446466aa0dc006600200a640026aa0f444a66a6a0d600226aa0180064426a6aa0e000444a66a60fa66018004010266aa02200e0022600c00600424424660020060042400222424446006008224424446600400a00822424446002008224002640026aa0da442244a66a6a0c00022a0c444266a0c6600800466aa600c240020080024466e0000800488d4c05400888888888894cd4d4178ccd54c0c84800540d494cd4c1d4ccd5cd19b8f00c0010770761350610011506000321077107523530220012220022353062001222003223370200400246a60c000244400246a600600244444444401046a60040024444444440044444444442466666666600201401201000e00c00a0080060044002222444246660020080060042224002400244666ae68cdc400100082f8300900091a9802000911a98040011111111111299a9a8289980f005005909a9810000911a9812000911199aa980a09000911a98148011111a9817004111a98180029119299a983b99a9826802919a98270021299a983c999ab9a3371e0040020f60f42a00620f440f4466a609c00840f44a66a60f2666ae68cdc780100083d83d0a801883d099a83500500488048a99a9a83000190a99a9a8308011099a9825801119a9826001119a9828001119a9828801119812001000903e919a9828801103e91981200100091103e91119a9827002103e911299a983f199ab9a3370e00c006100020fe2a66a60fc666ae68cdc38028010400083f89982b802000883f883f883c0a99a9a8300009083c083c283080789931a982799ab9c4901024c6600050498c8004d5417088448894cd4d41400044008884cc014008ccd54c01c4800401401000488ccd5cd19b8f00200105c05b2212330010030022001222222222212333333333300100b00a0090080070060050040030022001122123300100300212001122123300100300212001122123300100300212001121222300300411222002112220011200122533335300f0012150372150372150372133355300a12001500d2353005001225335304f5335304f333573466e3cd4c06000888008d4c060010880081441404ccd5cd19b873530180022200135301800422001051050105013503b0031503a003221233001003002200122212333001004003002200122123300100300220013200135504522112225335350390011350060032213335009005300400233355300712001005004001123535004001220011235350030012200213350022253353502b002210031001502a12212330010030021200121222230040052122223003005212222300200521222230010052001221233001003002200121222222230070082212222222330060090082122222223005008122222220041222222200322122222223300200900822122222223300100900820012122300200322212233300100500400320012122300200321223001003200122333573466e1c0080040ac0a88ccc00800522100488100222323230010053200135502c223353501d0014800088d4d54088008894cd4c0bcccd5cd19b8f00200903103013007001130060033200135502b223353501c0014800088d4d54084008894cd4c0b8ccd5cd19b8f00200703002f100113006003112232001320013550292253353501a0011003221330060023004001235301f0012220021222200412222003122220021222200120011200112001225335301d0021001101e2323232323333333574800a46666ae68cdc39aab9d5005480008cccd55cfa8029280691999aab9f50052500e233335573ea00a4a01e46666aae7cd5d128031299a9a807a99a9a807a99a9a80798061aba150092135012223330240030020011501021533535010300d35742a012426a02660040022a0222a02042a66a6a020646666666ae900049404c9404c9404c8d4050dd6801128098081aba150082135013300200115011150102501000d00c00b00a2500c4989402c9402c9402c9402c0204d5d1280089aba25001135573ca00226ea80048ccccccd5d20009280312803128031280311a8039bae00200312001200112122300200311220011200112253335300c0022153335300d00221330050020012130161613015162153335300d0022130161621330050020011301516153335300c001213015162130151610172253353014333573466e3cd4c03c008888008d4c03c0048880080580544ccd5cd19b8735300f00222200135300f00122200101601510152233223370600400266e080092014001262611220021221223300100400312001112212330010030021120012122230030042122230020041222001200122212333001004003002200126262612200212200120011123230010012233003300200200133223322332233333333300248811cd5e6bf0500378d4f0da4e8dde6becec7621cd8cbf5cbb9b87013d4cc0048811c6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d520048810853706163654275640048810b5370616365427564426964003335550044891c826d9fafe1b3acf15bd250de69c04e3fc92c4493785939e069932e8900483001920e209335500648811c88269f8b051a739300fe743a7b315026f4614ce1216a4bb45d7fd0f500482209d20882748203db810920a09c012222222221233333333300100a00900800700600500400300220011112221233300100400300211120011122123300100300211200110482d866820181d866820083581c5627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9443330353301d8668200800581840000d866820380821a004c4b401a77359400f5a1190195a10045d866820080")
        //     .unwrap()
        // ).unwrap();
        // println!("{:?}", tx.to_json());
        // assert_eq!(false, true);
        // based on tx 18565ab3c960c000531e5b359432397907d663c0ac5f5dbae80e1bf88d25c8a0 on mainnet

        let mut spacebudz_asset = MultiAsset::new();
        spacebudz_asset.set(
            PolicyId::from_hex("6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d52").unwrap(),
            AssetName::new(hex::decode("537061636542756442696433303533").unwrap()).unwrap(),
            1,
        );

        // not the real private key used for the tx on Cardano mainnet
        let private_key = PrivateKey::from_normal_bytes(
            &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
                .unwrap(),
        )
        .unwrap();

        // add input
        {
            let required_signers = vec![
                // real tx was using 5627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9 instead
                private_key.to_public().hash(),
            ];

            let input_utxo = TransactionOutputBuilder::new()
                .with_address(
                    Address::from_bech32(
                        "addr1wx468s53gytznzs5dt6hmq2kk9vr7xplcpwq4fywa9d7cug7fd0ed",
                    )
                    .unwrap(),
                )
                .next()
                .unwrap()
                .with_value(Value::new(70000000, spacebudz_asset.clone()))
                .build()
                .unwrap();
            tx_builder.add_input(SingleInputBuilder::new(
                TransactionInput::new(
                    TransactionHash::from_hex("473899cb48414442ea107735f7fc3e020f0293122e9d05e4be6f03ffafde5a0c").unwrap(),
                    0
                ),
                input_utxo.output
            ).plutus_script(
                PartialPlutusWitness::new(
                    PlutusScriptWitness::from(
                        PlutusScript::PlutusV1(PlutusV1Script::new(
                            hex::decode("59193d010000332332233223232333332222233332222332232333222323332223233333333222222223233322232333322223232332232333222323332223232332233223232333332222233223322332233223322332222323223223232533530343330093333573466e1d401920042304e3055357426aae7940208cccd5cd19b875007480088c140c158d5d09aab9e500923333573466e1d40212000204f235058353059335738921035054310005a49926499263333573466e1d40112006205223333573466e1d40152004205523333573466e1d40192002205323333573466e1d401d2000205623505935305a3357389201035054310005b4992649926498cccd5cd19b8735573aa004900011980619191919191919191919191999ab9a3370e6aae75402920002333333333301a335028232323333573466e1cd55cea8012400046604060766ae854008c0b4d5d09aba25002235066353067335738921035054310006849926135573ca00226ea8004d5d0a80519a8140149aba150093335502f75ca05c6ae854020ccd540bdd728171aba1500733502804435742a00c66a05066aa0aa09aeb4d5d0a8029919191999ab9a3370e6aae754009200023350223232323333573466e1cd55cea80124000466a05466a086eb4d5d0a80118241aba135744a00446a0d46a60d666ae712401035054310006c49926135573ca00226ea8004d5d0a8011919191999ab9a3370e6aae7540092000233502833504375a6ae854008c120d5d09aba2500223506a35306b3357389201035054310006c49926135573ca00226ea8004d5d09aba250022350663530673357389201035054310006849926135573ca00226ea8004d5d0a80219a8143ae35742a00666a05066aa0aaeb88004d5d0a801181d1aba135744a00446a0c46a60c666ae71241035054310006449926135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135573ca00226ea8004d5d0a8011919191999ab9a3370ea00290031180f981e1aba135573ca00646666ae68cdc3a801240084603c608c6ae84d55cf280211999ab9a3370ea00690011180f18189aba135573ca00a46666ae68cdc3a80224000460426eb8d5d09aab9e500623505d35305e3357389201035054310005f49926499264984d55cea80089baa001357426ae8940088d4158d4c15ccd5ce2490350543100058499261057135055353056335738920103505435000574984d55cf280089baa001135573a6ea80044d55cea80089baa0012212330010030022001222222222212333333333300100b00a00900800700600500400300220012212330010030022001122123300100300212001122123300100300212001122123300100300212001212222300400521222230030052122223002005212222300100520011232230023758002640026aa080446666aae7c004940388cd4034c010d5d080118019aba200203f23232323333573466e1cd55cea801a4000466600e6464646666ae68cdc39aab9d5002480008cc034c0c4d5d0a80119a8098169aba135744a00446a0846a608666ae712401035054310004449926135573ca00226ea8004d5d0a801999aa805bae500a35742a00466a01eeb8d5d09aba2500223503e35303f335738921035054310004049926135744a00226aae7940044dd50009110919980080200180110009109198008018011000899aa800bae75a224464460046eac004c8004d540e888c8cccd55cf80112804919a80419aa81718031aab9d5002300535573ca00460086ae8800c0e84d5d08008891001091091198008020018900089119191999ab9a3370ea002900011a80418029aba135573ca00646666ae68cdc3a801240044a01046a06a6a606c66ae7124010350543100037499264984d55cea80089baa001121223002003112200112001232323333573466e1cd55cea8012400046600c600e6ae854008dd69aba135744a00446a05e6a606066ae71241035054310003149926135573ca00226ea80048848cc00400c00880048c8cccd5cd19b8735573aa002900011bae357426aae7940088d40acd4c0b0cd5ce2481035054310002d499261375400224464646666ae68cdc3a800a40084a00e46666ae68cdc3a8012400446a014600c6ae84d55cf280211999ab9a3370ea00690001280511a8171a981799ab9c490103505431000304992649926135573aa00226ea8004484888c00c0104488800844888004480048c8cccd5cd19b8750014800880188cccd5cd19b8750024800080188d4098d4c09ccd5ce2490350543100028499264984d55ce9baa0011220021220012001232323232323333573466e1d4005200c200b23333573466e1d4009200a200d23333573466e1d400d200823300b375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c46601a6eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc048c050d5d0a8049bae357426ae8940248cccd5cd19b875006480088c050c054d5d09aab9e500b23333573466e1d401d2000230133016357426aae7940308d40acd4c0b0cd5ce2481035054310002d49926499264992649926135573aa00826aae79400c4d55cf280109aab9e500113754002424444444600e01044244444446600c012010424444444600a010244444440082444444400644244444446600401201044244444446600201201040024646464646666ae68cdc3a800a400446660106eb4d5d0a8021bad35742a0066eb4d5d09aba2500323333573466e1d400920002300a300b357426aae7940188d4070d4c074cd5ce249035054310001e499264984d55cea80189aba25001135573ca00226ea80048488c00800c888488ccc00401401000c80048c8c8cccd5cd19b875001480088c018dd71aba135573ca00646666ae68cdc3a80124000460106eb8d5d09aab9e500423501635301733573892010350543100018499264984d55cea80089baa001212230020032122300100320011122232323333573466e1cd55cea80124000466aa010600c6ae854008c014d5d09aba25002235013353014335738921035054310001549926135573ca00226ea8004448848cc00400c00844800484888c00c01084888c00801048880048004488880104888800c488880084888800480048c8c8c8cccd5cd19b8735573aa006900011999111998068018010009bae35742a0066eb8d5d0a8011bad357426ae8940088d4018d4c01ccd5ce2481035054310000849926135744a00226aae7940044dd5000893090009000911091998008020018011000889191800800911980198010010009991999111919191991199991111991199911191919191919991119911919191919199999111119191919191999111999111999999991111111199119999911111991191919191991199119911919999111199119911991199119911919191919191991199119191919191919191919999111199119191191919191919111191919191919192999a983d80510a9999a9831805099835199a8342839183f8009a9aa83d280311000998351991199ab9a3371200400211202110026603860bea00460506a604802444444444400260bea00a660d46601aa00a60c4002a66a610a026603aa010603e002210e0226605260be66026a010603e00260bea0042c2660d46603aa010603e002660d4666a0d0a0e46a6aa0f4a00c440020fa6601aa00a60c40022660d4666a0d0a0e46a6aa0f4a00c440020fa660d46601aa00a60c4002660d46601866026a010603e00260c4002660086a05460bea004a00642a6666a60c60142c2660d46601866026a010a00660c4002660d46605260420026046024660086042002603e00226603aa010603e0022c2a6666a60c40122a66a6108026644666ae68cdc4801000843808440098082800a40042a66a6a0ec605401026102022c442a66a6a0f000226106022c46442a66a6a0f600226a6aa0fc6a6aa0fca0044400444a666a61040200242660e26602800660d2002660e2660606a06260cc0066054032660e2666a0de0ca605000290011a9aa840809a9aa84080a80291000912999a98428080090b10b0999a83883399814980d2805a4004603400442c2660e0666a0dc0c86604c602ea0109001180b8011a9aa840009a9aa84000a80211000912999a98420080090998399980b001983580099839998191a8199834001981600d999a8388339815000a400442c2c4426110022c266aa0fa601200660120022a66a6a0ec605401026104022c4646442a66a6a0f40022a666a60fe6a6aa0faa0064400242660dc66022a00660cc002660dc6605a6a05c60c6a006604e02c666a0d80c4604a002900110b0b1109844008b09a9aa83da80091001098038008b0b0b0a99a9a8369a9816003911a981800111111111111982300500590980e9a981e000910008b0a99a9a83a191a98170009111111111001a802898390b110a99a9a83b0008801110983b0b1191919191299a98438099815803241012179fa042660d86605660c26602aa014a0226054a004660d86605660c26602aa0146a6aa0f8a020440046054a0066605660c26602aa014002605466044660446604400ca004a0066a6aaa050a0084440022660d86605660c26602aa014a0226054a00a6605660c26602aa01400260546604400ca00a26a6aaa04ca00444400626a6aaa04aa0024440042666aaa04a660e40046a6aaa048a01c444002660e40046a6aa0f0a01844002660e40046a60440204444444440062660e20026a6aaa046a01a44400426a6aa0eaa002440042a66a6a0e2604a006260e02c442a66a6a0e60022600600444260e82c46a60766a60720024440064466a60ae0044c4a66a6a0d86a607800844400242a66a6a0da646a605e0024444444444a66a6a0f0666aa609824002a09e46a6aa1080200244a66a612202666ae68cdc7801007849808490089a83e8018a83e001109a83d9a9aa84200800910008a83ca80311919190a99a9a8389999999aba400423333573466e1d40092004233335573ea0084a0ea46666aae7cd5d128029299a9a83a98389aba150062135078308501001150762507607307223333573466e1d400d2002233335573ea00a4a0ec46666aae7cd5d128031299a9a83b18391aba150072135079308701001150772507707407323333573466e1d40112000233335573ea00c46a0f0108024a0ee0e84a0ec9324c93128399283992839928398381099aa83f18108050008b09aab9d5002135573ca00226ea800458584d4c0980048800888cc07cccc158008d4c068020888888888024ccd417dc51a980d004111111111003800a4004446603c6660aa004602e00e666a0bce28d4c06401c8888888880180052002135301600422222222200413535550175001222003135301400222222222200523322300200132233200132001333550023233503b22333503a0030010023503700133503a22230033002001200122337000029001000a400060662400266466aa603a2400244a66a60f06006004266a0d60040022002a0d446a6aaa02e002444660bc666a0b8042602c00c006666a0b80a400290011919a800a834a835091199aa829911a9aa83700111199aa82b911a9aa83900111299a983f999ab9a3370e002900004080840008801899805199aaa81080300100080180180080080191199aa980d890009119aa98060900091a9aa8360009119aa83780119aa98078900091a9aa8378009119aa839001199a9aa80700091980a24000002446602a004002466028002900000099aa98060900091a9aa8360009119aa837801199a9aa805800919aa98080900091a9aa8380009119aa8398011aa80900080091199aaa805011801000919aa98080900091a9aa8380009119aa8398011aa808000800999aaa80280f001000a8341a980f8011111111111199aa981289000911a981d0011111a981f8019119a982d8011299a984300999ab9a3371e0260021100210e02266a0f200a00e200e400ea0e4012222444666aa603624002a0ce66aa60142400246a6aa0d40024466aa0da0046aa018002666aa603624002446a6aa0d600444a66a60f0666aa606c240026466a07844666a6a016006440040040026a6a0120024400266a01244a66a60f400420f820020f246a6aa0dc002446601400400a00c2006266a0d6008006a0d000266aa60142400246a6aa0d4002446466aa0dc006600200a640026aa0f444a66a6a0d600226aa0180064426a6aa0e000444a66a60fa66018004010266aa02200e0022600c00600424424660020060042400222424446006008224424446600400a00822424446002008224002640026aa0da442244a66a6a0c00022a0c444266a0c6600800466aa600c240020080024466e0000800488d4c05400888888888894cd4d4178ccd54c0c84800540d494cd4c1d4ccd5cd19b8f00c0010770761350610011506000321077107523530220012220022353062001222003223370200400246a60c000244400246a600600244444444401046a60040024444444440044444444442466666666600201401201000e00c00a0080060044002222444246660020080060042224002400244666ae68cdc400100082f8300900091a9802000911a98040011111111111299a9a8289980f005005909a9810000911a9812000911199aa980a09000911a98148011111a9817004111a98180029119299a983b99a9826802919a98270021299a983c999ab9a3371e0040020f60f42a00620f440f4466a609c00840f44a66a60f2666ae68cdc780100083d83d0a801883d099a83500500488048a99a9a83000190a99a9a8308011099a9825801119a9826001119a9828001119a9828801119812001000903e919a9828801103e91981200100091103e91119a9827002103e911299a983f199ab9a3370e00c006100020fe2a66a60fc666ae68cdc38028010400083f89982b802000883f883f883c0a99a9a8300009083c083c283080789931a982799ab9c4901024c6600050498c8004d5417088448894cd4d41400044008884cc014008ccd54c01c4800401401000488ccd5cd19b8f00200105c05b2212330010030022001222222222212333333333300100b00a0090080070060050040030022001122123300100300212001122123300100300212001122123300100300212001121222300300411222002112220011200122533335300f0012150372150372150372133355300a12001500d2353005001225335304f5335304f333573466e3cd4c06000888008d4c060010880081441404ccd5cd19b873530180022200135301800422001051050105013503b0031503a003221233001003002200122212333001004003002200122123300100300220013200135504522112225335350390011350060032213335009005300400233355300712001005004001123535004001220011235350030012200213350022253353502b002210031001502a12212330010030021200121222230040052122223003005212222300200521222230010052001221233001003002200121222222230070082212222222330060090082122222223005008122222220041222222200322122222223300200900822122222223300100900820012122300200322212233300100500400320012122300200321223001003200122333573466e1c0080040ac0a88ccc00800522100488100222323230010053200135502c223353501d0014800088d4d54088008894cd4c0bcccd5cd19b8f00200903103013007001130060033200135502b223353501c0014800088d4d54084008894cd4c0b8ccd5cd19b8f00200703002f100113006003112232001320013550292253353501a0011003221330060023004001235301f0012220021222200412222003122220021222200120011200112001225335301d0021001101e2323232323333333574800a46666ae68cdc39aab9d5005480008cccd55cfa8029280691999aab9f50052500e233335573ea00a4a01e46666aae7cd5d128031299a9a807a99a9a807a99a9a80798061aba150092135012223330240030020011501021533535010300d35742a012426a02660040022a0222a02042a66a6a020646666666ae900049404c9404c9404c8d4050dd6801128098081aba150082135013300200115011150102501000d00c00b00a2500c4989402c9402c9402c9402c0204d5d1280089aba25001135573ca00226ea80048ccccccd5d20009280312803128031280311a8039bae00200312001200112122300200311220011200112253335300c0022153335300d00221330050020012130161613015162153335300d0022130161621330050020011301516153335300c001213015162130151610172253353014333573466e3cd4c03c008888008d4c03c0048880080580544ccd5cd19b8735300f00222200135300f00122200101601510152233223370600400266e080092014001262611220021221223300100400312001112212330010030021120012122230030042122230020041222001200122212333001004003002200126262612200212200120011123230010012233003300200200133223322332233333333300248811cd5e6bf0500378d4f0da4e8dde6becec7621cd8cbf5cbb9b87013d4cc0048811c6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d520048810853706163654275640048810b5370616365427564426964003335550044891c826d9fafe1b3acf15bd250de69c04e3fc92c4493785939e069932e8900483001920e209335500648811c88269f8b051a739300fe743a7b315026f4614ce1216a4bb45d7fd0f500482209d20882748203db810920a09c012222222221233333333300100a0090080070060050040030022001111222123330010040030021112001112212330010030021120011").unwrap()
                        ))
                    ),
                    PlutusData::from_cbor_bytes(&hex::decode("D866820380").unwrap()).unwrap(),
                ),
                required_signers,
                PlutusData::from_cbor_bytes(&hex::decode("d866820181d866820083581c5627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9443330353301").unwrap()).unwrap()
            ).unwrap()).unwrap();
        }

        // add change output
        {
            let output_utxo = TransactionOutputBuilder::new()
                .with_address(
                    Address::from_bech32(
                        "addr1wx468s53gytznzs5dt6hmq2kk9vr7xplcpwq4fywa9d7cug7fd0ed",
                    )
                    .unwrap(),
                )
                .with_data(DatumOption::new_hash(
                    DatumHash::from_hex(
                        "f7f2f57c58b5e4872201ab678928b0d63935e82d022d385e1bad5bfe347e89d8",
                    )
                    .unwrap(),
                ))
                .next()
                .unwrap()
                .with_value(Value::new(1851850, spacebudz_asset))
                .build()
                .unwrap();
            tx_builder.add_output(output_utxo).unwrap();
        }

        // add user output
        {
            let output_utxo = TransactionOutputBuilder::new()
                .with_address(Address::from_bech32("addr1q9tzwgthsm4hs8alk5v3rgjn7nf9pldlmnc3nrns6dvct2dqzvgjxvajrmzsvwh9fucmp65gxc6mv3fskurctfyuj5zqc7q30l").unwrap())
                .next()
                .unwrap()
                .with_value(67250397)
                .build()
                .unwrap();
            tx_builder.add_output(output_utxo).unwrap();
        }

        // add collateral
        {
            let input_utxo = TransactionOutputBuilder::new()
                .with_address(Address::from_bech32("addr1q9tzwgthsm4hs8alk5v3rgjn7nf9pldlmnc3nrns6dvct2dqzvgjxvajrmzsvwh9fucmp65gxc6mv3fskurctfyuj5zqc7q30l").unwrap())
                .next()
                .unwrap()
                .with_value(5000000)
                .build()
                .unwrap();
            tx_builder
                .add_collateral(
                    SingleInputBuilder::new(
                        TransactionInput::new(
                            TransactionHash::from_hex(
                                "a90a895d07049afc725a0d6a38c6b82218b8d1de60e7bd70ecdd58f1d9e1218b",
                            )
                            .unwrap(),
                            0,
                        ),
                        input_utxo.output,
                    )
                    .payment_key()
                    .unwrap(),
                )
                .unwrap();
        }

        // metadata
        {
            let mut map = MetadatumMap::new();
            map.set(
                TransactionMetadatum::new_int(Int::from(0u64)),
                TransactionMetadatum::new_bytes(hex::decode("d866820080").unwrap()),
            );

            let mut aux_data = AuxiliaryData::new();
            aux_data
                .metadata_mut()
                .set(405, TransactionMetadatum::new_map(map));
            tx_builder.add_auxiliary_data(aux_data);
        }

        let original_tx_fee = tx_builder.min_fee(false).unwrap();
        assert_eq!(original_tx_fee, 469629);
        tx_builder.set_fee(897753);

        {
            tx_builder.set_exunits(
                RedeemerWitnessKey::new(RedeemerTag::Spend, 0),
                ExUnits::new(5000000, 2000000000),
            );
        }
        let tx = tx_builder.build(ChangeSelectionAlgo::Default, &Address::from_bech32("addr1q9tzwgthsm4hs8alk5v3rgjn7nf9pldlmnc3nrns6dvct2dqzvgjxvajrmzsvwh9fucmp65gxc6mv3fskurctfyuj5zqc7q30l").unwrap()).unwrap();
        assert_eq!(hex::encode(tx.body.to_cbor_bytes()), "a70081825820473899cb48414442ea107735f7fc3e020f0293122e9d05e4be6f03ffafde5a0c00018283581d71aba3c2914116298a146af57d8156b1583f183fc05c0aa48ee95bec71821a001c41caa1581c6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d52a14f537061636542756442696433303533015820f7f2f57c58b5e4872201ab678928b0d63935e82d022d385e1bad5bfe347e89d8825839015627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9a013112333b21ec5063ae54f31b0ea883635b64530b70785a49c95041a040228dd021a000db2d907582029ed935cc80249c4de9f3e96fdcea6b7da123a543bbe75fffe9e2c66119e426d0b58201907c235a0df870e95152669f7c147d6e3a7e251b57e4d5227556d1fd0caca0b0d81825820a90a895d07049afc725a0d6a38c6b82218b8d1de60e7bd70ecdd58f1d9e1218b000e81581c1c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c");
    }

    #[test]
    fn test_contract_dummy_exunit() {
        let mut tx_builder = create_realistic_tx_builder();

        let mut spacebudz_asset = MultiAsset::new();
        spacebudz_asset.set(
            PolicyId::from_hex("6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d52").unwrap(),
            AssetName::new(hex::decode("537061636542756442696433303533").unwrap()).unwrap(),
            1,
        );

        // not the real private key used for the tx on Cardano mainnet
        let private_key = &PrivateKey::from_normal_bytes(
            &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
                .unwrap(),
        )
        .unwrap();

        // add input
        {
            let required_signers = vec![
                // real tx was using 5627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9 instead
                private_key.to_public().hash(),
            ];

            let input_utxo = TransactionOutputBuilder::new()
                .with_address(
                    Address::from_bech32(
                        "addr1wx468s53gytznzs5dt6hmq2kk9vr7xplcpwq4fywa9d7cug7fd0ed",
                    )
                    .unwrap(),
                )
                .next()
                .unwrap()
                .with_value(Value::new(70000000, spacebudz_asset.clone()))
                .build()
                .unwrap();
            tx_builder.add_input(SingleInputBuilder::new(
                TransactionInput::new(
                    TransactionHash::from_hex("473899cb48414442ea107735f7fc3e020f0293122e9d05e4be6f03ffafde5a0c").unwrap(),
                    0
                ),
                input_utxo.output
            ).plutus_script(
                PartialPlutusWitness::new(
                    PlutusScriptWitness::from(
                        PlutusScript::PlutusV1(PlutusV1Script::new(
                            hex::decode("59193d010000332332233223232333332222233332222332232333222323332223233333333222222223233322232333322223232332232333222323332223232332233223232333332222233223322332233223322332222323223223232533530343330093333573466e1d401920042304e3055357426aae7940208cccd5cd19b875007480088c140c158d5d09aab9e500923333573466e1d40212000204f235058353059335738921035054310005a49926499263333573466e1d40112006205223333573466e1d40152004205523333573466e1d40192002205323333573466e1d401d2000205623505935305a3357389201035054310005b4992649926498cccd5cd19b8735573aa004900011980619191919191919191919191999ab9a3370e6aae75402920002333333333301a335028232323333573466e1cd55cea8012400046604060766ae854008c0b4d5d09aba25002235066353067335738921035054310006849926135573ca00226ea8004d5d0a80519a8140149aba150093335502f75ca05c6ae854020ccd540bdd728171aba1500733502804435742a00c66a05066aa0aa09aeb4d5d0a8029919191999ab9a3370e6aae754009200023350223232323333573466e1cd55cea80124000466a05466a086eb4d5d0a80118241aba135744a00446a0d46a60d666ae712401035054310006c49926135573ca00226ea8004d5d0a8011919191999ab9a3370e6aae7540092000233502833504375a6ae854008c120d5d09aba2500223506a35306b3357389201035054310006c49926135573ca00226ea8004d5d09aba250022350663530673357389201035054310006849926135573ca00226ea8004d5d0a80219a8143ae35742a00666a05066aa0aaeb88004d5d0a801181d1aba135744a00446a0c46a60c666ae71241035054310006449926135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135573ca00226ea8004d5d0a8011919191999ab9a3370ea00290031180f981e1aba135573ca00646666ae68cdc3a801240084603c608c6ae84d55cf280211999ab9a3370ea00690011180f18189aba135573ca00a46666ae68cdc3a80224000460426eb8d5d09aab9e500623505d35305e3357389201035054310005f49926499264984d55cea80089baa001357426ae8940088d4158d4c15ccd5ce2490350543100058499261057135055353056335738920103505435000574984d55cf280089baa001135573a6ea80044d55cea80089baa0012212330010030022001222222222212333333333300100b00a00900800700600500400300220012212330010030022001122123300100300212001122123300100300212001122123300100300212001212222300400521222230030052122223002005212222300100520011232230023758002640026aa080446666aae7c004940388cd4034c010d5d080118019aba200203f23232323333573466e1cd55cea801a4000466600e6464646666ae68cdc39aab9d5002480008cc034c0c4d5d0a80119a8098169aba135744a00446a0846a608666ae712401035054310004449926135573ca00226ea8004d5d0a801999aa805bae500a35742a00466a01eeb8d5d09aba2500223503e35303f335738921035054310004049926135744a00226aae7940044dd50009110919980080200180110009109198008018011000899aa800bae75a224464460046eac004c8004d540e888c8cccd55cf80112804919a80419aa81718031aab9d5002300535573ca00460086ae8800c0e84d5d08008891001091091198008020018900089119191999ab9a3370ea002900011a80418029aba135573ca00646666ae68cdc3a801240044a01046a06a6a606c66ae7124010350543100037499264984d55cea80089baa001121223002003112200112001232323333573466e1cd55cea8012400046600c600e6ae854008dd69aba135744a00446a05e6a606066ae71241035054310003149926135573ca00226ea80048848cc00400c00880048c8cccd5cd19b8735573aa002900011bae357426aae7940088d40acd4c0b0cd5ce2481035054310002d499261375400224464646666ae68cdc3a800a40084a00e46666ae68cdc3a8012400446a014600c6ae84d55cf280211999ab9a3370ea00690001280511a8171a981799ab9c490103505431000304992649926135573aa00226ea8004484888c00c0104488800844888004480048c8cccd5cd19b8750014800880188cccd5cd19b8750024800080188d4098d4c09ccd5ce2490350543100028499264984d55ce9baa0011220021220012001232323232323333573466e1d4005200c200b23333573466e1d4009200a200d23333573466e1d400d200823300b375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c46601a6eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc048c050d5d0a8049bae357426ae8940248cccd5cd19b875006480088c050c054d5d09aab9e500b23333573466e1d401d2000230133016357426aae7940308d40acd4c0b0cd5ce2481035054310002d49926499264992649926135573aa00826aae79400c4d55cf280109aab9e500113754002424444444600e01044244444446600c012010424444444600a010244444440082444444400644244444446600401201044244444446600201201040024646464646666ae68cdc3a800a400446660106eb4d5d0a8021bad35742a0066eb4d5d09aba2500323333573466e1d400920002300a300b357426aae7940188d4070d4c074cd5ce249035054310001e499264984d55cea80189aba25001135573ca00226ea80048488c00800c888488ccc00401401000c80048c8c8cccd5cd19b875001480088c018dd71aba135573ca00646666ae68cdc3a80124000460106eb8d5d09aab9e500423501635301733573892010350543100018499264984d55cea80089baa001212230020032122300100320011122232323333573466e1cd55cea80124000466aa010600c6ae854008c014d5d09aba25002235013353014335738921035054310001549926135573ca00226ea8004448848cc00400c00844800484888c00c01084888c00801048880048004488880104888800c488880084888800480048c8c8c8cccd5cd19b8735573aa006900011999111998068018010009bae35742a0066eb8d5d0a8011bad357426ae8940088d4018d4c01ccd5ce2481035054310000849926135744a00226aae7940044dd5000893090009000911091998008020018011000889191800800911980198010010009991999111919191991199991111991199911191919191919991119911919191919199999111119191919191999111999111999999991111111199119999911111991191919191991199119911919999111199119911991199119911919191919191991199119191919191919191919999111199119191191919191919111191919191919192999a983d80510a9999a9831805099835199a8342839183f8009a9aa83d280311000998351991199ab9a3371200400211202110026603860bea00460506a604802444444444400260bea00a660d46601aa00a60c4002a66a610a026603aa010603e002210e0226605260be66026a010603e00260bea0042c2660d46603aa010603e002660d4666a0d0a0e46a6aa0f4a00c440020fa6601aa00a60c40022660d4666a0d0a0e46a6aa0f4a00c440020fa660d46601aa00a60c4002660d46601866026a010603e00260c4002660086a05460bea004a00642a6666a60c60142c2660d46601866026a010a00660c4002660d46605260420026046024660086042002603e00226603aa010603e0022c2a6666a60c40122a66a6108026644666ae68cdc4801000843808440098082800a40042a66a6a0ec605401026102022c442a66a6a0f000226106022c46442a66a6a0f600226a6aa0fc6a6aa0fca0044400444a666a61040200242660e26602800660d2002660e2660606a06260cc0066054032660e2666a0de0ca605000290011a9aa840809a9aa84080a80291000912999a98428080090b10b0999a83883399814980d2805a4004603400442c2660e0666a0dc0c86604c602ea0109001180b8011a9aa840009a9aa84000a80211000912999a98420080090998399980b001983580099839998191a8199834001981600d999a8388339815000a400442c2c4426110022c266aa0fa601200660120022a66a6a0ec605401026104022c4646442a66a6a0f40022a666a60fe6a6aa0faa0064400242660dc66022a00660cc002660dc6605a6a05c60c6a006604e02c666a0d80c4604a002900110b0b1109844008b09a9aa83da80091001098038008b0b0b0a99a9a8369a9816003911a981800111111111111982300500590980e9a981e000910008b0a99a9a83a191a98170009111111111001a802898390b110a99a9a83b0008801110983b0b1191919191299a98438099815803241012179fa042660d86605660c26602aa014a0226054a004660d86605660c26602aa0146a6aa0f8a020440046054a0066605660c26602aa014002605466044660446604400ca004a0066a6aaa050a0084440022660d86605660c26602aa014a0226054a00a6605660c26602aa01400260546604400ca00a26a6aaa04ca00444400626a6aaa04aa0024440042666aaa04a660e40046a6aaa048a01c444002660e40046a6aa0f0a01844002660e40046a60440204444444440062660e20026a6aaa046a01a44400426a6aa0eaa002440042a66a6a0e2604a006260e02c442a66a6a0e60022600600444260e82c46a60766a60720024440064466a60ae0044c4a66a6a0d86a607800844400242a66a6a0da646a605e0024444444444a66a6a0f0666aa609824002a09e46a6aa1080200244a66a612202666ae68cdc7801007849808490089a83e8018a83e001109a83d9a9aa84200800910008a83ca80311919190a99a9a8389999999aba400423333573466e1d40092004233335573ea0084a0ea46666aae7cd5d128029299a9a83a98389aba150062135078308501001150762507607307223333573466e1d400d2002233335573ea00a4a0ec46666aae7cd5d128031299a9a83b18391aba150072135079308701001150772507707407323333573466e1d40112000233335573ea00c46a0f0108024a0ee0e84a0ec9324c93128399283992839928398381099aa83f18108050008b09aab9d5002135573ca00226ea800458584d4c0980048800888cc07cccc158008d4c068020888888888024ccd417dc51a980d004111111111003800a4004446603c6660aa004602e00e666a0bce28d4c06401c8888888880180052002135301600422222222200413535550175001222003135301400222222222200523322300200132233200132001333550023233503b22333503a0030010023503700133503a22230033002001200122337000029001000a400060662400266466aa603a2400244a66a60f06006004266a0d60040022002a0d446a6aaa02e002444660bc666a0b8042602c00c006666a0b80a400290011919a800a834a835091199aa829911a9aa83700111199aa82b911a9aa83900111299a983f999ab9a3370e002900004080840008801899805199aaa81080300100080180180080080191199aa980d890009119aa98060900091a9aa8360009119aa83780119aa98078900091a9aa8378009119aa839001199a9aa80700091980a24000002446602a004002466028002900000099aa98060900091a9aa8360009119aa837801199a9aa805800919aa98080900091a9aa8380009119aa8398011aa80900080091199aaa805011801000919aa98080900091a9aa8380009119aa8398011aa808000800999aaa80280f001000a8341a980f8011111111111199aa981289000911a981d0011111a981f8019119a982d8011299a984300999ab9a3371e0260021100210e02266a0f200a00e200e400ea0e4012222444666aa603624002a0ce66aa60142400246a6aa0d40024466aa0da0046aa018002666aa603624002446a6aa0d600444a66a60f0666aa606c240026466a07844666a6a016006440040040026a6a0120024400266a01244a66a60f400420f820020f246a6aa0dc002446601400400a00c2006266a0d6008006a0d000266aa60142400246a6aa0d4002446466aa0dc006600200a640026aa0f444a66a6a0d600226aa0180064426a6aa0e000444a66a60fa66018004010266aa02200e0022600c00600424424660020060042400222424446006008224424446600400a00822424446002008224002640026aa0da442244a66a6a0c00022a0c444266a0c6600800466aa600c240020080024466e0000800488d4c05400888888888894cd4d4178ccd54c0c84800540d494cd4c1d4ccd5cd19b8f00c0010770761350610011506000321077107523530220012220022353062001222003223370200400246a60c000244400246a600600244444444401046a60040024444444440044444444442466666666600201401201000e00c00a0080060044002222444246660020080060042224002400244666ae68cdc400100082f8300900091a9802000911a98040011111111111299a9a8289980f005005909a9810000911a9812000911199aa980a09000911a98148011111a9817004111a98180029119299a983b99a9826802919a98270021299a983c999ab9a3371e0040020f60f42a00620f440f4466a609c00840f44a66a60f2666ae68cdc780100083d83d0a801883d099a83500500488048a99a9a83000190a99a9a8308011099a9825801119a9826001119a9828001119a9828801119812001000903e919a9828801103e91981200100091103e91119a9827002103e911299a983f199ab9a3370e00c006100020fe2a66a60fc666ae68cdc38028010400083f89982b802000883f883f883c0a99a9a8300009083c083c283080789931a982799ab9c4901024c6600050498c8004d5417088448894cd4d41400044008884cc014008ccd54c01c4800401401000488ccd5cd19b8f00200105c05b2212330010030022001222222222212333333333300100b00a0090080070060050040030022001122123300100300212001122123300100300212001122123300100300212001121222300300411222002112220011200122533335300f0012150372150372150372133355300a12001500d2353005001225335304f5335304f333573466e3cd4c06000888008d4c060010880081441404ccd5cd19b873530180022200135301800422001051050105013503b0031503a003221233001003002200122212333001004003002200122123300100300220013200135504522112225335350390011350060032213335009005300400233355300712001005004001123535004001220011235350030012200213350022253353502b002210031001502a12212330010030021200121222230040052122223003005212222300200521222230010052001221233001003002200121222222230070082212222222330060090082122222223005008122222220041222222200322122222223300200900822122222223300100900820012122300200322212233300100500400320012122300200321223001003200122333573466e1c0080040ac0a88ccc00800522100488100222323230010053200135502c223353501d0014800088d4d54088008894cd4c0bcccd5cd19b8f00200903103013007001130060033200135502b223353501c0014800088d4d54084008894cd4c0b8ccd5cd19b8f00200703002f100113006003112232001320013550292253353501a0011003221330060023004001235301f0012220021222200412222003122220021222200120011200112001225335301d0021001101e2323232323333333574800a46666ae68cdc39aab9d5005480008cccd55cfa8029280691999aab9f50052500e233335573ea00a4a01e46666aae7cd5d128031299a9a807a99a9a807a99a9a80798061aba150092135012223330240030020011501021533535010300d35742a012426a02660040022a0222a02042a66a6a020646666666ae900049404c9404c9404c8d4050dd6801128098081aba150082135013300200115011150102501000d00c00b00a2500c4989402c9402c9402c9402c0204d5d1280089aba25001135573ca00226ea80048ccccccd5d20009280312803128031280311a8039bae00200312001200112122300200311220011200112253335300c0022153335300d00221330050020012130161613015162153335300d0022130161621330050020011301516153335300c001213015162130151610172253353014333573466e3cd4c03c008888008d4c03c0048880080580544ccd5cd19b8735300f00222200135300f00122200101601510152233223370600400266e080092014001262611220021221223300100400312001112212330010030021120012122230030042122230020041222001200122212333001004003002200126262612200212200120011123230010012233003300200200133223322332233333333300248811cd5e6bf0500378d4f0da4e8dde6becec7621cd8cbf5cbb9b87013d4cc0048811c6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d520048810853706163654275640048810b5370616365427564426964003335550044891c826d9fafe1b3acf15bd250de69c04e3fc92c4493785939e069932e8900483001920e209335500648811c88269f8b051a739300fe743a7b315026f4614ce1216a4bb45d7fd0f500482209d20882748203db810920a09c012222222221233333333300100a0090080070060050040030022001111222123330010040030021112001112212330010030021120011").unwrap()
                        ))
                    ),
                    PlutusData::from_cbor_bytes(&hex::decode("D866820380").unwrap()).unwrap(),
                ),
                required_signers,
                PlutusData::from_cbor_bytes(&hex::decode("d866820181d866820083581c5627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9443330353301").unwrap()).unwrap()
            ).unwrap()).unwrap();
        }

        // add change output
        {
            let output_utxo = TransactionOutputBuilder::new()
                .with_address(
                    Address::from_bech32(
                        "addr1wx468s53gytznzs5dt6hmq2kk9vr7xplcpwq4fywa9d7cug7fd0ed",
                    )
                    .unwrap(),
                )
                .with_data(DatumOption::new_hash(
                    DatumHash::from_hex(
                        "f7f2f57c58b5e4872201ab678928b0d63935e82d022d385e1bad5bfe347e89d8",
                    )
                    .unwrap(),
                ))
                .next()
                .unwrap()
                .with_value(Value::new(1851850, spacebudz_asset))
                .build()
                .unwrap();
            tx_builder.add_output(output_utxo).unwrap();
        }

        // add user output
        {
            let output_utxo = TransactionOutputBuilder::new()
                .with_address(Address::from_bech32("addr1q9tzwgthsm4hs8alk5v3rgjn7nf9pldlmnc3nrns6dvct2dqzvgjxvajrmzsvwh9fucmp65gxc6mv3fskurctfyuj5zqc7q30l").unwrap())
                .next()
                .unwrap()
                .with_value(67250397)
                .build()
                .unwrap();
            tx_builder.add_output(output_utxo).unwrap();
        }

        // add collateral
        {
            let input_utxo = TransactionOutputBuilder::new()
                .with_address(Address::from_bech32("addr1q9tzwgthsm4hs8alk5v3rgjn7nf9pldlmnc3nrns6dvct2dqzvgjxvajrmzsvwh9fucmp65gxc6mv3fskurctfyuj5zqc7q30l").unwrap())
                .next()
                .unwrap()
                .with_value(5000000)
                .build()
                .unwrap();
            tx_builder
                .add_collateral(
                    SingleInputBuilder::new(
                        TransactionInput::new(
                            TransactionHash::from_hex(
                                "a90a895d07049afc725a0d6a38c6b82218b8d1de60e7bd70ecdd58f1d9e1218b",
                            )
                            .unwrap(),
                            0,
                        ),
                        input_utxo.output,
                    )
                    .payment_key()
                    .unwrap(),
                )
                .unwrap();
        }

        // metadata
        {
            let mut map = MetadatumMap::new();
            map.set(
                TransactionMetadatum::new_int(0u64.into()),
                TransactionMetadatum::new_bytes(hex::decode("d866820080").unwrap()),
            );

            let mut aux_data = AuxiliaryData::new();
            aux_data
                .metadata_mut()
                .set(405, TransactionMetadatum::new_map(map));
            tx_builder.add_auxiliary_data(aux_data);
        }

        tx_builder.set_fee(897753);

        let mut tx_redeemer_builder = tx_builder
            .build_for_evaluation(
                ChangeSelectionAlgo::Default,
                &Address::from_bech32("addr1wx468s53gytznzs5dt6hmq2kk9vr7xplcpwq4fywa9d7cug7fd0ed")
                    .unwrap(),
            )
            .unwrap();

        let fake_script_hash = tx_redeemer_builder.draft_body().script_data_hash.unwrap();
        assert_eq!(
            fake_script_hash.to_hex(),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
        {
            tx_redeemer_builder.set_exunits(
                RedeemerWitnessKey::new(RedeemerTag::Spend, 0),
                ExUnits::new(5000000, 2000000000),
            );
            tx_builder.set_exunits(
                RedeemerWitnessKey::new(RedeemerTag::Spend, 0),
                ExUnits::new(5000000, 2000000000),
            );
        }
        let signed_tx_builder = tx_builder
            .build(
                ChangeSelectionAlgo::Default,
                &Address::from_bech32("addr1wx468s53gytznzs5dt6hmq2kk9vr7xplcpwq4fywa9d7cug7fd0ed")
                    .unwrap(),
            )
            .unwrap();
        let real_script_hash = signed_tx_builder.body.script_data_hash.as_ref().unwrap();
        assert_eq!(
            real_script_hash.to_hex(),
            "1907c235a0df870e95152669f7c147d6e3a7e251b57e4d5227556d1fd0caca0b"
        );

        let tx = &signed_tx_builder.body;
        assert_eq!(hex::encode(tx.to_cbor_bytes()), "a70081825820473899cb48414442ea107735f7fc3e020f0293122e9d05e4be6f03ffafde5a0c00018283581d71aba3c2914116298a146af57d8156b1583f183fc05c0aa48ee95bec71821a001c41caa1581c6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d52a14f537061636542756442696433303533015820f7f2f57c58b5e4872201ab678928b0d63935e82d022d385e1bad5bfe347e89d8825839015627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9a013112333b21ec5063ae54f31b0ea883635b64530b70785a49c95041a040228dd021a000db2d907582029ed935cc80249c4de9f3e96fdcea6b7da123a543bbe75fffe9e2c66119e426d0b58201907c235a0df870e95152669f7c147d6e3a7e251b57e4d5227556d1fd0caca0b0d81825820a90a895d07049afc725a0d6a38c6b82218b8d1de60e7bd70ecdd58f1d9e1218b000e81581c1c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c");
    }

    #[test]
    fn test_collateral() {
        let mut tx_builder = create_realistic_tx_builder();

        // variant of the tx 18565ab3c960c000531e5b359432397907d663c0ac5f5dbae80e1bf88d25c8a0 on mainnet

        let mut spacebudz_asset = MultiAsset::new();
        spacebudz_asset.set(
            PolicyId::from_hex("6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d52").unwrap(),
            AssetName::new(hex::decode("537061636542756442696433303533").unwrap()).unwrap(),
            1,
        );

        // not the real private key used for the tx on Cardano mainnet
        let private_key = PrivateKey::from_normal_bytes(
            &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
                .unwrap(),
        )
        .unwrap();

        // add input
        {
            let required_signers = vec![
                // real tx was using 5627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9 instead
                private_key.to_public().hash(),
            ];

            let input_utxo = TransactionOutputBuilder::new()
                .with_address(
                    Address::from_bech32(
                        "addr1wx468s53gytznzs5dt6hmq2kk9vr7xplcpwq4fywa9d7cug7fd0ed",
                    )
                    .unwrap(),
                )
                .next()
                .unwrap()
                .with_value(Value::new(70000000, spacebudz_asset.clone()))
                .build()
                .unwrap();
            tx_builder.add_input(SingleInputBuilder::new(
                TransactionInput::new(
                    TransactionHash::from_hex("473899cb48414442ea107735f7fc3e020f0293122e9d05e4be6f03ffafde5a0c").unwrap(),
                    0
                ),
                input_utxo.output
            ).plutus_script(
                PartialPlutusWitness::new(
                    PlutusScriptWitness::from(
                        PlutusScript::PlutusV1(PlutusV1Script::new(
                            hex::decode("59193d010000332332233223232333332222233332222332232333222323332223233333333222222223233322232333322223232332232333222323332223232332233223232333332222233223322332233223322332222323223223232533530343330093333573466e1d401920042304e3055357426aae7940208cccd5cd19b875007480088c140c158d5d09aab9e500923333573466e1d40212000204f235058353059335738921035054310005a49926499263333573466e1d40112006205223333573466e1d40152004205523333573466e1d40192002205323333573466e1d401d2000205623505935305a3357389201035054310005b4992649926498cccd5cd19b8735573aa004900011980619191919191919191919191999ab9a3370e6aae75402920002333333333301a335028232323333573466e1cd55cea8012400046604060766ae854008c0b4d5d09aba25002235066353067335738921035054310006849926135573ca00226ea8004d5d0a80519a8140149aba150093335502f75ca05c6ae854020ccd540bdd728171aba1500733502804435742a00c66a05066aa0aa09aeb4d5d0a8029919191999ab9a3370e6aae754009200023350223232323333573466e1cd55cea80124000466a05466a086eb4d5d0a80118241aba135744a00446a0d46a60d666ae712401035054310006c49926135573ca00226ea8004d5d0a8011919191999ab9a3370e6aae7540092000233502833504375a6ae854008c120d5d09aba2500223506a35306b3357389201035054310006c49926135573ca00226ea8004d5d09aba250022350663530673357389201035054310006849926135573ca00226ea8004d5d0a80219a8143ae35742a00666a05066aa0aaeb88004d5d0a801181d1aba135744a00446a0c46a60c666ae71241035054310006449926135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135573ca00226ea8004d5d0a8011919191999ab9a3370ea00290031180f981e1aba135573ca00646666ae68cdc3a801240084603c608c6ae84d55cf280211999ab9a3370ea00690011180f18189aba135573ca00a46666ae68cdc3a80224000460426eb8d5d09aab9e500623505d35305e3357389201035054310005f49926499264984d55cea80089baa001357426ae8940088d4158d4c15ccd5ce2490350543100058499261057135055353056335738920103505435000574984d55cf280089baa001135573a6ea80044d55cea80089baa0012212330010030022001222222222212333333333300100b00a00900800700600500400300220012212330010030022001122123300100300212001122123300100300212001122123300100300212001212222300400521222230030052122223002005212222300100520011232230023758002640026aa080446666aae7c004940388cd4034c010d5d080118019aba200203f23232323333573466e1cd55cea801a4000466600e6464646666ae68cdc39aab9d5002480008cc034c0c4d5d0a80119a8098169aba135744a00446a0846a608666ae712401035054310004449926135573ca00226ea8004d5d0a801999aa805bae500a35742a00466a01eeb8d5d09aba2500223503e35303f335738921035054310004049926135744a00226aae7940044dd50009110919980080200180110009109198008018011000899aa800bae75a224464460046eac004c8004d540e888c8cccd55cf80112804919a80419aa81718031aab9d5002300535573ca00460086ae8800c0e84d5d08008891001091091198008020018900089119191999ab9a3370ea002900011a80418029aba135573ca00646666ae68cdc3a801240044a01046a06a6a606c66ae7124010350543100037499264984d55cea80089baa001121223002003112200112001232323333573466e1cd55cea8012400046600c600e6ae854008dd69aba135744a00446a05e6a606066ae71241035054310003149926135573ca00226ea80048848cc00400c00880048c8cccd5cd19b8735573aa002900011bae357426aae7940088d40acd4c0b0cd5ce2481035054310002d499261375400224464646666ae68cdc3a800a40084a00e46666ae68cdc3a8012400446a014600c6ae84d55cf280211999ab9a3370ea00690001280511a8171a981799ab9c490103505431000304992649926135573aa00226ea8004484888c00c0104488800844888004480048c8cccd5cd19b8750014800880188cccd5cd19b8750024800080188d4098d4c09ccd5ce2490350543100028499264984d55ce9baa0011220021220012001232323232323333573466e1d4005200c200b23333573466e1d4009200a200d23333573466e1d400d200823300b375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c46601a6eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc048c050d5d0a8049bae357426ae8940248cccd5cd19b875006480088c050c054d5d09aab9e500b23333573466e1d401d2000230133016357426aae7940308d40acd4c0b0cd5ce2481035054310002d49926499264992649926135573aa00826aae79400c4d55cf280109aab9e500113754002424444444600e01044244444446600c012010424444444600a010244444440082444444400644244444446600401201044244444446600201201040024646464646666ae68cdc3a800a400446660106eb4d5d0a8021bad35742a0066eb4d5d09aba2500323333573466e1d400920002300a300b357426aae7940188d4070d4c074cd5ce249035054310001e499264984d55cea80189aba25001135573ca00226ea80048488c00800c888488ccc00401401000c80048c8c8cccd5cd19b875001480088c018dd71aba135573ca00646666ae68cdc3a80124000460106eb8d5d09aab9e500423501635301733573892010350543100018499264984d55cea80089baa001212230020032122300100320011122232323333573466e1cd55cea80124000466aa010600c6ae854008c014d5d09aba25002235013353014335738921035054310001549926135573ca00226ea8004448848cc00400c00844800484888c00c01084888c00801048880048004488880104888800c488880084888800480048c8c8c8cccd5cd19b8735573aa006900011999111998068018010009bae35742a0066eb8d5d0a8011bad357426ae8940088d4018d4c01ccd5ce2481035054310000849926135744a00226aae7940044dd5000893090009000911091998008020018011000889191800800911980198010010009991999111919191991199991111991199911191919191919991119911919191919199999111119191919191999111999111999999991111111199119999911111991191919191991199119911919999111199119911991199119911919191919191991199119191919191919191919999111199119191191919191919111191919191919192999a983d80510a9999a9831805099835199a8342839183f8009a9aa83d280311000998351991199ab9a3371200400211202110026603860bea00460506a604802444444444400260bea00a660d46601aa00a60c4002a66a610a026603aa010603e002210e0226605260be66026a010603e00260bea0042c2660d46603aa010603e002660d4666a0d0a0e46a6aa0f4a00c440020fa6601aa00a60c40022660d4666a0d0a0e46a6aa0f4a00c440020fa660d46601aa00a60c4002660d46601866026a010603e00260c4002660086a05460bea004a00642a6666a60c60142c2660d46601866026a010a00660c4002660d46605260420026046024660086042002603e00226603aa010603e0022c2a6666a60c40122a66a6108026644666ae68cdc4801000843808440098082800a40042a66a6a0ec605401026102022c442a66a6a0f000226106022c46442a66a6a0f600226a6aa0fc6a6aa0fca0044400444a666a61040200242660e26602800660d2002660e2660606a06260cc0066054032660e2666a0de0ca605000290011a9aa840809a9aa84080a80291000912999a98428080090b10b0999a83883399814980d2805a4004603400442c2660e0666a0dc0c86604c602ea0109001180b8011a9aa840009a9aa84000a80211000912999a98420080090998399980b001983580099839998191a8199834001981600d999a8388339815000a400442c2c4426110022c266aa0fa601200660120022a66a6a0ec605401026104022c4646442a66a6a0f40022a666a60fe6a6aa0faa0064400242660dc66022a00660cc002660dc6605a6a05c60c6a006604e02c666a0d80c4604a002900110b0b1109844008b09a9aa83da80091001098038008b0b0b0a99a9a8369a9816003911a981800111111111111982300500590980e9a981e000910008b0a99a9a83a191a98170009111111111001a802898390b110a99a9a83b0008801110983b0b1191919191299a98438099815803241012179fa042660d86605660c26602aa014a0226054a004660d86605660c26602aa0146a6aa0f8a020440046054a0066605660c26602aa014002605466044660446604400ca004a0066a6aaa050a0084440022660d86605660c26602aa014a0226054a00a6605660c26602aa01400260546604400ca00a26a6aaa04ca00444400626a6aaa04aa0024440042666aaa04a660e40046a6aaa048a01c444002660e40046a6aa0f0a01844002660e40046a60440204444444440062660e20026a6aaa046a01a44400426a6aa0eaa002440042a66a6a0e2604a006260e02c442a66a6a0e60022600600444260e82c46a60766a60720024440064466a60ae0044c4a66a6a0d86a607800844400242a66a6a0da646a605e0024444444444a66a6a0f0666aa609824002a09e46a6aa1080200244a66a612202666ae68cdc7801007849808490089a83e8018a83e001109a83d9a9aa84200800910008a83ca80311919190a99a9a8389999999aba400423333573466e1d40092004233335573ea0084a0ea46666aae7cd5d128029299a9a83a98389aba150062135078308501001150762507607307223333573466e1d400d2002233335573ea00a4a0ec46666aae7cd5d128031299a9a83b18391aba150072135079308701001150772507707407323333573466e1d40112000233335573ea00c46a0f0108024a0ee0e84a0ec9324c93128399283992839928398381099aa83f18108050008b09aab9d5002135573ca00226ea800458584d4c0980048800888cc07cccc158008d4c068020888888888024ccd417dc51a980d004111111111003800a4004446603c6660aa004602e00e666a0bce28d4c06401c8888888880180052002135301600422222222200413535550175001222003135301400222222222200523322300200132233200132001333550023233503b22333503a0030010023503700133503a22230033002001200122337000029001000a400060662400266466aa603a2400244a66a60f06006004266a0d60040022002a0d446a6aaa02e002444660bc666a0b8042602c00c006666a0b80a400290011919a800a834a835091199aa829911a9aa83700111199aa82b911a9aa83900111299a983f999ab9a3370e002900004080840008801899805199aaa81080300100080180180080080191199aa980d890009119aa98060900091a9aa8360009119aa83780119aa98078900091a9aa8378009119aa839001199a9aa80700091980a24000002446602a004002466028002900000099aa98060900091a9aa8360009119aa837801199a9aa805800919aa98080900091a9aa8380009119aa8398011aa80900080091199aaa805011801000919aa98080900091a9aa8380009119aa8398011aa808000800999aaa80280f001000a8341a980f8011111111111199aa981289000911a981d0011111a981f8019119a982d8011299a984300999ab9a3371e0260021100210e02266a0f200a00e200e400ea0e4012222444666aa603624002a0ce66aa60142400246a6aa0d40024466aa0da0046aa018002666aa603624002446a6aa0d600444a66a60f0666aa606c240026466a07844666a6a016006440040040026a6a0120024400266a01244a66a60f400420f820020f246a6aa0dc002446601400400a00c2006266a0d6008006a0d000266aa60142400246a6aa0d4002446466aa0dc006600200a640026aa0f444a66a6a0d600226aa0180064426a6aa0e000444a66a60fa66018004010266aa02200e0022600c00600424424660020060042400222424446006008224424446600400a00822424446002008224002640026aa0da442244a66a6a0c00022a0c444266a0c6600800466aa600c240020080024466e0000800488d4c05400888888888894cd4d4178ccd54c0c84800540d494cd4c1d4ccd5cd19b8f00c0010770761350610011506000321077107523530220012220022353062001222003223370200400246a60c000244400246a600600244444444401046a60040024444444440044444444442466666666600201401201000e00c00a0080060044002222444246660020080060042224002400244666ae68cdc400100082f8300900091a9802000911a98040011111111111299a9a8289980f005005909a9810000911a9812000911199aa980a09000911a98148011111a9817004111a98180029119299a983b99a9826802919a98270021299a983c999ab9a3371e0040020f60f42a00620f440f4466a609c00840f44a66a60f2666ae68cdc780100083d83d0a801883d099a83500500488048a99a9a83000190a99a9a8308011099a9825801119a9826001119a9828001119a9828801119812001000903e919a9828801103e91981200100091103e91119a9827002103e911299a983f199ab9a3370e00c006100020fe2a66a60fc666ae68cdc38028010400083f89982b802000883f883f883c0a99a9a8300009083c083c283080789931a982799ab9c4901024c6600050498c8004d5417088448894cd4d41400044008884cc014008ccd54c01c4800401401000488ccd5cd19b8f00200105c05b2212330010030022001222222222212333333333300100b00a0090080070060050040030022001122123300100300212001122123300100300212001122123300100300212001121222300300411222002112220011200122533335300f0012150372150372150372133355300a12001500d2353005001225335304f5335304f333573466e3cd4c06000888008d4c060010880081441404ccd5cd19b873530180022200135301800422001051050105013503b0031503a003221233001003002200122212333001004003002200122123300100300220013200135504522112225335350390011350060032213335009005300400233355300712001005004001123535004001220011235350030012200213350022253353502b002210031001502a12212330010030021200121222230040052122223003005212222300200521222230010052001221233001003002200121222222230070082212222222330060090082122222223005008122222220041222222200322122222223300200900822122222223300100900820012122300200322212233300100500400320012122300200321223001003200122333573466e1c0080040ac0a88ccc00800522100488100222323230010053200135502c223353501d0014800088d4d54088008894cd4c0bcccd5cd19b8f00200903103013007001130060033200135502b223353501c0014800088d4d54084008894cd4c0b8ccd5cd19b8f00200703002f100113006003112232001320013550292253353501a0011003221330060023004001235301f0012220021222200412222003122220021222200120011200112001225335301d0021001101e2323232323333333574800a46666ae68cdc39aab9d5005480008cccd55cfa8029280691999aab9f50052500e233335573ea00a4a01e46666aae7cd5d128031299a9a807a99a9a807a99a9a80798061aba150092135012223330240030020011501021533535010300d35742a012426a02660040022a0222a02042a66a6a020646666666ae900049404c9404c9404c8d4050dd6801128098081aba150082135013300200115011150102501000d00c00b00a2500c4989402c9402c9402c9402c0204d5d1280089aba25001135573ca00226ea80048ccccccd5d20009280312803128031280311a8039bae00200312001200112122300200311220011200112253335300c0022153335300d00221330050020012130161613015162153335300d0022130161621330050020011301516153335300c001213015162130151610172253353014333573466e3cd4c03c008888008d4c03c0048880080580544ccd5cd19b8735300f00222200135300f00122200101601510152233223370600400266e080092014001262611220021221223300100400312001112212330010030021120012122230030042122230020041222001200122212333001004003002200126262612200212200120011123230010012233003300200200133223322332233333333300248811cd5e6bf0500378d4f0da4e8dde6becec7621cd8cbf5cbb9b87013d4cc0048811c6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d520048810853706163654275640048810b5370616365427564426964003335550044891c826d9fafe1b3acf15bd250de69c04e3fc92c4493785939e069932e8900483001920e209335500648811c88269f8b051a739300fe743a7b315026f4614ce1216a4bb45d7fd0f500482209d20882748203db810920a09c012222222221233333333300100a0090080070060050040030022001111222123330010040030021112001112212330010030021120011").unwrap()
                        ))
                    ),
                    PlutusData::from_cbor_bytes(&hex::decode("D866820380").unwrap()).unwrap(),
                ),
                required_signers,
                PlutusData::from_cbor_bytes(&hex::decode("d866820181d866820083581c5627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9443330353301").unwrap()).unwrap()
            ).unwrap()).unwrap();
        }

        // add change output
        {
            let output_utxo = TransactionOutputBuilder::new()
                .with_address(
                    Address::from_bech32(
                        "addr1wx468s53gytznzs5dt6hmq2kk9vr7xplcpwq4fywa9d7cug7fd0ed",
                    )
                    .unwrap(),
                )
                .with_data(DatumOption::new_hash(
                    DatumHash::from_hex(
                        "f7f2f57c58b5e4872201ab678928b0d63935e82d022d385e1bad5bfe347e89d8",
                    )
                    .unwrap(),
                ))
                .next()
                .unwrap()
                .with_value(Value::new(1851850, spacebudz_asset))
                .build()
                .unwrap();
            tx_builder.add_output(output_utxo).unwrap();
        }

        // add user output
        {
            let output_utxo = TransactionOutputBuilder::new()
                .with_address(Address::from_bech32("addr1q9tzwgthsm4hs8alk5v3rgjn7nf9pldlmnc3nrns6dvct2dqzvgjxvajrmzsvwh9fucmp65gxc6mv3fskurctfyuj5zqc7q30l").unwrap())
                .next()
                .unwrap()
                .with_value(67250397)
                .build()
                .unwrap();
            tx_builder.add_output(output_utxo).unwrap();
        }

        // add collateral
        {
            let input_utxo = TransactionOutputBuilder::new()
                .with_address(Address::from_bech32("addr1q9tzwgthsm4hs8alk5v3rgjn7nf9pldlmnc3nrns6dvct2dqzvgjxvajrmzsvwh9fucmp65gxc6mv3fskurctfyuj5zqc7q30l").unwrap())
                .next()
                .unwrap()
                .with_value(5000000)
                .build()
                .unwrap();
            tx_builder
                .add_collateral(
                    SingleInputBuilder::new(
                        TransactionInput::new(
                            TransactionHash::from_hex(
                                "a90a895d07049afc725a0d6a38c6b82218b8d1de60e7bd70ecdd58f1d9e1218b",
                            )
                            .unwrap(),
                            0,
                        ),
                        input_utxo.output,
                    )
                    .payment_key()
                    .unwrap(),
                )
                .unwrap();
        }

        // metadata
        {
            let mut map = MetadatumMap::new();
            map.set(
                TransactionMetadatum::new_int(0u64.into()),
                TransactionMetadatum::new_bytes(hex::decode("d866820080").unwrap()),
            );

            let mut aux_data = AuxiliaryData::new();
            aux_data
                .metadata_mut()
                .set(405, TransactionMetadatum::new_map(map));
            tx_builder.add_auxiliary_data(aux_data);
        }

        tx_builder.set_collateral_return(
            TransactionOutputBuilder::new()
                .with_address(Address::from_bech32("addr1q9tzwgthsm4hs8alk5v3rgjn7nf9pldlmnc3nrns6dvct2dqzvgjxvajrmzsvwh9fucmp65gxc6mv3fskurctfyuj5zqc7q30l").unwrap())
                .next()
                .unwrap()
                .with_value(2000000)
                .build()
                .unwrap()
                .output
        );

        tx_builder.set_fee(897753);

        {
            tx_builder.set_exunits(
                RedeemerWitnessKey::new(RedeemerTag::Spend, 0),
                ExUnits::new(5000000, 2000000000),
            );
        }
        let signed_tx_builder = tx_builder
            .build(
                ChangeSelectionAlgo::Default,
                &Address::from_bech32("addr1wx468s53gytznzs5dt6hmq2kk9vr7xplcpwq4fywa9d7cug7fd0ed")
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(signed_tx_builder.body.total_collateral, Some(3000000));
    }

    #[test]
    fn build_tx_with_ref_input() {
        let mut tx_builder = create_default_tx_builder();
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (_, stake_cred), addr_net_0) = create_account();
        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_net_0.clone(), Value::from(5_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };
        tx_builder.add_input(input).unwrap();
        //add collateral
        {
            let input_utxo = TransactionOutputBuilder::new()
                .with_address(Address::from_bech32("addr1q9tzwgthsm4hs8alk5v3rgjn7nf9pldlmnc3nrns6dvct2dqzvgjxvajrmzsvwh9fucmp65gxc6mv3fskurctfyuj5zqc7q30l").unwrap())
                .next()
                .unwrap()
                .with_value(5000000)
                .build()
                .unwrap();
            tx_builder
                .add_collateral(
                    SingleInputBuilder::new(
                        TransactionInput::new(
                            TransactionHash::from_hex(
                                "a90a895d07049afc725a0d6a38c6b82218b8d1de60e7bd70ecdd58f1d9e1218b",
                            )
                            .unwrap(),
                            0,
                        ),
                        input_utxo.output,
                    )
                    .payment_key()
                    .unwrap(),
                )
                .unwrap();
        }

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        {
            let output = TransactionOutputBuilder::new()
                .with_address(BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred.clone(), stake_cred.clone()).to_address())
                .with_reference_script(
                    Script::new_plutus_v1(
                        PlutusV1Script::new(
                            hex::decode("59193d010000332332233223232333332222233332222332232333222323332223233333333222222223233322232333322223232332232333222323332223232332233223232333332222233223322332233223322332222323223223232533530343330093333573466e1d401920042304e3055357426aae7940208cccd5cd19b875007480088c140c158d5d09aab9e500923333573466e1d40212000204f235058353059335738921035054310005a49926499263333573466e1d40112006205223333573466e1d40152004205523333573466e1d40192002205323333573466e1d401d2000205623505935305a3357389201035054310005b4992649926498cccd5cd19b8735573aa004900011980619191919191919191919191999ab9a3370e6aae75402920002333333333301a335028232323333573466e1cd55cea8012400046604060766ae854008c0b4d5d09aba25002235066353067335738921035054310006849926135573ca00226ea8004d5d0a80519a8140149aba150093335502f75ca05c6ae854020ccd540bdd728171aba1500733502804435742a00c66a05066aa0aa09aeb4d5d0a8029919191999ab9a3370e6aae754009200023350223232323333573466e1cd55cea80124000466a05466a086eb4d5d0a80118241aba135744a00446a0d46a60d666ae712401035054310006c49926135573ca00226ea8004d5d0a8011919191999ab9a3370e6aae7540092000233502833504375a6ae854008c120d5d09aba2500223506a35306b3357389201035054310006c49926135573ca00226ea8004d5d09aba250022350663530673357389201035054310006849926135573ca00226ea8004d5d0a80219a8143ae35742a00666a05066aa0aaeb88004d5d0a801181d1aba135744a00446a0c46a60c666ae71241035054310006449926135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135573ca00226ea8004d5d0a8011919191999ab9a3370ea00290031180f981e1aba135573ca00646666ae68cdc3a801240084603c608c6ae84d55cf280211999ab9a3370ea00690011180f18189aba135573ca00a46666ae68cdc3a80224000460426eb8d5d09aab9e500623505d35305e3357389201035054310005f49926499264984d55cea80089baa001357426ae8940088d4158d4c15ccd5ce2490350543100058499261057135055353056335738920103505435000574984d55cf280089baa001135573a6ea80044d55cea80089baa0012212330010030022001222222222212333333333300100b00a00900800700600500400300220012212330010030022001122123300100300212001122123300100300212001122123300100300212001212222300400521222230030052122223002005212222300100520011232230023758002640026aa080446666aae7c004940388cd4034c010d5d080118019aba200203f23232323333573466e1cd55cea801a4000466600e6464646666ae68cdc39aab9d5002480008cc034c0c4d5d0a80119a8098169aba135744a00446a0846a608666ae712401035054310004449926135573ca00226ea8004d5d0a801999aa805bae500a35742a00466a01eeb8d5d09aba2500223503e35303f335738921035054310004049926135744a00226aae7940044dd50009110919980080200180110009109198008018011000899aa800bae75a224464460046eac004c8004d540e888c8cccd55cf80112804919a80419aa81718031aab9d5002300535573ca00460086ae8800c0e84d5d08008891001091091198008020018900089119191999ab9a3370ea002900011a80418029aba135573ca00646666ae68cdc3a801240044a01046a06a6a606c66ae7124010350543100037499264984d55cea80089baa001121223002003112200112001232323333573466e1cd55cea8012400046600c600e6ae854008dd69aba135744a00446a05e6a606066ae71241035054310003149926135573ca00226ea80048848cc00400c00880048c8cccd5cd19b8735573aa002900011bae357426aae7940088d40acd4c0b0cd5ce2481035054310002d499261375400224464646666ae68cdc3a800a40084a00e46666ae68cdc3a8012400446a014600c6ae84d55cf280211999ab9a3370ea00690001280511a8171a981799ab9c490103505431000304992649926135573aa00226ea8004484888c00c0104488800844888004480048c8cccd5cd19b8750014800880188cccd5cd19b8750024800080188d4098d4c09ccd5ce2490350543100028499264984d55ce9baa0011220021220012001232323232323333573466e1d4005200c200b23333573466e1d4009200a200d23333573466e1d400d200823300b375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c46601a6eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc048c050d5d0a8049bae357426ae8940248cccd5cd19b875006480088c050c054d5d09aab9e500b23333573466e1d401d2000230133016357426aae7940308d40acd4c0b0cd5ce2481035054310002d49926499264992649926135573aa00826aae79400c4d55cf280109aab9e500113754002424444444600e01044244444446600c012010424444444600a010244444440082444444400644244444446600401201044244444446600201201040024646464646666ae68cdc3a800a400446660106eb4d5d0a8021bad35742a0066eb4d5d09aba2500323333573466e1d400920002300a300b357426aae7940188d4070d4c074cd5ce249035054310001e499264984d55cea80189aba25001135573ca00226ea80048488c00800c888488ccc00401401000c80048c8c8cccd5cd19b875001480088c018dd71aba135573ca00646666ae68cdc3a80124000460106eb8d5d09aab9e500423501635301733573892010350543100018499264984d55cea80089baa001212230020032122300100320011122232323333573466e1cd55cea80124000466aa010600c6ae854008c014d5d09aba25002235013353014335738921035054310001549926135573ca00226ea8004448848cc00400c00844800484888c00c01084888c00801048880048004488880104888800c488880084888800480048c8c8c8cccd5cd19b8735573aa006900011999111998068018010009bae35742a0066eb8d5d0a8011bad357426ae8940088d4018d4c01ccd5ce2481035054310000849926135744a00226aae7940044dd5000893090009000911091998008020018011000889191800800911980198010010009991999111919191991199991111991199911191919191919991119911919191919199999111119191919191999111999111999999991111111199119999911111991191919191991199119911919999111199119911991199119911919191919191991199119191919191919191919999111199119191191919191919111191919191919192999a983d80510a9999a9831805099835199a8342839183f8009a9aa83d280311000998351991199ab9a3371200400211202110026603860bea00460506a604802444444444400260bea00a660d46601aa00a60c4002a66a610a026603aa010603e002210e0226605260be66026a010603e00260bea0042c2660d46603aa010603e002660d4666a0d0a0e46a6aa0f4a00c440020fa6601aa00a60c40022660d4666a0d0a0e46a6aa0f4a00c440020fa660d46601aa00a60c4002660d46601866026a010603e00260c4002660086a05460bea004a00642a6666a60c60142c2660d46601866026a010a00660c4002660d46605260420026046024660086042002603e00226603aa010603e0022c2a6666a60c40122a66a6108026644666ae68cdc4801000843808440098082800a40042a66a6a0ec605401026102022c442a66a6a0f000226106022c46442a66a6a0f600226a6aa0fc6a6aa0fca0044400444a666a61040200242660e26602800660d2002660e2660606a06260cc0066054032660e2666a0de0ca605000290011a9aa840809a9aa84080a80291000912999a98428080090b10b0999a83883399814980d2805a4004603400442c2660e0666a0dc0c86604c602ea0109001180b8011a9aa840009a9aa84000a80211000912999a98420080090998399980b001983580099839998191a8199834001981600d999a8388339815000a400442c2c4426110022c266aa0fa601200660120022a66a6a0ec605401026104022c4646442a66a6a0f40022a666a60fe6a6aa0faa0064400242660dc66022a00660cc002660dc6605a6a05c60c6a006604e02c666a0d80c4604a002900110b0b1109844008b09a9aa83da80091001098038008b0b0b0a99a9a8369a9816003911a981800111111111111982300500590980e9a981e000910008b0a99a9a83a191a98170009111111111001a802898390b110a99a9a83b0008801110983b0b1191919191299a98438099815803241012179fa042660d86605660c26602aa014a0226054a004660d86605660c26602aa0146a6aa0f8a020440046054a0066605660c26602aa014002605466044660446604400ca004a0066a6aaa050a0084440022660d86605660c26602aa014a0226054a00a6605660c26602aa01400260546604400ca00a26a6aaa04ca00444400626a6aaa04aa0024440042666aaa04a660e40046a6aaa048a01c444002660e40046a6aa0f0a01844002660e40046a60440204444444440062660e20026a6aaa046a01a44400426a6aa0eaa002440042a66a6a0e2604a006260e02c442a66a6a0e60022600600444260e82c46a60766a60720024440064466a60ae0044c4a66a6a0d86a607800844400242a66a6a0da646a605e0024444444444a66a6a0f0666aa609824002a09e46a6aa1080200244a66a612202666ae68cdc7801007849808490089a83e8018a83e001109a83d9a9aa84200800910008a83ca80311919190a99a9a8389999999aba400423333573466e1d40092004233335573ea0084a0ea46666aae7cd5d128029299a9a83a98389aba150062135078308501001150762507607307223333573466e1d400d2002233335573ea00a4a0ec46666aae7cd5d128031299a9a83b18391aba150072135079308701001150772507707407323333573466e1d40112000233335573ea00c46a0f0108024a0ee0e84a0ec9324c93128399283992839928398381099aa83f18108050008b09aab9d5002135573ca00226ea800458584d4c0980048800888cc07cccc158008d4c068020888888888024ccd417dc51a980d004111111111003800a4004446603c6660aa004602e00e666a0bce28d4c06401c8888888880180052002135301600422222222200413535550175001222003135301400222222222200523322300200132233200132001333550023233503b22333503a0030010023503700133503a22230033002001200122337000029001000a400060662400266466aa603a2400244a66a60f06006004266a0d60040022002a0d446a6aaa02e002444660bc666a0b8042602c00c006666a0b80a400290011919a800a834a835091199aa829911a9aa83700111199aa82b911a9aa83900111299a983f999ab9a3370e002900004080840008801899805199aaa81080300100080180180080080191199aa980d890009119aa98060900091a9aa8360009119aa83780119aa98078900091a9aa8378009119aa839001199a9aa80700091980a24000002446602a004002466028002900000099aa98060900091a9aa8360009119aa837801199a9aa805800919aa98080900091a9aa8380009119aa8398011aa80900080091199aaa805011801000919aa98080900091a9aa8380009119aa8398011aa808000800999aaa80280f001000a8341a980f8011111111111199aa981289000911a981d0011111a981f8019119a982d8011299a984300999ab9a3371e0260021100210e02266a0f200a00e200e400ea0e4012222444666aa603624002a0ce66aa60142400246a6aa0d40024466aa0da0046aa018002666aa603624002446a6aa0d600444a66a60f0666aa606c240026466a07844666a6a016006440040040026a6a0120024400266a01244a66a60f400420f820020f246a6aa0dc002446601400400a00c2006266a0d6008006a0d000266aa60142400246a6aa0d4002446466aa0dc006600200a640026aa0f444a66a6a0d600226aa0180064426a6aa0e000444a66a60fa66018004010266aa02200e0022600c00600424424660020060042400222424446006008224424446600400a00822424446002008224002640026aa0da442244a66a6a0c00022a0c444266a0c6600800466aa600c240020080024466e0000800488d4c05400888888888894cd4d4178ccd54c0c84800540d494cd4c1d4ccd5cd19b8f00c0010770761350610011506000321077107523530220012220022353062001222003223370200400246a60c000244400246a600600244444444401046a60040024444444440044444444442466666666600201401201000e00c00a0080060044002222444246660020080060042224002400244666ae68cdc400100082f8300900091a9802000911a98040011111111111299a9a8289980f005005909a9810000911a9812000911199aa980a09000911a98148011111a9817004111a98180029119299a983b99a9826802919a98270021299a983c999ab9a3371e0040020f60f42a00620f440f4466a609c00840f44a66a60f2666ae68cdc780100083d83d0a801883d099a83500500488048a99a9a83000190a99a9a8308011099a9825801119a9826001119a9828001119a9828801119812001000903e919a9828801103e91981200100091103e91119a9827002103e911299a983f199ab9a3370e00c006100020fe2a66a60fc666ae68cdc38028010400083f89982b802000883f883f883c0a99a9a8300009083c083c283080789931a982799ab9c4901024c6600050498c8004d5417088448894cd4d41400044008884cc014008ccd54c01c4800401401000488ccd5cd19b8f00200105c05b2212330010030022001222222222212333333333300100b00a0090080070060050040030022001122123300100300212001122123300100300212001122123300100300212001121222300300411222002112220011200122533335300f0012150372150372150372133355300a12001500d2353005001225335304f5335304f333573466e3cd4c06000888008d4c060010880081441404ccd5cd19b873530180022200135301800422001051050105013503b0031503a003221233001003002200122212333001004003002200122123300100300220013200135504522112225335350390011350060032213335009005300400233355300712001005004001123535004001220011235350030012200213350022253353502b002210031001502a12212330010030021200121222230040052122223003005212222300200521222230010052001221233001003002200121222222230070082212222222330060090082122222223005008122222220041222222200322122222223300200900822122222223300100900820012122300200322212233300100500400320012122300200321223001003200122333573466e1c0080040ac0a88ccc00800522100488100222323230010053200135502c223353501d0014800088d4d54088008894cd4c0bcccd5cd19b8f00200903103013007001130060033200135502b223353501c0014800088d4d54084008894cd4c0b8ccd5cd19b8f00200703002f100113006003112232001320013550292253353501a0011003221330060023004001235301f0012220021222200412222003122220021222200120011200112001225335301d0021001101e2323232323333333574800a46666ae68cdc39aab9d5005480008cccd55cfa8029280691999aab9f50052500e233335573ea00a4a01e46666aae7cd5d128031299a9a807a99a9a807a99a9a80798061aba150092135012223330240030020011501021533535010300d35742a012426a02660040022a0222a02042a66a6a020646666666ae900049404c9404c9404c8d4050dd6801128098081aba150082135013300200115011150102501000d00c00b00a2500c4989402c9402c9402c9402c0204d5d1280089aba25001135573ca00226ea80048ccccccd5d20009280312803128031280311a8039bae00200312001200112122300200311220011200112253335300c0022153335300d00221330050020012130161613015162153335300d0022130161621330050020011301516153335300c001213015162130151610172253353014333573466e3cd4c03c008888008d4c03c0048880080580544ccd5cd19b8735300f00222200135300f00122200101601510152233223370600400266e080092014001262611220021221223300100400312001112212330010030021120012122230030042122230020041222001200122212333001004003002200126262612200212200120011123230010012233003300200200133223322332233333333300248811cd5e6bf0500378d4f0da4e8dde6becec7621cd8cbf5cbb9b87013d4cc0048811c6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d520048810853706163654275640048810b5370616365427564426964003335550044891c826d9fafe1b3acf15bd250de69c04e3fc92c4493785939e069932e8900483001920e209335500648811c88269f8b051a739300fe743a7b315026f4614ce1216a4bb45d7fd0f500482209d20882748203db810920a09c012222222221233333333300100a0090080070060050040030022001111222123330010040030021112001112212330010030021120011").unwrap()
                        )
                    )
                )
                .with_data(DatumOption::new_datum(PlutusData::from_cbor_bytes(&hex::decode("d866820181d866820083581c5627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9443330353301").unwrap()).unwrap()))
                .next().unwrap()
                .with_value(880_000)
                .build().unwrap();

            tx_builder.add_reference_input(TransactionUnspentOutput::new(
                TransactionInput::new(genesis_id(), 1),
                output.output,
            ));
        }
        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(880_000)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);

        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();
        tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert_eq!(tx_builder.outputs.len(), 2);
        let final_tx = tx_builder
            .build(ChangeSelectionAlgo::Default, &change_addr)
            .unwrap()
            .build_unchecked();

        assert_eq!(final_tx.body.reference_inputs.unwrap().len(), 1);
        assert!(final_tx.witness_set.plutus_v1_scripts.is_none());
    }

    #[test]
    fn build_tx_with_ref_input_script() {
        let mut tx_builder = create_default_tx_builder();
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let (_, (_, stake_cred), addr_net_0) = create_account();

        let script = Script::new_plutus_v2(
            PlutusV2Script::new(
                hex::decode("59193d020000332332233223232333332222233332222332232333222323332223233333333222222223233322232333322223232332232333222323332223232332233223232333332222233223322332233223322332222323223223232533530343330093333573466e1d401920042304e3055357426aae7940208cccd5cd19b875007480088c140c158d5d09aab9e500923333573466e1d40212000204f235058353059335738921035054310005a49926499263333573466e1d40112006205223333573466e1d40152004205523333573466e1d40192002205323333573466e1d401d2000205623505935305a3357389201035054310005b4992649926498cccd5cd19b8735573aa004900011980619191919191919191919191999ab9a3370e6aae75402920002333333333301a335028232323333573466e1cd55cea8012400046604060766ae854008c0b4d5d09aba25002235066353067335738921035054310006849926135573ca00226ea8004d5d0a80519a8140149aba150093335502f75ca05c6ae854020ccd540bdd728171aba1500733502804435742a00c66a05066aa0aa09aeb4d5d0a8029919191999ab9a3370e6aae754009200023350223232323333573466e1cd55cea80124000466a05466a086eb4d5d0a80118241aba135744a00446a0d46a60d666ae712401035054310006c49926135573ca00226ea8004d5d0a8011919191999ab9a3370e6aae7540092000233502833504375a6ae854008c120d5d09aba2500223506a35306b3357389201035054310006c49926135573ca00226ea8004d5d09aba250022350663530673357389201035054310006849926135573ca00226ea8004d5d0a80219a8143ae35742a00666a05066aa0aaeb88004d5d0a801181d1aba135744a00446a0c46a60c666ae71241035054310006449926135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135573ca00226ea8004d5d0a8011919191999ab9a3370ea00290031180f981e1aba135573ca00646666ae68cdc3a801240084603c608c6ae84d55cf280211999ab9a3370ea00690011180f18189aba135573ca00a46666ae68cdc3a80224000460426eb8d5d09aab9e500623505d35305e3357389201035054310005f49926499264984d55cea80089baa001357426ae8940088d4158d4c15ccd5ce2490350543100058499261057135055353056335738920103505435000574984d55cf280089baa001135573a6ea80044d55cea80089baa0012212330010030022001222222222212333333333300100b00a00900800700600500400300220012212330010030022001122123300100300212001122123300100300212001122123300100300212001212222300400521222230030052122223002005212222300100520011232230023758002640026aa080446666aae7c004940388cd4034c010d5d080118019aba200203f23232323333573466e1cd55cea801a4000466600e6464646666ae68cdc39aab9d5002480008cc034c0c4d5d0a80119a8098169aba135744a00446a0846a608666ae712401035054310004449926135573ca00226ea8004d5d0a801999aa805bae500a35742a00466a01eeb8d5d09aba2500223503e35303f335738921035054310004049926135744a00226aae7940044dd50009110919980080200180110009109198008018011000899aa800bae75a224464460046eac004c8004d540e888c8cccd55cf80112804919a80419aa81718031aab9d5002300535573ca00460086ae8800c0e84d5d08008891001091091198008020018900089119191999ab9a3370ea002900011a80418029aba135573ca00646666ae68cdc3a801240044a01046a06a6a606c66ae7124010350543100037499264984d55cea80089baa001121223002003112200112001232323333573466e1cd55cea8012400046600c600e6ae854008dd69aba135744a00446a05e6a606066ae71241035054310003149926135573ca00226ea80048848cc00400c00880048c8cccd5cd19b8735573aa002900011bae357426aae7940088d40acd4c0b0cd5ce2481035054310002d499261375400224464646666ae68cdc3a800a40084a00e46666ae68cdc3a8012400446a014600c6ae84d55cf280211999ab9a3370ea00690001280511a8171a981799ab9c490103505431000304992649926135573aa00226ea8004484888c00c0104488800844888004480048c8cccd5cd19b8750014800880188cccd5cd19b8750024800080188d4098d4c09ccd5ce2490350543100028499264984d55ce9baa0011220021220012001232323232323333573466e1d4005200c200b23333573466e1d4009200a200d23333573466e1d400d200823300b375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c46601a6eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc048c050d5d0a8049bae357426ae8940248cccd5cd19b875006480088c050c054d5d09aab9e500b23333573466e1d401d2000230133016357426aae7940308d40acd4c0b0cd5ce2481035054310002d49926499264992649926135573aa00826aae79400c4d55cf280109aab9e500113754002424444444600e01044244444446600c012010424444444600a010244444440082444444400644244444446600401201044244444446600201201040024646464646666ae68cdc3a800a400446660106eb4d5d0a8021bad35742a0066eb4d5d09aba2500323333573466e1d400920002300a300b357426aae7940188d4070d4c074cd5ce249035054310001e499264984d55cea80189aba25001135573ca00226ea80048488c00800c888488ccc00401401000c80048c8c8cccd5cd19b875001480088c018dd71aba135573ca00646666ae68cdc3a80124000460106eb8d5d09aab9e500423501635301733573892010350543100018499264984d55cea80089baa001212230020032122300100320011122232323333573466e1cd55cea80124000466aa010600c6ae854008c014d5d09aba25002235013353014335738921035054310001549926135573ca00226ea8004448848cc00400c00844800484888c00c01084888c00801048880048004488880104888800c488880084888800480048c8c8c8cccd5cd19b8735573aa006900011999111998068018010009bae35742a0066eb8d5d0a8011bad357426ae8940088d4018d4c01ccd5ce2481035054310000849926135744a00226aae7940044dd5000893090009000911091998008020018011000889191800800911980198010010009991999111919191991199991111991199911191919191919991119911919191919199999111119191919191999111999111999999991111111199119999911111991191919191991199119911919999111199119911991199119911919191919191991199119191919191919191919999111199119191191919191919111191919191919192999a983d80510a9999a9831805099835199a8342839183f8009a9aa83d280311000998351991199ab9a3371200400211202110026603860bea00460506a604802444444444400260bea00a660d46601aa00a60c4002a66a610a026603aa010603e002210e0226605260be66026a010603e00260bea0042c2660d46603aa010603e002660d4666a0d0a0e46a6aa0f4a00c440020fa6601aa00a60c40022660d4666a0d0a0e46a6aa0f4a00c440020fa660d46601aa00a60c4002660d46601866026a010603e00260c4002660086a05460bea004a00642a6666a60c60142c2660d46601866026a010a00660c4002660d46605260420026046024660086042002603e00226603aa010603e0022c2a6666a60c40122a66a6108026644666ae68cdc4801000843808440098082800a40042a66a6a0ec605401026102022c442a66a6a0f000226106022c46442a66a6a0f600226a6aa0fc6a6aa0fca0044400444a666a61040200242660e26602800660d2002660e2660606a06260cc0066054032660e2666a0de0ca605000290011a9aa840809a9aa84080a80291000912999a98428080090b10b0999a83883399814980d2805a4004603400442c2660e0666a0dc0c86604c602ea0109001180b8011a9aa840009a9aa84000a80211000912999a98420080090998399980b001983580099839998191a8199834001981600d999a8388339815000a400442c2c4426110022c266aa0fa601200660120022a66a6a0ec605401026104022c4646442a66a6a0f40022a666a60fe6a6aa0faa0064400242660dc66022a00660cc002660dc6605a6a05c60c6a006604e02c666a0d80c4604a002900110b0b1109844008b09a9aa83da80091001098038008b0b0b0a99a9a8369a9816003911a981800111111111111982300500590980e9a981e000910008b0a99a9a83a191a98170009111111111001a802898390b110a99a9a83b0008801110983b0b1191919191299a98438099815803241012179fa042660d86605660c26602aa014a0226054a004660d86605660c26602aa0146a6aa0f8a020440046054a0066605660c26602aa014002605466044660446604400ca004a0066a6aaa050a0084440022660d86605660c26602aa014a0226054a00a6605660c26602aa01400260546604400ca00a26a6aaa04ca00444400626a6aaa04aa0024440042666aaa04a660e40046a6aaa048a01c444002660e40046a6aa0f0a01844002660e40046a60440204444444440062660e20026a6aaa046a01a44400426a6aa0eaa002440042a66a6a0e2604a006260e02c442a66a6a0e60022600600444260e82c46a60766a60720024440064466a60ae0044c4a66a6a0d86a607800844400242a66a6a0da646a605e0024444444444a66a6a0f0666aa609824002a09e46a6aa1080200244a66a612202666ae68cdc7801007849808490089a83e8018a83e001109a83d9a9aa84200800910008a83ca80311919190a99a9a8389999999aba400423333573466e1d40092004233335573ea0084a0ea46666aae7cd5d128029299a9a83a98389aba150062135078308501001150762507607307223333573466e1d400d2002233335573ea00a4a0ec46666aae7cd5d128031299a9a83b18391aba150072135079308701001150772507707407323333573466e1d40112000233335573ea00c46a0f0108024a0ee0e84a0ec9324c93128399283992839928398381099aa83f18108050008b09aab9d5002135573ca00226ea800458584d4c0980048800888cc07cccc158008d4c068020888888888024ccd417dc51a980d004111111111003800a4004446603c6660aa004602e00e666a0bce28d4c06401c8888888880180052002135301600422222222200413535550175001222003135301400222222222200523322300200132233200132001333550023233503b22333503a0030010023503700133503a22230033002001200122337000029001000a400060662400266466aa603a2400244a66a60f06006004266a0d60040022002a0d446a6aaa02e002444660bc666a0b8042602c00c006666a0b80a400290011919a800a834a835091199aa829911a9aa83700111199aa82b911a9aa83900111299a983f999ab9a3370e002900004080840008801899805199aaa81080300100080180180080080191199aa980d890009119aa98060900091a9aa8360009119aa83780119aa98078900091a9aa8378009119aa839001199a9aa80700091980a24000002446602a004002466028002900000099aa98060900091a9aa8360009119aa837801199a9aa805800919aa98080900091a9aa8380009119aa8398011aa80900080091199aaa805011801000919aa98080900091a9aa8380009119aa8398011aa808000800999aaa80280f001000a8341a980f8011111111111199aa981289000911a981d0011111a981f8019119a982d8011299a984300999ab9a3371e0260021100210e02266a0f200a00e200e400ea0e4012222444666aa603624002a0ce66aa60142400246a6aa0d40024466aa0da0046aa018002666aa603624002446a6aa0d600444a66a60f0666aa606c240026466a07844666a6a016006440040040026a6a0120024400266a01244a66a60f400420f820020f246a6aa0dc002446601400400a00c2006266a0d6008006a0d000266aa60142400246a6aa0d4002446466aa0dc006600200a640026aa0f444a66a6a0d600226aa0180064426a6aa0e000444a66a60fa66018004010266aa02200e0022600c00600424424660020060042400222424446006008224424446600400a00822424446002008224002640026aa0da442244a66a6a0c00022a0c444266a0c6600800466aa600c240020080024466e0000800488d4c05400888888888894cd4d4178ccd54c0c84800540d494cd4c1d4ccd5cd19b8f00c0010770761350610011506000321077107523530220012220022353062001222003223370200400246a60c000244400246a600600244444444401046a60040024444444440044444444442466666666600201401201000e00c00a0080060044002222444246660020080060042224002400244666ae68cdc400100082f8300900091a9802000911a98040011111111111299a9a8289980f005005909a9810000911a9812000911199aa980a09000911a98148011111a9817004111a98180029119299a983b99a9826802919a98270021299a983c999ab9a3371e0040020f60f42a00620f440f4466a609c00840f44a66a60f2666ae68cdc780100083d83d0a801883d099a83500500488048a99a9a83000190a99a9a8308011099a9825801119a9826001119a9828001119a9828801119812001000903e919a9828801103e91981200100091103e91119a9827002103e911299a983f199ab9a3370e00c006100020fe2a66a60fc666ae68cdc38028010400083f89982b802000883f883f883c0a99a9a8300009083c083c283080789931a982799ab9c4901024c6600050498c8004d5417088448894cd4d41400044008884cc014008ccd54c01c4800401401000488ccd5cd19b8f00200105c05b2212330010030022001222222222212333333333300100b00a0090080070060050040030022001122123300100300212001122123300100300212001122123300100300212001121222300300411222002112220011200122533335300f0012150372150372150372133355300a12001500d2353005001225335304f5335304f333573466e3cd4c06000888008d4c060010880081441404ccd5cd19b873530180022200135301800422001051050105013503b0031503a003221233001003002200122212333001004003002200122123300100300220013200135504522112225335350390011350060032213335009005300400233355300712001005004001123535004001220011235350030012200213350022253353502b002210031001502a12212330010030021200121222230040052122223003005212222300200521222230010052001221233001003002200121222222230070082212222222330060090082122222223005008122222220041222222200322122222223300200900822122222223300100900820012122300200322212233300100500400320012122300200321223001003200122333573466e1c0080040ac0a88ccc00800522100488100222323230010053200135502c223353501d0014800088d4d54088008894cd4c0bcccd5cd19b8f00200903103013007001130060033200135502b223353501c0014800088d4d54084008894cd4c0b8ccd5cd19b8f00200703002f100113006003112232001320013550292253353501a0011003221330060023004001235301f0012220021222200412222003122220021222200120011200112001225335301d0021001101e2323232323333333574800a46666ae68cdc39aab9d5005480008cccd55cfa8029280691999aab9f50052500e233335573ea00a4a01e46666aae7cd5d128031299a9a807a99a9a807a99a9a80798061aba150092135012223330240030020011501021533535010300d35742a012426a02660040022a0222a02042a66a6a020646666666ae900049404c9404c9404c8d4050dd6801128098081aba150082135013300200115011150102501000d00c00b00a2500c4989402c9402c9402c9402c0204d5d1280089aba25001135573ca00226ea80048ccccccd5d20009280312803128031280311a8039bae00200312001200112122300200311220011200112253335300c0022153335300d00221330050020012130161613015162153335300d0022130161621330050020011301516153335300c001213015162130151610172253353014333573466e3cd4c03c008888008d4c03c0048880080580544ccd5cd19b8735300f00222200135300f00122200101601510152233223370600400266e080092014001262611220021221223300100400312001112212330010030021120012122230030042122230020041222001200122212333001004003002200126262612200212200120011123230010012233003300200200133223322332233333333300248811cd5e6bf0500378d4f0da4e8dde6becec7621cd8cbf5cbb9b87013d4cc0048811c6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d520048810853706163654275640048810b5370616365427564426964003335550044891c826d9fafe1b3acf15bd250de69c04e3fc92c4493785939e069932e8900483001920e209335500648811c88269f8b051a739300fe743a7b315026f4614ce1216a4bb45d7fd0f500482209d20882748203db810920a09c012222222221233333333300100a0090080070060050040030022001111222123330010040030021112001112212330010030021120011").unwrap()
            )
        );

        let script_hash = script.hash();

        let script_base_address = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            StakeCredential::new_script(script_hash),
            stake_cred.clone(),
        );

        let input = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 0),
                TransactionOutput::new(addr_net_0.clone(), Value::from(5_000_000), None, None),
            )
            .payment_key()
            .unwrap()
        };

        let input2 = {
            SingleInputBuilder::new(
                TransactionInput::new(genesis_id(), 1),
                TransactionOutput::new(
                    script_base_address.to_address(),
                    Value::from(5_000_000),
                    None,
                    None,
                ),
            )
            .plutus_script(
                PartialPlutusWitness::new(
                    PlutusScriptWitness::from(script_hash),
                    PlutusData::new_bytes(vec![]),
                ),
                vec![],
                PlutusData::from_cbor_bytes(&hex::decode("D866820380").unwrap()).unwrap(),
            )
            .unwrap()
        };

        let change_cred = StakeCredential::new_pub_key(change_key.to_raw_key().hash());
        let output = TransactionOutputBuilder::new()
            .with_address(BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred.clone(), stake_cred.clone()).to_address())
            .with_reference_script(
                Script::new_plutus_v2(
                    PlutusV2Script::new(
                        hex::decode("59193d020000332332233223232333332222233332222332232333222323332223233333333222222223233322232333322223232332232333222323332223232332233223232333332222233223322332233223322332222323223223232533530343330093333573466e1d401920042304e3055357426aae7940208cccd5cd19b875007480088c140c158d5d09aab9e500923333573466e1d40212000204f235058353059335738921035054310005a49926499263333573466e1d40112006205223333573466e1d40152004205523333573466e1d40192002205323333573466e1d401d2000205623505935305a3357389201035054310005b4992649926498cccd5cd19b8735573aa004900011980619191919191919191919191999ab9a3370e6aae75402920002333333333301a335028232323333573466e1cd55cea8012400046604060766ae854008c0b4d5d09aba25002235066353067335738921035054310006849926135573ca00226ea8004d5d0a80519a8140149aba150093335502f75ca05c6ae854020ccd540bdd728171aba1500733502804435742a00c66a05066aa0aa09aeb4d5d0a8029919191999ab9a3370e6aae754009200023350223232323333573466e1cd55cea80124000466a05466a086eb4d5d0a80118241aba135744a00446a0d46a60d666ae712401035054310006c49926135573ca00226ea8004d5d0a8011919191999ab9a3370e6aae7540092000233502833504375a6ae854008c120d5d09aba2500223506a35306b3357389201035054310006c49926135573ca00226ea8004d5d09aba250022350663530673357389201035054310006849926135573ca00226ea8004d5d0a80219a8143ae35742a00666a05066aa0aaeb88004d5d0a801181d1aba135744a00446a0c46a60c666ae71241035054310006449926135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135573ca00226ea8004d5d0a8011919191999ab9a3370ea00290031180f981e1aba135573ca00646666ae68cdc3a801240084603c608c6ae84d55cf280211999ab9a3370ea00690011180f18189aba135573ca00a46666ae68cdc3a80224000460426eb8d5d09aab9e500623505d35305e3357389201035054310005f49926499264984d55cea80089baa001357426ae8940088d4158d4c15ccd5ce2490350543100058499261057135055353056335738920103505435000574984d55cf280089baa001135573a6ea80044d55cea80089baa0012212330010030022001222222222212333333333300100b00a00900800700600500400300220012212330010030022001122123300100300212001122123300100300212001122123300100300212001212222300400521222230030052122223002005212222300100520011232230023758002640026aa080446666aae7c004940388cd4034c010d5d080118019aba200203f23232323333573466e1cd55cea801a4000466600e6464646666ae68cdc39aab9d5002480008cc034c0c4d5d0a80119a8098169aba135744a00446a0846a608666ae712401035054310004449926135573ca00226ea8004d5d0a801999aa805bae500a35742a00466a01eeb8d5d09aba2500223503e35303f335738921035054310004049926135744a00226aae7940044dd50009110919980080200180110009109198008018011000899aa800bae75a224464460046eac004c8004d540e888c8cccd55cf80112804919a80419aa81718031aab9d5002300535573ca00460086ae8800c0e84d5d08008891001091091198008020018900089119191999ab9a3370ea002900011a80418029aba135573ca00646666ae68cdc3a801240044a01046a06a6a606c66ae7124010350543100037499264984d55cea80089baa001121223002003112200112001232323333573466e1cd55cea8012400046600c600e6ae854008dd69aba135744a00446a05e6a606066ae71241035054310003149926135573ca00226ea80048848cc00400c00880048c8cccd5cd19b8735573aa002900011bae357426aae7940088d40acd4c0b0cd5ce2481035054310002d499261375400224464646666ae68cdc3a800a40084a00e46666ae68cdc3a8012400446a014600c6ae84d55cf280211999ab9a3370ea00690001280511a8171a981799ab9c490103505431000304992649926135573aa00226ea8004484888c00c0104488800844888004480048c8cccd5cd19b8750014800880188cccd5cd19b8750024800080188d4098d4c09ccd5ce2490350543100028499264984d55ce9baa0011220021220012001232323232323333573466e1d4005200c200b23333573466e1d4009200a200d23333573466e1d400d200823300b375c6ae854014dd69aba135744a00a46666ae68cdc3a8022400c46601a6eb8d5d0a8039bae357426ae89401c8cccd5cd19b875005480108cc048c050d5d0a8049bae357426ae8940248cccd5cd19b875006480088c050c054d5d09aab9e500b23333573466e1d401d2000230133016357426aae7940308d40acd4c0b0cd5ce2481035054310002d49926499264992649926135573aa00826aae79400c4d55cf280109aab9e500113754002424444444600e01044244444446600c012010424444444600a010244444440082444444400644244444446600401201044244444446600201201040024646464646666ae68cdc3a800a400446660106eb4d5d0a8021bad35742a0066eb4d5d09aba2500323333573466e1d400920002300a300b357426aae7940188d4070d4c074cd5ce249035054310001e499264984d55cea80189aba25001135573ca00226ea80048488c00800c888488ccc00401401000c80048c8c8cccd5cd19b875001480088c018dd71aba135573ca00646666ae68cdc3a80124000460106eb8d5d09aab9e500423501635301733573892010350543100018499264984d55cea80089baa001212230020032122300100320011122232323333573466e1cd55cea80124000466aa010600c6ae854008c014d5d09aba25002235013353014335738921035054310001549926135573ca00226ea8004448848cc00400c00844800484888c00c01084888c00801048880048004488880104888800c488880084888800480048c8c8c8cccd5cd19b8735573aa006900011999111998068018010009bae35742a0066eb8d5d0a8011bad357426ae8940088d4018d4c01ccd5ce2481035054310000849926135744a00226aae7940044dd5000893090009000911091998008020018011000889191800800911980198010010009991999111919191991199991111991199911191919191919991119911919191919199999111119191919191999111999111999999991111111199119999911111991191919191991199119911919999111199119911991199119911919191919191991199119191919191919191919999111199119191191919191919111191919191919192999a983d80510a9999a9831805099835199a8342839183f8009a9aa83d280311000998351991199ab9a3371200400211202110026603860bea00460506a604802444444444400260bea00a660d46601aa00a60c4002a66a610a026603aa010603e002210e0226605260be66026a010603e00260bea0042c2660d46603aa010603e002660d4666a0d0a0e46a6aa0f4a00c440020fa6601aa00a60c40022660d4666a0d0a0e46a6aa0f4a00c440020fa660d46601aa00a60c4002660d46601866026a010603e00260c4002660086a05460bea004a00642a6666a60c60142c2660d46601866026a010a00660c4002660d46605260420026046024660086042002603e00226603aa010603e0022c2a6666a60c40122a66a6108026644666ae68cdc4801000843808440098082800a40042a66a6a0ec605401026102022c442a66a6a0f000226106022c46442a66a6a0f600226a6aa0fc6a6aa0fca0044400444a666a61040200242660e26602800660d2002660e2660606a06260cc0066054032660e2666a0de0ca605000290011a9aa840809a9aa84080a80291000912999a98428080090b10b0999a83883399814980d2805a4004603400442c2660e0666a0dc0c86604c602ea0109001180b8011a9aa840009a9aa84000a80211000912999a98420080090998399980b001983580099839998191a8199834001981600d999a8388339815000a400442c2c4426110022c266aa0fa601200660120022a66a6a0ec605401026104022c4646442a66a6a0f40022a666a60fe6a6aa0faa0064400242660dc66022a00660cc002660dc6605a6a05c60c6a006604e02c666a0d80c4604a002900110b0b1109844008b09a9aa83da80091001098038008b0b0b0a99a9a8369a9816003911a981800111111111111982300500590980e9a981e000910008b0a99a9a83a191a98170009111111111001a802898390b110a99a9a83b0008801110983b0b1191919191299a98438099815803241012179fa042660d86605660c26602aa014a0226054a004660d86605660c26602aa0146a6aa0f8a020440046054a0066605660c26602aa014002605466044660446604400ca004a0066a6aaa050a0084440022660d86605660c26602aa014a0226054a00a6605660c26602aa01400260546604400ca00a26a6aaa04ca00444400626a6aaa04aa0024440042666aaa04a660e40046a6aaa048a01c444002660e40046a6aa0f0a01844002660e40046a60440204444444440062660e20026a6aaa046a01a44400426a6aa0eaa002440042a66a6a0e2604a006260e02c442a66a6a0e60022600600444260e82c46a60766a60720024440064466a60ae0044c4a66a6a0d86a607800844400242a66a6a0da646a605e0024444444444a66a6a0f0666aa609824002a09e46a6aa1080200244a66a612202666ae68cdc7801007849808490089a83e8018a83e001109a83d9a9aa84200800910008a83ca80311919190a99a9a8389999999aba400423333573466e1d40092004233335573ea0084a0ea46666aae7cd5d128029299a9a83a98389aba150062135078308501001150762507607307223333573466e1d400d2002233335573ea00a4a0ec46666aae7cd5d128031299a9a83b18391aba150072135079308701001150772507707407323333573466e1d40112000233335573ea00c46a0f0108024a0ee0e84a0ec9324c93128399283992839928398381099aa83f18108050008b09aab9d5002135573ca00226ea800458584d4c0980048800888cc07cccc158008d4c068020888888888024ccd417dc51a980d004111111111003800a4004446603c6660aa004602e00e666a0bce28d4c06401c8888888880180052002135301600422222222200413535550175001222003135301400222222222200523322300200132233200132001333550023233503b22333503a0030010023503700133503a22230033002001200122337000029001000a400060662400266466aa603a2400244a66a60f06006004266a0d60040022002a0d446a6aaa02e002444660bc666a0b8042602c00c006666a0b80a400290011919a800a834a835091199aa829911a9aa83700111199aa82b911a9aa83900111299a983f999ab9a3370e002900004080840008801899805199aaa81080300100080180180080080191199aa980d890009119aa98060900091a9aa8360009119aa83780119aa98078900091a9aa8378009119aa839001199a9aa80700091980a24000002446602a004002466028002900000099aa98060900091a9aa8360009119aa837801199a9aa805800919aa98080900091a9aa8380009119aa8398011aa80900080091199aaa805011801000919aa98080900091a9aa8380009119aa8398011aa808000800999aaa80280f001000a8341a980f8011111111111199aa981289000911a981d0011111a981f8019119a982d8011299a984300999ab9a3371e0260021100210e02266a0f200a00e200e400ea0e4012222444666aa603624002a0ce66aa60142400246a6aa0d40024466aa0da0046aa018002666aa603624002446a6aa0d600444a66a60f0666aa606c240026466a07844666a6a016006440040040026a6a0120024400266a01244a66a60f400420f820020f246a6aa0dc002446601400400a00c2006266a0d6008006a0d000266aa60142400246a6aa0d4002446466aa0dc006600200a640026aa0f444a66a6a0d600226aa0180064426a6aa0e000444a66a60fa66018004010266aa02200e0022600c00600424424660020060042400222424446006008224424446600400a00822424446002008224002640026aa0da442244a66a6a0c00022a0c444266a0c6600800466aa600c240020080024466e0000800488d4c05400888888888894cd4d4178ccd54c0c84800540d494cd4c1d4ccd5cd19b8f00c0010770761350610011506000321077107523530220012220022353062001222003223370200400246a60c000244400246a600600244444444401046a60040024444444440044444444442466666666600201401201000e00c00a0080060044002222444246660020080060042224002400244666ae68cdc400100082f8300900091a9802000911a98040011111111111299a9a8289980f005005909a9810000911a9812000911199aa980a09000911a98148011111a9817004111a98180029119299a983b99a9826802919a98270021299a983c999ab9a3371e0040020f60f42a00620f440f4466a609c00840f44a66a60f2666ae68cdc780100083d83d0a801883d099a83500500488048a99a9a83000190a99a9a8308011099a9825801119a9826001119a9828001119a9828801119812001000903e919a9828801103e91981200100091103e91119a9827002103e911299a983f199ab9a3370e00c006100020fe2a66a60fc666ae68cdc38028010400083f89982b802000883f883f883c0a99a9a8300009083c083c283080789931a982799ab9c4901024c6600050498c8004d5417088448894cd4d41400044008884cc014008ccd54c01c4800401401000488ccd5cd19b8f00200105c05b2212330010030022001222222222212333333333300100b00a0090080070060050040030022001122123300100300212001122123300100300212001122123300100300212001121222300300411222002112220011200122533335300f0012150372150372150372133355300a12001500d2353005001225335304f5335304f333573466e3cd4c06000888008d4c060010880081441404ccd5cd19b873530180022200135301800422001051050105013503b0031503a003221233001003002200122212333001004003002200122123300100300220013200135504522112225335350390011350060032213335009005300400233355300712001005004001123535004001220011235350030012200213350022253353502b002210031001502a12212330010030021200121222230040052122223003005212222300200521222230010052001221233001003002200121222222230070082212222222330060090082122222223005008122222220041222222200322122222223300200900822122222223300100900820012122300200322212233300100500400320012122300200321223001003200122333573466e1c0080040ac0a88ccc00800522100488100222323230010053200135502c223353501d0014800088d4d54088008894cd4c0bcccd5cd19b8f00200903103013007001130060033200135502b223353501c0014800088d4d54084008894cd4c0b8ccd5cd19b8f00200703002f100113006003112232001320013550292253353501a0011003221330060023004001235301f0012220021222200412222003122220021222200120011200112001225335301d0021001101e2323232323333333574800a46666ae68cdc39aab9d5005480008cccd55cfa8029280691999aab9f50052500e233335573ea00a4a01e46666aae7cd5d128031299a9a807a99a9a807a99a9a80798061aba150092135012223330240030020011501021533535010300d35742a012426a02660040022a0222a02042a66a6a020646666666ae900049404c9404c9404c8d4050dd6801128098081aba150082135013300200115011150102501000d00c00b00a2500c4989402c9402c9402c9402c0204d5d1280089aba25001135573ca00226ea80048ccccccd5d20009280312803128031280311a8039bae00200312001200112122300200311220011200112253335300c0022153335300d00221330050020012130161613015162153335300d0022130161621330050020011301516153335300c001213015162130151610172253353014333573466e3cd4c03c008888008d4c03c0048880080580544ccd5cd19b8735300f00222200135300f00122200101601510152233223370600400266e080092014001262611220021221223300100400312001112212330010030021120012122230030042122230020041222001200122212333001004003002200126262612200212200120011123230010012233003300200200133223322332233333333300248811cd5e6bf0500378d4f0da4e8dde6becec7621cd8cbf5cbb9b87013d4cc0048811c6bec713b08a2d7c64baa3596d200b41b560850919d72e634944f2d520048810853706163654275640048810b5370616365427564426964003335550044891c826d9fafe1b3acf15bd250de69c04e3fc92c4493785939e069932e8900483001920e209335500648811c88269f8b051a739300fe743a7b315026f4614ce1216a4bb45d7fd0f500482209d20882748203db810920a09c012222222221233333333300100a0090080070060050040030022001111222123330010040030021112001112212330010030021120011").unwrap()
                    )
                )
            )
            .with_data(DatumOption::new_datum(PlutusData::from_cbor_bytes(&hex::decode("d866820181d866820083581c5627217786eb781fbfb51911a253f4d250fdbfdcf1198e70d35985a9443330353301").unwrap()).unwrap()))
            .next().unwrap()
            .with_value(880_000)
            .build().unwrap();

        tx_builder.add_reference_input(TransactionUnspentOutput::new(
            TransactionInput::new(genesis_id(), 1),
            output.output,
        ));
        tx_builder.add_input(input).unwrap();

        tx_builder.add_input(input2).unwrap();

        tx_builder
            .add_output(
                TransactionOutputBuilder::new()
                    .with_address(addr_net_0)
                    .next()
                    .unwrap()
                    .with_value(880_000)
                    .build()
                    .unwrap(),
            )
            .unwrap();
        tx_builder.set_ttl(1000);

        let change_addr =
            BaseAddress::new(NetworkInfo::testnet().network_id(), change_cred, stake_cred)
                .to_address();
        tx_builder
            .add_change_if_needed_for_tests(&change_addr)
            .unwrap();
        assert_eq!(tx_builder.outputs.len(), 2);
        let final_tx = tx_builder
            .build(ChangeSelectionAlgo::Default, &change_addr)
            .unwrap()
            .build_unchecked();

        assert_eq!(final_tx.body.reference_inputs.unwrap().len(), 1);
        assert!(final_tx.witness_set.plutus_v2_scripts.is_none());
        assert!(final_tx.witness_set.plutus_v1_scripts.is_none());
    }
}
