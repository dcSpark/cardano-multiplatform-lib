// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

// This file records every collection wrapper this crate borrows from workspace deps.
// It is machine-read by those deps' generation runs (--wrapper-requests) and compiled
// here, so a wrapper a dep stops providing fails THIS crate's build, naming the type.
// Rows are (dep rust-crate name, wrapper name, shape in CDDL syntax with the dep's idents).
#[allow(unused_imports)]
mod borrowed {
    use cml_chain_wasm::collections::AlonzoFormatTxOutList;
    use cml_chain_wasm::collections::AssetNameList;
    use cml_chain_wasm::collections::BootstrapWitnessList;
    use cml_chain_wasm::collections::Ed25519KeyHashList;
    use cml_chain_wasm::collections::GenesisHashList;
    use cml_chain_wasm::collections::MapAssetNameToNonZeroInt64;
    use cml_chain_wasm::collections::MapStakeCredentialToCoin;
    use cml_chain_wasm::collections::MapStakeCredentialToDeltaCoin;
    use cml_chain_wasm::collections::MapTransactionIndexToMetadata;
    use cml_chain_wasm::collections::NativeScriptList;
    use cml_chain_wasm::collections::PlutusDataList;
    use cml_chain_wasm::collections::PlutusV1ScriptList;
    use cml_chain_wasm::collections::PlutusV2ScriptList;
    use cml_chain_wasm::collections::PolicyIdList;
    use cml_chain_wasm::collections::RewardAccountList;
    use cml_chain_wasm::collections::StakeCredentialList;
    use cml_chain_wasm::collections::TransactionInputList;
    use cml_chain_wasm::collections::VkeywitnessList;
}
#[allow(dead_code)]
pub(crate) const BORROWED_SHAPES: &[(&str, &str, &str)] = &[
    (
        "cml_chain",
        "AlonzoFormatTxOutList",
        "[* alonzo_format_tx_out]",
    ),
    ("cml_chain", "AssetNameList", "[* asset_name]"),
    ("cml_chain", "BootstrapWitnessList", "[* bootstrap_witness]"),
    ("cml_chain", "Ed25519KeyHashList", "[* ed25519_key_hash]"),
    ("cml_chain", "GenesisHashList", "[* genesis_hash]"),
    (
        "cml_chain",
        "MapAssetNameToNonZeroInt64",
        "{* asset_name => non_zero_int64}",
    ),
    (
        "cml_chain",
        "MapStakeCredentialToCoin",
        "{* stake_credential => coin}",
    ),
    (
        "cml_chain",
        "MapStakeCredentialToDeltaCoin",
        "{* stake_credential => delta_coin}",
    ),
    (
        "cml_chain",
        "MapTransactionIndexToMetadata",
        "{* transaction_index => metadata}",
    ),
    ("cml_chain", "NativeScriptList", "[* native_script]"),
    ("cml_chain", "PlutusDataList", "[* plutus_data]"),
    ("cml_chain", "PlutusV1ScriptList", "[* plutus_v1_script]"),
    ("cml_chain", "PlutusV2ScriptList", "[* plutus_v2_script]"),
    ("cml_chain", "PolicyIdList", "[* policy_id]"),
    ("cml_chain", "RewardAccountList", "[* reward_account]"),
    ("cml_chain", "StakeCredentialList", "[* stake_credential]"),
    ("cml_chain", "TransactionInputList", "[* transaction_input]"),
    ("cml_chain", "VkeywitnessList", "[* vkeywitness]"),
];
