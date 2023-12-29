use super::*;

pub use cml_chain::address::{AddressHeaderKind, AddressKind};

use cml_core::CertificateIndex;
use cml_core_wasm::{impl_wasm_conversions, impl_wasm_json_api};

use crate::certs::StakeCredential;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Address(cml_chain::address::Address);

impl_wasm_conversions!(cml_chain::address::Address, Address);

impl_wasm_json_api!(Address);

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
        cml_chain::address::Address::header_matches_kind(header, kind)
    }

    pub fn to_bech32(&self, prefix: Option<String>) -> Result<String, JsError> {
        self.0.to_bech32(prefix).map_err(Into::into)
    }

    pub fn from_bech32(bech_str: String) -> Result<Address, JsError> {
        cml_chain::address::Address::from_bech32(&bech_str)
            .map(Into::into)
            .map_err(Into::into)
    }

    /**
     * Note: bech32-encoded Byron addresses will also pass validation here
     */
    pub fn is_valid_bech32(bech_str: String) -> bool {
        cml_chain::address::Address::is_valid_bech32(&bech_str)
    }

    // pub fn is_valid_byron(base58: &str) -> bool {
    //     cml_chain::address::Address::is_valid_byron(base58)
    // }

    pub fn is_valid(bech_str: String) -> bool {
        cml_chain::address::Address::is_valid(&bech_str)
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
        self.0.kind()
    }

    pub fn to_raw_bytes(&self) -> Vec<u8> {
        self.0.to_raw_bytes()
    }

    pub fn from_raw_bytes(data: &[u8]) -> Result<Address, JsError> {
        cml_chain::address::Address::from_raw_bytes(data)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }

    pub fn from_hex(hex: &str) -> Result<Address, JsError> {
        cml_chain::address::Address::from_hex(hex)
            .map(Into::into)
            .map_err(Into::into)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BaseAddress(cml_chain::address::BaseAddress);

impl_wasm_conversions!(cml_chain::address::BaseAddress, BaseAddress);

#[wasm_bindgen]
impl BaseAddress {
    pub fn new(network: u8, payment: &StakeCredential, stake: &StakeCredential) -> Self {
        Self(cml_chain::address::BaseAddress::new(
            network,
            payment.as_ref().clone(),
            stake.as_ref().clone(),
        ))
    }

    pub fn to_address(&self) -> Address {
        Address(self.0.clone().to_address())
    }

    pub fn from_address(address: &Address) -> Option<BaseAddress> {
        match &address.0 {
            cml_chain::address::Address::Base(ba) => Some(ba.clone().into()),
            _ => None,
        }
    }

    pub fn network_id(&self) -> u8 {
        self.0.network
    }

    pub fn payment(&self) -> StakeCredential {
        self.0.payment.clone().into()
    }

    pub fn stake(&self) -> StakeCredential {
        self.0.stake.clone().into()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct EnterpriseAddress(cml_chain::address::EnterpriseAddress);

impl_wasm_conversions!(cml_chain::address::EnterpriseAddress, EnterpriseAddress);

#[wasm_bindgen]
impl EnterpriseAddress {
    pub fn new(network: u8, payment: &StakeCredential) -> Self {
        Self(cml_chain::address::EnterpriseAddress::new(
            network,
            payment.as_ref().clone(),
        ))
    }

    pub fn to_address(&self) -> Address {
        Address(self.0.clone().to_address())
    }

    pub fn from_address(address: &Address) -> Option<EnterpriseAddress> {
        match &address.0 {
            cml_chain::address::Address::Enterprise(ea) => Some(ea.clone().into()),
            _ => None,
        }
    }

    pub fn network_id(&self) -> u8 {
        self.0.network
    }

    pub fn payment(&self) -> StakeCredential {
        self.0.payment.clone().into()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Pointer(cml_chain::address::Pointer);

impl_wasm_conversions!(cml_chain::address::Pointer, Pointer);

impl Pointer {
    pub fn new(slot: Slot, tx_index: TransactionIndex, cert_index: CertificateIndex) -> Self {
        Self(cml_chain::address::Pointer::new(slot, tx_index, cert_index))
    }

    /// This will be truncated if above u64::MAX
    pub fn slot(&self) -> Slot {
        self.0.slot()
    }

    /// This will be truncated if above u64::MAX
    pub fn tx_index(&self) -> Slot {
        self.0.tx_index()
    }

    /// This will be truncated if above u64::MAX
    pub fn cert_index(&self) -> Slot {
        self.0.cert_index()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PointerAddress(cml_chain::address::PointerAddress);

impl_wasm_conversions!(cml_chain::address::PointerAddress, PointerAddress);

#[wasm_bindgen]
impl PointerAddress {
    pub fn new(network: u8, payment: &StakeCredential, stake: &Pointer) -> Self {
        Self(cml_chain::address::PointerAddress::new(
            network,
            payment.as_ref().clone(),
            stake.as_ref().clone(),
        ))
    }

    pub fn to_address(&self) -> Address {
        Address(self.0.clone().to_address())
    }

    pub fn from_address(address: &Address) -> Option<PointerAddress> {
        match &address.0 {
            cml_chain::address::Address::Ptr(pa) => Some(pa.clone().into()),
            _ => None,
        }
    }

    pub fn network_id(&self) -> u8 {
        self.0.network
    }

    pub fn payment(&self) -> StakeCredential {
        self.0.payment.clone().into()
    }

    pub fn stake(&self) -> Pointer {
        self.0.stake.clone().into()
    }
}

pub type RewardAccount = RewardAddress;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct RewardAddress(cml_chain::address::RewardAddress);

impl_wasm_conversions!(cml_chain::address::RewardAddress, RewardAddress);

impl_wasm_json_api!(RewardAddress);

#[wasm_bindgen]
impl RewardAddress {
    pub fn new(network: u8, payment: &StakeCredential) -> Self {
        Self(cml_chain::address::RewardAddress::new(
            network,
            payment.as_ref().clone(),
        ))
    }

    pub fn to_address(&self) -> Address {
        Address(self.0.clone().to_address())
    }

    pub fn from_address(address: &Address) -> Option<RewardAddress> {
        match &address.0 {
            cml_chain::address::Address::Reward(ra) => Some(ra.clone().into()),
            _ => None,
        }
    }

    pub fn network_id(&self) -> u8 {
        self.0.network
    }

    pub fn payment(&self) -> StakeCredential {
        self.0.payment.clone().into()
    }
}
