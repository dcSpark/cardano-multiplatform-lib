// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::byron::{
    AddressIdList, Blake2b256, ByronPubKey, ByronSignature, BytesList, EpochId, StakeholderIdList,
    VssDecryptedShareList, VssPubKeyList,
};
use crate::impl_wasm_cbor_json_api_byron;
use cml_chain_wasm::byron::{AddressId, StakeholderId};
use cml_core_wasm::impl_wasm_conversions;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Ssc(cml_multi_era::byron::mpc::Ssc);

impl_wasm_cbor_json_api_byron!(Ssc);

impl_wasm_conversions!(cml_multi_era::byron::mpc::Ssc, Ssc);

#[wasm_bindgen]
impl Ssc {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCert(cml_multi_era::byron::mpc::SscCert);

impl_wasm_cbor_json_api_byron!(SscCert);

impl_wasm_conversions!(cml_multi_era::byron::mpc::SscCert, SscCert);

#[wasm_bindgen]
impl SscCert {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCertificatesPayload(cml_multi_era::byron::mpc::SscCertificatesPayload);

impl_wasm_cbor_json_api_byron!(SscCertificatesPayload);

impl_wasm_conversions!(
    cml_multi_era::byron::mpc::SscCertificatesPayload,
    SscCertificatesPayload
);

#[wasm_bindgen]
impl SscCertificatesPayload {
    pub fn ssc_certs(&self) -> SscCerts {
        self.0.ssc_certs.clone().into()
    }

    pub fn new(ssc_certs: &SscCerts) -> Self {
        Self(cml_multi_era::byron::mpc::SscCertificatesPayload::new(
            ssc_certs.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCertificatesProof(cml_multi_era::byron::mpc::SscCertificatesProof);

impl_wasm_cbor_json_api_byron!(SscCertificatesProof);

impl_wasm_conversions!(
    cml_multi_era::byron::mpc::SscCertificatesProof,
    SscCertificatesProof
);

#[wasm_bindgen]
impl SscCertificatesProof {
    pub fn blake2b256(&self) -> Blake2b256 {
        self.0.blake2b256.clone().into()
    }

    pub fn new(blake2b256: &Blake2b256) -> Self {
        Self(cml_multi_era::byron::mpc::SscCertificatesProof::new(
            blake2b256.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCerts(Vec<cml_multi_era::byron::mpc::SscCert>);

impl_wasm_conversions!(Vec<cml_multi_era::byron::mpc::SscCert>, SscCerts);

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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCommitment(cml_multi_era::byron::mpc::SscCommitment);

impl_wasm_cbor_json_api_byron!(SscCommitment);

impl_wasm_conversions!(cml_multi_era::byron::mpc::SscCommitment, SscCommitment);

#[wasm_bindgen]
impl SscCommitment {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCommitmentsPayload(cml_multi_era::byron::mpc::SscCommitmentsPayload);

impl_wasm_cbor_json_api_byron!(SscCommitmentsPayload);

impl_wasm_conversions!(
    cml_multi_era::byron::mpc::SscCommitmentsPayload,
    SscCommitmentsPayload
);

#[wasm_bindgen]
impl SscCommitmentsPayload {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscCommitmentsProof(cml_multi_era::byron::mpc::SscCommitmentsProof);

impl_wasm_cbor_json_api_byron!(SscCommitmentsProof);

impl_wasm_conversions!(
    cml_multi_era::byron::mpc::SscCommitmentsProof,
    SscCommitmentsProof
);

#[wasm_bindgen]
impl SscCommitmentsProof {
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

impl_wasm_cbor_json_api_byron!(SscOpeningsPayload);

impl_wasm_conversions!(
    cml_multi_era::byron::mpc::SscOpeningsPayload,
    SscOpeningsPayload
);

#[wasm_bindgen]
impl SscOpeningsPayload {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscOpeningsProof(cml_multi_era::byron::mpc::SscOpeningsProof);

impl_wasm_cbor_json_api_byron!(SscOpeningsProof);

impl_wasm_conversions!(
    cml_multi_era::byron::mpc::SscOpeningsProof,
    SscOpeningsProof
);

#[wasm_bindgen]
impl SscOpeningsProof {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscOpens(cml_multi_era::byron::mpc::SscOpens);

impl_wasm_conversions!(cml_multi_era::byron::mpc::SscOpens, SscOpens);

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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscProof(cml_multi_era::byron::mpc::SscProof);

impl_wasm_cbor_json_api_byron!(SscProof);

impl_wasm_conversions!(cml_multi_era::byron::mpc::SscProof, SscProof);

#[wasm_bindgen]
impl SscProof {
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

impl_wasm_conversions!(cml_multi_era::byron::mpc::SscShares, SscShares);

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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscSharesPayload(cml_multi_era::byron::mpc::SscSharesPayload);

impl_wasm_cbor_json_api_byron!(SscSharesPayload);

impl_wasm_conversions!(
    cml_multi_era::byron::mpc::SscSharesPayload,
    SscSharesPayload
);

#[wasm_bindgen]
impl SscSharesPayload {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscSharesProof(cml_multi_era::byron::mpc::SscSharesProof);

impl_wasm_cbor_json_api_byron!(SscSharesProof);

impl_wasm_conversions!(cml_multi_era::byron::mpc::SscSharesProof, SscSharesProof);

#[wasm_bindgen]
impl SscSharesProof {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscSharesSubmap(cml_multi_era::byron::mpc::SscSharesSubmap);

impl_wasm_conversions!(cml_multi_era::byron::mpc::SscSharesSubmap, SscSharesSubmap);

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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscSignedCommitment(cml_multi_era::byron::mpc::SscSignedCommitment);

impl_wasm_cbor_json_api_byron!(SscSignedCommitment);

impl_wasm_conversions!(
    cml_multi_era::byron::mpc::SscSignedCommitment,
    SscSignedCommitment
);

#[wasm_bindgen]
impl SscSignedCommitment {
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SscSignedCommitments(Vec<cml_multi_era::byron::mpc::SscSignedCommitment>);

impl_wasm_conversions!(
    Vec<cml_multi_era::byron::mpc::SscSignedCommitment>,
    SscSignedCommitments
);

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

pub type VssDecryptedShare = Vec<u8>;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VssEncryptedShare(cml_multi_era::byron::mpc::VssEncryptedShare);

impl_wasm_cbor_json_api_byron!(VssEncryptedShare);

impl_wasm_conversions!(
    cml_multi_era::byron::mpc::VssEncryptedShare,
    VssEncryptedShare
);

#[wasm_bindgen]
impl VssEncryptedShare {
    pub fn index_0(&self) -> Vec<u8> {
        self.0.index_0.clone()
    }

    pub fn new(index_0: Vec<u8>) -> Self {
        Self(cml_multi_era::byron::mpc::VssEncryptedShare::new(index_0))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VssProof(cml_multi_era::byron::mpc::VssProof);

impl_wasm_cbor_json_api_byron!(VssProof);

impl_wasm_conversions!(cml_multi_era::byron::mpc::VssProof, VssProof);

#[wasm_bindgen]
impl VssProof {
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

pub type VssPubKey = Vec<u8>;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VssShares(cml_multi_era::byron::mpc::VssShares);

impl_wasm_conversions!(cml_multi_era::byron::mpc::VssShares, VssShares);

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

pub type Vsssec = Vec<u8>;
