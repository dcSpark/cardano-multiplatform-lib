#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct AlonzoTxOut {
    pub address: Address,
    pub amount: Value,
    pub datum_hash: Hash32,
    #[serde(skip)]
    pub encodings: Option<AlonzoTxOutEncoding>,
}

impl AlonzoTxOut {
    pub fn new(address: Address, amount: Value, datum_hash: Hash32) -> Self {
        Self {
            address,
            amount,
            datum_hash,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BabbageTxOut {
    pub key_0: Address,
    pub key_1: Value,
    pub key_2: Option<DatumOption>,
    pub key_3: Option<ScriptRef>,
    #[serde(skip)]
    pub encodings: Option<BabbageTxOutEncoding>,
}

impl BabbageTxOut {
    pub fn new(key_0: Address, key_1: Value) -> Self {
        Self {
            key_0,
            key_1,
            key_2: None,
            key_3: None,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum DatumOption {
    DatumOption0(DatumOption0),
    DatumOption1(DatumOption1),
}

impl DatumOption {
    pub fn new_datum_option0(hash32: Hash32) -> Self {
        Self::DatumOption0(DatumOption0::new(hash32))
    }

    pub fn new_datum_option1(data: Data) -> Self {
        Self::DatumOption1(DatumOption1::new(data))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct InvalidBefore {
    pub index_1: u64,
    #[serde(skip)]
    pub encodings: Option<InvalidBeforeEncoding>,
}

impl InvalidBefore {
    pub fn new(index_1: u64) -> Self {
        Self {
            index_1,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct InvalidHereafter {
    pub index_1: u64,
    #[serde(skip)]
    pub encodings: Option<InvalidHereafterEncoding>,
}

impl InvalidHereafter {
    pub fn new(index_1: u64) -> Self {
        Self {
            index_1,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum NativeScript {
    ScriptPubkey(ScriptPubkey),
    ScriptAll(ScriptAll),
    ScriptAny(ScriptAny),
    ScriptNOfK(ScriptNOfK),
    InvalidBefore(InvalidBefore),
    InvalidHereafter(InvalidHereafter),
}

impl NativeScript {
    pub fn new_script_pubkey(addr_keyhash: AddrKeyhash) -> Self {
        Self::ScriptPubkey(ScriptPubkey::new(addr_keyhash))
    }

    pub fn new_script_all(native_scripts: Vec<NativeScript>) -> Self {
        Self::ScriptAll(ScriptAll::new(native_scripts))
    }

    pub fn new_script_any(native_scripts: Vec<NativeScript>) -> Self {
        Self::ScriptAny(ScriptAny::new(native_scripts))
    }

    pub fn new_script_n_of_k(n: u64, native_scripts: Vec<NativeScript>) -> Self {
        Self::ScriptNOfK(ScriptNOfK::new(n, native_scripts))
    }

    pub fn new_invalid_before(index_1: u64) -> Self {
        Self::InvalidBefore(InvalidBefore::new(index_1))
    }

    pub fn new_invalid_hereafter(index_1: u64) -> Self {
        Self::InvalidHereafter(InvalidHereafter::new(index_1))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct RequiredSigners {
    pub addr_keyhash: AddrKeyhash,
    #[serde(skip)]
    pub encodings: Option<RequiredSignersEncoding>,
}

impl RequiredSigners {
    pub fn new(addr_keyhash: AddrKeyhash) -> Self {
        Self {
            addr_keyhash,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Script {
    Script0(Script0),
    Script1(Script1),
    Script2(Script2),
}

impl Script {
    pub fn new_script0(native_script: NativeScript) -> Self {
        Self::Script0(Script0::new(native_script))
    }

    pub fn new_script1(plutus_v1_script: PlutusV1Script) -> Self {
        Self::Script1(Script1::new(plutus_v1_script))
    }

    pub fn new_script2(plutus_v2_script: PlutusV2Script) -> Self {
        Self::Script2(Script2::new(plutus_v2_script))
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ScriptAll {
    pub native_scripts: Vec<NativeScript>,
    #[serde(skip)]
    pub encodings: Option<ScriptAllEncoding>,
}

impl ScriptAll {
    pub fn new(native_scripts: Vec<NativeScript>) -> Self {
        Self {
            native_scripts,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ScriptAny {
    pub native_scripts: Vec<NativeScript>,
    #[serde(skip)]
    pub encodings: Option<ScriptAnyEncoding>,
}

impl ScriptAny {
    pub fn new(native_scripts: Vec<NativeScript>) -> Self {
        Self {
            native_scripts,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ScriptNOfK {
    pub n: u64,
    pub native_scripts: Vec<NativeScript>,
    #[serde(skip)]
    pub encodings: Option<ScriptNOfKEncoding>,
}

impl ScriptNOfK {
    pub fn new(n: u64, native_scripts: Vec<NativeScript>) -> Self {
        Self {
            n,
            native_scripts,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ScriptPubkey {
    pub addr_keyhash: AddrKeyhash,
    #[serde(skip)]
    pub encodings: Option<ScriptPubkeyEncoding>,
}

impl ScriptPubkey {
    pub fn new(addr_keyhash: AddrKeyhash) -> Self {
        Self {
            addr_keyhash,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ShelleyTxOut {
    pub address: Address,
    pub amount: Value,
    #[serde(skip)]
    pub encodings: Option<ShelleyTxOutEncoding>,
}

impl ShelleyTxOut {
    pub fn new(address: Address, amount: Value) -> Self {
        Self {
            address,
            amount,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Transaction {
    pub transaction_body: TransactionBody,
    pub transaction_witness_set: TransactionWitnessSet,
    pub index_2: bool,
    pub auxiliary_data: Option<AuxiliaryData>,
    #[serde(skip)]
    pub encodings: Option<TransactionEncoding>,
}

impl Transaction {
    pub fn new(transaction_body: TransactionBody, transaction_witness_set: TransactionWitnessSet, index_2: bool, auxiliary_data: Option<AuxiliaryData>) -> Self {
        Self {
            transaction_body,
            transaction_witness_set,
            index_2,
            auxiliary_data,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct TransactionBody {
    pub key_0: Vec<TransactionInput>,
    pub key_1: Vec<TransactionOutput>,
    pub key_2: Coin,
    pub key_3: Option<u64>,
    pub key_4: Option<Vec<Certificate>>,
    pub key_5: Option<Withdrawals>,
    pub key_6: Option<Update>,
    pub key_7: Option<AuxiliaryDataHash>,
    pub key_8: Option<u64>,
    pub key_9: Option<Mint>,
    pub key_11: Option<ScriptDataHash>,
    pub key_13: Option<Vec<TransactionInput>>,
    pub key_14: Option<RequiredSigners>,
    pub key_15: Option<NetworkId>,
    pub key_16: Option<TransactionOutput>,
    pub key_17: Option<Coin>,
    pub key_18: Option<Vec<TransactionInput>>,
    #[serde(skip)]
    pub encodings: Option<TransactionBodyEncoding>,
}

impl TransactionBody {
    pub fn new(key_0: Vec<TransactionInput>, key_1: Vec<TransactionOutput>, key_2: Coin) -> Self {
        Self {
            key_0,
            key_1,
            key_2,
            key_3: None,
            key_4: None,
            key_5: None,
            key_6: None,
            key_7: None,
            key_8: None,
            key_9: None,
            key_11: None,
            key_13: None,
            key_14: None,
            key_15: None,
            key_16: None,
            key_17: None,
            key_18: None,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct TransactionInput {
    pub transaction_id: Hash32,
    pub index: u64,
    #[serde(skip)]
    pub encodings: Option<TransactionInputEncoding>,
}

impl TransactionInput {
    pub fn new(transaction_id: Hash32, index: u64) -> Self {
        Self {
            transaction_id,
            index,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum TransactionOutput {
    ShelleyTxOut(ShelleyTxOut),
    AlonzoTxOut(AlonzoTxOut),
    BabbageTxOut(BabbageTxOut),
}

impl TransactionOutput {
    pub fn new_shelley_tx_out(shelley_tx_out: ShelleyTxOut) -> Self {
        Self::ShelleyTxOut(shelley_tx_out)
    }

    pub fn new_alonzo_tx_out(alonzo_tx_out: AlonzoTxOut) -> Self {
        Self::AlonzoTxOut(alonzo_tx_out)
    }

    pub fn new_babbage_tx_out(babbage_tx_out: BabbageTxOut) -> Self {
        Self::BabbageTxOut(babbage_tx_out)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct TransactionWitnessSet {
    pub key_0: Option<Vec<Vkeywitness>>,
    pub key_1: Option<Vec<NativeScript>>,
    pub key_2: Option<Vec<BootstrapWitness>>,
    pub key_3: Option<Vec<PlutusV1Script>>,
    pub key_4: Option<Vec<PlutusData>>,
    pub key_5: Option<Vec<Redeemer>>,
    pub key_6: Option<Vec<PlutusV2Script>>,
    #[serde(skip)]
    pub encodings: Option<TransactionWitnessSetEncoding>,
}

impl TransactionWitnessSet {
    pub fn new() -> Self {
        Self {
            key_0: None,
            key_1: None,
            key_2: None,
            key_3: None,
            key_4: None,
            key_5: None,
            key_6: None,
            encodings: None,
        }
    }
}

use super::*;