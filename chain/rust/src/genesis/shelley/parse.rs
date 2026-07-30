use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use cml_core::DeserializeError;
use cml_crypto::{
    Ed25519KeyHash, PoolMetadataHash, TransactionHash, VRFKeyHash, chain_crypto::Blake2b256,
};
use core::str::FromStr;
use num::rational::Ratio;
use serde_json;

use crate::{
    SetEd25519KeyHash, UnitInterval,
    address::{Address, RewardAccount},
    block::ProtocolVersion,
    certs::{Ipv4, Ipv6, PoolMetadata, PoolParams, Relay, StakeCredential, Url},
};

use super::{
    config,
    raw::{self},
};

#[derive(Debug, thiserror::Error)]
pub enum GenesisJSONError {
    #[error("JSON: {0:?}")]
    Serde(#[from] serde_json::Error),
    #[error("Deserialize: {0:?}")]
    Deserialize(#[from] DeserializeError),
    #[error("ParseInt: {0:?}")]
    ParseInt(#[from] core::num::ParseIntError),
    #[error("ParseIP: {0:?}")]
    ParseIP(#[from] crate::certs::utils::IPStringParsingError),
    #[error("Unexpected network type: {0:?}")]
    ParseNetwork(String),
}

pub fn parse_genesis_data(json: &[u8]) -> Result<config::ShelleyGenesisData, GenesisJSONError> {
    let data: raw::ShelleyGenesisData = serde_json::from_slice(json)?;

    let mut initial_funds = BTreeMap::new();
    for (addr_hex, balance) in &data.initialFunds {
        initial_funds.insert(Address::from_hex(addr_hex)?, *balance);
    }

    let network_id = match data.networkId.as_str() {
        "Mainnet" => crate::NetworkId::mainnet().get(),
        "Testnet" => crate::NetworkId::testnet().get(),
        val => return Err(GenesisJSONError::ParseNetwork(val.to_string())),
    };

    let staking = match data.staking.as_ref() {
        Some(raw) => {
            // 1) Get stake pools
            let mut pools: BTreeMap<Ed25519KeyHash, PoolParams> = BTreeMap::new();
            for (pool_id, params) in &raw.pools {
                let ration = json_number_to_ratio(&params.margin);
                let mut owners = Vec::<Ed25519KeyHash>::new();
                for owner in &params.owners {
                    owners.push(Ed25519KeyHash::from_hex(owner)?);
                }
                let mut relays = Vec::<Relay>::new();
                for relay in &params.relays {
                    if let Some((key, value)) = relay.iter().next() {
                        match key.as_str() {
                            "single host address" => {
                                let ipv4 = match value.IPv4.as_ref() {
                                    Some(s) => Some(Ipv4::from_str(s)?),
                                    _ => None,
                                };
                                let ipv6 = match value.IPv6.as_ref() {
                                    Some(s) => Some(Ipv6::from_str(s)?),
                                    _ => None,
                                };
                                relays.push(Relay::new_single_host_addr(value.port, ipv4, ipv6));
                            }
                            _ => panic!(
                                "Only single host address relays are supported in cardano-node Relay JSON parsing"
                            ),
                        }
                    }
                }
                let pool_metadata = match params.metadata.as_ref() {
                    Some(metadata) => Some(PoolMetadata::new(
                        Url::new(metadata.url.clone()).unwrap(),
                        PoolMetadataHash::from_hex(&metadata.hash)?,
                    )),
                    _ => None,
                };
                let parsed_params = PoolParams::new(
                    Ed25519KeyHash::from_hex(&params.publicKey)?,
                    VRFKeyHash::from_hex(&params.vrf)?,
                    params.pledge,
                    params.cost,
                    UnitInterval::new(*ration.numer(), *ration.denom()),
                    RewardAccount::new(
                        match data.networkId.as_str() {
                            "Mainnet" => crate::NetworkId::mainnet().get() as u8,
                            "Testnet" => crate::NetworkId::testnet().get() as u8,
                            val => return Err(GenesisJSONError::ParseNetwork(val.to_string())),
                        },
                        StakeCredential::new_pub_key(Ed25519KeyHash::from_hex(
                            &params.rewardAccount.credential.keyHash,
                        )?),
                    ),
                    // pool owners are a set on-chain; duplicates in the genesis JSON are
                    // invalid and surface via the GenesisJSONError::Deserialize conversion
                    SetEd25519KeyHash::try_from(owners)?,
                    relays,
                    pool_metadata,
                );
                pools.insert(Ed25519KeyHash::from_hex(pool_id)?, parsed_params);
            }
            // 2) Get initial delegations
            let mut stake: BTreeMap<Ed25519KeyHash, Ed25519KeyHash> = BTreeMap::new();
            for (staking_key, pool_id) in &raw.stake {
                stake.insert(
                    Ed25519KeyHash::from_hex(staking_key)?,
                    Ed25519KeyHash::from_hex(pool_id)?,
                );
            }
            Some(config::ShelleyGenesisStaking { stake, pools })
        }
        _ => None,
    };

    let mut gen_delegs = BTreeMap::new();
    for (key, val) in data.genDelegs.iter() {
        gen_delegs.insert(
            Ed25519KeyHash::from_hex(key)?,
            config::ShelleyGenesisDelegations {
                delegate: Ed25519KeyHash::from_hex(&val.delegate)?,
                vrf: VRFKeyHash::from_hex(&val.vrf)?,
            },
        );
    }
    Ok(config::ShelleyGenesisData {
        active_slots_coeff: json_number_to_ratio(&data.activeSlotsCoeff),
        epoch_length: data.epochLength,
        gen_delegs,
        initial_funds,
        max_kes_evolutions: data.maxKESEvolutions,
        max_lovelace_supply: data.maxLovelaceSupply,
        network_id,
        network_magic: data.networkMagic,
        protocol_params: config::ShelleyGenesisProtocolParameters {
            a0: json_number_to_ratio(&data.protocolParams.a0),
            decentralisation_param: json_number_to_ratio(
                &data.protocolParams.decentralisationParam,
            ),
            e_max: data.protocolParams.eMax,
            extra_entropy: config::ShelleyGenesisExtraEntropy {
                tag: data.protocolParams.extraEntropy.tag,
            },
            key_deposit: data.protocolParams.keyDeposit,
            max_block_body_size: data.protocolParams.maxBlockBodySize,
            max_block_header_size: data.protocolParams.maxBlockHeaderSize,
            max_tx_size: data.protocolParams.maxTxSize,
            min_fee_a: data.protocolParams.minFeeA,
            min_fee_b: data.protocolParams.minFeeB,
            min_pool_cost: data.protocolParams.minPoolCost,
            min_utxo_value: data.protocolParams.minUTxOValue,
            n_opt: data.protocolParams.nOpt,
            pool_deposit: data.protocolParams.poolDeposit,
            protocol_version: ProtocolVersion::new(
                data.protocolParams.protocolVersion.major,
                data.protocolParams.protocolVersion.minor,
            ),
            rho: json_number_to_ratio(&data.protocolParams.rho),
            tau: json_number_to_ratio(&data.protocolParams.tau),
        },
        security_param: data.securityParam,
        slot_length: json_number_to_ratio(&data.slotLength),
        slots_per_kes_period: data.slotsPerKESPeriod,
        staking,
        system_start: data.systemStart.parse().expect("Failed to parse date"),
        update_quorum: data.updateQuorum,
    })
}

/// Lossless decimal/scientific JSON literal -> exact rational (e.g. "0.05" -> 1/20, "4.5e-2" ->
/// 9/200). serde_json's arbitrary_precision feature preserves the literal, so no value ever
/// round-trips through f64. Genesis rationals are never negative; a sign is rejected rather than
/// widened away.
fn json_number_to_ratio(param: &serde_json::Number) -> Ratio<u64> {
    let s = param.to_string();
    assert!(
        !s.starts_with('-'),
        "negative rational in genesis JSON: {s}"
    );

    let (mantissa, exp) = match s.find(['e', 'E']) {
        Some(exp_position) => {
            let (a, b) = s.split_at(exp_position);
            (a, i32::from_str(&b[1..]).unwrap())
        }
        None => (s.as_str(), 0i32),
    };

    let (numer, denom) = match mantissa.find('.') {
        Some(dot_position) => {
            let (int_part, frac_part) = (&mantissa[..dot_position], &mantissa[dot_position + 1..]);
            let scale = 10u64.checked_pow(frac_part.len() as u32).unwrap();
            let int = u64::from_str(int_part).unwrap();
            let frac = if frac_part.is_empty() {
                0
            } else {
                u64::from_str(frac_part).unwrap()
            };
            (
                int.checked_mul(scale).unwrap().checked_add(frac).unwrap(),
                scale,
            )
        }
        None => (u64::from_str(mantissa).unwrap(), 1),
    };

    let ratio = Ratio::new(numer, denom);
    match exp {
        0 => ratio,
        e if e > 0 => ratio * Ratio::from_integer(10u64.checked_pow(e as u32).unwrap()),
        e => ratio / Ratio::from_integer(10u64.checked_pow(e.unsigned_abs()).unwrap()),
    }
}

pub fn redeem_address_to_txid(pubkey: &Address) -> TransactionHash {
    let txid = Blake2b256::new(&pubkey.to_raw_bytes());
    TransactionHash::from(*txid.as_hash_bytes())
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_genesis_data() -> &'static str {
        include_str!("./test_data/test.json")
    }

    fn get_test_genesis_data_yaci() -> &'static str {
        include_str!("./test_data/test-yaci.json")
    }

    #[test]
    fn calc_address_txid() {
        let hash = redeem_address_to_txid(
            &Address::from_bech32("addr_test1qpefp65049pncyz95nyyww2e44sgumqr5kx8mcemm0fuumeftwv8zdtpqct0836wz8y56aakem2uejf604cee7cn2p3qp9p8te").unwrap(),
        );
        assert_eq!(
            hash.to_hex(),
            "66dc6b2e628bf1fb6204797f1a07f8e949d9520a70e859ecbf3ea3076029871e"
        );
    }

    #[test]
    fn parse_test_genesis_files() {
        let genesis_data = super::parse_genesis_data(get_test_genesis_data().as_bytes()).unwrap();

        assert_eq!(genesis_data.epoch_length, 432000u64);
        assert_eq!(genesis_data.network_id, 0);
        assert_eq!(genesis_data.network_magic, 764824073u64);

        assert_eq!(
            *genesis_data
                .initial_funds
                .iter()
                .find(|(n, _)| n.to_hex()
                    == "605276322ac7882434173dcc6441905f6737689bd309b68ad8b3614fd8")
                .unwrap()
                .1,
            3000000000000000u64
        );
    }

    #[test]
    fn parse_test_genesis_yaci_files() {
        super::parse_genesis_data(get_test_genesis_data_yaci().as_bytes()).unwrap();
    }

    fn number_to_ratio(s: &str) -> Ratio<u64> {
        json_number_to_ratio(&serde_json::Number::from_str(s).unwrap())
    }

    #[test]
    fn json_number_to_ratio_forms() {
        assert_eq!(number_to_ratio("0.05"), Ratio::new(1, 20));
        assert_eq!(number_to_ratio("0.45"), Ratio::new(9, 20));
        assert_eq!(number_to_ratio("4.5e-2"), Ratio::new(9, 200));
        assert_eq!(number_to_ratio("4.5E-2"), Ratio::new(9, 200));
        assert_eq!(number_to_ratio("20"), Ratio::from_integer(20));
        assert_eq!(number_to_ratio("1e2"), Ratio::from_integer(100));
        assert_eq!(number_to_ratio("2.5e2"), Ratio::from_integer(250));
        assert_eq!(number_to_ratio("0.1"), Ratio::new(1, 10));
        assert_eq!(number_to_ratio("0"), Ratio::from_integer(0));
        assert_eq!(number_to_ratio("0.259"), Ratio::new(259, 1000));
    }

    #[test]
    #[should_panic(expected = "negative rational")]
    fn json_number_to_ratio_rejects_negative() {
        number_to_ratio("-0.5");
    }
}
