use crate::*;
use crate::witness_builder::{InputAggregateWitnessData, PartialPlutusWitness};
use std::collections::{HashSet};

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

fn check_cert_script_hash(cert_enum: &Certificate, expected_hash: &ScriptHash) -> Result<bool, JsError> {
    match &cert_enum.0 {
        // stake key registrations do not require a witness
        CertificateEnum::StakeRegistration(_cert) => Ok(false),
        CertificateEnum::StakeDeregistration(cert) => match cert.stake_credential().to_scripthash() {
            Some(script_hash) => {
                match *expected_hash == script_hash {
                    true => Ok(true),
                    false => return Err(JsError::from_str(&format!("Deregistration certificate contains wrong script hash. Expected {}, got {}.\n{:#?}", expected_hash, script_hash, cert.to_json()))),
                }
            },
            None => return Err(JsError::from_str(&format!("Deregistration certificate contains public key hash. Expected script.\n{:#?}", cert.to_json()))),
        },
        CertificateEnum::StakeDelegation(cert) => match cert.stake_credential().to_scripthash() {
            Some(script_hash) => {
                match *expected_hash == script_hash {
                    true => Ok(true),
                    false => return Err(JsError::from_str(&format!("Delegation certificate contains wrong script hash. Expected {}, got {}.\n{:#?}", expected_hash, script_hash, cert.to_json()))),
                }
            },
            None => return Err(JsError::from_str(&format!("Delegation certificate contains public key hash. Expected script.\n{:#?}", cert.to_json()))),
        },
        // can't register pools with scripts
        CertificateEnum::PoolRegistration(_cert) => Ok(false),
        CertificateEnum::PoolRetirement(_cert) => Ok(false),
        // genesis keys are not scripts
        CertificateEnum::GenesisKeyDelegation(_cert) => Ok(false),
        // no witness as there is no single core node or genesis key that posts the certificate
        CertificateEnum::MoveInstantaneousRewardsCert(_cert) => Ok(false),
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CertificateBuilderResult {
    cert: Certificate,
    aggregate_witness: InputAggregateWitnessData,
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

    pub fn no_script(&self) -> Result<CertificateBuilderResult, JsError> {
        let mut vkey_set = HashSet::<Ed25519KeyHash>::new();
        add_cert_vkeys(&self.cert, &mut vkey_set)?;
        Ok(CertificateBuilderResult {
            cert: self.cert.clone(),
            aggregate_witness: InputAggregateWitnessData::Vkeys(vkey_set.clone()),
        })
    }

    pub fn native_script(&self, native_script: &NativeScript) -> Result<CertificateBuilderResult, JsError> {
        let expected_hash = native_script.hash(ScriptHashNamespace::NativeScript);
        check_cert_script_hash(&self.cert, &expected_hash)?;
        Ok(CertificateBuilderResult {
            cert: self.cert.clone(),
            aggregate_witness: InputAggregateWitnessData::NativeScript(native_script.clone()),
        })
    }

    pub fn plutus_script(&self, partial_witness: &PartialPlutusWitness) -> Result<CertificateBuilderResult, JsError> {
        // TODO: support PlutusV2
        let expected_hash = partial_witness.script().hash(ScriptHashNamespace::PlutusV1);
        check_cert_script_hash(&self.cert, &expected_hash)?;
        Ok(CertificateBuilderResult {
            cert: self.cert.clone(),
            aggregate_witness: InputAggregateWitnessData::PlutusScriptNoDatum(partial_witness.clone()),
        })
    }
}
