use cbor_event::cbor;
use serde_json;
use std::collections::BTreeMap;
use std::io::Read;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

use crate::byron::{ProtocolMagic, StakeholderId, ByronAddress, AddressContent};
use crate::chain_crypto::byron_proxy_key::ProxySecretKey;
use crate::chain_crypto::{Ed25519, self, Ed25519Bip32, Signature, Blake2b256};
use crate::crypto::{BlockHeaderHash, blake2b256, TransactionHash, self, Bip32PublicKey};
use crate::ledger::alonzo::fees::LinearFee;
use crate::ledger::common::value::{Coin, BigNum};

use super::{raw, config};

pub fn parse<R: Read>(json: R) -> config::GenesisData {
    // FIXME: use Result

    let data_value: serde_json::Value = serde_json::from_reader(json).unwrap();
    let genesis_prev = BlockHeaderHash::from_bytes(
        blake2b256(data_value.to_string().as_bytes()).to_vec()
    ).unwrap();
    let data: raw::GenesisData = serde_json::from_value(data_value.clone()).unwrap();

    let protocol_magic = ProtocolMagic::from(data.protocolConsts.protocolMagic);

    let parse_fee_constant = |s: &str| -> BigNum {
        let n = s.parse::<u64>().unwrap();
        BigNum::from(n)
    };

    let mut avvm_distr = BTreeMap::new();
    for (avvm, balance) in &data.avvmDistr {
        avvm_distr.insert(
            chain_crypto::PublicKey::<Ed25519>::from_binary(&base64::decode_config(avvm, base64::URL_SAFE).unwrap())
                .unwrap(),
            Coin::from(balance.parse::<u64>().unwrap()),
        );
    }

    let slot_duration = {
        let v = data.blockVersionData.slotDuration.parse::<u64>().unwrap();
        Duration::from_millis(v)
    };

    let start_time = {
        let unix_displacement = Duration::from_secs(data.startTime);
        SystemTime::UNIX_EPOCH + unix_displacement
    };

    let mut non_avvm_balances = BTreeMap::new();
    for (address, balance) in &data.nonAvvmBalances {
        non_avvm_balances.insert(
            ByronAddress::from_str(address).unwrap(),
            Coin::from(balance.parse::<u64>().unwrap()),
        );
    }

    let mut boot_stakeholders = BTreeMap::new();

    for (stakeholder_id, weight) in &data.bootStakeholders {
        let heavy = data.heavyDelegation.get(stakeholder_id).unwrap();

        let stakeholder_id = StakeholderId::from_hex(stakeholder_id).unwrap();

        let psk = ProxySecretKey {
            omega: 0,
            issuer_pk: chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&base64::decode(&heavy.issuerPk).unwrap())
                .unwrap(),
            delegate_pk: chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&base64::decode(&heavy.delegatePk).unwrap())
                .unwrap(),
            cert: Signature::<(), Ed25519Bip32>::from_str(&heavy.cert).unwrap(),
        };

        // Check that the stakeholder ID corresponds to the issuer public key.
        assert_eq!(stakeholder_id, StakeholderId::new(&Bip32PublicKey(psk.issuer_pk.clone())));

        // Check that the certificate is correct.
        assert!(psk.verify(protocol_magic));

        boot_stakeholders.insert(
            stakeholder_id,
            config::BootStakeholder {
                weight: *weight,
                issuer_pk: psk.issuer_pk,
                delegate_pk: psk.delegate_pk,
                cert: psk.cert,
            },
        );
    }

    config::GenesisData {
        genesis_prev,
        epoch_stability_depth: data.protocolConsts.k,
        protocol_magic,
        fee_policy: LinearFee::new(
            &parse_fee_constant(&data.blockVersionData.txFeePolicy.multiplier),
            &parse_fee_constant(&data.blockVersionData.txFeePolicy.summand),
        ),
        avvm_distr,
        non_avvm_balances,
        start_time,
        slot_duration,
        boot_stakeholders,
    }
}

pub fn canonicalize_json<R: Read>(json: R) -> String {
    let data: serde_json::Value = serde_json::from_reader(json).unwrap();
    data.to_string()
}

pub fn redeem_pubkey_to_txid(
    pubkey: &chain_crypto::PublicKey<Ed25519>,
    protocol_magic: Option<ProtocolMagic>,
) -> (TransactionHash, AddressContent) {
    let address = AddressContent::new_redeem(&crypto::PublicKey(pubkey.clone()), protocol_magic);
    let txid = Blake2b256::new(&cbor!(&address).unwrap());
    (TransactionHash(*txid.as_hash_bytes()), address)
}


#[cfg(test)]
mod test {

    use crate::{crypto::BlockHeaderHash};

    use super::*;

    #[test]
    pub fn parse_test_genesis_files() {
        let genesis_hash = BlockHeaderHash::from_hex(
            &"c6a004d3d178f600cd8caa10abbebe1549bef878f0665aea2903472d5abf7323",
        )
        .unwrap();

        let genesis_data = super::parse(
            super::super::data::get_test_genesis_data(&genesis_hash)
                .unwrap()
                .as_bytes(),
        );

        assert_eq!(genesis_data.epoch_stability_depth, 2160);
        assert_eq!(
            genesis_data
                .start_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            1506450213
        );
        assert_eq!(genesis_data.slot_duration.as_secs(), 20);
        assert_eq!(genesis_data.slot_duration.subsec_millis(), 0);
        assert_eq!(genesis_data.protocol_magic, 633343913.into());
        assert_eq!(u64::from(genesis_data.fee_policy.coefficient), 43946 * 1_000_000u64);
        assert_eq!(u64::from(genesis_data.fee_policy.constant), 155381 * 1_000_000_000u64);

        assert_eq!(
            base64::encode_config(
                genesis_data
                    .avvm_distr
                    .iter()
                    .find(|(_, v)| **v == Coin::from(9999300000000))
                    .unwrap()
                    .0,
                base64::URL_SAFE
            ),
            "-0BJDi-gauylk4LptQTgjMeo7kY9lTCbZv12vwOSTZk="
        );

        let genesis_hash = BlockHeaderHash::from_hex(
            &"b7f76950bc4866423538ab7764fc1c7020b24a5f717a5bee3109ff2796567214",
        )
        .unwrap();

        let genesis_data = super::parse(
            super::super::data::get_test_genesis_data(&genesis_hash)
                .unwrap()
                .as_bytes(),
        );

        assert_eq!(
            genesis_data
                .non_avvm_balances
                .iter()
                .find(|(n, _)| n.to_string()
                    == "2cWKMJemoBaheSTiK9XEtQDf47Z3My8jwN25o5jjm7s7jaXin2nothhWQrTDd8m433M8K")
                .unwrap()
                .1,
            &Coin::from(5428571428571429)
        );
    }
}
