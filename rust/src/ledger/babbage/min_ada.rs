#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use wasm_bindgen::prelude::*;

use std::cmp;

use crate::Datum;
use crate::TransactionOutput;
use crate::address::Address;
use crate::error::JsError;
use crate::ledger::common::value::BigNum;
use crate::ledger::common::value::Coin;
use crate::ledger::common::value::Value;
use crate::ledger::common::value::to_bignum;
use crate::plutus::ScriptRef;

#[wasm_bindgen]
pub fn min_ada_required(
    output: &TransactionOutput,
    coins_per_utxo_byte: &BigNum, // protocol parameter (in lovelace)
) -> Result<BigNum, JsError> {
    // constant from figure 5 in Babbage spec meant to represent the size the input in a UTXO
    let constant_overhead = 160_u64;

    let old_coin_size = output.amount.coin.to_bytes().len();

    // most recent estimate of the size in bytes to include the minimum ADA value
    let mut latest_size = old_coin_size;

    // we calculate min ada in a loop because every time we increase the min ADA, it may increase the CBOR size in bytes
    loop {
        let size_diff = latest_size as i128 - old_coin_size as i128;

        let tentative_min_ada = to_bignum((output.to_bytes().len() as i128 + constant_overhead as i128 + size_diff) as u64)
            .checked_mul(coins_per_utxo_byte)?;

        let new_coin_size = tentative_min_ada
            .to_bytes()
            .len();

        let is_done = latest_size == new_coin_size;
        latest_size = new_coin_size;
        if is_done {
            break;
        }
    }

    // how many bytes the size changed from including the minimum ADA value
    let size_change = latest_size as i128 - old_coin_size as i128;

    let adjusted_min_ada = to_bignum((output.to_bytes().len() as i128 + constant_overhead as i128 + size_change) as u64)
        .checked_mul(coins_per_utxo_byte)?;
    Ok(adjusted_min_ada)
}

// note: don't expose to WASM since these Option<> types would be dangerous
pub fn min_pure_ada(coins_per_utxo_byte: &BigNum, address: &Address, datum: &Option<Datum>, script_ref: &Option<ScriptRef>) -> Result<BigNum, JsError> {
    let mut output = TransactionOutput::new(
        address,
        // arbitrary value that happens to give the right number of bytes at the CBOR level
        &Value::new(&Coin::from_str("1000000")?),
    );
    if let Some(d) = &datum {
        output.set_datum(d);
    }
    if let Some(s) = &script_ref {
        output.set_script_ref(s);
    }
    min_ada_required( 
        &output,
        coins_per_utxo_byte,
    )
}

#[cfg(test)]
mod tests {
    use crate::{crypto::{ScriptHash, Bip32PrivateKey}, address::{StakeCredential, BaseAddress, NetworkInfo}, MultiAsset, Assets, PolicyID, AssetName, ledger::common::value::{from_bignum, Value}};

    use super::*;

    // this is what is used in mainnet
    const COINS_PER_UTXO_BYTE: u64 = 4310;

    fn test_output() -> TransactionOutput {
        fn harden(index: u32) -> u32 {
            index | 0x80_00_00_00
        }
        fn root_key_15() -> Bip32PrivateKey {
            // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
            let entropy = [
                0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22,
                0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12,
            ];
            Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
        }
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
        let address = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            &spend_cred,
            &stake_cred,
        )
        .to_address();
        TransactionOutput::new(&address, &Value::new(&to_bignum(0)))
    }

    // taken from https://github.com/input-output-hk/cardano-ledger-specs/blob/master/doc/explanations/min-utxo-alonzo.rst
    fn one_policy_one_0_char_asset() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![]),
            &BigNum::from(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        Value {
            coin: BigNum::from(0),
            multiasset: Some(token_bundle),
        }
    }

    fn one_policy_one_1_char_asset() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![1]),
            &BigNum::from(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        Value {
            coin: BigNum::from(1407406),
            multiasset: Some(token_bundle),
        }
    }

    fn one_policy_three_1_char_assets() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![1]),
            &BigNum::from(1)
        );
        asset_list.insert(
            &AssetName(vec![2]),
            &BigNum::from(1)
        );
        asset_list.insert(
            &AssetName(vec![3]),
            &BigNum::from(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        Value {
            coin: BigNum::from(1555554),
            multiasset: Some(token_bundle),
        }
    }

    fn two_policies_one_0_char_asset() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![]),
            &BigNum::from(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        token_bundle.insert(
            &PolicyID::from([1; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        Value {
            coin: BigNum::from(1592591),
            multiasset: Some(token_bundle),
        }
    }

    fn two_policies_one_1_char_asset() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![1]),
            &BigNum::from(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        token_bundle.insert(
            &PolicyID::from([1; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        Value {
            coin: BigNum::from(1592591),
            multiasset: Some(token_bundle),
        }
    }

    fn three_policies_96_1_char_assets() -> Value {
        let mut token_bundle = MultiAsset::new();
        fn add_policy(token_bundle: &mut MultiAsset, index: u8) {
            let mut asset_list = Assets::new();

            for i in 0..32 {
                asset_list.insert(
                    &AssetName(vec![index * 32 + i]),
                    &BigNum::from(1)
                );
            }
            token_bundle.insert(
                &PolicyID::from([index; ScriptHash::BYTE_COUNT]),
                &asset_list
            );
        }
        add_policy(&mut token_bundle, 1);
        add_policy(&mut token_bundle, 2);
        add_policy(&mut token_bundle, 3);
        Value {
            coin: BigNum::from(7592585),
            multiasset: Some(token_bundle),
        }
    }

    fn one_policy_three_32_char_assets() -> Value {
        let mut token_bundle = MultiAsset::new();
        let mut asset_list = Assets::new();
        asset_list.insert(
            &AssetName(vec![1; 32]),
            &BigNum::from(1)
        );
        asset_list.insert(
            &AssetName(vec![2; 32]),
            &BigNum::from(1)
        );
        asset_list.insert(
            &AssetName(vec![3; 32]),
            &BigNum::from(1)
        );
        token_bundle.insert(
            &PolicyID::from([0; ScriptHash::BYTE_COUNT]),
            &asset_list
        );
        Value {
            coin: BigNum::from(1555554),
            multiasset: Some(token_bundle),
        }
    }

    #[test]
    fn min_ada_value_no_multiasset() {
        let check_output = test_output();
        assert_eq!(
            from_bignum(&min_ada_required(&check_output, &to_bignum(COINS_PER_UTXO_BYTE)).unwrap()),
            969750,
        );
    }

    #[test]
    fn min_ada_value_one_policy_one_0_char_asset() {
        let mut check_output = test_output();
        check_output.amount = one_policy_one_0_char_asset();
        assert_eq!(
            from_bignum(&min_ada_required(&check_output, &to_bignum(COINS_PER_UTXO_BYTE)).unwrap()),
            1120600,
        );
    }

    #[test]
    fn min_ada_value_one_policy_one_1_char_asset() {
        let mut check_output = test_output();
        check_output.amount = one_policy_one_1_char_asset();
        assert_eq!(
            from_bignum(&min_ada_required(&check_output, &to_bignum(COINS_PER_UTXO_BYTE)).unwrap()),
            1124910,
        );
    }

    #[test]
    fn min_ada_value_one_policy_three_1_char_assets() {
        let mut check_output = test_output();
        check_output.amount = one_policy_three_1_char_assets();
        assert_eq!(
            from_bignum(&min_ada_required(&check_output, &to_bignum(COINS_PER_UTXO_BYTE)).unwrap()),
            1150770,
        );
    }

    #[test]
    fn min_ada_value_two_policies_one_0_char_asset() {
        let mut check_output = test_output();
        check_output.amount = two_policies_one_0_char_asset();
        assert_eq!(
            from_bignum(&min_ada_required(&check_output, &to_bignum(COINS_PER_UTXO_BYTE)).unwrap()),
            1262830,
        );
    }

    #[test]
    fn min_ada_value_two_policies_one_1_char_asset() {
        let mut check_output = test_output();
        check_output.amount = two_policies_one_1_char_asset();
        assert_eq!(
            from_bignum(&min_ada_required(&check_output, &to_bignum(COINS_PER_UTXO_BYTE)).unwrap()),
            1271450,
        );
    }

    #[test]
    fn min_ada_value_three_policies_96_1_char_assets() {
        let mut check_output = test_output();
        check_output.amount = three_policies_96_1_char_assets();
        assert_eq!(
            from_bignum(&min_ada_required(&check_output, &to_bignum(COINS_PER_UTXO_BYTE)).unwrap()),
            2633410,
        );
    }

    #[test]
    fn min_ada_value_one_policy_one_0_char_asset_datum_hash() {
        let mut check_output = test_output();
        check_output.amount = one_policy_one_0_char_asset();
        assert_eq!(
            from_bignum(&min_ada_required(&check_output, &to_bignum(COINS_PER_UTXO_BYTE)).unwrap()),
            1120600,
        );
    }

    #[test]
    fn min_ada_value_one_policy_three_32_char_assets_datum_hash() {
        let mut check_output = test_output();
        check_output.amount = one_policy_three_32_char_assets();
        assert_eq!(
            from_bignum(&min_ada_required(&check_output, &to_bignum(COINS_PER_UTXO_BYTE)).unwrap()),
            1564530,
        );
    }

    #[test]
    fn min_ada_value_two_policies_one_0_char_asset_datum_hash() {
        let mut check_output = test_output();
        check_output.amount = two_policies_one_0_char_asset();
        assert_eq!(
            from_bignum(&min_ada_required(&check_output, &to_bignum(COINS_PER_UTXO_BYTE)).unwrap()),
            1262830,
        );
    }
}