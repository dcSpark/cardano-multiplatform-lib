use crate::byron::utils::ProtocolMagic;
use cml_core_wasm::impl_wasm_conversions;
use cml_crypto_wasm::TransactionHash;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct NetworkInfo(cml_chain::genesis::network_info::NetworkInfo);

impl_wasm_conversions!(cml_chain::genesis::network_info::NetworkInfo, NetworkInfo);

#[wasm_bindgen]
impl NetworkInfo {
    pub fn new(network_id: u8, protocol_magic: &ProtocolMagic) -> Self {
        cml_chain::genesis::network_info::NetworkInfo::new(network_id, *protocol_magic.as_ref())
            .into()
    }

    pub fn network_id(&self) -> u8 {
        self.0.network_id()
    }

    pub fn protocol_magic(&self) -> ProtocolMagic {
        self.0.protocol_magic().into()
    }

    /// This is the old testnet - most likely you want to use preview()/preprod()
    pub fn testnet() -> Self {
        cml_chain::genesis::network_info::NetworkInfo::testnet().into()
    }

    pub fn mainnet() -> Self {
        cml_chain::genesis::network_info::NetworkInfo::mainnet().into()
    }

    pub fn preview() -> Self {
        cml_chain::genesis::network_info::NetworkInfo::preview().into()
    }

    pub fn preprod() -> Self {
        cml_chain::genesis::network_info::NetworkInfo::preprod().into()
    }

    pub fn sancho_testnet() -> Self {
        cml_chain::genesis::network_info::NetworkInfo::sancho_testnet().into()
    }
}

#[wasm_bindgen]
pub struct ByronGenesisRedeem(
    pub(crate) (cml_crypto::TransactionHash, cml_chain::byron::ByronAddress),
);

#[wasm_bindgen]
impl ByronGenesisRedeem {
    pub fn new(txid: &TransactionHash, address: &crate::byron::ByronAddress) -> Self {
        ByronGenesisRedeem((txid.clone().into(), address.clone().into()))
    }

    pub fn txid(&self) -> TransactionHash {
        self.0 .0.into()
    }

    pub fn address(&self) -> crate::byron::ByronAddress {
        self.0 .1.clone().into()
    }
}

#[wasm_bindgen]
pub fn genesis_txid_byron(
    pubkey: &cml_crypto_wasm::PublicKey,
    protocol_magic: Option<u32>,
) -> ByronGenesisRedeem {
    let redeem = cml_chain::genesis::byron::parse::redeem_pubkey_to_txid(
        &pubkey.clone().as_ref().0,
        protocol_magic.map(|pm| ProtocolMagic::new(pm).into()),
    );
    ByronGenesisRedeem(redeem)
}

#[wasm_bindgen]
pub fn genesis_txid_shelley(address: &crate::address::Address) -> TransactionHash {
    TransactionHash::from(cml_chain::genesis::shelley::parse::redeem_address_to_txid(
        &address.clone().into(),
    ))
}
