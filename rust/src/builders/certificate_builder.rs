use crate::*;
use crate::builders::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};
use std::collections::{HashSet};

use super::witness_builder::{RequiredWitnessSet, NativeScriptWitnessInfo};

// comes from witsVKeyNeeded in the Ledger spec
pub fn cert_required_wits(cert_enum: &Certificate, required_witnesses: &mut RequiredWitnessSet) {
    match &cert_enum.0 {
        // stake key registrations do not require a witness
        CertificateEnum::StakeRegistration(_cert) => (),
        CertificateEnum::StakeDeregistration(cert) => match cert.stake_credential().kind() {
            StakeCredKind::Script => {
                required_witnesses.add_script_hash(&cert.stake_credential().to_scripthash().unwrap());
            }
            StakeCredKind::Key => {
                required_witnesses.add_vkey_key_hash(&cert.stake_credential().to_keyhash().unwrap());
            }
        },
        CertificateEnum::StakeDelegation(cert) => match cert.stake_credential().kind() {
            StakeCredKind::Script => {
                required_witnesses.add_script_hash(&cert.stake_credential().to_scripthash().unwrap());
            }
            StakeCredKind::Key => {
                required_witnesses.add_vkey_key_hash(&cert.stake_credential().to_keyhash().unwrap());
            }
        },
        CertificateEnum::PoolRegistration(cert) => {
            for owner in &cert.pool_params().pool_owners().0 {
                required_witnesses.add_vkey_key_hash(&owner.clone());
            }
            required_witnesses.add_vkey_key_hash(&cert.pool_params().operator());
        },
        CertificateEnum::PoolRetirement(cert) => {
            required_witnesses.add_vkey_key_hash(&cert.pool_keyhash());
        },
        CertificateEnum::GenesisKeyDelegation(cert) => {
            required_witnesses.add_vkey_key_hash(
                &Ed25519KeyHash::from_bytes(cert.genesis_delegate_hash().to_bytes()).unwrap()
            );
        },
        // no witness as there is no single core node or genesis key that posts the certificate
        CertificateEnum::MoveInstantaneousRewardsCert(_cert) => {},
    };
}

// comes from witsVKeyNeeded in the Ledger spec
pub fn add_cert_vkeys(cert_enum: &Certificate, vkeys: &mut HashSet<Ed25519KeyHash>) -> Result<(), JsError> {
    match &cert_enum.0 {
        // stake key registrations do not require a witness
        CertificateEnum::StakeRegistration(_cert) => {},
        CertificateEnum::StakeDeregistration(cert) => match cert.stake_credential().kind() {
            StakeCredKind::Script => return Err(JsError::from_str(&format!("Deregistration certificate contains script. Expected public key hash.\n{:#?}", cert.to_json()))),
            StakeCredKind::Key => {
                vkeys.insert(cert.stake_credential().to_keyhash().unwrap());
            }
        },
        CertificateEnum::StakeDelegation(cert) => match cert.stake_credential().kind() {
            StakeCredKind::Script => return Err(JsError::from_str(&format!("Delegation certificate contains script. Expected public key hash.\n{:#?}", cert.to_json()))),
            StakeCredKind::Key => {
                vkeys.insert(cert.stake_credential().to_keyhash().unwrap());
            }
        },
        CertificateEnum::PoolRegistration(cert) => {
            for owner in &cert.pool_params().pool_owners().0 {
                vkeys.insert(owner.clone());
            }
            vkeys.insert(cert.pool_params().operator());
        },
        CertificateEnum::PoolRetirement(cert) => {
            vkeys.insert(cert.pool_keyhash());
        },
        CertificateEnum::GenesisKeyDelegation(cert) => {
            vkeys.insert(
                Ed25519KeyHash::from_bytes(cert.genesis_delegate_hash().to_bytes()).unwrap()
            );
        },
        // no witness as there is no single core node or genesis key that posts the certificate
        CertificateEnum::MoveInstantaneousRewardsCert(_cert) => {},
    };
    Ok(())
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CertificateBuilderResult {
    pub(crate) cert: Certificate,
    pub(crate) aggregate_witness: Option<InputAggregateWitnessData>,
    pub(crate) required_wits: RequiredWitnessSet,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SingleCertificateBuilder {
    cert: Certificate,
}

#[wasm_bindgen]
impl SingleCertificateBuilder {
    pub fn new(cert: &Certificate) -> Self {
        Self {
            cert: cert.clone(),
        }
    }

    /// note: particularly useful for StakeRegistration which doesn't require witnessing
    pub fn skip_witness(&self) -> CertificateBuilderResult {
        let mut required_wits = RequiredWitnessSet::default();
        cert_required_wits(&self.cert, &mut required_wits);

        CertificateBuilderResult {
            cert: self.cert.clone(),
            aggregate_witness: None,
            required_wits,
        }
    }

    pub fn payment_key(&self) -> Result<CertificateBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        cert_required_wits(&self.cert, &mut required_wits);

        if !required_wits.scripts.is_empty() {
            return Err(JsError::from_str(&format!("Certificate required a script, not a payment key: \n{:#?}", self.cert.to_json())));
        }


        Ok(CertificateBuilderResult {
            cert: self.cert.clone(),
            aggregate_witness: None,
            required_wits,
        })
    }

    /** Signer keys don't have to be set. You can leave it empty and then add the required witnesses later */
    pub fn native_script(&self, native_script: &NativeScript, witness_info: &NativeScriptWitnessInfo) -> Result<CertificateBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        cert_required_wits(&self.cert, &mut required_wits);
        let mut required_wits_left = required_wits.clone();

        // the user may have provided more witnesses than required. Strip it down to just the required wits
        // often happens because users aren't aware StakeRegistration doesn't require a witness
        let contains = required_wits_left.scripts.contains(&native_script.hash());

        // check the user provided all the required witnesses
        required_wits_left.scripts.remove(&native_script.hash());

        if required_wits_left.len() > 0 {
            return Err(JsError::from_str(&format!("Missing the following witnesses for the certificate: \n{:#?}", required_wits_left.to_str()))); 
        }

        Ok(CertificateBuilderResult {
            cert: self.cert.clone(),
            aggregate_witness: if contains { Some(InputAggregateWitnessData::NativeScript(native_script.clone(), witness_info.clone())) } else { None },
            required_wits,
        })
    }

    pub fn plutus_script(&self, partial_witness: &PartialPlutusWitness, required_signers: &RequiredSigners) -> Result<CertificateBuilderResult, JsError> {
        let mut required_wits = RequiredWitnessSet::default();
        required_signers.0.iter().for_each(|required_signer| required_wits.add_vkey_key_hash(required_signer));
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
            return Err(JsError::from_str(&format!("Missing the following witnesses for the certificate: \n{:#?}", required_wits_left.to_str())));
        }

        Ok(CertificateBuilderResult {
            cert: self.cert.clone(),
            aggregate_witness: if contains { Some(InputAggregateWitnessData::PlutusScript(partial_witness.clone(), required_signers.clone(), None)) } else { None },
            required_wits,
        })
    }
}
