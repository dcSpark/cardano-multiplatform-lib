use crate::{
    byron::ProtocolMagic,
    plutus::CostModels
};
use cml_core::Int;

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
            protocol_magic: ProtocolMagic::from(1097911063)
        }
    }
    pub fn mainnet() -> NetworkInfo {
        NetworkInfo {
            network_id: 0b0001,
            protocol_magic: ProtocolMagic::from(764824073)
        }
    }
}

// TODO: https://github.com/dcSpark/cardano-multiplatform-lib/issues/92
pub fn plutus_alonzo_cost_models() -> CostModels {
    let ops: [u64; 166] = [197209, 0, 1, 1, 396231, 621, 0, 1, 150000, 1000, 0, 1, 150000, 32, 2477736, 29175, 4, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 100, 100, 29773, 100, 150000, 32, 150000, 32, 150000, 32, 150000, 1000, 0, 1, 150000, 32, 150000, 1000, 0, 8, 148000, 425507, 118, 0, 1, 1, 150000, 1000, 0, 8, 150000, 112536, 247, 1, 150000, 10000, 1, 136542, 1326, 1, 1000, 150000, 1000, 1, 150000, 32, 150000, 32, 150000, 32, 1, 1, 150000, 1, 150000, 4, 103599, 248, 1, 103599, 248, 1, 145276, 1366, 1, 179690, 497, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 148000, 425507, 118, 0, 1, 1, 61516, 11218, 0, 1, 150000, 32, 148000, 425507, 118, 0, 1, 1, 148000, 425507, 118, 0, 1, 1, 2477736, 29175, 4, 0, 82363, 4, 150000, 5000, 0, 1, 150000, 32, 197209, 0, 1, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 3345831, 1, 1];

    let mut res = CostModels::new();
    res.plutus_v1 = Some(ops.iter().map(|&i| Int::from(i)).collect());
    res
}
