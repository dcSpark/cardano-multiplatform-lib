// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::{Coin, Ed25519KeyHashList, Epoch, Port, RelayList, UnitInterval};
use crate::address::RewardAccount;
use crate::governance::Anchor;

use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions};
use cml_crypto_wasm::{Ed25519KeyHash, PoolMetadataHash, ScriptHash, VRFKeyHash};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct AuthCommitteeHotCert(cml_chain::certs::AuthCommitteeHotCert);

impl_wasm_cbor_json_api!(AuthCommitteeHotCert);

impl_wasm_conversions!(cml_chain::certs::AuthCommitteeHotCert, AuthCommitteeHotCert);

#[wasm_bindgen]
impl AuthCommitteeHotCert {
    pub fn committee_cold_credential(&self) -> CommitteeColdCredential {
        self.0.committee_cold_credential.clone().into()
    }

    pub fn committee_hot_credential(&self) -> CommitteeHotCredential {
        self.0.committee_hot_credential.clone().into()
    }

    pub fn new(
        committee_cold_credential: &CommitteeColdCredential,
        committee_hot_credential: &CommitteeHotCredential,
    ) -> Self {
        Self(cml_chain::certs::AuthCommitteeHotCert::new(
            committee_cold_credential.clone().into(),
            committee_hot_credential.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Certificate(cml_chain::certs::Certificate);

impl_wasm_cbor_json_api!(Certificate);

impl_wasm_conversions!(cml_chain::certs::Certificate, Certificate);

#[wasm_bindgen]
impl Certificate {
    pub fn new_stake_registration(stake_credential: &StakeCredential) -> Self {
        Self(cml_chain::certs::Certificate::new_stake_registration(
            stake_credential.clone().into(),
        ))
    }

    pub fn new_stake_deregistration(stake_credential: &StakeCredential) -> Self {
        Self(cml_chain::certs::Certificate::new_stake_deregistration(
            stake_credential.clone().into(),
        ))
    }

    pub fn new_stake_delegation(
        stake_credential: &StakeCredential,
        ed25519_key_hash: &Ed25519KeyHash,
    ) -> Self {
        Self(cml_chain::certs::Certificate::new_stake_delegation(
            stake_credential.clone().into(),
            ed25519_key_hash.clone().into(),
        ))
    }

    pub fn new_pool_registration(pool_params: &PoolParams) -> Self {
        Self(cml_chain::certs::Certificate::new_pool_registration(
            pool_params.clone().into(),
        ))
    }

    pub fn new_pool_retirement(ed25519_key_hash: &Ed25519KeyHash, epoch: Epoch) -> Self {
        Self(cml_chain::certs::Certificate::new_pool_retirement(
            ed25519_key_hash.clone().into(),
            epoch,
        ))
    }

    pub fn new_reg_cert(stake_credential: &StakeCredential, coin: Coin) -> Self {
        Self(cml_chain::certs::Certificate::new_reg_cert(
            stake_credential.clone().into(),
            coin,
        ))
    }

    pub fn new_unreg_cert(stake_credential: &StakeCredential, coin: Coin) -> Self {
        Self(cml_chain::certs::Certificate::new_unreg_cert(
            stake_credential.clone().into(),
            coin,
        ))
    }

    pub fn new_vote_deleg_cert(stake_credential: &StakeCredential, d_rep: &DRep) -> Self {
        Self(cml_chain::certs::Certificate::new_vote_deleg_cert(
            stake_credential.clone().into(),
            d_rep.clone().into(),
        ))
    }

    pub fn new_stake_vote_deleg_cert(
        stake_credential: &StakeCredential,
        ed25519_key_hash: &Ed25519KeyHash,
        d_rep: &DRep,
    ) -> Self {
        Self(cml_chain::certs::Certificate::new_stake_vote_deleg_cert(
            stake_credential.clone().into(),
            ed25519_key_hash.clone().into(),
            d_rep.clone().into(),
        ))
    }

    pub fn new_stake_reg_deleg_cert(
        stake_credential: &StakeCredential,
        ed25519_key_hash: &Ed25519KeyHash,
        coin: Coin,
    ) -> Self {
        Self(cml_chain::certs::Certificate::new_stake_reg_deleg_cert(
            stake_credential.clone().into(),
            ed25519_key_hash.clone().into(),
            coin,
        ))
    }

    pub fn new_vote_reg_deleg_cert(
        stake_credential: &StakeCredential,
        d_rep: &DRep,
        coin: Coin,
    ) -> Self {
        Self(cml_chain::certs::Certificate::new_vote_reg_deleg_cert(
            stake_credential.clone().into(),
            d_rep.clone().into(),
            coin,
        ))
    }

    pub fn new_stake_vote_reg_deleg_cert(
        stake_credential: &StakeCredential,
        ed25519_key_hash: &Ed25519KeyHash,
        d_rep: &DRep,
        coin: Coin,
    ) -> Self {
        Self(
            cml_chain::certs::Certificate::new_stake_vote_reg_deleg_cert(
                stake_credential.clone().into(),
                ed25519_key_hash.clone().into(),
                d_rep.clone().into(),
                coin,
            ),
        )
    }

    pub fn new_auth_committee_hot_cert(
        committee_cold_credential: &CommitteeColdCredential,
        committee_hot_credential: &CommitteeHotCredential,
    ) -> Self {
        Self(cml_chain::certs::Certificate::new_auth_committee_hot_cert(
            committee_cold_credential.clone().into(),
            committee_hot_credential.clone().into(),
        ))
    }

    pub fn new_resign_committee_cold_cert(
        committee_cold_credential: &CommitteeColdCredential,
    ) -> Self {
        Self(
            cml_chain::certs::Certificate::new_resign_committee_cold_cert(
                committee_cold_credential.clone().into(),
            ),
        )
    }

    pub fn new_reg_drep_cert(
        drep_credential: &DrepCredential,
        coin: Coin,
        anchor: Option<Anchor>,
    ) -> Self {
        Self(cml_chain::certs::Certificate::new_reg_drep_cert(
            drep_credential.clone().into(),
            coin,
            anchor.map(Into::into),
        ))
    }

    pub fn new_unreg_drep_cert(drep_credential: &DrepCredential, coin: Coin) -> Self {
        Self(cml_chain::certs::Certificate::new_unreg_drep_cert(
            drep_credential.clone().into(),
            coin,
        ))
    }

    pub fn new_update_drep_cert(drep_credential: &DrepCredential, anchor: Option<Anchor>) -> Self {
        Self(cml_chain::certs::Certificate::new_update_drep_cert(
            drep_credential.clone().into(),
            anchor.map(Into::into),
        ))
    }

    pub fn kind(&self) -> CertificateKind {
        match &self.0 {
            cml_chain::certs::Certificate::StakeRegistration(_) => {
                CertificateKind::StakeRegistration
            }
            cml_chain::certs::Certificate::StakeDeregistration(_) => {
                CertificateKind::StakeDeregistration
            }
            cml_chain::certs::Certificate::StakeDelegation(_) => CertificateKind::StakeDelegation,
            cml_chain::certs::Certificate::PoolRegistration(_) => CertificateKind::PoolRegistration,
            cml_chain::certs::Certificate::PoolRetirement(_) => CertificateKind::PoolRetirement,
            cml_chain::certs::Certificate::RegCert(_) => CertificateKind::RegCert,
            cml_chain::certs::Certificate::UnregCert(_) => CertificateKind::UnregCert,
            cml_chain::certs::Certificate::VoteDelegCert(_) => CertificateKind::VoteDelegCert,
            cml_chain::certs::Certificate::StakeVoteDelegCert(_) => {
                CertificateKind::StakeVoteDelegCert
            }
            cml_chain::certs::Certificate::StakeRegDelegCert(_) => {
                CertificateKind::StakeRegDelegCert
            }
            cml_chain::certs::Certificate::VoteRegDelegCert(_) => CertificateKind::VoteRegDelegCert,
            cml_chain::certs::Certificate::StakeVoteRegDelegCert(_) => {
                CertificateKind::StakeVoteRegDelegCert
            }
            cml_chain::certs::Certificate::AuthCommitteeHotCert(_) => {
                CertificateKind::AuthCommitteeHotCert
            }
            cml_chain::certs::Certificate::ResignCommitteeColdCert(_) => {
                CertificateKind::ResignCommitteeColdCert
            }
            cml_chain::certs::Certificate::RegDrepCert(_) => CertificateKind::RegDrepCert,
            cml_chain::certs::Certificate::UnregDrepCert(_) => CertificateKind::UnregDrepCert,
            cml_chain::certs::Certificate::UpdateDrepCert(_) => CertificateKind::UpdateDrepCert,
        }
    }

    pub fn as_stake_registration(&self) -> Option<StakeRegistration> {
        match &self.0 {
            cml_chain::certs::Certificate::StakeRegistration(stake_registration) => {
                Some(stake_registration.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_deregistration(&self) -> Option<StakeDeregistration> {
        match &self.0 {
            cml_chain::certs::Certificate::StakeDeregistration(stake_deregistration) => {
                Some(stake_deregistration.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_delegation(&self) -> Option<StakeDelegation> {
        match &self.0 {
            cml_chain::certs::Certificate::StakeDelegation(stake_delegation) => {
                Some(stake_delegation.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_pool_registration(&self) -> Option<PoolRegistration> {
        match &self.0 {
            cml_chain::certs::Certificate::PoolRegistration(pool_registration) => {
                Some(pool_registration.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_pool_retirement(&self) -> Option<PoolRetirement> {
        match &self.0 {
            cml_chain::certs::Certificate::PoolRetirement(pool_retirement) => {
                Some(pool_retirement.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_reg_cert(&self) -> Option<RegCert> {
        match &self.0 {
            cml_chain::certs::Certificate::RegCert(reg_cert) => Some(reg_cert.clone().into()),
            _ => None,
        }
    }

    pub fn as_unreg_cert(&self) -> Option<UnregCert> {
        match &self.0 {
            cml_chain::certs::Certificate::UnregCert(unreg_cert) => Some(unreg_cert.clone().into()),
            _ => None,
        }
    }

    pub fn as_vote_deleg_cert(&self) -> Option<VoteDelegCert> {
        match &self.0 {
            cml_chain::certs::Certificate::VoteDelegCert(vote_deleg_cert) => {
                Some(vote_deleg_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_vote_deleg_cert(&self) -> Option<StakeVoteDelegCert> {
        match &self.0 {
            cml_chain::certs::Certificate::StakeVoteDelegCert(stake_vote_deleg_cert) => {
                Some(stake_vote_deleg_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_reg_deleg_cert(&self) -> Option<StakeRegDelegCert> {
        match &self.0 {
            cml_chain::certs::Certificate::StakeRegDelegCert(stake_reg_deleg_cert) => {
                Some(stake_reg_deleg_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_vote_reg_deleg_cert(&self) -> Option<VoteRegDelegCert> {
        match &self.0 {
            cml_chain::certs::Certificate::VoteRegDelegCert(vote_reg_deleg_cert) => {
                Some(vote_reg_deleg_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_stake_vote_reg_deleg_cert(&self) -> Option<StakeVoteRegDelegCert> {
        match &self.0 {
            cml_chain::certs::Certificate::StakeVoteRegDelegCert(stake_vote_reg_deleg_cert) => {
                Some(stake_vote_reg_deleg_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_auth_committee_hot_cert(&self) -> Option<AuthCommitteeHotCert> {
        match &self.0 {
            cml_chain::certs::Certificate::AuthCommitteeHotCert(auth_committee_hot_cert) => {
                Some(auth_committee_hot_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_resign_committee_cold_cert(&self) -> Option<ResignCommitteeColdCert> {
        match &self.0 {
            cml_chain::certs::Certificate::ResignCommitteeColdCert(resign_committee_cold_cert) => {
                Some(resign_committee_cold_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_reg_drep_cert(&self) -> Option<RegDrepCert> {
        match &self.0 {
            cml_chain::certs::Certificate::RegDrepCert(reg_drep_cert) => {
                Some(reg_drep_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_unreg_drep_cert(&self) -> Option<UnregDrepCert> {
        match &self.0 {
            cml_chain::certs::Certificate::UnregDrepCert(unreg_drep_cert) => {
                Some(unreg_drep_cert.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_update_drep_cert(&self) -> Option<UpdateDrepCert> {
        match &self.0 {
            cml_chain::certs::Certificate::UpdateDrepCert(update_drep_cert) => {
                Some(update_drep_cert.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum CertificateKind {
    StakeRegistration,
    StakeDeregistration,
    StakeDelegation,
    PoolRegistration,
    PoolRetirement,
    RegCert,
    UnregCert,
    VoteDelegCert,
    StakeVoteDelegCert,
    StakeRegDelegCert,
    VoteRegDelegCert,
    StakeVoteRegDelegCert,
    AuthCommitteeHotCert,
    ResignCommitteeColdCert,
    RegDrepCert,
    UnregDrepCert,
    UpdateDrepCert,
}

pub type CommitteeColdCredential = Credential;

pub type CommitteeHotCredential = Credential;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Credential(cml_chain::certs::Credential);

impl_wasm_cbor_json_api!(Credential);

impl_wasm_conversions!(cml_chain::certs::Credential, Credential);

#[wasm_bindgen]
impl Credential {
    pub fn new_pub_key(hash: &Ed25519KeyHash) -> Self {
        Self(cml_chain::certs::Credential::new_pub_key(
            hash.clone().into(),
        ))
    }

    pub fn new_script(hash: &ScriptHash) -> Self {
        Self(cml_chain::certs::Credential::new_script(
            hash.clone().into(),
        ))
    }

    pub fn kind(&self) -> CredentialKind {
        match &self.0 {
            cml_chain::certs::Credential::PubKey { .. } => CredentialKind::PubKey,
            cml_chain::certs::Credential::Script { .. } => CredentialKind::Script,
        }
    }

    pub fn as_pub_key(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            cml_chain::certs::Credential::PubKey { hash, .. } => Some((*hash).into()),
            _ => None,
        }
    }

    pub fn as_script(&self) -> Option<ScriptHash> {
        match &self.0 {
            cml_chain::certs::Credential::Script { hash, .. } => Some((*hash).into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum CredentialKind {
    PubKey,
    Script,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DRep(cml_chain::certs::DRep);

impl_wasm_cbor_json_api!(DRep);

impl_wasm_conversions!(cml_chain::certs::DRep, DRep);

#[wasm_bindgen]
impl DRep {
    pub fn new_key(ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_chain::certs::DRep::new_key(
            ed25519_key_hash.clone().into(),
        ))
    }

    pub fn new_script(script_hash: &ScriptHash) -> Self {
        Self(cml_chain::certs::DRep::new_script(
            script_hash.clone().into(),
        ))
    }

    pub fn new_always_abstain() -> Self {
        Self(cml_chain::certs::DRep::new_always_abstain())
    }

    pub fn new_always_no_confidence() -> Self {
        Self(cml_chain::certs::DRep::new_always_no_confidence())
    }

    pub fn kind(&self) -> DRepKind {
        match &self.0 {
            cml_chain::certs::DRep::Key { .. } => DRepKind::Key,
            cml_chain::certs::DRep::Script { .. } => DRepKind::Script,
            cml_chain::certs::DRep::AlwaysAbstain { .. } => DRepKind::AlwaysAbstain,
            cml_chain::certs::DRep::AlwaysNoConfidence { .. } => DRepKind::AlwaysNoConfidence,
        }
    }

    pub fn as_key(&self) -> Option<Ed25519KeyHash> {
        match &self.0 {
            cml_chain::certs::DRep::Key {
                ed25519_key_hash, ..
            } => Some((*ed25519_key_hash).into()),
            _ => None,
        }
    }

    pub fn as_script(&self) -> Option<ScriptHash> {
        match &self.0 {
            cml_chain::certs::DRep::Script { script_hash, .. } => Some((*script_hash).into()),
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum DRepKind {
    Key,
    Script,
    AlwaysAbstain,
    AlwaysNoConfidence,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct DnsName(cml_chain::certs::DnsName);

impl_wasm_cbor_json_api!(DnsName);

impl_wasm_conversions!(cml_chain::certs::DnsName, DnsName);

#[wasm_bindgen]
impl DnsName {
    pub fn get(&self) -> String {
        self.0.get().clone()
    }
}

pub type DrepCredential = Credential;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Ipv4(cml_chain::certs::Ipv4);

impl_wasm_cbor_json_api!(Ipv4);

impl_wasm_conversions!(cml_chain::certs::Ipv4, Ipv4);

#[wasm_bindgen]
impl Ipv4 {
    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Ipv6(cml_chain::certs::Ipv6);

impl_wasm_cbor_json_api!(Ipv6);

impl_wasm_conversions!(cml_chain::certs::Ipv6, Ipv6);

#[wasm_bindgen]
impl Ipv6 {
    pub fn get(&self) -> Vec<u8> {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct MultiHostName(cml_chain::certs::MultiHostName);

impl_wasm_cbor_json_api!(MultiHostName);

impl_wasm_conversions!(cml_chain::certs::MultiHostName, MultiHostName);

#[wasm_bindgen]
impl MultiHostName {
    pub fn dns_name(&self) -> DnsName {
        self.0.dns_name.clone().into()
    }

    pub fn new(dns_name: &DnsName) -> Self {
        Self(cml_chain::certs::MultiHostName::new(
            dns_name.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolMetadata(cml_chain::certs::PoolMetadata);

impl_wasm_cbor_json_api!(PoolMetadata);

impl_wasm_conversions!(cml_chain::certs::PoolMetadata, PoolMetadata);

#[wasm_bindgen]
impl PoolMetadata {
    pub fn url(&self) -> Url {
        self.0.url.clone().into()
    }

    pub fn pool_metadata_hash(&self) -> PoolMetadataHash {
        self.0.pool_metadata_hash.into()
    }

    pub fn new(url: &Url, pool_metadata_hash: &PoolMetadataHash) -> Self {
        Self(cml_chain::certs::PoolMetadata::new(
            url.clone().into(),
            pool_metadata_hash.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolParams(cml_chain::certs::PoolParams);

impl_wasm_cbor_json_api!(PoolParams);

impl_wasm_conversions!(cml_chain::certs::PoolParams, PoolParams);

#[wasm_bindgen]
impl PoolParams {
    pub fn operator(&self) -> Ed25519KeyHash {
        self.0.operator.into()
    }

    pub fn vrf_keyhash(&self) -> VRFKeyHash {
        self.0.vrf_keyhash.into()
    }

    pub fn pledge(&self) -> Coin {
        self.0.pledge
    }

    pub fn cost(&self) -> Coin {
        self.0.cost
    }

    pub fn margin(&self) -> UnitInterval {
        self.0.margin.clone().into()
    }

    pub fn reward_account(&self) -> RewardAccount {
        self.0.reward_account.clone().into()
    }

    pub fn pool_owners(&self) -> Ed25519KeyHashList {
        self.0.pool_owners.clone().into()
    }

    pub fn relays(&self) -> RelayList {
        self.0.relays.clone().into()
    }

    pub fn pool_metadata(&self) -> Option<PoolMetadata> {
        self.0.pool_metadata.clone().map(std::convert::Into::into)
    }

    pub fn new(
        operator: &Ed25519KeyHash,
        vrf_keyhash: &VRFKeyHash,
        pledge: Coin,
        cost: Coin,
        margin: &UnitInterval,
        reward_account: &RewardAccount,
        pool_owners: &Ed25519KeyHashList,
        relays: &RelayList,
        pool_metadata: Option<PoolMetadata>,
    ) -> Self {
        Self(cml_chain::certs::PoolParams::new(
            operator.clone().into(),
            vrf_keyhash.clone().into(),
            pledge,
            cost,
            margin.clone().into(),
            reward_account.clone().into(),
            pool_owners.clone().into(),
            relays.clone().into(),
            pool_metadata.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolRegistration(cml_chain::certs::PoolRegistration);

impl_wasm_cbor_json_api!(PoolRegistration);

impl_wasm_conversions!(cml_chain::certs::PoolRegistration, PoolRegistration);

#[wasm_bindgen]
impl PoolRegistration {
    pub fn pool_params(&self) -> PoolParams {
        self.0.pool_params.clone().into()
    }

    pub fn new(pool_params: &PoolParams) -> Self {
        Self(cml_chain::certs::PoolRegistration::new(
            pool_params.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct PoolRetirement(cml_chain::certs::PoolRetirement);

impl_wasm_cbor_json_api!(PoolRetirement);

impl_wasm_conversions!(cml_chain::certs::PoolRetirement, PoolRetirement);

#[wasm_bindgen]
impl PoolRetirement {
    pub fn ed25519_key_hash(&self) -> Ed25519KeyHash {
        self.0.ed25519_key_hash.into()
    }

    pub fn epoch(&self) -> Epoch {
        self.0.epoch
    }

    pub fn new(ed25519_key_hash: &Ed25519KeyHash, epoch: Epoch) -> Self {
        Self(cml_chain::certs::PoolRetirement::new(
            ed25519_key_hash.clone().into(),
            epoch,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RegCert(cml_chain::certs::RegCert);

impl_wasm_cbor_json_api!(RegCert);

impl_wasm_conversions!(cml_chain::certs::RegCert, RegCert);

#[wasm_bindgen]
impl RegCert {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn coin(&self) -> Coin {
        self.0.coin
    }

    pub fn new(stake_credential: &StakeCredential, coin: Coin) -> Self {
        Self(cml_chain::certs::RegCert::new(
            stake_credential.clone().into(),
            coin,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RegDrepCert(cml_chain::certs::RegDrepCert);

impl_wasm_cbor_json_api!(RegDrepCert);

impl_wasm_conversions!(cml_chain::certs::RegDrepCert, RegDrepCert);

#[wasm_bindgen]
impl RegDrepCert {
    pub fn drep_credential(&self) -> DrepCredential {
        self.0.drep_credential.clone().into()
    }

    pub fn coin(&self) -> Coin {
        self.0.coin
    }

    pub fn anchor(&self) -> Option<Anchor> {
        self.0.anchor.clone().map(std::convert::Into::into)
    }

    pub fn new(drep_credential: &DrepCredential, coin: Coin, anchor: Option<Anchor>) -> Self {
        Self(cml_chain::certs::RegDrepCert::new(
            drep_credential.clone().into(),
            coin,
            anchor.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Relay(cml_chain::certs::Relay);

impl_wasm_cbor_json_api!(Relay);

impl_wasm_conversions!(cml_chain::certs::Relay, Relay);

#[wasm_bindgen]
impl Relay {
    pub fn new_single_host_addr(
        port: Option<Port>,
        ipv4: Option<Ipv4>,
        ipv6: Option<Ipv6>,
    ) -> Self {
        Self(cml_chain::certs::Relay::new_single_host_addr(
            port,
            ipv4.map(Into::into),
            ipv6.map(Into::into),
        ))
    }

    pub fn new_single_host_name(port: Option<Port>, dns_name: &DnsName) -> Self {
        Self(cml_chain::certs::Relay::new_single_host_name(
            port,
            dns_name.clone().into(),
        ))
    }

    pub fn new_multi_host_name(dns_name: &DnsName) -> Self {
        Self(cml_chain::certs::Relay::new_multi_host_name(
            dns_name.clone().into(),
        ))
    }

    pub fn kind(&self) -> RelayKind {
        match &self.0 {
            cml_chain::certs::Relay::SingleHostAddr(_) => RelayKind::SingleHostAddr,
            cml_chain::certs::Relay::SingleHostName(_) => RelayKind::SingleHostName,
            cml_chain::certs::Relay::MultiHostName(_) => RelayKind::MultiHostName,
        }
    }

    pub fn as_single_host_addr(&self) -> Option<SingleHostAddr> {
        match &self.0 {
            cml_chain::certs::Relay::SingleHostAddr(single_host_addr) => {
                Some(single_host_addr.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_single_host_name(&self) -> Option<SingleHostName> {
        match &self.0 {
            cml_chain::certs::Relay::SingleHostName(single_host_name) => {
                Some(single_host_name.clone().into())
            }
            _ => None,
        }
    }

    pub fn as_multi_host_name(&self) -> Option<MultiHostName> {
        match &self.0 {
            cml_chain::certs::Relay::MultiHostName(multi_host_name) => {
                Some(multi_host_name.clone().into())
            }
            _ => None,
        }
    }
}

#[wasm_bindgen]
pub enum RelayKind {
    SingleHostAddr,
    SingleHostName,
    MultiHostName,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct ResignCommitteeColdCert(cml_chain::certs::ResignCommitteeColdCert);

impl_wasm_cbor_json_api!(ResignCommitteeColdCert);

impl_wasm_conversions!(
    cml_chain::certs::ResignCommitteeColdCert,
    ResignCommitteeColdCert
);

#[wasm_bindgen]
impl ResignCommitteeColdCert {
    pub fn committee_cold_credential(&self) -> CommitteeColdCredential {
        self.0.committee_cold_credential.clone().into()
    }

    pub fn new(committee_cold_credential: &CommitteeColdCredential) -> Self {
        Self(cml_chain::certs::ResignCommitteeColdCert::new(
            committee_cold_credential.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SingleHostAddr(cml_chain::certs::SingleHostAddr);

impl_wasm_cbor_json_api!(SingleHostAddr);

impl_wasm_conversions!(cml_chain::certs::SingleHostAddr, SingleHostAddr);

#[wasm_bindgen]
impl SingleHostAddr {
    pub fn port(&self) -> Option<Port> {
        self.0.port
    }

    pub fn ipv4(&self) -> Option<Ipv4> {
        self.0.ipv4.clone().map(std::convert::Into::into)
    }

    pub fn ipv6(&self) -> Option<Ipv6> {
        self.0.ipv6.clone().map(std::convert::Into::into)
    }

    pub fn new(port: Option<Port>, ipv4: Option<Ipv4>, ipv6: Option<Ipv6>) -> Self {
        Self(cml_chain::certs::SingleHostAddr::new(
            port,
            ipv4.map(Into::into),
            ipv6.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct SingleHostName(cml_chain::certs::SingleHostName);

impl_wasm_cbor_json_api!(SingleHostName);

impl_wasm_conversions!(cml_chain::certs::SingleHostName, SingleHostName);

#[wasm_bindgen]
impl SingleHostName {
    pub fn port(&self) -> Option<Port> {
        self.0.port
    }

    pub fn dns_name(&self) -> DnsName {
        self.0.dns_name.clone().into()
    }

    pub fn new(port: Option<Port>, dns_name: &DnsName) -> Self {
        Self(cml_chain::certs::SingleHostName::new(
            port,
            dns_name.clone().into(),
        ))
    }
}

pub type StakeCredential = Credential;

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeDelegation(cml_chain::certs::StakeDelegation);

impl_wasm_cbor_json_api!(StakeDelegation);

impl_wasm_conversions!(cml_chain::certs::StakeDelegation, StakeDelegation);

#[wasm_bindgen]
impl StakeDelegation {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn ed25519_key_hash(&self) -> Ed25519KeyHash {
        self.0.ed25519_key_hash.into()
    }

    pub fn new(stake_credential: &StakeCredential, ed25519_key_hash: &Ed25519KeyHash) -> Self {
        Self(cml_chain::certs::StakeDelegation::new(
            stake_credential.clone().into(),
            ed25519_key_hash.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeDeregistration(cml_chain::certs::StakeDeregistration);

impl_wasm_cbor_json_api!(StakeDeregistration);

impl_wasm_conversions!(cml_chain::certs::StakeDeregistration, StakeDeregistration);

#[wasm_bindgen]
impl StakeDeregistration {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn new(stake_credential: &StakeCredential) -> Self {
        Self(cml_chain::certs::StakeDeregistration::new(
            stake_credential.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeRegDelegCert(cml_chain::certs::StakeRegDelegCert);

impl_wasm_cbor_json_api!(StakeRegDelegCert);

impl_wasm_conversions!(cml_chain::certs::StakeRegDelegCert, StakeRegDelegCert);

#[wasm_bindgen]
impl StakeRegDelegCert {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn ed25519_key_hash(&self) -> Ed25519KeyHash {
        self.0.ed25519_key_hash.into()
    }

    pub fn coin(&self) -> Coin {
        self.0.coin
    }

    pub fn new(
        stake_credential: &StakeCredential,
        ed25519_key_hash: &Ed25519KeyHash,
        coin: Coin,
    ) -> Self {
        Self(cml_chain::certs::StakeRegDelegCert::new(
            stake_credential.clone().into(),
            ed25519_key_hash.clone().into(),
            coin,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeRegistration(cml_chain::certs::StakeRegistration);

impl_wasm_cbor_json_api!(StakeRegistration);

impl_wasm_conversions!(cml_chain::certs::StakeRegistration, StakeRegistration);

#[wasm_bindgen]
impl StakeRegistration {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn new(stake_credential: &StakeCredential) -> Self {
        Self(cml_chain::certs::StakeRegistration::new(
            stake_credential.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeVoteDelegCert(cml_chain::certs::StakeVoteDelegCert);

impl_wasm_cbor_json_api!(StakeVoteDelegCert);

impl_wasm_conversions!(cml_chain::certs::StakeVoteDelegCert, StakeVoteDelegCert);

#[wasm_bindgen]
impl StakeVoteDelegCert {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn ed25519_key_hash(&self) -> Ed25519KeyHash {
        self.0.ed25519_key_hash.into()
    }

    pub fn d_rep(&self) -> DRep {
        self.0.d_rep.clone().into()
    }

    pub fn new(
        stake_credential: &StakeCredential,
        ed25519_key_hash: &Ed25519KeyHash,
        d_rep: &DRep,
    ) -> Self {
        Self(cml_chain::certs::StakeVoteDelegCert::new(
            stake_credential.clone().into(),
            ed25519_key_hash.clone().into(),
            d_rep.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct StakeVoteRegDelegCert(cml_chain::certs::StakeVoteRegDelegCert);

impl_wasm_cbor_json_api!(StakeVoteRegDelegCert);

impl_wasm_conversions!(
    cml_chain::certs::StakeVoteRegDelegCert,
    StakeVoteRegDelegCert
);

#[wasm_bindgen]
impl StakeVoteRegDelegCert {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn ed25519_key_hash(&self) -> Ed25519KeyHash {
        self.0.ed25519_key_hash.into()
    }

    pub fn d_rep(&self) -> DRep {
        self.0.d_rep.clone().into()
    }

    pub fn coin(&self) -> Coin {
        self.0.coin
    }

    pub fn new(
        stake_credential: &StakeCredential,
        ed25519_key_hash: &Ed25519KeyHash,
        d_rep: &DRep,
        coin: Coin,
    ) -> Self {
        Self(cml_chain::certs::StakeVoteRegDelegCert::new(
            stake_credential.clone().into(),
            ed25519_key_hash.clone().into(),
            d_rep.clone().into(),
            coin,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct UnregCert(cml_chain::certs::UnregCert);

impl_wasm_cbor_json_api!(UnregCert);

impl_wasm_conversions!(cml_chain::certs::UnregCert, UnregCert);

#[wasm_bindgen]
impl UnregCert {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn coin(&self) -> Coin {
        self.0.coin
    }

    pub fn new(stake_credential: &StakeCredential, coin: Coin) -> Self {
        Self(cml_chain::certs::UnregCert::new(
            stake_credential.clone().into(),
            coin,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct UnregDrepCert(cml_chain::certs::UnregDrepCert);

impl_wasm_cbor_json_api!(UnregDrepCert);

impl_wasm_conversions!(cml_chain::certs::UnregDrepCert, UnregDrepCert);

#[wasm_bindgen]
impl UnregDrepCert {
    pub fn drep_credential(&self) -> DrepCredential {
        self.0.drep_credential.clone().into()
    }

    pub fn coin(&self) -> Coin {
        self.0.coin
    }

    pub fn new(drep_credential: &DrepCredential, coin: Coin) -> Self {
        Self(cml_chain::certs::UnregDrepCert::new(
            drep_credential.clone().into(),
            coin,
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct UpdateDrepCert(cml_chain::certs::UpdateDrepCert);

impl_wasm_cbor_json_api!(UpdateDrepCert);

impl_wasm_conversions!(cml_chain::certs::UpdateDrepCert, UpdateDrepCert);

#[wasm_bindgen]
impl UpdateDrepCert {
    pub fn drep_credential(&self) -> DrepCredential {
        self.0.drep_credential.clone().into()
    }

    pub fn anchor(&self) -> Option<Anchor> {
        self.0.anchor.clone().map(std::convert::Into::into)
    }

    pub fn new(drep_credential: &DrepCredential, anchor: Option<Anchor>) -> Self {
        Self(cml_chain::certs::UpdateDrepCert::new(
            drep_credential.clone().into(),
            anchor.map(Into::into),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Url(cml_chain::certs::Url);

impl_wasm_cbor_json_api!(Url);

impl_wasm_conversions!(cml_chain::certs::Url, Url);

#[wasm_bindgen]
impl Url {
    pub fn get(&self) -> String {
        self.0.get().clone()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VoteDelegCert(cml_chain::certs::VoteDelegCert);

impl_wasm_cbor_json_api!(VoteDelegCert);

impl_wasm_conversions!(cml_chain::certs::VoteDelegCert, VoteDelegCert);

#[wasm_bindgen]
impl VoteDelegCert {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn d_rep(&self) -> DRep {
        self.0.d_rep.clone().into()
    }

    pub fn new(stake_credential: &StakeCredential, d_rep: &DRep) -> Self {
        Self(cml_chain::certs::VoteDelegCert::new(
            stake_credential.clone().into(),
            d_rep.clone().into(),
        ))
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct VoteRegDelegCert(cml_chain::certs::VoteRegDelegCert);

impl_wasm_cbor_json_api!(VoteRegDelegCert);

impl_wasm_conversions!(cml_chain::certs::VoteRegDelegCert, VoteRegDelegCert);

#[wasm_bindgen]
impl VoteRegDelegCert {
    pub fn stake_credential(&self) -> StakeCredential {
        self.0.stake_credential.clone().into()
    }

    pub fn d_rep(&self) -> DRep {
        self.0.d_rep.clone().into()
    }

    pub fn coin(&self) -> Coin {
        self.0.coin
    }

    pub fn new(stake_credential: &StakeCredential, d_rep: &DRep, coin: Coin) -> Self {
        Self(cml_chain::certs::VoteRegDelegCert::new(
            stake_credential.clone().into(),
            d_rep.clone().into(),
            coin,
        ))
    }
}
