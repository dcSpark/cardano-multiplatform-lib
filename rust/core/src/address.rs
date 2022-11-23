use super::*;
use schemars::JsonSchema;
use cbor_event::{de::Deserializer, se::Serializer};
use bech32::ToBase32;
//use crate::byron::{ProtocolMagic, ByronAddress, ByronAddressError};
use derivative::Derivative;
//use crate::genesis::network_info::NetworkInfo;
use std::convert::TryInto;


#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Copy, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct ProtocolMagic(pub(crate) u32);

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NetworkInfo {
    network_id: u8,
    protocol_magic: ProtocolMagic,
}
impl NetworkInfo {
    pub fn new(network_id: u8, protocol_magic: ProtocolMagic) -> Self {
        Self {
            network_id,
            protocol_magic,
        }
    }
    pub fn network_id(&self) -> u8 {
        self.network_id
    }
    pub fn protocol_magic(&self) -> ProtocolMagic {
        self.protocol_magic
    }

    pub fn testnet() -> NetworkInfo {
        NetworkInfo {
            network_id: 0b0000,
            protocol_magic: ProtocolMagic(1097911063)
        }
    }
    pub fn mainnet() -> NetworkInfo {
        NetworkInfo {
            network_id: 0b0001,
            protocol_magic: ProtocolMagic(764824073)
        }
    }
}

// returns (Number represented, bytes read) if valid encoding
// or None if decoding prematurely finished
fn variable_nat_decode(bytes: &[u8]) -> Option<(num_bigint::BigUint, usize)> {
    let mut output = num_bigint::BigUint::from(0u64);
    let mut bytes_read = 0;
    for byte in bytes {
        output = (output * 128u8) + (byte & 0x7F);
        bytes_read += 1;
        if (byte & 0x80) == 0 {
            return Some((output, bytes_read));
        }
    }
    None
}

fn variable_nat_encode(mut num: num_bigint::BigUint) -> Vec<u8> {
    use num_integer::Integer;
    let zero = num_bigint::BigUint::from(0u64);
    let divider = num_bigint::BigUint::from(128u64);
    let (next, chunk) = num.div_rem(&divider);
    let chunk_byte: u8 = chunk.try_into().unwrap();
    let mut output = vec![chunk_byte];
    num = next;
    while num > zero {
        let (next, chunk) = num.div_rem(&divider);
        let chunk_byte: u8 = chunk.try_into().unwrap();
        num = next;
        output.push(chunk_byte | 0x80);
    }
    output.reverse();
    output
}

#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("Bech32: {0}")]
    Bech32(#[from] bech32::Error),
    //#[error("ByronError: {0}")]
    //Byron(#[from] ByronAddressError),
    #[error("CBOR: {0}")]
    CBOR(#[from] DeserializeError),
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Address {
    Base(BaseAddress),
    Ptr(PointerAddress),
    Enterprise(EnterpriseAddress),
    Reward(RewardAddress),
    //Byron(ByronAddress),
}

#[derive(Clone, Debug)]
pub struct AddressEncoding {
    // Some addresses were able to make it onchain with trailing data
    pub(crate) trailing: Option<Vec<u8>>,
    pub(crate) bytes_encoding: StringEncoding,
}

impl serde::Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let bech32 = self.to_bech32(None)
            .map_err(|e| serde::ser::Error::custom(format!("to_bech32: {:?}", e)))?;
        serializer.serialize_str(&bech32)
    }
}

impl <'de> serde::de::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
    D: serde::de::Deserializer<'de> {
        let bech32 = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        Address::from_bech32(&bech32)
            .map_err(|_e| serde::de::Error::invalid_value(serde::de::Unexpected::Str(&bech32), &"bech32 address string"))
    }
}

impl JsonSchema for Address {
    fn schema_name() -> String { String::from("Address") }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { String::json_schema(gen) }
    fn is_referenceable() -> bool { String::is_referenceable() }
}

/// Careful: this enum doesn't include the network ID part of the header
/// ex: base address isn't 0b0000_0000 but instead 0b0000
/// Use `header_matches_kind` if you don't want to implement the bitwise operators yourself
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AddressHeaderKind {
    BasePaymentKeyStakeKey = 0b0000,
    BasePaymentScriptStakeKey = 0b0001,
    BasePaymentKeyStakeScript = 0b0010,
    BasePaymentScriptStakeScript = 0b0011,
    PointerKey = 0b0100,
    PointerScript = 0b0101,
    EnterpriseKey = 0b0110,
    EnterpriseScript = 0b0111,
    Byron = 0b1000,
    RewardKey = 0b1110,
    RewardScript = 0b1111
    // 1001-1101 are left for future formats
}

impl Address {
    /// header has 4 bits addr type discrim then 4 bits network discrim.
    /// Copied from shelley.cddl:
    ///
    /// base address
    /// bits 7-6: 00
    /// bit 5: stake cred is keyhash/scripthash
    /// bit 4: payment cred is keyhash/scripthash
    /// bits 3-0: network id
    /// 
    /// pointer address
    /// bits 7-5: 010
    /// bit 4: payment cred is keyhash/scripthash
    /// bits 3-0: network id
    /// 
    /// enterprise address
    /// bits 7-5: 010
    /// bit 4: payment cred is keyhash/scripthash
    /// bits 3-0: network id
    ///
    /// reward addresses:
    /// bits 7-5: 111
    /// bit 4: credential is keyhash/scripthash
    /// bits 3-0: network id
    ///
    /// byron addresses:
    /// bits 7-4: 1000
    /// bits 3-0: unrelated data (recall: no network ID in Byron addresses)
    pub fn header(&self) -> u8 {
        fn stake_cred_bit(cred: &StakeCredential) -> u8 {
            match cred {
                StakeCredential::Key(_) => 0,
                StakeCredential::Script(_) => 1,
            }
        }
        match self {
            Self::Base(base) => (stake_cred_bit(&base.payment) << 4)
                | (stake_cred_bit(&base.stake) << 5)
                | (base.network & 0xF),
            Self::Ptr(ptr) => 0b0100_0000
                | (stake_cred_bit(&ptr.payment) << 4)
                | (ptr.network & 0xF),
            Self::Enterprise(enterprise) => 0b0110_0000
                | (stake_cred_bit(&enterprise.payment) << 4)
                | (enterprise.network & 0xF),
            //Self::Byron(_) => 0b1000 << 4, // note: no network ID for Byron
            Self::Reward(reward) => 0b1110_0000
                | (stake_cred_bit(&reward.payment) << 4)
                | (reward.network & 0xF),
        }
    }

    pub fn header_matches_kind(header: u8, kind: AddressHeaderKind) -> bool {
        (header >> 4) == kind as u8
    }

    /// The raw bytes of the Address - does not include any wrapping CBOR
    pub fn to_raw_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            Self::Base(base) => {
                buf.push(self.header());
                buf.extend(base.payment.to_raw_bytes());
                buf.extend(base.stake.to_raw_bytes());
            },
            Self::Ptr(ptr) => {
                buf.push(self.header());
                buf.extend(ptr.payment.to_raw_bytes());
                buf.extend(variable_nat_encode(ptr.stake.slot.clone()));
                buf.extend(variable_nat_encode(ptr.stake.tx_index.clone()));
                buf.extend(variable_nat_encode(ptr.stake.cert_index.clone()));
            },
            Self::Enterprise(enterprise) => {
                buf.push(self.header());
                buf.extend(enterprise.payment.to_raw_bytes());
            },
            Self::Reward(reward) => {
                buf.push(self.header());
                buf.extend(reward.payment.to_raw_bytes());
            },
            //Self::Byron(byron) => {
            //    buf.extend(byron.to_bytes())
            //},
        }
        if let Some(Some(trailing_bytes)) = self.encoding().map(|enc| &enc.trailing) {
            buf.extend(trailing_bytes.iter());
        }
        buf
    }

    pub fn from_raw_bytes(data: &[u8]) -> Result<Address, DeserializeError> {
        Self::from_bytes_impl(data, None)
    }

    pub(crate) fn from_bytes_impl(data: &[u8], bytes_encoding: Option<StringEncoding>) -> Result<Address, DeserializeError> {
        const TRAILING_WHITELIST: [&[u8]; 1] = [
            &[203, 87, 175, 176, 179, 95, 200, 156, 99, 6, 28, 153, 20, 224, 85, 0, 26, 81, 140, 117, 22]
        ];
        (|| -> Result<Self, DeserializeError> {
            let header = data[0];
            let network = header & 0x0F;
            const HASH_LEN: usize = Ed25519KeyHash::BYTE_COUNT;
            // should be static assert but it's maybe not worth importing a whole external crate for it now
            assert_eq!(ScriptHash::BYTE_COUNT, HASH_LEN);
            // checks the /bit/ bit of the header for key vs scripthash then reads the credential starting at byte position /pos/
            let read_addr_cred = |bit: u8, pos: usize| {
                let hash_bytes: [u8; HASH_LEN] = data[pos..pos+HASH_LEN].try_into().unwrap();
                if header & (1 << bit)  == 0 {
                    StakeCredential::Key(KeyStakeCredential::new(Ed25519KeyHash::from(hash_bytes)))
                } else {
                    StakeCredential::Script(ScriptStakeCredential::new(ScriptHash::from(hash_bytes)))
                }
            };
            fn make_encoding(bytes_encoding: Option<StringEncoding>, trailing: Option<Vec<u8>>) -> Result<Option<AddressEncoding>, DeserializeError> {
                if trailing.is_some() || bytes_encoding.is_some() {
                    if let Some(trailing) = &trailing {
                        let mut found = false;
                        for ending in TRAILING_WHITELIST.iter() {
                            if trailing.as_slice() == *ending {
                                found = true;
                            }
                        }
                        if !found {
                            return Err(cbor_event::Error::TrailingData.into());
                        }
                    }
                    Ok(Some(AddressEncoding {
                        trailing,
                        bytes_encoding: bytes_encoding.unwrap_or_default(),
                    }))
                } else {
                    Ok(None)
                }
            }
            fn len_check_trailing(data: &[u8], addr_size: usize) -> Result<Option<Vec<u8>>, DeserializeFailure> {
                if data.len() < addr_size {
                    Err(DeserializeFailure::CBOR(cbor_event::Error::NotEnough(data.len(), addr_size)))
                } else if data.len() > addr_size {
                    Ok(Some(data[addr_size..].to_vec()))
                } else {
                    Ok(None)
                }
            }
            match (header & 0xF0) >> 4 {
                // base
                0b0000 | 0b0001 | 0b0010 | 0b0011 => {
                    const BASE_ADDR_SIZE: usize = 1 + HASH_LEN * 2;
                    let trailing = len_check_trailing(data, BASE_ADDR_SIZE)?;
                    Ok(Address::Base(BaseAddress {
                        network,
                        payment: read_addr_cred(4, 1),
                        stake: read_addr_cred(5, 1 + HASH_LEN),
                        encoding: make_encoding(bytes_encoding, trailing)?,
                    }))
                },
                // pointer
                0b0100 | 0b0101 => {
                    // header + keyhash + 3 natural numbers (min 1 byte each)
                    const PTR_ADDR_MIN_SIZE: usize = 1 + HASH_LEN + 1 + 1 + 1;
                    if data.len() < PTR_ADDR_MIN_SIZE {
                        // possibly more, but depends on how many bytes the natural numbers are for the pointer
                        return Err(cbor_event::Error::NotEnough(data.len(), PTR_ADDR_MIN_SIZE).into());
                    }
                    let mut byte_index = 1;
                    let payment_cred = read_addr_cred(4, 1);
                    byte_index += HASH_LEN;
                    let (slot, slot_bytes) = variable_nat_decode(&data[byte_index..])
                        .ok_or_else(|| DeserializeError::new("Address.Pointer.slot", DeserializeFailure::VariableLenNatDecodeFailed))?;
                    byte_index += slot_bytes;
                    let (tx_index, tx_bytes) = variable_nat_decode(&data[byte_index..])
                        .ok_or_else(|| DeserializeError::new("Address.Pointer.tx_index", DeserializeFailure::VariableLenNatDecodeFailed))?;
                    byte_index += tx_bytes;
                    let (cert_index, cert_bytes) = variable_nat_decode(&data[byte_index..])
                        .ok_or_else(|| DeserializeError::new("Address.Pointer.cert_index", DeserializeFailure::VariableLenNatDecodeFailed))?;
                    byte_index += cert_bytes;
                    let trailing = if byte_index < data.len() {
                        Some(data[byte_index..].to_vec())
                    } else {
                        None
                    };
                    Ok(Address::Ptr(PointerAddress {
                        network,
                        payment: payment_cred,
                        stake: Pointer {
                            slot,
                            tx_index,
                            cert_index
                        },
                        encoding: make_encoding(bytes_encoding, trailing)?,
                    }))
                },
                // enterprise
                0b0110 | 0b0111 => {
                    const ENTERPRISE_ADDR_SIZE: usize = 1 + HASH_LEN;
                    let trailing = len_check_trailing(data, ENTERPRISE_ADDR_SIZE)?;
                    Ok(Address::Enterprise(EnterpriseAddress {
                        network,
                        payment: read_addr_cred(4, 1),
                        encoding: make_encoding(bytes_encoding, trailing)?,
                    }))
                },
                // reward
                0b1110 | 0b1111 => {
                    const REWARD_ADDR_SIZE: usize = 1 + HASH_LEN;
                    let trailing = len_check_trailing(data, REWARD_ADDR_SIZE)?;
                    Ok(Address::Reward(RewardAddress {
                        network,
                        payment: read_addr_cred(4, 1),
                        encoding: make_encoding(bytes_encoding, trailing)?,
                    }))
                }
                // byron
                0b1000 => {
                    // note: 0b1000 was chosen because all existing Byron addresses actually start with 0b1000
                    // Therefore you can re-use Byron addresses as-is
                    // match ByronAddress::from_bytes(data.to_vec()) {
                    //     Ok(addr) => Ok(Address::Byron(addr)),
                    //     Err(e) => Err(cbor_event::Error::CustomError(e.as_string().unwrap_or_default()).into()),
                    // }
                    todo!();
                },
                _ => Err(DeserializeFailure::BadAddressType(header).into()),
            }
        })().map_err(|e| e.annotate("Address"))
    }

    pub fn to_bech32(&self, prefix: Option<String>) -> Result<String, AddressError> {
        let final_prefix = match prefix {
            Some(prefix) => prefix,
            None => {
                // see CIP5 for bech32 prefix rules
                let prefix_header = match self {
                    Self::Reward(_) => "stake",
                    _ => "addr",
                };
                let prefix_tail = match self.network_id()? {
                    id if id == NetworkInfo::testnet().network_id() => "_test",
                    _ => "",
                };
                format!("{}{}", prefix_header, prefix_tail)
            }
        };
        bech32::encode(&final_prefix, self.to_raw_bytes().to_base32()).map_err(|e| e.into())
    }

    pub fn from_bech32(bech_str: &str) -> Result<Address, AddressError> {
        let (_hrp, u5data) = bech32::decode(bech_str)?;
        let data: Vec<u8> = bech32::FromBase32::from_base32(&u5data).unwrap();
        Ok(Self::from_bytes_impl(data.as_ref(), None)?)
    }
    
    /**
     * Note: bech32-encoded Byron addresses will also pass validation here
     */
    pub fn is_valid_bech32(bech_str: &str) -> bool {
        match Self::from_bech32(bech_str) {
            Ok(_v) => true,
            Err(_err) => false,
        }
    }

    //pub fn is_valid_byron(base58: &str) -> bool {
    //    ByronAddress::is_valid(base58)
    //}

    pub fn is_valid(bech_str: &str) -> bool {
        Self::is_valid_bech32(bech_str)// || Self::is_valid_byron(bech_str)
    }

    pub fn network_id(&self) -> Result<u8, AddressError> {
        match self {
            Self::Base(a) => Ok(a.network),
            Self::Enterprise(a) => Ok(a.network),
            Self::Ptr(a) => Ok(a.network),
            Self::Reward(a) => Ok(a.network),
            //Self::Byron(a) => a.address_content().network_id(),
        }
    }

    /// Note: by convention, the key inside reward addresses are considered payment credentials
    pub fn payment_cred(&self) -> Option<&StakeCredential> {
        match self {
            Self::Base(a) => Some(&a.payment),
            Self::Enterprise(a) => Some(&a.payment),
            Self::Ptr(a) => Some(&a.payment),
            Self::Reward(a) => Some(&a.payment),
            //Self::Byron(_) => None,
        }
    }

    /// Note: by convention, the key inside reward addresses are NOT considered staking credentials
    /// Note: None is returned pointer addresses as the chain history is required to resolve its associated cred
    pub fn staking_cred(&self) -> Option<&StakeCredential> {
        match self {
            Self::Base(a) => Some(&a.stake),
            Self::Enterprise(_) => None,
            Self::Ptr(_) => None,
            Self::Reward(_) => None,
            //Self::Byron(_) => None,
        }
    }

    pub(crate) fn encoding(&self) -> Option<&AddressEncoding> {
        match self {
            Self::Base(a) => a.encoding.as_ref(),
            Self::Enterprise(a) => a.encoding.as_ref(),
            Self::Ptr(a) => a.encoding.as_ref(),
            Self::Reward(a) => a.encoding.as_ref(),
            //Self::Byron(_a) => None,
        }
    }
}

#[derive(Debug, Clone, Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BaseAddress {
    pub network: u8,
    pub payment: StakeCredential,
    pub stake: StakeCredential,
    #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
    pub(crate) encoding: Option<AddressEncoding>,
}

impl BaseAddress {
    pub fn new(network: u8, payment: StakeCredential, stake: StakeCredential) -> Self {
        Self {
            network,
            payment,
            stake,
            encoding: None,
        }
    }

    pub fn to_address(self) -> Address {
        Address::Base(self)
    }

    pub fn from_address(addr: &Address) -> Option<BaseAddress> {
        match addr {
            Address::Base(base) => Some(base.clone()),
            _ => None,
        }
    }
}


#[derive(Debug, Clone, Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EnterpriseAddress {
    pub network: u8,
    pub payment: StakeCredential,
    #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
    pub(crate) encoding: Option<AddressEncoding>,
}

impl EnterpriseAddress {
    pub fn new(network: u8, payment: StakeCredential) -> Self {
        Self {
            network,
            payment,
            encoding: None,
        }
    }

    pub fn to_address(self) -> Address {
        Address::Enterprise(self)
    }

    pub fn from_address(addr: &Address) -> Option<EnterpriseAddress> {
        match addr {
            Address::Enterprise(enterprise) => Some(enterprise.clone()),
            _ => None,
        }
    }
}

pub type RewardAccount = RewardAddress;

#[derive(Debug, Clone, Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RewardAddress {
    pub network: u8,
    pub payment: StakeCredential,
    #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
    pub(crate) encoding: Option<AddressEncoding>,
}

impl RewardAddress {
    pub fn new(network: u8, payment: StakeCredential) -> Self {
        Self {
            network,
            payment,
            encoding: None,
        }
    }

    pub fn to_address(self) -> Address {
        Address::Reward(self)
    }

    pub fn from_address(addr: &Address) -> Option<RewardAddress> {
        match addr {
            Address::Reward(reward) => Some(reward.clone()),
            _ => None,
        }
    }
}

impl serde::Serialize for RewardAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let bech32 = self
            .clone()
            .to_address()
            .to_bech32(None)
            .map_err(|e| serde::ser::Error::custom(format!("to_bech32: {:?}", e)))?;
        serializer.serialize_str(&bech32)
    }
}

impl <'de> serde::de::Deserialize<'de> for RewardAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
    D: serde::de::Deserializer<'de> {
        let bech32 = <String as serde::de::Deserialize>::deserialize(deserializer)?;
        match Address::from_bech32(&bech32).ok().map(|addr| RewardAddress::from_address(&addr)) {
            Some(Some(ra)) => Ok(ra),
            _ => Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(&bech32), &"bech32 reward address string")),
        }
    }
}

impl JsonSchema for RewardAddress {
    fn schema_name() -> String { String::from("RewardAddress") }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema { String::json_schema(gen) }
    fn is_referenceable() -> bool { String::is_referenceable() }
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Pointer {
    slot: num_bigint::BigUint,
    tx_index: num_bigint::BigUint,
    cert_index: num_bigint::BigUint,
}

impl Pointer {
    pub fn new(slot: Slot, tx_index: TransactionIndex, cert_index: CertificateIndex) -> Self {
        Self {
            slot: num_bigint::BigUint::from(slot),
            tx_index: num_bigint::BigUint::from(tx_index),
            cert_index: num_bigint::BigUint::from(cert_index),
        }
    }

    /// This will be truncated if above u64::MAX
    pub fn slot(&self) -> Slot {
        self.slot.clone().try_into().unwrap_or(u64::MAX)
    }

    /// This will be truncated if above u64::MAX
    pub fn tx_index(&self) -> Slot {
        self.tx_index.clone().try_into().unwrap_or(u64::MAX)
    }

    /// This will be truncated if above u64::MAX
    pub fn cert_index(&self) -> Slot {
        self.cert_index.clone().try_into().unwrap_or(u64::MAX)
    }
}

#[derive(Debug, Clone, Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PointerAddress {
    pub network: u8,
    pub payment: StakeCredential,
    pub stake: Pointer,
    #[derivative(PartialEq="ignore", Ord="ignore", PartialOrd="ignore", Hash="ignore")]
    pub(crate) encoding: Option<AddressEncoding>,
}

impl PointerAddress {
    pub fn new(network: u8, payment: StakeCredential, stake: Pointer) -> Self {
        Self {
            network,
            payment,
            stake,
            encoding: None,
        }
    }

    pub fn to_address(self) -> Address {
        Address::Ptr(self)
    }

    pub fn from_address(addr: &Address) -> Option<PointerAddress> {
        match addr {
            Address::Ptr(ptr) => Some(ptr.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ledger::common::hash::ScriptHashNamespace, byron::AddressContent};

    use super::*;
    use crypto::*;

    #[test]
    fn variable_nat_encoding() {
        let cases = [
            0u64,
            127u64,
            128u64,
            255u64,
            256275757658493284u64
        ];
        for case in cases.iter() {
            let case_biguint = num_bigint::BigUint::from(*case);
            let encoded = variable_nat_encode(case_biguint.clone());
            let decoded = variable_nat_decode(&encoded).unwrap().0;
            assert_eq!(case_biguint, decoded);
        }
    }

    #[test]
    fn base_serialize_consistency() {
        let base = BaseAddress::new(
            5,
            StakeCredential::from_keyhash(Ed25519KeyHash::from([23; Ed25519KeyHash::BYTE_COUNT])),
            StakeCredential::from_scripthash(ScriptHash::from([42; ScriptHash::BYTE_COUNT])));
        let addr = base.to_address();
        let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
        assert_eq!(addr.to_bytes(), addr2.to_bytes());
    }

    #[test]
    fn ptr_serialize_consistency() {
        let ptr = PointerAddress::new(
            25,
            StakeCredential::from_keyhash(Ed25519KeyHash::from([23; Ed25519KeyHash::BYTE_COUNT])),
            Pointer::new(2354556573, 127, 0));
        let addr = ptr.to_address();
        let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
        assert_eq!(addr.to_bytes(), addr2.to_bytes());
    }

    #[test]
    fn enterprise_serialize_consistency() {
        let enterprise = EnterpriseAddress::new(
            64,
            StakeCredential::from_keyhash(Ed25519KeyHash::from([23; Ed25519KeyHash::BYTE_COUNT])));
        let addr = enterprise.to_address();
        let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
        assert_eq!(addr.to_bytes(), addr2.to_bytes());
    }

    #[test]
    fn reward_serialize_consistency() {
        let reward = RewardAddress::new(
            9,
            StakeCredential::from_scripthash(ScriptHash::from([127; Ed25519KeyHash::BYTE_COUNT])));
        let addr = reward.to_address();
        let addr2 = Address::from_bytes(addr.to_bytes()).unwrap();
        assert_eq!(addr.to_bytes(), addr2.to_bytes());
    }

    #[test]
    fn address_header_matching() {
        let reward = Rew51ardAddress::new(
            0b1001,
            StakeCredential::from_scripthash(ScriptHash::from([127; Ed25519KeyHash::BYTE_COUNT]))
        ).to_address();
        assert_eq!(reward.header(), 0b1111_1001);
        assert!(Address::header_matches_kind(reward.header(), AddressHeaderKind::RewardScript))
    }

    fn root_key_12() -> Bip32PrivateKey {
        // test walk nut penalty hip pave soap entry language right filter choice
        let entropy = [0xdf, 0x9e, 0xd2, 0x5e, 0xd1, 0x46, 0xbf, 0x43, 0x33, 0x6a, 0x5d, 0x7c, 0xf7, 0x39, 0x59, 0x94];
        Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
    }

    fn root_key_15() -> Bip32PrivateKey {
        // art forum devote street sure rather head chuckle guard poverty release quote oak craft enemy
        let entropy = [0x0c, 0xcb, 0x74, 0xf3, 0x6b, 0x7d, 0xa1, 0x64, 0x9a, 0x81, 0x44, 0x67, 0x55, 0x22, 0xd4, 0xd8, 0x09, 0x7c, 0x64, 0x12];
        Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
    }

    fn root_key_24() -> Bip32PrivateKey {
        let entropy = [0x4e, 0x82, 0x8f, 0x9a, 0x67, 0xdd, 0xcf, 0xf0, 0xe6, 0x39, 0x1a, 0xd4, 0xf2, 0x6d, 0xdb, 0x75, 0x79, 0xf5, 0x9b, 0xa1, 0x4b, 0x6d, 0xd4, 0xba, 0xf6, 0x3d, 0xcf, 0xdb, 0x9d, 0x24, 0x20, 0xda];
        Bip32PrivateKey::from_bip39_entropy(&entropy, &[])
    }

    fn harden(index: u32) -> u32 {
        index | 0x80_00_00_00
    }

    #[test]
    fn bech32_parsing() {
        let addr = Address::from_bech32("addr1u8pcjgmx7962w6hey5hhsd502araxp26kdtgagakhaqtq8sxy9w7g").unwrap();
        assert_eq!(addr.to_bech32(Some("foobar".to_string())).unwrap(), "foobar1u8pcjgmx7962w6hey5hhsd502araxp26kdtgagakhaqtq8s92n4tm");
    }

    #[test]
    fn bip32_12_base() {
        let spend = root_key_12()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_12()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone(), stake_cred.clone()).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1qz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3jcu5d8ps7zex2k2xt3uqxgjqnnj83ws8lhrn648jjxtwq2ytjqp");
        let addr_net_3 = BaseAddress::new(NetworkInfo::mainnet().network_id(), spend_cred, stake_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3jcu5d8ps7zex2k2xt3uqxgjqnnj83ws8lhrn648jjxtwqfjkjv7");
    }

    #[test]
    fn bip32_12_enterprise() {
        let spend = root_key_12()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(spend.to_raw_key().hash());
        let addr_net_0 = EnterpriseAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone()).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1vz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzerspjrlsz");
        let addr_net_3 = EnterpriseAddress::new(NetworkInfo::mainnet().network_id(), spend_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1vx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzers66hrl8");
    }

    #[test]
    fn bip32_12_pointer() {
        let spend = root_key_12()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(spend.to_raw_key().hash());
        let addr_net_0 = PointerAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone(), Pointer::new(1, 2, 3)).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1gz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzerspqgpsqe70et");
        let addr_net_3 = PointerAddress::new(NetworkInfo::mainnet().network_id(), spend_cred, Pointer::new(24157, 177, 42)).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1gx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer5ph3wczvf2w8lunk");
    }

    #[test]
    fn bip32_15_base() {
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
        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone(), stake_cred.clone()).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1qpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qum8x5w");
        let addr_net_3 = BaseAddress::new(NetworkInfo::mainnet().network_id(), spend_cred, stake_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1q9u5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5ewvxwdrt70qlcpeeagscasafhffqsxy36t90ldv06wqrk2qld6xc3");
    }

    #[test]
    fn bip32_15_enterprise() {
        let spend = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(spend.to_raw_key().hash());
        let addr_net_0 = EnterpriseAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone()).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1vpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5eg57c2qv");
        let addr_net_3 = EnterpriseAddress::new(NetworkInfo::mainnet().network_id(), spend_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1v9u5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5eg0kvk0f");
    }

    #[test]
    fn bip32_15_pointer() {
        let spend = root_key_15()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(spend.to_raw_key().hash());
        let addr_net_0 = PointerAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone(), Pointer::new(1, 2, 3)).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1gpu5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5egpqgpsdhdyc0");
        let addr_net_3 = PointerAddress::new(NetworkInfo::mainnet().network_id(), spend_cred, Pointer::new(24157, 177, 42)).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1g9u5vlrf4xkxv2qpwngf6cjhtw542ayty80v8dyr49rf5evph3wczvf2kd5vam");
    }

    #[test]
    fn bip32_15_byron() {
        let byron_key = root_key_15()
            .derive(harden(44))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let byron_addr = AddressContent::icarus_from_key(&byron_key, NetworkInfo::mainnet().protocol_magic());
        assert_eq!(byron_addr.to_address().to_base58(), "Ae2tdPwUPEZHtBmjZBF4YpMkK9tMSPTE2ADEZTPN97saNkhG78TvXdp3GDk");
        assert!(ByronAddress::is_valid("Ae2tdPwUPEZHtBmjZBF4YpMkK9tMSPTE2ADEZTPN97saNkhG78TvXdp3GDk"));
        assert_eq!(byron_addr.network_id().unwrap(), 0b0001);

        // round-trip from generic address type and back
        let generic_addr = Address::from_bytes(byron_addr.to_address().to_bytes()).unwrap();
        let byron_addr_2 = ByronAddress::from_address(generic_addr).unwrap();
        assert_eq!(byron_addr.to_address().to_base58(), byron_addr_2.to_base58());
    }

    #[test]
    fn bip32_24_base() {
        let spend = root_key_24()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_24()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone(), stake_cred.clone()).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1qqy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmn8k8ttq8f3gag0h89aepvx3xf69g0l9pf80tqv7cve0l33sw96paj");
        let addr_net_3 = BaseAddress::new(NetworkInfo::mainnet().network_id(), spend_cred, stake_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1qyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmn8k8ttq8f3gag0h89aepvx3xf69g0l9pf80tqv7cve0l33sdn8p3d");
    }

    #[test]
    fn bip32_24_enterprise() {
        let spend = root_key_24()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(spend.to_raw_key().hash());
        let addr_net_0 = EnterpriseAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone()).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1vqy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqtjtf68");
        let addr_net_3 = EnterpriseAddress::new(NetworkInfo::mainnet().network_id(), spend_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1vyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqs6l44z");
    }

    #[test]
    fn bip32_24_pointer() {
        let spend = root_key_24()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(spend.to_raw_key().hash());
        let addr_net_0 = PointerAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone(), &Pointer::new(1, 2, 3)).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1gqy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnqpqgps5mee0p");
        let addr_net_3 = PointerAddress::new(NetworkInfo::mainnet().network_id(), spend_cred, &Pointer::new(24157, 177, 42)).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1gyy6nhfyks7wdu3dudslys37v252w2nwhv0fw2nfawemmnyph3wczvf2dqflgt");
    }

    #[test]
    fn bip32_12_reward() {
        let staking_key = root_key_12()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();
        let staking_cred = StakeCredential::from_keyhash(staking_key.to_raw_key().hash());
        let addr_net_0 = RewardAddress::new(NetworkInfo::testnet().network_id(), staking_cred.clone()).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "stake_test1uqevw2xnsc0pvn9t9r9c7qryfqfeerchgrlm3ea2nefr9hqp8n5xl");
        let addr_net_3 = RewardAddress::new(NetworkInfo::mainnet().network_id(), staking_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "stake1uyevw2xnsc0pvn9t9r9c7qryfqfeerchgrlm3ea2nefr9hqxdekzz");
    }

    #[test]
    fn bip32_24_base_multisig_hd_derivation() {
        let spend = root_key_24()
            .derive(harden(1854))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();
        let stake = root_key_24()
            .derive(harden(1854))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(2)
            .derive(0)
            .to_public();
        let spend_cred = StakeCredential::from_keyhash(spend.to_raw_key().hash());
        let stake_cred = StakeCredential::from_keyhash(stake.to_raw_key().hash());
        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone(), stake_cred.clone()).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1qz8fg2e9yn0ga6sav0760cxmx0antql96mfuhqgzcc5swugw2jqqlugnx9qjep9xvcx40z0zfyep55r2t3lav5smyjrs96cusg");
        let addr_net_3 = BaseAddress::new(NetworkInfo::mainnet().network_id(), spend_cred, stake_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1qx8fg2e9yn0ga6sav0760cxmx0antql96mfuhqgzcc5swugw2jqqlugnx9qjep9xvcx40z0zfyep55r2t3lav5smyjrsxv9uuh");
    }

    #[test]
    fn multisig_from_script() {

        let spend = root_key_24()
            .derive(harden(1852))
            .derive(harden(1815))
            .derive(harden(0))
            .derive(0)
            .derive(0)
            .to_public();


        let mut pubkey_native_scripts = NativeScripts::new();

        let spending_hash = spend.to_raw_key().hash();
        pubkey_native_scripts.add(NativeScript::new_script_pubkey(ScriptPubkey::new(spending_hash)));
        let oneof_native_script = NativeScript::new_script_n_of_k(ScriptNOfK::new(1, pubkey_native_scripts));

        let script_hash = ScriptHash::from_bytes(
            oneof_native_script.hash().to_bytes()
        ).unwrap();

        let spend_cred = StakeCredential::from_scripthash(script_hash);
        let stake_cred = StakeCredential::from_scripthash(script_hash);
        let addr_net_0 = BaseAddress::new(NetworkInfo::testnet().network_id(), spend_cred.clone(), stake_cred.clone()).to_address();
        assert_eq!(addr_net_0.to_bech32(None).unwrap(), "addr_test1xr0de0mz3m9xmgtlmqqzu06s0uvfsczskdec8k7v4jhr7077mjlk9rk2dkshlkqq9cl4qlccnps9pvmns0duet9w8uls8flvxc");
        let addr_net_3 = BaseAddress::new(NetworkInfo::mainnet().network_id(), spend_cred, stake_cred).to_address();
        assert_eq!(addr_net_3.to_bech32(None).unwrap(), "addr1x80de0mz3m9xmgtlmqqzu06s0uvfsczskdec8k7v4jhr7077mjlk9rk2dkshlkqq9cl4qlccnps9pvmns0duet9w8ulsylzv28");
    }

    #[test]
    fn pointer_address_big() {
        let addr = Address::from_bech32("addr_test1grqe6lg9ay8wkcu5k5e38lne63c80h3nq6xxhqfmhewf645pllllllllllll7lupllllllllllll7lupllllllllllll7lc9wayvj").unwrap();
        let ptr = PointerAddress::from_address(addr).unwrap().stake;
        let u64_max = num_bigint::BigUint::from(u64::MAX);
        assert_eq!(u64_max, ptr.slot);
        assert_eq!(u64_max, ptr.tx_index);
        assert_eq!(u64_max, ptr.cert_index);
    }

    #[test]
    fn long_address() {
        let long = Address::from_bech32("addr1q9d66zzs27kppmx8qc8h43q7m4hkxp5d39377lvxefvxd8j7eukjsdqc5c97t2zg5guqadepqqx6rc9m7wtnxy6tajjvk4a0kze4ljyuvvrpexg5up2sqxj33363v35gtew").unwrap();
        let long_trimmed = Address::from_bech32("addr1q9d66zzs27kppmx8qc8h43q7m4hkxp5d39377lvxefvxd8j7eukjsdqc5c97t2zg5guqadepqqx6rc9m7wtnxy6tajjq6r54x9").unwrap();
        assert_eq!(long, long_trimmed);
        assert_eq!(long.trailing, Some(vec![203u8, 87, 175, 176, 179, 95, 200, 156, 99, 6, 28, 153, 20, 224, 85, 0, 26, 81, 140, 117, 22]));
        assert_eq!(long_trimmed.trailing, None);
        assert_eq!(long.to_bytes(), hex::decode("015bad085057ac10ecc7060f7ac41edd6f63068d8963ef7d86ca58669e5ecf2d283418a60be5a848a2380eb721000da1e0bbf39733134beca4cb57afb0b35fc89c63061c9914e055001a518c7516").unwrap());
        let long_not_whitelisted = Address::from_bech32("addr_test1vqt3w9chzut3w9chzut3w9chzut3w9chzut3w9chzut3w9cqqspqvqcqsmxqdssg97");
        assert!(long_not_whitelisted.is_err());
    }

    #[test]
    fn ptr_addr_huge_slot() {
        let addr_bytes: Vec<u8> = vec![64, 193, 157, 125, 5, 233, 14, 235, 99, 148, 181, 51, 19, 254, 121, 212, 112, 119, 222, 51, 6, 140, 107, 129, 59, 190, 92, 157, 86, 129, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127, 129, 255, 255, 255, 255, 255, 255, 255, 255, 127, 129, 255, 255, 255, 255, 255, 255, 255, 255, 127];
        let addr = Address::from_bytes(addr_bytes.clone()).unwrap();
        assert_eq!(addr_bytes, addr.to_bytes());
    }
}
