use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine,
};
use cbor_event::cbor;
use cml_core::DeserializeError;
use cml_crypto::{CryptoError, RawBytesEncoding};
use serde_json;
use std::collections::BTreeMap;
use std::io::Read;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

use crate::byron::{
    AddressContent, ByronAddress, ParseExtendedAddrError, ProtocolMagic, StakeholderId,
};
use crate::crypto::{BlockHeaderHash, TransactionHash};
use crate::fees::LinearFee;
use cml_crypto::chain_crypto::byron_proxy_key::ByronProxySecretKey;
use cml_crypto::chain_crypto::{
    self, Blake2b256, Ed25519, Ed25519Bip32, Signature, SignatureFromStrError,
};
use cml_crypto::{blake2b256, Bip32PublicKey};

use super::{config, raw};

#[derive(Debug, thiserror::Error)]
pub enum GenesisJSONError {
    #[error("JSON: {0:?}")]
    Serde(#[from] serde_json::Error),
    #[error("Crypto: {0:?}")]
    CryptoError(#[from] CryptoError),
    #[error("Deserialize: {0:?}")]
    DeserializeError(#[from] DeserializeError),
    #[error("Base64: {0:?}")]
    Base64(#[from] base64::DecodeError),
    #[error("ParseInt: {0:?}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("ByronAddress: {0:?}")]
    ByronAddress(#[from] ParseExtendedAddrError),
    #[error("SignatureParse: {0:?}")]
    SignatureParse(#[from] SignatureFromStrError),
    #[error("Stakeholder not found: {0}")]
    StakeholderMissing(String),
}

pub fn parse_genesis_data<R: Read>(json: R) -> Result<config::GenesisData, GenesisJSONError> {
    let data_value: serde_json::Value = serde_json::from_reader(json)?;
    let genesis_prev =
        BlockHeaderHash::from_raw_bytes(&blake2b256(data_value.to_string().as_bytes()))?;
    let data: raw::GenesisData = serde_json::from_value(data_value.clone())?;

    let protocol_magic = ProtocolMagic::from(data.protocolConsts.protocolMagic);

    let parse_fee_constant = |s: &str| s.parse::<u64>();

    let mut avvm_distr = BTreeMap::new();
    for (avvm, balance) in &data.avvmDistr {
        avvm_distr.insert(
            chain_crypto::PublicKey::<Ed25519>::from_binary(&URL_SAFE.decode(avvm)?)
                .map_err(CryptoError::from)?,
            balance.parse::<u64>()?,
        );
    }

    let slot_duration = {
        let v = data.blockVersionData.slotDuration.parse::<u64>()?;
        Duration::from_millis(v)
    };

    let start_time = {
        let unix_displacement = Duration::from_secs(data.startTime);
        SystemTime::UNIX_EPOCH + unix_displacement
    };

    let mut non_avvm_balances = BTreeMap::new();
    for (address, balance) in &data.nonAvvmBalances {
        non_avvm_balances.insert(ByronAddress::from_str(address)?, balance.parse::<u64>()?);
    }

    let mut boot_stakeholders = BTreeMap::new();

    for (stakeholder_id, weight) in &data.bootStakeholders {
        let heavy = data
            .heavyDelegation
            .get(stakeholder_id)
            .ok_or_else(|| GenesisJSONError::StakeholderMissing(stakeholder_id.clone()))?;

        let stakeholder_id = StakeholderId::from_hex(stakeholder_id)?;

        let psk = ByronProxySecretKey {
            omega: 0,
            issuer_pk: chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(
                &STANDARD.decode(&heavy.issuerPk)?,
            )
            .map_err(CryptoError::from)?,
            delegate_pk: chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(
                &STANDARD.decode(&heavy.delegatePk)?,
            )
            .map_err(CryptoError::from)?,
            cert: Signature::<(), Ed25519Bip32>::from_str(&heavy.cert)?,
        };

        // Check that the stakeholder ID corresponds to the issuer public key.
        assert_eq!(
            stakeholder_id,
            StakeholderId::new(&Bip32PublicKey(psk.issuer_pk.clone()))
        );

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

    Ok(config::GenesisData {
        genesis_prev,
        epoch_stability_depth: data.protocolConsts.k,
        protocol_magic,
        fee_policy: LinearFee::new(
            parse_fee_constant(&data.blockVersionData.txFeePolicy.multiplier)?,
            parse_fee_constant(&data.blockVersionData.txFeePolicy.summand)?,
            0,
        ),
        avvm_distr,
        non_avvm_balances,
        start_time,
        slot_duration,
        boot_stakeholders,
    })
}

pub fn canonicalize_json<R: Read>(json: R) -> Result<String, GenesisJSONError> {
    let data: serde_json::Value = serde_json::from_reader(json)?;
    Ok(data.to_string())
}

pub fn redeem_pubkey_to_txid(
    pubkey: &chain_crypto::PublicKey<Ed25519>,
    protocol_magic: Option<ProtocolMagic>,
) -> (TransactionHash, ByronAddress) {
    let address_content =
        AddressContent::new_redeem(cml_crypto::PublicKey(pubkey.clone()), protocol_magic);
    let byron_address = address_content.to_address();
    let txid = Blake2b256::new(&cbor!(&byron_address).unwrap());
    (TransactionHash::from(*txid.as_hash_bytes()), byron_address)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::crypto::BlockHeaderHash;

    fn get_test_genesis_data(genesis_prev: &BlockHeaderHash) -> Result<&str, BlockHeaderHash> {
        if genesis_prev
            == &BlockHeaderHash::from_hex(
                "5f20df933584822601f9e3f8c024eb5eb252fe8cefb24d1317dc3d432e940ebb",
            )
            .unwrap()
        {
            Ok(include_str!(
                "./test_data/5f20df933584822601f9e3f8c024eb5eb252fe8cefb24d1317dc3d432e940ebb.json"
            ))
        } else if genesis_prev
            == &BlockHeaderHash::from_hex(
                "b7f76950bc4866423538ab7764fc1c7020b24a5f717a5bee3109ff2796567214",
            )
            .unwrap()
        {
            Ok(include_str!(
                "./test_data/b7f76950bc4866423538ab7764fc1c7020b24a5f717a5bee3109ff2796567214.json"
            ))
        } else if genesis_prev
            == &BlockHeaderHash::from_hex(
                "c6a004d3d178f600cd8caa10abbebe1549bef878f0665aea2903472d5abf7323",
            )
            .unwrap()
        {
            Ok(include_str!(
                "./test_data/c6a004d3d178f600cd8caa10abbebe1549bef878f0665aea2903472d5abf7323.json"
            ))
        } else if genesis_prev
            == &BlockHeaderHash::from_hex(
                "96fceff972c2c06bd3bb5243c39215333be6d56aaf4823073dca31afe5038471",
            )
            .unwrap()
        {
            Ok(include_str!(
                "./test_data/96fceff972c2c06bd3bb5243c39215333be6d56aaf4823073dca31afe5038471.json"
            ))
        } else {
            Err(*genesis_prev)
        }
    }

    #[test]
    pub fn calc_redeem_txid() {
        let (hash, address) = redeem_pubkey_to_txid(
            &chain_crypto::PublicKey::<Ed25519>::from_binary(
                &URL_SAFE
                    .decode("AAG3vJwTzCcL0zp2-1yfI-mn_7haYvSYJln2xR_aBS8=")
                    .unwrap(),
            )
            .unwrap(),
            None,
        );
        assert_eq!(
            hash.to_hex(),
            "927edb96f3386ab91b5f5d85d84cb4253c65b1c2f65fa7df25f81fab1d62987a"
        );
        assert_eq!(
            address.to_base58(),
            "Ae2tdPwUPEZ9vtyppa1FdJzvqJZkEcXgdHxVYAzTWcPaoNycVq5rc36LC1S"
        );
    }

    #[test]
    pub fn parse_test_genesis_files() {
        let genesis_hash = BlockHeaderHash::from_hex(
            "c6a004d3d178f600cd8caa10abbebe1549bef878f0665aea2903472d5abf7323",
        )
        .unwrap();

        let genesis_data =
            super::parse_genesis_data(get_test_genesis_data(&genesis_hash).unwrap().as_bytes())
                .unwrap();

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
        assert_eq!(genesis_data.fee_policy.coefficient, 43946 * 1_000_000u64);
        assert_eq!(genesis_data.fee_policy.constant, 155381 * 1_000_000_000u64);

        assert_eq!(
            URL_SAFE.encode(
                genesis_data
                    .avvm_distr
                    .iter()
                    .find(|(_, v)| **v == 9999300000000u64)
                    .unwrap()
                    .0
            ),
            "-0BJDi-gauylk4LptQTgjMeo7kY9lTCbZv12vwOSTZk="
        );

        let genesis_hash = BlockHeaderHash::from_hex(
            "b7f76950bc4866423538ab7764fc1c7020b24a5f717a5bee3109ff2796567214",
        )
        .unwrap();

        let genesis_data =
            super::parse_genesis_data(get_test_genesis_data(&genesis_hash).unwrap().as_bytes())
                .unwrap();

        assert_eq!(
            *genesis_data
                .non_avvm_balances
                .iter()
                .find(|(n, _)| n.to_string()
                    == "2cWKMJemoBaheSTiK9XEtQDf47Z3My8jwN25o5jjm7s7jaXin2nothhWQrTDd8m433M8K")
                .unwrap()
                .1,
            5428571428571429u64
        );
    }
}
