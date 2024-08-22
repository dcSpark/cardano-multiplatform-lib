// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::byron::{
    Blake2b256, ByronAny, ByronAnyList, ByronPubKey, ByronSignature, ByronTxId, ByronTxInList,
    ByronTxOutList,
};
use cml_core_wasm::{
    impl_wasm_cbor_json_api_cbor_event_serialize, impl_wasm_conversions, impl_wasm_map_btree,
};
use wasm_bindgen::prelude::wasm_bindgen;

impl_wasm_map_btree!(
    cml_multi_era::byron::ByronAny,
    cml_multi_era::byron::ByronAny,
    ByronAny,
    ByronAny,
    ByronAnyList,
    ByronAttributes
);

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronPkWitness(cml_multi_era::byron::transaction::ByronPkWitness);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronPkWitness);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronPkWitness,
    ByronPkWitness
);

#[wasm_bindgen]
impl ByronPkWitness {
    pub fn index_1(&self) -> ByronPkWitnessEntry {
        self.0.index_1.clone().into()
    }

    pub fn new(index_1: &ByronPkWitnessEntry) -> Self {
        Self(cml_multi_era::byron::transaction::ByronPkWitness::new(
            index_1.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronPkWitnessEntry(cml_multi_era::byron::transaction::ByronPkWitnessEntry);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronPkWitnessEntry);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronPkWitnessEntry,
    ByronPkWitnessEntry
);

#[wasm_bindgen]
impl ByronPkWitnessEntry {
    pub fn byron_pub_key(&self) -> ByronPubKey {
        self.0.byron_pub_key.clone()
    }

    pub fn byron_signature(&self) -> ByronSignature {
        self.0.byron_signature.clone()
    }

    pub fn new(byron_pub_key: ByronPubKey, byron_signature: ByronSignature) -> Self {
        Self(cml_multi_era::byron::transaction::ByronPkWitnessEntry::new(
            byron_pub_key,
            byron_signature,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronRedeemWitness(cml_multi_era::byron::transaction::ByronRedeemWitness);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronRedeemWitness);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronRedeemWitness,
    ByronRedeemWitness
);

#[wasm_bindgen]
impl ByronRedeemWitness {
    pub fn index_1(&self) -> ByronRedeemerWitnessEntry {
        self.0.index_1.clone().into()
    }

    pub fn new(index_1: &ByronRedeemerWitnessEntry) -> Self {
        Self(cml_multi_era::byron::transaction::ByronRedeemWitness::new(
            index_1.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronRedeemerScript(cml_multi_era::byron::transaction::ByronRedeemerScript);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronRedeemerScript);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronRedeemerScript,
    ByronRedeemerScript
);

#[wasm_bindgen]
impl ByronRedeemerScript {
    pub fn u16(&self) -> u16 {
        self.0.u16
    }

    pub fn index_1(&self) -> Vec<u8> {
        self.0.index_1.clone()
    }

    pub fn new(u16: u16, index_1: Vec<u8>) -> Self {
        Self(cml_multi_era::byron::transaction::ByronRedeemerScript::new(
            u16, index_1,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronRedeemerWitnessEntry(cml_multi_era::byron::transaction::ByronRedeemerWitnessEntry);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronRedeemerWitnessEntry);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronRedeemerWitnessEntry,
    ByronRedeemerWitnessEntry
);

#[wasm_bindgen]
impl ByronRedeemerWitnessEntry {
    pub fn byron_pub_key(&self) -> ByronPubKey {
        self.0.byron_pub_key.clone()
    }

    pub fn byron_signature(&self) -> ByronSignature {
        self.0.byron_signature.clone()
    }

    pub fn new(byron_pub_key: ByronPubKey, byron_signature: ByronSignature) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronRedeemerWitnessEntry::new(
                byron_pub_key,
                byron_signature,
            ),
        )
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronScriptWitness(cml_multi_era::byron::transaction::ByronScriptWitness);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronScriptWitness);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronScriptWitness,
    ByronScriptWitness
);

#[wasm_bindgen]
impl ByronScriptWitness {
    pub fn index_1(&self) -> ByronScriptWitnessEntry {
        self.0.index_1.clone().into()
    }

    pub fn new(index_1: &ByronScriptWitnessEntry) -> Self {
        Self(cml_multi_era::byron::transaction::ByronScriptWitness::new(
            index_1.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronScriptWitnessEntry(cml_multi_era::byron::transaction::ByronScriptWitnessEntry);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronScriptWitnessEntry);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronScriptWitnessEntry,
    ByronScriptWitnessEntry
);

#[wasm_bindgen]
impl ByronScriptWitnessEntry {
    pub fn byron_validator_script(&self) -> ByronValidatorScript {
        self.0.byron_validator_script.clone().into()
    }

    pub fn byron_redeemer_script(&self) -> ByronRedeemerScript {
        self.0.byron_redeemer_script.clone().into()
    }

    pub fn new(
        byron_validator_script: &ByronValidatorScript,
        byron_redeemer_script: &ByronRedeemerScript,
    ) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronScriptWitnessEntry::new(
                byron_validator_script.clone().into(),
                byron_redeemer_script.clone().into(),
            ),
        )
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTx(cml_multi_era::byron::transaction::ByronTx);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronTx);

impl_wasm_conversions!(cml_multi_era::byron::transaction::ByronTx, ByronTx);

#[wasm_bindgen]
impl ByronTx {
    pub fn inputs(&self) -> ByronTxInList {
        self.0.inputs.clone().into()
    }

    pub fn outputs(&self) -> ByronTxOutList {
        self.0.outputs.clone().into()
    }

    pub fn attrs(&self) -> ByronAttributes {
        self.0.attrs.clone().into()
    }

    pub fn new(inputs: &ByronTxInList, outputs: &ByronTxOutList, attrs: &ByronAttributes) -> Self {
        Self(cml_multi_era::byron::transaction::ByronTx::new(
            inputs.clone().into(),
            outputs.clone().into(),
            attrs.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxIn(cml_multi_era::byron::transaction::ByronTxIn);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronTxIn);

impl_wasm_conversions!(cml_multi_era::byron::transaction::ByronTxIn, ByronTxIn);

#[wasm_bindgen]
impl ByronTxIn {
    pub fn new_byron_tx_in_regular(byron_tx_in_regular: &ByronTxInRegular) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronTxIn::new_byron_tx_in_regular(
                byron_tx_in_regular.clone().into(),
            ),
        )
    }

    pub fn new_byron_tx_in_genesis(byron_tx_in_genesis: &ByronTxInGenesis) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronTxIn::new_byron_tx_in_genesis(
                byron_tx_in_genesis.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> ByronTxInKind {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxIn::ByronTxInRegular(_) => {
                ByronTxInKind::ByronTxInRegular
            }
            cml_multi_era::byron::transaction::ByronTxIn::ByronTxInGenesis(_) => {
                ByronTxInKind::ByronTxInGenesis
            }
        }
    }

    pub fn as_byron_tx_in_regular(&self) -> Option<ByronTxInRegular> {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxIn::ByronTxInRegular(byron_tx_in_regular) => {
                Some(byron_tx_in_regular.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_byron_tx_in_genesis(&self) -> Option<ByronTxInGenesis> {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxIn::ByronTxInGenesis(byron_tx_in_genesis) => {
                Some(byron_tx_in_genesis.clone().into())
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxInGenesis(cml_multi_era::byron::transaction::ByronTxInGenesis);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronTxInGenesis);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronTxInGenesis,
    ByronTxInGenesis
);

#[wasm_bindgen]
impl ByronTxInGenesis {
    pub fn u8(&self) -> u8 {
        self.0.u8
    }

    pub fn index_1(&self) -> Vec<u8> {
        self.0.index_1.clone()
    }

    pub fn new(u8: u8, index_1: Vec<u8>) -> Self {
        Self(cml_multi_era::byron::transaction::ByronTxInGenesis::new(
            u8, index_1,
        ))
    }
}

#[wasm_bindgen]
pub enum ByronTxInKind {
    ByronTxInRegular,
    ByronTxInGenesis,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxInRegular(cml_multi_era::byron::transaction::ByronTxInRegular);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronTxInRegular);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronTxInRegular,
    ByronTxInRegular
);

#[wasm_bindgen]
impl ByronTxInRegular {
    pub fn index_1(&self) -> ByronTxOutPtr {
        self.0.index_1.clone().into()
    }

    pub fn new(index_1: &ByronTxOutPtr) -> Self {
        Self(cml_multi_era::byron::transaction::ByronTxInRegular::new(
            index_1.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxOutPtr(cml_multi_era::byron::transaction::ByronTxOutPtr);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronTxOutPtr);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronTxOutPtr,
    ByronTxOutPtr
);

#[wasm_bindgen]
impl ByronTxOutPtr {
    pub fn byron_tx_id(&self) -> ByronTxId {
        self.0.byron_tx_id.into()
    }

    pub fn u32(&self) -> u32 {
        self.0.u32
    }

    pub fn new(byron_tx_id: &ByronTxId, u32: u32) -> Self {
        Self(cml_multi_era::byron::transaction::ByronTxOutPtr::new(
            byron_tx_id.clone().into(),
            u32,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxProof(cml_multi_era::byron::transaction::ByronTxProof);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronTxProof);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronTxProof,
    ByronTxProof
);

#[wasm_bindgen]
impl ByronTxProof {
    pub fn u32(&self) -> u32 {
        self.0.u32
    }

    pub fn blake2b256(&self) -> Blake2b256 {
        self.0.blake2b256.into()
    }

    pub fn blake2b2562(&self) -> Blake2b256 {
        self.0.blake2b2562.into()
    }

    pub fn new(u32: u32, blake2b256: &Blake2b256, blake2b2562: &Blake2b256) -> Self {
        Self(cml_multi_era::byron::transaction::ByronTxProof::new(
            u32,
            blake2b256.clone().into(),
            blake2b2562.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronTxWitness(cml_multi_era::byron::transaction::ByronTxWitness);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronTxWitness);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronTxWitness,
    ByronTxWitness
);

#[wasm_bindgen]
impl ByronTxWitness {
    pub fn new_byron_pk_witness(index_1: &ByronPkWitnessEntry) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronTxWitness::new_byron_pk_witness(
                index_1.clone().into(),
            ),
        )
    }

    pub fn new_byron_script_witness(index_1: &ByronScriptWitnessEntry) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronTxWitness::new_byron_script_witness(
                index_1.clone().into(),
            ),
        )
    }

    pub fn new_byron_redeem_witness(index_1: &ByronRedeemerWitnessEntry) -> Self {
        Self(
            cml_multi_era::byron::transaction::ByronTxWitness::new_byron_redeem_witness(
                index_1.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> ByronTxWitnessKind {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxWitness::ByronPkWitness(_) => {
                ByronTxWitnessKind::ByronPkWitness
            }
            cml_multi_era::byron::transaction::ByronTxWitness::ByronScriptWitness(_) => {
                ByronTxWitnessKind::ByronScriptWitness
            }
            cml_multi_era::byron::transaction::ByronTxWitness::ByronRedeemWitness(_) => {
                ByronTxWitnessKind::ByronRedeemWitness
            }
        }
    }

    pub fn as_byron_pk_witness(&self) -> Option<ByronPkWitness> {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxWitness::ByronPkWitness(byron_pk_witness) => {
                Some(byron_pk_witness.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_byron_script_witness(&self) -> Option<ByronScriptWitness> {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxWitness::ByronScriptWitness(
                byron_script_witness,
            ) => Some(byron_script_witness.clone().into()),
            _ => None,
        }
    }

    pub fn as_byron_redeem_witness(&self) -> Option<ByronRedeemWitness> {
        match &self.0 {
            cml_multi_era::byron::transaction::ByronTxWitness::ByronRedeemWitness(
                byron_redeem_witness,
            ) => Some(byron_redeem_witness.clone().into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum ByronTxWitnessKind {
    ByronPkWitness,
    ByronScriptWitness,
    ByronRedeemWitness,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronValidatorScript(cml_multi_era::byron::transaction::ByronValidatorScript);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronValidatorScript);

impl_wasm_conversions!(
    cml_multi_era::byron::transaction::ByronValidatorScript,
    ByronValidatorScript
);

#[wasm_bindgen]
impl ByronValidatorScript {
    pub fn u16(&self) -> u16 {
        self.0.u16
    }

    pub fn index_1(&self) -> Vec<u8> {
        self.0.index_1.clone()
    }

    pub fn new(u16: u16, index_1: Vec<u8>) -> Self {
        Self(cml_multi_era::byron::transaction::ByronValidatorScript::new(u16, index_1))
    }
}
