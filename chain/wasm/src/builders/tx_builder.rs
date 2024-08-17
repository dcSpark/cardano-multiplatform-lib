use cml_chain::builders::tx_builder::{ChangeSelectionAlgo, CoinSelectionStrategyCIP2};
use cml_core_wasm::{impl_wasm_cbor_event_serialize_api, impl_wasm_conversions};
use cml_crypto_wasm::Ed25519KeyHash;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use crate::{
    address::Address,
    assets::Mint,
    auxdata::AuxiliaryData,
    builders::{
        certificate_builder::CertificateBuilderResult, input_builder::InputBuilderResult,
        mint_builder::MintBuilderResult, output_builder::SingleOutputBuilderResult,
        proposal_builder::ProposalBuilderResult, redeemer_builder::RedeemerWitnessKey,
        vote_builder::VoteBuilderResult, withdrawal_builder::WithdrawalBuilderResult,
        witness_builder::TransactionWitnessSetBuilder,
    },
    crypto::{BootstrapWitness, Vkeywitness},
    fees::LinearFee,
    plutus::{CostModels, ExUnitPrices, ExUnits, Redeemers},
    transaction::{Transaction, TransactionBody, TransactionInput, TransactionOutput},
    Coin, NetworkId, Slot, Value, Withdrawals,
};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionUnspentOutput(cml_chain::builders::tx_builder::TransactionUnspentOutput);

impl_wasm_conversions!(
    cml_chain::builders::tx_builder::TransactionUnspentOutput,
    TransactionUnspentOutput
);

impl_wasm_cbor_event_serialize_api!(TransactionUnspentOutput);

#[wasm_bindgen]
impl TransactionUnspentOutput {
    pub fn new(input: &TransactionInput, output: &TransactionOutput) -> Self {
        cml_chain::builders::tx_builder::TransactionUnspentOutput::new(
            input.clone().into(),
            output.clone().into(),
        )
        .into()
    }

    pub fn input(&self) -> TransactionInput {
        self.0.input.clone().into()
    }

    pub fn output(&self) -> TransactionOutput {
        self.0.output.clone().into()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilderConfig(cml_chain::builders::tx_builder::TransactionBuilderConfig);

impl_wasm_conversions!(
    cml_chain::builders::tx_builder::TransactionBuilderConfig,
    TransactionBuilderConfig
);

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct TransactionBuilderConfigBuilder(
    cml_chain::builders::tx_builder::TransactionBuilderConfigBuilder,
);

impl_wasm_conversions!(
    cml_chain::builders::tx_builder::TransactionBuilderConfigBuilder,
    TransactionBuilderConfigBuilder
);

#[wasm_bindgen]
impl TransactionBuilderConfigBuilder {
    pub fn new() -> Self {
        // we have to provide new to expose it to WASM builds
        Self::default()
    }

    pub fn fee_algo(&self, fee_algo: &LinearFee) -> Self {
        self.0.clone().fee_algo(fee_algo.clone().into()).into()
    }

    pub fn coins_per_utxo_byte(&self, coins_per_utxo_byte: Coin) -> Self {
        self.0
            .clone()
            .coins_per_utxo_byte(coins_per_utxo_byte)
            .into()
    }

    pub fn pool_deposit(&self, pool_deposit: u64) -> Self {
        self.0.clone().pool_deposit(pool_deposit).into()
    }

    pub fn key_deposit(&self, key_deposit: u64) -> Self {
        self.0.clone().key_deposit(key_deposit).into()
    }

    pub fn max_value_size(&self, max_value_size: u32) -> Self {
        self.0.clone().max_value_size(max_value_size).into()
    }

    pub fn max_tx_size(&self, max_tx_size: u32) -> Self {
        self.0.clone().max_tx_size(max_tx_size).into()
    }

    pub fn prefer_pure_change(&self, prefer_pure_change: bool) -> Self {
        self.0.clone().prefer_pure_change(prefer_pure_change).into()
    }

    pub fn ex_unit_prices(&self, ex_unit_prices: &ExUnitPrices) -> Self {
        self.0
            .clone()
            .ex_unit_prices(ex_unit_prices.clone().into())
            .into()
    }

    pub fn cost_models(&self, cost_models: &CostModels) -> Self {
        self.0
            .clone()
            .cost_models(cost_models.clone().into())
            .into()
    }

    pub fn collateral_percentage(&self, collateral_percentage: u32) -> Self {
        self.0
            .clone()
            .collateral_percentage(collateral_percentage)
            .into()
    }

    pub fn max_collateral_inputs(&self, max_collateral_inputs: u32) -> Self {
        self.0
            .clone()
            .max_collateral_inputs(max_collateral_inputs)
            .into()
    }

    pub fn build(&self) -> Result<TransactionBuilderConfig, JsError> {
        self.0.clone().build().map(Into::into).map_err(Into::into)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct TransactionBuilder(cml_chain::builders::tx_builder::TransactionBuilder);

impl_wasm_conversions!(
    cml_chain::builders::tx_builder::TransactionBuilder,
    TransactionBuilder
);

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
        self.0.select_utxos(strategy).map_err(Into::into)
    }

    pub fn add_input(&mut self, result: &InputBuilderResult) -> Result<(), JsError> {
        self.0.add_input(result.clone().into()).map_err(Into::into)
    }

    pub fn add_utxo(&mut self, result: &InputBuilderResult) {
        self.0.add_utxo(result.clone().into())
    }

    /// calculates how much the fee would increase if you added a given output
    pub fn fee_for_input(&self, result: &InputBuilderResult) -> Result<Coin, JsError> {
        self.0.fee_for_input(result.as_ref()).map_err(Into::into)
    }

    /// Add a reference input. Must be called BEFORE adding anything (inputs, certs, etc) that refer to this reference input.
    pub fn add_reference_input(&mut self, utxo: &TransactionUnspentOutput) {
        self.0.add_reference_input(utxo.clone().into())
    }

    /// Add explicit output via a TransactionOutput object
    pub fn add_output(
        &mut self,
        builder_result: &SingleOutputBuilderResult,
    ) -> Result<(), JsError> {
        self.0
            .add_output(builder_result.clone().into())
            .map_err(Into::into)
    }

    /// calculates how much the fee would increase if you added a given output
    pub fn fee_for_output(&self, builder: &SingleOutputBuilderResult) -> Result<Coin, JsError> {
        self.0.fee_for_output(builder.as_ref()).map_err(Into::into)
    }

    pub fn set_fee(&mut self, fee: Coin) {
        self.0.set_fee(fee)
    }

    pub fn set_ttl(&mut self, ttl: Slot) {
        self.0.set_ttl(ttl)
    }

    pub fn set_validity_start_interval(&mut self, validity_start_interval: Slot) {
        self.0.set_validity_start_interval(validity_start_interval)
    }

    pub fn add_cert(&mut self, result: &CertificateBuilderResult) {
        self.0.add_cert(result.clone().into())
    }

    pub fn add_proposal(&mut self, result: ProposalBuilderResult) {
        self.0.add_proposal(result.clone().into())
    }

    pub fn add_vote(&mut self, result: VoteBuilderResult) {
        self.0.add_vote(result.clone().into())
    }

    pub fn get_withdrawals(&self) -> Option<Withdrawals> {
        self.0.get_withdrawals().map(|wd| wd.into())
    }

    pub fn add_withdrawal(&mut self, result: &WithdrawalBuilderResult) {
        self.0.add_withdrawal(result.clone().into())
    }

    pub fn get_auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.0.get_auxiliary_data().map(|aux| aux.into())
    }

    pub fn set_auxiliary_data(&mut self, new_aux_data: &AuxiliaryData) {
        self.0.set_auxiliary_data(new_aux_data.clone().into())
    }

    pub fn add_auxiliary_data(&mut self, new_aux_data: &AuxiliaryData) {
        self.0.add_auxiliary_data(new_aux_data.clone().into())
    }

    pub fn add_mint(&mut self, result: &MintBuilderResult) -> Result<(), JsError> {
        self.0.add_mint(result.clone().into()).map_err(Into::into)
    }

    /// Returns a copy of the current mint state in the builder
    pub fn get_mint(&self) -> Option<Mint> {
        self.0.get_mint().map(|m| m.into())
    }

    pub fn new(cfg: &TransactionBuilderConfig) -> Self {
        cml_chain::builders::tx_builder::TransactionBuilder::new(cfg.clone().into()).into()
    }

    pub fn add_collateral(&mut self, result: &InputBuilderResult) -> Result<(), JsError> {
        self.0
            .add_collateral(result.clone().into())
            .map_err(Into::into)
    }

    pub fn add_required_signer(&mut self, hash: &Ed25519KeyHash) {
        self.0.add_required_signer(hash.clone().into())
    }

    pub fn set_network_id(&mut self, network_id: &NetworkId) {
        self.0.set_network_id(network_id.clone().into())
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        self.0.network_id().map(Into::into)
    }

    /// does not include refunds or withdrawals
    pub fn get_explicit_input(&self) -> Result<Value, JsError> {
        self.0
            .get_explicit_input()
            .map(Into::into)
            .map_err(Into::into)
    }

    /// withdrawals and refunds
    pub fn get_implicit_input(&self) -> Result<Value, JsError> {
        self.0
            .get_implicit_input()
            .map(Into::into)
            .map_err(Into::into)
    }

    /// Return explicit input plus implicit input plus mint
    pub fn get_total_input(&self) -> Result<Value, JsError> {
        self.0.get_total_input().map(Into::into).map_err(Into::into)
    }

    /// Return explicit output plus implicit output plus burn (does not consider fee directly)
    pub fn get_total_output(&self) -> Result<Value, JsError> {
        self.0
            .get_total_output()
            .map(Into::into)
            .map_err(Into::into)
    }

    /// does not include fee
    pub fn get_explicit_output(&self) -> Result<Value, JsError> {
        self.0
            .get_explicit_output()
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn get_deposit(&self) -> Result<Coin, JsError> {
        self.0.get_deposit().map_err(Into::into)
    }

    pub fn get_fee_if_set(&self) -> Option<Coin> {
        self.0.get_fee_if_set()
    }

    pub fn set_collateral_return(&mut self, output: &TransactionOutput) {
        self.0.set_collateral_return(output.clone().into())
    }

    pub fn full_size(&self) -> Result<usize, JsError> {
        self.0.full_size().map_err(Into::into)
    }

    pub fn output_sizes(&self) -> Vec<usize> {
        self.0.output_sizes()
    }

    // TODO: switch from ChangeSelectionAlgo to ChangeSelectionBuilder
    /// Builds the transaction and moves to the next step redeemer units can be added and a draft tx can
    /// be evaluated
    /// NOTE: is_valid set to true
    pub fn build_for_evaluation(
        &self,
        algo: ChangeSelectionAlgo,
        change_address: &Address,
    ) -> Result<TxRedeemerBuilder, JsError> {
        self.0
            .build_for_evaluation(algo, change_address.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }

    // TODO: switch from ChangeSelectionAlgo to ChangeSelectionBuilder
    /// Builds the transaction and moves to the next step where any real witness can be added
    /// NOTE: is_valid set to true
    pub fn build(
        &mut self,
        algo: ChangeSelectionAlgo,
        change_address: &Address,
    ) -> Result<SignedTxBuilder, JsError> {
        self.0
            .build(algo, change_address.as_ref())
            .map(Into::into)
            .map_err(Into::into)
    }

    /// used to override the exunit values initially provided when adding inputs
    pub fn set_exunits(&mut self, redeemer: &RedeemerWitnessKey, ex_units: &ExUnits) {
        self.0
            .set_exunits((*redeemer).into(), ex_units.clone().into())
    }

    /// warning: sum of all parts of a transaction must equal 0. You cannot just set the fee to the min value and forget about it
    /// warning: min_fee may be slightly larger than the actual minimum fee (ex: a few lovelaces)
    /// this is done to simplify the library code, but can be fixed later
    pub fn min_fee(&self, script_calulation: bool) -> Result<Coin, JsError> {
        self.0.min_fee(script_calulation).map_err(Into::into)
    }

    /// Warning: this function will mutate the /fee/ field
    /// Make sure to call this function last after setting all other tx-body properties
    /// Editing inputs, outputs, mint, etc. after change been calculated
    /// might cause a mismatch in calculated fee versus the required fee
    pub fn add_change_if_needed(
        &mut self,
        address: &Address,
        include_exunits: bool,
    ) -> Result<bool, JsError> {
        cml_chain::builders::tx_builder::add_change_if_needed(
            &mut self.0,
            address.as_ref(),
            include_exunits,
        )
        .map_err(Into::into)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TxRedeemerBuilder(cml_chain::builders::tx_builder::TxRedeemerBuilder);

impl_wasm_conversions!(
    cml_chain::builders::tx_builder::TxRedeemerBuilder,
    TxRedeemerBuilder
);

#[wasm_bindgen]
impl TxRedeemerBuilder {
    /// Builds the transaction and moves to the next step where any real witness can be added
    /// NOTE: is_valid set to true
    /// Will NOT require you to have set required signers & witnesses
    pub fn build(&self) -> Result<Redeemers, JsError> {
        self.0.build().map(Into::into).map_err(Into::into)
    }

    /// used to override the exunit values initially provided when adding inputs
    pub fn set_exunits(&mut self, redeemer: &RedeemerWitnessKey, ex_units: &ExUnits) {
        self.0
            .set_exunits((*redeemer).into(), ex_units.clone().into())
    }

    /// Transaction body with a dummy values for redeemers & script_data_hash
    /// Used for calculating exunits or required signers
    pub fn draft_body(&self) -> TransactionBody {
        self.0.draft_body().into()
    }

    pub fn auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.0.auxiliary_data().map(Into::into)
    }

    /// Transaction body with a dummy values for redeemers & script_data_hash and padded with dummy witnesses
    /// Used for calculating exunits
    /// note: is_valid set to true
    pub fn draft_tx(&self) -> Result<Transaction, JsError> {
        self.0.draft_tx().map(Into::into).map_err(Into::into)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SignedTxBuilder(cml_chain::builders::tx_builder::SignedTxBuilder);

impl_wasm_conversions!(
    cml_chain::builders::tx_builder::SignedTxBuilder,
    SignedTxBuilder
);

#[wasm_bindgen]
impl SignedTxBuilder {
    pub fn new_with_data(
        body: &TransactionBody,
        witness_set: &TransactionWitnessSetBuilder,
        is_valid: bool,
        auxiliary_data: &AuxiliaryData,
    ) -> SignedTxBuilder {
        cml_chain::builders::tx_builder::SignedTxBuilder::new_with_data(
            body.clone().into(),
            witness_set.clone().into(),
            is_valid,
            auxiliary_data.clone().into(),
        )
        .into()
    }

    pub fn new_without_data(
        body: &TransactionBody,
        witness_set: &TransactionWitnessSetBuilder,
        is_valid: bool,
    ) -> SignedTxBuilder {
        cml_chain::builders::tx_builder::SignedTxBuilder::new_without_data(
            body.clone().into(),
            witness_set.clone().into(),
            is_valid,
        )
        .into()
    }

    /**
     * Builds the final transaction and checks that all witnesses are there
     */
    pub fn build_checked(&self) -> Result<Transaction, JsError> {
        self.0
            .clone()
            .build_checked()
            .map(Into::into)
            .map_err(Into::into)
    }

    /**
     * Builds the transaction without doing any witness checks.
     *
     * This can be useful if other witnesses will be added later.
     * e.g. CIP30 signing takes a Transaction with possible witnesses
     * to send to the wallet to fill in the missing ones.
     */
    pub fn build_unchecked(&self) -> Transaction {
        self.0.clone().build_unchecked().into()
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
    pub fn add_vkey(&mut self, vkey: &Vkeywitness) {
        self.0.add_vkey(vkey.clone().into())
    }

    pub fn add_bootstrap(&mut self, bootstrap: &BootstrapWitness) {
        self.0.add_bootstrap(bootstrap.clone().into())
    }

    pub fn body(&self) -> TransactionBody {
        self.0.body().into()
    }

    pub fn witness_set(&self) -> TransactionWitnessSetBuilder {
        self.0.witness_set().into()
    }

    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    pub fn auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.0.auxiliary_data().map(|aux| aux.into())
    }
}
