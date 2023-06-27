use std::convert::TryInto;

use cml_core::ArithmeticError;
use cml_crypto::Serialize;

use crate::{
    Coin,
    transaction::TransactionOutput,
};

pub fn min_ada_required(
    output: &TransactionOutput,
    coins_per_utxo_byte: Coin, // protocol parameter (in lovelace)
) -> Result<Coin, ArithmeticError> {
    use cml_core::serialization::fit_sz;

    let output_size = output.to_cbor_bytes().len();

    // constant from figure 5 in Babbage spec meant to represent the size the input in a UTXO
    let constant_overhead = 160_u64;

    // how many bytes the Coin part of the Value will take. can vary based on encoding used.
    let old_coin_size = 1 + fit_sz(
        output.amount().coin,
        output.amount().encodings.as_ref().and_then(|enc| enc.coin_encoding),
        false
    ).bytes_following();

    // most recent estimate of the size in bytes to include the minimum ADA value
    let mut latest_size = old_coin_size;

    // we calculate min ada in a loop because every time we increase the min ADA, it may increase the CBOR size in bytes
    loop {
        let size_diff = latest_size as i128 - old_coin_size as i128;

        let tentative_min_ada = (output_size as i128 + constant_overhead as i128 + size_diff)
            .try_into()
            .ok()
            .and_then(|x: u64| x.checked_mul(coins_per_utxo_byte))
            .ok_or(ArithmeticError::IntegerOverflow)?;

        let new_coin_size = 1 + fit_sz(tentative_min_ada, None, false)
            .bytes_following();

        let is_done = latest_size == new_coin_size;
        latest_size = new_coin_size;
        if is_done {
            break;
        }
    }

    // how many bytes the size changed from including the minimum ADA value
    let size_change = latest_size as i128 - old_coin_size as i128;

    let adjusted_min_ada = (output_size as i128 + constant_overhead as i128 + size_change)
        .try_into()
        .ok()
        .and_then(|x: u64| x.checked_mul(coins_per_utxo_byte))
        .ok_or(ArithmeticError::IntegerOverflow)?;
    Ok(adjusted_min_ada)
}


#[cfg(test)]
mod tests {
    use crate::{
        genesis::network_info::NetworkInfo,
        address::BaseAddress,
        certs::StakeCredential,
        assets::{MultiAsset, Value},
        PolicyId,
        AssetName,
        transaction::{ShelleyTxOut}
    };

    use cml_core::ordered_hash_map::OrderedHashMap;
    use cml_crypto::{ScriptHash, Bip32PrivateKey};

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

        let spend_cred = StakeCredential::new_pub_key(spend.to_raw_key().hash());
        let stake_cred = StakeCredential::new_pub_key(stake.to_raw_key().hash());
        let address = BaseAddress::new(
            NetworkInfo::testnet().network_id(),
            spend_cred,
            stake_cred,
        )
        .to_address();
        ShelleyTxOut::new(address, Value::from(0)).into()
    }

    // taken from https://github.com/input-output-hk/cardano-ledger-specs/blob/master/doc/explanations/min-utxo-alonzo.rst
    fn one_policy_one_0_char_asset() -> Value {
        let mut token_bundle = MultiAsset::default();
        token_bundle.set(
            PolicyId::from([0; ScriptHash::BYTE_COUNT]),
            AssetName::new(vec![]).unwrap(),
            1,
        );
        Value::new(
            0,
            token_bundle,
        )
    }

    fn one_policy_one_1_char_asset() -> Value {
        let mut token_bundle = MultiAsset::default();
        token_bundle.set(
            PolicyId::from([0; ScriptHash::BYTE_COUNT]),
            AssetName::new(vec![1]).unwrap(),
            1,
        );
        Value::new(
            1407406,
            token_bundle,
        )
    }

    fn one_policy_three_1_char_assets() -> Value {
        let mut token_bundle = MultiAsset::default();
        let mut asset_list = OrderedHashMap::new();
        asset_list.insert(
            AssetName::new(vec![1]).unwrap(),
            1
        );
        asset_list.insert(
            AssetName::new(vec![2]).unwrap(),
            1
        );
        asset_list.insert(
            AssetName::new(vec![3]).unwrap(),
            1
        );
        token_bundle.insert(
            PolicyId::from([0; ScriptHash::BYTE_COUNT]),
            asset_list
        );
        Value::new(1555554, token_bundle)
    }

    fn two_policies_one_0_char_asset() -> Value {
        let mut token_bundle = MultiAsset::default();
        token_bundle.set(
            PolicyId::from([0; ScriptHash::BYTE_COUNT]),
            AssetName::new(vec![]).unwrap(),
            1
        );
        token_bundle.set(
            PolicyId::from([1; ScriptHash::BYTE_COUNT]),
            AssetName::new(vec![]).unwrap(),
            1
        );
        Value::new(1592591, token_bundle)
    }

    fn two_policies_one_1_char_asset() -> Value {
        let mut token_bundle = MultiAsset::default();
        token_bundle.set(
            PolicyId::from([0; ScriptHash::BYTE_COUNT]),
            AssetName::new(vec![1]).unwrap(),
            1,
        );
        token_bundle.set(
            PolicyId::from([1; ScriptHash::BYTE_COUNT]),
            AssetName::new(vec![1]).unwrap(),
            1,
        );
        Value::new(1592591, token_bundle)
    }

    fn three_policies_96_1_char_assets() -> Value {
        let mut token_bundle = MultiAsset::default();
        fn add_policy(token_bundle: &mut MultiAsset, index: u8) {
            let mut asset_list = OrderedHashMap::new();

            for i in 0..32 {
                asset_list.insert(
                    AssetName::new(vec![index * 32 + i]).unwrap(),
                    1
                );
            }
            token_bundle.insert(
                PolicyId::from([index; ScriptHash::BYTE_COUNT]),
                asset_list
            );
        }
        add_policy(&mut token_bundle, 1);
        add_policy(&mut token_bundle, 2);
        add_policy(&mut token_bundle, 3);
        Value::new(7592585, token_bundle)
    }

    fn one_policy_three_32_char_assets() -> Value {
        let mut token_bundle = MultiAsset::default();
        let mut asset_list = OrderedHashMap::new();
        asset_list.insert(
            AssetName::new(vec![1; 32]).unwrap(),
            1
        );
        asset_list.insert(
            AssetName::new(vec![2; 32]).unwrap(),
            1
        );
        asset_list.insert(
            AssetName::new(vec![3; 32]).unwrap(),
            1
        );
        token_bundle.insert(
            PolicyId::from([0; ScriptHash::BYTE_COUNT]),
            asset_list
        );
        Value::new(1555554, token_bundle)
    }

    #[test]
    fn min_ada_value_no_multiasset() {
        let check_output = test_output();
        assert_eq!(
            min_ada_required(&check_output, COINS_PER_UTXO_BYTE).unwrap(),
            969750,
        );
    }

    #[test]
    fn min_ada_value_one_policy_one_0_char_asset() {
        let mut check_output = test_output();
        check_output.set_amount(one_policy_one_0_char_asset());
        assert_eq!(
            min_ada_required(&check_output, COINS_PER_UTXO_BYTE).unwrap(),
            1120600,
        );
    }

    #[test]
    fn min_ada_value_one_policy_one_1_char_asset() {
        let mut check_output = test_output();
        check_output.set_amount(one_policy_one_1_char_asset());
        assert_eq!(
            min_ada_required(&check_output, COINS_PER_UTXO_BYTE).unwrap(),
            1124910,
        );
    }

    #[test]
    fn min_ada_value_one_policy_three_1_char_assets() {
        let mut check_output = test_output();
        check_output.set_amount(one_policy_three_1_char_assets());
        assert_eq!(
            min_ada_required(&check_output, COINS_PER_UTXO_BYTE).unwrap(),
            1150770,
        );
    }

    #[test]
    fn min_ada_value_two_policies_one_0_char_asset() {
        let mut check_output = test_output();
        check_output.set_amount(two_policies_one_0_char_asset());
        assert_eq!(
            min_ada_required(&check_output, COINS_PER_UTXO_BYTE).unwrap(),
            1262830,
        );
    }

    #[test]
    fn min_ada_value_two_policies_one_1_char_asset() {
        let mut check_output = test_output();
        check_output.set_amount(two_policies_one_1_char_asset());
        assert_eq!(
            min_ada_required(&check_output, COINS_PER_UTXO_BYTE).unwrap(),
            1271450,
        );
    }

    #[test]
    fn min_ada_value_three_policies_96_1_char_assets() {
        let mut check_output = test_output();
        check_output.set_amount(three_policies_96_1_char_assets());
        assert_eq!(
            min_ada_required(&check_output, COINS_PER_UTXO_BYTE).unwrap(),
            2633410,
        );
    }

    #[test]
    fn min_ada_value_one_policy_one_0_char_asset_datum_hash() {
        let mut check_output = test_output();
        check_output.set_amount(one_policy_one_0_char_asset());
        assert_eq!(
            min_ada_required(&check_output, COINS_PER_UTXO_BYTE).unwrap(),
            1120600,
        );
    }

    #[test]
    fn min_ada_value_one_policy_three_32_char_assets_datum_hash() {
        let mut check_output = test_output();
        check_output.set_amount(one_policy_three_32_char_assets());
        assert_eq!(
            min_ada_required(&check_output, COINS_PER_UTXO_BYTE).unwrap(),
            1564530,
        );
    }

    #[test]
    fn min_ada_value_two_policies_one_0_char_asset_datum_hash() {
        let mut check_output = test_output();
        check_output.set_amount(two_policies_one_0_char_asset());
        assert_eq!(
            min_ada_required(&check_output, COINS_PER_UTXO_BYTE).unwrap(),
            1262830,
        );
    }
}
