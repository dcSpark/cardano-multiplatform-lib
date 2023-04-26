// re-export the serialization stuff in cml_core as the other modules' serialization.rs
// will expect to find that stuff here.
pub use cml_core::{
    error::Key,
    serialization::{
        fit_sz, CBORReadLen, Deserialize, DeserializeEmbeddedGroup, Serialize,
        SerializeEmbeddedGroup,
    },
};

// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use super::cbor_encodings::*;
use super::*;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_crypto::RawBytesEncoding;
use std::io::{BufRead, Seek, SeekFrom, Write};

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

impl Serialize for PositiveInterval {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag_sz(
            30u64,
            fit_sz(
                30u64,
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
            self.strart,
            fit_sz(
                self.strart,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.strart_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.end,
            fit_sz(
                self.end,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.end_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for PositiveInterval {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (tag, tag_encoding) = raw.tag_sz()?;
        if tag != 30 {
            return Err(DeserializeError::new(
                "PositiveInterval",
                DeserializeFailure::TagMismatch {
                    found: tag,
                    expected: 30,
                },
            ));
        }
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (strart, strart_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("strart"))?;
            let (end, end_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("end"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(PositiveInterval {
                strart,
                end,
                encodings: Some(PositiveIntervalEncoding {
                    len_encoding,
                    tag_encoding: Some(tag_encoding),
                    strart_encoding,
                    end_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("PositiveInterval"))
    }
}

impl Serialize for ProtocolParamUpdate {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(
                    match &self.minfee_a {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.minfee_b {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_block_body_size {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_transaction_size {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_block_header_size {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.key_deposit {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.pool_deposit {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.maximum_epoch {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.n_opt {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.pool_pledge_influence {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.expansion_rate {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.treasury_growth_rate {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.protocol_version {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.min_pool_cost {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.ada_per_utxo_byte {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.cost_models_for_script_languages {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.execution_costs {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_tx_ex_units {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_block_ex_units {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_value_size {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.collateral_percentage {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.max_collateral_inputs {
                        Some(_) => 1,
                        None => 0,
                    },
                    force_canonical,
                ),
        )?;
        let deser_order = self
            .encodings
            .as_ref()
            .filter(|encs| {
                !force_canonical
                    && encs.orig_deser_order.len()
                        == match &self.minfee_a {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.minfee_b {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_block_body_size {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_transaction_size {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_block_header_size {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.key_deposit {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.pool_deposit {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.maximum_epoch {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.n_opt {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.pool_pledge_influence {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.expansion_rate {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.treasury_growth_rate {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.protocol_version {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.min_pool_cost {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.ada_per_utxo_byte {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.cost_models_for_script_languages {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.execution_costs {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_tx_ex_units {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_block_ex_units {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_value_size {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.collateral_percentage {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.max_collateral_inputs {
                            Some(_) => 1,
                            None => 0,
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| {
                vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                ]
            });
        for field_index in deser_order {
            match field_index {
                0 => {
                    if let Some(field) = &self.minfee_a {
                        serializer.write_unsigned_integer_sz(
                            0u64,
                            fit_sz(
                                0u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.minfee_a_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.minfee_a_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                1 => {
                    if let Some(field) = &self.minfee_b {
                        serializer.write_unsigned_integer_sz(
                            1u64,
                            fit_sz(
                                1u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.minfee_b_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.minfee_b_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                2 => {
                    if let Some(field) = &self.max_block_body_size {
                        serializer.write_unsigned_integer_sz(
                            2u64,
                            fit_sz(
                                2u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_block_body_size_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_block_body_size_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                3 => {
                    if let Some(field) = &self.max_transaction_size {
                        serializer.write_unsigned_integer_sz(
                            3u64,
                            fit_sz(
                                3u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_transaction_size_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_transaction_size_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                4 => {
                    if let Some(field) = &self.max_block_header_size {
                        serializer.write_unsigned_integer_sz(
                            4u64,
                            fit_sz(
                                4u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_block_header_size_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_block_header_size_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                5 => {
                    if let Some(field) = &self.key_deposit {
                        serializer.write_unsigned_integer_sz(
                            5u64,
                            fit_sz(
                                5u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.key_deposit_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.key_deposit_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                6 => {
                    if let Some(field) = &self.pool_deposit {
                        serializer.write_unsigned_integer_sz(
                            6u64,
                            fit_sz(
                                6u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.pool_deposit_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.pool_deposit_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                7 => {
                    if let Some(field) = &self.maximum_epoch {
                        serializer.write_unsigned_integer_sz(
                            7u64,
                            fit_sz(
                                7u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.maximum_epoch_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.maximum_epoch_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                8 => {
                    if let Some(field) = &self.n_opt {
                        serializer.write_unsigned_integer_sz(
                            8u64,
                            fit_sz(
                                8u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.n_opt_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.n_opt_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                9 => {
                    if let Some(field) = &self.pool_pledge_influence {
                        serializer.write_unsigned_integer_sz(
                            9u64,
                            fit_sz(
                                9u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.pool_pledge_influence_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                10 => {
                    if let Some(field) = &self.expansion_rate {
                        serializer.write_unsigned_integer_sz(
                            10u64,
                            fit_sz(
                                10u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.expansion_rate_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                11 => {
                    if let Some(field) = &self.treasury_growth_rate {
                        serializer.write_unsigned_integer_sz(
                            11u64,
                            fit_sz(
                                11u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.treasury_growth_rate_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                12 => {
                    if let Some(field) = &self.protocol_version {
                        serializer.write_unsigned_integer_sz(
                            14u64,
                            fit_sz(
                                14u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.protocol_version_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                13 => {
                    if let Some(field) = &self.min_pool_cost {
                        serializer.write_unsigned_integer_sz(
                            16u64,
                            fit_sz(
                                16u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.min_pool_cost_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.min_pool_cost_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                14 => {
                    if let Some(field) = &self.ada_per_utxo_byte {
                        serializer.write_unsigned_integer_sz(
                            17u64,
                            fit_sz(
                                17u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.ada_per_utxo_byte_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.ada_per_utxo_byte_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                15 => {
                    if let Some(field) = &self.cost_models_for_script_languages {
                        serializer.write_unsigned_integer_sz(
                            18u64,
                            fit_sz(
                                18u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.cost_models_for_script_languages_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                16 => {
                    if let Some(field) = &self.execution_costs {
                        serializer.write_unsigned_integer_sz(
                            19u64,
                            fit_sz(
                                19u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.execution_costs_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                17 => {
                    if let Some(field) = &self.max_tx_ex_units {
                        serializer.write_unsigned_integer_sz(
                            20u64,
                            fit_sz(
                                20u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_tx_ex_units_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                18 => {
                    if let Some(field) = &self.max_block_ex_units {
                        serializer.write_unsigned_integer_sz(
                            21u64,
                            fit_sz(
                                21u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_block_ex_units_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                19 => {
                    if let Some(field) = &self.max_value_size {
                        serializer.write_unsigned_integer_sz(
                            22u64,
                            fit_sz(
                                22u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_value_size_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_value_size_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                20 => {
                    if let Some(field) = &self.collateral_percentage {
                        serializer.write_unsigned_integer_sz(
                            23u64,
                            fit_sz(
                                23u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.collateral_percentage_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.collateral_percentage_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                21 => {
                    if let Some(field) = &self.max_collateral_inputs {
                        serializer.write_unsigned_integer_sz(
                            24u64,
                            fit_sz(
                                24u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_collateral_inputs_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        serializer.write_unsigned_integer_sz(
                            *field,
                            fit_sz(
                                *field,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.max_collateral_inputs_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                _ => unreachable!(),
            };
        }
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ProtocolParamUpdate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.map_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        (|| -> Result<_, DeserializeError> {
            let mut orig_deser_order = Vec::new();
            let mut minfee_a_encoding = None;
            let mut minfee_a_key_encoding = None;
            let mut minfee_a = None;
            let mut minfee_b_encoding = None;
            let mut minfee_b_key_encoding = None;
            let mut minfee_b = None;
            let mut max_block_body_size_encoding = None;
            let mut max_block_body_size_key_encoding = None;
            let mut max_block_body_size = None;
            let mut max_transaction_size_encoding = None;
            let mut max_transaction_size_key_encoding = None;
            let mut max_transaction_size = None;
            let mut max_block_header_size_encoding = None;
            let mut max_block_header_size_key_encoding = None;
            let mut max_block_header_size = None;
            let mut key_deposit_encoding = None;
            let mut key_deposit_key_encoding = None;
            let mut key_deposit = None;
            let mut pool_deposit_encoding = None;
            let mut pool_deposit_key_encoding = None;
            let mut pool_deposit = None;
            let mut maximum_epoch_encoding = None;
            let mut maximum_epoch_key_encoding = None;
            let mut maximum_epoch = None;
            let mut n_opt_encoding = None;
            let mut n_opt_key_encoding = None;
            let mut n_opt = None;
            let mut pool_pledge_influence_key_encoding = None;
            let mut pool_pledge_influence = None;
            let mut expansion_rate_key_encoding = None;
            let mut expansion_rate = None;
            let mut treasury_growth_rate_key_encoding = None;
            let mut treasury_growth_rate = None;
            let mut protocol_version_key_encoding = None;
            let mut protocol_version = None;
            let mut min_pool_cost_encoding = None;
            let mut min_pool_cost_key_encoding = None;
            let mut min_pool_cost = None;
            let mut ada_per_utxo_byte_encoding = None;
            let mut ada_per_utxo_byte_key_encoding = None;
            let mut ada_per_utxo_byte = None;
            let mut cost_models_for_script_languages_key_encoding = None;
            let mut cost_models_for_script_languages = None;
            let mut execution_costs_key_encoding = None;
            let mut execution_costs = None;
            let mut max_tx_ex_units_key_encoding = None;
            let mut max_tx_ex_units = None;
            let mut max_block_ex_units_key_encoding = None;
            let mut max_block_ex_units = None;
            let mut max_value_size_encoding = None;
            let mut max_value_size_key_encoding = None;
            let mut max_value_size = None;
            let mut collateral_percentage_encoding = None;
            let mut collateral_percentage_key_encoding = None;
            let mut collateral_percentage = None;
            let mut max_collateral_inputs_encoding = None;
            let mut max_collateral_inputs_key_encoding = None;
            let mut max_collateral_inputs = None;
            let mut read = 0;
            while match len {
                cbor_event::LenSz::Len(n, _) => read < n,
                cbor_event::LenSz::Indefinite => true,
            } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) => {
                            if minfee_a.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_minfee_a, tmp_minfee_a_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("minfee_a"))?;
                            minfee_a = Some(tmp_minfee_a);
                            minfee_a_encoding = tmp_minfee_a_encoding;
                            minfee_a_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        }
                        (1, key_enc) => {
                            if minfee_b.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_minfee_b, tmp_minfee_b_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("minfee_b"))?;
                            minfee_b = Some(tmp_minfee_b);
                            minfee_b_encoding = tmp_minfee_b_encoding;
                            minfee_b_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        }
                        (2, key_enc) => {
                            if max_block_body_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_max_block_body_size, tmp_max_block_body_size_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("max_block_body_size"))?;
                            max_block_body_size = Some(tmp_max_block_body_size);
                            max_block_body_size_encoding = tmp_max_block_body_size_encoding;
                            max_block_body_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        }
                        (3, key_enc) => {
                            if max_transaction_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_max_transaction_size, tmp_max_transaction_size_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("max_transaction_size"))?;
                            max_transaction_size = Some(tmp_max_transaction_size);
                            max_transaction_size_encoding = tmp_max_transaction_size_encoding;
                            max_transaction_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        }
                        (4, key_enc) => {
                            if max_block_header_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            let (tmp_max_block_header_size, tmp_max_block_header_size_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("max_block_header_size"))?;
                            max_block_header_size = Some(tmp_max_block_header_size);
                            max_block_header_size_encoding = tmp_max_block_header_size_encoding;
                            max_block_header_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(4);
                        }
                        (5, key_enc) => {
                            if key_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            let (tmp_key_deposit, tmp_key_deposit_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("key_deposit"))?;
                            key_deposit = Some(tmp_key_deposit);
                            key_deposit_encoding = tmp_key_deposit_encoding;
                            key_deposit_key_encoding = Some(key_enc);
                            orig_deser_order.push(5);
                        }
                        (6, key_enc) => {
                            if pool_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            let (tmp_pool_deposit, tmp_pool_deposit_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("pool_deposit"))?;
                            pool_deposit = Some(tmp_pool_deposit);
                            pool_deposit_encoding = tmp_pool_deposit_encoding;
                            pool_deposit_key_encoding = Some(key_enc);
                            orig_deser_order.push(6);
                        }
                        (7, key_enc) => {
                            if maximum_epoch.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            let (tmp_maximum_epoch, tmp_maximum_epoch_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("maximum_epoch"))?;
                            maximum_epoch = Some(tmp_maximum_epoch);
                            maximum_epoch_encoding = tmp_maximum_epoch_encoding;
                            maximum_epoch_key_encoding = Some(key_enc);
                            orig_deser_order.push(7);
                        }
                        (8, key_enc) => {
                            if n_opt.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(8)).into());
                            }
                            let (tmp_n_opt, tmp_n_opt_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("n_opt"))?;
                            n_opt = Some(tmp_n_opt);
                            n_opt_encoding = tmp_n_opt_encoding;
                            n_opt_key_encoding = Some(key_enc);
                            orig_deser_order.push(8);
                        }
                        (9, key_enc) => {
                            if pool_pledge_influence.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(9)).into());
                            }
                            let tmp_pool_pledge_influence = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Rational::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("pool_pledge_influence"))?;
                            pool_pledge_influence = Some(tmp_pool_pledge_influence);
                            pool_pledge_influence_key_encoding = Some(key_enc);
                            orig_deser_order.push(9);
                        }
                        (10, key_enc) => {
                            if expansion_rate.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(10)).into());
                            }
                            let tmp_expansion_rate = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                UnitInterval::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("expansion_rate"))?;
                            expansion_rate = Some(tmp_expansion_rate);
                            expansion_rate_key_encoding = Some(key_enc);
                            orig_deser_order.push(10);
                        }
                        (11, key_enc) => {
                            if treasury_growth_rate.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(11)).into());
                            }
                            let tmp_treasury_growth_rate = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                UnitInterval::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("treasury_growth_rate"))?;
                            treasury_growth_rate = Some(tmp_treasury_growth_rate);
                            treasury_growth_rate_key_encoding = Some(key_enc);
                            orig_deser_order.push(11);
                        }
                        (14, key_enc) => {
                            if protocol_version.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(14)).into());
                            }
                            let tmp_protocol_version = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ProtocolVersionStruct::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("protocol_version"))?;
                            protocol_version = Some(tmp_protocol_version);
                            protocol_version_key_encoding = Some(key_enc);
                            orig_deser_order.push(12);
                        }
                        (16, key_enc) => {
                            if min_pool_cost.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(16)).into());
                            }
                            let (tmp_min_pool_cost, tmp_min_pool_cost_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("min_pool_cost"))?;
                            min_pool_cost = Some(tmp_min_pool_cost);
                            min_pool_cost_encoding = tmp_min_pool_cost_encoding;
                            min_pool_cost_key_encoding = Some(key_enc);
                            orig_deser_order.push(13);
                        }
                        (17, key_enc) => {
                            if ada_per_utxo_byte.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(17)).into());
                            }
                            let (tmp_ada_per_utxo_byte, tmp_ada_per_utxo_byte_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("ada_per_utxo_byte"))?;
                            ada_per_utxo_byte = Some(tmp_ada_per_utxo_byte);
                            ada_per_utxo_byte_encoding = tmp_ada_per_utxo_byte_encoding;
                            ada_per_utxo_byte_key_encoding = Some(key_enc);
                            orig_deser_order.push(14);
                        }
                        (18, key_enc) => {
                            if cost_models_for_script_languages.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(18)).into());
                            }
                            let tmp_cost_models_for_script_languages =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    CostModels::deserialize(raw)
                                })()
                                .map_err(|e| e.annotate("cost_models_for_script_languages"))?;
                            cost_models_for_script_languages =
                                Some(tmp_cost_models_for_script_languages);
                            cost_models_for_script_languages_key_encoding = Some(key_enc);
                            orig_deser_order.push(15);
                        }
                        (19, key_enc) => {
                            if execution_costs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(19)).into());
                            }
                            let tmp_execution_costs = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ExUnitPrices::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("execution_costs"))?;
                            execution_costs = Some(tmp_execution_costs);
                            execution_costs_key_encoding = Some(key_enc);
                            orig_deser_order.push(16);
                        }
                        (20, key_enc) => {
                            if max_tx_ex_units.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(20)).into());
                            }
                            let tmp_max_tx_ex_units = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ExUnits::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("max_tx_ex_units"))?;
                            max_tx_ex_units = Some(tmp_max_tx_ex_units);
                            max_tx_ex_units_key_encoding = Some(key_enc);
                            orig_deser_order.push(17);
                        }
                        (21, key_enc) => {
                            if max_block_ex_units.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(21)).into());
                            }
                            let tmp_max_block_ex_units = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ExUnits::deserialize(raw)
                            })()
                            .map_err(|e| e.annotate("max_block_ex_units"))?;
                            max_block_ex_units = Some(tmp_max_block_ex_units);
                            max_block_ex_units_key_encoding = Some(key_enc);
                            orig_deser_order.push(18);
                        }
                        (22, key_enc) => {
                            if max_value_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(22)).into());
                            }
                            let (tmp_max_value_size, tmp_max_value_size_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("max_value_size"))?;
                            max_value_size = Some(tmp_max_value_size);
                            max_value_size_encoding = tmp_max_value_size_encoding;
                            max_value_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(19);
                        }
                        (23, key_enc) => {
                            if collateral_percentage.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(23)).into());
                            }
                            let (tmp_collateral_percentage, tmp_collateral_percentage_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("collateral_percentage"))?;
                            collateral_percentage = Some(tmp_collateral_percentage);
                            collateral_percentage_encoding = tmp_collateral_percentage_encoding;
                            collateral_percentage_key_encoding = Some(key_enc);
                            orig_deser_order.push(20);
                        }
                        (24, key_enc) => {
                            if max_collateral_inputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(24)).into());
                            }
                            let (tmp_max_collateral_inputs, tmp_max_collateral_inputs_encoding) =
                                (|| -> Result<_, DeserializeError> {
                                    read_len.read_elems(1)?;
                                    raw.unsigned_integer_sz()
                                        .map(|(x, enc)| (x, Some(enc)))
                                        .map_err(Into::<DeserializeError>::into)
                                })()
                                .map_err(|e| e.annotate("max_collateral_inputs"))?;
                            max_collateral_inputs = Some(tmp_max_collateral_inputs);
                            max_collateral_inputs_encoding = tmp_max_collateral_inputs_encoding;
                            max_collateral_inputs_key_encoding = Some(key_enc);
                            orig_deser_order.push(21);
                        }
                        (unknown_key, _enc) => {
                            return Err(
                                DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()
                            )
                        }
                    },
                    cbor_event::Type::Text => {
                        return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into())
                    }
                    cbor_event::Type::Special => match len {
                        cbor_event::LenSz::Len(_, _) => {
                            return Err(DeserializeFailure::BreakInDefiniteLen.into())
                        }
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            cbor_event::Special::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => {
                        return Err(DeserializeFailure::UnexpectedKeyType(other_type).into())
                    }
                }
                read += 1;
            }
            read_len.finish()?;
            Ok(Self {
                minfee_a,
                minfee_b,
                max_block_body_size,
                max_transaction_size,
                max_block_header_size,
                key_deposit,
                pool_deposit,
                maximum_epoch,
                n_opt,
                pool_pledge_influence,
                expansion_rate,
                treasury_growth_rate,
                protocol_version,
                min_pool_cost,
                ada_per_utxo_byte,
                cost_models_for_script_languages,
                execution_costs,
                max_tx_ex_units,
                max_block_ex_units,
                max_value_size,
                collateral_percentage,
                max_collateral_inputs,
                encodings: Some(ProtocolParamUpdateEncoding {
                    len_encoding,
                    orig_deser_order,
                    minfee_a_key_encoding,
                    minfee_a_encoding,
                    minfee_b_key_encoding,
                    minfee_b_encoding,
                    max_block_body_size_key_encoding,
                    max_block_body_size_encoding,
                    max_transaction_size_key_encoding,
                    max_transaction_size_encoding,
                    max_block_header_size_key_encoding,
                    max_block_header_size_encoding,
                    key_deposit_key_encoding,
                    key_deposit_encoding,
                    pool_deposit_key_encoding,
                    pool_deposit_encoding,
                    maximum_epoch_key_encoding,
                    maximum_epoch_encoding,
                    n_opt_key_encoding,
                    n_opt_encoding,
                    pool_pledge_influence_key_encoding,
                    expansion_rate_key_encoding,
                    treasury_growth_rate_key_encoding,
                    protocol_version_key_encoding,
                    min_pool_cost_key_encoding,
                    min_pool_cost_encoding,
                    ada_per_utxo_byte_key_encoding,
                    ada_per_utxo_byte_encoding,
                    cost_models_for_script_languages_key_encoding,
                    execution_costs_key_encoding,
                    max_tx_ex_units_key_encoding,
                    max_block_ex_units_key_encoding,
                    max_value_size_key_encoding,
                    max_value_size_encoding,
                    collateral_percentage_key_encoding,
                    collateral_percentage_encoding,
                    max_collateral_inputs_key_encoding,
                    max_collateral_inputs_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("ProtocolParamUpdate"))
    }
}

impl Serialize for ProtocolVersionStruct {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(2, force_canonical),
        )?;
        self.protocol_version
            .serialize_as_embedded_group(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for ProtocolVersionStruct {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let protocol_version =
                ProtocolVersion::deserialize_as_embedded_group(raw, &mut read_len, len)
                    .map_err(|e: DeserializeError| e.annotate("protocol_version"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ProtocolVersionStruct {
                protocol_version,
                encodings: Some(ProtocolVersionStructEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("ProtocolVersionStruct"))
    }
}

impl Serialize for Rational {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag_sz(
            30u64,
            fit_sz(
                30u64,
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
            self.numerator,
            fit_sz(
                self.numerator,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.numerator_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.denominator,
            fit_sz(
                self.denominator,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.denominator_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for Rational {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (tag, tag_encoding) = raw.tag_sz()?;
        if tag != 30 {
            return Err(DeserializeError::new(
                "Rational",
                DeserializeFailure::TagMismatch {
                    found: tag,
                    expected: 30,
                },
            ));
        }
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (numerator, numerator_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("numerator"))?;
            let (denominator, denominator_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("denominator"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Rational {
                numerator,
                denominator,
                encodings: Some(RationalEncoding {
                    len_encoding,
                    tag_encoding: Some(tag_encoding),
                    numerator_encoding,
                    denominator_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("Rational"))
    }
}

impl Serialize for Script {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        match self {
            Script::Native {
                script,
                len_encoding,
                tag_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    0u64,
                    fit_sz(0u64, *tag_encoding, force_canonical),
                )?;
                script.serialize(serializer, force_canonical)?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            Script::PlutusV1 {
                script,
                len_encoding,
                tag_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    1u64,
                    fit_sz(1u64, *tag_encoding, force_canonical),
                )?;
                script.serialize(serializer, force_canonical)?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
            Script::PlutusV2 {
                script,
                len_encoding,
                tag_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    2u64,
                    fit_sz(2u64, *tag_encoding, force_canonical),
                )?;
                script.serialize(serializer, force_canonical)?;
                len_encoding.end(serializer, force_canonical)?;
                Ok(serializer)
            }
        }
    }
}

impl Deserialize for Script {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array_sz()?;
            let len_encoding: LenEncoding = len.into();
            let mut read_len = CBORReadLen::new(len);
            let initial_position = raw.as_mut_ref().seek(SeekFrom::Current(0)).unwrap();
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let tag_encoding = (|| -> Result<_, DeserializeError> {
                    let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                    if tag_value != 0 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(tag_value),
                            expected: Key::Uint(0),
                        }
                        .into());
                    }
                    Ok(Some(tag_encoding))
                })()
                .map_err(|e| e.annotate("tag"))?;
                let script = NativeScript::deserialize(raw)
                    .map_err(|e: DeserializeError| e.annotate("script"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::Native {
                    script,
                    len_encoding,
                    tag_encoding,
                })
            })(raw)
            {
                Ok(variant) => return Ok(variant),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let tag_encoding = (|| -> Result<_, DeserializeError> {
                    let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                    if tag_value != 1 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(tag_value),
                            expected: Key::Uint(1),
                        }
                        .into());
                    }
                    Ok(Some(tag_encoding))
                })()
                .map_err(|e| e.annotate("tag"))?;
                let script = PlutusV1Script::deserialize(raw)
                    .map_err(|e: DeserializeError| e.annotate("script"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::PlutusV1 {
                    script,
                    len_encoding,
                    tag_encoding,
                })
            })(raw)
            {
                Ok(variant) => return Ok(variant),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let tag_encoding = (|| -> Result<_, DeserializeError> {
                    let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                    if tag_value != 2 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(tag_value),
                            expected: Key::Uint(2),
                        }
                        .into());
                    }
                    Ok(Some(tag_encoding))
                })()
                .map_err(|e| e.annotate("tag"))?;
                let script = PlutusV2Script::deserialize(raw)
                    .map_err(|e: DeserializeError| e.annotate("script"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::PlutusV2 {
                    script,
                    len_encoding,
                    tag_encoding,
                })
            })(raw)
            {
                Ok(variant) => return Ok(variant),
                Err(_) => raw
                    .as_mut_ref()
                    .seek(SeekFrom::Start(initial_position))
                    .unwrap(),
            };
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Err(DeserializeError::new(
                "Script",
                DeserializeFailure::NoVariantMatched,
            ))
        })()
        .map_err(|e| e.annotate("Script"))
    }
}

impl Serialize for UnitInterval {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_tag_sz(
            30u64,
            fit_sz(
                30u64,
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
            self.start,
            fit_sz(
                self.start,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.start_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        serializer.write_unsigned_integer_sz(
            self.end,
            fit_sz(
                self.end,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.end_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for UnitInterval {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let (tag, tag_encoding) = raw.tag_sz()?;
        if tag != 30 {
            return Err(DeserializeError::new(
                "UnitInterval",
                DeserializeFailure::TagMismatch {
                    found: tag,
                    expected: 30,
                },
            ));
        }
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (start, start_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("start"))?;
            let (end, end_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("end"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(UnitInterval {
                start,
                end,
                encodings: Some(UnitIntervalEncoding {
                    len_encoding,
                    tag_encoding: Some(tag_encoding),
                    start_encoding,
                    end_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("UnitInterval"))
    }
}

impl Serialize for Update {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
        force_canonical: bool,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.len_encoding)
                .unwrap_or_default()
                .to_len_sz(2, force_canonical),
        )?;
        serializer.write_map_sz(
            self.encodings
                .as_ref()
                .map(|encs| encs.proposed_protocol_parameter_updates_encoding)
                .unwrap_or_default()
                .to_len_sz(
                    self.proposed_protocol_parameter_updates.len() as u64,
                    force_canonical,
                ),
        )?;
        let mut key_order = self
            .proposed_protocol_parameter_updates
            .iter()
            .map(|(k, v)| {
                let mut buf = cbor_event::se::Serializer::new_vec();
                let proposed_protocol_parameter_updates_key_encoding = self
                    .encodings
                    .as_ref()
                    .and_then(|encs| {
                        encs.proposed_protocol_parameter_updates_key_encodings
                            .get(k)
                    })
                    .cloned()
                    .unwrap_or_default();
                buf.write_bytes_sz(
                    &k.to_raw_bytes(),
                    proposed_protocol_parameter_updates_key_encoding
                        .to_str_len_sz(k.to_raw_bytes().len() as u64, force_canonical),
                )?;
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
        self.encodings
            .as_ref()
            .map(|encs| encs.proposed_protocol_parameter_updates_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)?;
        serializer.write_unsigned_integer_sz(
            self.epoch,
            fit_sz(
                self.epoch,
                self.encodings
                    .as_ref()
                    .map(|encs| encs.epoch_encoding)
                    .unwrap_or_default(),
                force_canonical,
            ),
        )?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for Update {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let (
                proposed_protocol_parameter_updates,
                proposed_protocol_parameter_updates_encoding,
                proposed_protocol_parameter_updates_key_encodings,
            ) = (|| -> Result<_, DeserializeError> {
                let mut proposed_protocol_parameter_updates_table = OrderedHashMap::new();
                let proposed_protocol_parameter_updates_len = raw.map_sz()?;
                let proposed_protocol_parameter_updates_encoding =
                    proposed_protocol_parameter_updates_len.into();
                let mut proposed_protocol_parameter_updates_key_encodings = BTreeMap::new();
                while match proposed_protocol_parameter_updates_len {
                    cbor_event::LenSz::Len(n, _) => {
                        (proposed_protocol_parameter_updates_table.len() as u64) < n
                    }
                    cbor_event::LenSz::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let (
                        proposed_protocol_parameter_updates_key,
                        proposed_protocol_parameter_updates_key_encoding,
                    ) = raw
                        .bytes_sz()
                        .map_err(Into::<DeserializeError>::into)
                        .and_then(|(bytes, enc)| {
                            GenesisHash::from_raw_bytes(&bytes)
                                .map(|bytes| (bytes, StringEncoding::from(enc)))
                                .map_err(|e| {
                                    DeserializeFailure::InvalidStructure(Box::new(e)).into()
                                })
                        })?;
                    let proposed_protocol_parameter_updates_value =
                        ProtocolParamUpdate::deserialize(raw)?;
                    if proposed_protocol_parameter_updates_table
                        .insert(
                            proposed_protocol_parameter_updates_key.clone(),
                            proposed_protocol_parameter_updates_value,
                        )
                        .is_some()
                    {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                    proposed_protocol_parameter_updates_key_encodings.insert(
                        proposed_protocol_parameter_updates_key.clone(),
                        proposed_protocol_parameter_updates_key_encoding,
                    );
                }
                Ok((
                    proposed_protocol_parameter_updates_table,
                    proposed_protocol_parameter_updates_encoding,
                    proposed_protocol_parameter_updates_key_encodings,
                ))
            })()
            .map_err(|e| e.annotate("proposed_protocol_parameter_updates"))?;
            let (epoch, epoch_encoding) = raw
                .unsigned_integer_sz()
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(Into::<DeserializeError>::into)
                .map_err(|e: DeserializeError| e.annotate("epoch"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Update {
                proposed_protocol_parameter_updates,
                epoch,
                encodings: Some(UpdateEncoding {
                    len_encoding,
                    proposed_protocol_parameter_updates_encoding,
                    proposed_protocol_parameter_updates_key_encodings,
                    epoch_encoding,
                }),
            })
        })()
        .map_err(|e| e.annotate("Update"))
    }
}
