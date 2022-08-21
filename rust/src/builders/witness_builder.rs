use std::{collections::{HashMap}, fmt::Debug};
use crate::{*, ledger::common::hash::hash_plutus_data, byron::ByronAddress};

use super::redeemer_builder::RedeemerWitnessKey;

/// A partial Plutus witness
/// It contains all the information needed to witness the Plutus script execution
/// except for the redeemer tag and index
/// Note: no datum is attached because only input script types have datums
#[wasm_bindgen]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PartialPlutusWitness {
    pub(crate) script: PlutusScript,
    pub(crate) data: PlutusData,
}

#[wasm_bindgen]
impl PartialPlutusWitness {
    pub fn new(
        script: &PlutusScript,
        data: &PlutusData
    ) -> Self {
        Self {
            script: script.clone(),
            data: data.clone(),
        }
    }

    pub fn script(&self) -> PlutusScript {
        self.script.clone()
    }

    pub fn data(&self) -> PlutusData {
        self.data.clone()
    }
}

#[derive(Clone, Debug)]
pub enum InputAggregateWitnessData {
    NativeScript(NativeScript, NativeScriptWitnessInfo),
    PlutusScript(PartialPlutusWitness, RequiredSigners, Option<PlutusData>)
}

impl InputAggregateWitnessData {
    pub fn plutus_data(&self) -> Option<PlutusData> {
        match self {
            InputAggregateWitnessData::PlutusScript(witness, _, _) => {
                Some(witness.data())
            }
            _ => None
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct RequiredWitnessSet {
    // note: the real key type for these is Vkey
    // but cryptographically these should be equivalent and Ed25519KeyHash is more flexible
    pub(crate) vkeys: BTreeSet<Ed25519KeyHash>,
    // note: the real key type for these is Vkey
    // but cryptographically these should be equivalent as ByronAddress contains an AddressId
    // which is the hash of data that includes the public key
    pub(crate) bootstraps: BTreeSet<ByronAddress>,
    // note: no way to differentiate Plutus script from native script
    pub(crate) scripts: BTreeSet<ScriptHash>,
    pub(crate) plutus_data: BTreeSet<DataHash>,
    pub(crate) redeemers: BTreeSet<RedeemerWitnessKey>,
    pub(crate) script_refs: BTreeSet<ScriptHash>,
}

#[wasm_bindgen]
impl RequiredWitnessSet {
    pub fn add_vkey(&mut self, vkey: &Vkeywitness) {
        self.add_vkey_key(&vkey.vkey());
    }
    pub fn add_vkey_key(&mut self, vkey: &Vkey) {
        self.add_vkey_key_hash(&vkey.public_key().hash());
    }
    pub fn add_vkey_key_hash(&mut self, hash: &Ed25519KeyHash) {
        self.vkeys.insert(hash.clone());
    }

    pub fn add_bootstrap(&mut self, address: &ByronAddress) {
        self.bootstraps.insert(address.clone());
    }

    pub fn add_script_ref(&mut self, script_hash: &ScriptHash) {
        self.script_refs.insert(script_hash.clone());
        self.scripts.remove(script_hash);
    }

    pub fn add_native_script(&mut self, native_script: &NativeScript) {
        self.add_script_hash(&native_script.hash());
    }

    pub fn add_script_hash(&mut self, script_hash: &ScriptHash) {
        match self.script_refs.get(script_hash) {
            None => { self.scripts.insert(script_hash.clone()); },
            Some(_) => {}
        }
    }

    pub fn add_plutus_script(&mut self, plutus_v1_script: &PlutusScript) {
        self.add_script_hash(&plutus_v1_script.hash());
    }

    pub fn add_plutus_datum(&mut self, plutus_datum: &PlutusData) {
        self.add_plutus_datum_hash(&hash_plutus_data(plutus_datum));
    }
    pub fn add_plutus_datum_hash(&mut self, plutus_datum: &DataHash) {
        self.plutus_data.insert(plutus_datum.clone());
    }

    pub fn add_redeemer(&mut self, redeemer: &Redeemer) {
        self.add_redeemer_tag(&RedeemerWitnessKey::new(&redeemer.tag(), &redeemer.index()));
    }
    pub fn add_redeemer_tag(&mut self, redeemer: &RedeemerWitnessKey) {
        self.redeemers.insert(redeemer.clone());
    }

    pub fn add_all(&mut self, requirements: &RequiredWitnessSet) {
        self.vkeys.extend(requirements.vkeys.iter().cloned());
        self.bootstraps.extend(requirements.bootstraps.iter().cloned());
        self.scripts.extend(requirements.scripts.iter().cloned());
        self.plutus_data.extend(requirements.plutus_data.iter().cloned());
        self.redeemers.extend(requirements.redeemers.iter().cloned());
    }

    pub (crate) fn to_str(&self) -> String {
        let vkeys = self.vkeys.iter().map(|key| format!("Vkey:{}", hex::encode(key.to_bytes()))).collect::<Vec<String>>().join(",");
        let bootstraps = self.bootstraps.iter().map(|data| format!("Legacy Bootstraps addresses:{}", data.to_base58())).collect::<Vec<String>>().join(",");
        let scripts = self.scripts.iter().map(|hash| format!("Script hash:{}", hex::encode(hash.to_bytes()))).collect::<Vec<String>>().join(",");
        let plutus_data = self.plutus_data.iter().map(|hash| format!("Plutus data hash:{}", hex::encode(hash.to_bytes()))).collect::<Vec<String>>().join(",");
        let redeemers = self.redeemers.iter().map(|key| format!("Redeemer:{}-{}", hex::encode(key.tag().to_bytes()), key.index().to_str())).collect::<Vec<String>>().join(",");

        [vkeys, bootstraps, scripts, plutus_data, redeemers].iter().filter(|msg| !msg.is_empty()).cloned().collect::<Vec<String>>().join("\n")
    }

    pub (crate) fn len(&self) -> usize {
        self.vkeys.len() +
            self.bootstraps.len() +
            self.scripts.len() +
            self.plutus_data.len() +
            self.redeemers.len()
    }


    // This method add fake vkeys to calculate the fee.
    // In order to prevent that fake keys get deduplicated when it is called more than once,
    // its index starts from the current amount of vkeys.
    // WARN: this function might fail at runtime when there are more than 255 witnesses,
    // however this is unrealistic because the limit of transaction size. (101 bytes each witness)
    pub(crate) fn add_fake_vkey_witnesses_by_num(&mut self, num: usize) {
        for _ in 0..num {
            self.add_vkey_key_hash(&fake_key_hash(self.vkeys.len() as u8));
        }
    }

    pub(crate) fn add_input_aggregate_fake_witness_data(&mut self, data: &InputAggregateWitnessData) {
        match data {
            InputAggregateWitnessData::NativeScript(script, info) => {
                match info.0 {
                    NativeScriptWitnessInfoKind::Count(num) => self.add_fake_vkey_witnesses_by_num(num),
                    NativeScriptWitnessInfoKind::Vkeys(ref vkeys) => { vkeys.iter().for_each(|vkey| self.add_vkey_key_hash(vkey)); },
                    NativeScriptWitnessInfoKind::AssumeWorst => {
                        // we get the size instead of the hashes themselves
                        // since there is no way to know if any of these will actually be required to sign the tx
                        let num = script.get_required_signers().len();
                        self.add_fake_vkey_witnesses_by_num(num);
                    }
                }
            }
            InputAggregateWitnessData::PlutusScript(_witness, required_signers, _option) => {
                required_signers.0.iter().for_each(|vkey| self.add_vkey_key_hash(vkey));
            }
        }
    }

    pub fn new() -> Self {
        // have to expose new so it's visible in WASM
        Self::default()
    }
}

/// Builder de-duplicates witnesses as they are added
#[wasm_bindgen]
#[derive(Clone, Default, Debug)]
pub struct TransactionWitnessSetBuilder {
    // See Alonzo spec section 3.1 which defines the keys for these types
    pub(crate) vkeys: HashMap<Vkey, Vkeywitness>,
    pub(crate) bootstraps: HashMap<Vkey, BootstrapWitness>,
    pub(crate) scripts: HashMap<ScriptHash, ScriptEnum>,
    pub(crate) plutus_data: HashMap<DataHash, PlutusData>,
    pub(crate) redeemers: HashMap<RedeemerWitnessKey, Redeemer>,

    /// witnesses that need to be added for the build function to succeed
    /// this allows checking that witnesses are present at build time (instead of when submitting to a node)
    /// This is useful for APIs that can keep track of which witnesses will be required (like transaction builders)
    pub(crate) required_wits: RequiredWitnessSet,
}

#[wasm_bindgen]
impl TransactionWitnessSetBuilder {
    pub fn get_vkeys(&self) -> Vkeys {
        Vkeys(self.vkeys.clone().into_keys().collect())
    }

    pub fn add_vkey(&mut self, vkey: &Vkeywitness) {
        self.vkeys.insert(vkey.vkey(), vkey.clone());
    }

    pub fn add_bootstrap(&mut self, bootstrap: &BootstrapWitness) {
        self.bootstraps.insert(bootstrap.vkey(), bootstrap.clone());
    }

    pub fn get_bootstraps(&self) -> Vkeys {
        Vkeys(self.bootstraps.clone().into_keys().collect())
    }

    pub fn add_script(&mut self, script: &Script) {
        match &script.0 {
            ScriptEnum::Native(native) => self.add_native_script(native),
            ScriptEnum::PlutusV1(plutus_v1) => self.add_plutus_v1_script(plutus_v1),
            ScriptEnum::PlutusV2(plutus_v2) => self.add_plutus_v2_script(plutus_v2),
        }
    }

    pub fn add_native_script(&mut self, native_script: &NativeScript) {
        self.scripts.insert(native_script.hash(),  ScriptEnum::Native(native_script.clone()));
    }

    pub fn get_native_script(&self) -> NativeScripts {
        let scripts: Vec<NativeScript> = self.scripts
            .iter()
            .filter(|entry| self.required_wits.script_refs.get(entry.0).is_none())
            .fold(
                Vec::<NativeScript>::new(),
                |mut acc, script| match &script.1 {
                    &ScriptEnum::Native(native_script) => { acc.push(native_script.clone()); acc },
                    _ => acc
                }
            );
        NativeScripts(scripts)
    }

    pub fn add_plutus_v1_script(&mut self, plutus_v1_script: &PlutusV1Script) {
        self.scripts.insert(plutus_v1_script.hash(), ScriptEnum::PlutusV1(plutus_v1_script.clone()));
    }

    pub fn get_plutus_v1_script(&self) -> PlutusV1Scripts {
        let scripts: Vec<PlutusV1Script> = self.scripts
            .iter()
            .filter(|entry| self.required_wits.script_refs.get(entry.0).is_none())
            .fold(
                Vec::<PlutusV1Script>::new(),
                |mut acc, script| match &script.1 {
                    &ScriptEnum::PlutusV1(plutus_script) => { acc.push(plutus_script.clone()); acc },
                    _ => acc
                }
            );
            PlutusV1Scripts(scripts)
    }

    pub fn add_plutus_v2_script(&mut self, plutus_v2_script: &PlutusV2Script) {
        self.scripts.insert(plutus_v2_script.hash(), ScriptEnum::PlutusV2(plutus_v2_script.clone()));
    }

    pub fn get_plutus_v2_script(&self) -> PlutusV2Scripts {
        let scripts: Vec<PlutusV2Script> = self.scripts
            .iter()
            .filter(|entry| self.required_wits.script_refs.get(entry.0).is_none())
            .fold(
                Vec::<PlutusV2Script>::new(),
                |mut acc, script| match &script.1 {
                    &ScriptEnum::PlutusV2(plutus_script) => { acc.push(plutus_script.clone()); acc },
                    _ => acc
                }
            );
        PlutusV2Scripts(scripts)
    }

    pub fn add_plutus_datum(&mut self, plutus_datum: &PlutusData) {
        self.plutus_data.insert(hash_plutus_data(plutus_datum), plutus_datum.clone());
    }

    pub fn get_plutus_datum(&self) -> PlutusList {
        PlutusList {
            elems: self.plutus_data.clone().into_values().collect(),
            definite_encoding: None
        }
    }

    pub fn add_redeemer(&mut self, redeemer: &Redeemer) {
        self.redeemers.insert(
            RedeemerWitnessKey::new(&redeemer.tag(), &redeemer.index()),
            redeemer.clone()
        );
    }

    pub fn add_redeemers(&mut self, redeemers: &Redeemers) {
        redeemers.0.iter().for_each(|redeemer| self.add_redeemer(redeemer));
    }

    pub fn get_redeemer(&self) -> Redeemers {
        Redeemers(self.redeemers.clone().into_values().collect())
    }

    pub fn add_required_wits(&mut self, required_wits: &RequiredWitnessSet) {
        self.required_wits.add_all(required_wits)
    }

    pub fn new() -> Self {
        // have to expose new so it's visible in WASM
        Self::default()
    }

    pub fn add_existing(&mut self, wit_set: &TransactionWitnessSet) {
        if let Some(vkeys) = wit_set.vkeys() {
            vkeys.0.iter().for_each(|vkey| { self.add_vkey(vkey); } );
        }
        if let Some(bootstraps) = &wit_set.bootstraps() {
            bootstraps.0.iter().for_each(|bootstrap| { self.add_bootstrap(bootstrap); } );
        }
        if let Some(native_scripts) = &wit_set.native_scripts() {
            native_scripts.0.iter().for_each(|native_script| { self.add_native_script(native_script); } );
        }
        if let Some(plutus_scripts) = &wit_set.plutus_v1_scripts() {
            plutus_scripts.0.iter().for_each(|plutus_script| { self.add_plutus_v1_script(plutus_script); } );
        }
        if let Some(plutus_scripts) = &wit_set.plutus_v2_scripts() {
            plutus_scripts.0.iter().for_each(|plutus_script| { self.add_plutus_v2_script(plutus_script); } );
        }
        if let Some(redeemers) = &wit_set.redeemers() {
            redeemers.0.iter().for_each(|redeemer| { self.add_redeemer(redeemer); } );
        }
    }

    pub(crate) fn add_input_aggregate_real_witness_data(&mut self, data: &InputAggregateWitnessData) {
        match data {
            InputAggregateWitnessData::NativeScript(script, _info) => {
                self.add_native_script(script);
            }
            InputAggregateWitnessData::PlutusScript(witness, _info, option) => {
                match &witness.script.0 {
                    PlutusScriptEnum::PlutusV1(script) => self.add_plutus_v1_script(script),
                    PlutusScriptEnum::PlutusV2(script) => self.add_plutus_v2_script(script)
                }
                if let Some(ref data) = option {
                    self.add_plutus_datum(data);
                }
            }
        }
    }

    pub fn build(&self) -> TransactionWitnessSet {
        let mut result = TransactionWitnessSet::new();

        if !self.vkeys.is_empty() {
            result.set_vkeys(&Vkeywitnesses(self.vkeys.values().cloned().collect()));
        }
        if !self.bootstraps.is_empty() {
            result.set_bootstraps(&BootstrapWitnesses(self.bootstraps.values().cloned().collect()));
        }

        {
            let native_scripts = self.get_native_script();
            if !native_scripts.0.is_empty() {
                result.set_native_scripts(&native_scripts);
            }
        }
        {
            let plutus_scripts = self.get_plutus_v1_script();
            if !plutus_scripts.0.is_empty() {
                result.set_plutus_v1_scripts(&plutus_scripts);
            }
        }
        {
            let plutus_scripts = self.get_plutus_v2_script();
            if !plutus_scripts.0.is_empty() {
                result.set_plutus_v2_scripts(&plutus_scripts);
            }
        }
        if !self.plutus_data.is_empty() {
            result.set_plutus_data(&self.get_plutus_datum());
        }
        if !self.redeemers.is_empty() {
            result.set_redeemers(&Redeemers(self.redeemers.values().cloned().collect()));
        }

        result
    }

    pub fn remaining_wits(&self) -> RequiredWitnessSet {
        let mut remaining_wits = self.required_wits.clone();

        self.vkeys.keys().for_each(|key| { remaining_wits.vkeys.remove(&key.public_key().hash()); });
        self.bootstraps.values().for_each(|wit| { remaining_wits.bootstraps.remove(&wit.to_address().unwrap().to_address()); });
        self.scripts.keys().for_each(|hash| { remaining_wits.scripts.remove(hash); });
        self.plutus_data.keys().for_each(|hash| { remaining_wits.plutus_data.remove(hash); });
        self.redeemers.keys().for_each(|key| { remaining_wits.redeemers.remove(key); });

        remaining_wits
    }

    pub fn try_build(&self) -> Result<TransactionWitnessSet, JsError> {
        let remaining_wits = self.remaining_wits();

        if remaining_wits.len() > 0 {
            return Err(JsError::from_str(&format!("Missing following witnesses:\n{}", remaining_wits.to_str())))
        }

        Ok(self.build())
    }
}

pub fn merge_fake_witness(builder: &mut TransactionWitnessSetBuilder, required_wits: &RequiredWitnessSet) {
    let mut remaining_wits = required_wits.clone();
    builder.vkeys.keys().for_each(|key| { remaining_wits.vkeys.remove(&key.public_key().hash()); });
    builder.bootstraps.values().for_each(|wit| { remaining_wits.bootstraps.remove(&wit.to_address().unwrap().to_address()); });

    // Ed25519KeyHash and AddressId are both (under no collision assumption) 1-1 mapping to the real keys
    // so if all we care about is counting the number of witnesses,
    // we can convert them to fake witnesses that just pad a dummy prefix to their 28byte size to get them to 32 bytes
    let fake_prefix = [0u8; 4];

    for remaining_vkey in remaining_wits.vkeys.iter() {
        let fake_vkey = Vkey::new(&PublicKey::from_bytes(&[&fake_prefix, &remaining_vkey.0[..]].concat()).unwrap());
        let fake_sig = fake_raw_key_sig(0);
        let fake_vkey_witness = Vkeywitness::new(&fake_vkey, &fake_sig);
        builder.add_vkey(&fake_vkey_witness);
    }
    for remaining_bootstrap in remaining_wits.bootstraps.iter() {
        let address_content = remaining_bootstrap.address_content();
        let fake_vkey = Vkey::new(&PublicKey::from_bytes(&[&fake_prefix, &address_content.address_id().0[..]].concat()).unwrap());
        let fake_sig = fake_raw_key_sig(0);
        let fake_chaincode = [0u8; 32]; // constant size so it won't affect the fee calculation
        let fake_witness = BootstrapWitness::new(&fake_vkey, &fake_sig, fake_chaincode.to_vec(), &address_content.addr_attr());
        builder.add_bootstrap(&fake_witness);
    }
}

fn fake_raw_key_sig(id: u8) -> Ed25519Signature {
    Ed25519Signature::from_bytes(
        vec![id, 248, 153, 211, 155, 23, 253, 93, 102, 193, 146, 196, 181, 13, 52, 62, 66, 247, 35, 91, 48, 80, 76, 138, 231, 97, 159, 147, 200, 40, 220, 109, 206, 69, 104, 221, 105, 23, 124, 85, 24, 40, 73, 45, 119, 122, 103, 39, 253, 102, 194, 251, 204, 189, 168, 194, 174, 237, 146, 3, 44, 153, 121, 10]
    ).unwrap()
}

fn fake_raw_key_public(id: u8) -> PublicKey {
    PublicKey::from_bytes(
        &[id, 118, 57, 154, 33, 13, 232, 114, 14, 159, 168, 148, 228, 94, 65, 226, 154, 181, 37, 227, 11, 196, 2, 128, 28, 7, 98, 80, 209, 88, 91, 205]
    ).unwrap()
}

fn fake_key_private(id: u8) -> Bip32PrivateKey {
    Bip32PrivateKey::from_bytes(
        &[0xb8, id, 0xbe, 0xce, 0x9b, 0xdf, 0xe2, 0xb0, 0x28, 0x2f, 0x5b, 0xad, 0x70, 0x55, 0x62, 0xac, 0x99, 0x6e, 0xfb, 0x6a, 0xf9, 0x6b, 0x64, 0x8f,
            0x44, 0x45, 0xec, 0x44, 0xf4, 0x7a, 0xd9, 0x5c, 0x10, 0xe3, 0xd7, 0x2f, 0x26, 0xed, 0x07, 0x54, 0x22, 0xa3, 0x6e, 0xd8, 0x58, 0x5c, 0x74, 0x5a,
            0x0e, 0x11, 0x50, 0xbc, 0xce, 0xba, 0x23, 0x57, 0xd0, 0x58, 0x63, 0x69, 0x91, 0xf3, 0x8a, 0x37, 0x91, 0xe2, 0x48, 0xde, 0x50, 0x9c, 0x07, 0x0d,
            0x81, 0x2a, 0xb2, 0xfd, 0xa5, 0x78, 0x60, 0xac, 0x87, 0x6b, 0xc4, 0x89, 0x19, 0x2c, 0x1e, 0xf4, 0xce, 0x25, 0x3c, 0x19, 0x7e, 0xe2, 0x19, 0xa4]
    ).unwrap()
}

fn fake_key_hash(x: u8) -> Ed25519KeyHash {
    Ed25519KeyHash::from_bytes(
        vec![x, 239, 181, 120, 142, 135, 19, 200, 68, 223, 211, 43, 46, 145, 222, 30, 48, 159, 239, 255, 213, 85, 248, 39, 204, 158, 225, 100]
    ).unwrap()
}

#[derive(Clone, Debug)]
pub enum NativeScriptWitnessInfoKind {
    Count(usize),
    Vkeys(Vec<Ed25519KeyHash>),
    AssumeWorst,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct NativeScriptWitnessInfo(NativeScriptWitnessInfoKind);

#[wasm_bindgen]
impl NativeScriptWitnessInfo {
    /// Unsure which keys will sign, but you know the exact number to save on tx fee
    pub fn num_signatures(num: usize) -> NativeScriptWitnessInfo {
        NativeScriptWitnessInfo(NativeScriptWitnessInfoKind::Count(num))
    }

    /// This native script will be witnessed by exactly these keys
    pub fn vkeys(vkeys: &Ed25519KeyHashes) -> NativeScriptWitnessInfo {
        NativeScriptWitnessInfo(NativeScriptWitnessInfoKind::Vkeys(vkeys.0.clone()))
    }

    /// You don't know how many keys will sign, so the maximum possible case will be assumed
    pub fn assume_signature_count() -> NativeScriptWitnessInfo {
        NativeScriptWitnessInfo(NativeScriptWitnessInfoKind::AssumeWorst)
    }
}

#[cfg(test)]
mod tests {
    use crate::ledger::byron::witness::make_icarus_bootstrap_witness;

    use super::*;

    fn fake_private_key1() -> Bip32PrivateKey {
        Bip32PrivateKey::from_bytes(
            &[0xb8, 0xf2, 0xbe, 0xce, 0x9b, 0xdf, 0xe2, 0xb0, 0x28, 0x2f, 0x5b, 0xad, 0x70, 0x55, 0x62, 0xac, 0x99, 0x6e, 0xfb, 0x6a, 0xf9, 0x6b, 0x64, 0x8f,
                0x44, 0x45, 0xec, 0x44, 0xf4, 0x7a, 0xd9, 0x5c, 0x10, 0xe3, 0xd7, 0x2f, 0x26, 0xed, 0x07, 0x54, 0x22, 0xa3, 0x6e, 0xd8, 0x58, 0x5c, 0x74, 0x5a,
                0x0e, 0x11, 0x50, 0xbc, 0xce, 0xba, 0x23, 0x57, 0xd0, 0x58, 0x63, 0x69, 0x91, 0xf3, 0x8a, 0x37, 0x91, 0xe2, 0x48, 0xde, 0x50, 0x9c, 0x07, 0x0d,
                0x81, 0x2a, 0xb2, 0xfd, 0xa5, 0x78, 0x60, 0xac, 0x87, 0x6b, 0xc4, 0x89, 0x19, 0x2c, 0x1e, 0xf4, 0xce, 0x25, 0x3c, 0x19, 0x7e, 0xe2, 0x19, 0xa4]
        ).unwrap()
    }

    fn fake_private_key2() -> Bip32PrivateKey {
        Bip32PrivateKey::from_bytes(
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
                let script = PlutusScriptEnum::from_v1(&PlutusV1Script::new(vec![0]));
                PartialPlutusWitness::new(&PlutusScript(script), &PlutusData::new_integer(&0u64.into()))
            };
            let missing_signers = {
                let key = fake_raw_key_public(0);
                let mut missing_signers = Ed25519KeyHashes::new();
                missing_signers.add(&key.hash());
                missing_signers
            };
            InputAggregateWitnessData::PlutusScript(witness, missing_signers, None)
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
        required_wits.add_vkey_key_hash(&hash);

        let data = {
            let witness = {
                let script = PlutusScriptEnum::from_v1(&PlutusV1Script::new(vec![0]));
                PartialPlutusWitness::new(&PlutusScript(script), &PlutusData::new_integer(&0u64.into()))
            };
            let missing_signers = {
                let mut missing_signers = Ed25519KeyHashes::new();
                missing_signers.add(&hash);
                missing_signers
            };
            InputAggregateWitnessData::PlutusScript(witness, missing_signers, None)
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
        builder.add_vkey(&Vkeywitness::new(
            &Vkey::new(&raw_key_public),
            &fake_sig
        ));
        builder.add_vkey(&Vkeywitness::new(
            &Vkey::new(&raw_key_public),
            &fake_sig
        ));

        // add a different element
        builder.add_vkey(&Vkeywitness::new(
            &Vkey::new(&fake_raw_key_public(1)),
            &fake_raw_key_sig(1)
        ));

        let wit_set = builder.build();
        assert_eq!(
            wit_set.vkeys().unwrap().len(),
            2
        );
    }

    #[test]
    fn bootstrap_test() {
        let mut builder = TransactionWitnessSetBuilder::new();

        // add the same element twice
        let wit1 = make_icarus_bootstrap_witness(
            &TransactionHash::from([0u8; TransactionHash::BYTE_COUNT]),
            &ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap(),
            &fake_private_key1()
        );
        builder.add_bootstrap(&wit1);
        builder.add_bootstrap(&wit1);

        // add a different element
        builder.add_bootstrap(&make_icarus_bootstrap_witness(
            &TransactionHash::from([0u8; TransactionHash::BYTE_COUNT]),
            &ByronAddress::from_base58("Ae2tdPwUPEZGUEsuMAhvDcy94LKsZxDjCbgaiBBMgYpR8sKf96xJmit7Eho").unwrap(),
            &fake_private_key2()
        ));

        let wit_set = builder.build();
        assert_eq!(
            wit_set.bootstraps().unwrap().len(),
            2
        );
    }

    #[test]
    fn native_script_test() {
        let mut builder = TransactionWitnessSetBuilder::new();

        // add the same element twice
        let wit1 = NativeScript::new_timelock_start(
            &TimelockStart::new(&1.into()),
        );
        builder.add_native_script(&wit1);
        builder.add_native_script(&wit1);

        // add a different element
        builder.add_native_script(&NativeScript::new_timelock_start(
            &TimelockStart::new(&2.into()),
        ));

        let wit_set = builder.build();
        assert_eq!(
            wit_set.native_scripts().unwrap().len(),
            2
        );
    }

    // TODO: tests for plutus scripts (v1 & v2), plutus_data, redeemers
    // once we have mock data for them
    #[test]
    fn requirement_test_fail() {
        let mut builder = TransactionWitnessSetBuilder::new();

        let mut required_wits = RequiredWitnessSet::new();
        required_wits.add_vkey_key(&Vkey::new(&fake_raw_key_public(0)));
        required_wits.add_native_script(&NativeScript::new_timelock_start(
            &TimelockStart::new(&2.into()),
        ));
        builder.add_required_wits(&required_wits);

        // add a different element
        builder.add_vkey(&Vkeywitness::new(
            &Vkey::new(&fake_raw_key_public(1)),
            &fake_raw_key_sig(1)
        ));

        assert!(builder.try_build().is_err());
    }

    #[test]
    fn requirement_test_pass() {
        let mut builder = TransactionWitnessSetBuilder::new();

        let mut required_wits = RequiredWitnessSet::new();
        required_wits.add_vkey_key(&Vkey::new(&fake_raw_key_public(0)));
        builder.add_required_wits(&required_wits);

        // add a different element
        builder.add_vkey(&Vkeywitness::new(
            &Vkey::new(&fake_raw_key_public(0)),
            &fake_raw_key_sig(0)
        ));

        assert!(builder.try_build().is_ok());
    }

    #[test]
    fn tx_witness_set_roundtrip_test() {
        let data = "a102818458205e8379f58f0838234af67f73738f0fee0d8185232e200b8e42887f4f06544a9a5840f5cfea560d2f8645ed624b65bf08cf83346eb5168ee4df0f63ce2d0d5f677db88fef2d5d9f032f09223889b5e85504ab44dd0a0cde1f1fd8f57deefde8c2080658202d3b7d9b806f88f10f1193e94ef97e5c02370c1464f61a30a8f1ac1a46115b2d5829a201581e581c072931653330243cf126aea85d39e73c6bd04601fe77424efb9e371002451a4170cb17";
        let witness_set = TransactionWitnessSet::from_bytes(hex::decode(data).unwrap()).unwrap();
        let round_trip = witness_set.to_bytes();

        assert_eq!(data, hex::encode(&round_trip));
    }
}
