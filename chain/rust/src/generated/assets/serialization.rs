// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;

impl Serialize for AssetName {
    fn serialize<'se>(
        &self,
        serializer: &'se mut Serializer,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer> {
        serializer.write_bytes_sz(
            &self.inner,
            self.encodings
                .as_ref()
                .map(|encs| encs.inner_encoding.clone())
                .unwrap_or_default()
                .to_str_len_sz(self.inner.len() as u64, force_canonical),
        )
    }
}

impl Deserialize for AssetName {
    fn deserialize(raw: &mut Deserializer) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let (inner, inner_encoding) = raw
                .bytes_sz()
                .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
            if inner.len() > 32 {
                return Err(DeserializeFailure::RangeCheck {
                    found: inner.len() as i128,
                    min: Some(0),
                    max: Some(32),
                }
                .into());
            }
            Ok(Self {
                inner,
                encodings: Some(AssetNameEncoding { inner_encoding }),
            })
        })()
        .map_err(|e| e.annotate("AssetName"))
    }
}
