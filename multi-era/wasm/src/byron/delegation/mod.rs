// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::byron::{ByronPubKey, ByronSignature, EpochId};
use cml_core_wasm::{impl_wasm_cbor_json_api_cbor_event_serialize, impl_wasm_conversions};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronDelegation(cml_multi_era::byron::delegation::ByronDelegation);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronDelegation);

impl_wasm_conversions!(
    cml_multi_era::byron::delegation::ByronDelegation,
    ByronDelegation
);

#[wasm_bindgen]
impl ByronDelegation {
    pub fn epoch(&self) -> EpochId {
        self.0.epoch
    }

    pub fn issuer(&self) -> ByronPubKey {
        self.0.issuer.clone()
    }

    pub fn delegate(&self) -> ByronPubKey {
        self.0.delegate.clone()
    }

    pub fn certificate(&self) -> ByronSignature {
        self.0.certificate.clone()
    }

    pub fn new(
        epoch: EpochId,
        issuer: ByronPubKey,
        delegate: ByronPubKey,
        certificate: ByronSignature,
    ) -> Self {
        Self(cml_multi_era::byron::delegation::ByronDelegation::new(
            epoch,
            issuer,
            delegate,
            certificate,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ByronDelegationSignature(cml_multi_era::byron::delegation::ByronDelegationSignature);

impl_wasm_cbor_json_api_cbor_event_serialize!(ByronDelegationSignature);

impl_wasm_conversions!(
    cml_multi_era::byron::delegation::ByronDelegationSignature,
    ByronDelegationSignature
);

#[wasm_bindgen]
impl ByronDelegationSignature {
    pub fn byron_delegation(&self) -> ByronDelegation {
        self.0.byron_delegation.clone().into()
    }

    pub fn byron_signature(&self) -> ByronSignature {
        self.0.byron_signature.clone()
    }

    pub fn new(byron_delegation: &ByronDelegation, byron_signature: ByronSignature) -> Self {
        Self(
            cml_multi_era::byron::delegation::ByronDelegationSignature::new(
                byron_delegation.clone().into(),
                byron_signature,
            ),
        )
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct EpochRange(cml_multi_era::byron::delegation::EpochRange);

impl_wasm_cbor_json_api_cbor_event_serialize!(EpochRange);

impl_wasm_conversions!(cml_multi_era::byron::delegation::EpochRange, EpochRange);

#[wasm_bindgen]
impl EpochRange {
    pub fn epoch_id(&self) -> EpochId {
        self.0.epoch_id
    }

    pub fn epoch_id2(&self) -> EpochId {
        self.0.epoch_id2
    }

    pub fn new(epoch_id: EpochId, epoch_id2: EpochId) -> Self {
        Self(cml_multi_era::byron::delegation::EpochRange::new(
            epoch_id, epoch_id2,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct LightWeightDelegationSignature(
    cml_multi_era::byron::delegation::LightWeightDelegationSignature,
);

impl_wasm_cbor_json_api_cbor_event_serialize!(LightWeightDelegationSignature);

impl_wasm_conversions!(
    cml_multi_era::byron::delegation::LightWeightDelegationSignature,
    LightWeightDelegationSignature
);

#[wasm_bindgen]
impl LightWeightDelegationSignature {
    pub fn light_weight_dlg(&self) -> LightWeightDlg {
        self.0.light_weight_dlg.clone().into()
    }

    pub fn byron_signature(&self) -> ByronSignature {
        self.0.byron_signature.clone()
    }

    pub fn new(light_weight_dlg: &LightWeightDlg, byron_signature: ByronSignature) -> Self {
        Self(
            cml_multi_era::byron::delegation::LightWeightDelegationSignature::new(
                light_weight_dlg.clone().into(),
                byron_signature,
            ),
        )
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct LightWeightDlg(cml_multi_era::byron::delegation::LightWeightDlg);

impl_wasm_cbor_json_api_cbor_event_serialize!(LightWeightDlg);

impl_wasm_conversions!(
    cml_multi_era::byron::delegation::LightWeightDlg,
    LightWeightDlg
);

#[wasm_bindgen]
impl LightWeightDlg {
    pub fn epoch_range(&self) -> EpochRange {
        self.0.epoch_range.clone().into()
    }

    pub fn issuer(&self) -> ByronPubKey {
        self.0.issuer.clone()
    }

    pub fn delegate(&self) -> ByronPubKey {
        self.0.delegate.clone()
    }

    pub fn certificate(&self) -> ByronSignature {
        self.0.certificate.clone()
    }

    pub fn new(
        epoch_range: &EpochRange,
        issuer: ByronPubKey,
        delegate: ByronPubKey,
        certificate: ByronSignature,
    ) -> Self {
        Self(cml_multi_era::byron::delegation::LightWeightDlg::new(
            epoch_range.clone().into(),
            issuer,
            delegate,
            certificate,
        ))
    }
}
