// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::byron::delegation::{ByronDelegationSignature, LightWeightDelegationSignature};
use crate::byron::mpc::{Ssc, SscProof};
use crate::byron::transaction::{ByronAttributes, ByronTx, ByronTxProof};
use crate::byron::update::{ByronBlockVersion, ByronSoftwareVersion, ByronUpdate};
use crate::byron::{Blake2b256, ByronBlockId, ByronPubKey, ByronSignature, ByronSlotId, EpochId};
use crate::byron::{
    ByronAttributesList, ByronDelegationList, ByronTxWitnessList, StakeholderIdList,
};
use cml_core_wasm::{
    impl_wasm_cbor_json_api_cbor_event_serialize, impl_wasm_conversions, impl_wasm_list,
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct BlockHeaderExtraData(cml_multi_era::byron::block::BlockHeaderExtraData);

impl_wasm_cbor_json_api_cbor_event_serialize!(BlockHeaderExtraData);

impl_wasm_conversions!(
    cml_multi_era::byron::block::BlockHeaderExtraData,
    BlockHeaderExtraData
);

#[wasm_bindgen]
impl BlockHeaderExtraData {
    pub fn block_version(&self) -> ByronBlockVersion {
        self.0.block_version.clone().into()
    }

    pub fn software_version(&self) -> ByronSoftwareVersion {
        self.0.software_version.clone().into()
    }

    pub fn byron_attributes(&self) -> ByronAttributes {
        self.0.byron_attributes.clone().into()
    }

    pub fn extra_proof(&self) -> Blake2b256 {
        self.0.extra_proof.into()
    }

    pub fn new(
        block_version: &ByronBlockVersion,
        software_version: &ByronSoftwareVersion,
        byron_attributes: &ByronAttributes,
        extra_proof: &Blake2b256,
    ) -> Self {
        Self(cml_multi_era::byron::block::BlockHeaderExtraData::new(
            block_version.clone().into(),
            software_version.clone().into(),
            byron_attributes.clone().into(),
            extra_proof.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlock(cml_multi_era::byron::block::ByronBlock);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronBlock);

impl_wasm_conversions!(cml_multi_era::byron::block::ByronBlock, ByronBlock);

#[wasm_bindgen]
impl ByronBlock {
    pub fn new_epoch_boundary(epoch_boundary: &ByronEbBlock) -> Self {
        Self(cml_multi_era::byron::block::ByronBlock::new_epoch_boundary(
            epoch_boundary.clone().into(),
        ))
    }

    pub fn new_main(main: &ByronMainBlock) -> Self {
        Self(cml_multi_era::byron::block::ByronBlock::new_main(
            main.clone().into(),
        ))
    }

    pub fn kind(&self) -> ByronBlockKind {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlock::EpochBoundary(_) => {
                ByronBlockKind::EpochBoundary
            }
            cml_multi_era::byron::block::ByronBlock::Main(_) => ByronBlockKind::Main,
        }
    }

    pub fn as_epoch_boundary(&self) -> Option<ByronEbBlock> {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlock::EpochBoundary(epoch_boundary) => {
                Some(epoch_boundary.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_main(&self) -> Option<ByronMainBlock> {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlock::Main(main) => Some(main.clone().into()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockBody(cml_multi_era::byron::block::ByronBlockBody);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronBlockBody);

impl_wasm_conversions!(cml_multi_era::byron::block::ByronBlockBody, ByronBlockBody);

#[wasm_bindgen]
impl ByronBlockBody {
    pub fn tx_payload(&self) -> TxPayload {
        self.0.tx_payload.clone().into()
    }

    pub fn ssc_payload(&self) -> Ssc {
        self.0.ssc_payload.clone().into()
    }

    pub fn dlg_payload(&self) -> ByronDelegationList {
        self.0.dlg_payload.clone().into()
    }

    pub fn upd_payload(&self) -> ByronUpdate {
        self.0.upd_payload.clone().into()
    }

    pub fn new(
        tx_payload: &TxPayload,
        ssc_payload: &Ssc,
        dlg_payload: &ByronDelegationList,
        upd_payload: &ByronUpdate,
    ) -> Self {
        Self(cml_multi_era::byron::block::ByronBlockBody::new(
            tx_payload.clone().into(),
            ssc_payload.clone().into(),
            dlg_payload.clone().into(),
            upd_payload.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockConsensusData(cml_multi_era::byron::block::ByronBlockConsensusData);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronBlockConsensusData);

impl_wasm_conversions!(
    cml_multi_era::byron::block::ByronBlockConsensusData,
    ByronBlockConsensusData
);

#[wasm_bindgen]
impl ByronBlockConsensusData {
    pub fn byron_slot_id(&self) -> ByronSlotId {
        self.0.byron_slot_id.clone().into()
    }

    pub fn byron_pub_key(&self) -> ByronPubKey {
        self.0.byron_pub_key.clone()
    }

    pub fn byron_difficulty(&self) -> ByronDifficulty {
        self.0.byron_difficulty.clone().into()
    }

    pub fn byron_block_signature(&self) -> ByronBlockSignature {
        self.0.byron_block_signature.clone().into()
    }

    pub fn new(
        byron_slot_id: &ByronSlotId,
        byron_pub_key: ByronPubKey,
        byron_difficulty: &ByronDifficulty,
        byron_block_signature: &ByronBlockSignature,
    ) -> Self {
        Self(cml_multi_era::byron::block::ByronBlockConsensusData::new(
            byron_slot_id.clone().into(),
            byron_pub_key,
            byron_difficulty.clone().into(),
            byron_block_signature.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockHeader(cml_multi_era::byron::block::ByronBlockHeader);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronBlockHeader);

impl_wasm_conversions!(
    cml_multi_era::byron::block::ByronBlockHeader,
    ByronBlockHeader
);

#[wasm_bindgen]
impl ByronBlockHeader {
    pub fn protocol_magic(&self) -> u32 {
        self.0.protocol_magic
    }

    pub fn prev_block(&self) -> ByronBlockId {
        self.0.prev_block.into()
    }

    pub fn body_proof(&self) -> ByronBodyProof {
        self.0.body_proof.clone().into()
    }

    pub fn consensus_data(&self) -> ByronBlockConsensusData {
        self.0.consensus_data.clone().into()
    }

    pub fn extra_data(&self) -> BlockHeaderExtraData {
        self.0.extra_data.clone().into()
    }

    pub fn new(
        protocol_magic: u32,
        prev_block: &ByronBlockId,
        body_proof: &ByronBodyProof,
        consensus_data: &ByronBlockConsensusData,
        extra_data: &BlockHeaderExtraData,
    ) -> Self {
        Self(cml_multi_era::byron::block::ByronBlockHeader::new(
            protocol_magic,
            prev_block.clone().into(),
            body_proof.clone().into(),
            consensus_data.clone().into(),
            extra_data.clone().into(),
        ))
    }
}

#[wasm_bindgen]
pub enum ByronBlockKind {
    EpochBoundary,
    Main,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockSignature(cml_multi_era::byron::block::ByronBlockSignature);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronBlockSignature);

impl_wasm_conversions!(
    cml_multi_era::byron::block::ByronBlockSignature,
    ByronBlockSignature
);

#[wasm_bindgen]
impl ByronBlockSignature {
    pub fn new_signature(signature: &ByronBlockSignatureNormal) -> Self {
        Self(
            cml_multi_era::byron::block::ByronBlockSignature::new_signature(
                signature.clone().into(),
            ),
        )
    }

    pub fn new_proxy_light(proxy_light: &ByronBlockSignatureProxyLight) -> Self {
        Self(
            cml_multi_era::byron::block::ByronBlockSignature::new_proxy_light(
                proxy_light.clone().into(),
            ),
        )
    }

    pub fn new_proxy_heavy(proxy_heavy: &ByronBlockSignatureProxyHeavy) -> Self {
        Self(
            cml_multi_era::byron::block::ByronBlockSignature::new_proxy_heavy(
                proxy_heavy.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> ByronBlockSignatureKind {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlockSignature::Signature(_) => {
                ByronBlockSignatureKind::Signature
            }
            cml_multi_era::byron::block::ByronBlockSignature::ProxyLight(_) => {
                ByronBlockSignatureKind::ProxyLight
            }
            cml_multi_era::byron::block::ByronBlockSignature::ProxyHeavy(_) => {
                ByronBlockSignatureKind::ProxyHeavy
            }
        }
    }

    pub fn as_signature(&self) -> Option<ByronBlockSignatureNormal> {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlockSignature::Signature(signature) => {
                Some(signature.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_proxy_light(&self) -> Option<ByronBlockSignatureProxyLight> {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlockSignature::ProxyLight(proxy_light) => {
                Some(proxy_light.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_proxy_heavy(&self) -> Option<ByronBlockSignatureProxyHeavy> {
        match &self.0 {
            cml_multi_era::byron::block::ByronBlockSignature::ProxyHeavy(proxy_heavy) => {
                Some(proxy_heavy.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum ByronBlockSignatureKind {
    Signature,
    ProxyLight,
    ProxyHeavy,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockSignatureNormal(cml_multi_era::byron::block::ByronBlockSignatureNormal);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronBlockSignatureNormal);

impl_wasm_conversions!(
    cml_multi_era::byron::block::ByronBlockSignatureNormal,
    ByronBlockSignatureNormal
);

#[wasm_bindgen]
impl ByronBlockSignatureNormal {
    pub fn signature(&self) -> ByronSignature {
        self.0.signature.clone()
    }

    pub fn new(signature: ByronSignature) -> Self {
        Self(cml_multi_era::byron::block::ByronBlockSignatureNormal::new(
            signature,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockSignatureProxyHeavy(
    cml_multi_era::byron::block::ByronBlockSignatureProxyHeavy,
);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronBlockSignatureProxyHeavy);

impl_wasm_conversions!(
    cml_multi_era::byron::block::ByronBlockSignatureProxyHeavy,
    ByronBlockSignatureProxyHeavy
);

#[wasm_bindgen]
impl ByronBlockSignatureProxyHeavy {
    pub fn signature(&self) -> ByronDelegationSignature {
        self.0.signature.clone().into()
    }

    pub fn new(signature: &ByronDelegationSignature) -> Self {
        Self(
            cml_multi_era::byron::block::ByronBlockSignatureProxyHeavy::new(
                signature.clone().into(),
            ),
        )
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBlockSignatureProxyLight(
    cml_multi_era::byron::block::ByronBlockSignatureProxyLight,
);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronBlockSignatureProxyLight);

impl_wasm_conversions!(
    cml_multi_era::byron::block::ByronBlockSignatureProxyLight,
    ByronBlockSignatureProxyLight
);

#[wasm_bindgen]
impl ByronBlockSignatureProxyLight {
    pub fn signature(&self) -> LightWeightDelegationSignature {
        self.0.signature.clone().into()
    }

    pub fn new(signature: &LightWeightDelegationSignature) -> Self {
        Self(
            cml_multi_era::byron::block::ByronBlockSignatureProxyLight::new(
                signature.clone().into(),
            ),
        )
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronBodyProof(cml_multi_era::byron::block::ByronBodyProof);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronBodyProof);

impl_wasm_conversions!(cml_multi_era::byron::block::ByronBodyProof, ByronBodyProof);

#[wasm_bindgen]
impl ByronBodyProof {
    pub fn tx_proof(&self) -> ByronTxProof {
        self.0.tx_proof.clone().into()
    }

    pub fn ssc_proof(&self) -> SscProof {
        self.0.ssc_proof.clone().into()
    }

    pub fn dlg_proof(&self) -> Blake2b256 {
        self.0.dlg_proof.into()
    }

    pub fn upd_proof(&self) -> Blake2b256 {
        self.0.upd_proof.into()
    }

    pub fn new(
        tx_proof: &ByronTxProof,
        ssc_proof: &SscProof,
        dlg_proof: &Blake2b256,
        upd_proof: &Blake2b256,
    ) -> Self {
        Self(cml_multi_era::byron::block::ByronBodyProof::new(
            tx_proof.clone().into(),
            ssc_proof.clone().into(),
            dlg_proof.clone().into(),
            upd_proof.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronDifficulty(cml_multi_era::byron::block::ByronDifficulty);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronDifficulty);

impl_wasm_conversions!(
    cml_multi_era::byron::block::ByronDifficulty,
    ByronDifficulty
);

#[wasm_bindgen]
impl ByronDifficulty {
    pub fn u64(&self) -> u64 {
        self.0.u64
    }

    pub fn new(u64: u64) -> Self {
        Self(cml_multi_era::byron::block::ByronDifficulty::new(u64))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronEbBlock(cml_multi_era::byron::block::ByronEbBlock);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronEbBlock);

impl_wasm_conversions!(cml_multi_era::byron::block::ByronEbBlock, ByronEbBlock);

#[wasm_bindgen]
impl ByronEbBlock {
    pub fn header(&self) -> EbbHead {
        self.0.header.clone().into()
    }

    pub fn body(&self) -> StakeholderIdList {
        self.0.body.clone().into()
    }

    pub fn extra(&self) -> ByronAttributesList {
        self.0.extra.clone().into()
    }

    pub fn new(header: &EbbHead, body: &StakeholderIdList, extra: &ByronAttributesList) -> Self {
        Self(cml_multi_era::byron::block::ByronEbBlock::new(
            header.clone().into(),
            body.clone().into(),
            extra.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronMainBlock(cml_multi_era::byron::block::ByronMainBlock);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronMainBlock);

impl_wasm_conversions!(cml_multi_era::byron::block::ByronMainBlock, ByronMainBlock);

#[wasm_bindgen]
impl ByronMainBlock {
    pub fn header(&self) -> ByronBlockHeader {
        self.0.header.clone().into()
    }

    pub fn body(&self) -> ByronBlockBody {
        self.0.body.clone().into()
    }

    pub fn extra(&self) -> ByronAttributesList {
        self.0.extra.clone().into()
    }

    pub fn new(
        header: &ByronBlockHeader,
        body: &ByronBlockBody,
        extra: &ByronAttributesList,
    ) -> Self {
        Self(cml_multi_era::byron::block::ByronMainBlock::new(
            header.clone().into(),
            body.clone().into(),
            extra.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct EbbConsensusData(cml_multi_era::byron::block::EbbConsensusData);

impl_wasm_cbor_json_api_cbor_event_serialize!(EbbConsensusData);

impl_wasm_conversions!(
    cml_multi_era::byron::block::EbbConsensusData,
    EbbConsensusData
);

#[wasm_bindgen]
impl EbbConsensusData {
    pub fn epoch_id(&self) -> EpochId {
        self.0.epoch_id
    }

    pub fn byron_difficulty(&self) -> ByronDifficulty {
        self.0.byron_difficulty.clone().into()
    }

    pub fn new(epoch_id: EpochId, byron_difficulty: &ByronDifficulty) -> Self {
        Self(cml_multi_era::byron::block::EbbConsensusData::new(
            epoch_id,
            byron_difficulty.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct EbbHead(cml_multi_era::byron::block::EbbHead);

impl_wasm_cbor_json_api_cbor_event_serialize!(EbbHead);

impl_wasm_conversions!(cml_multi_era::byron::block::EbbHead, EbbHead);

#[wasm_bindgen]
impl EbbHead {
    pub fn protocol_magic(&self) -> u32 {
        self.0.protocol_magic
    }

    pub fn prev_block(&self) -> ByronBlockId {
        self.0.prev_block.into()
    }

    pub fn body_proof(&self) -> Blake2b256 {
        self.0.body_proof.into()
    }

    pub fn consensus_data(&self) -> EbbConsensusData {
        self.0.consensus_data.clone().into()
    }

    pub fn extra_data(&self) -> ByronAttributesList {
        self.0.extra_data.clone().into()
    }

    pub fn new(
        protocol_magic: u32,
        prev_block: &ByronBlockId,
        body_proof: &Blake2b256,
        consensus_data: &EbbConsensusData,
        extra_data: &ByronAttributesList,
    ) -> Self {
        Self(cml_multi_era::byron::block::EbbHead::new(
            protocol_magic,
            prev_block.clone().into(),
            body_proof.clone().into(),
            consensus_data.clone().into(),
            extra_data.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct TxAux(cml_multi_era::byron::block::TxAux);

impl_wasm_cbor_json_api_cbor_event_serialize!(TxAux);

impl_wasm_conversions!(cml_multi_era::byron::block::TxAux, TxAux);

#[wasm_bindgen]
impl TxAux {
    pub fn byron_tx(&self) -> ByronTx {
        self.0.byron_tx.clone().into()
    }

    pub fn byron_tx_witnesss(&self) -> ByronTxWitnessList {
        self.0.byron_tx_witnesss.clone().into()
    }

    pub fn new(byron_tx: &ByronTx, byron_tx_witnesss: &ByronTxWitnessList) -> Self {
        Self(cml_multi_era::byron::block::TxAux::new(
            byron_tx.clone().into(),
            byron_tx_witnesss.clone().into(),
        ))
    }
}

impl_wasm_list!(cml_multi_era::byron::block::TxAux, TxAux, TxPayload);
