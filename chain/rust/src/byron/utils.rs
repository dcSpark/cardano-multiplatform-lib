use cbor_event::cbor;

use super::*;
use crate::{
    address::{Address, AddressError},
    crypto::BootstrapWitness,
    genesis::network_info::NetworkInfo,
};
use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    serialization::{Deserialize, ToBytes},
};
use cml_crypto::{
    chain_crypto::{self, Sha3_256},
    impl_hash_type, Bip32PrivateKey, Bip32PublicKey, CryptoError, Ed25519Signature,
    LegacyDaedalusPrivateKey, PublicKey, RawBytesEncoding, TransactionHash,
};
use std::{convert::TryFrom, fmt};

#[derive(Debug, thiserror::Error)]
pub enum ByronAddressError {
    #[error("UnknownNetwork: {0}")]
    UnknownNetwork(ProtocolMagic),
    #[error("InvalidCRC: found {found}, expected {expected}")]
    InvalidCRC { found: Crc32, expected: Crc32 },
}

impl_hash_type!(AddressId, 28);
// not sure if this is a hash but it likely is and has the same byte format
impl_hash_type!(ByronScript, 32);
impl_hash_type!(StakeholderId, 28);

impl StakeholderId {
    pub fn new(pubk: &Bip32PublicKey) -> StakeholderId {
        // the reason for this unwrap is that we have to dynamically allocate 66 bytes
        // to serialize 64 bytes in cbor (2 bytes of cbor overhead).
        let buf = cbor!(pubk.0.as_ref()).unwrap();

        let hash = Sha3_256::new(&buf);
        StakeholderId(Blake2b224::new(hash.as_ref()).into())
    }
}

impl AddrAttributes {
    pub fn new_bootstrap_era(
        hdap: Option<HDAddressPayload>,
        protocol_magic: Option<ProtocolMagic>,
    ) -> Self {
        let adjusted_magic = match &protocol_magic {
            Some(magic) => {
                if *magic == NetworkInfo::mainnet().protocol_magic() {
                    None
                } else {
                    protocol_magic
                }
            }
            None => None,
        };
        AddrAttributes {
            derivation_path: hdap,
            stake_distribution: Some(StakeDistribution::BootstrapEra),
            protocol_magic: adjusted_magic,
        }
    }
    pub fn new_single_key(
        pubk: &Bip32PublicKey,
        hdap: Option<HDAddressPayload>,
        protocol_magic: ProtocolMagic,
    ) -> Self {
        AddrAttributes {
            derivation_path: hdap,
            stake_distribution: Some(StakeDistribution::new_single_key(StakeholderId::new(pubk))),
            protocol_magic: Some(protocol_magic),
        }
    }
}

impl AddressId {
    pub fn new(
        addr_type: ByronAddrType,
        spending_data: &SpendingData,
        attrs: &AddrAttributes,
    ) -> Self {
        // the reason for this unwrap is that we have to dynamically allocate 66 bytes
        // to serialize 64 bytes in cbor (2 bytes of cbor overhead).
        let addr_type_uint = addr_type as u32;
        let buf = cbor!(&(&addr_type_uint, spending_data, attrs))
            .expect("serialize the AddressId's digest data");

        let hash = Sha3_256::new(&buf);
        AddressId(*Blake2b224::new(hash.as_ref()).as_hash_bytes())
    }
}

impl From<AddressContent> for ByronAddress {
    fn from(content: AddressContent) -> Self {
        let content_bytes = content.to_bytes();
        let crc = super::crc32::crc32(&content_bytes).into();
        ByronAddress { content, crc }
    }
}

impl AddressContent {
    pub fn hash_and_create(
        addr_type: ByronAddrType,
        spending_data: &SpendingData,
        attributes: AddrAttributes,
    ) -> AddressContent {
        let address_id = AddressId::new(addr_type, spending_data, &attributes);
        AddressContent::new(address_id, attributes, addr_type)
    }

    // bootstrap era + no hdpayload address
    pub fn new_redeem(pubkey: PublicKey, protocol_magic: Option<ProtocolMagic>) -> Self {
        let attributes = AddrAttributes::new_bootstrap_era(None, protocol_magic);
        let addr_type = ByronAddrType::Redeem;
        let spending_data = &SpendingData::new_spending_data_redeem(pubkey);

        AddressContent::hash_and_create(addr_type, spending_data, attributes)
    }

    // bootstrap era + no hdpayload address
    pub fn new_simple(xpub: Bip32PublicKey, protocol_magic: Option<ProtocolMagic>) -> Self {
        let attributes = AddrAttributes::new_bootstrap_era(None, protocol_magic);
        let addr_type = ByronAddrType::PublicKey;
        let spending_data = SpendingData::new_spending_data_pub_key(xpub);

        AddressContent::hash_and_create(addr_type, &spending_data, attributes)
    }

    /// Do we want to remove this or keep it for people who were using old Byron code?
    pub fn to_address(&self) -> ByronAddress {
        self.clone().into()
    }

    /// returns the byron protocol magic embedded in the address, or mainnet id if none is present
    /// note: for bech32 addresses, you need to use network_id instead
    pub fn byron_protocol_magic(&self) -> ProtocolMagic {
        match self.addr_attributes.protocol_magic {
            Some(x) => x,
            None => NetworkInfo::mainnet().protocol_magic(), // mainnet is implied if omitted
        }
    }

    pub fn network_id(&self) -> Result<u8, ByronAddressError> {
        // premise: during the Byron-era, we had one mainnet (764824073) and many many testnets
        // with each testnet getting a different protocol magic
        // in Shelley, this changes so that:
        // 1) all testnets use the same u8 protocol magic
        // 2) mainnet is re-mapped to a single u8 protocol magic

        // recall: in Byron mainnet, the network_id is omitted from the address to save a few bytes
        // so here we return the mainnet id if none is found in the address

        // mainnet is implied if omitted
        let protocol_magic = self.byron_protocol_magic();
        if protocol_magic == NetworkInfo::mainnet().protocol_magic() {
            Ok(NetworkInfo::mainnet().network_id())
        } else if protocol_magic == NetworkInfo::testnet().protocol_magic()
            || protocol_magic == NetworkInfo::preprod().protocol_magic()
            || protocol_magic == NetworkInfo::preview().protocol_magic()
            || protocol_magic == NetworkInfo::sancho_testnet().protocol_magic()
        {
            Ok(NetworkInfo::testnet().network_id())
        } else {
            Err(ByronAddressError::UnknownNetwork(protocol_magic))
        }
    }

    // icarus-style address (Ae2)
    pub fn icarus_from_key(key: Bip32PublicKey, protocol_magic: ProtocolMagic) -> AddressContent {
        // need to ensure we use None for mainnet since Byron-era addresses omitted the network id
        let filtered_protocol_magic = if protocol_magic == NetworkInfo::mainnet().protocol_magic() {
            None
        } else {
            Some(protocol_magic)
        };
        AddressContent::new_simple(key, filtered_protocol_magic)
    }

    /// Check if the Addr can be reconstructed with a specific xpub
    pub fn identical_with_pubkey(&self, xpub: Bip32PublicKey) -> bool {
        let addr_type = ByronAddrType::PublicKey;
        let spending_data = SpendingData::new_spending_data_pub_key(xpub);
        let newea = AddressContent::hash_and_create(
            addr_type,
            &spending_data,
            self.addr_attributes.clone(),
        );

        *self == newea
    }
}

impl ByronAddress {
    pub fn to_base58(&self) -> String {
        base58::encode(&self.to_bytes())
    }

    pub fn from_base58(s: &str) -> Result<ByronAddress, ParseExtendedAddrError> {
        let bytes = base58::decode(s).map_err(ParseExtendedAddrError::Base58Error)?;
        //.map_err(|_| JsError::from_str("ByronAddress::from_base58 failed to parse base58"))?;
        Self::from_cbor_bytes(&bytes).map_err(ParseExtendedAddrError::DeserializeError)
        //.map_err(|_| JsError::from_str("ByronAddress::from_base58 failed to parse bytes"))
    }

    pub fn is_valid(s: &str) -> bool {
        use std::str::FromStr;
        match ByronAddress::from_str(s) {
            Ok(_v) => true,
            Err(_err) => false,
        }
    }

    pub fn to_address(self) -> Address {
        self.into()
    }

    pub fn from_address(addr: &Address) -> Option<Self> {
        match addr {
            Address::Byron(byron) => Some(byron.clone()),
            _ => None,
        }
    }
}

impl TryFrom<Address> for ByronAddress {
    type Error = AddressError;

    fn try_from(addr: Address) -> Result<Self, Self::Error> {
        match addr {
            Address::Byron(byron) => Ok(byron),
            _ => Err(AddressError::WrongKind(addr.kind())),
        }
    }
}

impl From<ByronAddress> for Address {
    fn from(byron: ByronAddress) -> Self {
        Self::Byron(byron)
    }
}

impl fmt::Display for ByronAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

impl ::std::str::FromStr for ByronAddress {
    type Err = ParseExtendedAddrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_base58(s)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseExtendedAddrError {
    #[error("Deserialize: {0:?}")]
    DeserializeError(DeserializeError),
    #[error("Base58: {0:?}")]
    Base58Error(base58::Error),
}

pub fn make_daedalus_bootstrap_witness(
    tx_body_hash: TransactionHash,
    addr: ByronAddress,
    key: LegacyDaedalusPrivateKey,
) -> BootstrapWitness {
    let chain_code = key.chaincode();

    let pubkey = Bip32PublicKey::from_raw_bytes(key.as_ref().to_public().as_ref()).unwrap();
    let vkey = pubkey.to_raw_key();
    let signature =
        Ed25519Signature::from_raw_bytes(key.as_ref().sign(&tx_body_hash.to_raw_bytes()).as_ref())
            .unwrap();

    BootstrapWitness::new(vkey, signature, chain_code, addr.content.addr_attributes).unwrap()
}

pub fn make_icarus_bootstrap_witness(
    tx_body_hash: TransactionHash,
    addr: ByronAddress,
    key: &Bip32PrivateKey,
) -> BootstrapWitness {
    let chain_code = key.chaincode();

    let raw_key = key.to_raw_key();
    let vkey = raw_key.to_public();
    let signature = raw_key.sign(tx_body_hash.to_raw_bytes());

    BootstrapWitness::new(vkey, signature, chain_code, addr.content.addr_attributes).unwrap()
}

impl serde::Serialize for ByronAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_base58())
    }
}

impl<'de> serde::de::Deserialize<'de> for ByronAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let base58 = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Self::from_base58(&base58).map_err(|_e| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&base58),
                &"base58 byron address string",
            )
        })
    }
}

impl schemars::JsonSchema for ByronAddress {
    fn schema_name() -> String {
        String::from("ByronAddress")
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
    fn is_referenceable() -> bool {
        String::is_referenceable()
    }
}

#[cfg(test)]
mod tests {
    use super::ByronAddress;
    use crate::genesis::network_info::NetworkInfo;
    use cml_core::serialization::ToBytes;
    use cml_crypto::{
        chain_crypto::{self, Ed25519Bip32},
        Deserialize,
    };

    fn assert_same_address(address: ByronAddress, xpub: chain_crypto::PublicKey<Ed25519Bip32>) {
        assert!(
            address.content.identical_with_pubkey(xpub.into()),
            "{}",
            "expected public key {xpub} to match address {address}",
        )
    }

    #[test]
    fn test_vector_1() {
        let address: ByronAddress = "DdzFFzCqrhsrcTVhLygT24QwTnNqQqQ8mZrq5jykUzMveU26sxaH529kMpo7VhPrt5pwW3dXeB2k3EEvKcNBRmzCfcQ7dTkyGzTs658C".parse().unwrap();
        let public_key = chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&[
            0x6a, 0x50, 0x96, 0x89, 0xc6, 0x53, 0x17, 0x58, 0x65, 0x98, 0x5a, 0xd1, 0xe0, 0xeb,
            0x5f, 0xf9, 0xad, 0xa6, 0x99, 0x7a, 0xa4, 0x03, 0xe6, 0x48, 0x61, 0x4b, 0x3b, 0x78,
            0xfc, 0xba, 0x9c, 0x27, 0x30, 0x82, 0x28, 0xd9, 0x87, 0x2a, 0xf8, 0xb6, 0x5b, 0x98,
            0x7f, 0xf2, 0x3e, 0x1a, 0x20, 0xcd, 0x90, 0xd8, 0x34, 0x6c, 0x31, 0xf0, 0xed, 0xb8,
            0x99, 0x89, 0x52, 0xdc, 0x67, 0x66, 0x55, 0x80,
        ])
        .unwrap();
        assert_same_address(address, public_key);
    }

    #[test]
    fn test_vector_2() {
        let address = "DdzFFzCqrht4it4GYgBp4J39FNnKBsPFejSppARXHCf2gGiTJcwXzpRvgDmxPvKQ8aZZmVqcLUz5L66a8Ja46pfKVtFRaKyn9eKdvpaC".parse().unwrap();
        let public_key = chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&[
            0xff, 0x7b, 0xf1, 0x29, 0x9d, 0xf3, 0xd7, 0x17, 0x98, 0xae, 0xfd, 0xc4, 0xae, 0xa7,
            0xdb, 0x2f, 0x8d, 0xb7, 0x60, 0x46, 0x56, 0x94, 0x41, 0xea, 0xe5, 0x8b, 0x72, 0x23,
            0xb6, 0x8b, 0x44, 0x04, 0x82, 0x15, 0xcb, 0xac, 0x94, 0xbc, 0xb7, 0xf2, 0xcf, 0x33,
            0x6c, 0x6c, 0x18, 0xbc, 0x3e, 0x71, 0x3f, 0xfd, 0x82, 0x67, 0x59, 0x4f, 0xf6, 0x34,
            0x93, 0x32, 0xce, 0x4f, 0x98, 0x04, 0xa7, 0xff,
        ])
        .unwrap();
        assert_same_address(address, public_key)
    }

    #[test]
    fn test_vector_3() {
        let address = "DdzFFzCqrhsvNQtyViTvEdGxfdc5T1E5RorzFWjYodqjhFDy8fQxfDPccmTc4ePbvkiwvRkR8dtqQ1SHpH53fDSoxD17fo9f6WkRjjAA".parse().unwrap();
        let public_key = chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&[
            0x5c, 0x36, 0x51, 0xe0, 0xeb, 0x9d, 0x6d, 0xc9, 0x64, 0x07, 0x13, 0x7c, 0xcc, 0x1f,
            0x37, 0x7a, 0x87, 0x94, 0x61, 0x77, 0xa5, 0x2c, 0xa3, 0x77, 0x2c, 0x6b, 0x4b, 0xeb,
            0x72, 0x39, 0x50, 0xdc, 0x50, 0x22, 0x46, 0x68, 0x21, 0x8b, 0x8b, 0x36, 0x62, 0x02,
            0xfe, 0x5b, 0x7d, 0x55, 0x6f, 0x50, 0x1c, 0x5c, 0x4e, 0x2d, 0x58, 0xe0, 0x54, 0x67,
            0xe1, 0xab, 0xc0, 0x44, 0xc6, 0xc1, 0xbf, 0x8e,
        ])
        .unwrap();
        assert_same_address(address, public_key)
    }

    #[test]
    fn test_vector_4() {
        let address = "DdzFFzCqrhsn7ZAhKy8mxkzW6G3wryM7K6bH38VAjE2FesJMxia3UviivMvGz146TP1FpDharxTE6nUgCCnZx2fmtKpmxAosg9Tf5b8y".parse().unwrap();
        let public_key = chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&[
            0xcd, 0x84, 0x2e, 0x01, 0x0d, 0x81, 0xa6, 0xbe, 0x1e, 0x16, 0x9f, 0xd6, 0x35, 0x21,
            0xdb, 0xb9, 0x5f, 0x42, 0x41, 0xfc, 0x82, 0x3f, 0x45, 0xb1, 0xcf, 0x1a, 0x1c, 0xb4,
            0xc5, 0x89, 0x57, 0x27, 0x1d, 0x4d, 0x14, 0x2a, 0x22, 0x94, 0xea, 0x5f, 0xa3, 0x16,
            0xa4, 0xad, 0xbf, 0xcd, 0x59, 0x7a, 0x7c, 0x89, 0x6a, 0x52, 0xa9, 0xa3, 0xa9, 0xce,
            0x49, 0x64, 0x4a, 0x10, 0x2d, 0x00, 0x71, 0x99,
        ])
        .unwrap();
        assert_same_address(address, public_key)
    }

    #[test]
    fn test_vector_5() {
        let address = "DdzFFzCqrhssTCJf4sv664bdQURovAwzx1hNKkMkNLwMNyaxZFuPSDdZTTRMcoDyXHuCiZhbD4umvMJcWGkvFMMzBoBUW5UBdBbDqXGX".parse().unwrap();
        let public_key = chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&[
            0x5a, 0xac, 0x2d, 0xd0, 0xa8, 0xdc, 0x5d, 0x61, 0x0a, 0x4b, 0x6f, 0xdf, 0x3f, 0x5e,
            0xf1, 0xb6, 0x4a, 0xcb, 0x76, 0xb1, 0xe8, 0x1f, 0x6a, 0x35, 0x70, 0x31, 0xfa, 0x19,
            0xd5, 0xe6, 0x56, 0x9d, 0xcc, 0x37, 0xb7, 0xae, 0x6f, 0x39, 0x15, 0x82, 0xfb, 0x05,
            0x4b, 0x72, 0xba, 0xda, 0x90, 0xab, 0x14, 0x6c, 0xdd, 0x01, 0x42, 0x0e, 0x4b, 0x40,
            0x18, 0xf1, 0xa0, 0x55, 0x29, 0x82, 0xd2, 0x31,
        ])
        .unwrap();
        assert_same_address(address, public_key)
    }

    #[test]
    fn test_vector_6() {
        let address = "DdzFFzCqrhsfi5fFjJUHYPSnfTYrnMohzh3PrrtrVQgwua33HWPKUdTJXo3o77pSGCmDNrjYaAiZmJddaPW9iHyUDatvU2WhX7MgnNMy".parse().unwrap();
        let public_key = chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&[
            0x2a, 0x6a, 0xd1, 0x51, 0x09, 0x96, 0xff, 0x2d, 0x10, 0x89, 0xcb, 0x8e, 0xd5, 0xf5,
            0xc0, 0x61, 0xf6, 0xad, 0x0a, 0xfb, 0xb5, 0x3d, 0x95, 0x40, 0xa0, 0xfc, 0x89, 0xef,
            0xc0, 0xa2, 0x63, 0xb9, 0x6d, 0xac, 0x00, 0xbd, 0x0d, 0x7b, 0xda, 0x7d, 0x16, 0x3a,
            0x08, 0xdb, 0x20, 0xba, 0x64, 0xb6, 0x33, 0x4d, 0xca, 0x34, 0xea, 0xc8, 0x2c, 0xf7,
            0xb4, 0x91, 0xc3, 0x5f, 0x5c, 0xae, 0xc7, 0xb0,
        ])
        .unwrap();
        assert_same_address(address, public_key)
    }

    #[test]
    fn test_vector_7() {
        let address = "DdzFFzCqrhsy2zYMDQRCF4Nw34C3P7aT5B7JwHFQ6gLAeoHgVXurCLPCm3AeV1nTa1Nd46uDoNt16cnsPFkb4fpLi1J17AmvphCtGFz2".parse().unwrap();
        let public_key = chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&[
            0x0c, 0xd2, 0x15, 0x54, 0xa0, 0xf9, 0xb8, 0x25, 0x9c, 0x46, 0x88, 0xdd, 0x00, 0xfc,
            0x01, 0x88, 0x43, 0x50, 0x79, 0x76, 0x4f, 0xa5, 0x50, 0xfb, 0x57, 0x38, 0x2b, 0xff,
            0x43, 0xe2, 0xd8, 0xd8, 0x27, 0x27, 0x4e, 0x2a, 0x12, 0x9f, 0x86, 0xc3, 0x80, 0x88,
            0x34, 0x37, 0x4d, 0xfe, 0x3f, 0xda, 0xa6, 0x28, 0x48, 0x30, 0xb8, 0xf6, 0xe4, 0x0d,
            0x29, 0x93, 0xde, 0xa2, 0xfb, 0x0a, 0xbe, 0x82,
        ])
        .unwrap();
        assert_same_address(address, public_key)
    }

    #[test]
    fn test_vector_8() {
        let address = "DdzFFzCqrht8ygB5pLM4uVbS2x4ek2NTDx6R3DJqP7fUaWEkx8RA9UFR8CHitp2R74XLDP876Pe3KLUByHnrWrKWnffpqPpm14rPCxeP".parse().unwrap();
        let public_key = chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&[
            0x1f, 0x0a, 0xb8, 0x33, 0xfd, 0xb1, 0xfa, 0x49, 0x58, 0xce, 0x74, 0x04, 0x81, 0x84,
            0x5b, 0x3a, 0x26, 0x6e, 0xfa, 0xab, 0x2d, 0x65, 0xd1, 0x6b, 0xdd, 0x3d, 0xfe, 0x7f,
            0xcb, 0xe4, 0x46, 0x30, 0x25, 0x9e, 0xd1, 0x91, 0x98, 0x93, 0x03, 0x9d, 0xfd, 0x40,
            0x02, 0x4a, 0x72, 0x03, 0x45, 0x5b, 0x03, 0xd6, 0xd0, 0x0d, 0x0a, 0x5c, 0xd6, 0xee,
            0x82, 0xde, 0x2e, 0xce, 0x73, 0x8a, 0xa1, 0xbf,
        ])
        .unwrap();
        assert_same_address(address, public_key)
    }

    #[test]
    fn test_vector_9() {
        let address = "DdzFFzCqrhssTywqjv3dw3EakpEydWQcc3phQzR3YF9NPgQN9Ftkx68FfLLnpJ4vhWo9mAjx5EcpM1wNvorSySrpARZGfk5QugHkVs58".parse().unwrap();
        let public_key = chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&[
            0x16, 0xf7, 0xd2, 0x55, 0x32, 0x6d, 0x77, 0x6e, 0xc1, 0xb5, 0xed, 0xd2, 0x5f, 0x75,
            0xd3, 0xe3, 0xeb, 0xe0, 0xb9, 0xd4, 0x9c, 0xdd, 0xb2, 0x46, 0xd8, 0x0c, 0xf4, 0x1b,
            0x25, 0x24, 0x64, 0xb6, 0x24, 0x50, 0xa2, 0x4e, 0xf5, 0x98, 0x7b, 0x4b, 0xd6, 0x5e,
            0x0d, 0x25, 0x23, 0x43, 0xab, 0xa8, 0xef, 0x77, 0x93, 0x34, 0x79, 0xde, 0xa8, 0xdd,
            0xe2, 0x9e, 0xec, 0x56, 0xcc, 0x6a, 0xc0, 0x69,
        ])
        .unwrap();
        assert_same_address(address, public_key)
    }

    #[test]
    fn test_vector_10() {
        let address = "DdzFFzCqrhsqTG4t3uq5UBqFrxhxGVM6bvF4q1QcZXqUpizFddEEip7dx5rbife2s9o2fRU3hVKhRp4higog7As8z42s4AMw6Pcu8vL4".parse().unwrap();
        let public_key = chain_crypto::PublicKey::<Ed25519Bip32>::from_binary(&[
            0x97, 0xb8, 0x6c, 0x69, 0xd1, 0x2a, 0xf1, 0x64, 0xdc, 0x87, 0xf2, 0x71, 0x26, 0x8f,
            0x33, 0xbc, 0x4d, 0xee, 0xb0, 0xdf, 0xd3, 0x73, 0xc3, 0xfd, 0x3b, 0xac, 0xd4, 0x47,
            0x53, 0xa3, 0x1d, 0xe7, 0x8f, 0x10, 0xe5, 0x55, 0x03, 0x7c, 0xd4, 0x00, 0x43, 0x6c,
            0xcf, 0xd5, 0x38, 0x0d, 0xbb, 0xcd, 0x4d, 0x7c, 0x28, 0x0a, 0xef, 0x9e, 0xc7, 0x57,
            0x4a, 0xe0, 0xac, 0xac, 0x0c, 0xf7, 0x9e, 0x89,
        ])
        .unwrap();
        assert_same_address(address, public_key)
    }

    #[test]
    fn byron_magic_parsing() {
        // mainnet address w/ protocol magic omitted
        let addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZ4YjgvykNpoFeYUxoyhNj2kg8KfKWN2FizsSpLUPv68MpTVDo",
        )
        .unwrap();
        assert_eq!(
            addr.content.byron_protocol_magic(),
            NetworkInfo::mainnet().protocol_magic()
        );
        assert_eq!(
            addr.content.network_id().unwrap(),
            NetworkInfo::mainnet().network_id()
        );

        // original Byron testnet address
        let addr = ByronAddress::from_base58(
            "2cWKMJemoBaipzQe9BArYdo2iPUfJQdZAjm4iCzDA1AfNxJSTgm9FZQTmFCYhKkeYrede",
        )
        .unwrap();
        assert_eq!(
            addr.content.byron_protocol_magic(),
            NetworkInfo::testnet().protocol_magic()
        );
        assert_eq!(
            addr.content.network_id().unwrap(),
            NetworkInfo::testnet().network_id()
        );
    }

    #[test]
    fn round_trip() {
        // mainnet address
        let start = "DdzFFzCqrhsqTG4t3uq5UBqFrxhxGVM6bvF4q1QcZXqUpizFddEEip7dx5rbife2s9o2fRU3hVKhRp4higog7As8z42s4AMw6Pcu8vL4";
        let addr = ByronAddress::from_base58(start).unwrap();
        let end = addr.content.to_address().to_base58();
        assert_eq!(start, end);

        // mainnet address w/ protocol magic omitted
        let start = "Ae2tdPwUPEZ4YjgvykNpoFeYUxoyhNj2kg8KfKWN2FizsSpLUPv68MpTVDo";
        let addr = ByronAddress::from_base58(start).unwrap();
        let end = addr.content.to_address().to_base58();
        assert_eq!(start, end);

        // original Byron testnet address
        let start = "2cWKMJemoBaipzQe9BArYdo2iPUfJQdZAjm4iCzDA1AfNxJSTgm9FZQTmFCYhKkeYrede";
        let addr = ByronAddress::from_base58(start).unwrap();
        let end = addr.content.to_address().to_base58();
        assert_eq!(start, end);

        // testnet genesis address
        let start = "37btjrVyb4KEg6anTcJ9E4EAvYtNV9xXL6LNpA15YLhgvm9zJ1D2jwme574HikZ36rKdTwaUmpEicCoL1bDw4CtH5PNcFnTRGQNaFd5ai6Wvo6CZsi";
        let addr = ByronAddress::from_base58(start).unwrap();
        let end = addr.content.to_address().to_base58();
        assert_eq!(start, end);
    }

    #[test]
    fn parse_redeem_address() {
        assert!(ByronAddress::is_valid(
            "Ae2tdPwUPEZ3MHKkpT5Bpj549vrRH7nBqYjNXnCV8G2Bc2YxNcGHEa8ykDp"
        ));
        let byron_addr = ByronAddress::from_base58(
            "Ae2tdPwUPEZ3MHKkpT5Bpj549vrRH7nBqYjNXnCV8G2Bc2YxNcGHEa8ykDp",
        )
        .unwrap();
        assert_eq!(
            byron_addr.to_base58(),
            "Ae2tdPwUPEZ3MHKkpT5Bpj549vrRH7nBqYjNXnCV8G2Bc2YxNcGHEa8ykDp"
        );
        let byron_addr2 = ByronAddress::from_cbor_bytes(&byron_addr.to_bytes()).unwrap();
        assert_eq!(
            byron_addr2.to_base58(),
            "Ae2tdPwUPEZ3MHKkpT5Bpj549vrRH7nBqYjNXnCV8G2Bc2YxNcGHEa8ykDp"
        );
    }
}
