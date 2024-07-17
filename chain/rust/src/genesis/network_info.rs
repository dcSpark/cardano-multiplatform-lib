use crate::{
    byron::ProtocolMagic,
    plutus::{CostModels, Language},
};
use cml_core::network::{
    BYRON_MAINNET_NETWORK_MAGIC, BYRON_TESTNET_NETWORK_MAGIC, PREPROD_NETWORK_MAGIC,
    PREVIEW_NETWORK_MAGIC, SANCHO_TESTNET_NETWORK_MAGIC,
};

#[derive(
    Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
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

    /// This is the old testnet - most likely you want to use preview()/preprod()
    pub fn testnet() -> Self {
        Self {
            network_id: 0b0000,
            protocol_magic: ProtocolMagic::from(BYRON_TESTNET_NETWORK_MAGIC),
        }
    }

    pub fn mainnet() -> Self {
        Self {
            network_id: 0b0001,
            protocol_magic: ProtocolMagic::from(BYRON_MAINNET_NETWORK_MAGIC),
        }
    }

    pub fn preview() -> Self {
        Self {
            network_id: 0b0000,
            protocol_magic: ProtocolMagic::from(PREVIEW_NETWORK_MAGIC),
        }
    }

    pub fn preprod() -> Self {
        Self {
            network_id: 0b0000,
            protocol_magic: ProtocolMagic::from(PREPROD_NETWORK_MAGIC),
        }
    }

    pub fn sancho_testnet() -> Self {
        Self {
            network_id: 0b0000,
            protocol_magic: ProtocolMagic::from(SANCHO_TESTNET_NETWORK_MAGIC),
        }
    }
}

// TODO: https://github.com/dcSpark/cardano-multiplatform-lib/issues/92
pub fn plutus_alonzo_cost_models() -> CostModels {
    let ops = vec![
        197209, 0, 1, 1, 396231, 621, 0, 1, 150000, 1000, 0, 1, 150000, 32, 2477736, 29175, 4,
        29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 100, 100, 29773,
        100, 150000, 32, 150000, 32, 150000, 32, 150000, 1000, 0, 1, 150000, 32, 150000, 1000, 0,
        8, 148000, 425507, 118, 0, 1, 1, 150000, 1000, 0, 8, 150000, 112536, 247, 1, 150000, 10000,
        1, 136542, 1326, 1, 1000, 150000, 1000, 1, 150000, 32, 150000, 32, 150000, 32, 1, 1,
        150000, 1, 150000, 4, 103599, 248, 1, 103599, 248, 1, 145276, 1366, 1, 179690, 497, 1,
        150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 148000, 425507,
        118, 0, 1, 1, 61516, 11218, 0, 1, 150000, 32, 148000, 425507, 118, 0, 1, 1, 148000, 425507,
        118, 0, 1, 1, 2477736, 29175, 4, 0, 82363, 4, 150000, 5000, 0, 1, 150000, 32, 197209, 0, 1,
        1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32,
        3345831, 1, 1,
    ];

    let mut res = CostModels::default();
    res.inner.insert(Language::PlutusV1 as u64, ops);
    res
}
