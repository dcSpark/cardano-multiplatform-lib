// Seeded by cddl-codegen on first export; never overwritten after that.
// All regenerated code lives in the `generated` module. Add your own
// modules/re-exports/attrs here freely (e.g. `pub mod utils;`).
mod generated;
pub use generated::*;

pub mod error;
pub mod utils;

// Convenience re-exports (not referenced by the generated code itself).
pub use cml_chain::{NetworkId, auxdata::Metadata};

// Extern-type re-export so the generated glue (`pub use crate::PaymentAddress;`) resolves.
pub use cml_chain::address::Address as PaymentAddress;

#[cfg(test)]
mod tests {
    use cml_chain::address::Address;
    use cml_crypto::*;

    use super::*;

    #[test]
    fn sign_data() {
        // TODO: test vectors don't fully specify how to arrive from these to the resulting delegations/stake creds/reward address
        // so we don't derive or anything and just put them straight into the structs but that would be a more complete test
        // and we might want to add deriving-related options (assuming these keys weren't in their already-derived state)
        // let payment_pub = PublicKey::from_raw_hex("3273a5316e4de228863bd7cf8dac90d57149e1a595f3dd131073b84e35546676").unwrap();
        // let staking_prv = Bip32PrivateKey::from_raw_hex("f5beaeff7932a4164d270afde7716067582412e8977e67986cd9b456fc082e3a").unwrap();
        // let staking_derived_prv = staking_prv
        //     .derive(1694)
        //     .derive(1815)
        //     .derive(0)
        //     .derive(0);
        // let catalyst_prv_key = hex::decode("4820f7ce221e177c8eae2b2ee5c1f1581a0d88ca5c14329d8f2389e77a465655c27662621bfb99cb9445bf8114cc2a630afd2dd53bc88c08c5f2aed8e9c7cb89").unwrap();
        // test case says: 0036ef3e1f0d3f5989e2d155ea54bdb2a72c4c456ccb959af4c94868f473f5a0
        // but the CBOR differs, so we use the ones in the CBOR for the test case.
        // Is this being derived or why does it not match?
        let stake_cred = PublicKey::from_raw_bytes(&[
            227, 205, 36, 4, 200, 77, 230, 95, 150, 145, 143, 24, 213, 180, 69, 188, 185, 51, 167,
            205, 161, 142, 237, 237, 121, 69, 221, 25, 30, 67, 35, 105,
        ])
        .unwrap();
        // let stake_cred = StakeCredential::from(staking_derived_prv.to_public().to_raw_key());
        let nonce = 1234;

        // legacy cip-15 format
        // test case says: addr_test1qprhw4s70k0vzyhvxp6h97hvrtlkrlcvlmtgmaxdtjz87xrjkctk27ypuv9dzlzxusqse89naweygpjn5dxnygvus05sdq9h07
        // but the CBOR differs, so we use the ones in the CBOR for the test case.
        // Is this being derived or why does it not match?
        let legacy_address = Address::from_raw_bytes(&[
            224, 114, 182, 23, 101, 120, 129, 227, 10, 209, 124, 70, 228, 1, 12, 156, 179, 235,
            178, 68, 6, 83, 163, 77, 50, 33, 156, 131, 233,
        ])
        .unwrap();
        let legacy_reg = CIP36KeyRegistration::new(
            CIP36DelegationDistribution::new_legacy(
                CIP36LegacyKeyRegistration::from_raw_bytes(&[
                    0, 54, 239, 62, 31, 13, 63, 89, 137, 226, 209, 85, 234, 84, 189, 178, 167, 44,
                    76, 69, 108, 203, 149, 154, 244, 201, 72, 104, 244, 115, 245, 160,
                ])
                .unwrap(),
            ),
            stake_cred.clone(),
            legacy_address,
            nonce,
        );
        let legacy_sign_data_hash = legacy_reg.hash_to_sign(false).unwrap();
        assert_eq!(
            "9946e71b5f6c16150cf431910a0f7dbb8084a992577847802e60d32becb3d6be",
            hex::encode(legacy_sign_data_hash)
        );

        // cip-36 format
        // test case says: addr_test1qprhw4s70k0vzyhvxp6h97hvrtlkrlcvlmtgmaxdtjz87xrjkctk27ypuv9dzlzxusqse89naweygpjn5dxnygvus05sdq9h07
        // but the CBOR differs, so we use the ones in the CBOR for the test case.
        // Is this being derived or why does it not match? It doesn't match the legacy one either
        let new_address = Address::from_raw_bytes(&[
            0, 71, 119, 86, 30, 125, 158, 193, 18, 236, 48, 117, 114, 250, 236, 26, 255, 97, 255,
            12, 254, 214, 141, 244, 205, 92, 132, 127, 24, 114, 182, 23, 101, 120, 129, 227, 10,
            209, 124, 70, 228, 1, 12, 156, 179, 235, 178, 68, 6, 83, 163, 77, 50, 33, 156, 131,
            233,
        ])
        .unwrap();
        let weighted_reg = CIP36KeyRegistration::new(
            CIP36DelegationDistribution::new_weighted(cml_core::non_empty::NonEmptyVec::new(
                CIP36Delegation::new(
                    CIP36VotingPubKey::from_raw_bytes(&[
                        0, 54, 239, 62, 31, 13, 63, 89, 137, 226, 209, 85, 234, 84, 189, 178, 167,
                        44, 76, 69, 108, 203, 149, 154, 244, 201, 72, 104, 244, 115, 245, 160,
                    ])
                    .unwrap(),
                    1,
                ),
            )),
            stake_cred,
            new_address,
            nonce,
        );
        let weighted_sign_data_hash = weighted_reg.hash_to_sign(false).unwrap();
        assert_eq!(
            "3110fbad72589a80de7fc174310e92dac35bbfece1690c2dce53c2235a9776fa",
            hex::encode(weighted_sign_data_hash)
        );

        // TODO: deregistration test? there are no official test vectors in CIP36
    }
}
