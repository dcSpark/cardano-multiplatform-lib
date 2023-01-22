use super::*;
pub use cml_core::{
    serialization::*,
    error::*,
};
use std::io::{Seek, SeekFrom};

impl cbor_event::se::Serialize for AssetNameV1 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for AssetNameV1 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(String64::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for AssetNameV2 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for AssetNameV2 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(raw.bytes()? as Vec<u8>))
    }
}

impl cbor_event::se::Serialize for FilesDetails {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(3))?;
        serializer.write_text(&"src")?;
        self.src.serialize(serializer)?;
        serializer.write_text(&"name")?;
        self.name.serialize(serializer)?;
        serializer.write_text(&"mediaType")?;
        self.media_type.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for FilesDetails {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(match len {
                cbor_event::Len::Len(n) => cbor_event::LenSz::Len(n, cbor_event::Sz::canonical(n)),
                cbor_event::Len::Indefinite => cbor_event::LenSz::Indefinite,
            });
            read_len.read_elems(3)?;
            let mut src = None;
            let mut name = None;
            let mut media_type = None;
            let mut read = 0;
            while match len { cbor_event::Len::Len(n) => read < n as usize, cbor_event::Len::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::Text => match raw.text()?.as_str() {
                        "src" =>  {
                            if src.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str("src".into())).into());
                            }
                            src = Some((|| -> Result<_, DeserializeError> {
                                Ok(ChunkableString::deserialize(raw)?)
                            })().map_err(|e| e.annotate("src"))?);
                        },
                        "name" =>  {
                            if name.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str("name".into())).into());
                            }
                            name = Some((|| -> Result<_, DeserializeError> {
                                Ok(String64::deserialize(raw)?)
                            })().map_err(|e| e.annotate("name"))?);
                        },
                        "mediaType" =>  {
                            if media_type.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str("mediaType".into())).into());
                            }
                            media_type = Some((|| -> Result<_, DeserializeError> {
                                Ok(String64::deserialize(raw)?)
                            })().map_err(|e| e.annotate("media_type"))?);
                        },
                        _unknown_key => /* CIP-25 allows permissive parsing */(),
                    },
                    CBORType::Special => match len {
                        cbor_event::Len::Len(_) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    _other_type => /* CIP-25 allows permissive parsing */(),
                }
                read += 1;
            }
            let name = match name {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("name"))).into()),
            };
            let media_type = match media_type {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("mediaType"))).into()),
            };
            let src = match src {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("src"))).into()),
            };
            ();
            Ok(Self {
                name,
                media_type,
                src,
            })
        })().map_err(|e| e.annotate("FilesDetails"))
    }
}

impl cbor_event::se::Serialize for LabelMetadata {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            LabelMetadata::LabelMetadataV1(label_metadata_v1) => {
                serializer.write_map(cbor_event::Len::Len(label_metadata_v1.len() as u64))?;
                for (key, value) in label_metadata_v1.iter() {
                    key.serialize(serializer)?;
                    serializer.write_map(cbor_event::Len::Len(value.len() as u64))?;
                    for (key, value) in value.iter() {
                        key.serialize(serializer)?;
                        value.serialize(serializer)?;
                    }
                }
                Ok(serializer)
            },
            LabelMetadata::LabelMetadataV2(label_metadata_v2) => {
                label_metadata_v2.serialize(serializer)
            },
        }
    }
}

impl Deserialize for LabelMetadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut label_metadata_v1_table = BTreeMap::new();
                let label_metadata_v1_len = raw.map()?;
                while match label_metadata_v1_len { cbor_event::Len::Len(n) => label_metadata_v1_table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    let label_metadata_v1_key = PolicyIdV1::deserialize(raw)?;
                    let mut label_metadata_v1_value_table = BTreeMap::new();
                    let label_metadata_v1_value_len = raw.map()?;
                    while match label_metadata_v1_value_len { cbor_event::Len::Len(n) => label_metadata_v1_value_table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                        if raw.cbor_type()? == CBORType::Special {
                            assert_eq!(raw.special()?, CBORSpecial::Break);
                            break;
                        }
                        let label_metadata_v1_value_key = AssetNameV1::deserialize(raw)?;
                        let label_metadata_v1_value_value = MetadataDetails::deserialize(raw)?;
                        if label_metadata_v1_value_table.insert(label_metadata_v1_value_key.clone(), label_metadata_v1_value_value).is_some() {
                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                        }
                    }
                    let label_metadata_v1_value = label_metadata_v1_value_table;
                    if label_metadata_v1_table.insert(label_metadata_v1_key.clone(), label_metadata_v1_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                    }
                }
                Ok(label_metadata_v1_table)
            })(raw)
            {
                Ok(label_metadata_v1) => return Ok(Self::LabelMetadataV1(label_metadata_v1)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(LabelMetadataV2::deserialize(raw)?)
            })(raw)
            {
                Ok(label_metadata_v2) => return Ok(Self::LabelMetadataV2(label_metadata_v2)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("LabelMetadata", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("LabelMetadata"))
    }
}

impl cbor_event::se::Serialize for LabelMetadataV2 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(2))?;
        serializer.write_text(&"data")?;
        serializer.write_map(cbor_event::Len::Len(self.data.len() as u64))?;
        for (key, value) in self.data.iter() {
            key.serialize(serializer)?;
            serializer.write_map(cbor_event::Len::Len(value.len() as u64))?;
            for (key, value) in value.iter() {
                key.serialize(serializer)?;
                value.serialize(serializer)?;
            }
        }
        serializer.write_text(&"version")?;
        serializer.write_unsigned_integer(2u64)?;
        Ok(serializer)
    }
}

impl Deserialize for LabelMetadataV2 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(match len {
                cbor_event::Len::Len(n) => cbor_event::LenSz::Len(n, cbor_event::Sz::canonical(n)),
                cbor_event::Len::Indefinite => cbor_event::LenSz::Indefinite,
            });
            read_len.read_elems(2)?;
            let mut data = None;
            let mut version_present = false;
            let mut read = 0;
            while match len { cbor_event::Len::Len(n) => read < n as usize, cbor_event::Len::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::Text => match raw.text()?.as_str() {
                        "data" =>  {
                            if data.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str("data".into())).into());
                            }
                            data = Some((|| -> Result<_, DeserializeError> {
                                let mut data_table = BTreeMap::new();
                                let data_len = raw.map()?;
                                while match data_len { cbor_event::Len::Len(n) => data_table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    let data_key = PolicyIdV2::deserialize(raw)?;
                                    let mut data_value_table = BTreeMap::new();
                                    let data_value_len = raw.map()?;
                                    while match data_value_len { cbor_event::Len::Len(n) => data_value_table.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                                        if raw.cbor_type()? == CBORType::Special {
                                            assert_eq!(raw.special()?, CBORSpecial::Break);
                                            break;
                                        }
                                        let data_value_key = AssetNameV2::deserialize(raw)?;
                                        let data_value_value = MetadataDetails::deserialize(raw)?;
                                        if data_value_table.insert(data_value_key.clone(), data_value_value).is_some() {
                                            return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                                        }
                                    }
                                    let data_value = data_value_table;
                                    if data_table.insert(data_key.clone(), data_value).is_some() {
                                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from("some complicated/unsupported type"))).into());
                                    }
                                }
                                Ok(data_table)
                            })().map_err(|e| e.annotate("data"))?);
                        },
                        "version" =>  {
                            if version_present {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str("version".into())).into());
                            }
                            version_present = (|| -> Result<_, DeserializeError> {
                                let version_value = raw.unsigned_integer()?;
                                if version_value != 2 {
                                    return Err(DeserializeFailure::FixedValueMismatch{ found: Key::Uint(version_value), expected: Key::Uint(2) }.into());
                                }
                                Ok(true)
                            })().map_err(|e| e.annotate("version"))?;
                        },
                        _unknown_key => /* CIP-25 allows permissive parsing */(),
                    },
                    CBORType::Special => match len {
                        cbor_event::Len::Len(_) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    _other_type => /* CIP-25 allows permissive parsing */(),
                }
                read += 1;
            }
            let data = match data {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("data"))).into()),
            };
            if !version_present {
                return Err(DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("version"))).into());
            }
            ();
            Ok(Self {
                data,
            })
        })().map_err(|e| e.annotate("LabelMetadataV2"))
    }
}

impl cbor_event::se::Serialize for CIP25Metadata {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(1))?;
        serializer.write_unsigned_integer(721u64)?;
        self.key_721.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for CIP25Metadata {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(match len {
                cbor_event::Len::Len(n) => cbor_event::LenSz::Len(n, cbor_event::Sz::canonical(n)),
                cbor_event::Len::Indefinite => cbor_event::LenSz::Indefinite,
            });
            read_len.read_elems(1)?;
            let mut key_721 = None;
            let mut read = 0;
            while match len { cbor_event::Len::Len(n) => read < n as usize, cbor_event::Len::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::UnsignedInteger => match raw.unsigned_integer()? {
                        721 =>  {
                            if key_721.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(721)).into());
                            }
                            key_721 = Some((|| -> Result<_, DeserializeError> {
                                Ok(LabelMetadata::deserialize(raw)?)
                            })().map_err(|e| e.annotate("key_721"))?);
                        },
                        _unknown_key => /* we must be permissive as we are looking at a subset of metadata here */(),
                    },
                    CBORType::Special => match len {
                        cbor_event::Len::Len(_) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    _other_type => /* we must be permissive as we are looking at a subset of metadata here */(),
                }
                read += 1;
            }
            let key_721 = match key_721 {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Uint(721)).into()),
            };
            ();
            Ok(Self {
                key_721,
            })
        })().map_err(|e| e.annotate("CIP25Metadata"))
    }
}

impl cbor_event::se::Serialize for MetadataDetails {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map(cbor_event::Len::Len(2 + match &self.media_type { Some(_) => 1, None => 0 } + match &self.description { Some(_) => 1, None => 0 } + match &self.files { Some(_) => 1, None => 0 }))?;
        serializer.write_text(&"name")?;
        self.name.serialize(serializer)?;
        if let Some(field) = &self.files {
            serializer.write_text(&"files")?;
            serializer.write_array(cbor_event::Len::Len(field.len() as u64))?;
            for element in field.iter() {
                element.serialize(serializer)?;
            }
        }
        serializer.write_text(&"image")?;
        self.image.serialize(serializer)?;
        if let Some(field) = &self.media_type {
            serializer.write_text(&"mediaType")?;
            field.serialize(serializer)?;
        }
        if let Some(field) = &self.description {
            serializer.write_text(&"description")?;
            field.serialize(serializer)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for MetadataDetails {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.map()?;
            let mut read_len = CBORReadLen::new(match len {
                cbor_event::Len::Len(n) => cbor_event::LenSz::Len(n, cbor_event::Sz::canonical(n)),
                cbor_event::Len::Indefinite => cbor_event::LenSz::Indefinite,
            });
            read_len.read_elems(2)?;
            let mut name = None;
            let mut files = None;
            let mut image = None;
            let mut media_type = None;
            let mut description = None;
            let mut read = 0;
            while match len { cbor_event::Len::Len(n) => read < n as usize, cbor_event::Len::Indefinite => true, } {
                match raw.cbor_type()? {
                    CBORType::Text => match raw.text()?.as_str() {
                        "name" =>  {
                            if name.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str("name".into())).into());
                            }
                            name = Some((|| -> Result<_, DeserializeError> {
                                Ok(String64::deserialize(raw)?)
                            })().map_err(|e| e.annotate("name"))?);
                        },
                        "files" =>  {
                            if files.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str("files".into())).into());
                            }
                            files = Some((|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                let mut files_arr = Vec::new();
                                let len = raw.array()?;
                                while match len { cbor_event::Len::Len(n) => files_arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                                    if raw.cbor_type()? == CBORType::Special {
                                        assert_eq!(raw.special()?, CBORSpecial::Break);
                                        break;
                                    }
                                    files_arr.push(FilesDetails::deserialize(raw)?);
                                }
                                Ok(files_arr)
                            })().map_err(|e| e.annotate("files"))?);
                        },
                        "image" =>  {
                            if image.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str("image".into())).into());
                            }
                            image = Some((|| -> Result<_, DeserializeError> {
                                Ok(ChunkableString::deserialize(raw)?)
                            })().map_err(|e| e.annotate("image"))?);
                        },
                        "mediaType" =>  {
                            if media_type.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str("mediaType".into())).into());
                            }
                            media_type = Some((|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(String64::deserialize(raw)?)
                            })().map_err(|e| e.annotate("media_type"))?);
                        },
                        "description" =>  {
                            if description.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Str("description".into())).into());
                            }
                            description = Some((|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Ok(ChunkableString::deserialize(raw)?)
                            })().map_err(|e| e.annotate("description"))?);
                        },
                        _unknown_key => /* CIP-25 allows permissive parsing */(),
                    },
                    CBORType::Special => match len {
                        cbor_event::Len::Len(_) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::Len::Indefinite => match raw.special()? {
                            CBORSpecial::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    _other_type => /* CIP-25 allows permissive parsing */(),
                }
                read += 1;
            }
            let name = match name {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("name"))).into()),
            };
            let image = match image {
                Some(x) => x,
                None => return Err(DeserializeFailure::MandatoryFieldMissing(Key::Str(String::from("image"))).into()),
            };
            read_len.finish()?;
            Ok(Self {
                name,
                image,
                media_type,
                description,
                files,
            })
        })().map_err(|e| e.annotate("MetadataDetails"))
    }
}

impl cbor_event::se::Serialize for PolicyIdV1 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        self.0.serialize(serializer)
    }
}

impl Deserialize for PolicyIdV1 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(String64::deserialize(raw)?))
    }
}

impl cbor_event::se::Serialize for PolicyIdV2 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_bytes(&self.0)
    }
}

impl Deserialize for PolicyIdV2 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        Ok(Self(raw.bytes()? as Vec<u8>))
    }
}

impl cbor_event::se::Serialize for String64 {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_text(&self.0)
    }
}

impl Deserialize for String64 {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let inner = raw.text()? as String;
        if inner.len() > 64 {
            return Err(DeserializeError::new("String64", DeserializeFailure::RangeCheck{ found: inner.len(), min: Some(0), max: Some(64) }));
        }
        Ok(Self(inner))
    }
}

impl cbor_event::se::Serialize for ChunkableString {
    fn serialize<'se, W: Write>(&self, serializer: &'se mut Serializer<W>) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            ChunkableString::Single(string64) => {
                string64.serialize(serializer)
            },
            ChunkableString::Chunked(arr_string64) => {
                serializer.write_array(cbor_event::Len::Len(arr_string64.len() as u64))?;
                for element in arr_string64.iter() {
                    element.serialize(serializer)?;
                }
                Ok(serializer)
            },
        }
    }
}

impl Deserialize for ChunkableString {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                Ok(String64::deserialize(raw)?)
            })(raw)
            {
                Ok(string64) => return Ok(Self::Single(string64)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut arr_string64_arr = Vec::new();
                let len = raw.array()?;
                while match len { cbor_event::Len::Len(n) => arr_string64_arr.len() < n as usize, cbor_event::Len::Indefinite => true, } {
                    if raw.cbor_type()? == CBORType::Special {
                        assert_eq!(raw.special()?, CBORSpecial::Break);
                        break;
                    }
                    arr_string64_arr.push(String64::deserialize(raw)?);
                }
                Ok(arr_string64_arr)
            })(raw)
            {
                Ok(arr_string64) => return Ok(Self::Chunked(arr_string64)),
                Err(_) => raw.as_mut_ref().seek(SeekFrom::Start(initial_position)).unwrap(),
            };
            Err(DeserializeError::new("ChunkableString", DeserializeFailure::NoVariantMatched.into()))
        })().map_err(|e| e.annotate("ChunkableString"))
    }
}
