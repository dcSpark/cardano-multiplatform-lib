use super::cbor_encodings::CostModelsEncoding;
use super::{CostModels, Language, Redeemer};
use super::{ExUnits, PlutusData, PlutusV1Script, PlutusV2Script};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::{error::*, Int};
use cml_core::serialization::*;
use std::collections::BTreeMap;
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
    pub prefer_compact: bool,
}

impl Serialize for ConstrPlutusData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match Self::alternative_to_compact_cbor_tag(self.alternative) {
            Some(compact_tag) if self.encodings.as_ref().map(|encs| encs.prefer_compact).unwrap_or(true) => {
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
            },
            _ => {
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
            },
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
                            prefer_compact: false,
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
                                prefer_compact: true,
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

impl CostModels {
    pub fn as_map<'a>(&'a self) -> BTreeMap<Language, &'a [Int]> {
        let mut map = BTreeMap::new();
        if let Some(v1_costs) = &self.plutus_v1 {
            map.insert(Language::PlutusV1, &v1_costs[..]);
        }
        if let Some(v2_costs) = &self.plutus_v2 {
            map.insert(Language::PlutusV1, &v2_costs[..]);
        }
        map
    }

    pub(crate) fn language_views_encoding(&self) -> Result<Vec<u8>, cbor_event::Error> {
        // ; language views CDDL:
        // ; { * language => script_integrity_data }
        // ;
        // ; This must be encoded canonically, using the same scheme as in
        // ; RFC7049 section 3.9:
        // ;  - Maps, strings, and bytestrings must use a definite-length encoding
        // ;  - Integers must be as small as possible.
        // ;  - The expressions for map length, string length, and bytestring length
        // ;    must be as short as possible.
        // ;  - The keys in the map must be sorted as follows:
        // ;     -  If two keys have different lengths, the shorter one sorts earlier.
        // ;     -  If two keys have the same length, the one with the lower value
        // ;        in (byte-wise) lexical order sorts earlier.
        let mut serializer = Serializer::new_vec();
        // as canonical encodings are used, we odn't need to check the keys' bytes encodings
        // and can order this statically.
        serializer.write_map(cbor_event::Len::Len(if self.plutus_v1.is_some() { 1 } else { 0 } + if self.plutus_v2.is_some() { 1 } else { 0 }))?;
        if let Some(v1_costs) = &self.plutus_v1 {
            // For PlutusV1 (language id 0), the language view is the following:
            //   * the value of costmdls map at key 0 is encoded as an indefinite length
            //     list and the result is encoded as a bytestring. (our apologies)
            //   * the language ID tag is also encoded twice. first as a uint then as
            //     a bytestring. (our apologies)
            let v1_key_canonical_bytes = [0];
            serializer.write_bytes(&v1_key_canonical_bytes)?;
            // Due to a bug in the cardano-node input-output-hk/cardano-ledger-specs/issues/2512
            // we must use indefinite length serialization in this inner bytestring to match it
            let mut cost_model_serializer = Serializer::new_vec();
            cost_model_serializer.write_array(cbor_event::Len::Indefinite)?;
            for cost in v1_costs {
                cost.serialize(&mut cost_model_serializer, true)?;
            }
            cost_model_serializer.write_special(cbor_event::Special::Break)?;
            serializer.write_bytes(cost_model_serializer.finalize())?;
        }
        if let Some(v2_costs) = &self.plutus_v2 {
            // For PlutusV2 (language id 1), the language view is the following:
            //    * the value of costmdls map at key 1 is encoded as an definite length list.
            let v2_key = 1;
            serializer.write_unsigned_integer(v2_key)?;
            serializer.write_array(cbor_event::Len::Len(v2_costs.len() as u64))?;
            for cost in v2_costs {
                cost.serialize(&mut serializer, true)?;
            }
        }
        Ok(serializer.finalize())
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

    pub fn version(&self) -> Language {
        match self {
            Self::PlutusV1(_) => Language::PlutusV1,
            Self::PlutusV2(_) => Language::PlutusV2,
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
    pub fn checked_add(&self, other: &ExUnits) -> Result<ExUnits, ArithmeticError> {
        let mem = self.mem.checked_add(other.mem).ok_or(ArithmeticError::IntegerOverflow)?;
        let step = self.steps.checked_add(other.steps).ok_or(ArithmeticError::IntegerOverflow)?;
        Ok(ExUnits::new(mem, step))
    }

     /// used to create a dummy ExUnits that takes up the maximum size possible in cbor to provide an upper bound on tx size
     pub fn dummy() -> ExUnits {
        ExUnits::new(u64::MAX, u64::MAX)
    }
}

pub fn compute_total_ex_units(redeemers: &[Redeemer]) -> Result<ExUnits, ArithmeticError> {
    let mut sum = ExUnits::new(0, 0);
    for redeemer in redeemers {
        sum = sum.checked_add(&redeemer.ex_units)?;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use cml_core::Int;
    use crate::plutus::CostModels;

    #[test]
    pub fn test_cost_model() {
        let arr = vec![
            197209, 0, 1, 1, 396231, 621, 0, 1, 150000, 1000, 0, 1, 150000, 32,
            2477736, 29175, 4, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773,
            100, 29773, 100, 100, 100, 29773, 100, 150000, 32, 150000, 32, 150000, 32,
            150000, 1000, 0, 1, 150000, 32, 150000, 1000, 0, 8, 148000, 425507, 118,
            0, 1, 1, 150000, 1000, 0, 8, 150000, 112536, 247, 1, 150000, 10000, 1,
            136542, 1326, 1, 1000, 150000, 1000, 1, 150000, 32, 150000, 32, 150000,
            32, 1, 1, 150000, 1, 150000, 4, 103599, 248, 1, 103599, 248, 1, 145276,
            1366, 1, 179690, 497, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32,
            150000, 32, 150000, 32, 148000, 425507, 118, 0, 1, 1, 61516, 11218, 0, 1,
            150000, 32, 148000, 425507, 118, 0, 1, 1, 148000, 425507, 118, 0, 1, 1,
            2477736, 29175, 4, 0, 82363, 4, 150000, 5000, 0, 1, 150000, 32, 197209, 0,
            1, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000,
            32, 150000, 32, 3345831, 1, 1,
        ];
        let mut cms = CostModels::new();
        cms.plutus_v1 = Some(arr.iter().map(|&i| Int::new_uint(i)).collect());
        assert_eq!(
            hex::encode(cms.language_views_encoding().unwrap()),
            "a141005901d59f1a000302590001011a00060bc719026d00011a000249f01903e800011a000249f018201a0025cea81971f70419744d186419744d186419744d186419744d186419744d186419744d18641864186419744d18641a000249f018201a000249f018201a000249f018201a000249f01903e800011a000249f018201a000249f01903e800081a000242201a00067e2318760001011a000249f01903e800081a000249f01a0001b79818f7011a000249f0192710011a0002155e19052e011903e81a000249f01903e8011a000249f018201a000249f018201a000249f0182001011a000249f0011a000249f0041a000194af18f8011a000194af18f8011a0002377c190556011a0002bdea1901f1011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000242201a00067e23187600010119f04c192bd200011a000249f018201a000242201a00067e2318760001011a000242201a00067e2318760001011a0025cea81971f704001a000141bb041a000249f019138800011a000249f018201a000302590001011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a00330da70101ff"
        );
    }
}