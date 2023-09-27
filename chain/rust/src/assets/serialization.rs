// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;
use std::io::{BufRead, Seek, Write};

impl Serialize for AssetName {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
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
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (inner, inner_encoding) = raw
            .bytes_sz()
            .map(|(bytes, enc)| (bytes, StringEncoding::from(enc)))?;
        if inner.len() > 32 {
            return Err(DeserializeError::new(
                "AssetName",
                DeserializeFailure::RangeCheck {
                    found: inner.len(),
                    min: Some(0),
                    max: Some(32),
                },
            ));
        }
        Ok(Self {
            inner,
            encodings: Some(AssetNameEncoding { inner_encoding }),
        })
    }
}
