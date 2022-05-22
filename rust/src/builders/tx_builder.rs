use crate::*;
use crate::fees;
use crate::utils;
use super::input_builder::InputBuilderResult;
use super::mint_builder::MintBuilderResult;
use super::certificate_builder::*;
use super::withdrawal_builder::WithdrawalBuilderResult;
use super::witness_builder::InputAggregateWitnessData;
use super::witness_builder::TransactionWitnessSetBuilder;
use std::collections::{BTreeMap, BTreeSet};
use rand::Rng;

// tx_body must be the result of building from tx_builder
// constructs the rest of the Transaction using fake witness data of the correct length
// for use in calculating the size of the final Transaction
fn fake_full_tx(tx_builder: &TransactionBuilder, body: TransactionBody) -> Result<Transaction, JsError> {
    Ok(Transaction {
        body,
        witness_set: tx_builder.witness_set_builder.build(),
        is_valid: true,
        auxiliary_data: tx_builder.auxiliary_data.clone(),
    })
}

fn min_fee(tx_builder: &TransactionBuilder) -> Result<Coin, JsError> {
    let full_tx = fake_full_tx(tx_builder, tx_builder.build()?)?;
    fees::min_fee(&full_tx, &tx_builder.config.fee_algo)
}


#[derive(Clone, Debug)]
struct TxBuilderInput {
    input: TransactionInput,
    amount: Value, // we need to keep track of the amount in the inputs for input selection
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

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilderConfig {
    fee_algo: fees::LinearFee,
    pool_deposit: BigNum,      // protocol parameter
    key_deposit: BigNum,       // protocol parameter
    max_value_size: u32,       // protocol parameter
    max_tx_size: u32,          // protocol parameter
    coins_per_utxo_word: Coin, // protocol parameter
    prefer_pure_change: bool,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct TransactionBuilderConfigBuilder {
    fee_algo: Option<fees::LinearFee>,
    pool_deposit: Option<BigNum>,      // protocol parameter
    key_deposit: Option<BigNum>,       // protocol parameter
    max_value_size: Option<u32>,       // protocol parameter
    max_tx_size: Option<u32>,          // protocol parameter
    coins_per_utxo_word: Option<Coin>, // protocol parameter
    prefer_pure_change: bool,
}

#[wasm_bindgen]
impl TransactionBuilderConfigBuilder {
    pub fn new() -> Self {
        // we have to provide new to expose it to WASM builds
        Self::default()
    }

    pub fn fee_algo(&self, fee_algo: &fees::LinearFee) -> Self {
        let mut cfg = self.clone();
        cfg.fee_algo = Some(fee_algo.clone());
        cfg
    }

    pub fn coins_per_utxo_word(&self, coins_per_utxo_word: &Coin) -> Self {
        let mut cfg = self.clone();
        cfg.coins_per_utxo_word = Some(*coins_per_utxo_word);
        cfg
    }

    pub fn pool_deposit(&self, pool_deposit: &BigNum) -> Self {
        let mut cfg = self.clone();
        cfg.pool_deposit = Some(*pool_deposit);
        cfg
    }

    pub fn key_deposit(&self, key_deposit: &BigNum) -> Self {
        let mut cfg = self.clone();
        cfg.key_deposit = Some(*key_deposit);
        cfg
    }

    pub fn max_value_size(&self, max_value_size: u32) -> Self {
        let mut cfg = self.clone();
        cfg.max_value_size = Some(max_value_size);
        cfg
    }

    pub fn max_tx_size(&self, max_tx_size: u32) -> Self {
        let mut cfg = self.clone();
        cfg.max_tx_size = Some(max_tx_size);
        cfg
    }

    pub fn prefer_pure_change(&self, prefer_pure_change: bool) -> Self {
        let mut cfg = self.clone();
        cfg.prefer_pure_change = prefer_pure_change;
        cfg
    }

    pub fn build(&self) -> Result<TransactionBuilderConfig, JsError> {
        let cfg = self.clone();
        Ok(TransactionBuilderConfig {
            fee_algo: cfg.fee_algo.ok_or_else(|| JsError::from_str("uninitialized field: fee_algo"))?,
            pool_deposit: cfg.pool_deposit.ok_or_else(|| JsError::from_str("uninitialized field: pool_deposit"))?,
            key_deposit: cfg.key_deposit.ok_or_else(|| JsError::from_str("uninitialized field: key_deposit"))?,
            max_value_size: cfg.max_value_size.ok_or_else(|| JsError::from_str("uninitialized field: max_value_size"))?,
            max_tx_size: cfg.max_tx_size.ok_or_else(|| JsError::from_str("uninitialized field: max_tx_size"))?,
            coins_per_utxo_word: cfg.coins_per_utxo_word.ok_or_else(|| JsError::from_str("uninitialized field: coins_per_utxo_word"))?,
            prefer_pure_change: cfg.prefer_pure_change,
        })
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilder {
    config: TransactionBuilderConfig,
    inputs: Vec<TxBuilderInput>,
    outputs: TransactionOutputs,
    fee: Option<Coin>,
    ttl: Option<Slot>, // absolute slot number
    certs: Option<Certificates>,
    withdrawals: Option<Withdrawals>,
    auxiliary_data: Option<AuxiliaryData>,
    validity_start_interval: Option<Slot>,
    mint: Option<Mint>,
    script_data_hash: Option<ScriptDataHash>,
    collateral: Option<TransactionInputs>,
    required_signers: Option<RequiredSigners>,
    network_id: Option<NetworkId>,
    witness_set_builder: TransactionWitnessSetBuilder,
    utxos: Vec<InputBuilderResult>
}

#[wasm_bindgen]
impl TransactionBuilder {
    /// This automatically selects and adds inputs from {inputs} consisting of just enough to cover
    /// the outputs that have already been added.
    /// This should be called after adding all certs/outputs/etc and will be an error otherwise.
    /// Uses CIP2: https://github.com/cardano-foundation/CIPs/blob/master/CIP-0002/CIP-0002.md
    /// Adding a change output must be called after via TransactionBuilder::add_change_if_needed()
    /// This function, diverging from CIP2, takes into account fees and will attempt to add additional
    /// inputs to cover the minimum fees. This does not, however, set the txbuilder's fee.
    pub fn select_utxos(&mut self, strategy: CoinSelectionStrategyCIP2) -> Result<(), JsError> {
        let available_inputs = &self.utxos.clone();
        let mut input_total = self.get_total_input()?;
        let mut output_total = self
            .get_explicit_output()?
            .checked_add(&Value::new(&self.get_deposit()?))?
            .checked_add(&Value::new(&self.min_fee()?))?;
        match strategy {
            CoinSelectionStrategyCIP2::LargestFirst => {
                if self.outputs.0.iter().any(|output| output.amount.multiasset.is_some()) {
                    return Err(JsError::from_str("Multiasset values not supported by LargestFirst. Please use LargestFirstMultiAsset"));
                }
                self.cip2_largest_first_by(
                    available_inputs,
                    &mut (0..available_inputs.len()).collect(),
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin))?;
            },
            CoinSelectionStrategyCIP2::RandomImprove => {
                if self.outputs.0.iter().any(|output| output.amount.multiasset.is_some()) {
                    return Err(JsError::from_str("Multiasset values not supported by RandomImprove. Please use RandomImproveMultiAsset"));
                }
                let mut rng = rand::thread_rng();
                let mut available_indices = (0..available_inputs.len()).collect::<BTreeSet<usize>>();
                self.cip2_random_improve_by(
                    available_inputs,
                    &mut available_indices,
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin),
                    &mut rng)?;
                // Phase 3: add extra inputs needed for fees (not covered by CIP-2)
                // We do this at the end because this new inputs won't be associated with
                // a specific output, so the improvement algorithm we do above does not apply here.
                while input_total.coin < output_total.coin {
                    if available_indices.is_empty() {
                        return Err(JsError::from_str("UTxO Balance Insufficient[x]"));
                    }
                    let i = *available_indices.iter().nth(rng.gen_range(0..available_indices.len())).unwrap();
                    available_indices.remove(&i);
                    let input = &available_inputs[i];
                    let input_fee = self.fee_for_input(&input)?;
                    self.add_input(&input);
                    input_total = input_total.checked_add(&input.utxo_info.amount)?;
                    output_total = output_total.checked_add(&Value::new(&input_fee))?;
                }
            },
            CoinSelectionStrategyCIP2::LargestFirstMultiAsset => {
                // indices into {available_inputs} for inputs that contain {policy_id}:{asset_name}
                let mut available_indices = (0..available_inputs.len()).collect::<Vec<usize>>();
                // run largest-fist by each asset type
                if let Some(ma) = output_total.multiasset.clone() {
                    for (policy_id, assets) in ma.0.iter() {
                        for (asset_name, _) in assets.0.iter() {
                            self.cip2_largest_first_by(
                                available_inputs,
                                &mut available_indices,
                                &mut input_total,
                                &mut output_total,
                                |value| value.multiasset.as_ref()?.get(policy_id)?.get(asset_name))?;
                        }
                    }
                }
                // add in remaining ADA
                self.cip2_largest_first_by(
                    available_inputs,
                    &mut available_indices,
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin))?;
            },
            CoinSelectionStrategyCIP2::RandomImproveMultiAsset => {
                let mut rng = rand::thread_rng();
                let mut available_indices = (0..available_inputs.len()).collect::<BTreeSet<usize>>();
                // run random-improve by each asset type
                if let Some(ma) = output_total.multiasset.clone() {
                    for (policy_id, assets) in ma.0.iter() {
                        for (asset_name, _) in assets.0.iter() {
                            self.cip2_random_improve_by(
                                available_inputs,
                                &mut available_indices,
                                &mut input_total,
                                &mut output_total,
                                |value| value.multiasset.as_ref()?.get(policy_id)?.get(asset_name),
                                &mut rng)?;
                        }
                    }
                }
                // add in remaining ADA
                self.cip2_random_improve_by(
                    available_inputs,
                    &mut available_indices,
                    &mut input_total,
                    &mut output_total,
                    |value| Some(value.coin),
                    &mut rng)?;
                // Phase 3: add extra inputs needed for fees (not covered by CIP-2)
                // We do this at the end because this new inputs won't be associated with
                // a specific output, so the improvement algorithm we do above does not apply here.
                while input_total.coin < output_total.coin {
                    if available_indices.is_empty() {
                        return Err(JsError::from_str("UTxO Balance Insufficient[x]"));
                    }
                    let i = *available_indices.iter().nth(rng.gen_range(0..available_indices.len())).unwrap();
                    available_indices.remove(&i);
                    let input = &available_inputs[i];
                    let input_fee = self.fee_for_input(&input)?;
                    self.add_input(&input);
                    input_total = input_total.checked_add(&input.utxo_info.amount)?;
                    output_total = output_total.checked_add(&Value::new(&input_fee))?;
                }
            },
        }

        Ok(())
    }

    fn cip2_largest_first_by<F>(
        &mut self,
        available_inputs: &[InputBuilderResult],
        available_indices: &mut Vec<usize>,
        input_total: &mut Value,
        output_total: &mut Value,
        by: F) -> Result<(), JsError>
    where
        F: Fn(&Value) -> Option<BigNum> {
        let mut relevant_indices = available_indices.clone();
        relevant_indices.retain(|i| by(&available_inputs[*i].utxo_info.amount).is_some());
        // ordered in ascending order by predicate {by}
        relevant_indices.sort_by_key(|i| by(&available_inputs[*i].utxo_info.amount).expect("filtered above"));

        // iterate in decreasing order for predicate {by}
        for i in relevant_indices.iter().rev() {
            if by(input_total).unwrap_or_else(BigNum::zero) >= by(output_total).expect("do not call on asset types that aren't in the output") {
                break;
            }
            let input = &available_inputs[*i];
            // differing from CIP2, we include the needed fees in the targets instead of just output values
            let input_fee = self.fee_for_input(input)?;
            self.add_input(&input);
            *input_total = input_total.checked_add(&input.utxo_info.amount)?;
            *output_total = output_total.checked_add(&Value::new(&input_fee))?;
            available_indices.swap_remove(available_indices.iter().position(|j| i == j).unwrap());
        }

        if by(input_total).unwrap_or_else(BigNum::zero) < by(output_total).expect("do not call on asset types that aren't in the output") {
            return Err(JsError::from_str("UTxO Balance Insufficient"));
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
        rng: &mut R) -> Result<(), JsError>
    where
        F: Fn(&Value) -> Option<BigNum> {
        // Phase 1: Random Selection
        let mut relevant_indices = available_indices.iter()
            .filter(|i| by(&available_inputs[**i].utxo_info.amount).is_some())
            .cloned()
            .collect::<Vec<usize>>();
        let mut associated_indices: BTreeMap<TransactionOutput, Vec<usize>> = BTreeMap::new();
        let mut outputs = self.outputs.0.iter()
            .filter(|output| by(&output.amount).is_some())
            .cloned()
            .collect::<Vec<TransactionOutput>>();
        outputs.sort_by_key(|output| by(&output.amount).expect("filtered above"));
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
            let mut added = BigNum::zero();
            let needed = by(&output.amount).unwrap();
            while added < needed {
                if relevant_indices.is_empty() {
                    return Err(JsError::from_str("UTxO Balance Insufficient"));
                }
                let random_index = rng.gen_range(0..relevant_indices.len());
                let i = relevant_indices.swap_remove(random_index);
                available_indices.remove(&i);
                let input = &available_inputs[i];
                added = added.checked_add(&by(&input.utxo_info.amount).expect("do not call on asset types that aren't in the output"))?;
                associated_indices.entry(output.clone()).or_default().push(i);
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
                        let cur = from_bignum(&input.utxo_info.amount.coin);
                        let new = from_bignum(&new_input.utxo_info.amount.coin);
                        let min = from_bignum(&output.amount.coin);
                        let ideal = 2 * min;
                        let max = 3 * min;
                        let move_closer = (ideal as i128 - new as i128).abs() < (ideal as i128 - cur as i128).abs();
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
                let input_fee = self.fee_for_input(&input)?;
                self.add_input(&input);
                *input_total = input_total.checked_add(&input.utxo_info.amount)?;
                *output_total = output_total.checked_add(&Value::new(&input_fee))?;
            }
        }

        Ok(())
    }

    pub fn get_witness_set_builder(&self) -> TransactionWitnessSetBuilder {
        self.witness_set_builder.clone()
    }

    pub fn set_witness_set_builder(&mut self, witness_set_builder: TransactionWitnessSetBuilder) {
        self.witness_set_builder = witness_set_builder;
    }

    pub fn add_input(&mut self, result: &InputBuilderResult) {
        self.inputs.push(TxBuilderInput {
            input: result.input.clone(),
            amount: result.utxo_info.amount.clone(),
        });
        if let Some(ref data) = result.aggregate_witness {
            self.witness_set_builder.add_input_aggregate_witness_data(data);
            if let InputAggregateWitnessData::PlutusScript(witness, _, _) = data {
                self.witness_set_builder.add_untagged_redeemer(&RedeemerTag::new_spend(), &witness.untagged_redeemer());
            }
        }
        self.witness_set_builder.add_required_wits(&result.required_wits);
    }

    pub fn add_utxo(&mut self, result: &InputBuilderResult) {
        self.utxos.push(result.clone());
    }

    /// calculates how much the fee would increase if you added a given output
    pub fn fee_for_input(&self, result: &InputBuilderResult) -> Result<Coin, JsError> {
        let mut self_copy = self.clone();

        // we need some value for these for it to be a a valid transaction
        // but since we're only calculating the difference between the fee of two transactions
        // it doesn't matter what these are set as, since it cancels out
        self_copy.set_fee(&to_bignum(0));

        let fee_before = min_fee(&self_copy)?;

        self_copy.add_input(result);
        let fee_after = min_fee(&self_copy)?;
        fee_after.checked_sub(&fee_before)
    }


    /// Add explicit output via a TransactionOutput object
    pub fn add_output(&mut self, output: &TransactionOutput) -> Result<(), JsError> {
        let value_size = output.amount.to_bytes().len();
        if value_size > self.config.max_value_size as usize {
            return Err(JsError::from_str(&format!(
                "Maximum value size of {} exceeded. Found: {}",
                self.config.max_value_size,
                value_size
            )));
        }
        let min_ada = min_ada_required(
            &output.amount(),
            output.data_hash.is_some(),
            &self.config.coins_per_utxo_word,
        )?;
        if output.amount().coin() < min_ada {
            Err(JsError::from_str(&format!(
                "Value {} less than the minimum UTXO value {}",
                from_bignum(&output.amount().coin()),
                from_bignum(&min_ada)
            )))
        } else {
            self.outputs.add(output);
            Ok(())
        }
    }

    /// calculates how much the fee would increase if you added a given output
    pub fn fee_for_output(&self, output: &TransactionOutput) -> Result<Coin, JsError> {
        let mut self_copy = self.clone();

        // we need some value for these for it to be a a valid transaction
        // but since we're only calculating the different between the fee of two transactions
        // it doesn't matter what these are set as, since it cancels out
        self_copy.set_fee(&to_bignum(0));

        let fee_before = min_fee(&self_copy)?;

        self_copy.add_output(output)?;
        let fee_after = min_fee(&self_copy)?;
        fee_after.checked_sub(&fee_before)
    }

    pub fn set_fee(&mut self, fee: &Coin) {
        self.fee = Some(*fee)
    }

    pub fn set_ttl(&mut self, ttl: &Slot) {
        self.ttl = Some(*ttl)
    }

    pub fn set_validity_start_interval(&mut self, validity_start_interval: &Slot) {
        self.validity_start_interval = Some(*validity_start_interval)
    }

    pub fn get_certs(&self) -> Option<Certificates> {
        self.certs.clone()
    }

    pub fn set_certs(&mut self, certs: Certificates) {
        self.certs = Some(certs);
    }

    pub fn add_cert(&mut self, result: &CertificateBuilderResult) {
        let mut certs = self.get_certs().unwrap_or_else(Certificates::new);
        certs.add(&result.cert);
        self.set_certs(certs);
        if let Some(ref data) = result.aggregate_witness {
            self.witness_set_builder.add_input_aggregate_witness_data(data);
            if let InputAggregateWitnessData::PlutusScript(witness, _, _) = data {
                self.witness_set_builder.add_untagged_redeemer(&RedeemerTag::new_cert(), &witness.untagged_redeemer());
            }
        }
        self.witness_set_builder.add_required_wits(&result.required_wits);
    }

    pub fn get_withdrawals(&self) -> Option<Withdrawals> {
        self.withdrawals.clone()
    }

    pub fn set_withdrawals(&mut self, withdrawals: Withdrawals) {
        self.withdrawals = Some(withdrawals);
    }

    pub fn add_withdrawal(&mut self, result: &WithdrawalBuilderResult) {
        let mut withdrawals = self.get_withdrawals().unwrap_or_else(Withdrawals::new);
        withdrawals.insert(&result.address, &result.amount);
        self.set_withdrawals(withdrawals);
        if let Some(ref data) = result.aggregate_witness {
            self.witness_set_builder.add_input_aggregate_witness_data(data);
            if let InputAggregateWitnessData::PlutusScript(witness, _, _) = data {
                self.witness_set_builder.add_untagged_redeemer(&RedeemerTag::new_reward(), &witness.untagged_redeemer());
            }
        }
        self.witness_set_builder.add_required_wits(&result.required_wits);
    }

    pub fn get_auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.auxiliary_data.clone()
    }

    /// Set explicit auxiliary data via an AuxiliaryData object
    /// It might contain some metadata plus native or Plutus scripts
    pub fn set_auxiliary_data(&mut self, auxiliary_data: &AuxiliaryData) {
        self.auxiliary_data = Some(auxiliary_data.clone())
    }

    /// Set metadata using a GeneralTransactionMetadata object
    /// It will be set to the existing or new auxiliary data in this builder
    pub fn set_metadata(&mut self, metadata: &GeneralTransactionMetadata) {
        let mut aux = self.auxiliary_data.as_ref().cloned().unwrap_or_else(AuxiliaryData::new);
        aux.set_metadata(metadata);
        self.set_auxiliary_data(&aux);
    }

    /// Add a single metadatum using TransactionMetadatumLabel and TransactionMetadatum objects
    /// It will be securely added to existing or new metadata in this builder
    pub fn add_metadatum(&mut self, key: &TransactionMetadatumLabel, val: &TransactionMetadatum) {
        let mut metadata = self.auxiliary_data.as_ref()
            .map(|aux| { aux.metadata().as_ref().cloned() })
            .unwrap_or(None)
            .unwrap_or_else(GeneralTransactionMetadata::new);
        metadata.insert(key, val);
        self.set_metadata(&metadata);
    }

    /// Add a single JSON metadatum using a TransactionMetadatumLabel and a String
    /// It will be securely added to existing or new metadata in this builder
    pub fn add_json_metadatum(
        &mut self,
        key: &TransactionMetadatumLabel,
        val: String,
    ) -> Result<(), JsError> {
        self.add_json_metadatum_with_schema(key, val, MetadataJsonSchema::NoConversions)
    }

    /// Add a single JSON metadatum using a TransactionMetadatumLabel, a String, and a MetadataJsonSchema object
    /// It will be securely added to existing or new metadata in this builder
    pub fn add_json_metadatum_with_schema(
        &mut self,
        key: &TransactionMetadatumLabel,
        val: String,
        schema: MetadataJsonSchema,
    ) -> Result<(), JsError> {
        let metadatum = encode_json_str_to_metadatum(val, schema)?;
        self.add_metadatum(key, &metadatum);
        Ok(())
    }

    pub fn add_mint(&mut self, result: &MintBuilderResult) {
        let mut mint = self.get_mint().unwrap_or_else(Mint::new);
        let assets = {
            let mut old_assets = mint.get(&result.policy_id).unwrap_or_else(MintAssets::new);
            let mut new_assets = result.assets.clone();
            old_assets.0.append(&mut new_assets.0);
            old_assets
        };
        mint.insert(&result.policy_id, &assets);
        self.set_mint(mint);
        if let Some(ref data) = result.aggregate_witness {
            self.witness_set_builder.add_input_aggregate_witness_data(data);
            if let InputAggregateWitnessData::PlutusScript(witness, _, _) = data {
                self.witness_set_builder.add_untagged_redeemer(&RedeemerTag::new_mint(), &witness.untagged_redeemer());
            }
        }
        self.witness_set_builder.add_required_wits(&result.required_wits);
    }

    /// Returns a copy of the current mint state in the builder
    pub fn get_mint(&self) -> Option<Mint> {
        self.mint.clone()
    }

    pub fn set_mint(&mut self, mint: Mint) {
        self.mint = Some(mint);
    }

    pub fn new(cfg: &TransactionBuilderConfig) -> Self {
        Self {
            config: cfg.clone(),
            inputs: Vec::new(),
            outputs: TransactionOutputs::new(),
            fee: None,
            ttl: None,
            certs: None,
            withdrawals: None,
            auxiliary_data: None,
            validity_start_interval: None,
            mint: None,
            script_data_hash: None,
            collateral: None,
            required_signers: None,
            network_id: None,
            witness_set_builder: TransactionWitnessSetBuilder::new(),
            utxos: Vec::new()
        }
    }

    pub fn set_script_data_hash(&mut self, script_data_hash: ScriptDataHash) {
        self.script_data_hash = Some(script_data_hash)
    }

    pub fn script_data_hash(&self) -> Option<ScriptDataHash> {
        self.script_data_hash.clone()
    }

    pub fn set_collateral(&mut self, collateral: TransactionInputs) {
        self.collateral = Some(collateral)
    }

    pub fn collateral(&self) -> Option<TransactionInputs> {
        self.collateral.clone()
    }

    pub fn set_required_signers(&mut self, required_signers: RequiredSigners) {
        self.required_signers = Some(required_signers)
    }

    pub fn required_signers(&self) -> Option<RequiredSigners> {
        self.required_signers.clone()
    }

    pub fn set_network_id(&mut self, network_id: NetworkId) {
        self.network_id = Some(network_id)
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        self.network_id
    }

    /// does not include refunds or withdrawals
    pub fn get_explicit_input(&self) -> Result<Value, JsError> {
        self.inputs
            .iter()
            .try_fold(Value::zero(), |acc, tx_builder_input| {
                acc.checked_add(&tx_builder_input.amount)
            })
    }

    /// withdrawals and refunds
    pub fn get_implicit_input(&self) -> Result<Value, JsError> {
        internal_get_implicit_input(
            &self.withdrawals,
            &self.certs,
            &self.config.pool_deposit,
            &self.config.key_deposit,
        )
    }

    /// Returns mint as tuple of (mint_value, burn_value) or two zero values
    fn get_mint_as_values(&self) -> (Value, Value) {
        self.mint.as_ref().map(|m| {
            (Value::new_from_assets(&m.as_positive_multiasset()),
             Value::new_from_assets(&m.as_negative_multiasset()))
        }).unwrap_or((Value::zero(), Value::zero()))
    }

    /// Return explicit input plus implicit input plus mint minus burn
    pub fn get_total_input(&self) -> Result<Value, JsError> {
        let (mint_value, burn_value) = self.get_mint_as_values();
        self.get_explicit_input()?
            .checked_add(&self.get_implicit_input()?)?
            .checked_add(&mint_value)?
            .checked_sub(&burn_value)
    }

    /// does not include fee
    pub fn get_explicit_output(&self) -> Result<Value, JsError> {
        self.outputs
            .0
            .iter()
            .try_fold(Value::new(&to_bignum(0)), |acc, output| {
                acc.checked_add(&output.amount())
            })
    }

    pub fn get_deposit(&self) -> Result<Coin, JsError> {
        internal_get_deposit(
            &self.certs,
            &self.config.pool_deposit,
            &self.config.key_deposit,
        )
    }

    pub fn get_fee_if_set(&self) -> Option<Coin> {
        self.fee
    }

    /// Warning: this function will mutate the /fee/ field
    /// Make sure to call this function last after setting all other tx-body properties
    /// Editing inputs, outputs, mint, etc. after change been calculated
    /// might cause a mismatch in calculated fee versus the required fee
    pub fn add_change_if_needed(&mut self, address: &Address) -> Result<bool, JsError> {
        let fee = match &self.fee {
            None => self.min_fee(),
            // generating the change output involves changing the fee
            Some(_x) => {
                return Err(JsError::from_str(
                    "Cannot calculate change if fee was explicitly specified",
                ))
            }
        }?;

        // note: can't add data_hash to change
        // because we don't know how many change outputs will need to be created
        let data_hash = None;

        let input_total = self.get_total_input()?;

        let output_total = self
            .get_explicit_output()?
            .checked_add(&Value::new(&self.get_deposit()?))?;

        use std::cmp::Ordering;
        match &input_total.partial_cmp(&output_total.checked_add(&Value::new(&fee))?) {
            Some(Ordering::Equal) => {
                // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
                self.set_fee(&input_total.checked_sub(&output_total)?.coin());
                Ok(false)
            },
            Some(Ordering::Less) => Err(JsError::from_str("Insufficient input in transaction")),
            Some(Ordering::Greater) => {
                fn has_assets(ma: Option<MultiAsset>) -> bool {
                    ma.map(|assets| assets.len() > 0).unwrap_or(false)
                }
                let change_estimator = input_total.checked_sub(&output_total)?;
                if has_assets(change_estimator.multiasset()) {
                    fn will_adding_asset_make_output_overflow(output: &TransactionOutput, current_assets: &Assets, asset_to_add: (PolicyID, AssetName, BigNum), max_value_size: u32, coins_per_utxo_word: &Coin) -> bool {
                        let (policy, asset_name, value) = asset_to_add;
                        let mut current_assets_clone = current_assets.clone();
                        current_assets_clone.insert(&asset_name, &value);
                        let mut amount_clone = output.amount.clone();
                        let mut val = Value::new(&Coin::zero());
                        let mut ma = MultiAsset::new();

                        ma.insert(&policy, &current_assets_clone);
                        val.set_multiasset(&ma);
                        amount_clone = amount_clone.checked_add(&val).unwrap();

                        // calculate minADA for more precise max value size
                        let min_ada = min_ada_required(&val, false, coins_per_utxo_word).unwrap();
                        amount_clone.set_coin(&min_ada);

                        amount_clone.to_bytes().len() > max_value_size as usize
                    }
                    fn pack_nfts_for_change(max_value_size: u32, coins_per_utxo_word: &Coin, change_address: &Address, change_estimator: &Value, data_hash: Option<DataHash>) -> Result<Vec<MultiAsset>, JsError> {
                        // we insert the entire available ADA temporarily here since that could potentially impact the size
                        // as it could be 1, 2 3 or 4 bytes for Coin.
                        let mut change_assets: Vec<MultiAsset> = Vec::new();

                        let mut base_coin = Value::new(&change_estimator.coin());
                        base_coin.set_multiasset(&MultiAsset::new());
                        let mut output = TransactionOutput {
                            address: change_address.clone(),
                            amount: base_coin.clone(),
                            data_hash: data_hash.clone(),
                        };
                        // If this becomes slow on large TXs we can optimize it like the following
                        // to avoid cloning + reserializing the entire output.
                        // This would probably be more relevant if we use a smarter packing algorithm
                        // which might need to compare more size differences than greedy
                        //let mut bytes_used = output.to_bytes().len();

                        // a greedy packing is done here to avoid an exponential bin-packing
                        // which in most cases likely shouldn't be the difference between
                        // having an extra change output or not unless there are gigantic
                        // differences in NFT policy sizes
                        for (policy, assets) in change_estimator.multiasset().unwrap().0.iter() {
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
                            let mut old_amount = output.amount.clone();
                            let mut val = Value::new(&Coin::zero());
                            let mut next_nft = MultiAsset::new();

                            let asset_names = assets.keys();
                            let mut rebuilt_assets = Assets::new();
                            for n in 0..asset_names.len() {
                                let asset_name = asset_names.get(n);
                                let value = assets.get(&asset_name).unwrap();

                                if will_adding_asset_make_output_overflow(&output, &rebuilt_assets, (policy.clone(), asset_name.clone(), value), max_value_size, coins_per_utxo_word) {
                                    // if we got here, this means we will run into a overflow error,
                                    // so we want to split into multiple outputs, for that we...

                                    // 1. insert the current assets as they are, as this won't overflow
                                    next_nft.insert(policy, &rebuilt_assets);
                                    val.set_multiasset(&next_nft);
                                    output.amount = output.amount.checked_add(&val)?;
                                    change_assets.push(output.amount.multiasset().unwrap());

                                    // 2. create a new output with the base coin value as zero
                                    base_coin = Value::new(&Coin::zero());
                                    base_coin.set_multiasset(&MultiAsset::new());
                                    output = TransactionOutput {
                                        address: change_address.clone(),
                                        amount: base_coin.clone(),
                                        data_hash: data_hash.clone(),
                                    };

                                    // 3. continue building the new output from the asset we stopped
                                    old_amount = output.amount.clone();
                                    val = Value::new(&Coin::zero());
                                    next_nft = MultiAsset::new();

                                    rebuilt_assets = Assets::new();
                                }

                                rebuilt_assets.insert(&asset_name, &value);
                            }

                            next_nft.insert(policy, &rebuilt_assets);
                            val.set_multiasset(&next_nft);
                            output.amount = output.amount.checked_add(&val)?;

                            // calculate minADA for more precise max value size
                            let mut amount_clone = output.amount.clone();
                            let min_ada = min_ada_required(&val, false, coins_per_utxo_word).unwrap();
                            amount_clone.set_coin(&min_ada);

                            if amount_clone.to_bytes().len() > max_value_size as usize {
                                output.amount = old_amount;
                                break;
                            }
                        }
                        change_assets.push(output.amount.multiasset().unwrap());
                        Ok(change_assets)
                    }
                    let mut change_left = input_total.checked_sub(&output_total)?;
                    let mut new_fee = fee;
                    // we might need multiple change outputs for cases where the change has many asset types
                    // which surpass the max UTXO size limit
                    let minimum_utxo_val = min_pure_ada(&self.config.coins_per_utxo_word, data_hash.is_some())?;
                    while let Some(Ordering::Greater) = change_left.multiasset.as_ref().map_or_else(|| None, |ma| ma.partial_cmp(&MultiAsset::new())) {
                        let nft_changes = pack_nfts_for_change(self.config.max_value_size, &self.config.coins_per_utxo_word, address, &change_left, data_hash.clone())?;
                        if nft_changes.is_empty() {
                            // this likely should never happen
                            return Err(JsError::from_str("NFTs too large for change output"));
                        }
                        // we only add the minimum needed (for now) to cover this output
                        let mut change_value = Value::new(&Coin::zero());
                        for nft_change in nft_changes.iter() {
                            change_value.set_multiasset(nft_change);
                            let min_ada = min_ada_required(&change_value, data_hash.is_some(), &self.config.coins_per_utxo_word)?;
                            change_value.set_coin(&min_ada);
                            let change_output = TransactionOutput {
                                address: address.clone(),
                                amount: change_value.clone(),
                                data_hash: data_hash.clone(),
                            };
                            // increase fee
                            let fee_for_change = self.fee_for_output(&change_output)?;
                            new_fee = new_fee.checked_add(&fee_for_change)?;
                            if change_left.coin() < min_ada.checked_add(&new_fee)? {
                                return Err(JsError::from_str("Not enough ADA leftover to include non-ADA assets in a change address"));
                            }
                            change_left = change_left.checked_sub(&change_value)?;
                            self.add_output(&change_output)?;
                        }
                    }
                    change_left = change_left.checked_sub(&Value::new(&new_fee))?;
                    // add potentially a separate pure ADA change output
                    let left_above_minimum = change_left.coin.compare(&minimum_utxo_val) > 0;
                    if self.config.prefer_pure_change && left_above_minimum {
                        let pure_output = TransactionOutput {
                            address: address.clone(),
                            amount: change_left.clone(),
                            data_hash: data_hash.clone(),
                        };
                        let additional_fee = self.fee_for_output(&pure_output)?;
                        let potential_pure_value = change_left.checked_sub(&Value::new(&additional_fee))?;
                        let potential_pure_above_minimum = potential_pure_value.coin.compare(&minimum_utxo_val) > 0;
                        if potential_pure_above_minimum {
                            new_fee = new_fee.checked_add(&additional_fee)?;
                            change_left = Value::zero();
                            self.add_output(&TransactionOutput {
                                address: address.clone(),
                                amount: potential_pure_value.clone(),
                                data_hash: data_hash.clone(),
                            })?;
                        }
                    }
                    self.set_fee(&new_fee);
                    // add in the rest of the ADA
                    if !change_left.is_zero() {
                        self.outputs.0.last_mut().unwrap().amount = self.outputs.0.last().unwrap().amount.checked_add(&change_left)?;
                    }
                    Ok(true)
                } else {
                    let min_ada = min_ada_required(
                        &change_estimator,
                        data_hash.is_some(),
                        &self.config.coins_per_utxo_word,
                    )?;
                    // no-asset case so we have no problem burning the rest if there is no other option
                    fn burn_extra(builder: &mut TransactionBuilder, burn_amount: &BigNum) -> Result<bool, JsError> {
                        // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
                        builder.set_fee(burn_amount);
                        Ok(false) // not enough input to covert the extra fee from adding an output so we just burn whatever is left
                    }
                    match change_estimator.coin() >= min_ada {
                        false => burn_extra(self, &change_estimator.coin()),
                        true => {
                            // check how much the fee would increase if we added a change output
                            let fee_for_change = self.fee_for_output(&TransactionOutput {
                                address: address.clone(),
                                amount: change_estimator.clone(),
                                data_hash: data_hash.clone(),
                            })?;

                            let new_fee = fee.checked_add(&fee_for_change)?;
                            match change_estimator.coin() >= min_ada.checked_add(&Value::new(&new_fee).coin())? {
                                false => burn_extra(self, &change_estimator.coin()),
                                true => {
                                    // recall: min_fee assumed the fee was the maximum possible so we definitely have enough input to cover whatever fee it ends up being
                                    self.set_fee(&new_fee);

                                    self.add_output(&TransactionOutput {
                                        address: address.clone(),
                                        amount: change_estimator.checked_sub(&Value::new(&new_fee.clone()))?,
                                        data_hash: data_hash.clone(),
                                    })?;

                                    Ok(true)
                                }
                            }
                        }
                    }
                }
            }
            None => Err(JsError::from_str("missing input or output for some native asset")),
        }
    }

    fn build_and_size(&self) -> Result<(TransactionBody, usize), JsError> {
        let fee = self.fee.ok_or_else(|| JsError::from_str("Fee not specified"))?;
        let built = TransactionBody {
            inputs: TransactionInputs(self.inputs.iter().map(|tx_builder_input| tx_builder_input.input.clone()).collect()),
            outputs: self.outputs.clone(),
            fee,
            ttl: self.ttl,
            certs: self.certs.clone(),
            withdrawals: self.withdrawals.clone(),
            update: None,
            auxiliary_data_hash: self.auxiliary_data.as_ref().map(utils::hash_auxiliary_data),
            validity_start_interval: self.validity_start_interval,
            mint: self.mint.clone(),
            script_data_hash: self.script_data_hash.clone(),
            collateral: self.collateral.clone(),
            required_signers: self.required_signers.clone(),
            network_id: self.network_id,
        };
        // we must build a tx with fake data (of correct size) to check the final Transaction size
        let full_tx = fake_full_tx(self, built)?;
        let full_tx_size = full_tx.to_bytes().len();
        Ok((full_tx.body, full_tx_size))
    }

    pub fn full_size(&self) -> Result<usize, JsError> {
        self.build_and_size().map(|r| { r.1 })
    }

    pub fn output_sizes(&self) -> Vec<usize> {
        self.outputs.0.iter().map(|o| { o.to_bytes().len() }).collect()
    }

    /// Returns object the body of the new transaction
    /// Auxiliary data itself is not included
    /// You can use `get_auxiliary_data` or `build_tx`
    pub fn build(&self) -> Result<TransactionBody, JsError> {
        let (body, full_tx_size) = self.build_and_size()?;
        if full_tx_size > self.config.max_tx_size as usize {
            Err(JsError::from_str(&format!(
                "Maximum transaction size of {} exceeded. Found: {}",
                self.config.max_tx_size,
                full_tx_size
            )))
        } else {
            Ok(body)
        }
    }

    /// Returns full Transaction object with the body and the auxiliary data
    /// NOTE: witness_set will contain all mint_scripts if any been added or set
    /// NOTE: is_valid set to true
    pub fn build_tx(&self) -> Result<Transaction, JsError> {
        Ok(Transaction {
            body: self.build()?,
            witness_set: self.witness_set_builder.try_build()?,
            is_valid: true,
            auxiliary_data: self.auxiliary_data.clone(),
        })
    }

    /// warning: sum of all parts of a transaction must equal 0. You cannot just set the fee to the min value and forget about it
    /// warning: min_fee may be slightly larger than the actual minimum fee (ex: a few lovelaces)
    /// this is done to simplify the library code, but can be fixed later
    pub fn min_fee(&self) -> Result<Coin, JsError> {
        let mut self_copy = self.clone();
        self_copy.set_fee(&to_bignum(0x1_00_00_00_00));
        min_fee(&self_copy)
    }
}

#[cfg(test)]
mod tests {
    use crate::builders::{mint_builder::SingleMintBuilder, witness_builder::NativeScriptWitnessInfo, input_builder::SingleInputBuilder};

    use super::*;
    use fees::*;
    use super::builders::output_builder::TransactionOutputBuilder;

    const MAX_VALUE_SIZE: u32 = 4000;
    const MAX_TX_SIZE: u32 = 8000; // might be out of date but suffices for our tests
    // this is what is used in mainnet
    static COINS_PER_UTXO_WORD: u64 = 34_482;

    fn genesis_id() -> TransactionHash {
        TransactionHash::from([0u8; TransactionHash::BYTE_COUNT])
    }

    fn root_key_15() -> Bip32PrivateKey {
        // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
        let entropy = [0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12];
        Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
    }

    fn fake_key_hash(x: u8) -> Ed25519KeyHash {
        Ed25519KeyHash::from_bytes(
            vec![x, 239, 181, 120, 142, 135, 19, 200, 68, 223, 211, 43, 46, 145, 222, 30, 48, 159, 239, 255, 213, 85, 248, 39, 204, 158, 225, 100]
        ).unwrap()
    }

    fn harden(index: u32) -> u32 {
        index | 0x80_00_00_00
    }

    fn byron_address() -> Address {
        ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address()
    }

    fn create_linear_fee(coefficient: u64, constant: u64) -> LinearFee {
        LinearFee::new(&to_bignum(coefficient), &to_bignum(constant))
    }

    fn create_default_linear_fee() -> LinearFee {
        create_linear_fee(500, 2)
    }

    fn create_tx_builder_full(
        linear_fee: &LinearFee,
        pool_deposit: u64,
        key_deposit: u64,
        max_val_size: u32,
        coins_per_utxo_word: u64,
    ) -> TransactionBuilder {
        let cfg = TransactionBuilderConfigBuilder::default()
            .fee_algo(linear_fee)
            .pool_deposit(&to_bignum(pool_deposit))
            .key_deposit(&to_bignum(key_deposit))
            .max_value_size(max_val_size)
            .max_tx_size(MAX_TX_SIZE)
            .coins_per_utxo_word(&to_bignum(coins_per_utxo_word))
            .build()
            .unwrap();
        TransactionBuilder::new(&cfg)
    }

    fn create_tx_builder(
        linear_fee: &LinearFee,
        coins_per_utxo_word: u64,
        pool_deposit: u64,
        key_deposit: u64,
    ) -> TransactionBuilder {
        create_tx_builder_full(linear_fee, pool_deposit, key_deposit, MAX_VALUE_SIZE, coins_per_utxo_word)
    }

    fn create_reallistic_tx_builder() -> TransactionBuilder {
        create_tx_builder(
            &create_linear_fee(44, 155381),
            COINS_PER_UTXO_WORD,
            500000000,
            2000000,
        )
    }

    fn create_tx_builder_with_fee_and_val_size(linear_fee: &LinearFee, max_val_size: u32) -> TransactionBuilder {
        create_tx_builder_full(linear_fee, 1, 1, max_val_size, 1)
    }

    fn create_tx_builder_with_fee(linear_fee: &LinearFee) -> TransactionBuilder {
        create_tx_builder(linear_fee, 1, 1, 1)
    }

    fn create_tx_builder_with_fee_and_pure_change(linear_fee: &LinearFee) -> TransactionBuilder {
        TransactionBuilder::new(&TransactionBuilderConfigBuilder::default()
            .fee_algo(linear_fee)
            .pool_deposit(&to_bignum(1))
            .key_deposit(&to_bignum(1))
            .max_value_size(MAX_VALUE_SIZE)
            .max_tx_size(MAX_TX_SIZE)
            .coins_per_utxo_word(&to_bignum(1))
            .prefer_pure_change(true)
            .build()
            .unwrap())
    }

    fn create_tx_builder_with_key_deposit(deposit: u64) -> TransactionBuilder {
        create_tx_builder(&create_default_linear_fee(), 1, 1, deposit)
    }

    fn create_default_tx_builder() -> TransactionBuilder {
        create_tx_builder_with_fee(&create_default_linear_fee())
    }

    fn create_account() -> ((Bip32PublicKey, StakeCredential), (Bip32PublicKey, StakeCredential), Address) {
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

        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let address = BaseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred, &stake_cred).to_address();

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
        let ((spend, _), (_, stake_cred), addr_net_0) = create_account();

        let input = {
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&addr_net_0, &Value::new(&to_bignum(1_000_000)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_coin(&to_bignum(29))
                .build().unwrap()
            ).unwrap();
        tx_builder.set_ttl(&1000.into());

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr
        );
        assert!(added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 2);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );
        assert_eq!(tx_builder.full_size().unwrap(), 285);
        assert_eq!(tx_builder.output_sizes(), vec![62, 65]);
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
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
        let ((spend, _), (_, stake_cred), addr_net_0) = create_account();
        let input = {
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&addr_net_0, &Value::new(&to_bignum(1_000_000)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_coin(&to_bignum(880_000))
                .build().unwrap()
            ).unwrap();
        tx_builder.set_ttl(&1000.into());

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr
        );
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
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
        let ((spend, _), (stake, stake_cred), addr_net_0) = create_account();

        let input = {
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&addr_net_0, &Value::new(&to_bignum(5_000_000)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);
        tx_builder.set_ttl(&1000.into());

        let cert = SingleCertificateBuilder::new(&Certificate::new_stake_registration(&StakeRegistration::new(&stake_cred)))
            .vkey(&Vkey::new(&stake.to_raw_key()))
            .unwrap();
        tx_builder.add_cert(&cert);

        let cert = SingleCertificateBuilder::new(&Certificate::new_stake_delegation(&StakeDelegation::new(
            &stake_cred,
            &stake.to_raw_key().hash(), // in reality, this should be the pool owner's key, not ours
        )))
            .vkey(&Vkey::new(&stake.to_raw_key()))
            .unwrap();
        tx_builder.add_cert(&cert);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        tx_builder.add_change_if_needed(
            &change_addr
        ).unwrap();
        assert_eq!(tx_builder.min_fee().unwrap().to_str(), "214002");
        assert_eq!(tx_builder.get_fee_if_set().unwrap().to_str(), "214002");
        assert_eq!(tx_builder.get_deposit().unwrap().to_str(), "1000000");
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder
                .get_explicit_output().unwrap()
                .checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
                .checked_add(&Value::new(&tx_builder.get_deposit().unwrap())).unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_exact_amount() {
        // transactions where sum(input) == sum(output) exact should pass
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let ((spend, _), (_, stake_cred), addr_net_0) = create_account();

        let input = {
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&addr_net_0, &Value::new(&to_bignum(100)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_coin(&to_bignum(100))
                .build().unwrap()
            ).unwrap();
        tx_builder.set_ttl(&0.into());

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr
        ).unwrap();
        assert!(!added_change);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 1);
    }

    #[test]
    fn build_tx_exact_change() {
        // transactions where we have exactly enough ADA to add change should pass
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let ((spend, _), (_, stake_cred), addr_net_0) = create_account();

        let input = {
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&addr_net_0, &Value::new(&to_bignum(58)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_coin(&to_bignum(29))
                .build().unwrap()
            ).unwrap();
        tx_builder.set_ttl(&0.into());

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr
        ).unwrap();
        assert!(added_change);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 2);
        assert_eq!(final_tx.outputs().get(1).amount().coin().to_str(), "29");
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
        let ((spend, _), (stake, stake_cred), addr_net_0) = create_account();

        let input = {
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&addr_net_0, &Value::new(&to_bignum(5)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_coin(&to_bignum(5))
                .build().unwrap()
            ).unwrap();
        tx_builder.set_ttl(&0.into());

        // add a cert which requires a deposit
        let cert = SingleCertificateBuilder::new(&Certificate::new_stake_registration(&StakeRegistration::new(&stake_cred)))
            .vkey(&Vkey::new(&stake.to_raw_key()))
            .unwrap();
        tx_builder.add_cert(&cert);

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        tx_builder.add_change_if_needed(&change_addr).unwrap();
    }

    #[test]
    fn build_tx_with_inputs() {
        let mut tx_builder = create_default_tx_builder();
        let ((spend, spend_cred), (_, stake_cred), _) = create_account();

        let input = {
            let address = &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &Value::new(&to_bignum(1_000_000)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        assert_eq!(tx_builder.fee_for_input(&input).unwrap().to_str(), "69500");
        tx_builder.add_input(&input);

        let input = {
            let address = &BaseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred, &stake_cred).to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &Value::new(&to_bignum(1_000_000)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);

        let input = {
            let address = &PointerAddress::new(
                NetworkInfo::testnet().network_id(),
                &spend_cred,
                &Pointer::new(&to_bignum(0), &to_bignum(0), &to_bignum(0))
            ).to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &Value::new(&to_bignum(1_000_000)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);

        let input = {
            let address = &ByronAddress::icarus_from_key(&spend, NetworkInfo::testnet().protocol_magic()).to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &Value::new(&to_bignum(1_000_000)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);

        assert_eq!(tx_builder.inputs.len(), 4);
    }

    #[test]
    fn build_tx_with_mint_all_sent() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let ((spend, spend_cred), (_, stake_cred), _) = create_account();

        // Input with 150 coins
        let input = {
            let address = &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &Value::new(&to_bignum(150)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

        let (min_script, policy_id) = mint_script_and_policy(0);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let amount = to_bignum(1234);

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name, Int::new(&amount)))
            .native_script(&min_script, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        let mut ass = Assets::new();
        ass.insert(&name, &amount);
        let mut mass = MultiAsset::new();
        mass.insert(&policy_id, &ass);

        // One coin and the minted asset goes into the output
        let mut output_amount = Value::new(&to_bignum(50));
        output_amount.set_multiasset(&mass);

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_value(&output_amount)
                .build().unwrap()
            ).unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(added_change);
        assert_eq!(tx_builder.outputs.len(), 2);

        // Change must be one remaining coin because fee is one constant coin
        let change = tx_builder.outputs.get(1).amount();
        assert_eq!(change.coin(), to_bignum(99));
        assert!(change.multiasset().is_none());
    }

    #[test]
    fn build_tx_with_mint_in_change() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let ((spend, spend_cred), (_, stake_cred), _) = create_account();

        // Input with 150 coins
        let input = {
            let address = &EnterpriseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred).to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &Value::new(&to_bignum(150)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);

        let addr_net_0 = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();

        let (min_script, policy_id) = mint_script_and_policy(0);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let amount_minted = to_bignum(1000);
        let amount_sent = to_bignum(500);

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name, Int::new(&amount_minted)))
            .native_script(&min_script, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        let mut ass = Assets::new();
        ass.insert(&name, &amount_sent);
        let mut mass = MultiAsset::new();
        mass.insert(&policy_id, &ass);

        // One coin and the minted asset goes into the output
        let mut output_amount = Value::new(&to_bignum(50));
        output_amount.set_multiasset(&mass);

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_value(&output_amount)
                .build().unwrap()
            ).unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(added_change);
        assert_eq!(tx_builder.outputs.len(), 2);

        // Change must be one remaining coin because fee is one constant coin
        let change = tx_builder.outputs.get(1).amount();
        assert_eq!(change.coin(), to_bignum(99));
        assert!(change.multiasset().is_some());

        let change_assets = change.multiasset().unwrap();
        let change_asset = change_assets.get(&policy_id).unwrap();
        assert_eq!(
            change_asset.get(&name).unwrap(),
            amount_minted.checked_sub(&amount_sent).unwrap(),
        );
    }

    #[test]
    fn build_tx_with_native_assets_change() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let ((spend, _), (_, stake_cred), addr_net_0) = create_account();

        let policy_id = &PolicyID::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let ma_input1 = 100;
        let ma_input2 = 200;
        let ma_output1 = 60;

        let multiassets = [ma_input1, ma_input2, ma_output1]
            .iter()
            .map(|input| {
                let mut multiasset = MultiAsset::new();
                multiasset.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(*input));
                    assets
                });
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets
            .iter()
            .zip([100u64, 100].iter().cloned().map(to_bignum))
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            let input = {
                let builder = SingleInputBuilder::new(
                    &TransactionInput::new(&genesis_id(), &0.into()),
                    &TransactionOutput::new(&addr_net_0, &input_amount)
                );
                let vkey = Vkey::new(&spend.to_raw_key());
                builder.vkey(&vkey).unwrap()
            };
            tx_builder.add_input(&input);
        }

        let mut output_amount = Value::new(&to_bignum(100));
        output_amount.set_multiasset(&multiassets[2]);

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_value(&output_amount)
                .build().unwrap()
            ).unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(added_change);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 2);
        assert_eq!(
            final_tx
                .outputs()
                .get(1)
                .amount()
                .multiasset()
                .unwrap()
                .get(policy_id)
                .unwrap()
                .get(&name)
                .unwrap(),
            to_bignum(ma_input1 + ma_input2 - ma_output1)
        );
        assert_eq!(
            final_tx.outputs().get(1).amount().coin(),
            to_bignum(99)
        );
    }

    #[test]
    fn build_tx_with_native_assets_change_and_purification() {
        let coin_per_utxo_word = to_bignum(1);
        // Prefer pure change!
        let mut tx_builder = create_tx_builder_with_fee_and_pure_change(&create_linear_fee(0, 1));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let ((spend, _), (_, stake_cred), addr_net_0) = create_account();

        let policy_id = &PolicyID::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let ma_input1 = 100;
        let ma_input2 = 200;
        let ma_output1 = 60;

        let multiassets = [ma_input1, ma_input2, ma_output1]
            .iter()
            .map(|input| {
                let mut multiasset = MultiAsset::new();
                multiasset.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(*input));
                    assets
                });
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets
            .iter()
            .zip([100u64, 100].iter().cloned().map(to_bignum))
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            let input = {
                let builder = SingleInputBuilder::new(
                    &TransactionInput::new(&genesis_id(), &0.into()),
                    &TransactionOutput::new(&addr_net_0, &input_amount)
                );
                let vkey = Vkey::new(&spend.to_raw_key());
                builder.vkey(&vkey).unwrap()
            };
            tx_builder.add_input(&input);
        }

        let mut output_amount = Value::new(&to_bignum(50));
        output_amount.set_multiasset(&multiassets[2]);

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_value(&output_amount)
                .build().unwrap()
            ).unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(added_change);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 3);
        assert_eq!(
            final_tx.outputs().get(0).amount().coin(),
            to_bignum(50)
        );
        assert_eq!(
            final_tx
                .outputs()
                .get(1)
                .amount()
                .multiasset()
                .unwrap()
                .get(policy_id)
                .unwrap()
                .get(&name)
                .unwrap(),
            to_bignum(ma_input1 + ma_input2 - ma_output1)
        );
        // The first change output that contains all the tokens contain minimum required Coin
        let min_coin_for_dirty_change = min_ada_required(
            &final_tx.outputs().get(1).amount(),
            false,
            &coin_per_utxo_word,
        ).unwrap();
        assert_eq!(
            final_tx.outputs().get(1).amount().coin(),
            min_coin_for_dirty_change
        );
        assert_eq!(
            final_tx.outputs().get(2).amount().coin(),
            to_bignum(110)
        );
        assert_eq!(
            final_tx.outputs().get(2).amount().multiasset(),
            None
        );
    }

    #[test]
    fn build_tx_with_native_assets_change_and_no_purification_cuz_not_enough_pure_coin() {
        // Prefer pure change!
        let mut tx_builder = create_tx_builder_with_fee_and_pure_change(&create_linear_fee(1, 1));
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let ((spend, _), (_, stake_cred), addr_net_0) = create_account();

        let policy_id = &PolicyID::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        let ma_input1 = 100;
        let ma_input2 = 200;
        let ma_output1 = 60;

        let multiassets = [ma_input1, ma_input2, ma_output1]
            .iter()
            .map(|input| {
                let mut multiasset = MultiAsset::new();
                multiasset.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(*input));
                    assets
                });
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets
            .iter()
            .zip([300u64, 300].iter().cloned().map(to_bignum))
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            let input = {
                let builder = SingleInputBuilder::new(
                    &TransactionInput::new(&genesis_id(), &0.into()),
                    &TransactionOutput::new(&addr_net_0, &input_amount)
                );
                let vkey = Vkey::new(&spend.to_raw_key());
                builder.vkey(&vkey).unwrap()
            };
            tx_builder.add_input(&input);
        }

        let mut output_amount = Value::new(&to_bignum(100));
        output_amount.set_multiasset(&multiassets[2]);

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_value(&output_amount)
                .build().unwrap()
            ).unwrap();

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &change_cred,
            &stake_cred,
        )
        .to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(added_change);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 2);
        assert_eq!(
            final_tx.outputs().get(0).amount().coin(),
            to_bignum(100)
        );
        assert_eq!(
            final_tx
                .outputs()
                .get(1)
                .amount()
                .multiasset()
                .unwrap()
                .get(policy_id)
                .unwrap()
                .get(&name)
                .unwrap(),
            to_bignum(ma_input1 + ma_input2 - ma_output1)
        );
        // The single change output contains more Coin then minimal utxo value
        // But not enough to cover the additional fee for a separate output
        assert_eq!(
            final_tx.outputs().get(1).amount().coin(),
            to_bignum(101)
        );
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
        let ((spend, _), (_, stake_cred), addr_net_0) = create_account();

        // add an input that contains an asset not present in the output
        let policy_id = &PolicyID::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let mut input_amount = Value::new(&to_bignum(1_000_000));
        let mut input_multiasset = MultiAsset::new();
        input_multiasset.insert(policy_id, &{
            let mut assets = Assets::new();
            assets.insert(&name, &to_bignum(100));
            assets
        });
        input_amount.set_multiasset(&input_multiasset);
        let input = {
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&addr_net_0, &input_amount)
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_coin(&to_bignum(880_000))
                .build().unwrap()
            ).unwrap();
        tx_builder.set_ttl(&1000.into());

        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let change_addr = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr
        );
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_burn_less_than_min_ada() {
        // with this mainnet value we should end up with a final min_ada_required of just under 1_000_000
        let mut tx_builder = create_reallistic_tx_builder();

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap();
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr.to_address())
                .next().unwrap()
                .with_value(&Value::new(&to_bignum(2_000_000)))
                .build().unwrap()
            ).unwrap();

        let input = {
            let address = &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &Value::new(&to_bignum(2_400_000)))
            );
            builder.skip_witness().unwrap()
        };
        tx_builder.add_input(&input);

        tx_builder.set_ttl(&1.into());

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr.to_address()
        );
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_burn_empty_assets() {
        let mut tx_builder = create_reallistic_tx_builder();

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap();
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr.to_address())
                .next().unwrap()
                .with_value(&Value::new(&to_bignum(2_000_000)))
                .build().unwrap()
            ).unwrap();

        let mut input_value = Value::new(&to_bignum(2_400_000));
        input_value.set_multiasset(&MultiAsset::new());
        let input = {
            let address = &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &input_value)
            );
            builder.skip_witness().unwrap()
        };
        tx_builder.add_input(&input);

        tx_builder.set_ttl(&1.into());

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr.to_address()
        );
        assert!(!added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap().coin(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap().coin()
        );
        let _final_tx = tx_builder.build(); // just test that it doesn't throw
    }

    #[test]
    fn build_tx_no_useless_multiasset() {
        let mut tx_builder = create_reallistic_tx_builder();

        let policy_id = &PolicyID::from([0u8; 28]);
        let name = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();

        // add an output that uses up all the token but leaves ADA
        let mut input_amount = Value::new(&to_bignum(5_000_000));
        let mut input_multiasset = MultiAsset::new();
        input_multiasset.insert(policy_id, &{
            let mut assets = Assets::new();
            assets.insert(&name, &to_bignum(100));
            assets
        });
        input_amount.set_multiasset(&input_multiasset);

        let input = {
            let address = &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &input_amount)
            );
            builder.skip_witness().unwrap()
        };
        tx_builder.add_input(&input);

        // add an input that contains an asset & ADA
        let mut output_amount = Value::new(&to_bignum(2_000_000));
        let mut output_multiasset = MultiAsset::new();
        output_multiasset.insert(policy_id, &{
            let mut assets = Assets::new();
            assets.insert(&name, &to_bignum(100));
            assets
        });
        output_amount.set_multiasset(&output_multiasset);

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap();
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr.to_address())
                .next().unwrap()
                .with_value(&output_amount)
                .build().unwrap()
            ).unwrap();

        tx_builder.set_ttl(&1.into());

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap();
        let added_change = tx_builder.add_change_if_needed(
            &change_addr.to_address()
        );
        assert!(added_change.unwrap());
        assert_eq!(tx_builder.outputs.len(), 2);
        let final_tx = tx_builder.build().unwrap();
        let change_output = final_tx.outputs().get(1);
        let change_assets = change_output.amount().multiasset();

        // since all tokens got sent in the output
        // the change should be only ADA and not have any multiasset struct in it
        assert!(change_assets.is_none());
    }

    fn create_multiasset() -> (MultiAsset, [ScriptHash; 3], [AssetName; 3]) {
        let policy_ids = [
            PolicyID::from([0u8; 28]),
            PolicyID::from([1u8; 28]),
            PolicyID::from([2u8; 28]),
        ];
        let names = [
            AssetName::new(vec![99u8; 32]).unwrap(),
            AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
            AssetName::new(vec![4u8, 5, 6, 7, 8, 9]).unwrap(),
        ];
        let multiasset = policy_ids
            .iter()
            .zip(names.iter())
            .fold(MultiAsset::new(), |mut acc, (policy_id, name)| {
                acc.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(name, &to_bignum(500));
                    assets
                });
                acc
            });
        (multiasset, policy_ids, names)
    }

    #[test]
    fn build_tx_add_change_split_nfts() {
        let max_value_size = 100; // super low max output size to test with fewer assets
        let mut tx_builder = create_tx_builder_with_fee_and_val_size(
            &create_linear_fee(0, 1),
            max_value_size,
        );

        let (multiasset, policy_ids, names) = create_multiasset();

        let mut input_value = Value::new(&to_bignum(1000));
        input_value.set_multiasset(&multiasset);

        let input = {
            let address = &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &input_value)
            );
            builder.skip_witness().unwrap()
        };
        tx_builder.add_input(&input);

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap().to_address();
        let output_amount = Value::new(&to_bignum(100));

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr)
                .next().unwrap()
                .with_value(&output_amount)
                .build().unwrap()
            ).unwrap();

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();

        let added_change = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(added_change);
        let final_tx = tx_builder.build().unwrap();
        assert_eq!(final_tx.outputs().len(), 3);
        for (policy_id, asset_name) in policy_ids.iter().zip(names.iter()) {
            assert!(final_tx
                .outputs
                .0
                .iter()
                .any(|output| output.amount.multiasset.as_ref().map_or_else(
                    || false,
                    |ma| ma.0.iter().any(|(pid, a)| pid == policy_id
                        && a.0.iter().any(|(name, _)| name == asset_name))
                ))
            );
        }
        for output in final_tx.outputs.0.iter() {
            assert!(output.amount.to_bytes().len() <= max_value_size as usize);
        }
    }

    #[test]
    fn build_tx_too_big_output() {
        let mut tx_builder = create_tx_builder_with_fee_and_val_size(
            &create_linear_fee(0, 1),
            10,
        );

        let input = {
            let address = &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &Value::new(&to_bignum(500)))
            );
            builder.skip_witness().unwrap()
        };
        tx_builder.add_input(&input);

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap().to_address();
        let mut output_amount = Value::new(&to_bignum(50));
        output_amount.set_multiasset(&create_multiasset().0);

        assert!(tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr)
                .next().unwrap()
                .with_value(&output_amount)
                .build().unwrap()
            ).is_err());
    }

    #[test]
    fn build_tx_add_change_nfts_not_enough_ada() {
        let mut tx_builder = create_tx_builder_with_fee_and_val_size(
            &create_linear_fee(0, 1),
            150,  // super low max output size to test with fewer assets
        );

        let policy_ids = [
            PolicyID::from([0u8; 28]),
            PolicyID::from([1u8; 28]),
            PolicyID::from([2u8; 28]),
        ];
        let names = [
            AssetName::new(vec![99u8; 32]).unwrap(),
            AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
            AssetName::new(vec![4u8, 5, 6, 7, 8, 9]).unwrap(),
        ];

        let multiasset = policy_ids
            .iter()
            .zip(names.iter())
            .fold(MultiAsset::new(), |mut acc, (policy_id, name)| {
                acc.insert(policy_id, &{
                    let mut assets = Assets::new();
                    assets.insert(name, &to_bignum(500));
                    assets
                });
                acc
            });

        let mut input_value = Value::new(&to_bignum(58));
        input_value.set_multiasset(&multiasset);

        let input = {
            let address = &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &input_value)
            );
            builder.skip_witness().unwrap()
        };
        tx_builder.add_input(&input);

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap().to_address();
        let output_amount = Value::new(&to_bignum(59));

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr)
                .next().unwrap()
                .with_value(&output_amount)
                .build().unwrap()
            ).unwrap();

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();

        assert!(tx_builder.add_change_if_needed(&change_addr).is_err())
    }

    fn make_input(input_hash_byte: u8, value: Value) -> InputBuilderResult {
        let ((spend, _), _, address) = create_account();
        let builder = SingleInputBuilder::new(
            &TransactionInput::new(&TransactionHash::from([input_hash_byte; 32]), &0.into()),
            &TransactionOutputBuilder::new()
                .with_address(&address)
                .next().unwrap()
                .with_value(&value)
                .build().unwrap()
        );
        let vkey = Vkey::new(&spend.to_raw_key());
        builder.vkey(&vkey).unwrap()
    }

    #[test]
    fn tx_builder_cip2_largest_first_increasing_fees() {
        // we have a = 1 to test increasing fees when more inputs are added
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 0));
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap())
                .next().unwrap()
                .with_coin(&to_bignum(1000))
                .build().unwrap()
            ).unwrap();
        tx_builder.add_utxo(&make_input(0u8, Value::new(&to_bignum(150))));
        tx_builder.add_utxo(&make_input(1u8, Value::new(&to_bignum(200))));
        tx_builder.add_utxo(&make_input(2u8, Value::new(&to_bignum(800))));
        tx_builder.add_utxo(&make_input(3u8, Value::new(&to_bignum(400))));
        tx_builder.add_utxo(&make_input(4u8, Value::new(&to_bignum(100))));
        tx_builder.select_utxos(CoinSelectionStrategyCIP2::LargestFirst).unwrap();
        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();
        let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(change_added);
        let tx = tx_builder.build().unwrap();
        // change needed
        assert_eq!(2, tx.outputs().len());
        assert_eq!(3, tx.inputs().len());
        // confirm order of only what is necessary
        assert_eq!(2u8, tx.inputs().get(0).transaction_id().0[0]);
        assert_eq!(3u8, tx.inputs().get(1).transaction_id().0[0]);
        assert_eq!(1u8, tx.inputs().get(2).transaction_id().0[0]);
    }


    #[test]
    fn tx_builder_cip2_largest_first_static_fees() {
        // we have a = 0 so we know adding inputs/outputs doesn't change the fee so we can analyze more
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap())
                .next().unwrap()
                .with_coin(&to_bignum(1200))
                .build().unwrap()
            ).unwrap();
        tx_builder.add_utxo(&make_input(0u8, Value::new(&to_bignum(150))));
        tx_builder.add_utxo(&make_input(1u8, Value::new(&to_bignum(200))));
        tx_builder.add_utxo(&make_input(2u8, Value::new(&to_bignum(800))));
        tx_builder.add_utxo(&make_input(3u8, Value::new(&to_bignum(400))));
        tx_builder.add_utxo(&make_input(4u8, Value::new(&to_bignum(100))));
        tx_builder.select_utxos(CoinSelectionStrategyCIP2::LargestFirst).unwrap();
        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();
        let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(!change_added);
        let tx = tx_builder.build().unwrap();
        // change not needed - should be exact
        assert_eq!(1, tx.outputs().len());
        assert_eq!(2, tx.inputs().len());
        // confirm order of only what is necessary
        assert_eq!(2u8, tx.inputs().get(0).transaction_id().0[0]);
        assert_eq!(3u8, tx.inputs().get(1).transaction_id().0[0]);
    }

    #[test]
    fn tx_builder_cip2_largest_first_multiasset() {
        // we have a = 0 so we know adding inputs/outputs doesn't change the fee so we can analyze more
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
        let pid1 = PolicyID::from([1u8; 28]);
        let pid2 = PolicyID::from([2u8; 28]);
        let asset_name1 = AssetName::new(vec![1u8; 8]).unwrap();
        let asset_name2 = AssetName::new(vec![2u8; 11]).unwrap();
        let asset_name3 = AssetName::new(vec![3u8; 9]).unwrap();

        let mut output_value = Value::new(&to_bignum(415));
        let mut output_ma = MultiAsset::new();
        output_ma.set_asset(&pid1, &asset_name1, to_bignum(5));
        output_ma.set_asset(&pid1, &asset_name2, to_bignum(1));
        output_ma.set_asset(&pid2, &asset_name2, to_bignum(2));
        output_ma.set_asset(&pid2, &asset_name3, to_bignum(4));
        output_value.set_multiasset(&output_ma);
        tx_builder.add_output(&TransactionOutput::new(
            &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap(),
            &output_value
        )).unwrap();

        // should not be taken
        tx_builder.add_utxo(&make_input(0u8, Value::new(&to_bignum(150))));

        // should not be taken
        let mut input1 = make_input(1u8, Value::new(&to_bignum(200)));
        let mut ma1 = MultiAsset::new();
        ma1.set_asset(&pid1, &asset_name1, to_bignum(10));
        ma1.set_asset(&pid1, &asset_name2, to_bignum(1));
        ma1.set_asset(&pid2, &asset_name2, to_bignum(2));
        input1.utxo_info.amount.set_multiasset(&ma1);
        tx_builder.add_utxo(&input1);

        // taken first to satisfy pid1:asset_name1 (but also satisfies pid2:asset_name3)
        let mut input2 = make_input(2u8, Value::new(&to_bignum(10)));
        let mut ma2 = MultiAsset::new();
        ma2.set_asset(&pid1, &asset_name1, to_bignum(20));
        ma2.set_asset(&pid2, &asset_name3, to_bignum(4));
        input2.utxo_info.amount.set_multiasset(&ma2);
        tx_builder.add_utxo(&input2.clone());

        // taken second to satisfy pid1:asset_name2 (but also satisfies pid2:asset_name1)
        let mut input3 = make_input(3u8, Value::new(&to_bignum(50)));
        let mut ma3 = MultiAsset::new();
        ma3.set_asset(&pid2, &asset_name1, to_bignum(5));
        ma3.set_asset(&pid1, &asset_name2, to_bignum(15));
        input3.utxo_info.amount.multiasset = Some(ma3);
        tx_builder.add_utxo(&input3.clone());

        // should not be taken either
        let mut input4 = make_input(4u8, Value::new(&to_bignum(10)));
        let mut ma4 = MultiAsset::new();
        ma4.set_asset(&pid1, &asset_name1, to_bignum(10));
        ma4.set_asset(&pid1, &asset_name2, to_bignum(10));
        input4.utxo_info.amount.multiasset = Some(ma4);
        tx_builder.add_utxo(&input4.clone());

        // taken third to satisfy pid2:asset_name_2
        let mut input5 = make_input(5u8, Value::new(&to_bignum(10)));
        let mut ma5 = MultiAsset::new();
        ma5.set_asset(&pid1, &asset_name2, to_bignum(10));
        ma5.set_asset(&pid2, &asset_name2, to_bignum(3));
        input5.utxo_info.amount.multiasset = Some(ma5);
        tx_builder.add_utxo(&input5.clone());

        // should be taken to get enough ADA
        let input6 = make_input(6u8, Value::new(&to_bignum(400)));
        tx_builder.add_utxo(&input6.clone());

        // should not be taken
        tx_builder.add_utxo(&make_input(7u8, Value::new(&to_bignum(100))));
        tx_builder.select_utxos(CoinSelectionStrategyCIP2::LargestFirstMultiAsset).unwrap();
        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();
        let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(change_added);
        let tx = tx_builder.build().unwrap();

        assert_eq!(2, tx.outputs().len());
        assert_eq!(4, tx.inputs().len());
        // check order expected per-asset
        assert_eq!(2u8, tx.inputs().get(0).transaction_id().0[0]);
        assert_eq!(3u8, tx.inputs().get(1).transaction_id().0[0]);
        assert_eq!(5u8, tx.inputs().get(2).transaction_id().0[0]);
        assert_eq!(6u8, tx.inputs().get(3).transaction_id().0[0]);

        let change = tx.outputs().get(1).amount;
        assert_eq!(from_bignum(&change.coin), 55);
        let change_ma = change.multiasset().unwrap();
        assert_eq!(15, from_bignum(&change_ma.get_asset(&pid1, &asset_name1)));
        assert_eq!(24, from_bignum(&change_ma.get_asset(&pid1, &asset_name2)));
        assert_eq!(1, from_bignum(&change_ma.get_asset(&pid2, &asset_name2)));
        assert_eq!(0, from_bignum(&change_ma.get_asset(&pid2, &asset_name3)));
        let expected_input = input2.utxo_info.amount
            .checked_add(&input3.utxo_info.amount)
            .unwrap()
            .checked_add(&input5.utxo_info.amount)
            .unwrap()
            .checked_add(&input6.utxo_info.amount)
            .unwrap();
        let expected_change = expected_input.checked_sub(&output_value).unwrap();
        assert_eq!(expected_change, change);
    }

    #[test]
    fn tx_builder_cip2_random_improve_multiasset() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 0));
        let pid1 = PolicyID::from([1u8; 28]);
        let pid2 = PolicyID::from([2u8; 28]);
        let asset_name1 = AssetName::new(vec![1u8; 8]).unwrap();
        let asset_name2 = AssetName::new(vec![2u8; 11]).unwrap();
        let asset_name3 = AssetName::new(vec![3u8; 9]).unwrap();

        let mut output_value = Value::new(&to_bignum(415));
        let mut output_ma = MultiAsset::new();
        output_ma.set_asset(&pid1, &asset_name1, to_bignum(5));
        output_ma.set_asset(&pid1, &asset_name2, to_bignum(1));
        output_ma.set_asset(&pid2, &asset_name2, to_bignum(2));
        output_ma.set_asset(&pid2, &asset_name3, to_bignum(4));
        output_value.set_multiasset(&output_ma);
        tx_builder.add_output(&TransactionOutput::new(
            &Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap(),
            &output_value
        )).unwrap();

        tx_builder.add_utxo(&make_input(0u8, Value::new(&to_bignum(150))));

        let mut input1 = make_input(1u8, Value::new(&to_bignum(200)));
        let mut ma1 = MultiAsset::new();
        ma1.set_asset(&pid1, &asset_name1, to_bignum(10));
        ma1.set_asset(&pid1, &asset_name2, to_bignum(1));
        ma1.set_asset(&pid2, &asset_name2, to_bignum(2));
        input1.utxo_info.amount.set_multiasset(&ma1);
        tx_builder.add_utxo(&input1);

        let mut input2 = make_input(2u8, Value::new(&to_bignum(10)));
        let mut ma2 = MultiAsset::new();
        ma2.set_asset(&pid1, &asset_name1, to_bignum(20));
        ma2.set_asset(&pid2, &asset_name3, to_bignum(4));
        input2.utxo_info.amount.set_multiasset(&ma2);
        tx_builder.add_utxo(&input2);

        let mut input3 = make_input(3u8, Value::new(&to_bignum(50)));
        let mut ma3 = MultiAsset::new();
        ma3.set_asset(&pid2, &asset_name1, to_bignum(5));
        ma3.set_asset(&pid1, &asset_name2, to_bignum(15));
        input3.utxo_info.amount.multiasset = Some(ma3);
        tx_builder.add_utxo(&input3);

        let mut input4 = make_input(4u8, Value::new(&to_bignum(10)));
        let mut ma4 = MultiAsset::new();
        ma4.set_asset(&pid1, &asset_name1, to_bignum(10));
        ma4.set_asset(&pid1, &asset_name2, to_bignum(10));
        input4.utxo_info.amount.multiasset = Some(ma4);
        tx_builder.add_utxo(&input4);

        let mut input5 = make_input(5u8, Value::new(&to_bignum(10)));
        let mut ma5 = MultiAsset::new();
        ma5.set_asset(&pid1, &asset_name2, to_bignum(10));
        ma5.set_asset(&pid2, &asset_name2, to_bignum(3));
        input5.utxo_info.amount.multiasset = Some(ma5);
        tx_builder.add_utxo(&input5);

        let input6 = make_input(6u8, Value::new(&to_bignum(400)));
        tx_builder.add_utxo(&input6);
        tx_builder.add_utxo(&make_input(7u8, Value::new(&to_bignum(100))));

        let mut input8 = make_input(8u8, Value::new(&to_bignum(10)));
        let mut ma8 = MultiAsset::new();
        ma8.set_asset(&pid2, &asset_name2, to_bignum(10));
        input8.utxo_info.amount.multiasset = Some(ma8);
        tx_builder.add_utxo(&input8);

        let mut input9 = make_input(9u8, Value::new(&to_bignum(10)));
        let mut ma9 = MultiAsset::new();
        ma9.set_asset(&pid2, &asset_name3, to_bignum(10));
        input9.utxo_info.amount.multiasset = Some(ma9);
        tx_builder.add_utxo(&input9);

        tx_builder.select_utxos(CoinSelectionStrategyCIP2::RandomImproveMultiAsset).unwrap();
        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();
        let change_added = tx_builder.add_change_if_needed(&change_addr).unwrap();
        assert!(change_added);
        let tx = tx_builder.build().unwrap();

        assert_eq!(2, tx.outputs().len());

        let input_total = tx_builder.get_explicit_input().unwrap();
        assert!(input_total >= output_value);
    }

    #[test]
    fn tx_builder_cip2_random_improve() {
        // we have a = 1 to test increasing fees when more inputs are added
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(1, 0));
        const COST: u64 = 10000;
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap())
                .next().unwrap()
                .with_coin(&to_bignum(COST))
                .build().unwrap()
            ).unwrap();
        tx_builder.utxos.push(make_input(0u8, Value::new(&to_bignum(1500))));
        tx_builder.utxos.push(make_input(1u8, Value::new(&to_bignum(2000))));
        tx_builder.utxos.push(make_input(2u8, Value::new(&to_bignum(8000))));
        tx_builder.utxos.push(make_input(3u8, Value::new(&to_bignum(4000))));
        tx_builder.utxos.push(make_input(4u8, Value::new(&to_bignum(1000))));
        tx_builder.utxos.push(make_input(5u8, Value::new(&to_bignum(2000))));
        tx_builder.utxos.push(make_input(6u8, Value::new(&to_bignum(1500))));
        let add_inputs_res = tx_builder.select_utxos(CoinSelectionStrategyCIP2::RandomImprove);
        assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();
        let add_change_res = tx_builder.add_change_if_needed(&change_addr);
        assert!(add_change_res.is_ok(), "{:?}", add_change_res.err());
        let tx_build_res = tx_builder.build();
        assert!(tx_build_res.is_ok(), "{:?}", tx_build_res.err());
        let tx = tx_build_res.unwrap();
        // we need to look up the values to ensure there's enough
        let mut input_values = BTreeMap::new();
        for utxo in tx_builder.utxos.iter() {
            input_values.insert(utxo.input.transaction_id(), utxo.utxo_info.amount.clone());
        }
        let mut encountered = std::collections::HashSet::new();
        let mut input_total = Value::new(&Coin::zero());
        for input in tx.inputs.0.iter() {
            let txid = input.transaction_id();
            if !encountered.insert(txid.clone()) {
                panic!("Input {:?} duplicated", txid);
            }
            let value = input_values.get(&txid).unwrap();
            input_total = input_total.checked_add(value).unwrap();
        }
        assert!(input_total >= Value::new(&tx_builder.min_fee().unwrap().checked_add(&to_bignum(COST)).unwrap()));
    }

    #[test]
    fn tx_builder_cip2_random_improve_exclude_used_indices() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(44, 155381));
        const COST: u64 = 1000000;
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap())
                .next().unwrap()
                .with_coin(&to_bignum(COST))
                .build().unwrap()
            ).unwrap();
        tx_builder.add_utxo(&make_input(0u8, Value::new(&to_bignum(1000000))));
        tx_builder.add_utxo(&make_input(1u8, Value::new(&to_bignum(10000000))));
        let mut input_total = tx_builder.get_total_input().unwrap();
        let mut output_total = tx_builder
            .get_explicit_output().unwrap()
            .checked_add(&Value::new(&tx_builder.get_deposit().unwrap())).unwrap()
            .checked_add(&Value::new(&tx_builder.min_fee().unwrap())).unwrap();
        let available_inputs = tx_builder.utxos.clone();
        let mut available_indices: BTreeSet<usize> = (0..available_inputs.len()).collect();
        assert!(available_indices.len() == 2);
        use rand::SeedableRng;
        let mut rng = rand_chacha::ChaChaRng::seed_from_u64(1);
        tx_builder.cip2_random_improve_by(
            &available_inputs,
            &mut available_indices,
            &mut input_total,
            &mut output_total,
            |value| Some(value.coin),
            &mut rng).unwrap();
        assert!(!available_indices.contains(&0));
        assert!(available_indices.contains(&1));
        assert!(available_indices.len() < 2);
    }

    #[test]
    fn tx_builder_cip2_random_improve_when_using_all_available_inputs() {
        // we have a = 1 to test increasing fees when more inputs are added
        let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(0));
        let cfg = TransactionBuilderConfigBuilder::default()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(0))
            .key_deposit(&to_bignum(0))
            .max_value_size(9999)
            .max_tx_size(9999)
            .coins_per_utxo_word(&Coin::zero())
            .build()
            .unwrap();
        let mut tx_builder = TransactionBuilder::new(&cfg);
        const COST: u64 = 1000;
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap())
                .next().unwrap()
                .with_coin(&to_bignum(COST))
                .build().unwrap()
            ).unwrap();
        tx_builder.add_utxo(&make_input(1u8, Value::new(&to_bignum(800))));
        tx_builder.add_utxo(&make_input(2u8, Value::new(&to_bignum(800))));
        let add_inputs_res = tx_builder.select_utxos(CoinSelectionStrategyCIP2::RandomImprove);
        assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
    }

    #[test]
    fn tx_builder_cip2_random_improve_adds_enough_for_fees() {
        // we have a = 1 to test increasing fees when more inputs are added
        let linear_fee = LinearFee::new(&to_bignum(1), &to_bignum(0));
        let cfg = TransactionBuilderConfigBuilder::default()
            .fee_algo(&linear_fee)
            .pool_deposit(&to_bignum(0))
            .key_deposit(&to_bignum(0))
            .max_value_size(9999)
            .max_tx_size(9999)
            .coins_per_utxo_word(&Coin::zero())
            .build()
            .unwrap();
        let mut tx_builder = TransactionBuilder::new(&cfg);
        const COST: u64 = 100;
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&Address::from_bech32("addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z").unwrap())
                .next().unwrap()
                .with_coin(&to_bignum(COST))
                .build().unwrap()
            ).unwrap();
        assert_eq!(tx_builder.min_fee().unwrap(), to_bignum(53));
        tx_builder.add_utxo(&make_input(1u8, Value::new(&to_bignum(150))));
        tx_builder.add_utxo(&make_input(2u8, Value::new(&to_bignum(150))));
        tx_builder.add_utxo(&make_input(3u8, Value::new(&to_bignum(150))));
        let add_inputs_res = tx_builder.select_utxos(CoinSelectionStrategyCIP2::RandomImprove);
        assert!(add_inputs_res.is_ok(), "{:?}", add_inputs_res.err());
        assert_eq!(tx_builder.min_fee().unwrap(), to_bignum(264));
        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();
        let add_change_res = tx_builder.add_change_if_needed(&change_addr);
        assert!(add_change_res.is_ok(), "{:?}", add_change_res.err());
    }

    #[test]
    fn build_tx_pay_to_multisig() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(10, 2));
        let ((spend, _), _, addr_net_0) = create_account();

        let input = {
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&addr_net_0, &Value::new(&to_bignum(1_000_000)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_coin(&to_bignum(999_000))
                .build().unwrap()
            ).unwrap();
        tx_builder.set_ttl(&1000.into());
        tx_builder.set_fee(&to_bignum(1_000));

        assert_eq!(tx_builder.outputs.len(),1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );


        let  _final_tx = tx_builder.build().unwrap();
        let _deser_t = TransactionBody::from_bytes(_final_tx.to_bytes()).unwrap();

        assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
    }

    fn build_full_tx(body: &TransactionBody,
        witness_set: &TransactionWitnessSet,
        auxiliary_data: Option<AuxiliaryData>
    ) -> Transaction {
            Transaction::new(
                body,
                witness_set,
                auxiliary_data
            )
        }

    #[test]
    fn build_tx_multisig_spend_1on1_unsigned() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(10, 2));

        let ((spend, _), (_, stake_cred), addr_multisig) = create_account();
        let change_key = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(1)
            .derive(0)
            .to_public();
        let change_cred = StakeCredential::from_keyhash(&change_key.to_raw_key().hash());
        let addr_output = BaseAddress::new(NetworkInfo::testnet().network_id(), &change_cred, &stake_cred).to_address();

        let input = {
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&addr_multisig, &Value::new(&to_bignum(1_000_000)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_output)
                .next().unwrap()
                .with_coin(&to_bignum(999_000))
                .build().unwrap()
            ).unwrap();
        tx_builder.set_ttl(&1000.into());
        tx_builder.set_fee(&to_bignum(1_000));

        let mut auxiliary_data = AuxiliaryData::new();
        let mut pubkey_native_scripts = NativeScripts::new();
        let mut oneof_native_scripts = NativeScripts::new();

        let spending_hash = spend.to_raw_key().hash();
        pubkey_native_scripts.add(&NativeScript::new_script_pubkey(&ScriptPubkey::new(&spending_hash)));
        oneof_native_scripts.add(&NativeScript::new_script_n_of_k(&ScriptNOfK::new(1, &pubkey_native_scripts)));
        auxiliary_data.set_native_scripts(&oneof_native_scripts);
        tx_builder.set_auxiliary_data(&auxiliary_data);


        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );


        let  _final_tx = tx_builder.build().unwrap();
        let _deser_t = TransactionBody::from_bytes(_final_tx.to_bytes()).unwrap();

        assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
        assert_eq!(_deser_t.auxiliary_data_hash.unwrap(), utils::hash_auxiliary_data(&auxiliary_data));
    }

    #[test]
    fn build_tx_multisig_1on1_signed() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(10, 2));
        let spend = root_key_15()
            .derive(harden(1854))//multisig
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_15()
            .derive(harden(1854))//multisig
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();

        let spend_cred = StakeCredential::from_keyhash(&spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(&stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), &spend_cred, &stake_cred).to_address();
        let input = {
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&addr_net_0, &Value::new(&to_bignum(1_000_000)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&addr_net_0)
                .next().unwrap()
                .with_coin(&to_bignum(999_000))
                .build().unwrap()
            ).unwrap();
        tx_builder.set_ttl(&1000.into());
        tx_builder.set_fee(&to_bignum(1_000));

        let mut auxiliary_data = AuxiliaryData::new();
        let mut pubkey_native_scripts = NativeScripts::new();
        let mut oneof_native_scripts = NativeScripts::new();

        let spending_hash = spend.to_raw_key().hash();
        pubkey_native_scripts.add(&NativeScript::new_script_pubkey(&ScriptPubkey::new(&spending_hash)));
        oneof_native_scripts.add(&NativeScript::new_script_n_of_k(&ScriptNOfK::new(1, &pubkey_native_scripts)));
        auxiliary_data.set_native_scripts(&oneof_native_scripts);
        tx_builder.set_auxiliary_data(&auxiliary_data);


        let body = tx_builder.build().unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        assert_eq!(
            tx_builder.get_explicit_input().unwrap().checked_add(&tx_builder.get_implicit_input().unwrap()).unwrap(),
            tx_builder.get_explicit_output().unwrap().checked_add(&Value::new(&tx_builder.get_fee_if_set().unwrap())).unwrap()
        );

        let mut witness_set = TransactionWitnessSet::new();
        let mut vkw = Vkeywitnesses::new();
        vkw.add(&make_vkey_witness(
            &hash_transaction(&body),
            &PrivateKey::from_normal_bytes(
                &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a").unwrap()
            ).unwrap()
        ));
        witness_set.set_vkeys(&vkw);

        let _final_tx = build_full_tx(&body, &witness_set, None);
        let _deser_t = Transaction::from_bytes(_final_tx.to_bytes()).unwrap();
        assert_eq!(_deser_t.to_bytes(), _final_tx.to_bytes());
        assert_eq!(_deser_t.body().auxiliary_data_hash.unwrap(), utils::hash_auxiliary_data(&auxiliary_data));
    }

    #[test]
    fn add_change_splits_change_into_multiple_outputs_when_nfts_overflow_output_size() {
        let linear_fee = LinearFee::new(&to_bignum(0), &to_bignum(1));
        let max_value_size = 100; // super low max output size to test with fewer assets
        let mut tx_builder = TransactionBuilder::new(
            &TransactionBuilderConfigBuilder::default()
                .fee_algo(&linear_fee)
                .pool_deposit(&to_bignum(0))
                .key_deposit(&to_bignum(0))
                .max_value_size(max_value_size)
                .max_tx_size(MAX_TX_SIZE)
                .coins_per_utxo_word(&to_bignum(1))
                .prefer_pure_change(true)
                .build()
                .unwrap()
        );

        let policy_id = PolicyID::from([0u8; 28]);
        let names = [
            AssetName::new(vec![99u8; 32]).unwrap(),
            AssetName::new(vec![0u8, 1, 2, 3]).unwrap(),
            AssetName::new(vec![4u8, 5, 6, 7]).unwrap(),
            AssetName::new(vec![5u8, 5, 6, 7]).unwrap(),
            AssetName::new(vec![6u8, 5, 6, 7]).unwrap(),
        ];
        let assets = names
            .iter()
            .fold(Assets::new(), |mut a, name| {
                a.insert(name, &to_bignum(500));
                a
            });
        let mut multiasset = MultiAsset::new();
        multiasset.insert(&policy_id, &assets);

        let mut input_value = Value::new(&to_bignum(300));
        input_value.set_multiasset(&multiasset);

        let input = {
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(
                    &ByronAddress::from_base58("Ae2tdPwUPEZ5uzkzh1o2DHECiUi3iugvnnKHRisPgRRP3CTF4KCMvy54Xd3").unwrap().to_address(),
                    &input_value
                )
            );
            builder.skip_witness().unwrap()
        };
        tx_builder.add_input(&input);

        let output_addr = ByronAddress::from_base58("Ae2tdPwUPEZD9QQf2ZrcYV34pYJwxK4vqXaF8EXkup1eYH73zUScHReM42b").unwrap().to_address();
        let output_amount = Value::new(&to_bignum(50));

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&output_addr)
                .next().unwrap()
                .with_value(&output_amount)
                .build().unwrap()
            ).unwrap();

        let change_addr = ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap().to_address();

        let add_change_result = tx_builder.add_change_if_needed(&change_addr);
        assert!(add_change_result.is_ok());
        assert_eq!(tx_builder.outputs.len(), 4);

        let change1 = tx_builder.outputs.get(1);
        let change2 = tx_builder.outputs.get(2);
        let change3 = tx_builder.outputs.get(3);

        assert_eq!(change1.address, change_addr);
        assert_eq!(change1.address, change2.address);
        assert_eq!(change1.address, change3.address);

        assert_eq!(change1.amount.coin, to_bignum(45));
        assert_eq!(change2.amount.coin, to_bignum(42));
        assert_eq!(change3.amount.coin, to_bignum(162));

        assert!(change1.amount.multiasset.is_some());
        assert!(change2.amount.multiasset.is_some());
        assert!(change3.amount.multiasset.is_none()); // purified

        let masset1 = change1.amount.multiasset.unwrap();
        let masset2 = change2.amount.multiasset.unwrap();

        assert_eq!(masset1.keys().len(), 1);
        assert_eq!(masset1.keys(), masset2.keys());

        let asset1 = masset1.get(&policy_id).unwrap();
        let asset2 = masset2.get(&policy_id).unwrap();
        assert_eq!(asset1.len(), 4);
        assert_eq!(asset2.len(), 1);

        names.iter().for_each(|name| {
            let v1 = asset1.get(name);
            let v2 = asset2.get(name);
            assert_ne!(v1.is_some(), v2.is_some());
            assert_eq!(v1.or(v2).unwrap(), to_bignum(500));
        });
    }

    fn create_json_metadatum_string() -> String {
        String::from("{ \"qwe\": 123 }")
    }

    fn create_json_metadatum() -> TransactionMetadatum {
        encode_json_str_to_metadatum(
            create_json_metadatum_string(),
            MetadataJsonSchema::NoConversions,
        ).unwrap()
    }

    fn create_aux_with_metadata(metadatum_key: &TransactionMetadatumLabel) -> AuxiliaryData {
        let mut metadata = GeneralTransactionMetadata::new();
        metadata.insert(metadatum_key, &create_json_metadatum());

        let mut aux = AuxiliaryData::new();
        aux.set_metadata(&metadata);

        let mut nats = NativeScripts::new();
        nats.add(
            &NativeScript::new_timelock_start(
                &TimelockStart::new(&123.into()),
            ),
        );
        aux.set_native_scripts(&nats);

        aux
    }

    fn assert_json_metadatum(dat: &TransactionMetadatum) {
        let map = dat.as_map().unwrap();
        assert_eq!(map.len(), 1);
        let key = TransactionMetadatum::new_text(String::from("qwe")).unwrap();
        let val = map.get(&key).unwrap();
        assert_eq!(val.as_int().unwrap(), Int::new_i32(123));
    }

    #[test]
    fn set_metadata_with_empty_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num = to_bignum(42);
        tx_builder.set_metadata(&create_aux_with_metadata(&num).metadata().unwrap());

        assert!(tx_builder.auxiliary_data.is_some());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();

        assert_eq!(met.len(), 1);
        assert_json_metadatum(&met.get(&num).unwrap());
    }

    #[test]
    fn set_metadata_with_existing_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num1 = to_bignum(42);
        tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

        let num2 = to_bignum(84);
        tx_builder.set_metadata(&create_aux_with_metadata(&num2).metadata().unwrap());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_some());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.len(), 1);
        assert!(met.get(&num1).is_none());
        assert_json_metadatum(&met.get(&num2).unwrap());
    }

    #[test]
    fn add_metadatum_with_empty_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num = to_bignum(42);
        tx_builder.add_metadatum(&num, &create_json_metadatum());

        assert!(tx_builder.auxiliary_data.is_some());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();

        assert_eq!(met.len(), 1);
        assert_json_metadatum(&met.get(&num).unwrap());
    }

    #[test]
    fn add_metadatum_with_existing_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num1 = to_bignum(42);
        tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

        let num2 = to_bignum(84);
        tx_builder.add_metadatum(&num2, &create_json_metadatum());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_some());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.len(), 2);
        assert_json_metadatum(&met.get(&num1).unwrap());
        assert_json_metadatum(&met.get(&num2).unwrap());
    }

    #[test]
    fn add_json_metadatum_with_empty_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num = to_bignum(42);
        tx_builder.add_json_metadatum(&num, create_json_metadatum_string()).unwrap();

        assert!(tx_builder.auxiliary_data.is_some());

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_none());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();

        assert_eq!(met.len(), 1);
        assert_json_metadatum(&met.get(&num).unwrap());
    }

    #[test]
    fn add_json_metadatum_with_existing_auxiliary() {
        let mut tx_builder = create_default_tx_builder();

        let num1 = to_bignum(42);
        tx_builder.set_auxiliary_data(&create_aux_with_metadata(&num1));

        let num2 = to_bignum(84);
        tx_builder.add_json_metadatum(&num2, create_json_metadatum_string()).unwrap();

        let aux = tx_builder.auxiliary_data.unwrap();
        assert!(aux.metadata().is_some());
        assert!(aux.native_scripts().is_some());
        assert!(aux.plutus_scripts().is_none());

        let met = aux.metadata().unwrap();
        assert_eq!(met.len(), 2);
        assert_json_metadatum(&met.get(&num1).unwrap());
        assert_json_metadatum(&met.get(&num2).unwrap());
    }

    fn create_asset_name() -> AssetName {
        AssetName::new(vec![0u8, 1, 2, 3]).unwrap()
    }

    fn create_mint_asset() -> MintAssets {
        MintAssets::new_from_entry(&create_asset_name(), Int::new_i32(1234))
    }

    fn create_assets() -> Assets {
        let mut assets = Assets::new();
        assets.insert(&create_asset_name(), &to_bignum(1234));
        assets
    }

    fn create_multiasset_one_asset(policy_id: &PolicyID) -> MultiAsset {
        let mut mint = MultiAsset::new();
        mint.insert(policy_id, &create_assets());
        mint
    }

    fn assert_mint_asset(mint: &Mint, policy_id: &PolicyID) {
        assert!(mint.get(policy_id).is_some());
        let result_asset = mint.get(policy_id).unwrap();
        assert_eq!(result_asset.len(), 1);
        assert_eq!(result_asset.get(&create_asset_name()).unwrap(), Int::new_i32(1234));
    }

    fn mint_script_and_policy_and_hash(x: u8) -> (NativeScript, PolicyID, Ed25519KeyHash) {
        let hash = fake_key_hash(x);
        let mint_script = NativeScript::new_script_pubkey(
            &ScriptPubkey::new(&hash)
        );
        let policy_id = mint_script.hash(ScriptHashNamespace::NativeScript);
        (mint_script, policy_id, hash)
    }

    fn mint_script_and_policy(x: u8) -> (NativeScript, PolicyID) {
        let (m, p, _) = mint_script_and_policy_and_hash(x);
        (m, p)
    }

    #[test]
    fn set_mint_asset_with_empty_mint() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script, policy_id) = mint_script_and_policy(0);

        let result = SingleMintBuilder::new(&create_mint_asset())
            .native_script(&mint_script, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

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

        let result = SingleMintBuilder::new(&create_mint_asset())
            .native_script(&mint_script1, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        let result = SingleMintBuilder::new(&create_mint_asset())
            .native_script(&mint_script2, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

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

        let result = SingleMintBuilder::new(&create_mint_asset())
            .native_script(&mint_script, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

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

        let result = SingleMintBuilder::new(&create_mint_asset())
            .native_script(&mint_script1, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        let result = SingleMintBuilder::new(&create_mint_asset())
            .native_script(&mint_script2, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        assert!(tx_builder.mint.is_some());

        let mint = tx_builder.mint.unwrap();

        assert_eq!(mint.len(), 2);
        assert_mint_asset(&mint, &policy_id1);
        assert_mint_asset(&mint, &policy_id2);
    }

    #[test]
    fn add_output_amount() {
        let mut tx_builder = create_default_tx_builder();

        let policy_id1 = PolicyID::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);
        let mut value = Value::new(&to_bignum(42));
        value.set_multiasset(&multiasset);

        let address = byron_address();
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&address)
                .next().unwrap()
                .with_value(&value)
                .build().unwrap()
            ).unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount, value);
    }

    #[test]
    fn add_output_coin() {
        let mut tx_builder = create_default_tx_builder();

        let address = byron_address();
        let coin = to_bignum(43);
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&address)
                .next().unwrap()
                .with_coin(&coin)
                .build().unwrap()
            ).unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.coin, coin);
        assert!(out.amount.multiasset.is_none());
    }

    #[test]
    fn add_output_coin_and_multiasset() {
        let mut tx_builder = create_default_tx_builder();

        let policy_id1 = PolicyID::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);

        let address = byron_address();
        let coin = to_bignum(42);

        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&address)
                .next().unwrap()
                .with_coin_and_asset(&coin, &multiasset)
                .build().unwrap()
            ).unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.coin, coin);
        assert_eq!(out.amount.multiasset.unwrap(), multiasset);
    }

    #[test]
    fn add_output_asset_and_min_required_coin() {
        let mut tx_builder = create_reallistic_tx_builder();

        let policy_id1 = PolicyID::from([0u8; 28]);
        let multiasset = create_multiasset_one_asset(&policy_id1);

        let address = byron_address();
        tx_builder.add_output(
            &TransactionOutputBuilder::new()
                .with_address(&address)
                .next().unwrap()
                .with_asset_and_min_required_coin(&multiasset, &tx_builder.config.coins_per_utxo_word).unwrap()
                .build().unwrap()
            ).unwrap();

        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.multiasset.unwrap(), multiasset);
        assert_eq!(out.amount.coin, to_bignum(1344798));
    }

    #[test]
    fn add_mint_asset_and_output() {
        let mut tx_builder = create_default_tx_builder();

        let (mint_script0, policy_id0) = mint_script_and_policy(0);
        let (mint_script1, policy_id1) = mint_script_and_policy(1);

        let name = create_asset_name();
        let amount = Int::new_i32(1234);

        let address = byron_address();
        let coin = to_bignum(100);

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name, amount.clone()))
            .native_script(&mint_script0, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        let multiasset = {
            let mut assets = Assets::new();
            assets.insert(&name, &to_bignum(1234));
            let mut multiasset = MultiAsset::new();
            multiasset.insert(&policy_id1, &assets);
            multiasset
        };

        let output = TransactionOutputBuilder::new()
            .with_address(&address)
            .next()
            .unwrap()
            .with_coin_and_asset(&coin, &multiasset)
            .build()
            .unwrap();

        tx_builder.add_output(&output).unwrap();

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name, amount.clone()))
            .native_script(&mint_script1, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        assert!(tx_builder.mint.is_some());

        let mint = tx_builder.mint.as_ref().unwrap();

        // Mint contains two entries
        assert_eq!(mint.len(), 2);
        assert_mint_asset(mint, &policy_id0);
        assert_mint_asset(mint, &policy_id1);

        // One new output is created
        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.coin, coin);

        let multiasset = out.amount.multiasset.unwrap();

        // Only second mint entry was added to the output
        assert_eq!(multiasset.len(), 1);
        assert!(multiasset.get(&policy_id0).is_none());
        assert!(multiasset.get(&policy_id1).is_some());

        let asset = multiasset.get(&policy_id1).unwrap();
        assert_eq!(asset.len(), 1);
        assert_eq!(asset.get(&name).unwrap(), to_bignum(1234));
    }

    #[test]
    fn add_mint_asset_and_min_required_coin() {
        let mut tx_builder = create_reallistic_tx_builder();

        let (mint_script0, policy_id0) = mint_script_and_policy(0);
        let (mint_script1, policy_id1) = mint_script_and_policy(1);

        let name = create_asset_name();
        let amount = Int::new_i32(1234);

        let address = byron_address();

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name, amount.clone()))
            .native_script(&mint_script0, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        let multiasset = {
            let mut assets = Assets::new();
            assets.insert(&name, &to_bignum(1234));
            let mut multiasset = MultiAsset::new();
            multiasset.insert(&policy_id1, &assets);
            multiasset
        };

        let output = TransactionOutputBuilder::new()
            .with_address(&address)
            .next()
            .unwrap()
            .with_asset_and_min_required_coin(&multiasset, &tx_builder.config.coins_per_utxo_word)
            .unwrap()
            .build()
            .unwrap();

        tx_builder.add_output(&output).unwrap();

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name, amount.clone()))
            .native_script(&mint_script1, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        assert!(tx_builder.mint.is_some());

        let mint = tx_builder.mint.as_ref().unwrap();

        // Mint contains two entries
        assert_eq!(mint.len(), 2);
        assert_mint_asset(mint, &policy_id0);
        assert_mint_asset(mint, &policy_id1);

        // One new output is created
        assert_eq!(tx_builder.outputs.len(), 1);
        let out = tx_builder.outputs.get(0);

        assert_eq!(out.address.to_bytes(), address.to_bytes());
        assert_eq!(out.amount.coin, to_bignum(1344798));

        let multiasset = out.amount.multiasset.unwrap();

        // Only second mint entry was added to the output
        assert_eq!(multiasset.len(), 1);
        assert!(multiasset.get(&policy_id0).is_none());
        assert!(multiasset.get(&policy_id1).is_some());

        let asset = multiasset.get(&policy_id1).unwrap();
        assert_eq!(asset.len(), 1);
        assert_eq!(asset.get(&name).unwrap(), to_bignum(1234));
    }

    #[test]
    fn add_mint_includes_witnesses_into_fee_estimation() {

        let mut tx_builder = create_reallistic_tx_builder();

        let (mint_script1, policy_id1) = mint_script_and_policy(1);
        let (_mint_script2, policy_id2) = mint_script_and_policy(2);
        let (_mint_script3, policy_id3) = mint_script_and_policy(3);

        let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let name2 = AssetName::new(vec![1u8, 1, 2, 3]).unwrap();
        let name3 = AssetName::new(vec![2u8, 1, 2, 3]).unwrap();
        let name4 = AssetName::new(vec![3u8, 1, 2, 3]).unwrap();
        let amount = Int::new_i32(1234);

        // One input from an unrelated address
        let input = {
            let ((spend, _), _, address) = create_account();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &Value::new(&to_bignum(10_000_000)))
            );
            let vkey = Vkey::new(&spend.to_raw_key());
            builder.vkey(&vkey).unwrap()
        };
        tx_builder.add_input(&input);

        // One input from a related address
        let input = {
            let cred = StakeCredential::from_scripthash(&policy_id1);
            let address = BaseAddress::new(NetworkInfo::testnet().network_id(), &cred, &cred).to_address();
            let builder = SingleInputBuilder::new(
                &TransactionInput::new(&genesis_id(), &0.into()),
                &TransactionOutput::new(&address, &Value::new(&to_bignum(10_000_000)))
            );
            builder.native_script(&mint_script1, &NativeScriptWitnessInfo::assume_signature_count()).unwrap()
        };
        tx_builder.add_input(&input);

        // Original tx fee now assumes:
        // 1. two VKey signatures for two inputs
        // 2. a native script witness for one input
        let original_tx_fee = tx_builder.min_fee().unwrap();
        // The original test did not include native script
        // assert_eq!(original_tx_fee, to_bignum(168361));
        assert_eq!(original_tx_fee, to_bignum(169857));

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name1, amount.clone()))
            .skip_witness(&policy_id1);

        tx_builder.add_mint(&result);

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name2, amount.clone()))
            .skip_witness(&policy_id2);

        tx_builder.add_mint(&result);

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name3, amount.clone()))
            .skip_witness(&policy_id3);

        tx_builder.add_mint(&result);

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name4, amount.clone()))
            .skip_witness(&policy_id3);

        tx_builder.add_mint(&result);

        let mint = tx_builder.get_mint().unwrap();
        let mint_len = mint.to_bytes().len();

        assert_eq!(mint.len(), 3);

        let mint_scripts = tx_builder.witness_set_builder.build();
        let mint_scripts_len = mint_scripts.to_bytes().len()
            - TransactionWitnessSet::new().to_bytes().len();

        assert_eq!(mint_scripts.native_scripts().unwrap().len(), 1);
        assert_eq!(mint_scripts.vkeys().unwrap().len(), 2);
        assert!(mint_scripts.bootstraps().is_none());
        assert!(mint_scripts.plutus_data().is_none());
        assert!(mint_scripts.plutus_scripts().is_none());
        assert!(mint_scripts.redeemers().is_none());

        let fee_coefficient = tx_builder.config.fee_algo.coefficient();

        let raw_mint_fee = fee_coefficient
            .checked_mul(&to_bignum(mint_len as u64))
            .unwrap();

        let raw_mint_script_fee = fee_coefficient
            .checked_mul(&to_bignum(mint_scripts_len as u64))
            .unwrap();

        assert_eq!(raw_mint_fee, to_bignum(5544));
        assert_eq!(raw_mint_script_fee, to_bignum(10472));
    }

    #[test]
    fn fee_estimation_fails_on_missing_mint_scripts() {
        let mut tx_builder = create_reallistic_tx_builder();

        // No error estimating fee without mint
        assert!(tx_builder.min_fee().is_ok());

        let (mint_script1, policy_id1) = mint_script_and_policy(0);
        let (mint_script2, _) = mint_script_and_policy(1);

        let name1 = AssetName::new(vec![0u8, 1, 2, 3]).unwrap();
        let amount = Int::new_i32(1234);

        let mut mint = Mint::new();
        mint.insert(
            &policy_id1,
            &MintAssets::new_from_entry(&name1, amount.clone()),
        );

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name1, amount.clone()))
            .native_script(&mint_script1, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        let est1 = tx_builder.min_fee();
        assert!(est1.is_ok());

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name1, amount.clone()))
            .native_script(&mint_script2, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        let est2 = tx_builder.min_fee();
        assert!(est2.is_ok());

        // Native script assertion has been commented out in `.min_fee`
        // Until implemented in a more performant manner
        // TODO: these test parts might be returned back when it's done

        // // Remove one mint script
        // tx_builder.native_scripts =
        //     Some(NativeScripts::from(vec![tx_builder.native_scripts.unwrap().get(1)]));
        //
        // // Now two different policies are minted but only one witness script is present
        // let est3 = tx_builder.min_fee();
        // assert!(est3.is_err());
        // assert!(est3.err().unwrap().to_string().contains(&format!("{:?}", hex::encode(policy_id1.to_bytes()))));
        //
        // // Remove all mint scripts
        // tx_builder.native_scripts = Some(NativeScripts::new());
        //
        // // Mint exists but no witness scripts at all present
        // let est4 = tx_builder.min_fee();
        // assert!(est4.is_err());
        // assert!(est4.err().unwrap().to_string().contains("witness scripts are not provided"));
        //
        // // Remove all mint scripts
        // tx_builder.native_scripts = None;
        //
        // // Mint exists but no witness scripts at all present
        // let est5 = tx_builder.min_fee();
        // assert!(est5.is_err());
        // assert!(est5.err().unwrap().to_string().contains("witness scripts are not provided"));
    }

    #[test]
    fn total_input_with_mint_and_burn() {
        let mut tx_builder = create_tx_builder_with_fee(&create_linear_fee(0, 1));
        let ((spend, _), (_stake, _), addr_test_0) = create_account();

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
                multiasset.insert(&policy_id1, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(*input));
                    assets
                });
                multiasset.insert(&policy_id2, &{
                    let mut assets = Assets::new();
                    assets.insert(&name, &to_bignum(*input));
                    assets
                });
                multiasset
            })
            .collect::<Vec<MultiAsset>>();

        for (multiasset, ada) in multiassets
            .iter()
            .zip([100u64, 100, 100].iter().cloned().map(to_bignum))
        {
            let mut input_amount = Value::new(&ada);
            input_amount.set_multiasset(multiasset);

            let input = {
                let builder = SingleInputBuilder::new(
                    &TransactionInput::new(&genesis_id(), &0.into()),
                    &TransactionOutput::new(&addr_test_0, &input_amount)
                );
                builder.vkey(&Vkey::new(&spend.to_raw_key())).unwrap()
            };
            tx_builder.add_input(&input);
        }

        let total_input_before_mint = tx_builder.get_total_input().unwrap();

        assert_eq!(total_input_before_mint.coin, to_bignum(300));
        let ma1 = total_input_before_mint.multiasset.unwrap();
        assert_eq!(ma1.get(&policy_id1).unwrap().get(&name).unwrap(), to_bignum(360));
        assert_eq!(ma1.get(&policy_id2).unwrap().get(&name).unwrap(), to_bignum(360));

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name, Int::new_i32(40)))
            .native_script(&mint_script1, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        let result = SingleMintBuilder::new(&MintAssets::new_from_entry(&name, Int::new_i32(-40)))
            .native_script(&mint_script2, &NativeScriptWitnessInfo::assume_signature_count())
            .unwrap();

        tx_builder.add_mint(&result);

        let total_input_after_mint = tx_builder.get_total_input().unwrap();

        assert_eq!(total_input_after_mint.coin, to_bignum(300));
        let ma2 = total_input_after_mint.multiasset.unwrap();
        assert_eq!(ma2.get(&policy_id1).unwrap().get(&name).unwrap(), to_bignum(400));
        assert_eq!(ma2.get(&policy_id2).unwrap().get(&name).unwrap(), to_bignum(320));
    }

}

