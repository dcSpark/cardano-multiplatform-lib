use crate::byron::utils::ProtocolMagic;
use cml_core_wasm::impl_wasm_conversions;
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
