use linked_hash_map::LinkedHashMap;
use std::{
    collections::{BTreeSet, HashMap},
    fmt::Debug,
};

use crate::{
    byron::ByronAddress,
    certs::Credential,
    crypto::{hash::hash_plutus_data, BootstrapWitness, Vkey, Vkeywitness},
    plutus::{
        LegacyRedeemer, PlutusData, PlutusScript, PlutusV1Script, PlutusV2Script, PlutusV3Script,
        Redeemers,
    },
    transaction::TransactionWitnessSet,
    NativeScript, RequiredSigners, Script,
};
use cml_crypto::{
    DatumHash, Ed25519KeyHash, Ed25519Signature, PublicKey, RawBytesEncoding, ScriptHash,
};

use super::{
    redeemer_builder::{MissingExunitError, RedeemerBuilderError, RedeemerWitnessKey},
    tx_builder::TransactionUnspentOutput,
};

#[derive(Debug, thiserror::Error)]
pub enum WitnessBuilderError {
    #[error("Missing the following witnesses: {0:?}")]
    MissingWitnesses(RequiredWitnessSet),
    #[error("Missing ExUnit: {0}")]
    MissingExUnit(#[from] MissingExunitError),
    #[error("LegacyRedeemer build failed: {0}")]
    RedeemBuildFailed(#[from] RedeemerBuilderError),
}

#[derive(Debug, Clone)] //, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlutusScriptWitness {
    Ref(ScriptHash),
    Script(PlutusScript),
}

impl PlutusScriptWitness {
    // pub fn script(&self) -> Option<PlutusScript> {
    //     match self {
    //         Self::Ref(_) => None,
    //         Self::Script(script) => Some(script.clone()),
    //     }
    // }

    pub fn hash(&self) -> ScriptHash {
        match self {
            Self::Ref(hash) => *hash,
            Self::Script(script) => script.hash(),
        }
    }
}

impl From<PlutusScript> for PlutusScriptWitness {
    fn from(script: PlutusScript) -> Self {
        PlutusScriptWitness::Script(script)
    }
}

impl From<ScriptHash> for PlutusScriptWitness {
    fn from(hash: ScriptHash) -> Self {
        PlutusScriptWitness::Ref(hash)
    }
}

/// A partial Plutus witness
/// It contains all the information needed to witness the Plutus script execution
/// except for the redeemer tag and index
/// Note: no datum is attached because only input script types have datums
#[derive(Clone, Debug)]
pub struct PartialPlutusWitness {
    pub script: PlutusScriptWitness,
    pub redeemer: PlutusData,
}

impl PartialPlutusWitness {
    pub fn new(script: PlutusScriptWitness, redeemer: PlutusData) -> Self {
        Self { script, redeemer }
    }
}

#[derive(Clone, Debug)]
pub enum InputAggregateWitnessData {
    NativeScript(NativeScript, NativeScriptWitnessInfo),
    PlutusScript(PartialPlutusWitness, RequiredSigners, Option<PlutusData>),
}

impl InputAggregateWitnessData {
    pub fn redeemer_plutus_data(&self) -> Option<&PlutusData> {
        match self {
            InputAggregateWitnessData::PlutusScript(witness, _, _) => Some(&witness.redeemer),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RequiredWitnessSet {
    // note: the real key type for these is Vkey
    // but cryptographically these should be equivalent and Ed25519KeyHash is more flexible
    pub vkeys: BTreeSet<Ed25519KeyHash>,
    // note: the real key type for these is Vkey
    // but cryptographically these should be equivalent as ByronAddress contains an AddressId
    // which is the hash of data that includes the public key
    pub bootstraps: BTreeSet<ByronAddress>,
    // note: no way to differentiate Plutus script from native script
    pub scripts: BTreeSet<ScriptHash>,
    pub plutus_data: BTreeSet<DatumHash>,
    pub redeemers: BTreeSet<RedeemerWitnessKey>,
    pub script_refs: BTreeSet<ScriptHash>,
}

impl RequiredWitnessSet {
    // pub fn add_vkey(&mut self, vkey: &Vkeywitness) {
    //     self.add_vkey_key(&vkey.vkey());
    // }
    // pub fn add_vkey_key(&mut self, vkey: &Vkey) {
    //     self.add_vkey_key_hash(&vkey.public_key().hash());
    // }
    pub fn add_vkey_key_hash(&mut self, hash: Ed25519KeyHash) {
        self.vkeys.insert(hash);
    }

    pub fn add_bootstrap(&mut self, address: ByronAddress) {
        self.bootstraps.insert(address);
    }

    pub fn add_script_ref(&mut self, script_hash: ScriptHash) {
        self.scripts.remove(&script_hash);
        self.script_refs.insert(script_hash);
    }

    // pub fn add_native_script(&mut self, native_script: &NativeScript) {
    //     self.add_script_hash(&native_script.hash());
    // }

    pub fn add_script_hash(&mut self, script_hash: ScriptHash) {
        match self.script_refs.get(&script_hash) {
            None => {
                self.scripts.insert(script_hash);
            }
            Some(_) => {}
        }
    }

    pub(crate) fn add_from_credential(&mut self, credential: Credential) {
        match credential {
            Credential::PubKey { hash, .. } => self.add_vkey_key_hash(hash),
            Credential::Script { hash, .. } => self.add_script_hash(hash),
        }
    }

    // pub fn add_plutus_script(&mut self, plutus_v1_script: &PlutusScript) {
    //     self.add_script_hash(&plutus_v1_script.hash());
    // }

    // pub fn add_plutus_datum(&mut self, plutus_datum: &PlutusData) {
    //     self.add_plutus_datum_hash(&hash_plutus_data(plutus_datum));
    // }
    pub fn add_plutus_datum_hash(&mut self, plutus_datum: DatumHash) {
        self.plutus_data.insert(plutus_datum);
    }

    // pub fn add_redeemer(&mut self, redeemer: &LegacyRedeemer) {
    //     self.add_redeemer_tag(&RedeemerWitnessKey::new(&redeemer.tag(), &redeemer.index()));
    // }
    pub fn add_redeemer_tag(&mut self, redeemer: RedeemerWitnessKey) {
        self.redeemers.insert(redeemer);
    }

    pub fn add_all(&mut self, requirements: RequiredWitnessSet) {
        self.vkeys.extend(requirements.vkeys);
        self.bootstraps.extend(requirements.bootstraps);
        self.scripts.extend(requirements.scripts);
        self.plutus_data.extend(requirements.plutus_data);
        self.redeemers.extend(requirements.redeemers);
    }

    pub fn remove_ref_scripts(&mut self, ref_inputs: &[TransactionUnspentOutput]) {
        ref_inputs.iter().for_each(|utxo| {
            utxo.output.script_ref().inspect(|script_ref| {
                self.scripts.remove(&script_ref.hash());
            });
        })
    }

    pub(crate) fn len(&self) -> usize {
        self.vkeys.len()
            + self.bootstraps.len()
            + self.scripts.len()
            + self.plutus_data.len()
            + self.redeemers.len()
    }

    // This method add fake vkeys to calculate the fee.
    // In order to prevent that fake keys get deduplicated when it is called more than once,
    // its index starts from the current amount of vkeys.
    // WARN: this function might fail at runtime when there are more than 255 witnesses,
    // however this is unrealistic because the limit of transaction size. (101 bytes each witness)
    pub(crate) fn add_fake_vkey_witnesses_by_num(&mut self, num: usize) {
        for _ in 0..num {
            self.add_vkey_key_hash(fake_key_hash(self.vkeys.len() as u8));
        }
    }

    pub(crate) fn add_input_aggregate_fake_witness_data(
        &mut self,
        data: &InputAggregateWitnessData,
    ) {
        match data {
            InputAggregateWitnessData::NativeScript(script, info) => {
                match info {
                    NativeScriptWitnessInfo::Count(num) => {
                        self.add_fake_vkey_witnesses_by_num(*num)
                    }
                    NativeScriptWitnessInfo::Vkeys(ref vkeys) => {
                        vkeys
                            .iter()
                            .cloned()
                            .for_each(|vkey| self.add_vkey_key_hash(vkey));
                    }
                    NativeScriptWitnessInfo::AssumeWorst => {
                        // we get the size instead of the hashes themselves
                        // since there is no way to know if any of these will actually be required to sign the tx
                        let num = script.get_required_signers().len();
                        self.add_fake_vkey_witnesses_by_num(num);
                    }
                }
            }
            InputAggregateWitnessData::PlutusScript(_witness, required_signers, _option) => {
                required_signers
                    .iter()
                    .cloned()
                    .for_each(|vkey| self.add_vkey_key_hash(vkey));
            }
        }
    }

    pub fn new() -> Self {
        // have to expose new so it's visible in WASM
        Self::default()
    }
}

/// Builder de-duplicates witnesses as they are added
#[derive(Clone, Default, Debug)]
pub struct TransactionWitnessSetBuilder {
    // See Alonzo spec section 3.1 which defines the keys for these types
    pub vkeys: HashMap<Vkey, Vkeywitness>,
    pub bootstraps: HashMap<Vkey, BootstrapWitness>,
    pub scripts: HashMap<ScriptHash, Script>,
    pub plutus_data: LinkedHashMap<DatumHash, PlutusData>,
    pub redeemers: LinkedHashMap<RedeemerWitnessKey, LegacyRedeemer>,

    /// witnesses that need to be added for the build function to succeed
    /// this allows checking that witnesses are present at build time (instead of when submitting to a node)
    /// This is useful for APIs that can keep track of which witnesses will be required (like transaction builders)
    pub required_wits: RequiredWitnessSet,
}

impl TransactionWitnessSetBuilder {
    // pub fn get_vkeys(&self) -> Vkeys {
    //     Vkeys(self.vkeys.clone().into_keys().collect())
    // }

    pub fn add_vkey(&mut self, vkey_witness: Vkeywitness) {
        let vkey = vkey_witness.vkey.clone();
        self.vkeys.insert(vkey, vkey_witness);
    }

    pub fn add_bootstrap(&mut self, bootstrap: BootstrapWitness) {
        self.bootstraps
            .insert(bootstrap.public_key.clone(), bootstrap);
    }

    // pub fn get_bootstraps(&self) -> Vkeys {
    //     Vkeys(self.bootstraps.clone().into_keys().collect())
    // }

    pub fn add_script(&mut self, script: Script) {
        self.scripts.insert(script.hash(), script);
    }

    pub fn get_native_script(&self) -> Vec<NativeScript> {
        self.scripts
            .iter()
            .filter(|entry| !self.required_wits.script_refs.contains(entry.0))
            .fold(
                Vec::<NativeScript>::new(),
                |mut acc, script| match &script.1 {
                    Script::Native { script, .. } => {
                        acc.push(script.clone());
                        acc
                    }
                    _ => acc,
                },
            )
    }

    pub fn get_plutus_v1_script(&self) -> Vec<PlutusV1Script> {
        self.scripts
            .iter()
            .filter(|entry| !self.required_wits.script_refs.contains(entry.0))
            .fold(
                Vec::<PlutusV1Script>::new(),
                |mut acc, script| match &script.1 {
                    Script::PlutusV1 { script, .. } => {
                        acc.push(script.clone());
                        acc
                    }
                    _ => acc,
                },
            )
    }

    pub fn get_plutus_v2_script(&self) -> Vec<PlutusV2Script> {
        self.scripts
            .iter()
            .filter(|entry| !self.required_wits.script_refs.contains(entry.0))
            .fold(
                Vec::<PlutusV2Script>::new(),
                |mut acc, script| match &script.1 {
                    &Script::PlutusV2 { script, .. } => {
                        acc.push(script.clone());
                        acc
                    }
                    _ => acc,
                },
            )
    }

    pub fn get_plutus_v3_script(&self) -> Vec<PlutusV3Script> {
        self.scripts
            .iter()
            .filter(|entry| !self.required_wits.script_refs.contains(entry.0))
            .fold(
                Vec::<PlutusV3Script>::new(),
                |mut acc, script| match &script.1 {
                    &Script::PlutusV3 { script, .. } => {
                        acc.push(script.clone());
                        acc
                    }
                    _ => acc,
                },
            )
    }

    pub fn add_plutus_datum(&mut self, plutus_datum: PlutusData) {
        self.plutus_data
            .insert(hash_plutus_data(&plutus_datum), plutus_datum);
    }

    pub fn get_plutus_datum(&self) -> Vec<PlutusData> {
        self.plutus_data.values().cloned().collect()
    }

    pub fn add_redeemer(&mut self, redeemer: LegacyRedeemer) {
        self.redeemers
            .insert(RedeemerWitnessKey::from(&redeemer), redeemer);
    }

    pub fn get_redeemer(&self) -> Vec<LegacyRedeemer> {
        self.redeemers.values().cloned().collect()
    }

    pub fn add_required_wits(&mut self, required_wits: RequiredWitnessSet) {
        self.required_wits.add_all(required_wits)
    }

    pub fn new() -> Self {
        // have to expose new so it's visible in WASM
        Self::default()
    }

    pub fn add_existing(&mut self, wit_set: TransactionWitnessSet) {
        if let Some(vkeys) = wit_set.vkeywitnesses {
            vkeys.iter().for_each(|vkey| {
                self.add_vkey(vkey.clone());
            });
        }
        if let Some(bootstraps) = wit_set.bootstrap_witnesses {
            bootstraps.iter().for_each(|bootstrap| {
                self.add_bootstrap(bootstrap.clone());
            });
        }
        if let Some(native_scripts) = wit_set.native_scripts {
            native_scripts.iter().for_each(|native_script| {
                self.add_script(native_script.clone().into());
            });
        }
        if let Some(plutus_scripts) = wit_set.plutus_v1_scripts {
            plutus_scripts.iter().for_each(|plutus_script| {
                self.add_script(plutus_script.clone().into());
            });
        }
        if let Some(plutus_scripts) = wit_set.plutus_v2_scripts {
            plutus_scripts.iter().for_each(|plutus_script| {
                self.add_script(plutus_script.clone().into());
            });
        }
        if let Some(plutus_scripts) = wit_set.plutus_v3_scripts {
            plutus_scripts.iter().for_each(|plutus_script| {
                self.add_script(plutus_script.clone().into());
            });
        }
        if let Some(redeemers) = wit_set.redeemers {
            redeemers.to_flat_format().into_iter().for_each(|redeemer| {
                self.add_redeemer(redeemer);
            });
        }
        if let Some(plutus_datums) = wit_set.plutus_datums {
            plutus_datums.iter().for_each(|plutus_datum| {
                self.add_plutus_datum(plutus_datum.clone());
            });
        }
    }

    pub(crate) fn add_input_aggregate_real_witness_data(
        &mut self,
        data: &InputAggregateWitnessData,
    ) {
        match data {
            InputAggregateWitnessData::NativeScript(script, _info) => {
                self.add_script(script.clone().into());
            }
            InputAggregateWitnessData::PlutusScript(witness, _info, option) => {
                match &witness.script {
                    PlutusScriptWitness::Script(plutus_script) => {
                        self.add_script(plutus_script.clone().into());
                    }
                    PlutusScriptWitness::Ref(_) => {
                        // We don't add the script references to the witness set
                    }
                }
                if let Some(data) = option {
                    self.add_plutus_datum(data.clone());
                }
            }
        }
    }

    pub fn build(self) -> TransactionWitnessSet {
        let mut result = TransactionWitnessSet::new();
        let native_scripts = self.get_native_script();
        let plutus_v1_scripts = self.get_plutus_v1_script();
        let plutus_v2_scripts = self.get_plutus_v2_script();
        let plutus_v3_scripts = self.get_plutus_v3_script();
        let plutus_datums = self.get_plutus_datum();

        if !self.vkeys.is_empty() {
            result.vkeywitnesses = Some(self.vkeys.into_values().collect::<Vec<_>>().into());
        }

        if !self.bootstraps.is_empty() {
            result.bootstrap_witnesses =
                Some(self.bootstraps.into_values().collect::<Vec<_>>().into());
        }

        if !native_scripts.is_empty() {
            result.native_scripts = Some(native_scripts.into());
        }

        if !plutus_v1_scripts.is_empty() {
            result.plutus_v1_scripts = Some(plutus_v1_scripts.into());
        }

        if !plutus_v2_scripts.is_empty() {
            result.plutus_v2_scripts = Some(plutus_v2_scripts.into());
        }

        if !plutus_v3_scripts.is_empty() {
            result.plutus_v3_scripts = Some(plutus_v3_scripts.into());
        }

        if !self.plutus_data.is_empty() {
            result.plutus_datums = Some(plutus_datums.into());
        }

        if !self.redeemers.is_empty() {
            result.redeemers = Some(Redeemers::new_arr_legacy_redeemer(
                self.redeemers.values().cloned().collect::<Vec<_>>(),
            ));
        }

        result
    }

    pub fn remaining_wits(&self) -> RequiredWitnessSet {
        let mut remaining_wits = self.required_wits.clone();

        self.vkeys.keys().for_each(|key| {
            remaining_wits.vkeys.remove(&key.hash());
        });
        self.bootstraps.values().for_each(|wit| {
            remaining_wits
                .bootstraps
                .remove(&wit.to_address().unwrap().to_address());
        });
        self.scripts.keys().for_each(|hash| {
            remaining_wits.scripts.remove(hash);
        });
        self.plutus_data.keys().for_each(|hash| {
            remaining_wits.plutus_data.remove(hash);
        });
        self.redeemers.keys().for_each(|key| {
            remaining_wits.redeemers.remove(key);
        });

        remaining_wits
    }

    pub fn try_build(&self) -> Result<TransactionWitnessSet, WitnessBuilderError> {
        let remaining_wits = self.remaining_wits();

        if remaining_wits.len() > 0 {
            return Err(WitnessBuilderError::MissingWitnesses(remaining_wits));
        }

        Ok(self.clone().build())
    }
}

pub fn merge_fake_witness(
    builder: &mut TransactionWitnessSetBuilder,
    required_wits: &RequiredWitnessSet,
) {
    let mut remaining_wits = required_wits.clone();
    builder.vkeys.keys().for_each(|key| {
        remaining_wits.vkeys.remove(&key.hash());
    });
    builder.bootstraps.values().for_each(|wit| {
        remaining_wits
            .bootstraps
            .remove(&wit.to_address().unwrap().to_address());
    });

    // Ed25519KeyHash and AddressId are both (under no collision assumption) 1-1 mapping to the real keys
    // so if all we care about is counting the number of witnesses,
    // we can convert them to fake witnesses that just pad a dummy prefix to their 28byte size to get them to 32 bytes
    let fake_prefix = [0u8; 4];

    for remaining_vkey in remaining_wits.vkeys.iter() {
        let fake_vkey =
            PublicKey::from_raw_bytes(&[&fake_prefix, remaining_vkey.to_raw_bytes()].concat())
                .unwrap();
        let fake_sig = fake_raw_key_sig(0);
        let fake_vkey_witness = Vkeywitness::new(fake_vkey, fake_sig);

        // avoid accidentally overriding real witness
        if !builder.vkeys.contains_key(&fake_vkey_witness.vkey) {
            builder.add_vkey(fake_vkey_witness);
        }
    }
    for remaining_bootstrap in remaining_wits.bootstraps.iter() {
        let address_content = &remaining_bootstrap.content;
        let fake_vkey = PublicKey::from_raw_bytes(
            &[&fake_prefix, address_content.address_id.to_raw_bytes()].concat(),
        )
        .unwrap();
        let fake_sig = fake_raw_key_sig(0);
        let fake_chaincode = [0u8; 32]; // constant size so it won't affect the fee calculation
        let fake_witness = BootstrapWitness::new(
            fake_vkey,
            fake_sig,
            fake_chaincode.to_vec(),
            address_content.addr_attributes.clone(),
        )
        .unwrap();

        // avoid accidentally overriding real witness
        if !builder.bootstraps.contains_key(&fake_witness.public_key) {
            builder.add_bootstrap(fake_witness);
        }
    }
}

fn fake_raw_key_sig(id: u8) -> Ed25519Signature {
    Ed25519Signature::from_raw_bytes(&[
        id, 248, 153, 211, 155, 23, 253, 93, 102, 193, 146, 196, 181, 13, 52, 62, 66, 247, 35, 91,
        48, 80, 76, 138, 231, 97, 159, 147, 200, 40, 220, 109, 206, 69, 104, 221, 105, 23, 124, 85,
        24, 40, 73, 45, 119, 122, 103, 39, 253, 102, 194, 251, 204, 189, 168, 194, 174, 237, 146,
        3, 44, 153, 121, 10,
    ])
    .unwrap()
}

fn fake_key_hash(x: u8) -> Ed25519KeyHash {
    Ed25519KeyHash::from_raw_bytes(&[
        x, 239, 181, 120, 142, 135, 19, 200, 68, 223, 211, 43, 46, 145, 222, 30, 48, 159, 239, 255,
        213, 85, 248, 39, 204, 158, 225, 100,
    ])
    .unwrap()
}

#[derive(Clone, Debug)]
pub enum NativeScriptWitnessInfo {
    Count(usize),
    Vkeys(Vec<Ed25519KeyHash>),
    AssumeWorst,
}

impl NativeScriptWitnessInfo {
    /// Unsure which keys will sign, but you know the exact number to save on tx fee
    pub fn num_signatures(num: usize) -> Self {
        NativeScriptWitnessInfo::Count(num)
    }

    /// This native script will be witnessed by exactly these keys
    pub fn vkeys(vkeys: Vec<Ed25519KeyHash>) -> Self {
        NativeScriptWitnessInfo::Vkeys(vkeys)
    }

    /// You don't know how many keys will sign, so the maximum possible case will be assumed
    pub fn assume_signature_count() -> Self {
        NativeScriptWitnessInfo::AssumeWorst
    }
}

#[cfg(test)]
mod tests {
    use cml_crypto::{Bip32PrivateKey, Deserialize, Serialize, TransactionHash};

    use crate::byron::make_icarus_bootstrap_witness;

    use super::*;

    fn fake_raw_key_public(id: u8) -> PublicKey {
        PublicKey::from_raw_bytes(&[
            id, 118, 57, 154, 33, 13, 232, 114, 14, 159, 168, 148, 228, 94, 65, 226, 154, 181, 37,
            227, 11, 196, 2, 128, 28, 7, 98, 80, 209, 88, 91, 205,
        ])
        .unwrap()
    }

    fn fake_private_key1() -> Bip32PrivateKey {
        Bip32PrivateKey::from_raw_bytes(&[
            0xb8, 0xf2, 0xbe, 0xce, 0x9b, 0xdf, 0xe2, 0xb0, 0x28, 0x2f, 0x5b, 0xad, 0x70, 0x55,
            0x62, 0xac, 0x99, 0x6e, 0xfb, 0x6a, 0xf9, 0x6b, 0x64, 0x8f, 0x44, 0x45, 0xec, 0x44,
            0xf4, 0x7a, 0xd9, 0x5c, 0x10, 0xe3, 0xd7, 0x2f, 0x26, 0xed, 0x07, 0x54, 0x22, 0xa3,
            0x6e, 0xd8, 0x58, 0x5c, 0x74, 0x5a, 0x0e, 0x11, 0x50, 0xbc, 0xce, 0xba, 0x23, 0x57,
            0xd0, 0x58, 0x63, 0x69, 0x91, 0xf3, 0x8a, 0x37, 0x91, 0xe2, 0x48, 0xde, 0x50, 0x9c,
            0x07, 0x0d, 0x81, 0x2a, 0xb2, 0xfd, 0xa5, 0x78, 0x60, 0xac, 0x87, 0x6b, 0xc4, 0x89,
            0x19, 0x2c, 0x1e, 0xf4, 0xce, 0x25, 0x3c, 0x19, 0x7e, 0xe2, 0x19, 0xa4,
        ])
        .unwrap()
    }

    fn fake_private_key2() -> Bip32PrivateKey {
        Bip32PrivateKey::from_raw_bytes(
            &hex::decode("d84c65426109a36edda5375ea67f1b738e1dacf8629f2bb5a2b0b20f3cd5075873bf5cdfa7e533482677219ac7d639e30a38e2e645ea9140855f44ff09e60c52c8b95d0d35fe75a70f9f5633a3e2439b2994b9e2bc851c49e9f91d1a5dcbb1a3").unwrap()
        ).unwrap()
    }

    #[test]
    fn test_add_fake_vkey_witnesses_by_num() {
        let mut builder = RequiredWitnessSet::new();
        builder.add_fake_vkey_witnesses_by_num(2);
        assert_eq!(builder.vkeys.len(), 2);
        builder.add_fake_vkey_witnesses_by_num(1);
        assert_eq!(builder.vkeys.len(), 3);
    }

    #[test]
    fn test_add_input_aggregate_witness_data() {
        let mut required_wits = RequiredWitnessSet::new();
        let data = {
            let witness = {
                let script = PlutusScript::PlutusV1(PlutusV1Script::new(vec![0]));
                PartialPlutusWitness {
                    script: PlutusScriptWitness::Script(script),
                    redeemer: PlutusData::new_integer(0u64.into()),
                }
            };
            let missing_signers = vec![fake_raw_key_public(0).hash()];
            InputAggregateWitnessData::PlutusScript(witness, missing_signers.into(), None)
        };

        assert_eq!(required_wits.vkeys.len(), 0);
        required_wits.add_input_aggregate_fake_witness_data(&data);
        assert_eq!(required_wits.vkeys.len(), 1);
    }

    #[test]
    fn test_add_input_aggregate_witness_data_with_existing_key_hash() {
        let mut required_wits = RequiredWitnessSet::new();
        let key = fake_raw_key_public(0);
        let hash = key.hash();
        required_wits.add_vkey_key_hash(hash);

        let data = {
            let witness = {
                let script = PlutusScript::PlutusV1(PlutusV1Script::new(vec![0]));
                PartialPlutusWitness {
                    script: PlutusScriptWitness::Script(script),
                    redeemer: PlutusData::new_integer(0u64.into()),
                }
            };
            let missing_signers = vec![hash];
            InputAggregateWitnessData::PlutusScript(witness, missing_signers.into(), None)
        };

        assert_eq!(required_wits.vkeys.len(), 1);
        required_wits.add_input_aggregate_fake_witness_data(&data);
        assert_eq!(required_wits.vkeys.len(), 1);
    }

    #[test]
    fn vkey_test() {
        let mut builder = TransactionWitnessSetBuilder::new();
        let raw_key_public = fake_raw_key_public(0);
        let fake_sig = fake_raw_key_sig(0);

        // add the same element twice
        builder.add_vkey(Vkeywitness::new(raw_key_public.clone(), fake_sig.clone()));
        builder.add_vkey(Vkeywitness::new(raw_key_public, fake_sig));

        // add a different element
        builder.add_vkey(Vkeywitness::new(
            fake_raw_key_public(1),
            fake_raw_key_sig(1),
        ));

        let wit_set = builder.build();
        assert_eq!(wit_set.vkeywitnesses.unwrap().len(), 2);
    }

    #[test]
    fn bootstrap_test() {
        let mut builder = TransactionWitnessSetBuilder::new();

        // add the same element twice
        let wit1 = make_icarus_bootstrap_witness(
            TransactionHash::from([0u8; TransactionHash::BYTE_COUNT]),
            ByronAddress::from_base58(
                "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
            )
            .unwrap(),
            &fake_private_key1(),
        );
        builder.add_bootstrap(wit1.clone());
        builder.add_bootstrap(wit1);

        // add a different element
        builder.add_bootstrap(make_icarus_bootstrap_witness(
            TransactionHash::from([0u8; TransactionHash::BYTE_COUNT]),
            ByronAddress::from_base58(
                "Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho",
            )
            .unwrap(),
            &fake_private_key2(),
        ));

        let wit_set = builder.build();
        assert_eq!(wit_set.bootstrap_witnesses.unwrap().len(), 2);
    }

    #[test]
    fn native_script_test() {
        let mut builder = TransactionWitnessSetBuilder::new();

        // add the same element twice
        let wit1: Script = NativeScript::new_script_invalid_before(1).into();
        builder.add_script(wit1.clone());
        builder.add_script(wit1);

        // add a different element
        builder.add_script(NativeScript::new_script_invalid_before(2).into());

        let wit_set = builder.build();
        assert_eq!(wit_set.native_scripts.unwrap().len(), 2);
    }

    // TODO: tests for plutus scripts (v1 & v2), plutus_data, redeemers
    // once we have mock data for them
    #[test]
    fn requirement_test_fail() {
        let mut builder = TransactionWitnessSetBuilder::new();

        let mut required_wits = RequiredWitnessSet::new();
        required_wits.add_vkey_key_hash(fake_raw_key_public(0).hash());
        required_wits.add_script_hash(NativeScript::new_script_invalid_before(2).hash());
        builder.add_required_wits(required_wits);

        // add a different element
        builder.add_vkey(Vkeywitness::new(
            fake_raw_key_public(1),
            fake_raw_key_sig(1),
        ));

        assert!(builder.try_build().is_err());
    }

    #[test]
    fn requirement_test_pass() {
        let mut builder = TransactionWitnessSetBuilder::new();

        let mut required_wits = RequiredWitnessSet::new();
        required_wits.add_vkey_key_hash(fake_raw_key_public(0).hash());
        builder.add_required_wits(required_wits);

        // add a different element
        builder.add_vkey(Vkeywitness::new(
            fake_raw_key_public(0),
            fake_raw_key_sig(0),
        ));

        assert!(builder.try_build().is_ok());
    }

    #[test]
    fn tx_witness_set_roundtrip_test() {
        let data = "a102818458205e8379f58f0838234af67f73738f0fee0d8185232e200b8e42887f4f06544a9a5840f5cfea560d2f8645ed624b65bf08cf83346eb5168ee4df0f63ce2d0d5f677db88fef2d5d9f032f09223889b5e85504ab44dd0a0cde1f1fd8f57deefde8c2080658202d3b7d9b806f88f10f1193e94ef97e5c02370c1464f61a30a8f1ac1a46115b2d5829a201581e581c072931653330243cf126aea85d39e73c6bd04601fe77424efb9e371002451a4170cb17";
        let witness_set =
            TransactionWitnessSet::from_cbor_bytes(&hex::decode(data).unwrap()).unwrap();
        let round_trip = witness_set.to_cbor_bytes();

        assert_eq!(data, hex::encode(round_trip));
    }
}
