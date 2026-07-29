// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

extern crate alloc;
use super::*;
use alloc::string::String;
use alloc::vec::Vec;
use cbor_event::de::Deserializer;
use cbor_event::se::{Serialize, Serializer};
use cml_core::error::*;
use cml_core::serialization::*;

impl cbor_event::se::Serialize for CIP25ChunkableString {
    fn serialize<'se>(
        &self,
        serializer: &'se mut Serializer,
    ) -> cbor_event::Result<&'se mut Serializer> {
        match self {
            CIP25ChunkableString::Single(single) => single.serialize(serializer),
            CIP25ChunkableString::Chunked(chunked) => {
                serializer.write_array(cbor_event::Len::Len(chunked.len() as u64))?;
                for element in chunked.iter() {
                    element.serialize(serializer)?;
                }
                Ok(serializer)
            }
        }
    }
}

impl Deserialize for CIP25ChunkableString {
    fn deserialize(raw: &mut Deserializer) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            match raw.cbor_type()? {
                cbor_event::Type::Text => Ok(CIP25ChunkableString::Single(
                    CIP25String64::deserialize(raw)?,
                )),
                cbor_event::Type::Array => {
                    let mut chunked_arr = Vec::new();
                    let len = raw.array()?;
                    while match len {
                        cbor_event::Len::Len(n) => (chunked_arr.len() as u64) < n,
                        cbor_event::Len::Indefinite => true,
                    } {
                        if matches!(len, cbor_event::Len::Indefinite)
                            && raw.cbor_type()? == cbor_event::Type::Special
                            && raw.special_break()?
                        {
                            break;
                        }
                        chunked_arr.push(CIP25String64::deserialize(raw)?);
                    }
                    let chunked = chunked_arr;
                    Ok(Self::Chunked(chunked))
                }
                _ => Err(DeserializeFailure::NoVariantMatched.into()),
            }
        })()
        .map_err(|e| e.annotate("CIP25ChunkableString"))
    }
}

impl cbor_event::se::Serialize for CIP25FilesDetails {
    fn serialize<'se>(
        &self,
        serializer: &'se mut Serializer,
    ) -> cbor_event::Result<&'se mut Serializer> {
        serializer.write_map(cbor_event::Len::Len(3))?;
        serializer.write_text("src")?;
        self.src.serialize(serializer)?;
        serializer.write_text("name")?;
        self.name.serialize(serializer)?;
        serializer.write_text("mediaType")?;
        self.media_type.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for CIP25FilesDetails {
    fn deserialize(raw: &mut Deserializer) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::from(len);
            // cddl-codegen:replace-start
            read_len.read_elems(3)?;
            // cddl-codegen:replaces
            // read_len.read_elems(3)?;
            // read_len.finish()?;
            // cddl-codegen:replace-end
            let mut src = None;
            let mut name = None;
            let mut media_type = None;
            let mut read = 0;
            while match len {
                cbor_event::Len::Len(n) => read < n,
                cbor_event::Len::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => {
                        return Err(DeserializeFailure::UnknownKey(Key::Uint(
                            raw.unsigned_integer()?,
                        ))
                        .into());
                    }
                    cbor_event::Type::Text => match raw.text()?.as_str() {
                        "src" => {
                            if src.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "src".into(),
                                ))
                                .into());
                            }
                            src = Some(
                                CIP25ChunkableString::deserialize(raw)
                                    .map_err(|e: DeserializeError| e.annotate("src"))?,
                            );
                        }
                        "name" => {
                            if name.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "name".into(),
                                ))
                                .into());
                            }
                            name = Some(
                                CIP25String64::deserialize(raw)
                                    .map_err(|e: DeserializeError| e.annotate("name"))?,
                            );
                        }
                        "mediaType" => {
                            if media_type.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "mediaType".into(),
                                ))
                                .into());
                            }
                            media_type = Some(
                                CIP25String64::deserialize(raw)
                                    .map_err(|e: DeserializeError| e.annotate("media_type"))?,
                            );
                        }
                        // cddl-codegen:replace-start
                        _unknown_key => {
                            // CIP-25 allows permissive parsing
                            read_len.read_elems(1)?;
                            // we still need to read the data to move on to the CBOR after it
                            let _other_metadatum =
                                cml_chain::auxdata::TransactionMetadatum::deserialize(raw)?;
                        } // cddl-codegen:replaces
                          // unknown_key => {
                          //     return Err(DeserializeFailure::UnknownKey(Key::Str(
                          //         unknown_key.to_owned(),
                          //     ))
                          //     .into());
                          // }
                          // cddl-codegen:replace-end
                    },
                    cbor_event::Type::Special => match len {
                        cbor_event::Len::Len(_) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into());
                        }
                        cbor_event::Len::Indefinite => match raw.special()? {
                            cbor_event::Special::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    // cddl-codegen:replace-start
                    _other_type => {
                        // CIP-25 allows permissive parsing
                        read_len.read_elems(1)?;
                        // we still need to read the data to move on to the CBOR after it
                        let _other_key =
                            cml_chain::auxdata::TransactionMetadatum::deserialize(raw)?;
                        let _other_value =
                            cml_chain::auxdata::TransactionMetadatum::deserialize(raw)?;
                    } // cddl-codegen:replaces
                      // other_type => {
                      //     return Err(DeserializeFailure::UnexpectedKeyType(other_type).into());
                      // }
                      // cddl-codegen:replace-end
                }
                read += 1;
            }
            let name = match name {
                Some(x) => x,
                None => {
                    return Err(
                        DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("name")))
                            .into(),
                    );
                }
            };
            let media_type =
                match media_type {
                    Some(x) => x,
                    None => {
                        return Err(DeserializeFailure::MandatoryFieldMissing(Key::Str(
                            String::from("mediaType"),
                        ))
                        .into());
                    }
                };
            let src = match src {
                Some(x) => x,
                None => {
                    return Err(
                        DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("src")))
                            .into(),
                    );
                }
            };
            // cddl-codegen:insert-start
            read_len.finish()?;
            // cddl-codegen:insert-end
            Ok(Self {
                name,
                media_type,
                src,
            })
        })()
        .map_err(|e| e.annotate("CIP25FilesDetails"))
    }
}

impl cbor_event::se::Serialize for CIP25Metadata {
    fn serialize<'se>(
        &self,
        serializer: &'se mut Serializer,
    ) -> cbor_event::Result<&'se mut Serializer> {
        serializer.write_map(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(721u64)?;
        self.key_721.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for CIP25Metadata {
    fn deserialize(raw: &mut Deserializer) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::from(len);
            // cddl-codegen:replace-start
            read_len.read_elems(1)?;
            // cddl-codegen:replaces
            // read_len.read_elems(1)?;
            // read_len.finish()?;
            // cddl-codegen:replace-end
            let mut key_721 = None;
            let mut read = 0;
            while match len {
                cbor_event::Len::Len(n) => read < n,
                cbor_event::Len::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => match raw.unsigned_integer()? {
                        721 => {
                            if key_721.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(721)).into());
                            }
                            key_721 = Some(
                                CIP25LabelMetadata::deserialize(raw)
                                    .map_err(|e: DeserializeError| e.annotate("key_721"))?,
                            );
                        }
                        // cddl-codegen:replace-start
                        _unknown_key => {
                            // CIP-25 allows permissive parsing
                            read_len.read_elems(1)?;
                            // we still need to read the data to move on to the CBOR after it
                            let _other_metadatum =
                                cml_chain::auxdata::TransactionMetadatum::deserialize(raw)?;
                        } // cddl-codegen:replaces
                          // unknown_key => {
                          //     return Err(
                          //         DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()
                          //     );
                          // }
                          // cddl-codegen:replace-end
                    },
                    cbor_event::Type::Text => {
                        return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into());
                    }
                    cbor_event::Type::Special => match len {
                        cbor_event::Len::Len(_) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into());
                        }
                        cbor_event::Len::Indefinite => match raw.special()? {
                            cbor_event::Special::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => {
                        return Err(DeserializeFailure::UnexpectedKeyType(other_type).into());
                    }
                }
                read += 1;
            }
            let key_721 = match key_721 {
                Some(x) => x,
                None => {
                    return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(721)).into());
                }
            };
            // cddl-codegen:insert-start
            read_len.finish()?;
            // cddl-codegen:insert-end
            Ok(Self { key_721 })
        })()
        .map_err(|e| e.annotate("CIP25Metadata"))
    }
}

impl cbor_event::se::Serialize for CIP25MetadataDetails {
    fn serialize<'se>(
        &self,
        serializer: &'se mut Serializer,
    ) -> cbor_event::Result<&'se mut Serializer> {
        serializer.write_map(cbor_event::Len::Len(
            2 + match &self.media_type {
                Some(_) => 1,
                None => 0,
            } + match &self.description {
                Some(_) => 1,
                None => 0,
            } + match &self.files {
                Some(_) => 1,
                None => 0,
            },
        ))?;
        serializer.write_text("name")?;
        self.name.serialize(serializer)?;
        if let Some(field) = &self.files {
            serializer.write_text("files")?;
            serializer.write_array(cbor_event::Len::Len(field.len() as u64))?;
            for element in field.iter() {
                element.serialize(serializer)?;
            }
        }
        serializer.write_text("image")?;
        self.image.serialize(serializer)?;
        if let Some(field) = &self.media_type {
            serializer.write_text("mediaType")?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.description {
            serializer.write_text("description")?;
            field.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for CIP25MetadataDetails {
    fn deserialize(raw: &mut Deserializer) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::from(len);
            read_len.read_elems(2)?;
            let mut name = None;
            let mut files = None;
            let mut image = None;
            let mut media_type = None;
            let mut description = None;
            let mut read = 0;
            while match len {
                cbor_event::Len::Len(n) => read < n,
                cbor_event::Len::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => {
                        return Err(DeserializeFailure::UnknownKey(Key::Uint(
                            raw.unsigned_integer()?,
                        ))
                        .into());
                    }
                    cbor_event::Type::Text => match raw.text()?.as_str() {
                        "name" => {
                            if name.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "name".into(),
                                ))
                                .into());
                            }
                            name = Some(
                                CIP25String64::deserialize(raw)
                                    .map_err(|e: DeserializeError| e.annotate("name"))?,
                            );
                        }
                        "files" => {
                            if files.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "files".into(),
                                ))
                                .into());
                            }
                            files = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    let mut files_arr = Vec::new();
                                    let len = raw.array()?;
                                    while match len {
                                        cbor_event::Len::Len(n) => (files_arr.len() as u64) < n,
                                        cbor_event::Len::Indefinite => true,
                                    } {
                                        if matches!(len, cbor_event::Len::Indefinite)
                                            && raw.cbor_type()? == cbor_event::Type::Special
                                            && raw.special_break()?
                                        {
                                            break;
                                        }
                                        files_arr.push(CIP25FilesDetails::deserialize(raw)?);
                                    }
                                    Ok(files_arr)
                                })()
                                .map_err(|e| e.annotate("files"))?,
                            );
                        }
                        "image" => {
                            if image.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "image".into(),
                                ))
                                .into());
                            }
                            image = Some(
                                CIP25ChunkableString::deserialize(raw)
                                    .map_err(|e: DeserializeError| e.annotate("image"))?,
                            );
                        }
                        "mediaType" => {
                            if media_type.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "mediaType".into(),
                                ))
                                .into());
                            }
                            media_type = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    CIP25String64::deserialize(raw)
                                })()
                                .map_err(|e| e.annotate("media_type"))?,
                            );
                        }
                        "description" => {
                            if description.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str(
                                    "description".into(),
                                ))
                                .into());
                            }
                            description = Some(
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    CIP25ChunkableString::deserialize(raw)
                                })()
                                .map_err(|e| e.annotate("description"))?,
                            );
                        }
                        // cddl-codegen:replace-start
                        _unknown_key => {
                            // CIP-25 allows permissive parsing
                            read_len.read_elems(1)?;
                            // we still need to read the data to move on to the CBOR after it
                            let _other_metadatum =
                                cml_chain::auxdata::TransactionMetadatum::deserialize(raw)?;
                        } // cddl-codegen:replaces
                          // unknown_key => {
                          //     return Err(DeserializeFailure::UnknownKey(Key::Str(
                          //         unknown_key.to_owned(),
                          //     ))
                          //     .into());
                          // }
                          // cddl-codegen:replace-end
                    },
                    cbor_event::Type::Special => match len {
                        cbor_event::Len::Len(_) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into());
                        }
                        cbor_event::Len::Indefinite => match raw.special()? {
                            cbor_event::Special::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    // cddl-codegen:replace-start
                    _other_type => {
                        // CIP-25 allows permissive parsing
                        read_len.read_elems(1)?;
                        // we still need to read the data to move on to the CBOR after it
                        let _other_key =
                            cml_chain::auxdata::TransactionMetadatum::deserialize(raw)?;
                        let _other_value =
                            cml_chain::auxdata::TransactionMetadatum::deserialize(raw)?;
                    } // cddl-codegen:replaces
                      // other_type => {
                      //     return Err(DeserializeFailure::UnexpectedKeyType(other_type).into());
                      // }
                      // cddl-codegen:replace-end
                }
                read += 1;
            }
            let name = match name {
                Some(x) => x,
                None => {
                    return Err(
                        DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("name")))
                            .into(),
                    );
                }
            };
            let image = match image {
                Some(x) => x,
                None => {
                    return Err(
                        DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("image")))
                            .into(),
                    );
                }
            };
            read_len.finish()?;
            Ok(Self {
                name,
                image,
                media_type,
                description,
                files,
            })
        })()
        .map_err(|e| e.annotate("CIP25MetadataDetails"))
    }
}

impl cbor_event::se::Serialize for CIP25String64 {
    fn serialize<'se>(
        &self,
        serializer: &'se mut Serializer,
    ) -> cbor_event::Result<&'se mut Serializer> {
        serializer.write_text(&self.0)
    }
}

impl Deserialize for CIP25String64 {
    fn deserialize(raw: &mut Deserializer) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let inner = raw.text()?;
            if inner.len() > 64 {
                return Err(DeserializeFailure::RangeCheck {
                    found: inner.len() as i128,
                    min: Some(0),
                    max: Some(64),
                }
                .into());
            }
            Ok(Self(inner))
        })()
        .map_err(|e| e.annotate("CIP25String64"))
    }
}
