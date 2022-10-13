pub type Data = Vec<u8>;

pub type ScriptDataHash = Hash32;

pub type ScriptRef = Vec<u8>;

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct AlonzoTxOut(pub(crate) core::AlonzoTxOut);

#[wasm_bindgen]

impl AlonzoTxOut {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<AlonzoTxOut, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<AlonzoTxOut, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> Value {
        self.0.amount.clone().into()
    }

    pub fn datum_hash(&self) -> Hash32 {
        self.0.datum_hash.clone().into()
    }

    pub fn new(address: &Address, amount: &Value, datum_hash: &Hash32) -> Self {
        Self(core::AlonzoTxOut::new(address.clone().into(), amount.clone().into(), datum_hash.clone().into()))
    }
}

impl From<core::AlonzoTxOut> for AlonzoTxOut {
    fn from(native: core::AlonzoTxOut) -> Self {
        Self(native)
    }
}

impl From<AlonzoTxOut> for core::AlonzoTxOut {
    fn from(wasm: AlonzoTxOut) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct BabbageTxOut(pub(crate) core::BabbageTxOut);

#[wasm_bindgen]

impl BabbageTxOut {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<BabbageTxOut, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<BabbageTxOut, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn key_0(&self) -> Address {
        self.0.key_0.clone().into()
    }

    pub fn key_1(&self) -> Value {
        self.0.key_1.clone().into()
    }

    pub fn set_key_2(&mut self, key_2: &DatumOption) {
        self.0.key_2 = Some(key_2.clone().into())
    }

    pub fn key_2(&self) -> Option<DatumOption> {
        self.0.key_2.clone().map(std::convert::Into::into)
    }

    pub fn set_key_3(&mut self, key_3: ScriptRef) {
        self.0.key_3 = Some(key_3.into())
    }

    pub fn key_3(&self) -> Option<ScriptRef> {
        self.0.key_3.clone()
    }

    pub fn new(key_0: &Address, key_1: &Value) -> Self {
        Self(core::BabbageTxOut::new(key_0.clone().into(), key_1.clone().into()))
    }
}

impl From<core::BabbageTxOut> for BabbageTxOut {
    fn from(native: core::BabbageTxOut) -> Self {
        Self(native)
    }
}

impl From<BabbageTxOut> for core::BabbageTxOut {
    fn from(wasm: BabbageTxOut) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum DatumOptionKind {
    DatumOption0,
    DatumOption1,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct DatumOption(pub(crate) core::DatumOption);

#[wasm_bindgen]

impl DatumOption {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<DatumOption, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<DatumOption, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_datum_option0(hash32: &Hash32) -> Self {
        Self(core::DatumOption::new_datum_option0(hash32.clone().into()))
    }

    pub fn new_datum_option1(data: Data) -> Self {
        Self(core::DatumOption::new_datum_option1(data.into()))
    }

    pub fn kind(&self) -> DatumOptionKind {
        match &self.0 {
            core::DatumOption::DatumOption0(_) => DatumOptionKind::DatumOption0,
            core::DatumOption::DatumOption1(_) => DatumOptionKind::DatumOption1,
        }
    }

    pub fn as_datum_option0(&self) -> Option<DatumOption0> {
        match &self.0 {
            core::DatumOption::DatumOption0(datum_option0) => Some(datum_option0.clone().into()),
            _ => None,
        }
    }

    pub fn as_datum_option1(&self) -> Option<DatumOption1> {
        match &self.0 {
            core::DatumOption::DatumOption1(datum_option1) => Some(datum_option1.clone().into()),
            _ => None,
        }
    }
}

impl From<core::DatumOption> for DatumOption {
    fn from(native: core::DatumOption) -> Self {
        Self(native)
    }
}

impl From<DatumOption> for core::DatumOption {
    fn from(wasm: DatumOption) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct InvalidBefore(pub(crate) core::InvalidBefore);

#[wasm_bindgen]

impl InvalidBefore {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<InvalidBefore, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<InvalidBefore, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_1(&self) -> u64 {
        self.0.index_1
    }

    pub fn new(index_1: u64) -> Self {
        Self(core::InvalidBefore::new(index_1))
    }
}

impl From<core::InvalidBefore> for InvalidBefore {
    fn from(native: core::InvalidBefore) -> Self {
        Self(native)
    }
}

impl From<InvalidBefore> for core::InvalidBefore {
    fn from(wasm: InvalidBefore) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct InvalidHereafter(pub(crate) core::InvalidHereafter);

#[wasm_bindgen]

impl InvalidHereafter {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<InvalidHereafter, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<InvalidHereafter, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn index_1(&self) -> u64 {
        self.0.index_1
    }

    pub fn new(index_1: u64) -> Self {
        Self(core::InvalidHereafter::new(index_1))
    }
}

impl From<core::InvalidHereafter> for InvalidHereafter {
    fn from(native: core::InvalidHereafter) -> Self {
        Self(native)
    }
}

impl From<InvalidHereafter> for core::InvalidHereafter {
    fn from(wasm: InvalidHereafter) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum NativeScriptKind {
    ScriptPubkey,
    ScriptAll,
    ScriptAny,
    ScriptNOfK,
    InvalidBefore,
    InvalidHereafter,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct NativeScript(pub(crate) core::NativeScript);

#[wasm_bindgen]

impl NativeScript {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<NativeScript, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<NativeScript, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_script_pubkey(addr_keyhash: &AddrKeyhash) -> Self {
        Self(core::NativeScript::new_script_pubkey(addr_keyhash.clone().into()))
    }

    pub fn new_script_all(native_scripts: &NativeScripts) -> Self {
        Self(core::NativeScript::new_script_all(native_scripts.clone().into()))
    }

    pub fn new_script_any(native_scripts: &NativeScripts) -> Self {
        Self(core::NativeScript::new_script_any(native_scripts.clone().into()))
    }

    pub fn new_script_n_of_k(n: u64, native_scripts: &NativeScripts) -> Self {
        Self(core::NativeScript::new_script_n_of_k(n, native_scripts.clone().into()))
    }

    pub fn new_invalid_before(index_1: u64) -> Self {
        Self(core::NativeScript::new_invalid_before(index_1))
    }

    pub fn new_invalid_hereafter(index_1: u64) -> Self {
        Self(core::NativeScript::new_invalid_hereafter(index_1))
    }

    pub fn kind(&self) -> NativeScriptKind {
        match &self.0 {
            core::NativeScript::ScriptPubkey(_) => NativeScriptKind::ScriptPubkey,
            core::NativeScript::ScriptAll(_) => NativeScriptKind::ScriptAll,
            core::NativeScript::ScriptAny(_) => NativeScriptKind::ScriptAny,
            core::NativeScript::ScriptNOfK(_) => NativeScriptKind::ScriptNOfK,
            core::NativeScript::InvalidBefore(_) => NativeScriptKind::InvalidBefore,
            core::NativeScript::InvalidHereafter(_) => NativeScriptKind::InvalidHereafter,
        }
    }

    pub fn as_script_pubkey(&self) -> Option<ScriptPubkey> {
        match &self.0 {
            core::NativeScript::ScriptPubkey(script_pubkey) => Some(script_pubkey.clone().into()),
            _ => None,
        }
    }

    pub fn as_script_all(&self) -> Option<ScriptAll> {
        match &self.0 {
            core::NativeScript::ScriptAll(script_all) => Some(script_all.clone().into()),
            _ => None,
        }
    }

    pub fn as_script_any(&self) -> Option<ScriptAny> {
        match &self.0 {
            core::NativeScript::ScriptAny(script_any) => Some(script_any.clone().into()),
            _ => None,
        }
    }

    pub fn as_script_n_of_k(&self) -> Option<ScriptNOfK> {
        match &self.0 {
            core::NativeScript::ScriptNOfK(script_n_of_k) => Some(script_n_of_k.clone().into()),
            _ => None,
        }
    }

    pub fn as_invalid_before(&self) -> Option<InvalidBefore> {
        match &self.0 {
            core::NativeScript::InvalidBefore(invalid_before) => Some(invalid_before.clone().into()),
            _ => None,
        }
    }

    pub fn as_invalid_hereafter(&self) -> Option<InvalidHereafter> {
        match &self.0 {
            core::NativeScript::InvalidHereafter(invalid_hereafter) => Some(invalid_hereafter.clone().into()),
            _ => None,
        }
    }
}

impl From<core::NativeScript> for NativeScript {
    fn from(native: core::NativeScript) -> Self {
        Self(native)
    }
}

impl From<NativeScript> for core::NativeScript {
    fn from(wasm: NativeScript) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct RequiredSigners(pub(crate) core::RequiredSigners);

#[wasm_bindgen]

impl RequiredSigners {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<RequiredSigners, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<RequiredSigners, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn addr_keyhash(&self) -> AddrKeyhash {
        self.0.addr_keyhash.clone().into()
    }

    pub fn new(addr_keyhash: &AddrKeyhash) -> Self {
        Self(core::RequiredSigners::new(addr_keyhash.clone().into()))
    }
}

impl From<core::RequiredSigners> for RequiredSigners {
    fn from(native: core::RequiredSigners) -> Self {
        Self(native)
    }
}

impl From<RequiredSigners> for core::RequiredSigners {
    fn from(wasm: RequiredSigners) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum ScriptKind {
    Script0,
    Script1,
    Script2,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Script(pub(crate) core::Script);

#[wasm_bindgen]

impl Script {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Script, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Script, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_script0(native_script: &NativeScript) -> Self {
        Self(core::Script::new_script0(native_script.clone().into()))
    }

    pub fn new_script1(plutus_v1_script: PlutusV1Script) -> Self {
        Self(core::Script::new_script1(plutus_v1_script))
    }

    pub fn new_script2(plutus_v2_script: PlutusV2Script) -> Self {
        Self(core::Script::new_script2(plutus_v2_script))
    }

    pub fn kind(&self) -> ScriptKind {
        match &self.0 {
            core::Script::Script0(_) => ScriptKind::Script0,
            core::Script::Script1(_) => ScriptKind::Script1,
            core::Script::Script2(_) => ScriptKind::Script2,
        }
    }

    pub fn as_script0(&self) -> Option<Script0> {
        match &self.0 {
            core::Script::Script0(script0) => Some(script0.clone().into()),
            _ => None,
        }
    }

    pub fn as_script1(&self) -> Option<Script1> {
        match &self.0 {
            core::Script::Script1(script1) => Some(script1.clone().into()),
            _ => None,
        }
    }

    pub fn as_script2(&self) -> Option<Script2> {
        match &self.0 {
            core::Script::Script2(script2) => Some(script2.clone().into()),
            _ => None,
        }
    }
}

impl From<core::Script> for Script {
    fn from(native: core::Script) -> Self {
        Self(native)
    }
}

impl From<Script> for core::Script {
    fn from(wasm: Script) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ScriptAll(pub(crate) core::ScriptAll);

#[wasm_bindgen]

impl ScriptAll {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ScriptAll, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ScriptAll, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn native_scripts(&self) -> NativeScripts {
        self.0.native_scripts.clone().into()
    }

    pub fn new(native_scripts: &NativeScripts) -> Self {
        Self(core::ScriptAll::new(native_scripts.clone().into()))
    }
}

impl From<core::ScriptAll> for ScriptAll {
    fn from(native: core::ScriptAll) -> Self {
        Self(native)
    }
}

impl From<ScriptAll> for core::ScriptAll {
    fn from(wasm: ScriptAll) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ScriptAny(pub(crate) core::ScriptAny);

#[wasm_bindgen]

impl ScriptAny {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ScriptAny, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ScriptAny, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn native_scripts(&self) -> NativeScripts {
        self.0.native_scripts.clone().into()
    }

    pub fn new(native_scripts: &NativeScripts) -> Self {
        Self(core::ScriptAny::new(native_scripts.clone().into()))
    }
}

impl From<core::ScriptAny> for ScriptAny {
    fn from(native: core::ScriptAny) -> Self {
        Self(native)
    }
}

impl From<ScriptAny> for core::ScriptAny {
    fn from(wasm: ScriptAny) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ScriptNOfK(pub(crate) core::ScriptNOfK);

#[wasm_bindgen]

impl ScriptNOfK {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ScriptNOfK, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ScriptNOfK, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn n(&self) -> u64 {
        self.0.n
    }

    pub fn native_scripts(&self) -> NativeScripts {
        self.0.native_scripts.clone().into()
    }

    pub fn new(n: u64, native_scripts: &NativeScripts) -> Self {
        Self(core::ScriptNOfK::new(n, native_scripts.clone().into()))
    }
}

impl From<core::ScriptNOfK> for ScriptNOfK {
    fn from(native: core::ScriptNOfK) -> Self {
        Self(native)
    }
}

impl From<ScriptNOfK> for core::ScriptNOfK {
    fn from(wasm: ScriptNOfK) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ScriptPubkey(pub(crate) core::ScriptPubkey);

#[wasm_bindgen]

impl ScriptPubkey {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ScriptPubkey, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ScriptPubkey, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn addr_keyhash(&self) -> AddrKeyhash {
        self.0.addr_keyhash.clone().into()
    }

    pub fn new(addr_keyhash: &AddrKeyhash) -> Self {
        Self(core::ScriptPubkey::new(addr_keyhash.clone().into()))
    }
}

impl From<core::ScriptPubkey> for ScriptPubkey {
    fn from(native: core::ScriptPubkey) -> Self {
        Self(native)
    }
}

impl From<ScriptPubkey> for core::ScriptPubkey {
    fn from(wasm: ScriptPubkey) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ShelleyTxOut(pub(crate) core::ShelleyTxOut);

#[wasm_bindgen]

impl ShelleyTxOut {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ShelleyTxOut, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ShelleyTxOut, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn amount(&self) -> Value {
        self.0.amount.clone().into()
    }

    pub fn new(address: &Address, amount: &Value) -> Self {
        Self(core::ShelleyTxOut::new(address.clone().into(), amount.clone().into()))
    }
}

impl From<core::ShelleyTxOut> for ShelleyTxOut {
    fn from(native: core::ShelleyTxOut) -> Self {
        Self(native)
    }
}

impl From<ShelleyTxOut> for core::ShelleyTxOut {
    fn from(wasm: ShelleyTxOut) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct Transaction(pub(crate) core::Transaction);

#[wasm_bindgen]

impl Transaction {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Transaction, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<Transaction, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn transaction_body(&self) -> TransactionBody {
        self.0.transaction_body.clone().into()
    }

    pub fn transaction_witness_set(&self) -> TransactionWitnessSet {
        self.0.transaction_witness_set.clone().into()
    }

    pub fn index_2(&self) -> bool {
        self.0.index_2
    }

    pub fn auxiliary_data(&self) -> Option<AuxiliaryData> {
        self.0.auxiliary_data.clone().map(std::convert::Into::into)
    }

    pub fn new(transaction_body: &TransactionBody, transaction_witness_set: &TransactionWitnessSet, index_2: bool, auxiliary_data: Option<AuxiliaryData>) -> Self {
        Self(core::Transaction::new(transaction_body.clone().into(), transaction_witness_set.clone().into(), index_2, auxiliary_data.map(Into::into)))
    }
}

impl From<core::Transaction> for Transaction {
    fn from(native: core::Transaction) -> Self {
        Self(native)
    }
}

impl From<Transaction> for core::Transaction {
    fn from(wasm: Transaction) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct TransactionBody(pub(crate) core::TransactionBody);

#[wasm_bindgen]

impl TransactionBody {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionBody, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<TransactionBody, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn key_0(&self) -> TransactionInputs {
        self.0.key_0.clone().into()
    }

    pub fn key_1(&self) -> TransactionOutputs {
        self.0.key_1.clone().into()
    }

    pub fn key_2(&self) -> Coin {
        self.0.key_2
    }

    pub fn set_key_3(&mut self, key_3: u64) {
        self.0.key_3 = Some(key_3)
    }

    pub fn key_3(&self) -> Option<u64> {
        self.0.key_3
    }

    pub fn set_key_4(&mut self, key_4: &Certificates) {
        self.0.key_4 = Some(key_4.clone().into())
    }

    pub fn key_4(&self) -> Option<Certificates> {
        self.0.key_4.clone().map(std::convert::Into::into)
    }

    pub fn set_key_5(&mut self, key_5: Withdrawals) {
        self.0.key_5 = Some(key_5.clone().into())
    }

    pub fn key_5(&self) -> Option<Withdrawals> {
        self.0.key_5.clone().map(std::convert::Into::into)
    }

    pub fn set_key_6(&mut self, key_6: &Update) {
        self.0.key_6 = Some(key_6.clone().into())
    }

    pub fn key_6(&self) -> Option<Update> {
        self.0.key_6.clone().map(std::convert::Into::into)
    }

    pub fn set_key_7(&mut self, key_7: &AuxiliaryDataHash) {
        self.0.key_7 = Some(key_7.clone().into())
    }

    pub fn key_7(&self) -> Option<AuxiliaryDataHash> {
        self.0.key_7.clone().map(std::convert::Into::into)
    }

    pub fn set_key_8(&mut self, key_8: u64) {
        self.0.key_8 = Some(key_8)
    }

    pub fn key_8(&self) -> Option<u64> {
        self.0.key_8
    }

    pub fn set_key_9(&mut self, key_9: Mint) {
        self.0.key_9 = Some(key_9.clone().into())
    }

    pub fn key_9(&self) -> Option<Mint> {
        self.0.key_9.clone().map(std::convert::Into::into)
    }

    pub fn set_key_11(&mut self, key_11: &ScriptDataHash) {
        self.0.key_11 = Some(key_11.clone().into())
    }

    pub fn key_11(&self) -> Option<ScriptDataHash> {
        self.0.key_11.clone().map(std::convert::Into::into)
    }

    pub fn set_key_13(&mut self, key_13: &TransactionInputs) {
        self.0.key_13 = Some(key_13.clone().into())
    }

    pub fn key_13(&self) -> Option<TransactionInputs> {
        self.0.key_13.clone().map(std::convert::Into::into)
    }

    pub fn set_key_14(&mut self, key_14: &RequiredSigners) {
        self.0.key_14 = Some(key_14.clone().into())
    }

    pub fn key_14(&self) -> Option<RequiredSigners> {
        self.0.key_14.clone().map(std::convert::Into::into)
    }

    pub fn set_key_15(&mut self, key_15: &NetworkId) {
        self.0.key_15 = Some(key_15.clone().into())
    }

    pub fn key_15(&self) -> Option<NetworkId> {
        self.0.key_15.clone().map(std::convert::Into::into)
    }

    pub fn set_key_16(&mut self, key_16: &TransactionOutput) {
        self.0.key_16 = Some(key_16.clone().into())
    }

    pub fn key_16(&self) -> Option<TransactionOutput> {
        self.0.key_16.clone().map(std::convert::Into::into)
    }

    pub fn set_key_17(&mut self, key_17: Coin) {
        self.0.key_17 = Some(key_17)
    }

    pub fn key_17(&self) -> Option<Coin> {
        self.0.key_17
    }

    pub fn set_key_18(&mut self, key_18: &TransactionInputs) {
        self.0.key_18 = Some(key_18.clone().into())
    }

    pub fn key_18(&self) -> Option<TransactionInputs> {
        self.0.key_18.clone().map(std::convert::Into::into)
    }

    pub fn new(key_0: &TransactionInputs, key_1: &TransactionOutputs, key_2: Coin) -> Self {
        Self(core::TransactionBody::new(key_0.clone().into(), key_1.clone().into(), key_2))
    }
}

impl From<core::TransactionBody> for TransactionBody {
    fn from(native: core::TransactionBody) -> Self {
        Self(native)
    }
}

impl From<TransactionBody> for core::TransactionBody {
    fn from(wasm: TransactionBody) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct TransactionInput(pub(crate) core::TransactionInput);

#[wasm_bindgen]

impl TransactionInput {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionInput, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<TransactionInput, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn transaction_id(&self) -> Hash32 {
        self.0.transaction_id.clone().into()
    }

    pub fn index(&self) -> u64 {
        self.0.index
    }

    pub fn new(transaction_id: &Hash32, index: u64) -> Self {
        Self(core::TransactionInput::new(transaction_id.clone().into(), index))
    }
}

impl From<core::TransactionInput> for TransactionInput {
    fn from(native: core::TransactionInput) -> Self {
        Self(native)
    }
}

impl From<TransactionInput> for core::TransactionInput {
    fn from(wasm: TransactionInput) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum TransactionOutputKind {
    ShelleyTxOut,
    AlonzoTxOut,
    BabbageTxOut,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct TransactionOutput(pub(crate) core::TransactionOutput);

#[wasm_bindgen]

impl TransactionOutput {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionOutput, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<TransactionOutput, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_shelley_tx_out(shelley_tx_out: &ShelleyTxOut) -> Self {
        Self(core::TransactionOutput::new_shelley_tx_out(shelley_tx_out.clone().into()))
    }

    pub fn new_alonzo_tx_out(alonzo_tx_out: &AlonzoTxOut) -> Self {
        Self(core::TransactionOutput::new_alonzo_tx_out(alonzo_tx_out.clone().into()))
    }

    pub fn new_babbage_tx_out(babbage_tx_out: &BabbageTxOut) -> Self {
        Self(core::TransactionOutput::new_babbage_tx_out(babbage_tx_out.clone().into()))
    }

    pub fn kind(&self) -> TransactionOutputKind {
        match &self.0 {
            core::TransactionOutput::ShelleyTxOut(_) => TransactionOutputKind::ShelleyTxOut,
            core::TransactionOutput::AlonzoTxOut(_) => TransactionOutputKind::AlonzoTxOut,
            core::TransactionOutput::BabbageTxOut(_) => TransactionOutputKind::BabbageTxOut,
        }
    }

    pub fn as_shelley_tx_out(&self) -> Option<ShelleyTxOut> {
        match &self.0 {
            core::TransactionOutput::ShelleyTxOut(shelley_tx_out) => Some(shelley_tx_out.clone().into()),
            _ => None,
        }
    }

    pub fn as_alonzo_tx_out(&self) -> Option<AlonzoTxOut> {
        match &self.0 {
            core::TransactionOutput::AlonzoTxOut(alonzo_tx_out) => Some(alonzo_tx_out.clone().into()),
            _ => None,
        }
    }

    pub fn as_babbage_tx_out(&self) -> Option<BabbageTxOut> {
        match &self.0 {
            core::TransactionOutput::BabbageTxOut(babbage_tx_out) => Some(babbage_tx_out.clone().into()),
            _ => None,
        }
    }
}

impl From<core::TransactionOutput> for TransactionOutput {
    fn from(native: core::TransactionOutput) -> Self {
        Self(native)
    }
}

impl From<TransactionOutput> for core::TransactionOutput {
    fn from(wasm: TransactionOutput) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct TransactionWitnessSet(pub(crate) core::TransactionWitnessSet);

#[wasm_bindgen]

impl TransactionWitnessSet {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionWitnessSet, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<TransactionWitnessSet, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn set_key_0(&mut self, key_0: &Vkeywitnesss) {
        self.0.key_0 = Some(key_0.clone().into())
    }

    pub fn key_0(&self) -> Option<Vkeywitnesss> {
        self.0.key_0.clone().map(std::convert::Into::into)
    }

    pub fn set_key_1(&mut self, key_1: &NativeScripts) {
        self.0.key_1 = Some(key_1.clone().into())
    }

    pub fn key_1(&self) -> Option<NativeScripts> {
        self.0.key_1.clone().map(std::convert::Into::into)
    }

    pub fn set_key_2(&mut self, key_2: &BootstrapWitnesss) {
        self.0.key_2 = Some(key_2.clone().into())
    }

    pub fn key_2(&self) -> Option<BootstrapWitnesss> {
        self.0.key_2.clone().map(std::convert::Into::into)
    }

    pub fn set_key_3(&mut self, key_3: &PlutusV1Scripts) {
        self.0.key_3 = Some(key_3.clone().into())
    }

    pub fn key_3(&self) -> Option<PlutusV1Scripts> {
        self.0.key_3.clone().map(std::convert::Into::into)
    }

    pub fn set_key_4(&mut self, key_4: &PlutusDatas) {
        self.0.key_4 = Some(key_4.clone().into())
    }

    pub fn key_4(&self) -> Option<PlutusDatas> {
        self.0.key_4.clone().map(std::convert::Into::into)
    }

    pub fn set_key_5(&mut self, key_5: &Redeemers) {
        self.0.key_5 = Some(key_5.clone().into())
    }

    pub fn key_5(&self) -> Option<Redeemers> {
        self.0.key_5.clone().map(std::convert::Into::into)
    }

    pub fn set_key_6(&mut self, key_6: &PlutusV2Scripts) {
        self.0.key_6 = Some(key_6.clone().into())
    }

    pub fn key_6(&self) -> Option<PlutusV2Scripts> {
        self.0.key_6.clone().map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(core::TransactionWitnessSet::new())
    }
}

impl From<core::TransactionWitnessSet> for TransactionWitnessSet {
    fn from(native: core::TransactionWitnessSet) -> Self {
        Self(native)
    }
}

impl From<TransactionWitnessSet> for core::TransactionWitnessSet {
    fn from(wasm: TransactionWitnessSet) -> Self {
        wasm.0
    }
}

use super::*;