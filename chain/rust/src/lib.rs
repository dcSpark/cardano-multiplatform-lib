use std::io::{BufRead, Seek, Write};

use cbor_event::Special as CBORSpecial;
use cbor_event::Type as CBORType;
use cbor_event::{self, de::Deserializer, se::Serializer};
use std::collections::BTreeMap;
use std::convert::{From, TryFrom};

pub mod address;
pub mod auxdata;
pub mod crypto;
pub mod serialization;

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

// This library was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

// TODO: for regen, change babbage's cddl to have our own names in the first place
pub type AddrKeyhash = Ed25519KeyHash;

pub type BoundedBytes = Vec<u8>;

pub type Coin = u64;

// see comment on ScriptRef declaration
pub type Data = Vec<u8>;

// TODO: for regen, change babbage's cddl to have our own names in the first place
pub type DatumHash = DataHash;

pub type DeltaCoin = Int;

pub type Genesishashs = Vec<GenesisHash>;

// TODO: fix cddl-codegen to avoid generating this and make it a direct alias not a declared one
pub type Int64 = i64;

pub type Mint = OrderedHashMap<PolicyId, OrderedHashMap<AssetName, Int64>>;

pub type Multiasset = OrderedHashMap<PolicyId, OrderedHashMap<AssetName, u64>>;

// TODO for regen: why was this in babbage/crypto.cddl? it's not in babbage.cddl.
pub type Natural = Vec<u8>;

// we might want dedicated types for this anyway?
pub type PlutusV1Script = Vec<u8>;

pub type PlutusV2Script = Vec<u8>;

pub type PolicyId = ScriptHash;

pub type PolicyIds = Vec<PolicyId>;

pub type Port = u16;

pub type ProposedProtocolParameterUpdates = OrderedHashMap<GenesisHash, ProtocolParamUpdate>;

pub type RewardAccounts = Vec<RewardAccount>;

// TODO: this should NOT be generating an alias! (it's both tagged and a .cbor script)
// we should investigate cddl-codegen and open an issue there if it's still happening
// (top-level .cbor was recently being worked on and this is tagged but even then it should NOT be bytes here)
pub type ScriptRef = Vec<u8>;

pub type ShelleyAuxData = OrderedHashMap<TransactionMetadatumLabel, TransactionMetadatum>;

pub type SubCoin = PositiveInterval;

pub type Withdrawals = OrderedHashMap<RewardAccount, Coin>;

// for enums + byron. see comment in Cargo.toml
use wasm_bindgen::prelude::wasm_bindgen;

pub use address::*;

pub mod block;

pub use block::*;

pub mod certs;

pub use certs::*;

pub mod plutus;

pub use plutus::*;

pub mod transaction;

pub use transaction::*;
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, Derivative)]
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct DatumOption0 {
    pub hash32: DataHash,
    #[serde(skip)]
    pub encodings: Option<DatumOption0Encoding>,
}

impl DatumOption0 {
    pub fn new(hash32: DataHash) -> Self {
        Self {
            hash32,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct DatumOption1 {
    pub data: Data,
    #[serde(skip)]
    pub encodings: Option<DatumOption1Encoding>,
}

impl DatumOption1 {
    pub fn new(data: Data) -> Self {
        Self {
            data,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum I0OrI1 {
    I0 {
        #[serde(skip)]
        i0_encoding: Option<cbor_event::Sz>,
    },
    I1 {
        #[serde(skip)]
        i1_encoding: Option<cbor_event::Sz>,
    },
}

impl I0OrI1 {
    pub fn new_i0() -> Self {
        Self::I0 { i0_encoding: None }
    }

    pub fn new_i1() -> Self {
        Self::I1 { i1_encoding: None }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum NetworkId {
    I0 {
        #[serde(skip)]
        i0_encoding: Option<cbor_event::Sz>,
    },
    I1 {
        #[serde(skip)]
        i1_encoding: Option<cbor_event::Sz>,
    },
}

impl NetworkId {
    pub fn new_i0() -> Self {
        Self::I0 { i0_encoding: None }
    }

    pub fn new_i1() -> Self {
        Self::I1 { i1_encoding: None }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct PositiveInterval {
    #[serde(skip)]
    pub encodings: Option<PositiveIntervalEncoding>,
}

impl PositiveInterval {
    pub fn new() -> Self {
        Self { encodings: None }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ProtocolParamUpdate {
    pub key_0: Option<u64>,
    pub key_1: Option<u64>,
    pub key_2: Option<u64>,
    pub key_3: Option<u64>,
    pub key_4: Option<u64>,
    pub key_5: Option<Coin>,
    pub key_6: Option<Coin>,
    pub key_7: Option<Epoch>,
    pub key_8: Option<u64>,
    pub key_9: Option<Rational>,
    pub key_10: Option<UnitInterval>,
    pub key_11: Option<UnitInterval>,
    pub key_14: Option<ProtocolVersionStruct>,
    pub key_16: Option<Coin>,
    pub key_17: Option<Coin>,
    pub key_18: Option<Costmdls>,
    pub key_19: Option<ExUnitPrices>,
    pub key_20: Option<ExUnits>,
    pub key_21: Option<ExUnits>,
    pub key_22: Option<u64>,
    pub key_23: Option<u64>,
    pub key_24: Option<u64>,
    #[serde(skip)]
    pub encodings: Option<ProtocolParamUpdateEncoding>,
}

impl ProtocolParamUpdate {
    pub fn new() -> Self {
        Self {
            key_0: None,
            key_1: None,
            key_2: None,
            key_3: None,
            key_4: None,
            key_5: None,
            key_6: None,
            key_7: None,
            key_8: None,
            key_9: None,
            key_10: None,
            key_11: None,
            key_14: None,
            key_16: None,
            key_17: None,
            key_18: None,
            key_19: None,
            key_20: None,
            key_21: None,
            key_22: None,
            key_23: None,
            key_24: None,
            encodings: None,
        }
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Script0 {
    pub native_script: NativeScript,
    #[serde(skip)]
    pub encodings: Option<Script0Encoding>,
}

impl Script0 {
    pub fn new(native_script: NativeScript) -> Self {
        Self {
            native_script,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Script1 {
    pub plutus_v1_script: PlutusV1Script,
    #[serde(skip)]
    pub encodings: Option<Script1Encoding>,
}

impl Script1 {
    pub fn new(plutus_v1_script: PlutusV1Script) -> Self {
        Self {
            plutus_v1_script,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct Script2 {
    pub plutus_v2_script: PlutusV2Script,
    #[serde(skip)]
    pub encodings: Option<Script2Encoding>,
}

impl Script2 {
    pub fn new(plutus_v2_script: PlutusV2Script) -> Self {
        Self {
            plutus_v2_script,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct UnitInterval {
    pub index_0: u64,
    pub index_1: u64,
    #[serde(skip)]
    pub encodings: Option<UnitIntervalEncoding>,
}

impl UnitInterval {
    pub fn new(index_0: u64, index_1: u64) -> Self {
        Self {
            index_0,
            index_1,
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
