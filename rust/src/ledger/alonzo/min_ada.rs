use crate::ledger::common::value::{Value, to_bignum};

use super::super::super::*;
use std::ops::{Rem, Div, Sub};
use itertools::Itertools;

struct OutputSizeConstants {
    k0: usize,
    k1: usize,
    k2: usize,
}

fn quot<T>(a: T, b: T) -> T
where T: Sub<Output=T> + Rem<Output=T> + Div<Output=T> + Copy + Clone + std::fmt::Display {
    (a - (a % b)) / b
}

fn bundle_size(
    assets: &Value,
    constants: &OutputSizeConstants,
) -> usize {
    // based on https://github.com/input-output-hk/cardano-ledger-specs/blob/master/doc/explanations/min-utxo-alonzo.rst
    match &assets.multiasset {
        None => 2, // coinSize according the minimum value function
        Some (assets) => {
            let num_assets = assets.0
                .values()
                .fold(
                    0,
                    | acc, next| acc + next.len()
                );
            let sum_asset_name_lengths = assets.0
                .values()
                .flat_map(|assets| assets.0.keys())
                .unique_by(|asset| asset.name())
                .fold(
                    0,
                    | acc, next| acc + next.0.len()
                );
            let sum_policy_id_lengths = assets.0
                .keys()
                .fold(
                    0,
                    | acc, next| acc + next.0.len()
                );
            // converts bytes to 8-byte long words, rounding up
            fn roundup_bytes_to_words(b: usize) -> usize {
                quot(b + 7, 8)
            }
            constants.k0 + roundup_bytes_to_words(
                (num_assets * constants.k1) + sum_asset_name_lengths +
                (constants.k2 * sum_policy_id_lengths)
            )
        }
    }
}

#[deprecated(
    since = "0.4.0",
    note = "This calculation is no longer used in the Babbage era"
)]
pub fn min_ada_required(
    assets: &Value,
    has_data_hash: bool, // whether the output includes a data hash
    coins_per_utxo_word: &BigNum, // protocol parameter (in lovelace)
) -> Result<BigNum, JsError> {
    // based on https://github.com/input-output-hk/cardano-ledger-specs/blob/master/doc/explanations/min-utxo-alonzo.rst
    let data_hash_size = if has_data_hash { 10 } else { 0 }; // in words
    let utxo_entry_size_without_val = 27; // in words

    let size = bundle_size(
        assets,
        &OutputSizeConstants {
            k0: 6,
            k1: 12,
            k2: 1,
        },
    );
    let words = to_bignum(utxo_entry_size_without_val)
        .checked_add(&to_bignum(size as u64))?
        .checked_add(&to_bignum(data_hash_size))?;
    coins_per_utxo_word.checked_mul(&words)
}

#[deprecated(
    since = "0.4.0",
    note = "This calculation is no longer used in the Babbage era"
)]
pub fn min_pure_ada(coins_per_utxo_word: &BigNum, has_data_hash: bool) -> Result<BigNum, JsError> {
    min_ada_required(
        // arbitrary value that happens to give the right number of bytes at the CBOR level
        &Value::new(&Coin::from_str("1000000")?),
        has_data_hash,
        coins_per_utxo_word,
    )
}

#[cfg(test)]
mod tests {
    use crate::ledger::common::value::from_bignum;

    use super::*;

    // this is what is used in mainnet
    const COINS_PER_UTXO_WORD: u64 = 34_482;

    fn bundle_constants() -> OutputSizeConstants {
        OutputSizeConstants {
            k0: 6,
            k1: 12,
            k2: 1,
        }
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
        assert_eq!(
            from_bignum(&min_ada_required(&Value::new(&Coin::zero()), false, &to_bignum(COINS_PER_UTXO_WORD)).unwrap()),
            999978,
        );
    }

    #[test]
    fn min_ada_value_one_policy_one_0_char_asset() {
        assert_eq!(
            from_bignum(&min_ada_required(&one_policy_one_0_char_asset(), false, &to_bignum(COINS_PER_UTXO_WORD)).unwrap()),
            1_310_316,
        );
    }

    #[test]
    fn min_ada_value_one_policy_one_1_char_asset() {
        assert_eq!(
            from_bignum(&min_ada_required(&one_policy_one_1_char_asset(), false, &to_bignum(COINS_PER_UTXO_WORD)).unwrap()),
            1_344_798,
        );
    }

    #[test]
    fn min_ada_value_one_policy_three_1_char_assets() {
        assert_eq!(
            from_bignum(&min_ada_required(&one_policy_three_1_char_assets(), false, &to_bignum(COINS_PER_UTXO_WORD)).unwrap()),
            1_448_244,
        );
    }

    #[test]
    fn min_ada_value_two_policies_one_0_char_asset() {
        assert_eq!(
            from_bignum(&min_ada_required(&two_policies_one_0_char_asset(), false, &to_bignum(COINS_PER_UTXO_WORD)).unwrap()),
            1_482_726,
        );
    }

    #[test]
    fn min_ada_value_two_policies_one_1_char_asset() {
        assert_eq!(
            from_bignum(&min_ada_required(&two_policies_one_1_char_asset(), false, &to_bignum(COINS_PER_UTXO_WORD)).unwrap()),
            1_517_208,
        );
    }

    #[test]
    fn min_ada_value_three_policies_96_1_char_assets() {
        assert_eq!(
            from_bignum(&min_ada_required(&three_policies_96_1_char_assets(), false, &to_bignum(COINS_PER_UTXO_WORD)).unwrap()),
            6_896_400,
        );
    }

    #[test]
    fn min_ada_value_one_policy_one_0_char_asset_datum_hash() {
        assert_eq!(
            from_bignum(&min_ada_required(&one_policy_one_0_char_asset(), true, &to_bignum(COINS_PER_UTXO_WORD)).unwrap()),
            1_655_136,
        );
    }

    #[test]
    fn min_ada_value_one_policy_three_32_char_assets_datum_hash() {
        assert_eq!(
            from_bignum(&min_ada_required(&one_policy_three_32_char_assets(), true, &to_bignum(COINS_PER_UTXO_WORD)).unwrap()),
            2_172_366,
        );
    }

    #[test]
    fn min_ada_value_two_policies_one_0_char_asset_datum_hash() {
        assert_eq!(
            from_bignum(&min_ada_required(&two_policies_one_0_char_asset(), true, &to_bignum(COINS_PER_UTXO_WORD)).unwrap()),
            1_827_546,
        );
    }

    #[test]
    fn bundle_sizes() {
        assert_eq!(
            bundle_size(&one_policy_one_0_char_asset(), &bundle_constants()),
            11
        );
        assert_eq!(
            bundle_size(&one_policy_one_1_char_asset(), &bundle_constants()),
            12
        );
        assert_eq!(
            bundle_size(&one_policy_three_1_char_assets(), &bundle_constants()),
            15
        );
        assert_eq!(
            bundle_size(&two_policies_one_0_char_asset(), &bundle_constants()),
            16
        );
        assert_eq!(
            bundle_size(&two_policies_one_1_char_asset(), &bundle_constants()),
            17
        );
        assert_eq!(
            bundle_size(&three_policies_96_1_char_assets(), &bundle_constants()),
            173
        );
        assert_eq!(
            bundle_size(&one_policy_three_32_char_assets(), &bundle_constants()),
            26
        );
    }
}