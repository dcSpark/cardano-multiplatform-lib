use cml_core::DeserializeError;
use cml_crypto::{
    chain_crypto::Blake2b256, Ed25519KeyHash, PoolMetadataHash, TransactionHash, VRFKeyHash,
};
use serde_json;
use std::collections::BTreeMap;
use std::io::Read;
use std::str::FromStr;

use crate::{
    address::{Address, RewardAccount},
    block::ProtocolVersion,
    certs::{Ipv4, Ipv6, PoolMetadata, PoolParams, Relay, StakeCredential, Url},
    UnitInterval,
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
    ParseInt(#[from] std::num::ParseIntError),
    #[error("ParseIP: {0:?}")]
    ParseIP(#[from] crate::certs::utils::IPStringParsingError),
    #[error("Unexpected network type: {0:?}")]
    ParseNetwork(String),
}

pub fn parse_genesis_data<R: Read>(
    json: R,
) -> Result<config::ShelleyGenesisData, GenesisJSONError> {
    let data_value: serde_json::Value = serde_json::from_reader(json)?;
    let data: raw::ShelleyGenesisData = serde_json::from_value(data_value)?;

    let mut initial_funds = BTreeMap::new();
    for (addr_hex, balance) in &data.initialFunds {
        initial_funds.insert(Address::from_hex(addr_hex)?, *balance);
    }

    let network_id = match data.networkId.as_str() {
        "Mainnet" => crate::NetworkId::mainnet().network,
        "Testnet" => crate::NetworkId::testnet().network,
        val => return Err(GenesisJSONError::ParseNetwork(val.to_string())),
    };

    let staking = match data.staking.as_ref() {
        Some(raw) => {
            // 1) Get stake pools
            let mut pools: BTreeMap<Ed25519KeyHash, PoolParams> = BTreeMap::new();
            for (pool_id, params) in &raw.pools {
                let ration = fraction::Fraction::from_str(&params.margin).unwrap();
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
                                    _ => None
                                };
                                let ipv6 = match value.IPv6.as_ref() {
                                    Some(s) => Some(Ipv6::from_str(s)?),
                                    _ => None
                                };
                                relays.push(Relay::new_single_host_addr(
                                    value.port,
                                    ipv4,
                                    ipv6
                                ));
                            },
                            _ => panic!("Only single host address relays are supported in cardano-node Relay JSON parsing")
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
                    UnitInterval::new(*ration.numer().unwrap(), *ration.denom().unwrap()),
                    RewardAccount::new(
                        match data.networkId.as_str() {
                            "Mainnet" => crate::NetworkId::mainnet().network as u8,
                            "Testnet" => crate::NetworkId::testnet().network as u8,
                            val => return Err(GenesisJSONError::ParseNetwork(val.to_string())),
                        },
                        StakeCredential::new_pub_key(Ed25519KeyHash::from_hex(
                            &params.rewardAccount.credential.keyHash,
                        )?),
                    ),
                    owners.into(),
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
        active_slots_coeff: fraction::Fraction::from_str(&data.activeSlotsCoeff).unwrap(),
        epoch_length: data.epochLength,
        gen_delegs,
        initial_funds,
        max_kes_evolutions: data.maxKESEvolutions,
        max_lovelace_supply: data.maxLovelaceSupply,
        network_id,
        network_magic: data.networkMagic,
        protocol_params: config::ShelleyGenesisProtocolParameters {
            a0: fraction::Fraction::from_str(&data.protocolParams.a0).unwrap(),
            decentralisation_param: fraction::Fraction::from_str(
                &data.protocolParams.decentralisationParam,
            )
            .unwrap(),
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
            rho: fraction::Fraction::from_str(&data.protocolParams.rho).unwrap(),
            tau: fraction::Fraction::from_str(&data.protocolParams.tau).unwrap(),
        },
        security_param: data.securityParam,
        slot_length: data.slotLength,
        slots_per_kes_period: data.slotsPerKESPeriod,
        staking,
        system_start: data.systemStart.parse().expect("Failed to parse date"),
        update_quorum: data.updateQuorum,
    })
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
}
