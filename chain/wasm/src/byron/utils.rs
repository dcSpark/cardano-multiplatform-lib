use cml_crypto_wasm::{
    impl_hash_type_ext, Bip32PrivateKey, Bip32PublicKey, LegacyDaedalusPrivateKey, PublicKey,
    TransactionHash,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{
    address::Address,
    byron::{AddrAttributes, ByronAddress, HDAddressPayload, SpendingData},
    crypto::BootstrapWitness,
};
// this is alredy wasm-exposed since enum
pub use cml_chain::byron::ByronAddrType;

use super::AddressContent;

impl_hash_type_ext!(cml_chain::byron::AddressId, AddressId);
// not sure if this is a hash but it likely is and has the same byte format
impl_hash_type_ext!(cml_chain::byron::ByronScript, ByronScript);
impl_hash_type_ext!(cml_chain::byron::StakeholderId, StakeholderId);

#[wasm_bindgen]
impl StakeholderId {
    pub fn new(pubk: &Bip32PublicKey) -> StakeholderId {
        Self(cml_chain::byron::StakeholderId::new(pubk.as_ref()))
    }
}

#[wasm_bindgen]
impl AddrAttributes {
    pub fn new_bootstrap_era(
        hdap: Option<HDAddressPayload>,
        protocol_magic: Option<ProtocolMagic>,
    ) -> Self {
        cml_chain::byron::AddrAttributes::new_bootstrap_era(
            hdap.map(Into::into),
            protocol_magic.map(Into::into),
        )
        .into()
    }

    pub fn new_single_key(
        pubk: &Bip32PublicKey,
        hdap: Option<HDAddressPayload>,
        protocol_magic: ProtocolMagic,
    ) -> Self {
        cml_chain::byron::AddrAttributes::new_single_key(
            pubk.as_ref(),
            hdap.map(Into::into),
            protocol_magic.into(),
        )
        .into()
    }
}

#[wasm_bindgen]
impl AddressId {
    pub fn new(
        addr_type: ByronAddrType,
        spending_data: &SpendingData,
        attrs: &AddrAttributes,
    ) -> Self {
        cml_chain::byron::AddressId::new(addr_type, spending_data.as_ref(), attrs.as_ref()).into()
    }
}

#[wasm_bindgen]
impl AddressContent {
    pub fn hash_and_create(
        addr_type: ByronAddrType,
        spending_data: &SpendingData,
        attributes: &AddrAttributes,
    ) -> AddressContent {
        cml_chain::byron::AddressContent::hash_and_create(
            addr_type,
            spending_data.as_ref(),
            attributes.clone().into(),
        )
        .into()
    }

    // bootstrap era + no hdpayload address
    pub fn new_redeem(pubkey: &PublicKey, protocol_magic: Option<ProtocolMagic>) -> Self {
        cml_chain::byron::AddressContent::new_redeem(
            pubkey.clone().into(),
            protocol_magic.map(Into::into),
        )
        .into()
    }

    // bootstrap era + no hdpayload address
    pub fn new_simple(xpub: &Bip32PublicKey, protocol_magic: Option<ProtocolMagic>) -> Self {
        cml_chain::byron::AddressContent::new_simple(
            xpub.clone().into(),
            protocol_magic.map(Into::into),
        )
        .into()
    }

    /// Do we want to remove this or keep it for people who were using old Byron code?
    pub fn to_address(&self) -> ByronAddress {
        self.0.to_address().into()
    }

    /// returns the byron protocol magic embedded in the address, or mainnet id if none is present
    /// note: for bech32 addresses, you need to use network_id instead
    pub fn byron_protocol_magic(&self) -> ProtocolMagic {
        self.0.byron_protocol_magic().into()
    }

    pub fn network_id(&self) -> Result<u8, JsError> {
        self.0.network_id().map_err(Into::into)
    }

    // icarus-style address (Ae2)
    pub fn icarus_from_key(key: &Bip32PublicKey, protocol_magic: &ProtocolMagic) -> AddressContent {
        cml_chain::byron::AddressContent::icarus_from_key(
            key.clone().into(),
            (*protocol_magic).into(),
        )
        .into()
    }

    /// Check if the Addr can be reconstructed with a specific xpub
    pub fn identical_with_pubkey(&self, xpub: &Bip32PublicKey) -> bool {
        self.0.identical_with_pubkey(xpub.clone().into())
    }
}

#[wasm_bindgen]
impl ByronAddress {
    pub fn to_base58(&self) -> String {
        self.0.to_base58()
    }

    pub fn from_base58(s: &str) -> Result<ByronAddress, JsError> {
        cml_chain::byron::ByronAddress::from_base58(s)
            .map(Self)
            .map_err(|e| JsError::new(&format!("ByronAddress::from_base58: {e:?}")))
    }

    pub fn is_valid(s: &str) -> bool {
        cml_chain::byron::ByronAddress::is_valid(s)
    }

    pub fn to_address(&self) -> Address {
        self.0.clone().to_address().into()
    }

    pub fn from_address(addr: &Address) -> Option<ByronAddress> {
        cml_chain::byron::ByronAddress::from_address(addr.as_ref()).map(Self)
    }

    pub fn from_address_content(address_content: &AddressContent) -> Self {
        cml_chain::byron::ByronAddress::from(address_content.as_ref().clone()).into()
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct ProtocolMagic(cml_chain::byron::ProtocolMagic);

#[wasm_bindgen]
impl ProtocolMagic {
    pub fn new(pm: u32) -> Self {
        Self(pm.into())
    }

    pub fn to_int(&self) -> u32 {
        self.0.into()
    }
}

impl From<cml_chain::byron::ProtocolMagic> for ProtocolMagic {
    fn from(native: cml_chain::byron::ProtocolMagic) -> Self {
        Self(native)
    }
}

impl From<ProtocolMagic> for cml_chain::byron::ProtocolMagic {
    fn from(wasm: ProtocolMagic) -> Self {
        wasm.0
    }
}

impl AsRef<cml_chain::byron::ProtocolMagic> for ProtocolMagic {
    fn as_ref(&self) -> &cml_chain::byron::ProtocolMagic {
        &self.0
    }
}

#[wasm_bindgen]
pub fn make_daedalus_bootstrap_witness(
    tx_body_hash: &TransactionHash,
    addr: &ByronAddress,
    key: &LegacyDaedalusPrivateKey,
) -> BootstrapWitness {
    cml_chain::byron::make_daedalus_bootstrap_witness(
        tx_body_hash.clone().into(),
        addr.clone().into(),
        key.clone().into(),
    )
    .into()
}

#[wasm_bindgen]
pub fn make_icarus_bootstrap_witness(
    tx_body_hash: TransactionHash,
    addr: ByronAddress,
    key: &Bip32PrivateKey,
) -> BootstrapWitness {
    cml_chain::byron::make_icarus_bootstrap_witness(tx_body_hash.into(), addr.into(), key.as_ref())
        .into()
}
