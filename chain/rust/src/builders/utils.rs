//use crate::auxdata::{AuxiliaryData, GeneralTransactionMetadata};

// impl GeneralTransactionMetadata {
//     pub fn add(&mut self, other: &GeneralTransactionMetadata) {
//         for idx in 0..other.len() {
//             let key = other.keys().get(idx);
//             if let Some(val) =  other.get(&key) {
//                 self.insert(&key, &val);
//             }
//         }
//     }

//     /// Add a single JSON metadatum using a MetadataJsonSchema object and MetadataJsonScehma object.
//     pub fn add_json_metadatum_with_schema(
//         &mut self,
//         key: &TransactionMetadatumLabel,
//         val: String,
//         schema: MetadataJsonSchema,
//     ) -> Result<(), JsError> {
//         let metadatum = encode_json_str_to_metadatum(val, schema)?;
//         self.insert(key, &metadatum);
//         Ok(())
//     }
// }

// impl AuxiliaryData {
//     /// Add a single metadatum using TransactionMetadatum object under `key` TranscactionMetadatumLabel
//     pub fn add_metadatum(
//         &mut self,
//         key: &TransactionMetadatumLabel,
//         value: &TransactionMetadatum,
//     ) {
//         match self.metadata.as_mut() {
//             Some(metadata) => {
//                 metadata.insert(key, value);
//             },
//             None => {
//                 let mut general_metadata = GeneralTransactionMetadata::new();
//                 general_metadata.insert(key, value);
//                 self.metadata = Some(general_metadata)
//             }
//         }
//     }

//     /// Add a single JSON metadatum using a MetadataJsonSchema object and MetadataJsonScehma object.
//     pub fn add_json_metadatum_with_schema(
//         &mut self,
//         key: &TransactionMetadatumLabel,
//         val: String,
//         schema: MetadataJsonSchema,
//     ) -> Result<(), JsError> {
//         let metadatum = encode_json_str_to_metadatum(val, schema)?;
//         self.add_metadatum(key, &metadatum);
//         Ok(())
//     }

//     pub fn add(&mut self, other: &AuxiliaryData) {
//         match (self.metadata.as_mut(), other.metadata.as_ref()) {
//             (None, None) => {},
//             (Some(_), None) => {},
//             (None, val@Some(_)) => { self.metadata = val.cloned(); },
//             (Some(data1), Some(data2)) => {
//                 data1.add(data2);
//             }
//         };

//         match (self.native_scripts.as_mut(), other.native_scripts.as_ref()) {
//             (None, None) => {},
//             (Some(_), None) => {},
//             (None, val@Some(_)) => { self.native_scripts = val.cloned(); },
//             (Some(data1), Some(data2)) => {
//                 for script in &data2.0 {
//                     data1.add(script);
//                 }
//             }
//         };

//         match (self.plutus_v1_scripts.as_mut(), other.plutus_v1_scripts.as_ref()) {
//             (None, None) => {},
//             (Some(_), None) => {},
//             (None, val@Some(_)) => { self.plutus_v1_scripts = val.cloned(); },
//             (Some(data1), Some(data2)) => {
//                 for script in &data2.0 {
//                     data1.add(script);
//                 }
//             }
//         };

//         match (self.plutus_v2_scripts.as_mut(), other.plutus_v2_scripts.as_ref()) {
//             (None, None) => {},
//             (Some(_), None) => {},
//             (None, val@Some(_)) => { self.plutus_v2_scripts = val.cloned(); },
//             (Some(data1), Some(data2)) => {
//                 for script in &data2.0 {
//                     data1.add(script);
//                 }
//             }
//         };
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_aux_data() {
        let aux_data = AuxiliaryData {
            metadata: Some(GeneralTransactionMetadata::new()),
            native_scripts: Some(NativeScripts::new()),
            plutus_v1_scripts: Some(PlutusV1Scripts::new()),
            plutus_v2_scripts: Some(PlutusV2Scripts::new()),
            prefer_alonzo_format: false,
        };

        let mut base_aux = AuxiliaryData::new();
        base_aux.add(&aux_data);
        assert_eq!(base_aux, aux_data);
    }
}
