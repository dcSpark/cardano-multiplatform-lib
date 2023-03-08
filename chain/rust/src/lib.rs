use std::io::{BufRead, Seek, Write};

use cbor_event::Special as CBORSpecial;
use cbor_event::Type as CBORType;
use cbor_event::{self, de::Deserializer, se::Serializer};
use std::collections::BTreeMap;
use std::convert::{From, TryFrom};

pub mod address;
pub mod auxdata;
pub mod block;
pub mod certs;
pub mod crypto;
pub mod plutus;
pub mod serialization;
pub mod transaction;

use address::*;
use auxdata::*;

use crypto::*;

//pub mod legacy_address;

pub use cml_core::{
    error::{DeserializeError, DeserializeFailure},
    metadata::{TransactionMetadatum, TransactionMetadatumLabel},
    ordered_hash_map::OrderedHashMap,
    serialization::{Deserialize, LenEncoding, Serialize, StringEncoding},
    CertificateIndex, Epoch, Int, Slot, TransactionIndex,
};

pub mod cbor_encodings;

use cbor_encodings::*;

extern crate derivative;

pub(crate) use derivative::Derivative;
//#![allow(clippy::too_many_arguments)]

// TODO: replace with real bigint type
pub type BigInt = Int;
// TODO: same ^
pub type BoundedBytes = Int;

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use address::RewardAccount;
use block::ProtocolVersion;
use cbor_encodings::{
    AssetNameEncoding, BootstrapWitnessEncoding, PositiveIntervalEncoding,
    ProtocolParamUpdateEncoding, ProtocolVersionStructEncoding, RationalEncoding,
    UnitIntervalEncoding, UpdateEncoding, ValueEncoding, VkeywitnessEncoding,
};
use crypto::{Ed25519Signature, GenesisHash, Vkey};
use plutus::{CostModels, ExUnitPrices, ExUnits, PlutusV1Script, PlutusV2Script};
use transaction::NativeScript;

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AssetName {
    pub inner: Vec<u8>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    #[serde(skip)]
    pub encodings: Option<AssetNameEncoding>,
}

impl AssetName {
    pub fn get(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn new(inner: Vec<u8>) -> Result<Self, DeserializeError> {
        if inner.len() > 32 {
            return Err(DeserializeError::new(
                "AssetName",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(0),
                    max: Some(32),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: None,
        })
    }
}

impl TryFrom<Vec<u8>> for AssetName {
    type Error = DeserializeError;

    fn try_from(inner: Vec<u8>) -> Result<Self, Self::Error> {
        AssetName::new(inner)
    }
}

impl From<AssetName> for Vec<u8> {
    fn from(wrapper: AssetName) -> Self {
        wrapper.inner
    }
}

pub type Coin = u64;

pub type DeltaCoin = Int;

pub type GenesisHashList = Vec<GenesisHash>;

pub type Mint = OrderedHashMap<PolicyId, OrderedHashMap<AssetName, i64>>;

pub type Multiasset = OrderedHashMap<PolicyId, OrderedHashMap<AssetName, u64>>;

pub type NetworkId = u8;

pub type PolicyId = ScriptHash;

pub type PolicyIdList = Vec<PolicyId>;

pub type Port = u16;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct PositiveInterval {
    pub strart: u64,
    pub end: u64,
    #[serde(skip)]
    pub encodings: Option<PositiveIntervalEncoding>,
}

impl PositiveInterval {
    pub fn new(strart: u64, end: u64) -> Self {
        Self {
            strart,
            end,
            encodings: None,
        }
    }
}

pub type ProposedProtocolParameterUpdates = OrderedHashMap<GenesisHash, ProtocolParamUpdate>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ProtocolParamUpdate {
    pub minfee_a: Option<u64>,
    pub minfee_b: Option<u64>,
    pub max_block_body_size: Option<u64>,
    pub max_transaction_size: Option<u64>,
    pub max_block_header_size: Option<u64>,
    pub key_deposit: Option<Coin>,
    pub pool_deposit: Option<Coin>,
    pub maximum_epoch: Option<Epoch>,
    pub n_opt: Option<u64>,
    pub pool_pledge_influence: Option<Rational>,
    pub expansion_rate: Option<UnitInterval>,
    pub treasury_growth_rate: Option<UnitInterval>,
    pub protocol_version: Option<ProtocolVersionStruct>,
    pub min_pool_cost: Option<Coin>,
    pub ada_per_utxo_byte: Option<Coin>,
    pub cost_models_for_script_languages: Option<CostModels>,
    pub execution_costs: Option<ExUnitPrices>,
    pub max_tx_ex_units: Option<ExUnits>,
    pub max_block_ex_units: Option<ExUnits>,
    pub max_value_size: Option<u64>,
    pub collateral_percentage: Option<u64>,
    pub max_collateral_inputs: Option<u64>,
    #[serde(skip)]
    pub encodings: Option<ProtocolParamUpdateEncoding>,
}

impl ProtocolParamUpdate {
    pub fn new() -> Self {
        Self {
            minfee_a: None,
            minfee_b: None,
            max_block_body_size: None,
            max_transaction_size: None,
            max_block_header_size: None,
            key_deposit: None,
            pool_deposit: None,
            maximum_epoch: None,
            n_opt: None,
            pool_pledge_influence: None,
            expansion_rate: None,
            treasury_growth_rate: None,
            protocol_version: None,
            min_pool_cost: None,
            ada_per_utxo_byte: None,
            cost_models_for_script_languages: None,
            execution_costs: None,
            max_tx_ex_units: None,
            max_block_ex_units: None,
            max_value_size: None,
            collateral_percentage: None,
            max_collateral_inputs: None,
            encodings: None,
        }
    }
}

impl Default for ProtocolParamUpdate {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ProtocolVersionStruct {
    pub protocol_version: ProtocolVersion,
    #[serde(skip)]
    pub encodings: Option<ProtocolVersionStructEncoding>,
}

impl ProtocolVersionStruct {
    pub fn new(protocol_version: ProtocolVersion) -> Self {
        Self {
            protocol_version,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Rational {
    pub numerator: u64,
    pub denominator: u64,
    #[serde(skip)]
    pub encodings: Option<RationalEncoding>,
}

impl Rational {
    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            numerator,
            denominator,
            encodings: None,
        }
    }
}

pub type RewardAccountList = Vec<RewardAccount>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Script {
    Native {
        script: NativeScript,
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[serde(skip)]
        tag_encoding: Option<cbor_event::Sz>,
    },
    PlutusV1 {
        script: PlutusV1Script,
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[serde(skip)]
        tag_encoding: Option<cbor_event::Sz>,
    },
    PlutusV2 {
        script: PlutusV2Script,
        #[serde(skip)]
        len_encoding: LenEncoding,
        #[serde(skip)]
        tag_encoding: Option<cbor_event::Sz>,
    },
}

impl Script {
    pub fn new_native(script: NativeScript) -> Self {
        Self::Native {
            script,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
        }
    }

    pub fn new_plutus_v1(script: PlutusV1Script) -> Self {
        Self::PlutusV1 {
            script,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
        }
    }

    pub fn new_plutus_v2(script: PlutusV2Script) -> Self {
        Self::PlutusV2 {
            script,
            len_encoding: LenEncoding::default(),
            tag_encoding: None,
        }
    }
}

pub type SubCoin = PositiveInterval;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct UnitInterval {
    pub start: u64,
    pub end: u64,
    #[serde(skip)]
    pub encodings: Option<UnitIntervalEncoding>,
}

impl UnitInterval {
    pub fn new(start: u64, end: u64) -> Self {
        Self {
            start,
            end,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Update {
    pub proposed_protocol_parameter_updates: ProposedProtocolParameterUpdates,
    pub epoch: Epoch,
    #[serde(skip)]
    pub encodings: Option<UpdateEncoding>,
}

impl Update {
    pub fn new(
        proposed_protocol_parameter_updates: ProposedProtocolParameterUpdates,
        epoch: Epoch,
    ) -> Self {
        Self {
            proposed_protocol_parameter_updates,
            epoch,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Value {
    pub coin: Coin,
    pub multiasset: Multiasset,
    #[serde(skip)]
    pub encodings: Option<ValueEncoding>,
}

impl Value {
    pub fn new(coin: Coin, multiasset: Multiasset) -> Self {
        Self {
            coin,
            multiasset,
            encodings: None,
        }
    }
}

pub type Withdrawals = OrderedHashMap<RewardAccount, Coin>;
