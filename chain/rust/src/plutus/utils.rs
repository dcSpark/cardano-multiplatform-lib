use super::{CostModels, Language};
use super::{ExUnits, PlutusData, PlutusV1Script, PlutusV2Script};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::serialization::*;
use std::io::{BufRead, Seek, SeekFrom, Write};

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConstrPlutusData {
    pub alternative: u64,
    pub fields: Vec<PlutusData>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    #[serde(skip)]
    pub encodings: Option<ConstrPlutusDataEncoding>,
}

impl ConstrPlutusData {
    // see: https://github.com/input-output-hk/plutus/blob/1f31e640e8a258185db01fa899da63f9018c0e85/plutus-core/plutus-core/src/PlutusCore/Data.hs#L61
    // We don't directly serialize the alternative in the tag, instead the scheme is:
    // - Alternatives 0-6 -> tags 121-127, followed by the arguments in a list
    // - Alternatives 7-127 -> tags 1280-1400, followed by the arguments in a list
    // - Any alternatives, including those that don't fit in the above -> tag 102 followed by a list containing
    //   an unsigned integer for the actual alternative, and then the arguments in a (nested!) list.
    const GENERAL_FORM_TAG: u64 = 102;

    // None -> needs general tag serialization, not compact
    fn alternative_to_compact_cbor_tag(alt: u64) -> Option<u64> {
        if alt <= 6 {
            Some(121 + alt)
        } else if alt >= 7 && alt <= 127 {
            Some(1280 - 7 + alt)
        } else {
            None
        }
    }

    // None -> General tag(=102) OR Invalid CBOR tag for this scheme
    fn compact_cbor_tag_to_alternative(cbor_tag: u64) -> Option<u64> {
        if cbor_tag >= 121 && cbor_tag <= 127 {
            Some(cbor_tag - 121)
        } else if cbor_tag >= 1280 && cbor_tag <= 1400 {
            Some(cbor_tag - 1280 + 7)
        } else {
            None
        }
    }

    pub fn new(alternative: u64, fields: Vec<PlutusData>) -> Self {
        Self {
            alternative,
            fields,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ConstrPlutusDataEncoding {
    pub len_encoding: LenEncoding,
    pub tag_encoding: Option<cbor_event::Sz>,
    pub alternative_encoding: Option<cbor_event::Sz>,
    pub fields_encoding: LenEncoding,
}

impl Serialize for ConstrPlutusData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        if let Some(compact_tag) = Self::alternative_to_compact_cbor_tag(self.alternative) {
            // compact form
            serializer.write_tag_sz(
                compact_tag as u64,
                fit_sz(
                    compact_tag as u64,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.tag_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            )?;
            serializer.write_array_sz(
                self.encodings
                    .as_ref()
                    .map(|encs| encs.fields_encoding)
                    .unwrap_or_default()
                    .to_len_sz(self.fields.len() as u64, force_canonical),
            )?;
            for element in self.fields.iter() {
                element.serialize(serializer, force_canonical)?;
            }
            self.encodings
                .as_ref()
                .map(|encs| encs.fields_encoding)
                .unwrap_or_default()
                .end(serializer, force_canonical)
        } else {
            // general form
            serializer.write_tag_sz(
                Self::GENERAL_FORM_TAG,
                fit_sz(
                    Self::GENERAL_FORM_TAG,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.tag_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            )?;
            serializer.write_array_sz(
                self.encodings
                    .as_ref()
                    .map(|encs| encs.len_encoding)
                    .unwrap_or_default()
                    .to_len_sz(2, force_canonical),
            )?;
            serializer.write_unsigned_integer_sz(
                self.alternative,
                fit_sz(
                    self.alternative,
                    self.encodings
                        .as_ref()
                        .map(|encs| encs.alternative_encoding)
                        .unwrap_or_default(),
                    force_canonical,
                ),
            )?;
            serializer.write_array_sz(
                self.encodings
                    .as_ref()
                    .map(|encs| encs.fields_encoding)
                    .unwrap_or_default()
                    .to_len_sz(self.fields.len() as u64, force_canonical),
            )?;
            for element in self.fields.iter() {
                element.serialize(serializer, force_canonical)?;
            }
            self.encodings
                .as_ref()
                .map(|encs| encs.fields_encoding)
                .unwrap_or_default()
                .end(serializer, force_canonical)?;
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .end(serializer, force_canonical)
        }
    }
}

impl Deserialize for ConstrPlutusData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let (tag, tag_encoding) = raw.tag_sz()?;
            match tag {
                // general form
                Self::GENERAL_FORM_TAG => {
                    let len = raw.array_sz()?;
                    let len_encoding: LenEncoding = len.into();
                    let mut read_len = CBORReadLen::new(len);
                    read_len.read_elems(2)?;
                    let (alternative, alternative_encoding) = raw
                        .unsigned_integer_sz()
                        .map(|(x, enc)| (x, Some(enc)))
                        .map_err(Into::<DeserializeError>::into)
                        .map_err(|e: DeserializeError| e.annotate("alternative"))?;
                    let (fields, fields_encoding) = (|| -> Result<_, DeserializeError> {
                        let mut fields_arr = Vec::new();
                        let len = raw.array_sz()?;
                        let fields_encoding = len.into();
                        while match len {
                            cbor_event::LenSz::Len(n, _) => (fields_arr.len() as u64) < n,
                            cbor_event::LenSz::Indefinite => true,
                        } {
                            if raw.cbor_type()? == cbor_event::Type::Special {
                                assert_eq!(raw.special()?, cbor_event::Special::Break);
                                break;
                            }
                            fields_arr.push(PlutusData::deserialize(raw)?);
                        }
                        Ok((fields_arr, fields_encoding))
                    })()
                    .map_err(|e| e.annotate("fields"))?;
                    match len {
                        cbor_event::LenSz::Len(_, _) => (),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            cbor_event::Special::Break => (),
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    }
                    Ok(ConstrPlutusData {
                        alternative,
                        fields,
                        encodings: Some(ConstrPlutusDataEncoding {
                            len_encoding,
                            tag_encoding: Some(tag_encoding),
                            alternative_encoding,
                            fields_encoding,
                        }),
                    })
                }
                // concise form
                tag => {
                    if let Some(alternative) = Self::compact_cbor_tag_to_alternative(tag) {
                        let (fields, fields_encoding) = (|| -> Result<_, DeserializeError> {
                            let mut fields_arr = Vec::new();
                            let len = raw.array_sz()?;
                            let fields_encoding = len.into();
                            while match len {
                                cbor_event::LenSz::Len(n, _) => (fields_arr.len() as u64) < n,
                                cbor_event::LenSz::Indefinite => true,
                            } {
                                if raw.cbor_type()? == cbor_event::Type::Special {
                                    assert_eq!(raw.special()?, cbor_event::Special::Break);
                                    break;
                                }
                                fields_arr.push(PlutusData::deserialize(raw)?);
                            }
                            Ok((fields_arr, fields_encoding))
                        })()
                        .map_err(|e| e.annotate("fields"))?;
                        Ok(ConstrPlutusData {
                            alternative,
                            fields,
                            encodings: Some(ConstrPlutusDataEncoding {
                                len_encoding: LenEncoding::default(),
                                tag_encoding: Some(tag_encoding),
                                alternative_encoding: None,
                                fields_encoding,
                            }),
                        })
                    } else {
                        return Err(DeserializeFailure::TagMismatch {
                            found: tag,
                            expected: Self::GENERAL_FORM_TAG,
                        }
                        .into());
                    }
                }
            }
        })()
        .map_err(|e| e.annotate("ConstrPlutusData"))
    }
}

/// Version-agnostic Plutus script
#[derive(Clone, Debug)]
pub enum PlutusScript {
    PlutusV1(PlutusV1Script),
    PlutusV2(PlutusV2Script),
}

impl PlutusScript {
    pub fn hash(&self) -> ScriptHash {
        match &self {
            Self::PlutusV1(script) => script.hash(),
            Self::PlutusV2(script) => script.hash(),
        }
    }
}
use cml_crypto::ScriptHash;
use crate::crypto::hash::{hash_script, ScriptHashNamespace};

impl PlutusV1Script {
    pub fn hash(&self) -> ScriptHash {
        hash_script(ScriptHashNamespace::PlutusV1, self.get())
    }
}

impl PlutusV2Script {
    pub fn hash(&self) -> ScriptHash {
        hash_script(ScriptHashNamespace::PlutusV2, self.get())
    }
}

impl ExUnits {
    pub fn checked_add(&self, other: &ExUnits) -> Option<ExUnits> {
        let mem = self.mem.checked_add(other.mem)?;
        let step = self.steps.checked_add(other.steps)?;
        Some(ExUnits::new(mem, step))
    }

     /// used to create a dummy ExUnits that takes up the maximum size possible in cbor to provide an upper bound on tx size
     pub fn dummy() -> ExUnits {
        ExUnits::new(u64::MAX, u64::MAX)
    }
}
