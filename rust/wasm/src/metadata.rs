pub type ShelleyAuxData = MapTransactionMetadatumLabelToTransactionMetadatum;

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct AlonzoAuxData(pub(crate) core::AlonzoAuxData);

#[wasm_bindgen]

impl AlonzoAuxData {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<AlonzoAuxData, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<AlonzoAuxData, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn set_key_0(&mut self, key_0: Metadata) {
        self.0.key_0 = Some(key_0.clone().into())
    }

    pub fn key_0(&self) -> Option<Metadata> {
        self.0.key_0.clone().map(std::convert::Into::into)
    }

    pub fn set_key_1(&mut self, key_1: &NativeScripts) {
        self.0.key_1 = Some(key_1.clone().into())
    }

    pub fn key_1(&self) -> Option<NativeScripts> {
        self.0.key_1.clone().map(std::convert::Into::into)
    }

    pub fn set_key_2(&mut self, key_2: &PlutusV1Scripts) {
        self.0.key_2 = Some(key_2.clone().into())
    }

    pub fn key_2(&self) -> Option<PlutusV1Scripts> {
        self.0.key_2.clone().map(std::convert::Into::into)
    }

    pub fn set_key_3(&mut self, key_3: &PlutusV2Scripts) {
        self.0.key_3 = Some(key_3.clone().into())
    }

    pub fn key_3(&self) -> Option<PlutusV2Scripts> {
        self.0.key_3.clone().map(std::convert::Into::into)
    }

    pub fn new() -> Self {
        Self(core::AlonzoAuxData::new())
    }
}

impl From<core::AlonzoAuxData> for AlonzoAuxData {
    fn from(native: core::AlonzoAuxData) -> Self {
        Self(native)
    }
}

impl From<AlonzoAuxData> for core::AlonzoAuxData {
    fn from(wasm: AlonzoAuxData) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum AuxiliaryDataKind {
    ShelleyAuxData,
    ShelleyMaAuxData,
    AlonzoAuxData,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct AuxiliaryData(pub(crate) core::AuxiliaryData);

#[wasm_bindgen]

impl AuxiliaryData {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<AuxiliaryData, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<AuxiliaryData, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_shelley_aux_data(shelley_aux_data: ShelleyAuxData) -> Self {
        Self(core::AuxiliaryData::new_shelley_aux_data(shelley_aux_data.clone().into()))
    }

    pub fn new_shelley_ma_aux_data(shelley_ma_aux_data: &ShelleyMaAuxData) -> Self {
        Self(core::AuxiliaryData::new_shelley_ma_aux_data(shelley_ma_aux_data.clone().into()))
    }

    pub fn new_alonzo_aux_data(alonzo_aux_data: &AlonzoAuxData) -> Self {
        Self(core::AuxiliaryData::new_alonzo_aux_data(alonzo_aux_data.clone().into()))
    }

    pub fn kind(&self) -> AuxiliaryDataKind {
        match &self.0 {
            core::AuxiliaryData::ShelleyAuxData{ .. } => AuxiliaryDataKind::ShelleyAuxData,
            core::AuxiliaryData::ShelleyMaAuxData(_) => AuxiliaryDataKind::ShelleyMaAuxData,
            core::AuxiliaryData::AlonzoAuxData(_) => AuxiliaryDataKind::AlonzoAuxData,
        }
    }

    pub fn as_shelley_aux_data(&self) -> Option<ShelleyAuxData> {
        match &self.0 {
            core::AuxiliaryData::ShelleyAuxData{ shelley_aux_data, .. } => Some(shelley_aux_data.clone().into()),
            _ => None,
        }
    }

    pub fn as_shelley_ma_aux_data(&self) -> Option<ShelleyMaAuxData> {
        match &self.0 {
            core::AuxiliaryData::ShelleyMaAuxData(shelley_ma_aux_data) => Some(shelley_ma_aux_data.clone().into()),
            _ => None,
        }
    }

    pub fn as_alonzo_aux_data(&self) -> Option<AlonzoAuxData> {
        match &self.0 {
            core::AuxiliaryData::AlonzoAuxData(alonzo_aux_data) => Some(alonzo_aux_data.clone().into()),
            _ => None,
        }
    }
}

impl From<core::AuxiliaryData> for AuxiliaryData {
    fn from(native: core::AuxiliaryData) -> Self {
        Self(native)
    }
}

impl From<AuxiliaryData> for core::AuxiliaryData {
    fn from(wasm: AuxiliaryData) -> Self {
        wasm.0
    }
}

type Metadata = MapTransactionMetadatumLabelToTransactionMetadatum;

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct ShelleyMaAuxData(pub(crate) core::ShelleyMaAuxData);

#[wasm_bindgen]

impl ShelleyMaAuxData {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<ShelleyMaAuxData, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<ShelleyMaAuxData, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn transaction_metadata(&self) -> Metadata {
        self.0.transaction_metadata.clone().into()
    }

    pub fn auxiliary_scripts(&self) -> NativeScripts {
        self.0.auxiliary_scripts.clone().into()
    }

    pub fn new(transaction_metadata: Metadata, auxiliary_scripts: &NativeScripts) -> Self {
        Self(core::ShelleyMaAuxData::new(transaction_metadata.clone().into(), auxiliary_scripts.clone().into()))
    }
}

impl From<core::ShelleyMaAuxData> for ShelleyMaAuxData {
    fn from(native: core::ShelleyMaAuxData) -> Self {
        Self(native)
    }
}

impl From<ShelleyMaAuxData> for core::ShelleyMaAuxData {
    fn from(wasm: ShelleyMaAuxData) -> Self {
        wasm.0
    }
}

#[wasm_bindgen]

pub enum TransactionMetadatumKind {
    MapTransactionMetadatumToTransactionMetadatum,
    ArrTransactionMetadatum,
    Int,
    Bytes,
    Text,
}

#[wasm_bindgen]

#[derive(Clone, Debug)]
pub struct TransactionMetadatum(pub(crate) core::TransactionMetadatum);

#[wasm_bindgen]

impl TransactionMetadatum {
    pub fn to_bytes(&self, force_canonical: bool) -> Vec<u8> {
        use core::serialization::ToBytes;
        ToBytes::to_bytes(&self.0, force_canonical)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<TransactionMetadatum, JsValue> {
        use core::prelude::FromBytes;
        FromBytes::from_bytes(data).map(Self).map_err(|e| JsValue::from_str(&format!("from_bytes: {}", e)))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.0).map_err(|e| JsValue::from_str(&format!("to_json: {}", e)))
    }

    pub fn to_json_value(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(|e| JsValue::from_str(&format!("to_js_value: {}", e)))
    }

    pub fn from_json(json: &str) -> Result<TransactionMetadatum, JsValue> {
        serde_json::from_str(json).map(Self).map_err(|e| JsValue::from_str(&format!("from_json: {}", e)))
    }

    pub fn new_map_transaction_metadatum_to_transaction_metadatum(map_transaction_metadatum_to_transaction_metadatum: &MapTransactionMetadatumToTransactionMetadatum) -> Self {
        Self(core::TransactionMetadatum::new_map_transaction_metadatum_to_transaction_metadatum(map_transaction_metadatum_to_transaction_metadatum.clone().into()))
    }

    pub fn new_arr_transaction_metadatum(arr_transaction_metadatum: &TransactionMetadatums) -> Self {
        Self(core::TransactionMetadatum::new_arr_transaction_metadatum(arr_transaction_metadatum.clone().into()))
    }

    pub fn new_int(int: &Int) -> Self {
        Self(core::TransactionMetadatum::new_int(int.clone().into()))
    }

    pub fn new_bytes(bytes: Vec<u8>) -> Self {
        Self(core::TransactionMetadatum::new_bytes(bytes))
    }

    pub fn new_text(text: String) -> Self {
        Self(core::TransactionMetadatum::new_text(text))
    }

    pub fn kind(&self) -> TransactionMetadatumKind {
        match &self.0 {
            core::TransactionMetadatum::MapTransactionMetadatumToTransactionMetadatum{ .. } => TransactionMetadatumKind::MapTransactionMetadatumToTransactionMetadatum,
            core::TransactionMetadatum::ArrTransactionMetadatum{ .. } => TransactionMetadatumKind::ArrTransactionMetadatum,
            core::TransactionMetadatum::Int(_) => TransactionMetadatumKind::Int,
            core::TransactionMetadatum::Bytes{ .. } => TransactionMetadatumKind::Bytes,
            core::TransactionMetadatum::Text{ .. } => TransactionMetadatumKind::Text,
        }
    }

    pub fn as_map_transaction_metadatum_to_transaction_metadatum(&self) -> Option<MapTransactionMetadatumToTransactionMetadatum> {
        match &self.0 {
            core::TransactionMetadatum::MapTransactionMetadatumToTransactionMetadatum{ map_transaction_metadatum_to_transaction_metadatum, .. } => Some(map_transaction_metadatum_to_transaction_metadatum.clone().into()),
            _ => None,
        }
    }

    pub fn as_arr_transaction_metadatum(&self) -> Option<TransactionMetadatums> {
        match &self.0 {
            core::TransactionMetadatum::ArrTransactionMetadatum{ arr_transaction_metadatum, .. } => Some(arr_transaction_metadatum.clone().into()),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<Int> {
        match &self.0 {
            core::TransactionMetadatum::Int(int) => Some(int.clone().into()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        match &self.0 {
            core::TransactionMetadatum::Bytes{ bytes, .. } => Some(bytes.clone()),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<String> {
        match &self.0 {
            core::TransactionMetadatum::Text{ text, .. } => Some(text.clone()),
            _ => None,
        }
    }
}

impl From<core::TransactionMetadatum> for TransactionMetadatum {
    fn from(native: core::TransactionMetadatum) -> Self {
        Self(native)
    }
}

impl From<TransactionMetadatum> for core::TransactionMetadatum {
    fn from(wasm: TransactionMetadatum) -> Self {
        wasm.0
    }
}

use super::*;