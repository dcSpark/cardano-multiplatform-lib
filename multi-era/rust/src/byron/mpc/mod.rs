// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

pub mod serialization;

use crate::byron::{Blake2b256, ByronPubKey, ByronSignature, EpochId};
use cml_chain::byron::{AddressId, StakeholderId};
use std::collections::BTreeMap;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum Ssc {
    SscCommitmentsPayload(SscCommitmentsPayload),
    SscOpeningsPayload(SscOpeningsPayload),
    SscSharesPayload(SscSharesPayload),
    SscCertificatesPayload(SscCertificatesPayload),
}

impl Ssc {
    pub fn new_ssc_commitments_payload(ssc_commitments_payload: SscCommitmentsPayload) -> Self {
        Self::SscCommitmentsPayload(ssc_commitments_payload)
    }

    pub fn new_ssc_openings_payload(ssc_openings_payload: SscOpeningsPayload) -> Self {
        Self::SscOpeningsPayload(ssc_openings_payload)
    }

    pub fn new_ssc_shares_payload(ssc_shares_payload: SscSharesPayload) -> Self {
        Self::SscSharesPayload(ssc_shares_payload)
    }

    pub fn new_ssc_certificates_payload(ssc_certificates_payload: SscCertificatesPayload) -> Self {
        Self::SscCertificatesPayload(ssc_certificates_payload)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SscCert {
    pub vss_pub_key: VssPubKey,
    pub epoch_id: EpochId,
    pub byron_pub_key: ByronPubKey,
    pub byron_signature: ByronSignature,
}

impl SscCert {
    pub fn new(
        vss_pub_key: VssPubKey,
        epoch_id: EpochId,
        byron_pub_key: ByronPubKey,
        byron_signature: ByronSignature,
    ) -> Self {
        Self {
            vss_pub_key,
            epoch_id,
            byron_pub_key,
            byron_signature,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SscCertificatesPayload {
    pub ssc_certs: SscCerts,
}

impl SscCertificatesPayload {
    pub fn new(ssc_certs: SscCerts) -> Self {
        Self { ssc_certs }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SscCertificatesProof {
    pub blake2b256: Blake2b256,
}

impl SscCertificatesProof {
    pub fn new(blake2b256: Blake2b256) -> Self {
        Self { blake2b256 }
    }
}

pub type SscCerts = Vec<SscCert>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SscCommitment {
    pub vss_shares: VssShares,
    pub vss_proof: VssProof,
}

impl SscCommitment {
    pub fn new(vss_shares: VssShares, vss_proof: VssProof) -> Self {
        Self {
            vss_shares,
            vss_proof,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SscCommitmentsPayload {
    pub ssc_signed_commitments: SscSignedCommitments,
    pub ssc_certs: SscCerts,
}

impl SscCommitmentsPayload {
    pub fn new(ssc_signed_commitments: SscSignedCommitments, ssc_certs: SscCerts) -> Self {
        Self {
            ssc_signed_commitments,
            ssc_certs,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SscCommitmentsProof {
    pub blake2b256: Blake2b256,
    pub blake2b2562: Blake2b256,
}

impl SscCommitmentsProof {
    pub fn new(blake2b256: Blake2b256, blake2b2562: Blake2b256) -> Self {
        Self {
            blake2b256,
            blake2b2562,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SscOpeningsPayload {
    pub ssc_opens: SscOpens,
    pub ssc_certs: SscCerts,
}

impl SscOpeningsPayload {
    pub fn new(ssc_opens: SscOpens, ssc_certs: SscCerts) -> Self {
        Self {
            ssc_opens,
            ssc_certs,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SscOpeningsProof {
    pub blake2b256: Blake2b256,
    pub blake2b2562: Blake2b256,
}

impl SscOpeningsProof {
    pub fn new(blake2b256: Blake2b256, blake2b2562: Blake2b256) -> Self {
        Self {
            blake2b256,
            blake2b2562,
        }
    }
}

pub type SscOpens = BTreeMap<StakeholderId, Vsssec>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum SscProof {
    SscCommitmentsProof(SscCommitmentsProof),
    SscOpeningsProof(SscOpeningsProof),
    SscSharesProof(SscSharesProof),
    SscCertificatesProof(SscCertificatesProof),
}

impl SscProof {
    pub fn new_ssc_commitments_proof(ssc_commitments_proof: SscCommitmentsProof) -> Self {
        Self::SscCommitmentsProof(ssc_commitments_proof)
    }

    pub fn new_ssc_openings_proof(ssc_openings_proof: SscOpeningsProof) -> Self {
        Self::SscOpeningsProof(ssc_openings_proof)
    }

    pub fn new_ssc_shares_proof(ssc_shares_proof: SscSharesProof) -> Self {
        Self::SscSharesProof(ssc_shares_proof)
    }

    pub fn new_ssc_certificates_proof(ssc_certificates_proof: SscCertificatesProof) -> Self {
        Self::SscCertificatesProof(ssc_certificates_proof)
    }
}

pub type SscShares = BTreeMap<AddressId, SscSharesSubmap>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SscSharesPayload {
    pub ssc_shares: SscShares,
    pub ssc_certs: SscCerts,
}

impl SscSharesPayload {
    pub fn new(ssc_shares: SscShares, ssc_certs: SscCerts) -> Self {
        Self {
            ssc_shares,
            ssc_certs,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SscSharesProof {
    pub blake2b256: Blake2b256,
    pub blake2b2562: Blake2b256,
}

impl SscSharesProof {
    pub fn new(blake2b256: Blake2b256, blake2b2562: Blake2b256) -> Self {
        Self {
            blake2b256,
            blake2b2562,
        }
    }
}

pub type SscSharesSubmap = BTreeMap<AddressId, Vec<VssDecryptedShare>>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SscSignedCommitment {
    pub byron_pub_key: ByronPubKey,
    pub ssc_commitment: SscCommitment,
    pub byron_signature: ByronSignature,
}

impl SscSignedCommitment {
    pub fn new(
        byron_pub_key: ByronPubKey,
        ssc_commitment: SscCommitment,
        byron_signature: ByronSignature,
    ) -> Self {
        Self {
            byron_pub_key,
            ssc_commitment,
            byron_signature,
        }
    }
}

pub type SscSignedCommitments = Vec<SscSignedCommitment>;

pub type VssDecryptedShare = Vec<u8>;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct VssEncryptedShare {
    pub index_0: Vec<u8>,
}

impl VssEncryptedShare {
    pub fn new(index_0: Vec<u8>) -> Self {
        Self { index_0 }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct VssProof {
    pub extra_gen: Vec<u8>,
    pub proof: Vec<u8>,
    pub parallel_proofs: Vec<u8>,
    pub bytess: Vec<Vec<u8>>,
}

impl VssProof {
    pub fn new(
        extra_gen: Vec<u8>,
        proof: Vec<u8>,
        parallel_proofs: Vec<u8>,
        bytess: Vec<Vec<u8>>,
    ) -> Self {
        Self {
            extra_gen,
            proof,
            parallel_proofs,
            bytess,
        }
    }
}

pub type VssPubKey = Vec<u8>;

pub type VssShares = BTreeMap<VssPubKey, VssEncryptedShare>;

pub type Vsssec = Vec<u8>;
