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

use std::io::{BufRead, Seek, SeekFrom, Write};

impl Serialize for DRepVotingThresholds {
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
                .to_len_sz(10, force_canonical),
        )?;
        self.motion_no_confidence
            .serialize(serializer, force_canonical)?;
        self.committee_normal
            .serialize(serializer, force_canonical)?;
        self.committee_no_confidence
            .serialize(serializer, force_canonical)?;
        self.update_constitution
            .serialize(serializer, force_canonical)?;
        self.hard_fork_initiation
            .serialize(serializer, force_canonical)?;
        self.pp_network_group
            .serialize(serializer, force_canonical)?;
        self.pp_economic_group
            .serialize(serializer, force_canonical)?;
        self.pp_technical_group
            .serialize(serializer, force_canonical)?;
        self.pp_governance_group
            .serialize(serializer, force_canonical)?;
        self.treasury_withdrawal
            .serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for DRepVotingThresholds {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(10)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let motion_no_confidence = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("motion_no_confidence"))?;
            let committee_normal = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("committee_normal"))?;
            let committee_no_confidence = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("committee_no_confidence"))?;
            let update_constitution = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("update_constitution"))?;
            let hard_fork_initiation = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("hard_fork_initiation"))?;
            let pp_network_group = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("pp_network_group"))?;
            let pp_economic_group = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("pp_economic_group"))?;
            let pp_technical_group = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("pp_technical_group"))?;
            let pp_governance_group = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("pp_governance_group"))?;
            let treasury_withdrawal = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("treasury_withdrawal"))?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(DRepVotingThresholds {
                motion_no_confidence,
                committee_normal,
                committee_no_confidence,
                update_constitution,
                hard_fork_initiation,
                pp_network_group,
                pp_economic_group,
                pp_technical_group,
                pp_governance_group,
                treasury_withdrawal,
                encodings: Some(DRepVotingThresholdsEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("DRepVotingThresholds"))
    }
}

impl Serialize for PoolVotingThresholds {
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
                .to_len_sz(5, force_canonical),
        )?;
        self.motion_no_confidence
            .serialize(serializer, force_canonical)?;
        self.committee_normal
            .serialize(serializer, force_canonical)?;
        self.committee_no_confidence
            .serialize(serializer, force_canonical)?;
        self.hard_fork_initiation
            .serialize(serializer, force_canonical)?;
        self.security_relevant_parameter_voting_threshold
            .serialize(serializer, force_canonical)?;
        self.encodings
            .as_ref()
            .map(|encs| encs.len_encoding)
            .unwrap_or_default()
            .end(serializer, force_canonical)
    }
}

impl Deserialize for PoolVotingThresholds {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array_sz()?;
        let len_encoding: LenEncoding = len.into();
        let mut read_len = CBORReadLen::new(len);
        read_len.read_elems(5)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let motion_no_confidence = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("motion_no_confidence"))?;
            let committee_normal = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("committee_normal"))?;
            let committee_no_confidence = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("committee_no_confidence"))?;
            let hard_fork_initiation = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("hard_fork_initiation"))?;
            let security_relevant_parameter_voting_threshold = UnitInterval::deserialize(raw)
                .map_err(|e: DeserializeError| {
                    e.annotate("security_relevant_parameter_voting_threshold")
                })?;
            match len {
                cbor_event::LenSz::Len(_, _) => (),
                cbor_event::LenSz::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(PoolVotingThresholds {
                motion_no_confidence,
                committee_normal,
                committee_no_confidence,
                hard_fork_initiation,
                security_relevant_parameter_voting_threshold,
                encodings: Some(PoolVotingThresholdsEncoding { len_encoding }),
            })
        })()
        .map_err(|e| e.annotate("PoolVotingThresholds"))
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
                    } + match &self.pool_voting_thresholds {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.d_rep_voting_thresholds {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.min_committee_size {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.committee_term_limit {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.governance_action_validity_period {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.governance_action_deposit {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.d_rep_deposit {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.d_rep_inactivity_period {
                        Some(_) => 1,
                        None => 0,
                    } + match &self.min_fee_ref_script_cost_per_byte {
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
                        } + match &self.pool_voting_thresholds {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.d_rep_voting_thresholds {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.min_committee_size {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.committee_term_limit {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.governance_action_validity_period {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.governance_action_deposit {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.d_rep_deposit {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.d_rep_inactivity_period {
                            Some(_) => 1,
                            None => 0,
                        } + match &self.min_fee_ref_script_cost_per_byte {
                            Some(_) => 1,
                            None => 0,
                        }
            })
            .map(|encs| encs.orig_deser_order.clone())
            .unwrap_or_else(|| {
                vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29,
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
                13 => {
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
                14 => {
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
                15 => {
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
                16 => {
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
                17 => {
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
                18 => {
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
                19 => {
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
                20 => {
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
                21 => {
                    if let Some(field) = &self.pool_voting_thresholds {
                        serializer.write_unsigned_integer_sz(
                            25u64,
                            fit_sz(
                                25u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.pool_voting_thresholds_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                22 => {
                    if let Some(field) = &self.d_rep_voting_thresholds {
                        serializer.write_unsigned_integer_sz(
                            26u64,
                            fit_sz(
                                26u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.d_rep_voting_thresholds_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
                    }
                }
                23 => {
                    if let Some(field) = &self.min_committee_size {
                        serializer.write_unsigned_integer_sz(
                            27u64,
                            fit_sz(
                                27u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.min_committee_size_key_encoding)
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
                                    .map(|encs| encs.min_committee_size_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                24 => {
                    if let Some(field) = &self.committee_term_limit {
                        serializer.write_unsigned_integer_sz(
                            28u64,
                            fit_sz(
                                28u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.committee_term_limit_key_encoding)
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
                                    .map(|encs| encs.committee_term_limit_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                25 => {
                    if let Some(field) = &self.governance_action_validity_period {
                        serializer.write_unsigned_integer_sz(
                            29u64,
                            fit_sz(
                                29u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.governance_action_validity_period_key_encoding)
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
                                    .map(|encs| encs.governance_action_validity_period_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                26 => {
                    if let Some(field) = &self.governance_action_deposit {
                        serializer.write_unsigned_integer_sz(
                            30u64,
                            fit_sz(
                                30u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.governance_action_deposit_key_encoding)
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
                                    .map(|encs| encs.governance_action_deposit_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                27 => {
                    if let Some(field) = &self.d_rep_deposit {
                        serializer.write_unsigned_integer_sz(
                            31u64,
                            fit_sz(
                                31u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.d_rep_deposit_key_encoding)
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
                                    .map(|encs| encs.d_rep_deposit_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                28 => {
                    if let Some(field) = &self.d_rep_inactivity_period {
                        serializer.write_unsigned_integer_sz(
                            32u64,
                            fit_sz(
                                32u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.d_rep_inactivity_period_key_encoding)
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
                                    .map(|encs| encs.d_rep_inactivity_period_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                    }
                }
                29 => {
                    if let Some(field) = &self.min_fee_ref_script_cost_per_byte {
                        serializer.write_unsigned_integer_sz(
                            33u64,
                            fit_sz(
                                33u64,
                                self.encodings
                                    .as_ref()
                                    .map(|encs| encs.min_fee_ref_script_cost_per_byte_key_encoding)
                                    .unwrap_or_default(),
                                force_canonical,
                            ),
                        )?;
                        field.serialize(serializer, force_canonical)?;
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
            let mut pool_voting_thresholds_key_encoding = None;
            let mut pool_voting_thresholds = None;
            let mut d_rep_voting_thresholds_key_encoding = None;
            let mut d_rep_voting_thresholds = None;
            let mut min_committee_size_encoding = None;
            let mut min_committee_size_key_encoding = None;
            let mut min_committee_size = None;
            let mut committee_term_limit_encoding = None;
            let mut committee_term_limit_key_encoding = None;
            let mut committee_term_limit = None;
            let mut governance_action_validity_period_encoding = None;
            let mut governance_action_validity_period_key_encoding = None;
            let mut governance_action_validity_period = None;
            let mut governance_action_deposit_encoding = None;
            let mut governance_action_deposit_key_encoding = None;
            let mut governance_action_deposit = None;
            let mut d_rep_deposit_encoding = None;
            let mut d_rep_deposit_key_encoding = None;
            let mut d_rep_deposit = None;
            let mut d_rep_inactivity_period_encoding = None;
            let mut d_rep_inactivity_period_key_encoding = None;
            let mut d_rep_inactivity_period = None;
            let mut min_fee_ref_script_cost_per_byte_key_encoding = None;
            let mut min_fee_ref_script_cost_per_byte = None;
            let mut read = 0;
            while match len { cbor_event::LenSz::Len(n, _) => read < n, cbor_event::LenSz::Indefinite => true, } {
                match raw.cbor_type()? {
                    cbor_event::Type::UnsignedInteger => match raw.unsigned_integer_sz()? {
                        (0, key_enc) =>  {
                            if minfee_a.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(0)).into());
                            }
                            let (tmp_minfee_a, tmp_minfee_a_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("minfee_a"))?;
                            minfee_a = Some(tmp_minfee_a);
                            minfee_a_encoding = tmp_minfee_a_encoding;
                            minfee_a_key_encoding = Some(key_enc);
                            orig_deser_order.push(0);
                        },
                        (1, key_enc) =>  {
                            if minfee_b.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(1)).into());
                            }
                            let (tmp_minfee_b, tmp_minfee_b_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("minfee_b"))?;
                            minfee_b = Some(tmp_minfee_b);
                            minfee_b_encoding = tmp_minfee_b_encoding;
                            minfee_b_key_encoding = Some(key_enc);
                            orig_deser_order.push(1);
                        },
                        (2, key_enc) =>  {
                            if max_block_body_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(2)).into());
                            }
                            let (tmp_max_block_body_size, tmp_max_block_body_size_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("max_block_body_size"))?;
                            max_block_body_size = Some(tmp_max_block_body_size);
                            max_block_body_size_encoding = tmp_max_block_body_size_encoding;
                            max_block_body_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(2);
                        },
                        (3, key_enc) =>  {
                            if max_transaction_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(3)).into());
                            }
                            let (tmp_max_transaction_size, tmp_max_transaction_size_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("max_transaction_size"))?;
                            max_transaction_size = Some(tmp_max_transaction_size);
                            max_transaction_size_encoding = tmp_max_transaction_size_encoding;
                            max_transaction_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(3);
                        },
                        (4, key_enc) =>  {
                            if max_block_header_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(4)).into());
                            }
                            let (tmp_max_block_header_size, tmp_max_block_header_size_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("max_block_header_size"))?;
                            max_block_header_size = Some(tmp_max_block_header_size);
                            max_block_header_size_encoding = tmp_max_block_header_size_encoding;
                            max_block_header_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(4);
                        },
                        (5, key_enc) =>  {
                            if key_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(5)).into());
                            }
                            let (tmp_key_deposit, tmp_key_deposit_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("key_deposit"))?;
                            key_deposit = Some(tmp_key_deposit);
                            key_deposit_encoding = tmp_key_deposit_encoding;
                            key_deposit_key_encoding = Some(key_enc);
                            orig_deser_order.push(5);
                        },
                        (6, key_enc) =>  {
                            if pool_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(6)).into());
                            }
                            let (tmp_pool_deposit, tmp_pool_deposit_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("pool_deposit"))?;
                            pool_deposit = Some(tmp_pool_deposit);
                            pool_deposit_encoding = tmp_pool_deposit_encoding;
                            pool_deposit_key_encoding = Some(key_enc);
                            orig_deser_order.push(6);
                        },
                        (7, key_enc) =>  {
                            if maximum_epoch.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(7)).into());
                            }
                            let (tmp_maximum_epoch, tmp_maximum_epoch_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("maximum_epoch"))?;
                            maximum_epoch = Some(tmp_maximum_epoch);
                            maximum_epoch_encoding = tmp_maximum_epoch_encoding;
                            maximum_epoch_key_encoding = Some(key_enc);
                            orig_deser_order.push(7);
                        },
                        (8, key_enc) =>  {
                            if n_opt.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(8)).into());
                            }
                            let (tmp_n_opt, tmp_n_opt_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("n_opt"))?;
                            n_opt = Some(tmp_n_opt);
                            n_opt_encoding = tmp_n_opt_encoding;
                            n_opt_key_encoding = Some(key_enc);
                            orig_deser_order.push(8);
                        },
                        (9, key_enc) =>  {
                            if pool_pledge_influence.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(9)).into());
                            }
                            let tmp_pool_pledge_influence = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Rational::deserialize(raw)
                            })().map_err(|e| e.annotate("pool_pledge_influence"))?;
                            pool_pledge_influence = Some(tmp_pool_pledge_influence);
                            pool_pledge_influence_key_encoding = Some(key_enc);
                            orig_deser_order.push(9);
                        },
                        (10, key_enc) =>  {
                            if expansion_rate.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(10)).into());
                            }
                            let tmp_expansion_rate = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                UnitInterval::deserialize(raw)
                            })().map_err(|e| e.annotate("expansion_rate"))?;
                            expansion_rate = Some(tmp_expansion_rate);
                            expansion_rate_key_encoding = Some(key_enc);
                            orig_deser_order.push(10);
                        },
                        (11, key_enc) =>  {
                            if treasury_growth_rate.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(11)).into());
                            }
                            let tmp_treasury_growth_rate = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                UnitInterval::deserialize(raw)
                            })().map_err(|e| e.annotate("treasury_growth_rate"))?;
                            treasury_growth_rate = Some(tmp_treasury_growth_rate);
                            treasury_growth_rate_key_encoding = Some(key_enc);
                            orig_deser_order.push(11);
                        },
                        (16, key_enc) =>  {
                            if min_pool_cost.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(16)).into());
                            }
                            let (tmp_min_pool_cost, tmp_min_pool_cost_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("min_pool_cost"))?;
                            min_pool_cost = Some(tmp_min_pool_cost);
                            min_pool_cost_encoding = tmp_min_pool_cost_encoding;
                            min_pool_cost_key_encoding = Some(key_enc);
                            orig_deser_order.push(12);
                        },
                        (17, key_enc) =>  {
                            if ada_per_utxo_byte.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(17)).into());
                            }
                            let (tmp_ada_per_utxo_byte, tmp_ada_per_utxo_byte_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("ada_per_utxo_byte"))?;
                            ada_per_utxo_byte = Some(tmp_ada_per_utxo_byte);
                            ada_per_utxo_byte_encoding = tmp_ada_per_utxo_byte_encoding;
                            ada_per_utxo_byte_key_encoding = Some(key_enc);
                            orig_deser_order.push(13);
                        },
                        (18, key_enc) =>  {
                            if cost_models_for_script_languages.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(18)).into());
                            }
                            let tmp_cost_models_for_script_languages = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                CostModels::deserialize(raw)
                            })().map_err(|e| e.annotate("cost_models_for_script_languages"))?;
                            cost_models_for_script_languages = Some(tmp_cost_models_for_script_languages);
                            cost_models_for_script_languages_key_encoding = Some(key_enc);
                            orig_deser_order.push(14);
                        },
                        (19, key_enc) =>  {
                            if execution_costs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(19)).into());
                            }
                            let tmp_execution_costs = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ExUnitPrices::deserialize(raw)
                            })().map_err(|e| e.annotate("execution_costs"))?;
                            execution_costs = Some(tmp_execution_costs);
                            execution_costs_key_encoding = Some(key_enc);
                            orig_deser_order.push(15);
                        },
                        (20, key_enc) =>  {
                            if max_tx_ex_units.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(20)).into());
                            }
                            let tmp_max_tx_ex_units = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ExUnits::deserialize(raw)
                            })().map_err(|e| e.annotate("max_tx_ex_units"))?;
                            max_tx_ex_units = Some(tmp_max_tx_ex_units);
                            max_tx_ex_units_key_encoding = Some(key_enc);
                            orig_deser_order.push(16);
                        },
                        (21, key_enc) =>  {
                            if max_block_ex_units.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(21)).into());
                            }
                            let tmp_max_block_ex_units = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                ExUnits::deserialize(raw)
                            })().map_err(|e| e.annotate("max_block_ex_units"))?;
                            max_block_ex_units = Some(tmp_max_block_ex_units);
                            max_block_ex_units_key_encoding = Some(key_enc);
                            orig_deser_order.push(17);
                        },
                        (22, key_enc) =>  {
                            if max_value_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(22)).into());
                            }
                            let (tmp_max_value_size, tmp_max_value_size_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("max_value_size"))?;
                            max_value_size = Some(tmp_max_value_size);
                            max_value_size_encoding = tmp_max_value_size_encoding;
                            max_value_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(18);
                        },
                        (23, key_enc) =>  {
                            if collateral_percentage.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(23)).into());
                            }
                            let (tmp_collateral_percentage, tmp_collateral_percentage_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("collateral_percentage"))?;
                            collateral_percentage = Some(tmp_collateral_percentage);
                            collateral_percentage_encoding = tmp_collateral_percentage_encoding;
                            collateral_percentage_key_encoding = Some(key_enc);
                            orig_deser_order.push(19);
                        },
                        (24, key_enc) =>  {
                            if max_collateral_inputs.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(24)).into());
                            }
                            let (tmp_max_collateral_inputs, tmp_max_collateral_inputs_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("max_collateral_inputs"))?;
                            max_collateral_inputs = Some(tmp_max_collateral_inputs);
                            max_collateral_inputs_encoding = tmp_max_collateral_inputs_encoding;
                            max_collateral_inputs_key_encoding = Some(key_enc);
                            orig_deser_order.push(20);
                        },
                        (25, key_enc) =>  {
                            if pool_voting_thresholds.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(25)).into());
                            }
                            let tmp_pool_voting_thresholds = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                PoolVotingThresholds::deserialize(raw)
                            })().map_err(|e| e.annotate("pool_voting_thresholds"))?;
                            pool_voting_thresholds = Some(tmp_pool_voting_thresholds);
                            pool_voting_thresholds_key_encoding = Some(key_enc);
                            orig_deser_order.push(21);
                        },
                        (26, key_enc) =>  {
                            if d_rep_voting_thresholds.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(26)).into());
                            }
                            let tmp_d_rep_voting_thresholds = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                DRepVotingThresholds::deserialize(raw)
                            })().map_err(|e| e.annotate("d_rep_voting_thresholds"))?;
                            d_rep_voting_thresholds = Some(tmp_d_rep_voting_thresholds);
                            d_rep_voting_thresholds_key_encoding = Some(key_enc);
                            orig_deser_order.push(22);
                        },
                        (27, key_enc) =>  {
                            if min_committee_size.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(27)).into());
                            }
                            let (tmp_min_committee_size, tmp_min_committee_size_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("min_committee_size"))?;
                            min_committee_size = Some(tmp_min_committee_size);
                            min_committee_size_encoding = tmp_min_committee_size_encoding;
                            min_committee_size_key_encoding = Some(key_enc);
                            orig_deser_order.push(23);
                        },
                        (28, key_enc) =>  {
                            if committee_term_limit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(28)).into());
                            }
                            let (tmp_committee_term_limit, tmp_committee_term_limit_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("committee_term_limit"))?;
                            committee_term_limit = Some(tmp_committee_term_limit);
                            committee_term_limit_encoding = tmp_committee_term_limit_encoding;
                            committee_term_limit_key_encoding = Some(key_enc);
                            orig_deser_order.push(24);
                        },
                        (29, key_enc) =>  {
                            if governance_action_validity_period.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(29)).into());
                            }
                            let (tmp_governance_action_validity_period, tmp_governance_action_validity_period_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("governance_action_validity_period"))?;
                            governance_action_validity_period = Some(tmp_governance_action_validity_period);
                            governance_action_validity_period_encoding = tmp_governance_action_validity_period_encoding;
                            governance_action_validity_period_key_encoding = Some(key_enc);
                            orig_deser_order.push(25);
                        },
                        (30, key_enc) =>  {
                            if governance_action_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(30)).into());
                            }
                            let (tmp_governance_action_deposit, tmp_governance_action_deposit_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("governance_action_deposit"))?;
                            governance_action_deposit = Some(tmp_governance_action_deposit);
                            governance_action_deposit_encoding = tmp_governance_action_deposit_encoding;
                            governance_action_deposit_key_encoding = Some(key_enc);
                            orig_deser_order.push(26);
                        },
                        (31, key_enc) =>  {
                            if d_rep_deposit.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(31)).into());
                            }
                            let (tmp_d_rep_deposit, tmp_d_rep_deposit_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("d_rep_deposit"))?;
                            d_rep_deposit = Some(tmp_d_rep_deposit);
                            d_rep_deposit_encoding = tmp_d_rep_deposit_encoding;
                            d_rep_deposit_key_encoding = Some(key_enc);
                            orig_deser_order.push(27);
                        },
                        (32, key_enc) =>  {
                            if d_rep_inactivity_period.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(32)).into());
                            }
                            let (tmp_d_rep_inactivity_period, tmp_d_rep_inactivity_period_encoding) = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                raw.unsigned_integer_sz().map_err(Into::<DeserializeError>::into).map(|(x, enc)| (x, Some(enc)))
                            })().map_err(|e| e.annotate("d_rep_inactivity_period"))?;
                            d_rep_inactivity_period = Some(tmp_d_rep_inactivity_period);
                            d_rep_inactivity_period_encoding = tmp_d_rep_inactivity_period_encoding;
                            d_rep_inactivity_period_key_encoding = Some(key_enc);
                            orig_deser_order.push(28);
                        },
                        (33, key_enc) =>  {
                            if min_fee_ref_script_cost_per_byte.is_some() {
                                return Err(DeserializeFailure::DuplicateKey(Key::Uint(33)).into());
                            }
                            let tmp_min_fee_ref_script_cost_per_byte = (|| -> Result<_, DeserializeError> {
                                read_len.read_elems(1)?;
                                Rational::deserialize(raw)
                            })().map_err(|e| e.annotate("min_fee_ref_script_cost_per_byte"))?;
                            min_fee_ref_script_cost_per_byte = Some(tmp_min_fee_ref_script_cost_per_byte);
                            min_fee_ref_script_cost_per_byte_key_encoding = Some(key_enc);
                            orig_deser_order.push(29);
                        },
                        (unknown_key, _enc) => return Err(DeserializeFailure::UnknownKey(Key::Uint(unknown_key)).into()),
                    },
                    cbor_event::Type::Text => return Err(DeserializeFailure::UnknownKey(Key::Str(raw.text()?)).into()),
                    cbor_event::Type::Special => match len {
                        cbor_event::LenSz::Len(_, _) => return Err(DeserializeFailure::BreakInDefiniteLen.into()),
                        cbor_event::LenSz::Indefinite => match raw.special()? {
                            cbor_event::Special::Break => break,
                            _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                        },
                    },
                    other_type => return Err(DeserializeFailure::UnexpectedKeyType(other_type).into()),
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
                min_pool_cost,
                ada_per_utxo_byte,
                cost_models_for_script_languages,
                execution_costs,
                max_tx_ex_units,
                max_block_ex_units,
                max_value_size,
                collateral_percentage,
                max_collateral_inputs,
                pool_voting_thresholds,
                d_rep_voting_thresholds,
                min_committee_size,
                committee_term_limit,
                governance_action_validity_period,
                governance_action_deposit,
                d_rep_deposit,
                d_rep_inactivity_period,
                min_fee_ref_script_cost_per_byte,
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
                    pool_voting_thresholds_key_encoding,
                    d_rep_voting_thresholds_key_encoding,
                    min_committee_size_key_encoding,
                    min_committee_size_encoding,
                    committee_term_limit_key_encoding,
                    committee_term_limit_encoding,
                    governance_action_validity_period_key_encoding,
                    governance_action_validity_period_encoding,
                    governance_action_deposit_key_encoding,
                    governance_action_deposit_encoding,
                    d_rep_deposit_key_encoding,
                    d_rep_deposit_encoding,
                    d_rep_inactivity_period_key_encoding,
                    d_rep_inactivity_period_encoding,
                    min_fee_ref_script_cost_per_byte_key_encoding,
                }),
            })
        })().map_err(|e| e.annotate("ProtocolParamUpdate"))
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
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("numerator"))?;
            let (denominator, denominator_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
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
            Script::PlutusV3 {
                script,
                len_encoding,
                tag_encoding,
            } => {
                serializer.write_array_sz(len_encoding.to_len_sz(2, force_canonical))?;
                serializer.write_unsigned_integer_sz(
                    3u64,
                    fit_sz(3u64, *tag_encoding, force_canonical),
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
            let initial_position = raw.as_mut_ref().stream_position().unwrap();
            let mut errs = Vec::new();
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
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
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("Native"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
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
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("PlutusV1"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
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
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("PlutusV2"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            let variant_deser = (|raw: &mut Deserializer<_>| -> Result<_, DeserializeError> {
                let mut read_len = CBORReadLen::new(len);
                read_len.read_elems(2)?;
                read_len.finish()?;
                let tag_encoding = (|| -> Result<_, DeserializeError> {
                    let (tag_value, tag_encoding) = raw.unsigned_integer_sz()?;
                    if tag_value != 3 {
                        return Err(DeserializeFailure::FixedValueMismatch {
                            found: Key::Uint(tag_value),
                            expected: Key::Uint(3),
                        }
                        .into());
                    }
                    Ok(Some(tag_encoding))
                })()
                .map_err(|e| e.annotate("tag"))?;
                let script = PlutusV3Script::deserialize(raw)
                    .map_err(|e: DeserializeError| e.annotate("script"))?;
                match len {
                    cbor_event::LenSz::Len(_, _) => (),
                    cbor_event::LenSz::Indefinite => match raw.special()? {
                        cbor_event::Special::Break => (),
                        _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                    },
                }
                Ok(Self::PlutusV3 {
                    script,
                    len_encoding,
                    tag_encoding,
                })
            })(raw);
            match variant_deser {
                Ok(variant) => return Ok(variant),
                Err(e) => {
                    errs.push(e.annotate("PlutusV3"));
                    raw.as_mut_ref()
                        .seek(SeekFrom::Start(initial_position))
                        .unwrap();
                }
            };
            Err(DeserializeError::new(
                "Script",
                DeserializeFailure::NoVariantMatchedWithCauses(errs),
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
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
                .map_err(|e: DeserializeError| e.annotate("start"))?;
            let (end, end_encoding) = raw
                .unsigned_integer_sz()
                .map_err(Into::<DeserializeError>::into)
                .map(|(x, enc)| (x, Some(enc)))
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
