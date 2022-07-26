use std::{collections::{HashSet, HashMap, BTreeMap}, fmt::Debug};
use crate::{*, ledger::{common::hash::{ScriptHashNamespace, hash_plutus_data}, byron::witness::make_icarus_bootstrap_witness}, byron::{ByronAddress}};

use super::{input_builder::InputBuilderResult, mint_builder::MintBuilderResult, withdrawal_builder::WithdrawalBuilderResult, certificate_builder::CertificateBuilderResult};

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RedeemerWitnessKey {
    tag: RedeemerTag,
    index: BigNum,
}

#[wasm_bindgen]
impl RedeemerWitnessKey {

    pub fn tag(&self) -> RedeemerTag {
        self.tag
    }

    pub fn index(&self) -> BigNum {
        self.index
    }

    pub fn new(tag: &RedeemerTag, index: &BigNum) -> Self {
        Self {
            tag: *tag,
            index: *index,
        }
    }
}

/// Redeemer without the tag of index
/// This allows builder code to return partial redeemers
/// and then later have them placed in the right context
#[wasm_bindgen]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UntaggedRedeemer {
    data: PlutusData,
    ex_units: ExUnits,
}

#[wasm_bindgen]
impl UntaggedRedeemer {

    pub fn datum(&self) -> PlutusData {
        self.data.clone()
    }

    pub fn ex_units(&self) -> ExUnits {
        self.ex_units.clone()
    }

    pub fn new(data: &PlutusData, ex_units: &ExUnits) -> Self {
        Self {
            data: data.clone(),
            ex_units: ex_units.clone(),
        }
    }
}

/// A partial Plutus witness
/// It contains all the information needed to witness the Plutus script execution
/// except for the redeemer tag and index
/// Note: no datum is attached because only input script types have datums
#[wasm_bindgen]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PartialPlutusWitness {
    pub(crate) script: PlutusScript,
    pub(crate) untagged_redeemer: UntaggedRedeemer,
}

#[wasm_bindgen]
impl PartialPlutusWitness {
    pub fn new(
        script: &PlutusScript,
        untagged_redeemer: &UntaggedRedeemer
    ) -> Self {
        Self {
            script: script.clone(),
            untagged_redeemer: untagged_redeemer.clone(),
        }
    }

    pub fn script(&self) -> PlutusScript {
        self.script.clone()
    }

    pub fn untagged_redeemer(&self) -> UntaggedRedeemer {
        self.untagged_redeemer.clone()
    }
}

#[derive(Clone, Debug)]
pub enum InputAggregateWitnessData {
    // note: this struct may contains duplicates, but it will be de-duped later
    Vkeys(Vec<Vkey>),
    Bootstraps(Vec<(Vkey, ByronAddress)>),
    NativeScript(NativeScript, NativeScriptWitnessInfo),
    PlutusScript(PartialPlutusWitness, PlutusScriptWitnessInfo, Option<PlutusData>)
}

impl InputAggregateWitnessData {
    fn untagged_redeemer(&self) -> Option<UntaggedRedeemer> {
        match self {
            InputAggregateWitnessData::PlutusScript(witness, _, _) => {
                Some(witness.untagged_redeemer())
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
    pub(crate) vkeys: HashSet<Ed25519KeyHash>,
    pub(crate) bootstraps: HashSet<Ed25519KeyHash>,
    // note: no way to differentiate Plutus script from native script
    pub(crate) scripts: HashSet<ScriptHash>,
    pub(crate) plutus_data: HashSet<DataHash>,
    pub(crate) redeemers: HashSet<RedeemerWitnessKey>,
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

    pub fn add_bootstrap(&mut self, bootstrap: &BootstrapWitness) {
        self.add_bootstrap_key(&bootstrap.vkey());
    }
    pub fn add_bootstrap_key(&mut self, bootstrap: &Vkey) {
        self.add_bootstrap_key_hash(&bootstrap.public_key().hash());
    }
    pub fn add_bootstrap_key_hash(&mut self, hash: &Ed25519KeyHash) {
        self.bootstraps.insert(hash.clone());
    }

    pub fn add_native_script(&mut self, native_script: &NativeScript) {
        self.add_script_hash(&native_script.hash(ScriptHashNamespace::NativeScript));
    }
    pub fn add_script_hash(&mut self, script_hash: &ScriptHash) {
        self.scripts.insert(script_hash.clone());
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
        let bootstraps = self.bootstraps.iter().map(|key| format!("Legacy Bootstraps:{}", hex::encode(key.to_bytes()))).collect::<Vec<String>>().join(",");
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

    pub fn new() -> Self {
        // have to expose new so it's visible in WASM
        Self::default()
    }
}

/// In order to calculate the index from the sorted set, "add_*" methods in this builder
/// must be called along with the "add_*" methods in transaction builder.
#[wasm_bindgen]
#[derive(Clone, Default, Debug)]
pub struct RedeemerSetBuilder {
    // the set of inputs is an ordered set (according to the order defined on the type TxIn) -
    // this also is the order in which the elements of the set are indexed (lex order on the pair of TxId and Ix).
    // All inputs of a transaction are included in the set being indexed (not just the ones that point to a Plutus script UTxO)
    spend: BTreeMap<TransactionInput, Option<UntaggedRedeemer>>,

    // the set of policy IDs is ordered according to the order defined on PolicyID (lex).
    // The index of a PolicyID in this set of policy IDs is computed according to this order.
    // Note that at the use site, the set of policy IDs passed to indexof is the (unfiltered)
    // domain of the Value map in the mint field of the transaction.
    mint: BTreeMap<PolicyID, Option<UntaggedRedeemer>>,

    // the index of a reward account ract in the reward withdrawals map is the index of ract as a key in the (unfiltered) map.
    // The keys of the Wdrl map are arranged in the order defined on the RewardAcnt type, which is a lexicographical (abbrv. lex)
    // order on the pair of the Network and the Credential.
    reward: BTreeMap<RewardAddress, Option<UntaggedRedeemer>>,

    // certificates in the DCert list are indexed in the order in which they arranged in the (full, unfiltered)
    // list of certificates inside the transaction
    cert: Vec<Option<UntaggedRedeemer>>,
}

impl RedeemerSetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_spend(&mut self, result: &InputBuilderResult) {
        let untagged = {
            result.aggregate_witness.as_ref().and_then(|data| data.untagged_redeemer())
        };
        self.spend.insert(result.input.clone(), untagged);
    }

    pub fn add_mint(&mut self, result: &MintBuilderResult) {
        let untagged = {
            result.aggregate_witness.as_ref().and_then(|data| data.untagged_redeemer())
        };
        self.mint.insert(result.policy_id.clone(), untagged);
    }

    pub fn add_reward(&mut self, result: &WithdrawalBuilderResult) {
        let untagged = {
            result.aggregate_witness.as_ref().and_then(|data| data.untagged_redeemer())
        };
        self.reward.insert(result.address.clone(), untagged);
    }

    pub fn add_cert(&mut self, result: &CertificateBuilderResult) {
        let untagged = {
            result.aggregate_witness.as_ref().and_then(|data| data.untagged_redeemer())
        };
        self.cert.push(untagged);
    }

    pub fn build(&self) -> Redeemers {
        let mut redeemers = Vec::new();

        redeemers.append(&mut Self::tag_redeemer(&RedeemerTag::new_spend(), &Self::values(&self.spend)));
        redeemers.append(&mut Self::tag_redeemer(&RedeemerTag::new_mint(), &Self::values(&self.mint)));
        redeemers.append(&mut Self::tag_redeemer(&RedeemerTag::new_reward(), &Self::values(&self.reward)));
        redeemers.append(&mut Self::tag_redeemer(&RedeemerTag::new_cert(), &self.cert));

        Redeemers(redeemers)
    }

    fn values<K>(map: &BTreeMap<K, Option<UntaggedRedeemer>>) -> Vec<Option<UntaggedRedeemer>> {
        map.values().cloned().collect()
    }

    fn tag_redeemer(tag: &RedeemerTag, untagged_redeemers: &[Option<UntaggedRedeemer>]) -> Vec<Redeemer> {
        let mut result = Vec::new();

        for (index, value) in untagged_redeemers.iter().enumerate() {
            if let Some(untagged) = value {
                let redeemer = {
                    let index = index as u64;
                    Redeemer::new(tag, &index.into(), &untagged.data, &untagged.ex_units)
                };
                result.push(redeemer);
            }
        }

        result
    }
}

/// Builder de-duplicates witnesses as they are added
#[wasm_bindgen]
#[derive(Clone, Default, Debug)]
pub struct TransactionWitnessSetBuilder {
    // See Alonzo spec section 3.1 which defines the keys for these types
    vkeys: HashMap<Vkey, Vkeywitness>,
    bootstraps: HashMap<Vkey, BootstrapWitness>,
    native_scripts: HashMap<ScriptHash, NativeScript>,
    plutus_v1_scripts: HashMap<ScriptHash, PlutusV1Script>,
    plutus_v2_scripts: HashMap<ScriptHash, PlutusV2Script>,
    plutus_data: HashMap<DataHash, PlutusData>,
    redeemers: HashMap<RedeemerWitnessKey, Redeemer>,

    /// witnesses that need to be added for the build function to succeed
    /// this allows checking that witnesses are present at build time (instead of when submitting to a node)
    /// This is useful for APIs that can keep track of which witnesses will be required (like transaction builders)
    required_wits: RequiredWitnessSet,
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

    pub fn add_native_script(&mut self, native_script: &NativeScript) {
        self.native_scripts.insert(native_script.hash(ScriptHashNamespace::NativeScript), native_script.clone());
    }

    pub fn get_native_script(&self) -> NativeScripts {
        NativeScripts(self.native_scripts.clone().into_values().collect())
    }

    pub fn add_plutus_v1_script(&mut self, plutus_v1_script: &PlutusV1Script) {
        self.plutus_v1_scripts.insert(plutus_v1_script.hash(), plutus_v1_script.clone());
    }

    pub fn get_plutus_v1_script(&self) -> PlutusV1Scripts {
        PlutusV1Scripts(self.plutus_v1_scripts.clone().into_values().collect())
    }

    pub fn add_plutus_v2_script(&mut self, plutus_v2_script: &PlutusV2Script) {
        self.plutus_v2_scripts.insert(plutus_v2_script.hash(), plutus_v2_script.clone());
    }

    pub fn get_plutus_v2_script(&self) -> PlutusV2Scripts {
        PlutusV2Scripts(self.plutus_v2_scripts.clone().into_values().collect())
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

    fn add_fake_vkey_witnesses(&mut self, vkeys: &Vec<Vkey>) {
        let fake_sig = fake_raw_key_sig(0);
        for vkey in vkeys {
            let fake_vkey_witness = Vkeywitness::new(vkey, &fake_sig);
            self.add_vkey(&fake_vkey_witness);
        }
    }

    fn add_fake_bootstrap_witnesses(&mut self, entries: &Vec<(Vkey, ByronAddress)>) {
        let fake_key = fake_key_private(0);
        for entry in entries {
            // picking icarus over daedalus for fake witness generation shouldn't matter
            let bootstrap_wit = make_icarus_bootstrap_witness(
                &TransactionHash::from([0u8; TransactionHash::BYTE_COUNT]),
                &entry.1,
                &fake_key
            );
            self.add_bootstrap(&bootstrap_wit);
        }
    }

    // This method add fake vkeys to calculate the fee.
    // In order to prevent the fake keys get deduplicated when it is called more than once,
    // its index starts from the current amount of vkeys.
    // WARN: this function might fail at runtime when there are more than 255 witnesses,
    // however this is unrealistic because the limit of transaction size. (101 bytes each witness)
    fn add_fake_vkey_witnesses_by_num(&mut self, num: usize) {
        let vkeys: Vec<Vkey> = (0..num).into_iter().map(|i| Vkey::new(&fake_raw_key_public((i + self.vkeys.len()) as u8))).collect();
        self.add_fake_vkey_witnesses(&vkeys);
    }

    pub(crate) fn add_input_aggregate_real_witness_data(&mut self, data: &InputAggregateWitnessData) {
        match data {
            InputAggregateWitnessData::Vkeys(_vkeys) => {},
            InputAggregateWitnessData::Bootstraps(_witnesseses) => {}
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
    pub(crate) fn add_input_aggregate_fake_witness_data(&mut self, data: &InputAggregateWitnessData) {
        match data {
            InputAggregateWitnessData::Vkeys(vkeys) => self.add_fake_vkey_witnesses(vkeys),
            InputAggregateWitnessData::Bootstraps(witnesses) => self.add_fake_bootstrap_witnesses(witnesses),
            InputAggregateWitnessData::NativeScript(script, info) => {
                match info.0 {
                    NativeScriptWitnessInfoKind::Count(num) => self.add_fake_vkey_witnesses_by_num(num),
                    NativeScriptWitnessInfoKind::Vkeys(ref vkeys) => self.add_fake_vkey_witnesses(vkeys),
                    NativeScriptWitnessInfoKind::AssumeWorst => {
                        let num = script.get_required_signers().len();
                        self.add_fake_vkey_witnesses_by_num(num);
                    }
                }
            }
            InputAggregateWitnessData::PlutusScript(_witness, info, _option) => {
                self.add_plutus_witness_info(info);
            }
        }
    }

    fn add_plutus_witness_info(&mut self, info: &PlutusScriptWitnessInfo) {
        let known_signers = &info.known_signers.0;
        let missing_signers = &info.missing_signers.0;

        self.add_fake_vkey_witnesses(known_signers);
        self.add_fake_vkey_witnesses_by_num(
            // Exclude the missing signers whose key hashes are known.
            missing_signers.iter().filter(|hash| !self.required_wits.vkeys.contains(hash)).count()
        );
    }

    pub fn build(&self) -> TransactionWitnessSet {
        let mut result = TransactionWitnessSet::new();

        if !self.vkeys.is_empty() {
            result.set_vkeys(&Vkeywitnesses(self.vkeys.values().cloned().collect()));
        }
        if !self.bootstraps.is_empty() {
            result.set_bootstraps(&BootstrapWitnesses(self.bootstraps.values().cloned().collect()));
        }
        if !self.native_scripts.is_empty() {
            result.set_native_scripts(&NativeScripts(self.native_scripts.values().cloned().collect()));
        }
        if !self.plutus_v1_scripts.is_empty() {
            result.set_plutus_v1_scripts(&PlutusV1Scripts(self.plutus_v1_scripts.values().cloned().collect()));
        }
        if !self.plutus_v2_scripts.is_empty() {
            result.set_plutus_v2_scripts(&PlutusV2Scripts(self.plutus_v2_scripts.values().cloned().collect()));
        }
        if !self.plutus_data.is_empty() {
            result.set_plutus_data(&PlutusList {
                elems: self.plutus_data.values().cloned().collect(),
                definite_encoding: None,
            });
        }
        if !self.redeemers.is_empty() {
            result.set_redeemers(&Redeemers(self.redeemers.values().cloned().collect()));
        }

        result
    }

    pub fn remaining_wits(&self) -> RequiredWitnessSet {
        let mut remaining_wits = self.required_wits.clone();

        self.vkeys.keys().for_each(|key| { remaining_wits.vkeys.remove(&key.public_key().hash()); });
        self.bootstraps.keys().for_each(|key| { remaining_wits.bootstraps.remove(&key.public_key().hash()); });
        self.native_scripts.keys().for_each(|hash| { remaining_wits.scripts.remove(hash); });
        self.plutus_v1_scripts.keys().for_each(|hash| { remaining_wits.scripts.remove(hash); });
        self.plutus_v2_scripts.keys().for_each(|hash| { remaining_wits.scripts.remove(hash); });
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

#[derive(Clone, Debug)]
pub enum NativeScriptWitnessInfoKind {
    Count(usize),
    Vkeys(Vec<Vkey>),
    AssumeWorst,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct NativeScriptWitnessInfo(NativeScriptWitnessInfoKind);

impl NativeScriptWitnessInfo {
    /// Unsure which keys will sign, but you know the exact number to save on tx fee
    pub fn num_signatures(num: usize) -> NativeScriptWitnessInfo {
        NativeScriptWitnessInfo(NativeScriptWitnessInfoKind::Count(num))
    }

    /// This native script will be witnessed by exactly these keys
    pub fn vkeys(vkeys: &Vkeys) -> NativeScriptWitnessInfo {
        NativeScriptWitnessInfo(NativeScriptWitnessInfoKind::Vkeys(vkeys.0.clone()))
    }

    /// You don't know how many keys will sign, so the maximum possible case will be assumed
    pub fn assume_signature_count() -> NativeScriptWitnessInfo {
        NativeScriptWitnessInfo(NativeScriptWitnessInfoKind::AssumeWorst)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PlutusScriptWitnessInfo {
    pub(crate) missing_signers: RequiredSigners,
    pub(crate) known_signers: Vkeys,
}

impl PlutusScriptWitnessInfo {
    /// you can pass in an empty array if there are no required witnesses
    pub fn set_required_signers(known_signers: &Vkeys, missing_signers: &RequiredSigners) -> PlutusScriptWitnessInfo {
        Self {
            missing_signers: missing_signers.clone(),
            known_signers: known_signers.clone()
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::ledger::common::value::Value;

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
    fn test_redeemer_set_builder() {
        let mut builder = RedeemerSetBuilder::new();

        let data = {
            let witness = {
                let script = PlutusScriptEnum::from_v1(&PlutusV1Script::new(vec![0]));
                let untagged_redeemer = UntaggedRedeemer::new(&PlutusData::new_integer(&0u64.into()), &ExUnits::new(&to_bignum(10), &to_bignum(10)));
                PartialPlutusWitness::new(&PlutusScript(script), &untagged_redeemer)
            };
            let info = {
                let key = fake_raw_key_public(0);
                let mut missing_signers = Ed25519KeyHashes::new();
                missing_signers.add(&key.hash());
                PlutusScriptWitnessInfo::set_required_signers(&Vkeys::new(), &missing_signers)
            };
            InputAggregateWitnessData::PlutusScript(witness, info, None)
        };

        let address = Address::from_bech32(&"addr1qxeqxcja25k8q05evyngf4f88xn89asl54x2zg3ephgj26ndyt5qk02xmmras5pe9jz2c7tc93wu4c96rqwvg6e2v50qlpmx70").unwrap();

        let input_result = InputBuilderResult {
            input: TransactionInput { transaction_id: TransactionHash([1; 32]), index: 1u64.into() },
            utxo_info: TransactionOutput { address: address.clone(), amount: Value::zero(), datum_option: None, script_ref: None },
            aggregate_witness: None,
            required_wits: RequiredWitnessSet::new(),
        };

        builder.add_spend(&input_result);

        let input_result = InputBuilderResult {
            input: TransactionInput { transaction_id: TransactionHash([1; 32]), index: 0u64.into() },
            utxo_info: TransactionOutput { address: address.clone(), amount: Value::zero(), datum_option: None, script_ref: None },
            aggregate_witness: None,
            required_wits: RequiredWitnessSet::new(),
        };

        builder.add_spend(&input_result);

        let input_result = InputBuilderResult {
            input: TransactionInput { transaction_id: TransactionHash([0; 32]), index: 0u64.into() },
            utxo_info: TransactionOutput { address: address.clone(), amount: Value::zero(), datum_option: None, script_ref: None },
            aggregate_witness: Some(data.clone()),
            required_wits: RequiredWitnessSet::new(),
        };

        builder.add_spend(&input_result);

        let redeemers = builder.build();

        assert_eq!(redeemers.len(), 1);

        let spend_redeemer = &redeemers.0[0];

        assert_eq!(spend_redeemer.tag(), RedeemerTag::new_spend());
        assert_eq!(spend_redeemer.index(), BigNum::from(0u64));
    }

    #[test]
    fn test_add_fake_vkey_witnesses_by_num() {
        let mut builder = TransactionWitnessSetBuilder::new();
        builder.add_fake_vkey_witnesses_by_num(2);
        assert_eq!(builder.vkeys.len(), 2);
        builder.add_fake_vkey_witnesses_by_num(1);
        assert_eq!(builder.vkeys.len(), 3);
    }

    #[test]
    fn test_add_input_aggregate_witness_data() {
        let mut builder = TransactionWitnessSetBuilder::new();
        let data = {
            let witness = {
                let script = PlutusScriptEnum::from_v1(&PlutusV1Script::new(vec![0]));
                let untagged_redeemer = UntaggedRedeemer::new(&PlutusData::new_integer(&0u64.into()), &ExUnits::new(&to_bignum(10), &to_bignum(10)));
                PartialPlutusWitness::new(&PlutusScript(script), &untagged_redeemer)
            };
            let info = {
                let key = fake_raw_key_public(0);
                let mut missing_signers = Ed25519KeyHashes::new();
                missing_signers.add(&key.hash());
                PlutusScriptWitnessInfo::set_required_signers(&Vkeys::new(), &missing_signers)
            };
            InputAggregateWitnessData::PlutusScript(witness, info, None)
        };

        assert_eq!(builder.vkeys.len(), 0);
        builder.add_input_aggregate_fake_witness_data(&data);
        assert_eq!(builder.vkeys.len(), 1);
    }

    #[test]
    fn test_add_input_aggregate_witness_data_with_existing_key_hash() {
        let mut builder = TransactionWitnessSetBuilder::new();
        let key = fake_raw_key_public(0);
        let hash = key.hash();
        builder.required_wits.add_vkey_key_hash(&hash);

        let data = {
            let witness = {
                let script = PlutusScriptEnum::from_v1(&PlutusV1Script::new(vec![0]));
                let untagged_redeemer = UntaggedRedeemer::new(&PlutusData::new_integer(&0u64.into()), &ExUnits::new(&to_bignum(10), &to_bignum(10)));
                PartialPlutusWitness::new(&PlutusScript(script), &untagged_redeemer)
            };
            let info = {
                let mut missing_signers = Ed25519KeyHashes::new();
                missing_signers.add(&hash);
                PlutusScriptWitnessInfo::set_required_signers(&Vkeys::new(), &missing_signers)
            };
            InputAggregateWitnessData::PlutusScript(witness, info, None)
        };

        assert_eq!(builder.vkeys.len(), 0);
        builder.add_input_aggregate_fake_witness_data(&data);
        assert_eq!(builder.vkeys.len(), 0);
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
}
