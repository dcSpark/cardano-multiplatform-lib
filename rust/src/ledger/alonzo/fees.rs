use crate::ledger::common::value::{to_bignum, from_bignum};
use fraction::{Fraction, ToPrimitive};
use super::super::super::*;

/// Careful: although the linear fee is the same for Byron & Shelley
/// The value of the parameters and how fees are computed is not the same
#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LinearFee {
    constant: Coin,
    coefficient: Coin,
}

#[wasm_bindgen]
impl LinearFee {
    pub fn constant(&self) -> Coin {
        self.constant
    }

    pub fn coefficient(&self) -> Coin {
        self.coefficient
    }

    pub fn new(coefficient: &Coin, constant: &Coin) -> Self {
        Self {
            constant: constant.clone(),
            coefficient: coefficient.clone(),
        }
    }
}

#[wasm_bindgen]
pub fn min_script_fee(tx: &Transaction, ex_unit_prices: &ExUnitPrices) -> Result<Coin, JsError> {
    if let Some(redeemers) = tx.witness_set().redeemers() {
        let total_ex_units = redeemers.get_total_ex_units()?;
        let script_fee = to_bignum(
            (
                (
                    Fraction::new(from_bignum(&total_ex_units.mem()), 1u64)
                    * Fraction::new(
                        from_bignum(&ex_unit_prices.mem_price().numerator),
                        from_bignum(&ex_unit_prices.mem_price().denominator),
                    )
                )
                +
                (
                    Fraction::new(from_bignum(&total_ex_units.steps()), 1u64)
                    * Fraction::new(
                        from_bignum(&ex_unit_prices.step_price().numerator),
                        from_bignum(&ex_unit_prices.step_price().denominator),
                    )
                )
            ).ceil().to_u64().unwrap(),
        );
        Ok(script_fee)
    } else {
        Ok(Coin::zero())
    }
}

#[wasm_bindgen]
pub fn min_fee(
    tx: &Transaction,
    linear_fee: &LinearFee,
    ex_unit_prices: &ExUnitPrices
) -> Result<Coin, JsError> {
    // TODO: the fee should be 0 if all inputs are genesis redeem addresses
    let mut fee = to_bignum(tx.to_bytes().len() as u64)
        .checked_mul(&linear_fee.coefficient())?
        .checked_add(&linear_fee.constant())?;
    fee = fee.checked_add(&min_script_fee(tx, ex_unit_prices)?)?;
    Ok(fee)
}

// Note: all the tests below are disabled
// This is because these tests are meant to run on Alonzo-era tx formats to give the right result
// But currently CML doesn't allow you to encode txs in old formats explicitly
// which means these tests return the wrong value until we add this feature

// #[cfg(test)]
// mod tests {
//     use crate::ledger::{common::hash::hash_transaction, shelley::witness::make_vkey_witness, byron::witness::make_icarus_bootstrap_witness};

//     use super::*;
//     use crypto::*;
//     use address::*;
//     use super::builders::output_builder::TransactionOutputBuilder;

//     fn exunits_constants() -> ExUnitPrices {
//         ExUnitPrices::new(
//             &SubCoin::new(&BigNum::zero(), &BigNum::zero()),
//             &SubCoin::new(&BigNum::zero(), &BigNum::zero())
//         )
//     }

//     // based off tx test vectors (https://gist.github.com/KtorZ/5a2089df0915f21aca368d12545ab230)

//     // However, they don't match due to serialization differences in definite vs indefinite
//     // CBOR lengths for maps/arrays, thus for now we've got all the tests as >= instead.
//     // It's possible they're still off by a byte or two somewhere.

//     #[test]
//     fn tx_simple_utxo() {
//         // # Vector #1: simple transaction
//         let mut inputs = TransactionInputs::new();
//         inputs.add(&TransactionInput::new(
//             &TransactionHash::from_bytes(
//                 hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
//                     .unwrap(),
//             )
//             .unwrap(),
//             &0.into(),
//         ));
//         let mut outputs = TransactionOutputs::new();

//         outputs.add(
//             &TransactionOutputBuilder::new()
//                 .with_address(
//                     &Address::from_bytes(
//                         hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c")
//                             .unwrap(),
//                     )
//                     .unwrap(),
//                 )
//                 .next()
//                 .unwrap()
//                 .with_coin(&to_bignum(1))
//                 .build()
//                 .unwrap(),
//         );
//         let body = TransactionBody::new(&inputs, &outputs, &to_bignum(94002), Some(10.into()));

//         let mut w = TransactionWitnessSet::new();
//         let mut vkw = Vkeywitnesses::new();
//         vkw.add(&make_vkey_witness(
//             &hash_transaction(&body),
//             &PrivateKey::from_normal_bytes(
//                 &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
//                     .unwrap(),
//             )
//             .unwrap(),
//         ));
//         w.set_vkeys(&vkw);

//         let signed_tx = Transaction::new(&body, &w, None);

//         let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
//         assert_eq!(
//             hex::encode(signed_tx.to_bytes()),
//             "84a400818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7000181a200581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c0101021a00016f32030aa10081825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee5840c26d98f4898c4212f4dadb81b124104e57b9ad07578ffdbbf60fc955ccf250bf3fdbe7c6e73a0a03a604931a983d35e1a45e30bc1f2170e18bae278cb168a808f5f6"
//         );
//         assert_eq!(
//             min_fee(&signed_tx, &linear_fee, &exunits_constants())
//                 .unwrap()
//                 .to_str(),
//             "95502" // todo: compare to Haskell fee to make sure the diff is not too big
//         );
//     }

//     #[test]
//     fn tx_simple_byron_utxo() {
//         let mut inputs = TransactionInputs::new();
//         inputs.add(&TransactionInput::new(
//             &TransactionHash::from_bytes(
//                 hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
//                     .unwrap(),
//             )
//             .unwrap(),
//             &0.into(),
//         ));
//         let mut outputs = TransactionOutputs::new();

//         outputs.add(
//             &TransactionOutputBuilder::new()
//                 .with_address(
//                     &Address::from_bytes(
//                         hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c")
//                             .unwrap(),
//                     )
//                     .unwrap(),
//                 )
//                 .next()
//                 .unwrap()
//                 .with_coin(&to_bignum(1))
//                 .build()
//                 .unwrap(),
//         );
//         let body = TransactionBody::new(&inputs, &outputs, &to_bignum(112002), Some(10.into()));

//         let mut w = TransactionWitnessSet::new();
//         let mut bootstrap_wits = BootstrapWitnesses::new();
//         bootstrap_wits.add(&make_icarus_bootstrap_witness(
//             &hash_transaction(&body),
//             &ByronAddress::from_base58("Ae2tdPwUPEZ6r6zbg4ibhFrNnyKHg7SYuPSfDpjKxgvwFX9LquRep7gj7FQ").unwrap(),
//             &Bip32PrivateKey::from_bytes(
//                 &hex::decode("d84c65426109a36edda5375ea67f1b738e1dacf8629f2bb5a2b0b20f3cd5075873bf5cdfa7e533482677219ac7d639e30a38e2e645ea9140855f44ff09e60c52c8b95d0d35fe75a70f9f5633a3e2439b2994b9e2bc851c49e9f91d1a5dcbb1a3").unwrap()
//             ).unwrap()
//         ));
//         w.set_bootstraps(&bootstrap_wits);

//         let signed_tx = Transaction::new(&body, &w, None);

//         let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
//         assert_eq!(
//             hex::encode(signed_tx.to_bytes()),
//             "84a400818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7000181a200581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c0101021a0001b582030aa10281845820473811afd4d939b337c9be1a2ceeb2cb2c75108bddf224c5c21c51592a7b204a5840882aa695316ec7480fd299df98634cc2dce2946e1df20cf941b218986c0c1c3f9870a52679a439cf6a7f21492e69ce3830e1ed7b6f52a9d8d03f3dff18ecf1005820c8b95d0d35fe75a70f9f5633a3e2439b2994b9e2bc851c49e9f91d1a5dcbb1a341a0f5f6"
//         );
//         assert_eq!(
//             min_fee(&signed_tx, &linear_fee, &exunits_constants())
//                 .unwrap()
//                 .to_str(),
//             "113502" // todo: compare to Haskell fee to make sure the diff is not too big
//         );
//     }

//     #[test]
//     fn tx_multi_utxo() {
//         // # Vector #2: multiple outputs and inputs
//         let mut inputs = TransactionInputs::new();
//         inputs.add(&TransactionInput::new(
//             &TransactionHash::from_bytes(
//                 hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
//                     .unwrap(),
//             )
//             .unwrap(),
//             &42.into(),
//         ));
//         inputs.add(&TransactionInput::new(
//             &TransactionHash::from_bytes(
//                 hex::decode("82839f8200d81858248258203b40265111d8bb3c3c608d95b3a0bf83461ace32")
//                     .unwrap(),
//             )
//             .unwrap(),
//             &7.into(),
//         ));
//         let mut outputs = TransactionOutputs::new();

//         outputs.add(
//             &TransactionOutputBuilder::new()
//                 .with_address(
//                     &Address::from_bytes(
//                         hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c")
//                             .unwrap(),
//                     )
//                     .unwrap(),
//                 )
//                 .next()
//                 .unwrap()
//                 .with_coin(&to_bignum(289))
//                 .build()
//                 .unwrap(),
//         );
//         outputs.add(
//             &TransactionOutputBuilder::new()
//                 .with_address(
//                     &Address::from_bytes(
//                         hex::decode("61bcd18fcffa797c16c007014e2b8553b8b9b1e94c507688726243d611")
//                             .unwrap(),
//                     )
//                     .unwrap(),
//                 )
//                 .next()
//                 .unwrap()
//                 .with_coin(&to_bignum(874551452))
//                 .build()
//                 .unwrap(),
//         );
//         let body = TransactionBody::new(&inputs, &outputs, &to_bignum(183502), Some(999.into()));

//         let mut w = TransactionWitnessSet::new();
//         let mut vkw = Vkeywitnesses::new();
//         vkw.add(&make_vkey_witness(
//             &hash_transaction(&body),
//             &PrivateKey::from_normal_bytes(
//                 &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
//                     .unwrap(),
//             )
//             .unwrap(),
//         ));
//         vkw.add(&make_vkey_witness(
//             &hash_transaction(&body),
//             &PrivateKey::from_normal_bytes(
//                 &hex::decode("13fe79205e16c09536acb6f0524d04069f380329d13949698c5f22c65c989eb4")
//                     .unwrap(),
//             )
//             .unwrap(),
//         ));
//         w.set_vkeys(&vkw);

//         let signed_tx = Transaction::new(&body, &w, None);

//         let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
//         assert_eq!(
//             hex::encode(signed_tx.to_bytes()),
//             "84a400828258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7182a82582082839f8200d81858248258203b40265111d8bb3c3c608d95b3a0bf83461ace32070182a200581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c01190121a200581d61bcd18fcffa797c16c007014e2b8553b8b9b1e94c507688726243d611011a3420989c021a0002ccce031903e7a10082825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee58402748bcd7b4fa1828e29aa928cc43834bf37cec5d361a21f314c699d5ef52c15a3709bb1844f86c648b65237d6775dead74cbc8b4fa0b982b558ef8ded5c5400c8258206872b0a874acfe1cace12b20ea348559a7ecc912f2fc7f674f43481df973d92c5840abe78000c312cf99cd6c7f7cef0945352e150116a3256ceaf37a1302c5e6eb0b9820df6876ca3017f72e462a0c185297babc2260730fd56f97686cc6a549c60af5f6"
//         );
//         assert_eq!(
//             min_fee(&signed_tx, &linear_fee, &exunits_constants())
//                 .unwrap()
//                 .to_str(),
//             "186002" // todo: compare to Haskell fee to make sure the diff is not too big
//         );
//     }

//     #[test]
//     fn tx_register_stake() {
//         // # Vector #3: with stake pool registration certificate
//         let network = 1;
//         let mut inputs = TransactionInputs::new();
//         inputs.add(&TransactionInput::new(
//             &TransactionHash::from_bytes(
//                 hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
//                     .unwrap(),
//             )
//             .unwrap(),
//             &0.into(),
//         ));
//         let mut outputs = TransactionOutputs::new();

//         outputs.add(
//             &TransactionOutputBuilder::new()
//                 .with_address(
//                     &Address::from_bytes(
//                         hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c")
//                             .unwrap(),
//                     )
//                     .unwrap(),
//                 )
//                 .next()
//                 .unwrap()
//                 .with_coin(&to_bignum(1))
//                 .build()
//                 .unwrap(),
//         );
//         let mut body = TransactionBody::new(&inputs, &outputs, &to_bignum(266002), Some(10.into()));

//         let mut certs = Certificates::new();

//         let mut pool_owners = Ed25519KeyHashes::new();
//         pool_owners.add(
//             &PublicKey::from_bytes(
//                 &hex::decode("54d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f3")
//                     .unwrap(),
//             )
//             .unwrap()
//             .hash(),
//         );
//         let registration_cert = PoolRegistration::new(&PoolParams::new(
//             &PublicKey::from_bytes(
//                 &hex::decode("b24c040e65994bd5b0621a060166d32d356ef4be3cc1f848426a4cf386887089")
//                     .unwrap(),
//             )
//             .unwrap()
//             .hash(), // operator
//             &VRFKeyHash::from(blake2b256(
//                 &hex::decode("fbf6d41985670b9041c5bf362b5262cf34add5d265975de176d613ca05f37096")
//                     .unwrap(),
//             )), // vrf_keyhash
//             &to_bignum(1000000),                                // pledge
//             &to_bignum(1000000),                                // cost
//             &UnitInterval::new(&to_bignum(3), &to_bignum(100)), // margin
//             &RewardAddress::new(
//                 network,
//                 &StakeCredential::from_keyhash(
//                     &PublicKey::from_bytes(
//                         &hex::decode(
//                             "54d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f3",
//                         )
//                         .unwrap(),
//                     )
//                     .unwrap()
//                     .hash(),
//                 ),
//             ), // reward_address
//             &pool_owners,                                       // pool_owners
//             &Relays::new(),                                     // relays
//             None,                                               // metadata
//         ));
//         certs.add(&Certificate::new_pool_registration(&registration_cert));
//         body.set_certs(&certs);

//         let mut w = TransactionWitnessSet::new();
//         let mut vkw = Vkeywitnesses::new();
//         // input key witness
//         vkw.add(&make_vkey_witness(
//             &hash_transaction(&body),
//             &PrivateKey::from_normal_bytes(
//                 &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
//                     .unwrap(),
//             )
//             .unwrap(),
//         ));
//         // operator key witness
//         vkw.add(&make_vkey_witness(
//             &hash_transaction(&body),
//             &PrivateKey::from_normal_bytes(
//                 &hex::decode("2363f3660b9f3b41685665bf10632272e2d03c258e8a5323436f0f3406293505")
//                     .unwrap(),
//             )
//             .unwrap(),
//         ));
//         // owner key witness
//         vkw.add(&make_vkey_witness(
//             &hash_transaction(&body),
//             &PrivateKey::from_normal_bytes(
//                 &hex::decode("5ada7f4d92bce1ee1707c0a0e211eb7941287356e6ed0e76843806e307b07c8d")
//                     .unwrap(),
//             )
//             .unwrap(),
//         ));
//         w.set_vkeys(&vkw);

//         let signed_tx = Transaction::new(&body, &w, None);

//         let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
//         assert_eq!(
//             hex::encode(signed_tx.to_bytes()),
//             "84a500818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7000181a200581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c0101021a00040f12030a04818a03581c1c13374874c68016df54b1339b6cacdd801098431e7659b24928efc15820bd0000f498ccacdc917c28274cba51c415f3f21931ff41ca8dc1197499f8e1241a000f42401a000f4240d81e82031864581de151df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af7081581c51df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af7080f6a10083825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee5840415b1b64e77b55c5956c57041aef1671930aa778f91795147cb7aed4d8495a148d206939b93b139684431098ed939bfe919b4c8d59216a50a89ee6cffa9c3308825820b24c040e65994bd5b0621a060166d32d356ef4be3cc1f848426a4cf38688708958403df1c3597fe94c1b22c5f5993a25c9c4055313403b8283efd8545c5bef50378ef3d9e6a05b99fd44651a3d6f284a30c2482748db2d95389b15169d7bd574070882582054d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f35840931c34d472260e81b31f109311077f871fb48f74a76077004c9c724c8cfe64afd731c78e6f056d2687c33ff77d7116bf6fe9855ead6afdc0954358ec7d015c0ef5f6"
//         );
//         assert_eq!(
//             min_fee(&signed_tx, &linear_fee, &exunits_constants())
//                 .unwrap()
//                 .to_str(),
//             "270502" // todo: compare to Haskell fee to make sure the diff is not too big
//         );
//     }

//     // #[test]
//     // fn tx_delegate_stake() {
//     //     let mut inputs = TransactionInputs::new();
//     //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
//     //     let mut outputs = TransactionOutputs::new();
//     //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
//     //     let mut body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
//     //     let mut certs = Certificates::new();
//     //     certs.add(&Certificate::new_stake_delegation(&StakeDelegation::new(&bob_stake(), &alice_pool())));
//     //     body.set_certs(&certs);
//     //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &bob_key()]);
//     //     let tx = Transaction::new(&body, &w, None);
//     //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN * 2;
//     //     let our_crypto_bytes = witness_vkey_bytes_rust(&w) + Ed25519KeyHash::BYTE_COUNT + Ed25519KeyHash::BYTE_COUNT;
//     //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 178);
//     // }

//     // #[test]
//     // fn tx_deregister_stake() {
//     //     let mut inputs = TransactionInputs::new();
//     //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
//     //     let mut outputs = TransactionOutputs::new();
//     //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
//     //     let mut body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
//     //     let mut certs = Certificates::new();
//     //     certs.add(&Certificate::new_stake_deregistration(&StakeDeregistration::new(&alice_pay())));
//     //     body.set_certs(&certs);
//     //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
//     //     let tx = Transaction::new(&body, &w, None);
//     //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
//     //     let our_crypto_bytes = witness_vkey_bytes_rust(&w) + Ed25519KeyHash::BYTE_COUNT;
//     //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 150);
//     // }

//     // #[test]
//     // fn tx_register_pool() {
//     //     let mut inputs = TransactionInputs::new();
//     //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
//     //     let mut outputs = TransactionOutputs::new();
//     //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
//     //     let mut body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
//     //     let mut certs = Certificates::new();
//     //     let mut owners = Ed25519KeyHashes::new();
//     //     owners.add(&(alice_stake().to_keyhash().unwrap()));
//     //     let mut relays = Relays::new();
//     //     relays.add(&Relay::new_single_host_name(&SingleHostName::new(None, String::from("relay.io"))));
//     //     let params = PoolParams::new(
//     //         &alice_pool(),
//     //         &VRFKeyHash::from([0u8; VRFKeyHash::BYTE_COUNT]),
//     //         to_bignum(1),
//     //         to_bignum(5),
//     //         &UnitInterval::new(to_bignum(1), to_bignum(10)),
//     //         &RewardAddress::new(NetworkInfo::testnet().network_id(), &alice_stake()),
//     //         &owners,
//     //         &relays,
//     //         Some(PoolMetadata::new(String::from("alice.pool"), &MetadataHash::from([0u8; MetadataHash::BYTE_COUNT])))
//     //     );
//     //     certs.add(&Certificate::new_pool_registration(&PoolRegistration::new(&params)));
//     //     body.set_certs(&certs);
//     //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
//     //     let tx = Transaction::new(&body, &w, None);
//     //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w)
//     //         + HASKELL_HLEN // operator pool keyhash
//     //         + HASKELL_HLEN // vrf keyhash
//     //         + HASKELL_HLEN // reward account
//     //         + owners.len() * HASKELL_HLEN // owners' keyhashes
//     //         + HASKELL_HLEN; // metadata hash
//     //     let our_crypto_bytes = witness_vkey_bytes_rust(&w)
//     //         + Ed25519KeyHash::BYTE_COUNT
//     //         + VRFKeyHash::BYTE_COUNT
//     //         + Ed25519KeyHash::BYTE_COUNT
//     //         + owners.len() * Ed25519KeyHash::BYTE_COUNT
//     //         + MetadataHash::BYTE_COUNT;
//     //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 200);
//     // }

//     // #[test]
//     // fn tx_retire_pool() {
//     //     let mut inputs = TransactionInputs::new();
//     //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
//     //     let mut outputs = TransactionOutputs::new();
//     //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
//     //     let mut body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
//     //     let mut certs = Certificates::new();
//     //     certs.add(&Certificate::new_pool_retirement(&PoolRetirement::new(&alice_pool(), 5)));
//     //     body.set_certs(&certs);
//     //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
//     //     let tx = Transaction::new(&body, &w, None);
//     //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
//     //     let our_crypto_bytes = witness_vkey_bytes_rust(&w) + Ed25519KeyHash::BYTE_COUNT;
//     //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 149);
//     // }

//     // #[test]
//     // fn tx_metadata() {
//     //     let mut inputs = TransactionInputs::new();
//     //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
//     //     let mut outputs = TransactionOutputs::new();
//     //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
//     //     let mut body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
//     //     body.set_metadata_hash(&MetadataHash::from([37; MetadataHash::BYTE_COUNT]));
//     //     let w = make_mock_witnesses_vkey(&body, vec![&alice_key()]);
//     //     let mut metadata = TransactionMetadata::new();
//     //     let mut md_list = TransactionMetadatums::new();
//     //     md_list.add(&TransactionMetadatum::new_int(&Int::new(&to_bignum(5))));
//     //     md_list.add(&TransactionMetadatum::new_text(String::from("hello")));
//     //     metadata.insert(TransactionMetadatumLabel::new(0), &TransactionMetadatum::new_arr_transaction_metadatum(&md_list));
//     //     let tx = Transaction::new(&body, &w, Some(metadata));
//     //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w) + HASKELL_HLEN;
//     //     let our_crypto_bytes = witness_vkey_bytes_rust(&w) + MetadataHash::BYTE_COUNT;
//     //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes >= 154);
//     // }

//     // #[test]
//     // fn tx_multisig() {
//     //     let mut inputs = TransactionInputs::new();
//     //     inputs.add(&TransactionInput::new(&genesis_id(), 0));
//     //     let mut outputs = TransactionOutputs::new();
//     //     outputs.add(&TransactionOutput::new(&alice_addr(), to_bignum(10)));
//     //     let body = TransactionBody::new(&inputs, &outputs, to_bignum(94), 10);
//     //     let mut w = make_mock_witnesses_vkey(&body, vec![&alice_key(), &bob_key()]);
//     //     let mut script_witnesses = MultisigScripts::new();
//     //     let mut inner_scripts = MultisigScripts::new();
//     //     inner_scripts.add(&MultisigScript::new_msig_pubkey(&alice_pay().to_keyhash().unwrap()));
//     //     inner_scripts.add(&MultisigScript::new_msig_pubkey(&bob_pay().to_keyhash().unwrap()));
//     //     inner_scripts.add(&MultisigScript::new_msig_pubkey(&carl_pay().to_keyhash().unwrap()));
//     //     script_witnesses.add(&MultisigScript::new_msig_n_of_k(2, &inner_scripts));
//     //     w.set_scripts(&script_witnesses);
//     //     let tx = Transaction::new(&body, &w, None);
//     //     let haskell_crypto_bytes = witness_vkey_bytes_haskell(&w);
//     //     let our_crypto_bytes = witness_vkey_bytes_rust(&w);
//     //     assert!(txsize(&tx) - our_crypto_bytes + haskell_crypto_bytes - haskell_multisig_byte_diff(&script_witnesses) >= 189);
//     // }

//     #[test]
//     fn tx_withdrawal() {
//         // # Vector #8: with reward withdrawal
//         let mut inputs = TransactionInputs::new();
//         inputs.add(&TransactionInput::new(
//             &TransactionHash::from_bytes(
//                 hex::decode("3b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7")
//                     .unwrap(),
//             )
//             .unwrap(),
//             &0.into(),
//         ));
//         let mut outputs = TransactionOutputs::new();

//         outputs.add(
//             &TransactionOutputBuilder::new()
//                 .with_address(
//                     &Address::from_bytes(
//                         hex::decode("611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c")
//                             .unwrap(),
//                     )
//                     .unwrap(),
//                 )
//                 .next()
//                 .unwrap()
//                 .with_coin(&to_bignum(1))
//                 .build()
//                 .unwrap(),
//         );
//         let mut body = TransactionBody::new(&inputs, &outputs, &to_bignum(162502), Some(10.into()));
//         let mut withdrawals = Withdrawals::new();
//         withdrawals.insert(
//             &RewardAddress::from_address(
//                 &Address::from_bytes(
//                     hex::decode("e151df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af70")
//                         .unwrap(),
//                 )
//                 .unwrap(),
//             )
//             .unwrap(),
//             &to_bignum(1337),
//         );
//         body.set_withdrawals(&withdrawals);

//         let mut w = TransactionWitnessSet::new();
//         let mut vkw = Vkeywitnesses::new();
//         // input key witness
//         vkw.add(&make_vkey_witness(
//             &hash_transaction(&body),
//             &PrivateKey::from_normal_bytes(
//                 &hex::decode("c660e50315d76a53d80732efda7630cae8885dfb85c46378684b3c6103e1284a")
//                     .unwrap(),
//             )
//             .unwrap(),
//         ));
//         // withdrawal key witness
//         vkw.add(&make_vkey_witness(
//             &hash_transaction(&body),
//             &PrivateKey::from_normal_bytes(
//                 &hex::decode("5ada7f4d92bce1ee1707c0a0e211eb7941287356e6ed0e76843806e307b07c8d")
//                     .unwrap(),
//             )
//             .unwrap(),
//         ));
//         w.set_vkeys(&vkw);

//         let signed_tx = Transaction::new(&body, &w, None);

//         let linear_fee = LinearFee::new(&to_bignum(500), &to_bignum(2));
//         assert_eq!(
//             hex::encode(signed_tx.to_bytes()),
//             "84a500818258203b40265111d8bb3c3c608d95b3a0bf83461ace32d79336579a1939b3aad1c0b7000181a200581d611c616f1acb460668a9b2f123c80372c2adad3583b9c6cd2b1deeed1c0101021a00027ac6030a05a1581de151df9ba1b74a1c9608a487e114184556801e927d31d96425cb80af70190539a10082825820f9aa3fccb7fe539e471188ccc9ee65514c5961c070b06ca185962484a4813bee5840e56f10cf4c00aab3712a7f7d1994c76790ad99cdef4a547df42dc85d48653bbc2ff76bf83fe467ba9d64696e418051e35414ac7fb9e4f0cc11fd0ce252a2bc0a82582054d1a9c5ad69586ceeb839c438400c376c0bd34825fb4c17cc2f58c54e1437f358400c1f98fddc091aa0e18ae2c278e921e359cc9dd28f723330cd17026c721b0e6e2644b6519303c575f4d012d71022e4cdffd63f7b5d6ee4e2ea12e44f1a0b1e04f5f6"
//         );
//         assert_eq!(
//             min_fee(&signed_tx, &linear_fee, &exunits_constants())
//                 .unwrap()
//                 .to_str(),
//             "164002" // todo: compare to Haskell fee to make sure the diff is not too big
//         );
//     }
// }