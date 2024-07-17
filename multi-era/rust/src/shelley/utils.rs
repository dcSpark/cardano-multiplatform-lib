use cml_chain::{
    certs::{DNSName, PoolParams, PoolRegistration, Relay},
    transaction::{NativeScript, TransactionWitnessSet},
};

use super::{
    MultisigScript, ShelleyPoolRegistration, ShelleyRelay, ShelleyTransactionBody,
    ShelleyTransactionWitnessSet,
};

use cml_core::serialization::Serialize;
use cml_crypto::{blake2b256, TransactionHash};

impl ShelleyTransactionBody {
    pub fn hash(&self) -> TransactionHash {
        blake2b256(&self.to_cbor_bytes()).into()
    }
}

impl From<ShelleyTransactionWitnessSet> for TransactionWitnessSet {
    fn from(wits: ShelleyTransactionWitnessSet) -> Self {
        let mut new_wits = TransactionWitnessSet::new();
        new_wits.vkeywitnesses = wits.vkeywitnesses.map(Into::into);
        new_wits.native_scripts = wits.native_scripts.map(|native_scripts| {
            native_scripts
                .into_iter()
                .map(NativeScript::from)
                .collect::<Vec<_>>()
                .into()
        });
        new_wits.bootstrap_witnesses = wits.bootstrap_witnesses.map(Into::into);
        new_wits
    }
}

impl From<MultisigScript> for NativeScript {
    fn from(script: MultisigScript) -> Self {
        match script {
            MultisigScript::MultisigPubkey(key) => {
                NativeScript::new_script_pubkey(key.ed25519_key_hash)
            }
            MultisigScript::MultisigAll(all) => NativeScript::new_script_all(
                all.multisig_scripts
                    .into_iter()
                    .map(NativeScript::from)
                    .collect(),
            ),
            MultisigScript::MultisigAny(any) => NativeScript::new_script_any(
                any.multisig_scripts
                    .into_iter()
                    .map(NativeScript::from)
                    .collect(),
            ),
            MultisigScript::MultisigNOfK(nok) => NativeScript::new_script_n_of_k(
                nok.n,
                nok.multisig_scripts
                    .into_iter()
                    .map(NativeScript::from)
                    .collect(),
            ),
        }
    }
}

impl From<ShelleyPoolRegistration> for PoolRegistration {
    fn from(pool_reg: ShelleyPoolRegistration) -> Self {
        Self::new(PoolParams::new(
            pool_reg.pool_params.operator,
            pool_reg.pool_params.vrf_keyhash,
            pool_reg.pool_params.pledge,
            pool_reg.pool_params.cost,
            pool_reg.pool_params.margin,
            pool_reg.pool_params.reward_account,
            pool_reg.pool_params.pool_owners.into(),
            pool_reg
                .pool_params
                .relays
                .into_iter()
                .map(Into::into)
                .collect(),
            pool_reg.pool_params.pool_metadata,
        ))
    }
}

impl From<ShelleyRelay> for Relay {
    fn from(relay: ShelleyRelay) -> Self {
        match relay {
            ShelleyRelay::SingleHostAddr(host) => {
                Self::new_single_host_addr(host.port, host.ipv4, host.ipv6)
            }
            ShelleyRelay::ShelleySingleHostName(host) => Self::new_single_host_name(
                host.port,
                DNSName::new(host.shelley_dns_name.inner).unwrap(),
            ),
            ShelleyRelay::ShelleyMultiHostName(host) => {
                Self::new_multi_host_name(DNSName::new(host.shelley_dns_name.inner).unwrap())
            }
        }
    }
}
