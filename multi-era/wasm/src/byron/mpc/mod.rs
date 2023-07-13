// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::byron::{Blake2b256, ByronPubKey, ByronSignature, EpochId};
use crate::cml_chain::byron::{AddressId, StakeholderId};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Ssc(cml_multi_era::byron::mpc::Ssc);

#[wasm_bindgen]
impl Ssc {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<Ssc, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Ssc, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_ssc_commitments_payload(ssc_commitments_payload: &SscCommitmentsPayload) -> Self {
        Self(cml_multi_era::byron::mpc::Ssc::new_ssc_commitments_payload(
            ssc_commitments_payload.clone().into(),
        ))
    }

    pub fn new_ssc_openings_payload(ssc_openings_payload: &SscOpeningsPayload) -> Self {
        Self(cml_multi_era::byron::mpc::Ssc::new_ssc_openings_payload(
            ssc_openings_payload.clone().into(),
        ))
    }

    pub fn new_ssc_shares_payload(ssc_shares_payload: &SscSharesPayload) -> Self {
        Self(cml_multi_era::byron::mpc::Ssc::new_ssc_shares_payload(
            ssc_shares_payload.clone().into(),
        ))
    }

    pub fn new_ssc_certificates_payload(ssc_certificates_payload: &SscCertificatesPayload) -> Self {
        Self(
            cml_multi_era::byron::mpc::Ssc::new_ssc_certificates_payload(
                ssc_certificates_payload.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> SscKind {
        match &self.0 {
            cml_multi_era::byron::mpc::Ssc::SscCommitmentsPayload(_) => {
                SscKind::SscCommitmentsPayload
            }
            cml_multi_era::byron::mpc::Ssc::SscOpeningsPayload(_) => SscKind::SscOpeningsPayload,
            cml_multi_era::byron::mpc::Ssc::SscSharesPayload(_) => SscKind::SscSharesPayload,
            cml_multi_era::byron::mpc::Ssc::SscCertificatesPayload(_) => {
                SscKind::SscCertificatesPayload
            }
        }
    }

    pub fn as_ssc_commitments_payload(&self) -> Option<SscCommitmentsPayload> {
        match &self.0 {
            cml_multi_era::byron::mpc::Ssc::SscCommitmentsPayload(ssc_commitments_payload) => {
                Some(ssc_commitments_payload.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_ssc_openings_payload(&self) -> Option<SscOpeningsPayload> {
        match &self.0 {
            cml_multi_era::byron::mpc::Ssc::SscOpeningsPayload(ssc_openings_payload) => {
                Some(ssc_openings_payload.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_ssc_shares_payload(&self) -> Option<SscSharesPayload> {
        match &self.0 {
            cml_multi_era::byron::mpc::Ssc::SscSharesPayload(ssc_shares_payload) => {
                Some(ssc_shares_payload.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_ssc_certificates_payload(&self) -> Option<SscCertificatesPayload> {
        match &self.0 {
            cml_multi_era::byron::mpc::Ssc::SscCertificatesPayload(ssc_certificates_payload) => {
                Some(ssc_certificates_payload.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cml_multi_era::byron::mpc::Ssc> for Ssc {
    fn from(native: cml_multi_era::byron::mpc::Ssc) -> Self {
        Self(native)
    }
}

impl From<Ssc> for cml_multi_era::byron::mpc::Ssc {
    fn from(wasm: Ssc) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::Ssc> for Ssc {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::Ssc {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCert(cml_multi_era::byron::mpc::SscCert);

#[wasm_bindgen]
impl SscCert {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscCert, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscCert, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn vss_pub_key(&self) -> VssPubKey {
        self.0.vss_pub_key.clone()
    }

    pub fn epoch_id(&self) -> EpochId {
        self.0.epoch_id
    }

    pub fn byron_pub_key(&self) -> ByronPubKey {
        self.0.byron_pub_key.clone()
    }

    pub fn byron_signature(&self) -> ByronSignature {
        self.0.byron_signature.clone()
    }

    pub fn new(
        vss_pub_key: VssPubKey,
        epoch_id: EpochId,
        byron_pub_key: ByronPubKey,
        byron_signature: ByronSignature,
    ) -> Self {
        Self(cml_multi_era::byron::mpc::SscCert::new(
            vss_pub_key,
            epoch_id,
            byron_pub_key,
            byron_signature,
        ))
    }
}

impl From<cml_multi_era::byron::mpc::SscCert> for SscCert {
    fn from(native: cml_multi_era::byron::mpc::SscCert) -> Self {
        Self(native)
    }
}

impl From<SscCert> for cml_multi_era::byron::mpc::SscCert {
    fn from(wasm: SscCert) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscCert> for SscCert {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscCert {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCertificatesPayload(cml_multi_era::byron::mpc::SscCertificatesPayload);

#[wasm_bindgen]
impl SscCertificatesPayload {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscCertificatesPayload, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscCertificatesPayload, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn ssc_certs(&self) -> SscCerts {
        self.0.ssc_certs.clone().into()
    }

    pub fn new(ssc_certs: &SscCerts) -> Self {
        Self(cml_multi_era::byron::mpc::SscCertificatesPayload::new(
            ssc_certs.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::mpc::SscCertificatesPayload> for SscCertificatesPayload {
    fn from(native: cml_multi_era::byron::mpc::SscCertificatesPayload) -> Self {
        Self(native)
    }
}

impl From<SscCertificatesPayload> for cml_multi_era::byron::mpc::SscCertificatesPayload {
    fn from(wasm: SscCertificatesPayload) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscCertificatesPayload> for SscCertificatesPayload {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscCertificatesPayload {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCertificatesProof(cml_multi_era::byron::mpc::SscCertificatesProof);

#[wasm_bindgen]
impl SscCertificatesProof {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscCertificatesProof, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscCertificatesProof, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn blake2b256(&self) -> Blake2b256 {
        self.0.blake2b256.clone().into()
    }

    pub fn new(blake2b256: &Blake2b256) -> Self {
        Self(cml_multi_era::byron::mpc::SscCertificatesProof::new(
            blake2b256.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::mpc::SscCertificatesProof> for SscCertificatesProof {
    fn from(native: cml_multi_era::byron::mpc::SscCertificatesProof) -> Self {
        Self(native)
    }
}

impl From<SscCertificatesProof> for cml_multi_era::byron::mpc::SscCertificatesProof {
    fn from(wasm: SscCertificatesProof) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscCertificatesProof> for SscCertificatesProof {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscCertificatesProof {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCerts(Vec<cml_multi_era::byron::mpc::SscCert>);

#[wasm_bindgen]
impl SscCerts {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> SscCert {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &SscCert) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::mpc::SscCert>> for SscCerts {
    fn from(native: Vec<cml_multi_era::byron::mpc::SscCert>) -> Self {
        Self(native)
    }
}

impl From<SscCerts> for Vec<cml_multi_era::byron::mpc::SscCert> {
    fn from(wasm: SscCerts) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::mpc::SscCert>> for SscCerts {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::mpc::SscCert> {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCommitment(cml_multi_era::byron::mpc::SscCommitment);

#[wasm_bindgen]
impl SscCommitment {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscCommitment, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscCommitment, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn vss_shares(&self) -> VssShares {
        self.0.vss_shares.clone().into()
    }

    pub fn vss_proof(&self) -> VssProof {
        self.0.vss_proof.clone().into()
    }

    pub fn new(vss_shares: &VssShares, vss_proof: &VssProof) -> Self {
        Self(cml_multi_era::byron::mpc::SscCommitment::new(
            vss_shares.clone().into(),
            vss_proof.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::mpc::SscCommitment> for SscCommitment {
    fn from(native: cml_multi_era::byron::mpc::SscCommitment) -> Self {
        Self(native)
    }
}

impl From<SscCommitment> for cml_multi_era::byron::mpc::SscCommitment {
    fn from(wasm: SscCommitment) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscCommitment> for SscCommitment {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscCommitment {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCommitmentsPayload(cml_multi_era::byron::mpc::SscCommitmentsPayload);

#[wasm_bindgen]
impl SscCommitmentsPayload {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscCommitmentsPayload, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscCommitmentsPayload, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn ssc_signed_commitments(&self) -> SscSignedCommitments {
        self.0.ssc_signed_commitments.clone().into()
    }

    pub fn ssc_certs(&self) -> SscCerts {
        self.0.ssc_certs.clone().into()
    }

    pub fn new(ssc_signed_commitments: &SscSignedCommitments, ssc_certs: &SscCerts) -> Self {
        Self(cml_multi_era::byron::mpc::SscCommitmentsPayload::new(
            ssc_signed_commitments.clone().into(),
            ssc_certs.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::mpc::SscCommitmentsPayload> for SscCommitmentsPayload {
    fn from(native: cml_multi_era::byron::mpc::SscCommitmentsPayload) -> Self {
        Self(native)
    }
}

impl From<SscCommitmentsPayload> for cml_multi_era::byron::mpc::SscCommitmentsPayload {
    fn from(wasm: SscCommitmentsPayload) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscCommitmentsPayload> for SscCommitmentsPayload {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscCommitmentsPayload {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCommitmentsProof(cml_multi_era::byron::mpc::SscCommitmentsProof);

#[wasm_bindgen]
impl SscCommitmentsProof {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscCommitmentsProof, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscCommitmentsProof, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn blake2b256(&self) -> Blake2b256 {
        self.0.blake2b256.clone().into()
    }

    pub fn blake2b2562(&self) -> Blake2b256 {
        self.0.blake2b2562.clone().into()
    }

    pub fn new(blake2b256: &Blake2b256, blake2b2562: &Blake2b256) -> Self {
        Self(cml_multi_era::byron::mpc::SscCommitmentsProof::new(
            blake2b256.clone().into(),
            blake2b2562.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::mpc::SscCommitmentsProof> for SscCommitmentsProof {
    fn from(native: cml_multi_era::byron::mpc::SscCommitmentsProof) -> Self {
        Self(native)
    }
}

impl From<SscCommitmentsProof> for cml_multi_era::byron::mpc::SscCommitmentsProof {
    fn from(wasm: SscCommitmentsProof) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscCommitmentsProof> for SscCommitmentsProof {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscCommitmentsProof {
        &self.0
    }
}

#[wasm_bindgen]
pub enum SscKind {
    SscCommitmentsPayload,
    SscOpeningsPayload,
    SscSharesPayload,
    SscCertificatesPayload,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscOpeningsPayload(cml_multi_era::byron::mpc::SscOpeningsPayload);

#[wasm_bindgen]
impl SscOpeningsPayload {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscOpeningsPayload, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscOpeningsPayload, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn ssc_opens(&self) -> SscOpens {
        self.0.ssc_opens.clone().into()
    }

    pub fn ssc_certs(&self) -> SscCerts {
        self.0.ssc_certs.clone().into()
    }

    pub fn new(ssc_opens: &SscOpens, ssc_certs: &SscCerts) -> Self {
        Self(cml_multi_era::byron::mpc::SscOpeningsPayload::new(
            ssc_opens.clone().into(),
            ssc_certs.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::mpc::SscOpeningsPayload> for SscOpeningsPayload {
    fn from(native: cml_multi_era::byron::mpc::SscOpeningsPayload) -> Self {
        Self(native)
    }
}

impl From<SscOpeningsPayload> for cml_multi_era::byron::mpc::SscOpeningsPayload {
    fn from(wasm: SscOpeningsPayload) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscOpeningsPayload> for SscOpeningsPayload {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscOpeningsPayload {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscOpeningsProof(cml_multi_era::byron::mpc::SscOpeningsProof);

#[wasm_bindgen]
impl SscOpeningsProof {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscOpeningsProof, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscOpeningsProof, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn blake2b256(&self) -> Blake2b256 {
        self.0.blake2b256.clone().into()
    }

    pub fn blake2b2562(&self) -> Blake2b256 {
        self.0.blake2b2562.clone().into()
    }

    pub fn new(blake2b256: &Blake2b256, blake2b2562: &Blake2b256) -> Self {
        Self(cml_multi_era::byron::mpc::SscOpeningsProof::new(
            blake2b256.clone().into(),
            blake2b2562.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::mpc::SscOpeningsProof> for SscOpeningsProof {
    fn from(native: cml_multi_era::byron::mpc::SscOpeningsProof) -> Self {
        Self(native)
    }
}

impl From<SscOpeningsProof> for cml_multi_era::byron::mpc::SscOpeningsProof {
    fn from(wasm: SscOpeningsProof) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscOpeningsProof> for SscOpeningsProof {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscOpeningsProof {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscOpens(cml_multi_era::byron::mpc::SscOpens);

#[wasm_bindgen]
impl SscOpens {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &StakeholderId, value: Vsssec) -> Option<Vsssec> {
        self.0.insert(key.clone().into(), value)
    }

    pub fn get(&self, key: &StakeholderId) -> Option<Vsssec> {
        self.0.get(key.as_ref()).map(|v| v.clone())
    }

    pub fn keys(&self) -> StakeholderIdList {
        StakeholderIdList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<cml_multi_era::byron::mpc::SscOpens> for SscOpens {
    fn from(native: cml_multi_era::byron::mpc::SscOpens) -> Self {
        Self(native)
    }
}

impl From<SscOpens> for cml_multi_era::byron::mpc::SscOpens {
    fn from(wasm: SscOpens) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscOpens> for SscOpens {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscOpens {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscProof(cml_multi_era::byron::mpc::SscProof);

#[wasm_bindgen]
impl SscProof {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscProof, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscProof, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_ssc_commitments_proof(ssc_commitments_proof: &SscCommitmentsProof) -> Self {
        Self(
            cml_multi_era::byron::mpc::SscProof::new_ssc_commitments_proof(
                ssc_commitments_proof.clone().into(),
            ),
        )
    }

    pub fn new_ssc_openings_proof(ssc_openings_proof: &SscOpeningsProof) -> Self {
        Self(cml_multi_era::byron::mpc::SscProof::new_ssc_openings_proof(
            ssc_openings_proof.clone().into(),
        ))
    }

    pub fn new_ssc_shares_proof(ssc_shares_proof: &SscSharesProof) -> Self {
        Self(cml_multi_era::byron::mpc::SscProof::new_ssc_shares_proof(
            ssc_shares_proof.clone().into(),
        ))
    }

    pub fn new_ssc_certificates_proof(ssc_certificates_proof: &SscCertificatesProof) -> Self {
        Self(
            cml_multi_era::byron::mpc::SscProof::new_ssc_certificates_proof(
                ssc_certificates_proof.clone().into(),
            ),
        )
    }

    pub fn kind(&self) -> SscProofKind {
        match &self.0 {
            cml_multi_era::byron::mpc::SscProof::SscCommitmentsProof(_) => {
                SscProofKind::SscCommitmentsProof
            }
            cml_multi_era::byron::mpc::SscProof::SscOpeningsProof(_) => {
                SscProofKind::SscOpeningsProof
            }
            cml_multi_era::byron::mpc::SscProof::SscSharesProof(_) => SscProofKind::SscSharesProof,
            cml_multi_era::byron::mpc::SscProof::SscCertificatesProof(_) => {
                SscProofKind::SscCertificatesProof
            }
        }
    }

    pub fn as_ssc_commitments_proof(&self) -> Option<SscCommitmentsProof> {
        match &self.0 {
            cml_multi_era::byron::mpc::SscProof::SscCommitmentsProof(ssc_commitments_proof) => {
                Some(ssc_commitments_proof.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_ssc_openings_proof(&self) -> Option<SscOpeningsProof> {
        match &self.0 {
            cml_multi_era::byron::mpc::SscProof::SscOpeningsProof(ssc_openings_proof) => {
                Some(ssc_openings_proof.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_ssc_shares_proof(&self) -> Option<SscSharesProof> {
        match &self.0 {
            cml_multi_era::byron::mpc::SscProof::SscSharesProof(ssc_shares_proof) => {
                Some(ssc_shares_proof.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_ssc_certificates_proof(&self) -> Option<SscCertificatesProof> {
        match &self.0 {
            cml_multi_era::byron::mpc::SscProof::SscCertificatesProof(ssc_certificates_proof) => {
                Some(ssc_certificates_proof.clone().into())
            }
            _ => None,
        }
    }
}

impl From<cml_multi_era::byron::mpc::SscProof> for SscProof {
    fn from(native: cml_multi_era::byron::mpc::SscProof) -> Self {
        Self(native)
    }
}

impl From<SscProof> for cml_multi_era::byron::mpc::SscProof {
    fn from(wasm: SscProof) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscProof> for SscProof {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscProof {
        &self.0
    }
}

#[wasm_bindgen]
pub enum SscProofKind {
    SscCommitmentsProof,
    SscOpeningsProof,
    SscSharesProof,
    SscCertificatesProof,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscShares(cml_multi_era::byron::mpc::SscShares);

#[wasm_bindgen]
impl SscShares {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, key: &AddressId, value: &SscSharesSubmap) -> Option<SscSharesSubmap> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &AddressId) -> Option<SscSharesSubmap> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> AddressIdList {
        AddressIdList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<cml_multi_era::byron::mpc::SscShares> for SscShares {
    fn from(native: cml_multi_era::byron::mpc::SscShares) -> Self {
        Self(native)
    }
}

impl From<SscShares> for cml_multi_era::byron::mpc::SscShares {
    fn from(wasm: SscShares) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscShares> for SscShares {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscShares {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscSharesPayload(cml_multi_era::byron::mpc::SscSharesPayload);

#[wasm_bindgen]
impl SscSharesPayload {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscSharesPayload, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscSharesPayload, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn ssc_shares(&self) -> SscShares {
        self.0.ssc_shares.clone().into()
    }

    pub fn ssc_certs(&self) -> SscCerts {
        self.0.ssc_certs.clone().into()
    }

    pub fn new(ssc_shares: &SscShares, ssc_certs: &SscCerts) -> Self {
        Self(cml_multi_era::byron::mpc::SscSharesPayload::new(
            ssc_shares.clone().into(),
            ssc_certs.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::mpc::SscSharesPayload> for SscSharesPayload {
    fn from(native: cml_multi_era::byron::mpc::SscSharesPayload) -> Self {
        Self(native)
    }
}

impl From<SscSharesPayload> for cml_multi_era::byron::mpc::SscSharesPayload {
    fn from(wasm: SscSharesPayload) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscSharesPayload> for SscSharesPayload {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscSharesPayload {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscSharesProof(cml_multi_era::byron::mpc::SscSharesProof);

#[wasm_bindgen]
impl SscSharesProof {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscSharesProof, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscSharesProof, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn blake2b256(&self) -> Blake2b256 {
        self.0.blake2b256.clone().into()
    }

    pub fn blake2b2562(&self) -> Blake2b256 {
        self.0.blake2b2562.clone().into()
    }

    pub fn new(blake2b256: &Blake2b256, blake2b2562: &Blake2b256) -> Self {
        Self(cml_multi_era::byron::mpc::SscSharesProof::new(
            blake2b256.clone().into(),
            blake2b2562.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::mpc::SscSharesProof> for SscSharesProof {
    fn from(native: cml_multi_era::byron::mpc::SscSharesProof) -> Self {
        Self(native)
    }
}

impl From<SscSharesProof> for cml_multi_era::byron::mpc::SscSharesProof {
    fn from(wasm: SscSharesProof) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscSharesProof> for SscSharesProof {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscSharesProof {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscSharesSubmap(cml_multi_era::byron::mpc::SscSharesSubmap);

#[wasm_bindgen]
impl SscSharesSubmap {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: &AddressId,
        value: &VssDecryptedShareList,
    ) -> Option<VssDecryptedShareList> {
        self.0
            .insert(key.clone().into(), value.clone().into())
            .map(Into::into)
    }

    pub fn get(&self, key: &AddressId) -> Option<VssDecryptedShareList> {
        self.0.get(key.as_ref()).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> AddressIdList {
        AddressIdList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<cml_multi_era::byron::mpc::SscSharesSubmap> for SscSharesSubmap {
    fn from(native: cml_multi_era::byron::mpc::SscSharesSubmap) -> Self {
        Self(native)
    }
}

impl From<SscSharesSubmap> for cml_multi_era::byron::mpc::SscSharesSubmap {
    fn from(wasm: SscSharesSubmap) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscSharesSubmap> for SscSharesSubmap {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscSharesSubmap {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscSignedCommitment(cml_multi_era::byron::mpc::SscSignedCommitment);

#[wasm_bindgen]
impl SscSignedCommitment {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<SscSignedCommitment, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<SscSignedCommitment, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn byron_pub_key(&self) -> ByronPubKey {
        self.0.byron_pub_key.clone()
    }

    pub fn ssc_commitment(&self) -> SscCommitment {
        self.0.ssc_commitment.clone().into()
    }

    pub fn byron_signature(&self) -> ByronSignature {
        self.0.byron_signature.clone()
    }

    pub fn new(
        byron_pub_key: ByronPubKey,
        ssc_commitment: &SscCommitment,
        byron_signature: ByronSignature,
    ) -> Self {
        Self(cml_multi_era::byron::mpc::SscSignedCommitment::new(
            byron_pub_key,
            ssc_commitment.clone().into(),
            byron_signature,
        ))
    }
}

impl From<cml_multi_era::byron::mpc::SscSignedCommitment> for SscSignedCommitment {
    fn from(native: cml_multi_era::byron::mpc::SscSignedCommitment) -> Self {
        Self(native)
    }
}

impl From<SscSignedCommitment> for cml_multi_era::byron::mpc::SscSignedCommitment {
    fn from(wasm: SscSignedCommitment) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::SscSignedCommitment> for SscSignedCommitment {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::SscSignedCommitment {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscSignedCommitments(Vec<cml_multi_era::byron::mpc::SscSignedCommitment>);

#[wasm_bindgen]
impl SscSignedCommitments {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> SscSignedCommitment {
        self.0[index].clone().into()
    }

    pub fn add(&mut self, elem: &SscSignedCommitment) {
        self.0.push(elem.clone().into());
    }
}

impl From<Vec<cml_multi_era::byron::mpc::SscSignedCommitment>> for SscSignedCommitments {
    fn from(native: Vec<cml_multi_era::byron::mpc::SscSignedCommitment>) -> Self {
        Self(native)
    }
}

impl From<SscSignedCommitments> for Vec<cml_multi_era::byron::mpc::SscSignedCommitment> {
    fn from(wasm: SscSignedCommitments) -> Self {
        wasm.0
    }
}

impl AsRef<Vec<cml_multi_era::byron::mpc::SscSignedCommitment>> for SscSignedCommitments {
    fn as_ref(&self) -> &Vec<cml_multi_era::byron::mpc::SscSignedCommitment> {
        &self.0
    }
}

pub type VssDecryptedShare = Vec<u8>;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VssEncryptedShare(cml_multi_era::byron::mpc::VssEncryptedShare);

#[wasm_bindgen]
impl VssEncryptedShare {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<VssEncryptedShare, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<VssEncryptedShare, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_0(&self) -> Vec<u8> {
        self.0.index_0.clone()
    }

    pub fn new(index_0: Vec<u8>) -> Self {
        Self(cml_multi_era::byron::mpc::VssEncryptedShare::new(index_0))
    }
}

impl From<cml_multi_era::byron::mpc::VssEncryptedShare> for VssEncryptedShare {
    fn from(native: cml_multi_era::byron::mpc::VssEncryptedShare) -> Self {
        Self(native)
    }
}

impl From<VssEncryptedShare> for cml_multi_era::byron::mpc::VssEncryptedShare {
    fn from(wasm: VssEncryptedShare) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::VssEncryptedShare> for VssEncryptedShare {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::VssEncryptedShare {
        &self.0
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VssProof(cml_multi_era::byron::mpc::VssProof);

#[wasm_bindgen]
impl VssProof {
    pub fn to_cbor_bytes(&self) -> Vec<u8> {
        cml_multi_era::serialization::ToCBORBytes::to_cbor_bytes(&self.0)
    }

    pub fn from_cbor_bytes(cbor_bytes: &[u8]) -> Result<VssProof, JsValue> {
        cml_multi_era::serialization::Deserialize::from_cbor_bytes(cbor_bytes)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.0)
            .map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<VssProof, JsValue> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn extra_gen(&self) -> Vec<u8> {
        self.0.extra_gen.clone()
    }

    pub fn proof(&self) -> Vec<u8> {
        self.0.proof.clone()
    }

    pub fn parallel_proofs(&self) -> Vec<u8> {
        self.0.parallel_proofs.clone()
    }

    pub fn bytess(&self) -> BytesList {
        self.0.bytess.clone().into()
    }

    pub fn new(
        extra_gen: Vec<u8>,
        proof: Vec<u8>,
        parallel_proofs: Vec<u8>,
        bytess: &BytesList,
    ) -> Self {
        Self(cml_multi_era::byron::mpc::VssProof::new(
            extra_gen,
            proof,
            parallel_proofs,
            bytess.clone().into(),
        ))
    }
}

impl From<cml_multi_era::byron::mpc::VssProof> for VssProof {
    fn from(native: cml_multi_era::byron::mpc::VssProof) -> Self {
        Self(native)
    }
}

impl From<VssProof> for cml_multi_era::byron::mpc::VssProof {
    fn from(wasm: VssProof) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::VssProof> for VssProof {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::VssProof {
        &self.0
    }
}

pub type VssPubKey = Vec<u8>;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VssShares(cml_multi_era::byron::mpc::VssShares);

#[wasm_bindgen]
impl VssShares {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(
        &mut self,
        key: VssPubKey,
        value: &VssEncryptedShare,
    ) -> Option<VssEncryptedShare> {
        self.0.insert(key, value.clone().into()).map(Into::into)
    }

    pub fn get(&self, key: VssPubKey) -> Option<VssEncryptedShare> {
        self.0.get(&key).map(|v| v.clone().into())
    }

    pub fn keys(&self) -> VssPubKeyList {
        VssPubKeyList(self.0.iter().map(|(k, _v)| k.clone()).collect::<Vec<_>>())
    }
}

impl From<cml_multi_era::byron::mpc::VssShares> for VssShares {
    fn from(native: cml_multi_era::byron::mpc::VssShares) -> Self {
        Self(native)
    }
}

impl From<VssShares> for cml_multi_era::byron::mpc::VssShares {
    fn from(wasm: VssShares) -> Self {
        wasm.0
    }
}

impl AsRef<cml_multi_era::byron::mpc::VssShares> for VssShares {
    fn as_ref(&self) -> &cml_multi_era::byron::mpc::VssShares {
        &self.0
    }
}

pub type Vsssec = Vec<u8>;
