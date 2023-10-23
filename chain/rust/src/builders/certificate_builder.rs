use crate::builders::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};
use crate::*;
use std::collections::HashSet;

use super::witness_builder::{NativeScriptWitnessInfo, RequiredWitnessSet};

use crate::{
    certs::{Certificate, StakeCredential},
    transaction::RequiredSigners,
};

use cml_crypto::{Ed25519KeyHash, ScriptHash};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum CertBuilderError {
    #[error("Deregistration certificate contains script. Expected public key hash.\n{0:?}")]
    ExpectedKeyHash(ScriptHash),
    #[error("Deregistration certificate contains keyhash. Expected script hash.\n{0:?}")]
    ExpectedScriptHash(Certificate),
    #[error("Missing the following witnesses for the certificate: {0:?}")]
    MissingWitnesses(RequiredWitnessSet),
}

// comes from witsVKeyNeeded in the Ledger spec
pub fn cert_required_wits(cert: &Certificate, required_witnesses: &mut RequiredWitnessSet) {
    match cert {
        // stake key registrations do not require a witness
        Certificate::StakeRegistration(_cert) => (),
        Certificate::StakeDeregistration(cert) => match &cert.stake_credential {
            StakeCredential::Script { hash, .. } => {
                required_witnesses.add_script_hash(*hash);
            }
            StakeCredential::PubKey { hash, .. } => {
                required_witnesses.add_vkey_key_hash(*hash);
            }
        },
        Certificate::StakeDelegation(cert) => match &cert.stake_credential {
            StakeCredential::Script { hash, .. } => {
                required_witnesses.add_script_hash(*hash);
            }
            StakeCredential::PubKey { hash, .. } => {
                required_witnesses.add_vkey_key_hash(*hash);
            }
        },
        Certificate::PoolRegistration(cert) => {
            for owner in &cert.pool_params.pool_owners {
                required_witnesses.add_vkey_key_hash(*owner);
            }
            required_witnesses.add_vkey_key_hash(cert.pool_params.operator);
        }
        Certificate::PoolRetirement(cert) => {
            required_witnesses.add_vkey_key_hash(cert.ed25519_key_hash);
        }
        Certificate::RegCert(_cert) => todo!(),
        Certificate::UnregCert(_cert) => todo!(),
        Certificate::VoteDelegCert(_cert) => todo!(),
        Certificate::StakeVoteDelegCert(_cert) => todo!(),
        Certificate::StakeRegDelegCert(_cert) => todo!(),
        Certificate::VoteRegDelegCert(_cert) => todo!(),
        Certificate::StakeVoteRegDelegCert(_cert) => todo!(),
        Certificate::AuthCommitteeHotCert(_cert) => todo!(),
        Certificate::ResignCommitteeColdCert(_cert) => todo!(),
        Certificate::RegDrepCert(_cert) => todo!(),
        Certificate::UnregDrepCert(_cert) => todo!(),
        Certificate::UpdateDrepCert(_cert) => todo!(),
    };
}

// comes from witsVKeyNeeded in the Ledger spec
#[allow(clippy::result_large_err)]
pub fn add_cert_vkeys(
    cert: &Certificate,
    vkeys: &mut HashSet<Ed25519KeyHash>,
) -> Result<(), CertBuilderError> {
    match cert {
        // stake key registrations do not require a witness
        Certificate::StakeRegistration(_cert) => {}
        Certificate::StakeDeregistration(cert) => match &cert.stake_credential {
            StakeCredential::Script { hash, .. } => {
                return Err(CertBuilderError::ExpectedKeyHash(*hash))
            }
            StakeCredential::PubKey { hash, .. } => {
                vkeys.insert(*hash);
            }
        },
        Certificate::StakeDelegation(cert) => match &cert.stake_credential {
            StakeCredential::Script { hash, .. } => {
                return Err(CertBuilderError::ExpectedKeyHash(*hash))
            }
            StakeCredential::PubKey { hash, .. } => {
                vkeys.insert(*hash);
            }
        },
        Certificate::PoolRegistration(cert) => {
            for owner in &cert.pool_params.pool_owners {
                vkeys.insert(*owner);
            }
            vkeys.insert(cert.pool_params.operator);
        }
        Certificate::PoolRetirement(cert) => {
            vkeys.insert(cert.ed25519_key_hash);
        }
        Certificate::RegCert(_cert) => todo!(),
        Certificate::UnregCert(_cert) => todo!(),
        Certificate::VoteDelegCert(_cert) => todo!(),
        Certificate::StakeVoteDelegCert(_cert) => todo!(),
        Certificate::StakeRegDelegCert(_cert) => todo!(),
        Certificate::VoteRegDelegCert(_cert) => todo!(),
        Certificate::StakeVoteRegDelegCert(_cert) => todo!(),
        Certificate::AuthCommitteeHotCert(_cert) => todo!(),
        Certificate::ResignCommitteeColdCert(_cert) => todo!(),
        Certificate::RegDrepCert(_cert) => todo!(),
        Certificate::UnregDrepCert(_cert) => todo!(),
        Certificate::UpdateDrepCert(_cert) => todo!(),
    };
    Ok(())
}

#[derive(Clone)]
pub struct CertificateBuilderResult {
    pub cert: Certificate,
    pub aggregate_witness: Option<InputAggregateWitnessData>,
    pub required_wits: RequiredWitnessSet,
}

#[derive(Clone)]
pub struct SingleCertificateBuilder {
    cert: Certificate,
}

impl SingleCertificateBuilder {
    pub fn new(cert: Certificate) -> Self {
        Self { cert }
    }

    /// note: particularly useful for StakeRegistration which doesn't require witnessing
    pub fn skip_witness(self) -> CertificateBuilderResult {
        let mut required_wits = RequiredWitnessSet::default();
        cert_required_wits(&self.cert, &mut required_wits);

        CertificateBuilderResult {
            cert: self.cert,
            aggregate_witness: None,
            required_wits,
        }
    }

    #[allow(clippy::result_large_err)]
    pub fn payment_key(self) -> Result<CertificateBuilderResult, CertBuilderError> {
        let mut required_wits = RequiredWitnessSet::default();
        cert_required_wits(&self.cert, &mut required_wits);

        if !required_wits.scripts.is_empty() {
            return Err(CertBuilderError::ExpectedScriptHash(self.cert.clone()));
        }

        Ok(CertificateBuilderResult {
            cert: self.cert,
            aggregate_witness: None,
            required_wits,
        })
    }

    /** Signer keys don't have to be set. You can leave it empty and then add the required witnesses later */
    #[allow(clippy::result_large_err)]
    pub fn native_script(
        self,
        native_script: NativeScript,
        witness_info: NativeScriptWitnessInfo,
    ) -> Result<CertificateBuilderResult, CertBuilderError> {
        let mut required_wits = RequiredWitnessSet::default();
        cert_required_wits(&self.cert, &mut required_wits);
        let mut required_wits_left = required_wits.clone();

        let script_hash = native_script.hash();

        // the user may have provided more witnesses than required. Strip it down to just the required wits
        // often happens because users aren't aware StakeRegistration doesn't require a witness
        let contains = required_wits_left.scripts.contains(&script_hash);

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(&script_hash);

        if !required_wits_left.scripts.is_empty() {
            return Err(CertBuilderError::MissingWitnesses(required_wits_left));
        }

        Ok(CertificateBuilderResult {
            cert: self.cert,
            aggregate_witness: if contains {
                Some(InputAggregateWitnessData::NativeScript(
                    native_script,
                    witness_info,
                ))
            } else {
                None
            },
            required_wits,
        })
    }

    #[allow(clippy::result_large_err)]
    pub fn plutus_script(
        self,
        partial_witness: PartialPlutusWitness,
        required_signers: RequiredSigners,
    ) -> Result<CertificateBuilderResult, CertBuilderError> {
        let mut required_wits = RequiredWitnessSet::default();
        required_signers
            .iter()
            .for_each(|required_signer| required_wits.add_vkey_key_hash(*required_signer));
        cert_required_wits(&self.cert, &mut required_wits);
        let mut required_wits_left = required_wits.clone();

        // no way to know these at this time
        required_wits_left.vkeys.clear();

        let script_hash = partial_witness.script.hash();

        // the user may have provided more witnesses than required. Strip it down to just the required wits
        // often happens because users aren't aware StakeRegistration doesn't require a witness
        let contains = required_wits_left.scripts.contains(&script_hash);

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(&script_hash);

        if required_wits_left.len() > 0 {
            return Err(CertBuilderError::MissingWitnesses(required_wits_left));
        }

        Ok(CertificateBuilderResult {
            cert: self.cert,
            aggregate_witness: if contains {
                Some(InputAggregateWitnessData::PlutusScript(
                    partial_witness,
                    required_signers,
                    None,
                ))
            } else {
                None
            },
            required_wits,
        })
    }
}
