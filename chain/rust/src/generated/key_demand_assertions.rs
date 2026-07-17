// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

// Compile-time key-demand assertions for `@used_as_key` tags. Each
// `_demand_<rule>` fn makes the Rust compiler prove the tagged type implements the
// traits its tag demands, turning a distant downstream trait error into a near,
// named one at the tagged type's definition site.
#[allow(dead_code)]
fn _key_demand_hash<T: core::hash::Hash + Eq>() {}
#[allow(dead_code)]
fn _key_demand_ord<T: Ord>() {}
#[allow(dead_code)]
fn _demand_asset_name() {
    // required by `@used_as_key` on asset_name
    _key_demand_hash::<crate::generated::assets::AssetName>();
    _key_demand_ord::<crate::generated::assets::AssetName>();
}
#[allow(dead_code)]
fn _demand_ex_units() {
    // required by `@used_as_key` on ex_units
    _key_demand_hash::<crate::generated::plutus::ExUnits>();
    _key_demand_ord::<crate::generated::plutus::ExUnits>();
}
#[allow(dead_code)]
fn _demand_plutus_data() {
    // required by `@used_as_key` on plutus_data
    _key_demand_hash::<crate::generated::plutus::PlutusData>();
    _key_demand_ord::<crate::generated::plutus::PlutusData>();
}
#[allow(dead_code)]
fn _demand_script() {
    // required by `@used_as_key` on Script
    _key_demand_hash::<crate::generated::Script>();
    _key_demand_ord::<crate::generated::Script>();
}
#[allow(dead_code)]
fn _demand_transaction_input() {
    // required by `@used_as_key` on transaction_input
    _key_demand_hash::<crate::generated::transaction::TransactionInput>();
    _key_demand_ord::<crate::generated::transaction::TransactionInput>();
}
