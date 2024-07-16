use super::{CostModels, Language, LegacyRedeemer, RedeemerKey, RedeemerVal, Redeemers};
use super::{ExUnits, PlutusData, PlutusV1Script, PlutusV2Script, PlutusV3Script};
use crate::crypto::hash::{hash_script, ScriptHashNamespace};
use crate::json::plutus_datums::{
    decode_plutus_datum_to_json_value, encode_json_value_to_plutus_datum,
    CardanoNodePlutusDatumSchema,
};
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_core::error::*;
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::serialization::*;
use cml_crypto::ScriptHash;
use itertools::Itertools;
use std::convert::{TryFrom, TryInto};
use std::io::{BufRead, Seek, Write};

impl serde::Serialize for PlutusData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let json_value =
            decode_plutus_datum_to_json_value(self, CardanoNodePlutusDatumSchema::DetailedSchema)
                .expect("DetailedSchema can represent everything");
        serde_json::Value::from(json_value).serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for PlutusData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let serde_json_value =
            <serde_json::Value as serde::de::Deserialize>::deserialize(deserializer)?;
        let json_value = crate::json::json_serialize::Value::from(serde_json_value);
        encode_json_value_to_plutus_datum(
            json_value.clone(),
            CardanoNodePlutusDatumSchema::DetailedSchema,
        )
        .map_err(|_e| {
            serde::de::Error::invalid_value(
                (&json_value).into(),
                &"invalid plutus datum (cardano-node JSON format)",
            )
        })
    }
}

impl schemars::JsonSchema for PlutusData {
    fn schema_name() -> String {
        String::from("PlutusData")
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        schemars::schema::Schema::from(schemars::schema::SchemaObject::new_ref(
            "PlutusData".to_owned(),
        ))
    }

    fn is_referenceable() -> bool {
        true
    }
}

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
        } else if (7..=127).contains(&alt) {
            Some(1280 - 7 + alt)
        } else {
            None
        }
    }

    // None -> General tag(=102) OR Invalid CBOR tag for this scheme
    fn compact_cbor_tag_to_alternative(cbor_tag: u64) -> Option<u64> {
        if (121..=127).contains(&cbor_tag) {
            Some(cbor_tag - 121)
        } else if (1280..=1400).contains(&cbor_tag) {
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
            Some(compact_tag)
                if self
                    .encodings
                    .as_ref()
                    .map(|encs| encs.prefer_compact)
                    .unwrap_or(true) =>
            {
                // compact form
                serializer.write_tag_sz(
                    compact_tag,
                    fit_sz(
                        compact_tag,
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
            }
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
            }
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
                        Err(DeserializeFailure::TagMismatch {
                            found: tag,
                            expected: Self::GENERAL_FORM_TAG,
                        }
                        .into())
                    }
                }
            }
        })()
        .map_err(|e| e.annotate("ConstrPlutusData"))
    }
}

impl CostModels {
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
        // Due to PlutusV1 bug we can't re-use the generated serialization code
        // as it requires it to be encoded as CBOR bytes in CBOR
        serializer.write_map(cbor_event::Len::Len(self.inner.len() as u64))?;
        for (language, costs) in self.inner.iter() {
            match (*language).try_into() {
                Ok(Language::PlutusV1) => {
                    // For PlutusV1 (language id 0), the language view is the following:
                    //   * the value of costmdls map at key 0 is encoded as an indefinite length
                    //     list and the result is encoded as a bytestring. (our apologies)
                    //   * the language ID tag is also encoded twice. first as a uint then as
                    //     a bytestring. (our apologies)
                    let v1_key_canonical_bytes = [0];
                    serializer.write_bytes(v1_key_canonical_bytes)?;
                    // Due to a bug in the cardano-node input-output-hk/cardano-ledger-specs/issues/2512
                    // we must use indefinite length serialization in this inner bytestring to match it
                    let mut cost_model_serializer = Serializer::new_vec();
                    cost_model_serializer.write_array(cbor_event::Len::Indefinite)?;
                    for cost in costs {
                        if *cost >= 0 {
                            cost_model_serializer.write_unsigned_integer(cost.unsigned_abs())?;
                        } else {
                            cost_model_serializer.write_negative_integer(*cost)?;
                        }
                    }
                    cost_model_serializer.write_special(cbor_event::Special::Break)?;
                    serializer.write_bytes(cost_model_serializer.finalize())?;
                }
                _ => {
                    // For PlutusV2 (language id 1), the language view is the following:
                    //    * the value of costmdls map at key 1 is encoded as an definite length list.
                    // For PlutusV3 (language id 2), the language view is the following:
                    //   * the value of costmdls map at key 2 is encoded as a definite length list.
                    //
                    // We will assume all other languages also follow this and that the non-canonical
                    // encoding is just a bug pertaining to Plutus V1
                    serializer.write_unsigned_integer(*language)?;
                    serializer.write_array(cbor_event::Len::Len(costs.len() as u64))?;
                    for cost in costs {
                        if *cost >= 0 {
                            serializer.write_unsigned_integer(cost.unsigned_abs())?;
                        } else {
                            serializer.write_negative_integer(*cost)?;
                        }
                    }
                }
            }
        }
        Ok(serializer.finalize())
    }
}

impl AsRef<OrderedHashMap<u64, Vec<i64>>> for CostModels {
    fn as_ref(&self) -> &OrderedHashMap<u64, Vec<i64>> {
        &self.inner
    }
}

impl AsMut<OrderedHashMap<u64, Vec<i64>>> for CostModels {
    fn as_mut(&mut self) -> &mut OrderedHashMap<u64, Vec<i64>> {
        &mut self.inner
    }
}

/// Version-agnostic Plutus script
#[derive(Clone, Debug)]
pub enum PlutusScript {
    PlutusV1(PlutusV1Script),
    PlutusV2(PlutusV2Script),
    PlutusV3(PlutusV3Script),
}

impl PlutusScript {
    pub fn hash(&self) -> ScriptHash {
        match &self {
            Self::PlutusV1(script) => script.hash(),
            Self::PlutusV2(script) => script.hash(),
            Self::PlutusV3(script) => script.hash(),
        }
    }

    pub fn version(&self) -> Language {
        match self {
            Self::PlutusV1(_) => Language::PlutusV1,
            Self::PlutusV2(_) => Language::PlutusV2,
            Self::PlutusV3(_) => Language::PlutusV3,
        }
    }
}

impl From<PlutusV1Script> for PlutusScript {
    fn from(script: PlutusV1Script) -> Self {
        Self::PlutusV1(script)
    }
}

impl From<PlutusV2Script> for PlutusScript {
    fn from(script: PlutusV2Script) -> Self {
        Self::PlutusV2(script)
    }
}

impl From<PlutusV3Script> for PlutusScript {
    fn from(script: PlutusV3Script) -> Self {
        Self::PlutusV3(script)
    }
}

impl PlutusV1Script {
    pub fn hash(&self) -> ScriptHash {
        hash_script(ScriptHashNamespace::PlutusV1, self.to_raw_bytes())
    }
}

impl PlutusV2Script {
    pub fn hash(&self) -> ScriptHash {
        hash_script(ScriptHashNamespace::PlutusV2, self.to_raw_bytes())
    }
}

impl PlutusV3Script {
    pub fn hash(&self) -> ScriptHash {
        hash_script(ScriptHashNamespace::PlutusV3, self.to_raw_bytes())
    }
}

impl RawBytesEncoding for PlutusV1Script {
    fn to_raw_bytes(&self) -> &[u8] {
        self.inner.as_ref()
    }

    fn from_raw_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Ok(Self::new(bytes.to_vec()))
    }
}

impl RawBytesEncoding for PlutusV2Script {
    fn to_raw_bytes(&self) -> &[u8] {
        self.inner.as_ref()
    }

    fn from_raw_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Ok(Self::new(bytes.to_vec()))
    }
}

impl RawBytesEncoding for PlutusV3Script {
    fn to_raw_bytes(&self) -> &[u8] {
        self.inner.as_ref()
    }

    fn from_raw_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Ok(Self::new(bytes.to_vec()))
    }
}

impl ExUnits {
    pub fn checked_add(&self, other: &ExUnits) -> Result<ExUnits, ArithmeticError> {
        let mem = self
            .mem
            .checked_add(other.mem)
            .ok_or(ArithmeticError::IntegerOverflow)?;
        let step = self
            .steps
            .checked_add(other.steps)
            .ok_or(ArithmeticError::IntegerOverflow)?;
        Ok(ExUnits::new(mem, step))
    }

    /// used to create a dummy ExUnits that takes up the maximum size possible in cbor to provide an upper bound on tx size
    pub fn dummy() -> ExUnits {
        ExUnits::new(u64::MAX, u64::MAX)
    }
}

pub fn compute_total_ex_units(redeemers: &[LegacyRedeemer]) -> Result<ExUnits, ArithmeticError> {
    let mut sum = ExUnits::new(0, 0);
    for redeemer in redeemers {
        sum = sum.checked_add(&redeemer.ex_units)?;
    }
    Ok(sum)
}

impl TryFrom<u64> for Language {
    type Error = DeserializeError;

    fn try_from(language: u64) -> Result<Self, Self::Error> {
        match language {
            0 => Ok(Language::PlutusV1),
            1 => Ok(Language::PlutusV2),
            2 => Ok(Language::PlutusV3),
            _ => Err(DeserializeFailure::OutOfRange {
                found: language as usize,
                min: 0,
                max: 2,
            }
            .into()),
        }
    }
}

impl From<Language> for u64 {
    fn from(language: Language) -> u64 {
        match language {
            Language::PlutusV1 => 0,
            Language::PlutusV2 => 1,
            Language::PlutusV3 => 2,
        }
    }
}

#[derive(Clone, Debug, Default, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PlutusMap {
    // possibly duplicates (very rare - only found on testnet)
    pub entries: Vec<(PlutusData, PlutusData)>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encoding: LenEncoding,
}

impl PlutusMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Replaces all datums of a given key, if any exist.
    pub fn set(&mut self, key: PlutusData, value: PlutusData) {
        self.entries.retain(|(k, _)| *k != key);
        self.entries.push((key, value));
    }

    /// Gets the plutus datum corresponding to a given key, if it exists.
    /// Note: In the case of duplicate keys this only returns the first datum.
    /// This is an extremely rare occurence on-chain but can happen.
    pub fn get(&self, key: &PlutusData) -> Option<&PlutusData> {
        self.entries
            .iter()
            .find(|(k, _)| *k == *key)
            .map(|(_, value)| value)
    }

    /// In the extremely unlikely situation there are duplicate keys, this gets all of a single key
    pub fn get_all(&self, key: &PlutusData) -> Option<Vec<&PlutusData>> {
        let matches = self
            .entries
            .iter()
            .filter_map(|(k, v)| if *k == *key { Some(v) } else { None })
            .collect::<Vec<_>>();
        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }
}

impl Serialize for PlutusMap {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(
            self.encoding
                .to_len_sz(self.entries.len() as u64, force_canonical),
        )?;
        let mut key_order = self
            .entries
            .iter()
            .map(|(k, v)| {
                let mut buf = cbor_event::se::Serializer::new_vec();
                k.serialize(&mut buf, force_canonical)?;
                Ok((buf.finalize(), k, v))
            })
            .collect::<Result<Vec<(Vec<u8>, &_, &_)>, cbor_event::Error>>()?;
        if force_canonical {
            key_order.sort_by(|(lhs_bytes, _, _), (rhs_bytes, _, _)| {
                match lhs_bytes.len().cmp(&rhs_bytes.len()) {
                    std::cmp::Ordering::Equal => lhs_bytes.cmp(rhs_bytes),
                    diff_ord => diff_ord,
                }
            });
        }
        for (key_bytes, _key, value) in key_order {
            serializer.write_raw_bytes(&key_bytes)?;
            value.serialize(serializer, force_canonical)?;
        }
        self.encoding.end(serializer, force_canonical)
    }
}

impl Deserialize for PlutusMap {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let mut entries = Vec::new();
            let map_len = raw.map_sz()?;
            let encoding = map_len.into();
            while match map_len {
                cbor_event::LenSz::Len(n, _) => (entries.len() as u64) < n,
                cbor_event::LenSz::Indefinite => true,
            } {
                if raw.cbor_type()? == cbor_event::Type::Special {
                    assert_eq!(raw.special()?, cbor_event::Special::Break);
                    break;
                }
                let map_key = PlutusData::deserialize(raw)?;
                let map_value = PlutusData::deserialize(raw)?;
                entries.push((map_key, map_value));
            }
            Ok(Self { entries, encoding })
        })()
        .map_err(|e| e.annotate("PlutusMap"))
    }
}

impl Redeemers {
    pub fn to_flat_format(self) -> Vec<LegacyRedeemer> {
        match self {
            Self::ArrLegacyRedeemer {
                arr_legacy_redeemer,
                ..
            } => arr_legacy_redeemer,
            Self::MapRedeemerKeyToRedeemerVal {
                map_redeemer_key_to_redeemer_val,
                ..
            } => map_redeemer_key_to_redeemer_val
                .iter()
                .map(|(k, v)| {
                    LegacyRedeemer::new(k.tag, k.index, v.data.clone(), v.ex_units.clone())
                })
                .collect_vec(),
        }
    }

    pub fn to_map_format(self) -> OrderedHashMap<RedeemerKey, RedeemerVal> {
        match self {
            Self::ArrLegacyRedeemer {
                arr_legacy_redeemer,
                ..
            } => arr_legacy_redeemer
                .into_iter()
                .map(|r| {
                    (
                        RedeemerKey::new(r.tag, r.index),
                        RedeemerVal::new(r.data, r.ex_units),
                    )
                })
                .collect(),
            Self::MapRedeemerKeyToRedeemerVal {
                map_redeemer_key_to_redeemer_val,
                ..
            } => map_redeemer_key_to_redeemer_val,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::ArrLegacyRedeemer {
                arr_legacy_redeemer,
                ..
            } => arr_legacy_redeemer.is_empty(),
            Self::MapRedeemerKeyToRedeemerVal {
                map_redeemer_key_to_redeemer_val,
                ..
            } => map_redeemer_key_to_redeemer_val.is_empty(),
        }
    }

    pub fn extend(&mut self, other: Self) {
        match self {
            Self::ArrLegacyRedeemer {
                arr_legacy_redeemer,
                ..
            } => arr_legacy_redeemer.extend(other.to_flat_format()),
            Self::MapRedeemerKeyToRedeemerVal {
                map_redeemer_key_to_redeemer_val,
                ..
            } => {
                for (k, v) in other.to_map_format().take() {
                    map_redeemer_key_to_redeemer_val.insert(k, v);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plutus::{CostModels, Language};

    #[test]
    pub fn test_cost_model() {
        let v1_costs = vec![
            197209, 0, 1, 1, 396231, 621, 0, 1, 150000, 1000, 0, 1, 150000, 32, 2477736, 29175, 4,
            29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 29773, 100, 100, 100,
            29773, 100, 150000, 32, 150000, 32, 150000, 32, 150000, 1000, 0, 1, 150000, 32, 150000,
            1000, 0, 8, 148000, 425507, 118, 0, 1, 1, 150000, 1000, 0, 8, 150000, 112536, 247, 1,
            150000, 10000, 1, 136542, 1326, 1, 1000, 150000, 1000, 1, 150000, 32, 150000, 32,
            150000, 32, 1, 1, 150000, 1, 150000, 4, 103599, 248, 1, 103599, 248, 1, 145276, 1366,
            1, 179690, 497, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000,
            32, 148000, 425507, 118, 0, 1, 1, 61516, 11218, 0, 1, 150000, 32, 148000, 425507, 118,
            0, 1, 1, 148000, 425507, 118, 0, 1, 1, 2477736, 29175, 4, 0, 82363, 4, 150000, 5000, 0,
            1, 150000, 32, 197209, 0, 1, 1, 150000, 32, 150000, 32, 150000, 32, 150000, 32, 150000,
            32, 150000, 32, 150000, 32, 3345831, 1, 1,
        ];
        let mut cms = CostModels::default();
        cms.inner.insert(Language::PlutusV1.into(), v1_costs);
        assert_eq!(
            hex::encode(cms.language_views_encoding().unwrap()),
            "a141005901d59f1a000302590001011a00060bc719026d00011a000249f01903e800011a000249f018201a0025cea81971f70419744d186419744d186419744d186419744d186419744d186419744d18641864186419744d18641a000249f018201a000249f018201a000249f018201a000249f01903e800011a000249f018201a000249f01903e800081a000242201a00067e2318760001011a000249f01903e800081a000249f01a0001b79818f7011a000249f0192710011a0002155e19052e011903e81a000249f01903e8011a000249f018201a000249f018201a000249f0182001011a000249f0011a000249f0041a000194af18f8011a000194af18f8011a0002377c190556011a0002bdea1901f1011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000242201a00067e23187600010119f04c192bd200011a000249f018201a000242201a00067e2318760001011a000242201a00067e2318760001011a0025cea81971f704001a000141bb041a000249f019138800011a000249f018201a000302590001011a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a000249f018201a00330da70101ff"
        );
    }
}
