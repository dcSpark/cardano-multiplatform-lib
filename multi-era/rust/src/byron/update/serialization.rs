// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

use crate::byron::ByronAny;

use super::*;
use cbor_event;
use cbor_event::de::Deserializer;
use cbor_event::se::Serializer;
use cml_chain::utils::BigInteger;
use cml_core::error::*;
use cml_core::serialization::*;
use cml_crypto::RawBytesEncoding;
use std::io::{BufRead, Seek, Write};

impl cbor_event::se::Serialize for Bvermod {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(14))?;
        serializer.write_array(cbor_event::Len::Len(self.script_version.len() as u64))?;
        for element in self.script_version.iter() {
            serializer.write_unsigned_integer(*element as u64)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.slot_duration.len() as u64))?;
        for element in self.slot_duration.iter() {
            // hand-edit to use cml's Serialize
            cml_core::serialization::Serialize::serialize(element, serializer, true)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.max_block_size.len() as u64))?;
        for element in self.max_block_size.iter() {
            // hand-edit to use cml's Serialize
            cml_core::serialization::Serialize::serialize(element, serializer, true)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.max_header_size.len() as u64))?;
        for element in self.max_header_size.iter() {
            // hand-edit to use cml's Serialize
            cml_core::serialization::Serialize::serialize(element, serializer, true)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.max_tx_size.len() as u64))?;
        for element in self.max_tx_size.iter() {
            // hand-edit to use cml's Serialize
            cml_core::serialization::Serialize::serialize(element, serializer, true)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.max_proposal_size.len() as u64))?;
        for element in self.max_proposal_size.iter() {
            // hand-edit to use cml's Serialize
            cml_core::serialization::Serialize::serialize(element, serializer, true)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.mpc_thd.len() as u64))?;
        for element in self.mpc_thd.iter() {
            serializer.write_unsigned_integer(*element)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.heavy_del_thd.len() as u64))?;
        for element in self.heavy_del_thd.iter() {
            serializer.write_unsigned_integer(*element)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.update_vote_thd.len() as u64))?;
        for element in self.update_vote_thd.iter() {
            serializer.write_unsigned_integer(*element)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.update_proposal_thd.len() as u64))?;
        for element in self.update_proposal_thd.iter() {
            serializer.write_unsigned_integer(*element)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.update_implicit.len() as u64))?;
        for element in self.update_implicit.iter() {
            serializer.write_unsigned_integer(*element)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.soft_fork_rule.len() as u64))?;
        for element in self.soft_fork_rule.iter() {
            element.serialize(serializer)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.tx_fee_policy.len() as u64))?;
        for element in self.tx_fee_policy.iter() {
            element.serialize(serializer)?;
        }
        serializer.write_array(cbor_event::Len::Len(self.unlock_stake_epoch.len() as u64))?;
        for element in self.unlock_stake_epoch.iter() {
            serializer.write_unsigned_integer(*element)?;
        }
        Ok(serializer)
    }
}

impl Deserialize for Bvermod {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(14)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let script_version = (|| -> Result<_, DeserializeError> {
                let mut script_version_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (script_version_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    script_version_arr.push(raw.unsigned_integer()? as u16);
                }
                Ok(script_version_arr)
            })()
            .map_err(|e| e.annotate("script_version"))?;
            let slot_duration = (|| -> Result<_, DeserializeError> {
                let mut slot_duration_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (slot_duration_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    slot_duration_arr.push(BigInteger::deserialize(raw)?);
                }
                Ok(slot_duration_arr)
            })()
            .map_err(|e| e.annotate("slot_duration"))?;
            let max_block_size = (|| -> Result<_, DeserializeError> {
                let mut max_block_size_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (max_block_size_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    max_block_size_arr.push(BigInteger::deserialize(raw)?);
                }
                Ok(max_block_size_arr)
            })()
            .map_err(|e| e.annotate("max_block_size"))?;
            let max_header_size = (|| -> Result<_, DeserializeError> {
                let mut max_header_size_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (max_header_size_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    max_header_size_arr.push(BigInteger::deserialize(raw)?);
                }
                Ok(max_header_size_arr)
            })()
            .map_err(|e| e.annotate("max_header_size"))?;
            let max_tx_size = (|| -> Result<_, DeserializeError> {
                let mut max_tx_size_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (max_tx_size_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    max_tx_size_arr.push(BigInteger::deserialize(raw)?);
                }
                Ok(max_tx_size_arr)
            })()
            .map_err(|e| e.annotate("max_tx_size"))?;
            let max_proposal_size = (|| -> Result<_, DeserializeError> {
                let mut max_proposal_size_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (max_proposal_size_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    max_proposal_size_arr.push(BigInteger::deserialize(raw)?);
                }
                Ok(max_proposal_size_arr)
            })()
            .map_err(|e| e.annotate("max_proposal_size"))?;
            let mpc_thd = (|| -> Result<_, DeserializeError> {
                let mut mpc_thd_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (mpc_thd_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    mpc_thd_arr.push(raw.unsigned_integer()?);
                }
                Ok(mpc_thd_arr)
            })()
            .map_err(|e| e.annotate("mpc_thd"))?;
            let heavy_del_thd = (|| -> Result<_, DeserializeError> {
                let mut heavy_del_thd_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (heavy_del_thd_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    heavy_del_thd_arr.push(raw.unsigned_integer()?);
                }
                Ok(heavy_del_thd_arr)
            })()
            .map_err(|e| e.annotate("heavy_del_thd"))?;
            let update_vote_thd = (|| -> Result<_, DeserializeError> {
                let mut update_vote_thd_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (update_vote_thd_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    update_vote_thd_arr.push(raw.unsigned_integer()?);
                }
                Ok(update_vote_thd_arr)
            })()
            .map_err(|e| e.annotate("update_vote_thd"))?;
            let update_proposal_thd = (|| -> Result<_, DeserializeError> {
                let mut update_proposal_thd_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (update_proposal_thd_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    update_proposal_thd_arr.push(raw.unsigned_integer()?);
                }
                Ok(update_proposal_thd_arr)
            })()
            .map_err(|e| e.annotate("update_proposal_thd"))?;
            let update_implicit = (|| -> Result<_, DeserializeError> {
                let mut update_implicit_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (update_implicit_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    update_implicit_arr.push(raw.unsigned_integer()?);
                }
                Ok(update_implicit_arr)
            })()
            .map_err(|e| e.annotate("update_implicit"))?;
            let soft_fork_rule = (|| -> Result<_, DeserializeError> {
                let mut soft_fork_rule_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (soft_fork_rule_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    soft_fork_rule_arr.push(SoftForkRule::deserialize(raw)?);
                }
                Ok(soft_fork_rule_arr)
            })()
            .map_err(|e| e.annotate("soft_fork_rule"))?;
            let tx_fee_policy = (|| -> Result<_, DeserializeError> {
                let mut tx_fee_policy_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (tx_fee_policy_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    tx_fee_policy_arr.push(ByronTxFeePolicy::deserialize(raw)?);
                }
                Ok(tx_fee_policy_arr)
            })()
            .map_err(|e| e.annotate("tx_fee_policy"))?;
            let unlock_stake_epoch = (|| -> Result<_, DeserializeError> {
                let mut unlock_stake_epoch_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (unlock_stake_epoch_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    unlock_stake_epoch_arr.push(raw.unsigned_integer()?);
                }
                Ok(unlock_stake_epoch_arr)
            })()
            .map_err(|e| e.annotate("unlock_stake_epoch"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(Bvermod {
                script_version,
                slot_duration,
                max_block_size,
                max_header_size,
                max_tx_size,
                max_proposal_size,
                mpc_thd,
                heavy_del_thd,
                update_vote_thd,
                update_proposal_thd,
                update_implicit,
                soft_fork_rule,
                tx_fee_policy,
                unlock_stake_epoch,
            })
        })()
        .map_err(|e| e.annotate("Bvermod"))
    }
}

impl cbor_event::se::Serialize for ByronBlockVersion {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(self.u16 as u64)?;
        serializer.write_unsigned_integer(self.u162 as u64)?;
        serializer.write_unsigned_integer(self.u8 as u64)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronBlockVersion {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let u16 = Ok(raw.unsigned_integer()? as u16)
                .map_err(|e: DeserializeError| e.annotate("u16"))?;
            let u162 = Ok(raw.unsigned_integer()? as u16)
                .map_err(|e: DeserializeError| e.annotate("u162"))?;
            let u8 = Ok(raw.unsigned_integer()? as u8)
                .map_err(|e: DeserializeError| e.annotate("u8"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronBlockVersion { u16, u162, u8 })
        })()
        .map_err(|e| e.annotate("ByronBlockVersion"))
    }
}

impl cbor_event::se::Serialize for ByronSoftwareVersion {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_text(&self.application_name)?;
        serializer.write_unsigned_integer(self.u32 as u64)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronSoftwareVersion {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let application_name = Ok(raw.text()? as String)
                .map_err(|e: DeserializeError| e.annotate("application_name"))?;
            let u32 = Ok(raw.unsigned_integer()? as u32)
                .map_err(|e: DeserializeError| e.annotate("u32"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronSoftwareVersion {
                application_name,
                u32,
            })
        })()
        .map_err(|e| e.annotate("ByronSoftwareVersion"))
    }
}

impl cbor_event::se::Serialize for ByronTxFeePolicy {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_unsigned_integer(0u64)?;
        serializer.write_tag(24u64)?;
        let mut index_1_inner_se = Serializer::new_vec();
        self.index_1.serialize(&mut index_1_inner_se)?;
        let index_1_bytes = index_1_inner_se.finalize();
        serializer.write_bytes(&index_1_bytes)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronTxFeePolicy {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            (|| -> Result<_, DeserializeError> {
                let index_0_value = raw.unsigned_integer()?;
                if index_0_value != 0 {
                    return Err(DeserializeFailure::FixedValueMismatch {
                        found: Key::Uint(index_0_value),
                        expected: Key::Uint(0),
                    }
                    .into());
                }
                Ok(())
            })()
            .map_err(|e| e.annotate("index_0"))?;
            let index_1 = (|| -> Result<_, DeserializeError> {
                match raw.tag()? {
                    24 => {
                        let index_1_bytes = raw.bytes()?;
                        let inner_de = &mut Deserializer::from(std::io::Cursor::new(index_1_bytes));
                        StdFeePolicy::deserialize(inner_de)
                    }
                    tag => Err(DeserializeFailure::TagMismatch {
                        found: tag,
                        expected: 24,
                    }
                    .into()),
                }
            })()
            .map_err(|e| e.annotate("index_1"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronTxFeePolicy { index_1 })
        })()
        .map_err(|e| e.annotate("ByronTxFeePolicy"))
    }
}

impl cbor_event::se::Serialize for ByronUpdate {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        serializer.write_array(cbor_event::Len::Len(self.proposal.len() as u64))?;
        for element in self.proposal.iter() {
            element.serialize(serializer)?;
        }
        serializer.write_array(cbor_event::Len::Indefinite)?;
        //serializer.write_array(cbor_event::Len::Len((self.votes.len()) as u64))?;
        for element in self.votes.iter() {
            element.serialize(serializer)?;
        }
        serializer.write_special(cbor_event::Special::Break)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronUpdate {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let proposal = (|| -> Result<_, DeserializeError> {
                let mut proposal_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (proposal_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    proposal_arr.push(ByronUpdateProposal::deserialize(raw)?);
                }
                Ok(proposal_arr)
            })()
            .map_err(|e| e.annotate("proposal"))?;
            let votes = (|| -> Result<_, DeserializeError> {
                let mut votes_arr = Vec::new();
                let len = raw.array()?;
                while match len {
                    cbor_event::Len::Len(n) => (votes_arr.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    votes_arr.push(ByronUpdateVote::deserialize(raw)?);
                }
                Ok(votes_arr)
            })()
            .map_err(|e| e.annotate("votes"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronUpdate { proposal, votes })
        })()
        .map_err(|e| e.annotate("ByronUpdate"))
    }
}

impl cbor_event::se::Serialize for ByronUpdateData {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        serializer.write_bytes(self.blake2b256.to_raw_bytes())?;
        serializer.write_bytes(self.blake2b2562.to_raw_bytes())?;
        serializer.write_bytes(self.blake2b2563.to_raw_bytes())?;
        serializer.write_bytes(self.blake2b2564.to_raw_bytes())?;
        Ok(serializer)
    }
}

impl Deserialize for ByronUpdateData {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let blake2b256 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b256"))?;
            let blake2b2562 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b2562"))?;
            let blake2b2563 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b2563"))?;
            let blake2b2564 = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("blake2b2564"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronUpdateData {
                blake2b256,
                blake2b2562,
                blake2b2563,
                blake2b2564,
            })
        })()
        .map_err(|e| e.annotate("ByronUpdateData"))
    }
}

impl cbor_event::se::Serialize for ByronUpdateProposal {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(7))?;
        self.block_version.serialize(serializer)?;
        self.block_version_mod.serialize(serializer)?;
        self.software_version.serialize(serializer)?;
        serializer.write_map(cbor_event::Len::Len(self.data.len() as u64))?;
        for (key, value) in self.data.iter() {
            serializer.write_text(key)?;
            value.serialize(serializer)?;
        }
        serializer.write_map(cbor_event::Len::Len(self.byron_attributes.len() as u64))?;
        for (key, value) in self.byron_attributes.iter() {
            key.serialize(serializer)?;
            value.serialize(serializer)?;
        }
        serializer.write_bytes(&self.from)?;
        serializer.write_bytes(&self.signature)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronUpdateProposal {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(7)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let block_version = ByronBlockVersion::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("block_version"))?;
            let block_version_mod = Bvermod::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("block_version_mod"))?;
            let software_version = ByronSoftwareVersion::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("software_version"))?;
            let data = (|| -> Result<_, DeserializeError> {
                let mut data_table = BTreeMap::new();
                let data_len = raw.map()?;
                while match data_len {
                    cbor_event::Len::Len(n) => (data_table.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let data_key = raw.text()? as String;
                    let data_value = ByronUpdateData::deserialize(raw)?;
                    if data_table.insert(data_key.clone(), data_value).is_some() {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                }
                Ok(data_table)
            })()
            .map_err(|e| e.annotate("data"))?;
            let byron_attributes = (|| -> Result<_, DeserializeError> {
                let mut byron_attributes_table = BTreeMap::new();
                let byron_attributes_len = raw.map()?;
                while match byron_attributes_len {
                    cbor_event::Len::Len(n) => (byron_attributes_table.len() as u64) < n,
                    cbor_event::Len::Indefinite => true,
                } {
                    if raw.cbor_type()? == cbor_event::Type::Special {
                        assert_eq!(raw.special()?, cbor_event::Special::Break);
                        break;
                    }
                    let byron_attributes_key = ByronAny::deserialize(raw)?;
                    let byron_attributes_value = ByronAny::deserialize(raw)?;
                    if byron_attributes_table
                        .insert(byron_attributes_key.clone(), byron_attributes_value)
                        .is_some()
                    {
                        return Err(DeserializeFailure::DuplicateKey(Key::Str(String::from(
                            "some complicated/unsupported type",
                        )))
                        .into());
                    }
                }
                Ok(byron_attributes_table)
            })()
            .map_err(|e| e.annotate("byron_attributes"))?;
            let from =
                Ok(raw.bytes()? as Vec<u8>).map_err(|e: DeserializeError| e.annotate("from"))?;
            let signature = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronUpdateProposal {
                block_version,
                block_version_mod,
                software_version,
                data,
                byron_attributes,
                from,
                signature,
            })
        })()
        .map_err(|e| e.annotate("ByronUpdateProposal"))
    }
}

impl cbor_event::se::Serialize for ByronUpdateVote {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        serializer.write_bytes(&self.voter)?;
        serializer.write_bytes(self.proposal_id.to_raw_bytes())?;
        serializer.write_special(cbor_event::Special::Bool(self.vote))?;
        serializer.write_bytes(&self.signature)?;
        Ok(serializer)
    }
}

impl Deserialize for ByronUpdateVote {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(4)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let voter =
                Ok(raw.bytes()? as Vec<u8>).map_err(|e: DeserializeError| e.annotate("voter"))?;
            let proposal_id = raw
                .bytes()
                .map_err(Into::<DeserializeError>::into)
                .and_then(|bytes| {
                    Blake2b256::from_raw_bytes(&bytes)
                        .map_err(|e| DeserializeFailure::InvalidStructure(Box::new(e)).into())
                })
                .map_err(|e: DeserializeError| e.annotate("proposal_id"))?;
            let vote = raw
                .bool()
                .map_err(Into::into)
                .map_err(|e: DeserializeError| e.annotate("vote"))?;
            let signature = Ok(raw.bytes()? as Vec<u8>)
                .map_err(|e: DeserializeError| e.annotate("signature"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(ByronUpdateVote {
                voter,
                proposal_id,
                vote,
                signature,
            })
        })()
        .map_err(|e| e.annotate("ByronUpdateVote"))
    }
}

impl cbor_event::se::Serialize for SoftForkRule {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(3))?;
        serializer.write_unsigned_integer(self.coin_portion)?;
        serializer.write_unsigned_integer(self.coin_portion2)?;
        serializer.write_unsigned_integer(self.coin_portion3)?;
        Ok(serializer)
    }
}

impl Deserialize for SoftForkRule {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(3)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let coin_portion = Ok(raw.unsigned_integer()?)
                .map_err(|e: DeserializeError| e.annotate("coin_portion"))?;
            let coin_portion2 = Ok(raw.unsigned_integer()?)
                .map_err(|e: DeserializeError| e.annotate("coin_portion2"))?;
            let coin_portion3 = Ok(raw.unsigned_integer()?)
                .map_err(|e: DeserializeError| e.annotate("coin_portion3"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(SoftForkRule {
                coin_portion,
                coin_portion2,
                coin_portion3,
            })
        })()
        .map_err(|e| e.annotate("SoftForkRule"))
    }
}

impl cbor_event::se::Serialize for StdFeePolicy {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(2))?;
        // hand-edit to call our serialize instead
        cml_core::serialization::Serialize::serialize(&self.big_integer, serializer, true)?;
        cml_core::serialization::Serialize::serialize(&self.big_integer2, serializer, true)?;

        Ok(serializer)
    }
}

impl Deserialize for StdFeePolicy {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        (|| -> Result<_, DeserializeError> {
            let big_integer = BigInteger::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("big_integer"))?;
            let big_integer2 = BigInteger::deserialize(raw)
                .map_err(|e: DeserializeError| e.annotate("big_integer2"))?;
            match len {
                cbor_event::Len::Len(_) => (),
                cbor_event::Len::Indefinite => match raw.special()? {
                    cbor_event::Special::Break => (),
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
            }
            Ok(StdFeePolicy {
                big_integer,
                big_integer2,
            })
        })()
        .map_err(|e| e.annotate("StdFeePolicy"))
    }
}
