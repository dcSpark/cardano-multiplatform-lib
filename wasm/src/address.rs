use super::*;

use core::address::AddressHeaderKind;

#[wasm_bindgen]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum AddressKind {
    Base,
    Ptr,
    Enterprise,
    Reward,
    Byron,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Address(pub(crate) core::Address);

#[wasm_bindgen]
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
        self.0.header()
    }

    pub fn header_matches_kind(header: u8, kind: AddressHeaderKind) -> bool {
        core::Address::header_matches_kind(header, kind)
    }

    pub fn to_bech32(&self, prefix: Option<String>) -> Result<String, JsError> {
        self.0.to_bech32(prefix).map_err(Into::into)
    }

    pub fn from_bech32(bech_str: String) -> Result<Address, JsError> {
        core::Address::from_bech32(&bech_str).map(Into::into).map_err(Into::into)
    }
    
    /**
     * Note: bech32-encoded Byron addresses will also pass validation here
     */
    pub fn is_valid_bech32(bech_str: String) -> bool {
        core::Address::is_valid_bech32(&bech_str)
    }

    // pub fn is_valid_byron(base58: &str) -> bool {
    //     core::Address::is_valid_byron(base58)
    // }

    pub fn is_valid(bech_str: String) -> bool {
        core::Address::is_valid(&bech_str)
    }

    pub fn network_id(&self) -> Result<u8, JsError> {
        self.0.network_id().map_err(Into::into)
    }

    /// Note: by convention, the key inside reward addresses are considered payment credentials
    pub fn payment_cred(&self) -> Option<StakeCredential> {
        self.0.payment_cred().cloned().map(Into::into)
    }

    /// Note: by convention, the key inside reward addresses are NOT considered staking credentials
    /// Note: None is returned pointer addresses as the chain history is required to resolve its associated cred
    pub fn staking_cred(&self) -> Option<StakeCredential> {
        self.0.staking_cred().cloned().map(Into::into)
    }

    pub fn kind(&self) -> AddressKind {
        match &self.0 {
            core::Address::Base(_) => AddressKind::Base,
            core::Address::Ptr(_) => AddressKind::Ptr,
            core::Address::Enterprise(_) => AddressKind::Enterprise,
            core::Address::Reward(_) => AddressKind::Reward,
            //core::Address::Byron(_) => AddressKind::Byron,
        }
    }

    pub fn to_raw_bytes(&self) -> Vec<u8> {
        self.0.to_raw_bytes()
    }

    pub fn from_raw_bytes(data: &[u8]) -> Result<Address, JsError> {
        core::Address::from_raw_bytes(data).map(Self).map_err(Into::into)
    }

    pub fn to_json(&self) -> Result<String, JsError> {
        serde_json::to_string_pretty(&self.0).map_err(Into::into)
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsError> {
        JsValue::from_serde(&self.0).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<Address, JsError> {
        serde_json::from_str(json).map(Self).map_err(Into::into)
    }
}

impl From<core::Address> for Address {
    fn from(native: core::Address) -> Self {
        Self(native)
    }
}

impl From<Address> for core::Address {
    fn from(wasm: Address) -> Self {
        wasm.0
    }
}

pub type RewardAccount = RewardAddress;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct RewardAddress(pub(crate) core::RewardAddress);

#[wasm_bindgen]
impl RewardAddress {
    pub fn new(network: u8, payment: &StakeCredential) -> Self {
        Self(core::RewardAddress::new(network, payment.0.clone()))
    }

    pub fn to_address(&self) -> Address {
        Address(self.0.clone().to_address())
    }

    pub fn from_address(address: &Address) -> Option<RewardAddress> {
        match &address.0 {
            core::Address::Reward(ra) => Some(ra.clone().into()),
            _ => None,
        }
    }

    pub fn to_json(&self) -> Result<String, JsError> {
        serde_json::to_string_pretty(&self.0).map_err(Into::into)
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsError> {
        JsValue::from_serde(&self.0).map_err(Into::into)
    }

    pub fn from_json(json: &str) -> Result<RewardAddress, JsError> {
        serde_json::from_str(json).map(Self).map_err(Into::into)
    }
}

impl From<core::RewardAddress> for RewardAddress {
    fn from(native: core::RewardAddress) -> Self {
        Self(native)
    }
}

impl From<RewardAddress> for core::RewardAddress {
    fn from(wasm: RewardAddress) -> Self {
        wasm.0
    }
}