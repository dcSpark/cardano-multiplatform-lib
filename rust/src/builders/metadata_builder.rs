use crate::*;

#[wasm_bindgen]
pub struct TxMetadataBuilderResult {
    pub(crate) metadata: GeneralTransactionMetadata,
}

#[wasm_bindgen]
pub struct TxMetadataBuilder {
    metadata: GeneralTransactionMetadata,
}

#[wasm_bindgen]
impl TxMetadataBuilder {
    pub fn new() -> Self {
        Self {
            metadata: GeneralTransactionMetadata::new(),
        }
    }

    /// Add a single metadatum using TransactionMetadatum object under `key` TranscactionMetadatumLabel
    pub fn add_metadatum(
        &mut self,
        key: &TransactionMetadatumLabel,
        value: &TransactionMetadatum,
    ) -> TxMetadataBuilderResult {
        self.metadata.insert(&key, &value);
        TxMetadataBuilderResult {
            metadata: self.metadata.clone(),
        }
    }

    /// Add a single JSON metadatum using a MetadataJsonSchema object and MetadataJsonScehma object.
    pub fn add_json_metadatum_with_schema(
        &mut self,
        key: &TransactionMetadatumLabel,
        val: String,
        schema: MetadataJsonSchema,
    ) -> Result<TxMetadataBuilderResult, JsError> {
        let metadatum = encode_json_str_to_metadatum(val, schema)?;
        self.metadata.insert(&key, &metadatum);
        Ok(TxMetadataBuilderResult {
            metadata: self.metadata.clone(),
        })
    }
}
