// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

// This file records every map-key type this crate borrows from workspace deps.
// It is machine-read by those deps' generation runs (--key-requests) so they derive the key
// traits (Eq/Ord/PartialOrd, plus Hash under --preserve-encodings) on the borrowed type; the
// compiled self-check below fails THIS crate's build if a dep drops such a derive.
// Rows are (dep rust-crate name, cddl ident) of each borrowed map-key type.
#[allow(dead_code)]
fn _assert_key_traits<K: Eq + Ord + PartialOrd + core::hash::Hash>() {}
#[allow(dead_code)]
fn _borrowed_key_types_self_check() {
    _assert_key_traits::<cml_chain::assets::AssetName>();
    _assert_key_traits::<cml_chain::crypto::GenesisHash>();
    _assert_key_traits::<cml_chain::PolicyId>();
    _assert_key_traits::<cml_chain::address::RewardAccount>();
    _assert_key_traits::<cml_chain::certs::StakeCredential>();
}
#[allow(dead_code)]
pub(crate) const BORROWED_KEY_TYPES: &[(&str, &str)] = &[
    ("cml_chain", "asset_name"),
    ("cml_chain", "genesis_hash"),
    ("cml_chain", "policy_id"),
    ("cml_chain", "reward_account"),
    ("cml_chain", "stake_credential"),
];
