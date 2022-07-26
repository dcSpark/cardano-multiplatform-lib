use crate::byron::ProtocolMagic;

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
