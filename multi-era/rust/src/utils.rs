use std::borrow::Cow;

use crate::allegra::{
    AllegraCertificate, MIRAction, MoveInstantaneousReward, MoveInstantaneousRewardsCert,
};
use crate::alonzo::AlonzoProtocolParamUpdate;
use crate::babbage::{BabbageProtocolParamUpdate, BabbageTransactionOutput};
use crate::byron::block::{ByronBlockHeader, ByronEbBlock, ByronMainBlock, EbbHead};
use crate::byron::transaction::ByronTxIn;
use crate::mary::MaryTransactionOutput;
use crate::shelley::{
    GenesisKeyDelegation, ProtocolVersionStruct, ShelleyCertificate, ShelleyHeader,
    ShelleyProtocolParamUpdate, ShelleyTransactionOutput,
};
use crate::{
    allegra::AllegraBlock, alonzo::AlonzoBlock, babbage::BabbageBlock, byron::block::ByronBlock,
    mary::MaryBlock, shelley::ShelleyBlock,
};
use crate::{MultiEraBlock, MultiEraTransactionBody};

use cbor_event::de::Deserializer;
use cml_chain::address::Address;
use cml_chain::assets::{Mint, PositiveCoin};
use cml_chain::auxdata::AuxiliaryData;
use cml_chain::block::{Block, Header, OperationalCert, ProtocolVersion};
use cml_chain::byron::ByronTxOut;
use cml_chain::certs::{
    AuthCommitteeHotCert, Certificate, PoolRegistration, PoolRetirement, RegCert, RegDrepCert,
    ResignCommitteeColdCert, StakeDelegation, StakeDeregistration, StakeRegDelegCert,
    StakeRegistration, StakeVoteDelegCert, StakeVoteRegDelegCert, UnregCert, UnregDrepCert,
    UpdateDrepCert, VoteDelegCert, VoteRegDelegCert,
};
use cml_chain::crypto::{Nonce, VRFCert, Vkey};
use cml_chain::governance::{ProposalProcedure, VotingProcedures};
use cml_chain::plutus::{CostModels, ExUnitPrices, ExUnits};
use cml_chain::transaction::{
    AlonzoFormatTxOut, TransactionInput, TransactionOutput, TransactionWitnessSet,
};
use cml_chain::{
    Coin, DRepVotingThresholds, NetworkId, OrderedHashMap, PoolVotingThresholds,
    ProtocolParamUpdate, Rational, UnitInterval, Value, Withdrawals,
};
use cml_core::error::{DeserializeError, DeserializeFailure};
use cml_core::serialization::*;
use cml_core::{Epoch, Int, TransactionIndex};
use cml_crypto::{
    blake2b256, AuxiliaryDataHash, BlockBodyHash, BlockHeaderHash, Ed25519KeyHash, GenesisHash,
    RawBytesEncoding, ScriptDataHash, TransactionHash, VRFVkey,
};

impl MultiEraBlock {
    /**
     * Parses a block given the network block format with explicit era tag
     *
     * Some tools (e.g. Pallas/Oura) won't give you the block format from the binary spec directly,
     * but will instead have it wrapped in some network wrapper array containing the explicit era tag.
     * If your CBOR looks like `[uint, <actual block here>]`
     * (likely starting with `82` in hex e.g. `8201`, `8204`, `8207`, etc)
     * then you should use this function instead of the regular from_cbor_bytes().
     */
    pub fn from_explicit_network_cbor_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        let mut raw = Deserializer::from(std::io::Cursor::new(bytes));
        let len = raw.array()?;
        let mut read_len = CBORReadLen::from(len);
        read_len.read_elems(2)?;
        read_len.finish()?;
        let era = raw
            .unsigned_integer()
            .map_err(|e| DeserializeError::from(e).annotate("block_era_tag"))?;
        let block = match era {
            0 => ByronEbBlock::deserialize(&mut raw)
                .map(|ebb| Self::Byron(ByronBlock::EpochBoundary(ebb)))
                .map_err(|e| e.annotate("Byron EBB")),
            1 => ByronMainBlock::deserialize(&mut raw)
                .map(|mb| Self::Byron(ByronBlock::Main(mb)))
                .map_err(|e| e.annotate("Byron")),
            2 => ShelleyBlock::deserialize(&mut raw)
                .map(Self::Shelley)
                .map_err(|e| e.annotate("Shelley")),
            3 => AllegraBlock::deserialize(&mut raw)
                .map(Self::Allegra)
                .map_err(|e| e.annotate("Allegra")),
            4 => MaryBlock::deserialize(&mut raw)
                .map(Self::Mary)
                .map_err(|e| e.annotate("Mary")),
            5 => AlonzoBlock::deserialize(&mut raw)
                .map(Self::Alonzo)
                .map_err(|e| e.annotate("Alonzo")),
            6 => BabbageBlock::deserialize(&mut raw)
                .map(Self::Babbage)
                .map_err(|e| e.annotate("Babbage")),
            7 => Block::deserialize(&mut raw)
                .map(Self::Conway)
                .map_err(|e| e.annotate("Conway")),
            _ => Err(DeserializeFailure::NoVariantMatched.into()),
        }?;
        match len {
            cbor_event::Len::Len(_) => (),
            cbor_event::Len::Indefinite => match raw.special()? {
                cbor_event::Special::Break => (),
                _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
            },
        }
        Ok(block)
    }

    pub fn header(&self) -> MultiEraBlockHeader {
        match self {
            Self::Byron(block) => match block {
                ByronBlock::EpochBoundary(ebb) => MultiEraBlockHeader::ByronEB(ebb.header.clone()),
                ByronBlock::Main(mb) => MultiEraBlockHeader::Byron(mb.header.clone()),
            },
            Self::Shelley(block) => MultiEraBlockHeader::Shelley(block.header.clone()),
            Self::Allegra(block) => MultiEraBlockHeader::Shelley(block.header.clone()),
            Self::Mary(block) => MultiEraBlockHeader::Shelley(block.header.clone()),
            Self::Alonzo(block) => MultiEraBlockHeader::Shelley(block.header.clone()),
            Self::Babbage(block) => MultiEraBlockHeader::Babbage(block.header.clone()),
            Self::Conway(block) => MultiEraBlockHeader::Babbage(block.header.clone()),
        }
    }

    pub fn transaction_bodies(&self) -> Vec<MultiEraTransactionBody> {
        match self {
            Self::Byron(block) => match block {
                ByronBlock::EpochBoundary(_) => vec![],
                ByronBlock::Main(main) => main
                    .body
                    .tx_payload
                    .iter()
                    .map(|tx| MultiEraTransactionBody::Byron(tx.byron_tx.clone()))
                    .collect(),
            },
            Self::Shelley(block) => block
                .transaction_bodies
                .iter()
                .map(|i| MultiEraTransactionBody::Shelley(i.clone()))
                .collect(),
            Self::Allegra(block) => block
                .transaction_bodies
                .iter()
                .map(|i| MultiEraTransactionBody::Allegra(i.clone()))
                .collect(),
            Self::Mary(block) => block
                .transaction_bodies
                .iter()
                .map(|i| MultiEraTransactionBody::Mary(i.clone()))
                .collect(),
            Self::Alonzo(block) => block
                .transaction_bodies
                .iter()
                .map(|i| MultiEraTransactionBody::Alonzo(i.clone()))
                .collect(),
            Self::Babbage(block) => block
                .transaction_bodies
                .iter()
                .map(|i| MultiEraTransactionBody::Babbage(i.clone()))
                .collect(),
            Self::Conway(block) => block
                .transaction_bodies
                .iter()
                .map(|i| MultiEraTransactionBody::Conway(i.clone()))
                .collect(),
        }
    }

    pub fn transaction_witness_sets(&self) -> Vec<TransactionWitnessSet> {
        match self {
            Self::Byron(_block) => todo!(),
            Self::Shelley(block) => block
                .transaction_witness_sets
                .iter()
                .map(|wits| wits.clone().into())
                .collect(),
            Self::Allegra(block) => block
                .transaction_witness_sets
                .iter()
                .map(|wits| wits.clone().into())
                .collect(),
            Self::Mary(block) => block
                .transaction_witness_sets
                .iter()
                .map(|wits| wits.clone().into())
                .collect(),
            Self::Alonzo(block) => block
                .transaction_witness_sets
                .iter()
                .map(|wits| wits.clone().into())
                .collect(),
            Self::Babbage(block) => block
                .transaction_witness_sets
                .iter()
                .map(|wits| wits.clone().into())
                .collect(),
            Self::Conway(block) => block.transaction_witness_sets.clone(),
        }
    }

    pub fn auxiliary_data_set(&self) -> OrderedHashMap<TransactionIndex, AuxiliaryData> {
        match self {
            Self::Byron(_block) => OrderedHashMap::default(),
            Self::Shelley(block) => block
                .transaction_metadata_set
                .iter()
                .map(|(i, md)| (*i, AuxiliaryData::new_shelley(md.clone())))
                .collect(),
            Self::Allegra(block) => block
                .auxiliary_data_set
                .iter()
                .map(|(i, md)| (*i, md.clone().into()))
                .collect(),
            Self::Mary(block) => block
                .auxiliary_data_set
                .iter()
                .map(|(i, md)| (*i, md.clone().into()))
                .collect(),
            Self::Alonzo(block) => block
                .auxiliary_data_set
                .iter()
                .map(|(i, md)| (*i, md.clone().into()))
                .collect(),
            Self::Babbage(block) => block
                .auxiliary_data_set
                .iter()
                .map(|(i, md)| (*i, md.clone().into()))
                .collect(),
            Self::Conway(block) => block.auxiliary_data_set.clone(),
        }
    }

    pub fn invalid_transactions(&self) -> Vec<TransactionIndex> {
        match self {
            Self::Byron(_block) => vec![],
            Self::Shelley(_block) => vec![],
            Self::Allegra(_block) => vec![],
            Self::Mary(_block) => vec![],
            Self::Alonzo(block) => block.invalid_transactions.clone(),
            Self::Babbage(block) => block.invalid_transactions.clone(),
            Self::Conway(block) => block.invalid_transactions.clone(),
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        let bytes = match self {
            Self::Byron(block) => {
                // The hash for Byron is not calculated on header directly but instead
                // on the following CBOR structure: [0, ebb_head // 1, byron_block_header]
                // 0x82 is a canonical CBOR 2 element array
                // 0x00 and 0x01 are the integers 0 and 1
                // See: https://cardano-ledger.cardano.intersectmbo.org/cardano-ledger-byron/src/Cardano.Chain.Block.Header.html#wrapBoundaryBytes
                let mut tagged_bytes = vec![0x82];
                match block {
                    ByronBlock::EpochBoundary(ebb) => {
                        tagged_bytes.push(0x00);
                        tagged_bytes.extend(&ebb.header.to_bytes());
                    }
                    ByronBlock::Main(mb) => {
                        tagged_bytes.push(0x01);
                        tagged_bytes.extend(&mb.header.to_bytes());
                    }
                }
                tagged_bytes
            }
            Self::Shelley(block) => block.header.to_cbor_bytes(),
            Self::Allegra(block) => block.header.to_cbor_bytes(),
            Self::Mary(block) => block.header.to_cbor_bytes(),
            Self::Alonzo(block) => block.header.to_cbor_bytes(),
            Self::Babbage(block) => block.header.to_cbor_bytes(),
            Self::Conway(block) => block.header.to_cbor_bytes(),
        };

        blake2b256(&bytes)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            MultiEraBlock::Byron(b) => match b {
                ByronBlock::EpochBoundary(_) => true,
                ByronBlock::Main(block) => block.body.tx_payload.is_empty(),
            },
            MultiEraBlock::Shelley(block) => block.transaction_bodies.is_empty(),
            MultiEraBlock::Allegra(block) => block.transaction_bodies.is_empty(),
            MultiEraBlock::Mary(block) => block.transaction_bodies.is_empty(),
            MultiEraBlock::Alonzo(block) => block.transaction_bodies.is_empty(),
            MultiEraBlock::Babbage(block) => block.transaction_bodies.is_empty(),
            MultiEraBlock::Conway(block) => block.transaction_bodies.is_empty(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum MultiEraBlockHeader {
    ByronEB(EbbHead),
    Byron(ByronBlockHeader),
    Shelley(ShelleyHeader),
    Babbage(Header),
}

impl MultiEraBlockHeader {
    pub fn block_number(&self) -> u64 {
        match self {
            Self::ByronEB(eb) => eb.consensus_data.byron_difficulty.u64,
            Self::Byron(b) => b.consensus_data.byron_difficulty.u64,
            Self::Shelley(header) => header.body.block_number,
            Self::Babbage(header) => header.header_body.block_number,
        }
    }

    pub fn slot(&self) -> u64 {
        match self {
            Self::ByronEB(eb) => byron_epoch_slot_to_absolute(eb.consensus_data.epoch_id, 0),
            Self::Byron(b) => byron_epoch_slot_to_absolute(
                b.consensus_data.byron_slot_id.epoch,
                b.consensus_data.byron_slot_id.slot,
            ),
            Self::Shelley(header) => header.body.slot,
            Self::Babbage(header) => header.header_body.slot,
        }
    }

    pub fn prev_hash(&self) -> Option<BlockHeaderHash> {
        match self {
            Self::ByronEB(ebb) => {
                Some(BlockHeaderHash::from_raw_bytes(ebb.prev_block.to_raw_bytes()).unwrap())
            }
            Self::Byron(mb) => {
                Some(BlockHeaderHash::from_raw_bytes(mb.prev_block.to_raw_bytes()).unwrap())
            }
            Self::Shelley(header) => header.body.prev_hash,
            Self::Babbage(header) => header.header_body.prev_hash,
        }
    }

    pub fn issuer_vkey(&self) -> Option<&Vkey> {
        match self {
            Self::ByronEB(_) => None,
            Self::Byron(_) => None,
            Self::Shelley(header) => Some(&header.body.issuer_vkey),
            Self::Babbage(header) => Some(&header.header_body.issuer_vkey),
        }
    }

    pub fn vrf_vkey(&self) -> Option<&VRFVkey> {
        match self {
            Self::ByronEB(_) => None,
            Self::Byron(_) => None,
            Self::Shelley(header) => Some(&header.body.vrf_vkey),
            Self::Babbage(header) => Some(&header.header_body.vrf_vkey),
        }
    }

    pub fn nonce_vrf(&self) -> Option<&VRFCert> {
        match self {
            Self::ByronEB(_) => None,
            Self::Byron(_) => None,
            Self::Shelley(header) => Some(&header.body.nonce_vrf),
            Self::Babbage(_header) => None,
        }
    }

    pub fn leader_vrf(&self) -> Option<&VRFCert> {
        match self {
            Self::ByronEB(_) => None,
            Self::Byron(_) => None,
            Self::Shelley(header) => Some(&header.body.leader_vrf),
            Self::Babbage(_header) => None,
        }
    }

    pub fn vrf_result(&self) -> Option<&VRFCert> {
        match self {
            Self::ByronEB(_) => todo!(),
            Self::Byron(_) => todo!(),
            Self::Shelley(_header) => None,
            Self::Babbage(header) => Some(&header.header_body.vrf_result),
        }
    }

    pub fn block_body_size(&self) -> Option<u64> {
        match self {
            Self::ByronEB(_) => None,
            Self::Byron(_) => None,
            Self::Shelley(header) => Some(header.body.block_body_size),
            Self::Babbage(header) => Some(header.header_body.block_body_size),
        }
    }

    pub fn block_body_hash(&self) -> Option<BlockBodyHash> {
        match self {
            Self::ByronEB(_) => None,
            Self::Byron(_) => None,
            Self::Shelley(header) => Some(header.body.block_body_hash),
            Self::Babbage(header) => Some(header.header_body.block_body_hash),
        }
    }

    pub fn operational_cert(&self) -> Option<&OperationalCert> {
        match self {
            Self::ByronEB(_) => None,
            Self::Byron(_) => None,
            Self::Shelley(header) => Some(&header.body.operational_cert),
            Self::Babbage(header) => Some(&header.header_body.operational_cert),
        }
    }

    pub fn protocol_version(&self) -> Option<&ProtocolVersion> {
        match self {
            Self::ByronEB(_) => None,
            Self::Byron(_) => None,
            Self::Shelley(header) => Some(&header.body.protocol_version),
            Self::Babbage(header) => Some(&header.header_body.protocol_version),
        }
    }
}

impl MultiEraTransactionBody {
    pub fn inputs(&self) -> Vec<MultiEraTransactionInput> {
        match self {
            Self::Byron(tx) => tx
                .inputs
                .iter()
                .map(|i| MultiEraTransactionInput::Byron(i.clone()))
                .collect(),
            Self::Shelley(tx) => tx
                .inputs
                .iter()
                .map(|i| MultiEraTransactionInput::Shelley(i.clone()))
                .collect(),
            Self::Allegra(tx) => tx
                .inputs
                .iter()
                .map(|i| MultiEraTransactionInput::Shelley(i.clone()))
                .collect(),
            Self::Mary(tx) => tx
                .inputs
                .iter()
                .map(|i| MultiEraTransactionInput::Shelley(i.clone()))
                .collect(),
            Self::Alonzo(tx) => tx
                .inputs
                .iter()
                .map(|i| MultiEraTransactionInput::Shelley(i.clone()))
                .collect(),
            Self::Babbage(tx) => tx
                .inputs
                .iter()
                .map(|i| MultiEraTransactionInput::Shelley(i.clone()))
                .collect(),
            Self::Conway(tx) => tx
                .inputs
                .iter()
                .map(|i| MultiEraTransactionInput::Shelley(i.clone()))
                .collect(),
        }
    }

    pub fn outputs(&self) -> Vec<MultiEraTransactionOutput> {
        match self {
            Self::Byron(tx) => tx
                .outputs
                .iter()
                .map(|o| MultiEraTransactionOutput::Byron(o.clone()))
                .collect(),
            Self::Shelley(tx) => tx
                .outputs
                .clone()
                .into_iter()
                .map(MultiEraTransactionOutput::from)
                .collect(),
            Self::Allegra(tx) => tx
                .outputs
                .clone()
                .into_iter()
                .map(MultiEraTransactionOutput::from)
                .collect(),
            Self::Mary(tx) => tx
                .outputs
                .clone()
                .into_iter()
                .map(MultiEraTransactionOutput::from)
                .collect(),
            Self::Alonzo(tx) => tx
                .outputs
                .clone()
                .into_iter()
                .map(MultiEraTransactionOutput::from)
                .collect(),
            Self::Babbage(tx) => tx
                .outputs
                .clone()
                .into_iter()
                .map(MultiEraTransactionOutput::from)
                .collect(),
            Self::Conway(tx) => tx
                .outputs
                .iter()
                .map(|o| MultiEraTransactionOutput::Shelley(o.clone()))
                .collect(),
        }
    }

    pub fn fee(&self) -> Option<Coin> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(tx) => Some(tx.fee),
            Self::Allegra(tx) => Some(tx.fee),
            Self::Mary(tx) => Some(tx.fee),
            Self::Alonzo(tx) => Some(tx.fee),
            Self::Babbage(tx) => Some(tx.fee),
            Self::Conway(tx) => Some(tx.fee),
        }
    }

    pub fn ttl(&self) -> Option<u64> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(tx) => Some(tx.ttl),
            Self::Allegra(tx) => tx.ttl,
            Self::Mary(tx) => tx.ttl,
            Self::Alonzo(tx) => tx.ttl,
            Self::Babbage(tx) => tx.ttl,
            Self::Conway(tx) => tx.ttl,
        }
    }

    pub fn certs(&self) -> Option<Vec<MultiEraCertificate>> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(tx) => tx.certs.as_ref().map(|certs| {
                certs
                    .iter()
                    .map(|c| MultiEraCertificate::from(c.clone()))
                    .collect()
            }),
            Self::Allegra(tx) => tx.certs.as_ref().map(|certs| {
                certs
                    .iter()
                    .map(|c| MultiEraCertificate::from(c.clone()))
                    .collect()
            }),
            Self::Mary(tx) => tx.certs.as_ref().map(|certs| {
                certs
                    .iter()
                    .map(|c| MultiEraCertificate::from(c.clone()))
                    .collect()
            }),
            Self::Alonzo(tx) => tx.certs.as_ref().map(|certs| {
                certs
                    .iter()
                    .map(|c| MultiEraCertificate::from(c.clone()))
                    .collect()
            }),
            Self::Babbage(tx) => tx.certs.as_ref().map(|certs| {
                certs
                    .iter()
                    .map(|c| MultiEraCertificate::from(c.clone()))
                    .collect()
            }),
            Self::Conway(tx) => tx.certs.as_ref().map(|certs| {
                certs
                    .iter()
                    .map(|c| MultiEraCertificate::from(c.clone()))
                    .collect()
            }),
        }
    }

    pub fn withdrawals(&self) -> Option<&Withdrawals> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(tx) => tx.withdrawals.as_ref(),
            Self::Allegra(tx) => tx.withdrawals.as_ref(),
            Self::Mary(tx) => tx.withdrawals.as_ref(),
            Self::Alonzo(tx) => tx.withdrawals.as_ref(),
            Self::Babbage(tx) => tx.withdrawals.as_ref(),
            Self::Conway(tx) => tx.withdrawals.as_ref(),
        }
    }

    pub fn update(&self) -> Option<MultiEraUpdate> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(tx) => tx.update.as_ref().map(|u| MultiEraUpdate {
                epoch: u.epoch,
                proposed_protocol_parameter_updates: u
                    .shelley_proposed_protocol_parameter_updates
                    .iter()
                    .map(|(gh, ppu)| (*gh, MultiEraProtocolParamUpdate::Shelley(ppu.clone())))
                    .collect(),
            }),
            Self::Allegra(tx) => tx.update.as_ref().map(|u| MultiEraUpdate {
                epoch: u.epoch,
                proposed_protocol_parameter_updates: u
                    .shelley_proposed_protocol_parameter_updates
                    .iter()
                    .map(|(gh, ppu)| (*gh, MultiEraProtocolParamUpdate::Shelley(ppu.clone())))
                    .collect(),
            }),
            Self::Mary(tx) => tx.update.as_ref().map(|u| MultiEraUpdate {
                epoch: u.epoch,
                proposed_protocol_parameter_updates: u
                    .shelley_proposed_protocol_parameter_updates
                    .iter()
                    .map(|(gh, ppu)| (*gh, MultiEraProtocolParamUpdate::Shelley(ppu.clone())))
                    .collect(),
            }),
            Self::Alonzo(tx) => tx.update.as_ref().map(|u| MultiEraUpdate {
                epoch: u.epoch,
                proposed_protocol_parameter_updates: u
                    .proposed_protocol_parameter_updates
                    .iter()
                    .map(|(gh, ppu)| (*gh, MultiEraProtocolParamUpdate::Alonzo(ppu.clone())))
                    .collect(),
            }),
            Self::Babbage(tx) => tx.update.as_ref().map(|u| MultiEraUpdate {
                epoch: u.epoch,
                proposed_protocol_parameter_updates: u
                    .updates
                    .iter()
                    .map(|(gh, ppu)| (*gh, MultiEraProtocolParamUpdate::Babbage(ppu.clone())))
                    .collect(),
            }),
            Self::Conway(_tx) => None,
        }
    }

    pub fn auxiliary_data_hash(&self) -> Option<&AuxiliaryDataHash> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(tx) => tx.auxiliary_data_hash.as_ref(),
            Self::Allegra(tx) => tx.auxiliary_data_hash.as_ref(),
            Self::Mary(tx) => tx.auxiliary_data_hash.as_ref(),
            Self::Alonzo(tx) => tx.auxiliary_data_hash.as_ref(),
            Self::Babbage(tx) => tx.auxiliary_data_hash.as_ref(),
            Self::Conway(tx) => tx.auxiliary_data_hash.as_ref(),
        }
    }

    pub fn validity_interval_start(&self) -> Option<u64> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(tx) => tx.validity_interval_start,
            Self::Mary(tx) => tx.validity_interval_start,
            Self::Alonzo(tx) => tx.validity_interval_start,
            Self::Babbage(tx) => tx.validity_interval_start,
            Self::Conway(tx) => tx.validity_interval_start,
        }
    }

    pub fn mint(&self) -> Option<Cow<Mint>> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(tx) => tx.mint.as_ref().map(Cow::Borrowed),
            Self::Alonzo(tx) => tx.mint.as_ref().map(Cow::Borrowed),
            Self::Babbage(tx) => tx.mint.as_ref().map(|m| Cow::Owned(m.to_mint())),
            Self::Conway(tx) => tx.mint.as_ref().map(Cow::Borrowed),
        }
    }

    pub fn script_data_hash(&self) -> Option<ScriptDataHash> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(tx) => tx.script_data_hash,
            Self::Babbage(tx) => tx.script_data_hash,
            Self::Conway(tx) => tx.script_data_hash,
        }
    }

    pub fn collateral_inputs(&self) -> Option<&[TransactionInput]> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(tx) => tx.collateral_inputs.as_ref().map(|inputs| inputs.as_ref()),
            Self::Babbage(tx) => tx.collateral_inputs.as_ref().map(|inputs| inputs.as_ref()),
            Self::Conway(tx) => tx.collateral_inputs.as_ref().map(|inputs| inputs.as_ref()),
        }
    }

    pub fn required_signers(&self) -> Option<&[Ed25519KeyHash]> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(tx) => tx.required_signers.as_ref().map(|signers| signers.as_ref()),
            Self::Babbage(tx) => tx.required_signers.as_ref().map(|signers| signers.as_ref()),
            Self::Conway(tx) => tx.required_signers.as_ref().map(|signers| signers.as_ref()),
        }
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(tx) => tx.network_id,
            Self::Babbage(tx) => tx.network_id,
            Self::Conway(tx) => tx.network_id,
        }
    }

    pub fn collateral_return(&self) -> Option<MultiEraTransactionOutput> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(_tx) => None,
            Self::Babbage(tx) => tx.collateral_return.as_ref().map(|ret| ret.clone().into()),
            Self::Conway(tx) => tx.collateral_return.as_ref().map(|ret| ret.clone().into()),
        }
    }

    pub fn total_collateral(&self) -> Option<Coin> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(_tx) => None,
            Self::Babbage(tx) => tx.total_collateral,
            Self::Conway(tx) => tx.total_collateral,
        }
    }

    pub fn reference_inputs(&self) -> Option<&[TransactionInput]> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(_tx) => None,
            Self::Babbage(tx) => tx.reference_inputs.as_ref().map(|inputs| inputs.as_ref()),
            Self::Conway(tx) => tx.reference_inputs.as_ref().map(|inputs| inputs.as_ref()),
        }
    }

    pub fn voting_procedures(&self) -> Option<&VotingProcedures> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(_tx) => None,
            Self::Babbage(_tx) => None,
            Self::Conway(tx) => tx.voting_procedures.as_ref(),
        }
    }

    pub fn proposal_procedures(&self) -> Option<&[ProposalProcedure]> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(_tx) => None,
            Self::Babbage(_tx) => None,
            Self::Conway(tx) => tx.proposal_procedures.as_ref().map(|pps| pps.as_ref()),
        }
    }

    pub fn current_treasury_value(&self) -> Option<Coin> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(_tx) => None,
            Self::Babbage(_tx) => None,
            Self::Conway(tx) => tx.current_treasury_value,
        }
    }

    pub fn donation(&self) -> Option<PositiveCoin> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(_tx) => None,
            Self::Babbage(_tx) => None,
            Self::Conway(tx) => tx.donation,
        }
    }

    pub fn hash(&self) -> TransactionHash {
        match self {
            MultiEraTransactionBody::Byron(tx) => tx.hash(),
            MultiEraTransactionBody::Shelley(tx) => tx.hash(),
            MultiEraTransactionBody::Allegra(tx) => tx.hash(),
            MultiEraTransactionBody::Mary(tx) => tx.hash(),
            MultiEraTransactionBody::Alonzo(tx) => tx.hash(),
            MultiEraTransactionBody::Babbage(tx) => tx.hash(),
            MultiEraTransactionBody::Conway(tx) => tx.hash(),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum MultiEraCertificate {
    StakeRegistration(StakeRegistration),
    StakeDeregistration(StakeDeregistration),
    StakeDelegation(StakeDelegation),
    PoolRegistration(PoolRegistration),
    PoolRetirement(PoolRetirement),
    GenesisKeyDelegation(GenesisKeyDelegation),
    MoveInstantaneousRewardsCert(MoveInstantaneousRewardsCert),
    RegCert(RegCert),
    UnregCert(UnregCert),
    VoteDelegCert(VoteDelegCert),
    StakeVoteDelegCert(StakeVoteDelegCert),
    StakeRegDelegCert(StakeRegDelegCert),
    VoteRegDelegCert(VoteRegDelegCert),
    StakeVoteRegDelegCert(StakeVoteRegDelegCert),
    AuthCommitteeHotCert(AuthCommitteeHotCert),
    ResignCommitteeColdCert(ResignCommitteeColdCert),
    RegDrepCert(RegDrepCert),
    UnregDrepCert(UnregDrepCert),
    UpdateDrepCert(UpdateDrepCert),
}

impl From<ShelleyCertificate> for MultiEraCertificate {
    fn from(cert: ShelleyCertificate) -> Self {
        match cert {
            ShelleyCertificate::StakeRegistration(cert) => Self::StakeRegistration(cert),
            ShelleyCertificate::StakeDeregistration(cert) => Self::StakeDeregistration(cert),
            ShelleyCertificate::StakeDelegation(cert) => Self::StakeDelegation(cert),
            ShelleyCertificate::ShelleyPoolRegistration(cert) => {
                Self::PoolRegistration(cert.into())
            }
            ShelleyCertificate::PoolRetirement(cert) => Self::PoolRetirement(cert),
            ShelleyCertificate::GenesisKeyDelegation(cert) => Self::GenesisKeyDelegation(cert),
            ShelleyCertificate::ShelleyMoveInstantaneousRewardsCert(cert) => {
                Self::MoveInstantaneousRewardsCert(MoveInstantaneousRewardsCert::new(
                    MoveInstantaneousReward::new(
                        cert.shelley_move_instantaneous_reward.pot,
                        MIRAction::new_to_stake_credentials(
                            cert.shelley_move_instantaneous_reward
                                .to_stake_credentials
                                .iter()
                                .map(|(k, v)| (k.clone(), Int::from(*v)))
                                .collect(),
                        ),
                    ),
                ))
            }
        }
    }
}

impl From<AllegraCertificate> for MultiEraCertificate {
    fn from(cert: AllegraCertificate) -> Self {
        match cert {
            AllegraCertificate::StakeRegistration(cert) => Self::StakeRegistration(cert),
            AllegraCertificate::StakeDeregistration(cert) => Self::StakeDeregistration(cert),
            AllegraCertificate::StakeDelegation(cert) => Self::StakeDelegation(cert),
            AllegraCertificate::ShelleyPoolRegistration(cert) => {
                Self::PoolRegistration(cert.into())
            }
            AllegraCertificate::PoolRetirement(cert) => Self::PoolRetirement(cert),
            AllegraCertificate::GenesisKeyDelegation(cert) => Self::GenesisKeyDelegation(cert),
            AllegraCertificate::MoveInstantaneousRewardsCert(cert) => {
                Self::MoveInstantaneousRewardsCert(cert)
            }
        }
    }
}

impl From<Certificate> for MultiEraCertificate {
    fn from(cert: Certificate) -> Self {
        match cert {
            Certificate::StakeRegistration(cert) => Self::StakeRegistration(cert),
            Certificate::StakeDeregistration(cert) => Self::StakeDeregistration(cert),
            Certificate::StakeDelegation(cert) => Self::StakeDelegation(cert),
            Certificate::PoolRegistration(cert) => Self::PoolRegistration(cert),
            Certificate::PoolRetirement(cert) => Self::PoolRetirement(cert),
            Certificate::RegCert(cert) => Self::RegCert(cert),
            Certificate::UnregCert(cert) => Self::UnregCert(cert),
            Certificate::VoteDelegCert(cert) => Self::VoteDelegCert(cert),
            Certificate::StakeVoteDelegCert(cert) => Self::StakeVoteDelegCert(cert),
            Certificate::StakeRegDelegCert(cert) => Self::StakeRegDelegCert(cert),
            Certificate::VoteRegDelegCert(cert) => Self::VoteRegDelegCert(cert),
            Certificate::StakeVoteRegDelegCert(cert) => Self::StakeVoteRegDelegCert(cert),
            Certificate::AuthCommitteeHotCert(cert) => Self::AuthCommitteeHotCert(cert),
            Certificate::ResignCommitteeColdCert(cert) => Self::ResignCommitteeColdCert(cert),
            Certificate::RegDrepCert(cert) => Self::RegDrepCert(cert),
            Certificate::UnregDrepCert(cert) => Self::UnregDrepCert(cert),
            Certificate::UpdateDrepCert(cert) => Self::UpdateDrepCert(cert),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum MultiEraProtocolParamUpdate {
    Shelley(ShelleyProtocolParamUpdate),
    Alonzo(AlonzoProtocolParamUpdate),
    Babbage(BabbageProtocolParamUpdate),
    Conway(ProtocolParamUpdate),
}

impl MultiEraProtocolParamUpdate {
    pub fn minfee_a(&self) -> Option<u64> {
        match self {
            Self::Shelley(update) => update.minfee_a,
            Self::Alonzo(update) => update.minfee_a,
            Self::Babbage(update) => update.minfee_a,
            Self::Conway(update) => update.minfee_a,
        }
    }

    pub fn minfee_b(&self) -> Option<u64> {
        match self {
            Self::Shelley(update) => update.minfee_b,
            Self::Alonzo(update) => update.minfee_b,
            Self::Babbage(update) => update.minfee_b,
            Self::Conway(update) => update.minfee_b,
        }
    }

    pub fn max_block_body_size(&self) -> Option<u64> {
        match self {
            Self::Shelley(update) => update.max_block_body_size,
            Self::Alonzo(update) => update.max_block_body_size,
            Self::Babbage(update) => update.max_block_body_size,
            Self::Conway(update) => update.max_block_body_size,
        }
    }

    pub fn max_transaction_size(&self) -> Option<u64> {
        match self {
            Self::Shelley(update) => update.max_transaction_size,
            Self::Alonzo(update) => update.max_transaction_size,
            Self::Babbage(update) => update.max_transaction_size,
            Self::Conway(update) => update.max_transaction_size,
        }
    }

    pub fn max_block_header_size(&self) -> Option<u64> {
        match self {
            Self::Shelley(update) => update.max_block_header_size,
            Self::Alonzo(update) => update.max_block_header_size,
            Self::Babbage(update) => update.max_block_header_size,
            Self::Conway(update) => update.max_block_header_size,
        }
    }

    pub fn key_deposit(&self) -> Option<Coin> {
        match self {
            Self::Shelley(update) => update.key_deposit,
            Self::Alonzo(update) => update.key_deposit,
            Self::Babbage(update) => update.key_deposit,
            Self::Conway(update) => update.key_deposit,
        }
    }

    pub fn pool_deposit(&self) -> Option<Coin> {
        match self {
            Self::Shelley(update) => update.pool_deposit,
            Self::Alonzo(update) => update.pool_deposit,
            Self::Babbage(update) => update.pool_deposit,
            Self::Conway(update) => update.pool_deposit,
        }
    }

    pub fn maximum_epoch(&self) -> Option<Epoch> {
        match self {
            Self::Shelley(update) => update.maximum_epoch,
            Self::Alonzo(update) => update.maximum_epoch,
            Self::Babbage(update) => update.maximum_epoch,
            Self::Conway(update) => update.maximum_epoch,
        }
    }

    pub fn n_opt(&self) -> Option<u64> {
        match self {
            Self::Shelley(update) => update.n_opt,
            Self::Alonzo(update) => update.n_opt,
            Self::Babbage(update) => update.n_opt,
            Self::Conway(update) => update.n_opt,
        }
    }

    pub fn pool_pledge_influence(&self) -> Option<&Rational> {
        match self {
            Self::Shelley(update) => update.pool_pledge_influence.as_ref(),
            Self::Alonzo(update) => update.pool_pledge_influence.as_ref(),
            Self::Babbage(update) => update.pool_pledge_influence.as_ref(),
            Self::Conway(update) => update.pool_pledge_influence.as_ref(),
        }
    }

    pub fn expansion_rate(&self) -> Option<&UnitInterval> {
        match self {
            Self::Shelley(update) => update.expansion_rate.as_ref(),
            Self::Alonzo(update) => update.expansion_rate.as_ref(),
            Self::Babbage(update) => update.expansion_rate.as_ref(),
            Self::Conway(update) => update.expansion_rate.as_ref(),
        }
    }

    pub fn treasury_growth_rate(&self) -> Option<&UnitInterval> {
        match self {
            Self::Shelley(update) => update.treasury_growth_rate.as_ref(),
            Self::Alonzo(update) => update.treasury_growth_rate.as_ref(),
            Self::Babbage(update) => update.treasury_growth_rate.as_ref(),
            Self::Conway(update) => update.treasury_growth_rate.as_ref(),
        }
    }

    pub fn decentralization_constant(&self) -> Option<&UnitInterval> {
        match self {
            Self::Shelley(update) => update.decentralization_constant.as_ref(),
            Self::Alonzo(update) => update.decentralization_constant.as_ref(),
            Self::Babbage(_update) => None,
            Self::Conway(_update) => None,
        }
    }

    pub fn extra_entropy(&self) -> Option<&Nonce> {
        match self {
            Self::Shelley(update) => update.extra_entropy.as_ref(),
            Self::Alonzo(update) => update.extra_entropy.as_ref(),
            Self::Babbage(_update) => None,
            Self::Conway(_update) => None,
        }
    }

    pub fn protocol_version(&self) -> Option<&ProtocolVersionStruct> {
        match self {
            Self::Shelley(update) => update.protocol_version.as_ref(),
            Self::Alonzo(update) => update.protocol_version.as_ref(),
            Self::Babbage(update) => update.protocol_version.as_ref(),
            Self::Conway(_update) => None,
        }
    }

    pub fn min_utxo_value(&self) -> Option<Coin> {
        match self {
            Self::Shelley(update) => update.min_utxo_value,
            Self::Alonzo(_update) => None,
            Self::Babbage(_update) => None,
            Self::Conway(_update) => None,
        }
    }

    pub fn min_pool_cost(&self) -> Option<Coin> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(update) => update.min_pool_cost,
            Self::Babbage(update) => update.min_pool_cost,
            Self::Conway(update) => update.min_pool_cost,
        }
    }

    pub fn ada_per_utxo_byte(&self) -> Option<Coin> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(update) => update.ada_per_utxo_byte,
            Self::Babbage(update) => update.ada_per_utxo_byte,
            Self::Conway(update) => update.ada_per_utxo_byte,
        }
    }

    pub fn cost_models_for_script_languages(&self) -> Option<CostModels> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(update) => update
                .cost_models_for_script_languages
                .clone()
                .map(Into::into),
            Self::Babbage(update) => update
                .cost_models_for_script_languages
                .clone()
                .map(Into::into),
            Self::Conway(update) => update.cost_models_for_script_languages.clone(),
        }
    }

    pub fn execution_costs(&self) -> Option<&ExUnitPrices> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(update) => update.execution_costs.as_ref(),
            Self::Babbage(update) => update.execution_costs.as_ref(),
            Self::Conway(update) => update.execution_costs.as_ref(),
        }
    }

    pub fn max_tx_ex_units(&self) -> Option<&ExUnits> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(update) => update.max_tx_ex_units.as_ref(),
            Self::Babbage(update) => update.max_tx_ex_units.as_ref(),
            Self::Conway(update) => update.max_tx_ex_units.as_ref(),
        }
    }

    pub fn max_block_ex_units(&self) -> Option<&ExUnits> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(update) => update.max_block_ex_units.as_ref(),
            Self::Babbage(update) => update.max_block_ex_units.as_ref(),
            Self::Conway(update) => update.max_block_ex_units.as_ref(),
        }
    }

    pub fn max_value_size(&self) -> Option<u64> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(update) => update.max_value_size,
            Self::Babbage(update) => update.max_value_size,
            Self::Conway(update) => update.max_value_size,
        }
    }

    pub fn collateral_percentage(&self) -> Option<u64> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(update) => update.collateral_percentage,
            Self::Babbage(update) => update.collateral_percentage,
            Self::Conway(update) => update.collateral_percentage,
        }
    }

    pub fn max_collateral_inputs(&self) -> Option<u64> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(update) => update.max_collateral_inputs,
            Self::Babbage(update) => update.max_collateral_inputs,
            Self::Conway(update) => update.max_collateral_inputs,
        }
    }

    pub fn pool_voting_thresholds(&self) -> Option<&PoolVotingThresholds> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(_update) => None,
            Self::Babbage(_update) => None,
            Self::Conway(update) => update.pool_voting_thresholds.as_ref(),
        }
    }

    pub fn d_rep_voting_thresholds(&self) -> Option<&DRepVotingThresholds> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(_update) => None,
            Self::Babbage(_update) => None,
            Self::Conway(update) => update.d_rep_voting_thresholds.as_ref(),
        }
    }

    pub fn min_committee_size(&self) -> Option<u64> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(_update) => None,
            Self::Babbage(_update) => None,
            Self::Conway(update) => update.min_committee_size,
        }
    }

    pub fn committee_term_limit(&self) -> Option<u64> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(_update) => None,
            Self::Babbage(_update) => None,
            Self::Conway(update) => update.committee_term_limit,
        }
    }

    pub fn governance_action_validity_period(&self) -> Option<Epoch> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(_update) => None,
            Self::Babbage(_update) => None,
            Self::Conway(update) => update.governance_action_validity_period,
        }
    }

    pub fn governance_action_deposit(&self) -> Option<Coin> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(_update) => None,
            Self::Babbage(_update) => None,
            Self::Conway(update) => update.governance_action_deposit,
        }
    }

    pub fn d_rep_deposit(&self) -> Option<Coin> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(_update) => None,
            Self::Babbage(_update) => None,
            Self::Conway(update) => update.d_rep_deposit,
        }
    }

    pub fn d_rep_inactivity_period(&self) -> Option<Epoch> {
        match self {
            Self::Shelley(_update) => None,
            Self::Alonzo(_update) => None,
            Self::Babbage(_update) => None,
            Self::Conway(update) => update.d_rep_inactivity_period,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum MultiEraTransactionInput {
    Byron(ByronTxIn),
    /// All eras from Shelley onward have the same tx in format
    Shelley(TransactionInput),
}

impl MultiEraTransactionInput {
    /// Transaction hash this input was created in
    /// Will return None only for Byron Genesis inputs
    pub fn hash(&self) -> Option<&TransactionHash> {
        match self {
            Self::Byron(input) => match input {
                ByronTxIn::ByronTxInRegular(reg) => Some(&reg.index_1.byron_tx_id),
                ByronTxIn::ByronTxInGenesis(_gen) => None,
            },
            Self::Shelley(input) => Some(&input.transaction_id),
        }
    }

    /// Transaction index into the tx that this input was created in
    /// Will return None for only Byron Genesis inputs
    pub fn index(&self) -> Option<u64> {
        match self {
            Self::Byron(input) => match input {
                ByronTxIn::ByronTxInRegular(reg) => Some(reg.index_1.u32.into()),
                ByronTxIn::ByronTxInGenesis(_gen) => None,
            },
            Self::Shelley(input) => Some(input.index),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub enum MultiEraTransactionOutput {
    Byron(ByronTxOut),
    Shelley(TransactionOutput),
}

impl MultiEraTransactionOutput {
    pub fn address(&self) -> Address {
        match self {
            Self::Byron(output) => output.address.clone().to_address(),
            Self::Shelley(output) => output.address().clone(),
        }
    }

    pub fn amount(&self) -> Value {
        match self {
            Self::Byron(output) => output.amount.into(),
            Self::Shelley(output) => output.amount().clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultiEraUpdate {
    pub epoch: u64,
    pub proposed_protocol_parameter_updates:
        OrderedHashMap<GenesisHash, MultiEraProtocolParamUpdate>,
}

impl From<ShelleyTransactionOutput> for MultiEraTransactionOutput {
    fn from(o: ShelleyTransactionOutput) -> Self {
        MultiEraTransactionOutput::Shelley(TransactionOutput::new(
            o.address.clone(),
            Value::from(o.amount),
            None,
            None,
        ))
    }
}

impl From<MaryTransactionOutput> for MultiEraTransactionOutput {
    fn from(o: MaryTransactionOutput) -> Self {
        MultiEraTransactionOutput::Shelley(TransactionOutput::new(
            o.address.clone(),
            o.amount.clone(),
            None,
            None,
        ))
    }
}

impl From<AlonzoFormatTxOut> for MultiEraTransactionOutput {
    fn from(o: AlonzoFormatTxOut) -> Self {
        MultiEraTransactionOutput::Shelley(o.clone().into())
    }
}

impl From<BabbageTransactionOutput> for MultiEraTransactionOutput {
    fn from(o: BabbageTransactionOutput) -> Self {
        MultiEraTransactionOutput::Shelley(match o {
            BabbageTransactionOutput::AlonzoFormatTxOut(alonzo) => {
                TransactionOutput::AlonzoFormatTxOut(alonzo.clone())
            }
            BabbageTransactionOutput::BabbageFormatTxOut(babbage) => TransactionOutput::new(
                babbage.address.clone(),
                babbage.amount.clone(),
                babbage.datum_option.clone(),
                babbage.script_reference.clone().map(Into::into),
            ),
        })
    }
}

impl From<TransactionOutput> for MultiEraTransactionOutput {
    fn from(o: TransactionOutput) -> Self {
        MultiEraTransactionOutput::Shelley(o)
    }
}

const KNOWN_SLOT_LENGTH_SECS: u64 = 20; // 20 secs
const KNOWN_EPOCH_LENGTH_SECS: u64 = 5 * 24 * 60 * 60; // 5 days

fn byron_epoch_slot_to_absolute(epoch: u64, sub_epoch_slot: u64) -> u64 {
    ((epoch * KNOWN_EPOCH_LENGTH_SECS) / KNOWN_SLOT_LENGTH_SECS) + sub_epoch_slot
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn byron_network_block_parse() {
        let bytes = hex::decode("82018385015820f3e188a0adb6e8bc840286be2241bdf8d72efa8078d84534f4ac5e51189f537284830058200e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a85820afc0da64183bf2664f3d4eec7238d524ba607faeeab24fc100eb861dba69971b82035820d36a2619a672494604e11bb447cbcf5231e9f2ba25c2169177edc941bd50ad6c5820afc0da64183bf2664f3d4eec7238d524ba607faeeab24fc100eb861dba69971b58204e66280cd94d591072349bec0a3090a53aa945562efb6d08d56e53654b0e409884820119087158405eba3a05b57a84c877453667b2de00061b50dafafcdd83d7a0b7d0f0959eba7bef72eb9d18142f2deab055f197ac15a830e38aae8155e3cca07d212adb185110810c820282840058405eba3a05b57a84c877453667b2de00061b50dafafcdd83d7a0b7d0f0959eba7bef72eb9d18142f2deab055f197ac15a830e38aae8155e3cca07d212adb18511058409aae625d4d15bcb3733d420e064f1cd338f386e0af049fcd42b455a69d28ad366483d177ba2b801b4136e0d6662e5e9e0a24f2c80a0e78d4c235b4c08f201f4c5840939dcfe5555ee661b9db5d817a70d5c3fa9d1d97c2ae5849696d915606b530f7e9edda5d02a01e61524a766f9c356084616ba058a3de70ea51bf29cd187a5f0758402deb50bb6bb566cc688abe0548612b72e92e16a4b20542d2488eb479b31f6646457bdf8575e3bd9f168d278bb4cc7f91a27efaa7ec4e6e7ab24afafef84f7f0b8483010000826a63617264616e6f2d736c01a058204ba92aa320c60acc9ad7b9a64f2eda55c4d2ec28e604faf186708b4f0c4e8edf849fff8203d90102809fff82809fff81a0").unwrap();
        let _block = MultiEraBlock::from_explicit_network_cbor_bytes(&bytes).unwrap();
    }

    #[test]
    fn shelley_network_block_parse() {
        let bytes = hex::decode("820284828f182e1a00015180582045899e8002b27df291e09188bfe3aeb5397ac03546a7d0ead93aa2500860f1af5820d1a8de6caa8fd9b175c59862ecdd5abcd0477b84b82a0e52faecc6b3c85100a4582051995f616f8a025f974b20330a53c0c81e8ea95973d73d15fff7bab57589311d8258406ef0be9293f5a307037b60b7252eae99bb045d58a521d5ba7c8d3aa129292a012a9391eefff8843f3558b35265ec432501b24688a2e0a21964fc71cf69c25dae58503d43c14357cd83b0f941f8c7e7a573fc361b8702bfee211bd7adb1a59f9576fe1bd11e3fa32b5a982e7b246e18058b099f147362c9c18acc43e1e4e8b5b7c660ea3ed9ac5b7a7ec0feb59ccfb64729048258407b4043c26958ea012568b14b0b95ae50d0bd589ae50d58ca5a7190375854084292f87b7f8878887269aaa59251d02032bf1bef5f5fe8fdafd1217164f0d77d885850f68678b7d56dc4178307ba00737cb3339c41f0bf77a894d2156c6cb4ce2047597b2b23dce04a7e1b9abd07a1b870bcf9b18aee0ce27d1fc3c4f2f73cf91f0ca3167e3f67dbe90838da2e8bf15234cb060358201033376be025cb705fd8dd02eda11cc73975a062b5d14ffd74d6ff69e69a2ff758202b9a5add912f3edc5c325d6250b9cc154de8f35e2924f5b1c707a4123808d064000058407fb060b885ffc7b55bb6e095ea6999eaa5608b6b4e92f5cc64bc34ba85000ebe42839432f15d86df07740374f69ce60f72feb16a0d47f5cf2c43158caf37ad0303005901c0f1f0c338a257b27351cf337c4c56961e67235b93cfa351f9d5f348911c7870cb2e084ff7c1f3d93719245fdbba539165d8eea9a510251e5cc3210b3bae3bb40034d42d5ecf181026f436173ad3036d5be2ba595f5facf920bcb48e8fd8b7b5fbf4f8fad5e652fd99be5d322fe920e702cc4afd218d76bd6800812155d8012c8fd57538a7b9d64f2defee3e32879e36db649a934b00784e6223023bdfffa59f4e54609d63a6f5ad04850c419a3556db8b291b90467fadfc67194a3069ef6ff4c0f7d6677145ceb51be68d6d0c20d0e92f80313c48dabf5ae8e3acd9fc43f450874848221f71d2f895c18790082d17467de32ff047a22cee1799db7e77e651a35c15b32d4f838133cc80d467308587ff5cea12be5b3b8b7d2d0d2eadf066b67cd965100555f96457d0d70988ffc2a7c212afa73338df3ece84ee7de2170aadec1dafc360580432193ab2a25c9c4555e57bc0d88cf50d7036378b4dabde79e5f858539a464e0a547660374da91d7d19acd753e219a8fee41a43bd4190db235dc0b1224bcfb9a760fb2b39063dccce88453043c0297cb6c93bca145a9ebbd6bc3a916ed9439343ac3510c47886d17a9187e833b9149e5ac2854c4d88a7c4b4ee68828080a0").unwrap();
        let _block = MultiEraBlock::from_explicit_network_cbor_bytes(&bytes).unwrap();
    }

    #[test]
    fn allegra_network_block_parse() {
        let bytes = hex::decode("820384828f1954971a0007e9c85820cacf5da6b8d81bbdf77b5ce4f5ea7f7b6714a29b1e81dbf541b01e92d8e1a3215820618b625df30de53895ff29e7a3770dca56c2ff066d4aa05a6971905deecef6db5820707a5e99ceec213eb56768da310566da8f4ff56cbdd90431ebd0ae17f6c8cc8b82584091b1c2d55cc491732a4cfa591a4e9bfd1aada7610d25e0fb9bb62176a0daf709485271c911c275b007005a0cf17e41e6639dff95d59319bf96270ec1515c161958506195ed4ddd4efd642b1810aa5ff92f91cb25082f07a61be35c7b82f06c9b8dc3a2fb7f9f1d40ff5779e63d02b09253716971018f8dfc0e4aa07bbeaa0e26f3fb235e0de00f60ba879c8a52744e8d470f825840ed8ac2d394a4a8022224b6f4b4b859bb748e6af00b8daa998c2aad2a9f42f8f4dc4f3eba29e323b426099805d02a7daf79ba262b51191b26bf07fce07f3effb75850e58bd3d0326bf69fb3ed652a556f16fb61e4835f6766d92965ddeea69a7000fcff6d98fa5f5cae9f5c3cf99b5606a76319180eaaff4af81aea358077e4363237579c9078dfce08a72a0b5ca90c5d140e1904a958206f970389c3de6fedb3d8d981a32e9bd3791f6e3230cdf02632394f0f7115a54b582005424ee48b0616cdbd5bc631ed25a628518575912c22c6dfea7e2778aac12bba000058404fa969b5356abab0a3c8a42007a3ab177d17aebdf4bedd93a541f545544a01dbb6e2696ef58ee8cf96c214717a4ebd35f2fa992d5815db01382f1bd516a38c0504005901c076acf26e9c06a576578d9977e791cf29017ebee3ec38d341c33c858b4e4fa0a9546e26033bab39b59947c4f25a0a2539dae2ed8a114675e72105df7449fab404088e270b2f1d3ff6c7b6c9f3b8150ec67d7ce24c6732cec4cec8de0b932b3e60507a6d639ab6ba44468039bc4c2f67b7952eaf125e9e11e8df871cfd2ba5316cbc4bd8e012158d56011100489b821ec29c3b9bdc28ed00911a529e46a83dd50faa7c35e2e59af061766144b53289d730787a1575153e9b3622733134443656dc9ba5091ce00397cd56ed509a875c177cc8b8b52b5e1bdba6aa414d966c5c6fd20b05a932284ca9902735bf350c0eda9af447beaad02703960b427a7368bb73b38fe90d56d3364c761b9a3ae0acb285054cf7ce0573d05464e1ea1b298ed8e876442244df9e3f19c4b36f8b4e09e0c63aeb48ac8f1c16af7897aa2a7846983f9d58ad4f84a5fa579f48028b66b9bc0617a2e9c67625cf98fc0b2df820393c63cf8b35c98874f8592752a1c8b34b09ed2d08d3ffc97c567152af96a1044485d66316c4ba224361e8ce16423fb537346f6cb4a9e1c2b3cf496dffe8e5a59cdd274d40d8f7d1a2ba6fc0aa3ce682e635272b9e57bac008586e14b67926c5bb24124781a40081825820a00696a0c2d70c381a265a845e43c55e1d00f96b27c06defc015dc92eb20624000018182581d609e5614893238cf85e284c61ec56d5efd9f9cdc4863ba7e1bf00c2c7d1b006983fdc40382dd021a00032bd50682a7581c637f2e950b0fd8f8e3e811c5fbeb19e411e7a2bf37272b84b29c1a0ba20cd81e8200010e820400581c8a4b77c4f534f8b8cc6f269e5ebb7ba77fa63a476e50e05e66d7051ca20cd81e8200010e820400581cb00470cd193d67aac47c373602fccd4195aad3002c169b5570de1126a20cd81e8200010e820400581cb260ffdb6eba541fcf18601923457307647dce807851b9d19da133aba20cd81e8200010e820400581cced1599fd821a39593e00592e5292bdc1437ae0f7af388ef5257344aa20cd81e8200010e820400581cdd2a7d71a05bed11db61555ba4c658cb1ce06c8024193d064f2a66aea20cd81e8200010e820400581cf3b9e74f7d0f24d2314ea5dfbca94b65b2059d1ff94d97436b82d5b4a20cd81e8200010e8204000581a100888258208b0960d234bda67d52432c5d1a26aca2bfb5b9a09f966d9592a7bf0c728a1ecd584011a439a7391e34bd1bd4829f669a630276deb8cbe59f2a5ccca5190d19963bef9477e6f61e8d47438323ce9424befec3357c88908473fd332a7633ab2882c006825820618b625df30de53895ff29e7a3770dca56c2ff066d4aa05a6971905deecef6db58405cde79e14b9c033276fb503aaf6ae84fd0142d63e01c0a81ec1fb0794874184c2e3ac0fca64274f01be1ff3b7a93d2e7df60b485deb71fa8549a8ad879b0cb0782582069a14b724409e0ceef671c76ec4f8bce7509b5919bb971b3855bf92ca56532225840ecd0ea504800f96b34cc42742b1bd45990fa0068161c9cce3fb0703568c7dfe2a9283c02e63d0593bab15fa34fe9b732ad1915019d0f2d05a0fd0a570aa14205825820d1a8de6caa8fd9b175c59862ecdd5abcd0477b84b82a0e52faecc6b3c85100a4584060a4389a2a3ef54f7060c638a4268b5c7e2042bde1d1c7dc9ae9d29ffbe8bb9170fc929f27e3b0b298d42f34035fd3c149c1ede0fce7ec2981c3c882123f180e8258209aae625d4d15bcb3733d420e064f1cd338f386e0af049fcd42b455a69d28ad3658407e986eef76c9dcfb2483ca3fbe299f224c51a58da94b85ba1fcba41b384691b4cde236ca0d72237a2a21fe373a0d68c69ec490f0628cb6523b0263ca3338fc0a825820942bb3aaab0f6442b906b65ba6ddbf7969caa662d90968926211a3d56532f11d584090b5745d1007bfc524ffc53dfa17e58483ff74e9d37275f0b9e9ca084e180e2c2799b7947dcdb34774836719ea897ee4bd3e38b7e52513084ef61dfd1ead3809825820d4dd69a41071bc2dc8e64a97f4bd6379524ce0c2b665728043a067e34d3e218a5840d9b5a70f1f14b084385930fa47ed66ed0c8237812825f6c3923bdc702ab1f219cc4583b8c0e5d291cfd3e0ae586f4e98d5e87d251304ed3afd1c088c129a190f8258208ef320c2df6654a6188c45e9c639c0a686bf5a865295587d399dfeb05fe74ab65840a59197afd5188eba40323d57246103eda1bb231a4df0879e6b1c3ce512978af0c6e33355f53bb9db0e6f85cc8d835355b6b30af9dde11a94c8c7ed2c635a7603a0").unwrap();
        let _block = MultiEraBlock::from_explicit_network_cbor_bytes(&bytes).unwrap();
    }

    #[test]
    fn mary_network_block_parse() {
        let bytes = hex::decode("820484828f1a0052801c1a016001265820a650a3f398ba4a9427ec8c293e9f7156d81fd2f7ca849014d8d2c1156c359b3a582089c29f8c4af27b7accbe589747820134ebbaa1caf3ce949270a3d0c7dcfd541b5820a90a190462fc24c92f5adb29cf3bca2aa060c6d3ae5d7842b4e9248ed19c1a3a825840d2faeb342bc9ac044821d9e314459d88dfd77a707aaeaa353ab274b532a21c6c071f9ad8521367e7233ca85c52eedb9a119d741645952dff57c1253cced7872458501b0f56358eb58e2e6f92eb2ce894d2e938a39d7502315526460b9b7ca8edbec4e37e0c0261cb0e878e1a88a6aa1425e026d28efd8b899ce8920663068ed499bbfa5438bfdb668634e295caa5264154038258408b048dd6774460ba3413d5fdd94fcab36dd1fe1452035ed8dbd3a551eade6f137415a15e68039af8393253a60e2eeb43b1cfb548897512f38fdadf2836e23ace5850809bf118c20c5d1b656a54ec6605e30b305f58ad9110a3fabc4d1ce0961448f0b5c10a0fe6a2132cbc36e90041280812bd8744a64a75e10a48a47a90aaeb8729655ecd978fb89c5966ce4d861161b80a193f1c5820669896257fb55857bf716873c9a4b6cca319aacafd9680f482fec6f8666f119f58208cada7e2b5f6745ad1365a84a077309a1224be0fcd72a1797aa6837160b74c520218ac5840d342d671680b51f4b6d41d329d320db58c5ba02cea9ce68eafc8a998724c5187f8599f0915b89cf13e7294b101af482f21ffc52e4247a3f2a02db8bd5853c60804005901c0b59b277aa9c4e678274a6dfe4d8adf76b1d80c0122940c38b6f6d13e65e0c0816f2bbe29831620034a5c904d814e3508d3f1fad63f6ba46c46da36844a320e0e0f7df1a6d6fd3180c1ddd147a526e52485797c575a43885f7d6476745587b53bbb450683e07e2d3d7bb70cb468cf04aa8a497367cfdab2fb62ebd72769894f8baa3e1ae4f30ad5b6fc0a5f256c90623182d673862b8e4a969dae96d6868593e9ebdf144ea940b17b994a2a4c4f0d807cf9ae96354e44cb57ed8cc4007969d4848ef1f32da01f5f74c598cb871a1dfab37bc8a676333e3a28b85a329a022b7dde840b4416b7f93db15850150d311576e0fd4c750544b25fc7f2a3a049b4be53e7b9e35f0046edef313073c7a63d8c90685c519431d8798aeb8b418360de4a8a52d00cb1e6a672016f41706471e3c729e462eebd4d102878bc5568b4345f8a15772dbcd704fe70293a91485b2b9f436682bb7813ba73a754042f0faed80392e7eabf5e2c4150cda9e13e518b89e0e15689f2e6e8f75228266527d02f747509d5b47a85313697dfce521fb4781888ff88dd6075128f8955ef5f2364bb006c86424d902581dcf384fd5a5f8ca6c56e1cb6cac4e0099c9c1cd3529b20acf1176f2a0a9fa50082825820cea49c588ff5c48394741f8f6ab9f5441f0cd5cd7d09103a1e435ec9c6f4987900825820cea49c588ff5c48394741f8f6ab9f5441f0cd5cd7d09103a1e435ec9c6f498790101828258390131c44f81799a6550aa93224ccb95de995ea0c210daa346907ecd744531c44f81799a6550aa93224ccb95de995ea0c210daa346907ecd74451a000f42408258390131c44f81799a6550aa93224ccb95de995ea0c210daa346907ecd744531c44f81799a6550aa93224ccb95de995ea0c210daa346907ecd74451a3a57f33b021a0003096c031a05f5e100048182018200581c31c44f81799a6550aa93224ccb95de995ea0c210daa346907ecd7445a40081825820c7993fb38a504159edeb4c5d256034af8f6844e1bac1bbfe6a3dfa174edda64101018282583901d1e9744c80ad49dbbe6e1025571955c9585f0f05ff3245113123ee18b5cd29eb688d274dd12e80ccb3e9024577ad381640217f419e62d3e21a3b9aca0082584c82d818584283581c17037e3f8f2d06e5971ad9bd6e067b3934ebbf7bb75c1f90a67a1858a101581e581c13354432f02c83669eab035130755a29ef6e6956a5f94b2917e7dac1001a01ebb7bb1ae86ceee6021a0002a56d031a01601cb3a40081825820962ec940b10ff86b3a7b888c2290c5f889e8d0bba10dd6441b1fe7c9b988295d01018282583901e87d31beb81288e70fd7c5f7289eb63e32842ddfd6e8832483b1c1dac411474c6d6095c5f5bcd1dec91ce5e5de9dc5b04eff330a9389a0e21b000000028257646082581d6182200447d8bab38065e5b0cc2161a195e15a8e46009292e56c5205511b000000795e56e97e021a00028d31031a01e13380a50081825820205275e9b2dc141b2974230ac9f5d742c08f71aeec34ef8cbddbc9b9ac7c811e05018182583901b0afa1daa28b7b8ebfd03bdc3143aa4fb080e9bb30f0ae4e05a7e81abcb1d2a6776790d3d161032e2701a57a5239db241a83ccd1b9a007471a13a3a597021a0002ac21031a01601cb8048282008200581cbcb1d2a6776790d3d161032e2701a57a5239db241a83ccd1b9a0074783028200581cbcb1d2a6776790d3d161032e2701a57a5239db241a83ccd1b9a00747581c92fcdd76d1d9d13910cbe1930765543984e6f4ef9b7efed3cc6536a1a5008182582084bfc702d8530383ba64eba7a46883512f27d2042de6da21d28e0f430273e51a01018182583901e28e7c8bea3c201f7083aedf848b6cf2573360f9c841c547675e45f9a06792f94c463a823cadf5fe8276b5fb2746b5116970052b42b626f41a007765df021a0002ac21031a01601cbc048282008200581ca06792f94c463a823cadf5fe8276b5fb2746b5116970052b42b626f483028200581ca06792f94c463a823cadf5fe8276b5fb2746b5116970052b42b626f4581cd5b90a198d2411b5c82fbdfee5f94b86de49a8589bfa7720b9de358fa50081825820fe223726527c92a5a1b5787d8b38c8c1393e52d57156a5cce80f7453bf131fae000181825839017e3b076b30e6c3823829edba5c997036164f66f05b5d9b6c16c7aaec425aff7bbe040f54c7379d84ff176827bfb9a3ec848778d3efe243541a0794d316021a0002a649031a01601cc0048183028200581c425aff7bbe040f54c7379d84ff176827bfb9a3ec848778d3efe24354581cd5b90a198d2411b5c82fbdfee5f94b86de49a8589bfa7720b9de358fa500818258204fc7dacccc18cac885db1e6af59731a3e341eef8cbc5ead540e4bfb5687e70e40001818258390185a2c9f1f1b7de2ac3dbcc4a4f52280ee56f222ca903f6bd3864ff8e212bc6c241fc8d39bebff6a0befdf7c138c3409bb1a2f4958b5c723d1a02d71996021a0002a649031a01601ccf048183028200581c212bc6c241fc8d39bebff6a0befdf7c138c3409bb1a2f4958b5c723d581cff069b3ecf738d4adfe83f7c20c64b82f94f7285ee41eedb2bfd297ba5008182582018ef74bba9688ce8462c9454a9e5bc7a76e7617b909c46f81dc4dbbec1e552ee0001818258390115fd6f69054f0c262c40bafc34e114e0f7e7fcc31ca3933768ed42224e682ff9ed90c336e070535254552dc9688343e337d629b6728ea17b1b0000000129d2de56021a0002a389031a01601cbb048183028200581c4e682ff9ed90c336e070535254552dc9688343e337d629b6728ea17b581c490353aa6b85efb28922acd9e0ee1dcf6d0c269b9f0583718b0274baa50081825820d9a8ae2194e2e25e8079a04a4694e2679464a4f51512863a0008a35a85762ff00001818258390187be75696aea59b41b36415d0b7c8b7a0c21fe3dd5a63939acfbc3a0285f7ef46037e7fd61f04d19b4cabbc7ff7c2a7f8279a11f50422a95821a003d0900a1581c00000002df633853f6a47465c9496721d2d5b1291b8398016c0e87aea1476e7574636f696e01021a000f4240031a017290bf09a1581c00000002df633853f6a47465c9496721d2d5b1291b8398016c0e87aea1476e7574636f696e01a50081825820717ada10af06716ba68f6289396079a70abe95768fce36bf79c3ca6195a6b9300001818258390147d265cf01a2f56fe8506f3c33ea576db7c225b5d9d8d7a527b937f4f25eb703e8d3c42abc7ceb73e664aead7ccd3cb69e9c805a3be583d9821a004986e3a1581c3a9241cd79895e3a8d65261b40077d4437ce71e9d7c8c6c00e3f658ea1494669727374636f696e01021a0002c45d031a0160029f09a1581c3a9241cd79895e3a8d65261b40077d4437ce71e9d7c8c6c00e3f658ea1494669727374636f696e01a40081825820bec130129321240bc13ed66180a4e9d6b02d8484092d253db96df7ff6dcc8daf01018282583901566761ba7978aeddc6563246b72484a7daa234e5a2be38c111337a9ceffd4195b60e4b3c4887004c216805e8cc92a661533741332b4e75c41aacda7d0082584c82d818584283581ce627a88961c4397b233b92265eb8cdcbd5b99f21671ab0e794b7d824a101581e581c3083349abf53ead6314f8c5024331084c69650e4671c4f4f93e3f3f8001ada0a01ed1b000000f3e5242a86021a0002a56d031a01601cdea50081825820fadf5b93f8c4eb070a1a61b2b4e90db5f8ff469e30032d079c02797e98d8b1fc00018182583901afe3fc534a6fefb90fb878a21344fe5009cb1e46903d26f5df47b6069965122454f8bc00cfe28233b9ef1fa74d70ce37e87a22a336b0c8311a03da7fd6021a0002a649031a01601ce8048183028200581c9965122454f8bc00cfe28233b9ef1fa74d70ce37e87a22a336b0c831581c42c3740db23c3fc36cb9f63aa8070ec01417b4d22893463c8c8a21d4a50081825820966380d07c594d78a1a0b413de89d411409ea84fac9e3bed1e67cde49fcc362e181b018182583901537e62a502a70427e04a7897f5bd83321f6a1d81661f39c555536609cb6633dd90135e13a7e92001f79ac10645f719c90b3e13ef55a2b0671a23b15d77021a0002a649031a01601cef048183028200581ccb6633dd90135e13a7e92001f79ac10645f719c90b3e13ef55a2b067581c000000f66e28b0f18aef20555f4c4954234e3270dfbbdcc13f54e799a50081825820aa3462c62eaf606034a9da1829c2bcc487ea78fab9c27ff68e25c813c27ec96b01018182581d615383c9659d70a3c8f70c9809b94f3bcea3594c2abbf9b3bdf3dcb22d821a0020bf46a1581c02f68378e37af4545d027d0a9fa5581ac682897a3fc1f6d8f936ed2ba14541544144411a000f4240021a0002be2d031a0161872009a1581c02f68378e37af4545d027d0a9fa5581ac682897a3fc1f6d8f936ed2ba14541544144411a000f4240a50081825820fad00fddbf64b68bc6001a0a7f3d02b2ae98bdc75c09ff93af058a3bdf0d780700018182583901f992fc53ddc5a14748e2b232ed73ec48703f4575371af8ff8df6c28888aaaa1c15fce4ffada3ef15dd73a2c9f6368ff26f091778902de9c51ab2916075021a0002a649031a01601cf0048183028200581c88aaaa1c15fce4ffada3ef15dd73a2c9f6368ff26f091778902de9c5581c86a19d5418cd07294af7d71386cf14a84e0478fbfde5b89fe4d98a58a50081825820aa3462c62eaf606034a9da1829c2bcc487ea78fab9c27ff68e25c813c27ec96b00018182581d61a12e3551371670d5d44a5fe3c319d1f8f55ff06404fff9cc99a7a675821a002368d3a1581ce8e62d329e73190190c3e323fb5c9fb98ee55f0676332ba949f29d72a145464952535401021a0002bccd031a0161872009a1581ce8e62d329e73190190c3e323fb5c9fb98ee55f0676332ba949f29d72a145464952535401a50081825820dd0619f4d7773d96b4a34b8a514f89c6df82b978da73bcf6fe973516bc0ae43800018182581d61ba91d25d8a27dc19a3cf37f8226ba215bc21bc199b18eff513b11fa9821a002b08eba1581cac3f4224723e2ed9d166478662f6e48bae9ddf0fc5ee58f54f6c3229a14443454e541a00989680021a0002bdd5031a016002d809a1581cac3f4224723e2ed9d166478662f6e48bae9ddf0fc5ee58f54f6c3229a14443454e541a00989680a50081825820d0d07a3ecbc6ecb9b14d7211683805e82961403ddb8a416dcfc9caf231c6be4700018182583901b3dd7745334afca9aed1422975fc56dbf02d0dca784f0354c48e09d519cd43b2cb25daf869335fc988eeae8914859525a6fac0bbd1c6c3f31a06e4b296021a0002a649031a01601ced048183028200581c19cd43b2cb25daf869335fc988eeae8914859525a6fac0bbd1c6c3f3581c42c3740db23c3fc36cb9f63aa8070ec01417b4d22893463c8c8a21d4a40081825820ddc7c5fccc3f5a401b84bf76e8b2ee733bacbb55d1348e9643f2356c5857b7be0101828258390171aa776aa9070a4f464e2177f714922a2e200a9ee8d3a6fa8298d752cd0815b4fb0047f18ca403069546f672bb782e341085a9c534878be91a00b8bbf582584c82d818584283581c956364006fab28cdf4128a494589460750d980246d5b926ae5bf70d8a101581e581cd8d97175289f3a1d7513cd61b4dac6ad9324abd85fa1f563e44c3538001a1e828a601b000000013985db1a021a0002a56d031a01601d07a40081825820f757d89498b8cdfbed2f0c309b2a3253d94c18c0f586823f0864ef6a82803c3a01018282583901166347f6af974dc4a27f537891a91e17117524737ec3933eafbfeaf8d57703b78a21c95990c537df886d990cdabd223361bf4a1f6bfee80d1a447dbc2382584c82d818584283581cd213ed895eaccb222835a0e5dbb2122d7ba216690767ad526523aa7aa101581e581cd8d97175289f3a44642b7761173cbe9cfa81a2bc0a18ea4c8d642774001a7cd9bcf81b00000001498c3a85021a0002a56d031a01601d08a400888258202d5609e8b9db3466002110c218194f56fe925ad28a7bca415af5d3e133e14cdb008258206714429f3418b637106e6c5379a9825bde2c87303ca2860b4d5dabe2615103e9008258206bb277e875aede63feceed2db8f2f13f5aef438ea856ace877fa888f686446ed0182582081eefee68c890a4e078286e06362d16c266d37337fc8023043de1669d6164e7f0082582091fe40cbab55d1a60bba9653bd4ec9fc438a41dc8ee79b5f520835b603945fe500825820d98b4fec924debfe4e876fb261619c881cc80e7387347be426c4071660e7dc5600825820db1cc1172030e556a79865c9626649788e98fad289beed5ebdbc7cc9f830e5f400825820dc8f14f65efd45361365f05e358577e203df26465f58142d8bc95a66fe12168700018282584c82d818584283581c422e36209e2c33d240c71c85360f2305486fae2833344b97596bf9dea101581e581cd8d97175289f3a14d655b2617089ce27d2df668f2901bc633cf7fcf4001ae4ee58001a8e33932382584c82d818584283581c694e4bd2a5c3efe1f01a18b8d3552a17270918def72b2a5bdff1e9a6a101581e581cc9a4c68c636468073fa4ab51d4929ad5d81a53926f36130227f6d96f001ac1af646e1a71a5d5e1021a0003b161031a01601d09a50081825820aa3deb58e2bc6e239daac2026bf61f4501f027d9a4c093ee4d06470cc01bdf7e0101818258390121107d5b4400bdfca0c9c059192ac30aeff89c41e4686508227815ae2ca29e25bdd48979b0c887a45bfb7e73b472bca2baa96c848ed0d24a821a04453bfea1581c12e65fa3585d80cba39dcf4f59363bb68b77f9d3c0784734427b1517a14454534c411a1961cb09021a0002bdd5031a01601af309a1581c12e65fa3585d80cba39dcf4f59363bb68b77f9d3c0784734427b1517a14454534c411a1961cb09a40081825820225b984b3573c95a24470387c6fba632a000e364b06ffd5da34a3a82870a3d58000181825839013780f8f1bcfd0ea42ead706430a5d80b395ef2ca5154051b00893c25fdead883eddc968d0f8247b53b1c518503d6fd172994a6c1fc9effcf821a0095ec43a1581ce12ab5cf12f95cd57b739282d06af9dd61e1b1dde1e06f0c31f02511a14667696d62616c182a021a0002aa3d09a1581ce12ab5cf12f95cd57b739282d06af9dd61e1b1dde1e06f0c31f02511a14667696d62616c182aa50081825820e770f1bb246fa6873f78c6f8c947a591d4d4173b27529b8e5df074137d66be190001818258390141201fc5df296def03b966aa58b1050c13cdc296e119db8ebc33b80f09e99f03d267b383bb0559fdfc9115d10173b2d416e837ee63fc9b3d1a21cb22f2021a0002a389031a01601cef048183028200581c09e99f03d267b383bb0559fdfc9115d10173b2d416e837ee63fc9b3d581c50a251d84927d932b9393cb462f300d215faedd5e8a5e791e869f32ca500828258206a9c72ca6229c756d6527f8abe2784d76fc8f95a2d7dfb918a0272fdc5e09583018258200c111a4ff5e61e2a7ebfe30139b0ba1995e072386f2372db15831e1f318af6160c01828258390190eb605dc7a6c460afc560592eb1ef0d1bb274252f00a6492d9ef25a90eb605dc7a6c460afc560592eb1ef0d1bb274252f00a6492d9ef25a1a000f42408258390190eb605dc7a6c460afc560592eb1ef0d1bb274252f00a6492d9ef25a90eb605dc7a6c460afc560592eb1ef0d1bb274252f00a6492d9ef25a1a3fd220b1021a00032dfc031a05f5e100048282008200581c90eb605dc7a6c460afc560592eb1ef0d1bb274252f00a6492d9ef25a83028200581c90eb605dc7a6c460afc560592eb1ef0d1bb274252f00a6492d9ef25a581c9a22c008779c455a124d86191047d35690a0ca2c094a34d90cd29542a500818258206ed4a57078193bc208f9b7ad5040f3c666e89a14c268893490359f872c273ea40301828258390145504eba43c81ae5d52d913cbe9296bdb99f9be26ca04d7c5c772cb07e6c95ee0ce36bac4915d1844fb6a26825c10735d79fada8e382fb6f1a000f424082583901f4c5adab179dcbb06314633643132559b421346805a0f38020af69737e6c95ee0ce36bac4915d1844fb6a26825c10735d79fada8e382fb6f1a159aa257021a0002bca1031a01601d100758201f40e487af7997c67a6a995e527b7d1927b4087ab797df436e360e8ea5e2ee9da50081825820f73e2d9c031e73f28fcc8f9c0ea37fb7201856b99e96f73a804b46b8b5059fe0000181825839018b86dd71fe43380954a2572c0bb1e708588fd63d6b6bc881b6e6bd939a07e9543e94c9898da2f45263d8c874584b1f29d02297ec9dd506dc1a381b5e46021a0002a649031a01601cff048183028200581c9a07e9543e94c9898da2f45263d8c874584b1f29d02297ec9dd506dc581ca89b86373838360143aba3911eaae54bf9420868965d48d447e4510aa600818258205fdc3570b9e1669d08ca01b7deaf005fc57a710fba68d5665e2572c9ce07bb2b00018182581d6129fd280480232d72b2839fcf47275e790a1a3a45dafafd8f64180612821a0696e523a1581cda8c30857834c6ae7203935b89278c532b3995245295456f993e1d24a1424c511b00001319718a5000021a0002d535031a016002d807582012acb16d984dd3c6bacc700f9b3a1416e2ed82270549d4ea62c6cd4d744d723109a1581cda8c30857834c6ae7203935b89278c532b3995245295456f993e1d24a1424c511b00001319718a5000a500818258200ea7635bc09fa06f28272f7d04a4a2edbb92def0c3414984008dfa3dd4b904a500018182583901aed1f8c7b32b85f3f52585be924ddf41ae5ac66c04b02d36c6f3fb1f9d79c247f12202e2f1c84c879435579d559c664951eadd57fac729a81a04dc25a3021a0002a649031a01601cf3048183028200581c9d79c247f12202e2f1c84c879435579d559c664951eadd57fac729a8581cd5b90a198d2411b5c82fbdfee5f94b86de49a8589bfa7720b9de358fa4008182582081bc69b2580f46681c7b2009e5d7ecae6b748eb3fb24379175293239238bd2cb0101818258390132fc37625b3ba6b080a8e4b69c124ee6f97194d8c7cabfbf9a3eda844585a49e9d3a5fb1ce7ba15eb49a5e1791dd73ec7c2ee7dc23f7c92d1b00000001043fb9d4021a0005d8e0031a01601c99a50081825820ee46783cc668263b76bb20a60e7d2b3c7174f70d2d23e4a60ac5dd528d50cdc900018182583901921b91c4faa9e77472f6d8e4cddffbe18dfbf3a0dee2ccf27d606cb04a75a86cd544df6cc79fb212713acac422570caed537e14c1d6cc0821ae66072c5021a0002a649031a01601d02048183028200581c4a75a86cd544df6cc79fb212713acac422570caed537e14c1d6cc082581ce0bfd78fd14b0336186ff6ca79a45f3619102b5bb484f7e0124dfdc0a5008182582025869369a73f07dfc6653f3271dfb6902d9e2233d98aee64a752fe4a5a070bfe0101818258390183bf1e4dce6e2a14b20b911c00f531ea0ed610e35f3a51b837c7883da0a141f63f69a266b657e285ee53f3ef6accdb1c5cd463408e35a5421a007765df021a0002ac21031a01601d35048282008200581ca0a141f63f69a266b657e285ee53f3ef6accdb1c5cd463408e35a54283028200581ca0a141f63f69a266b657e285ee53f3ef6accdb1c5cd463408e35a542581c9dfeca095f80562460252072fd8438f08936ef5a2077583a9722bd97a400818258203daadb9446b4a53d95462d727a42d517fcdd92a175486d03a40f5891324c71fc0001828258390115acbe7cfe2ce324b0a71214f7e19efa271e25462de521f41e92c8c3b694cf6923a844158166febc2d391c262f239577c1f521c8139ee7771a0098968082583901caf238555ab52df3659f6c71baec4fc76974dcc6d5d9120cbf4ac780fdccb0cb3af2a1ab6f4d39aa2f56b31b24148145602085deb4141ee41b00000001110a3e0a021a00029361031a01601d3da500818258208d2bce88c994dec05ab0052efd58d79619c2ee5b126be8313d94e14ef5bed155000182825839019f857b5feff2b13e90142bb4ac20b95063747e1df8ea5cb8b827f8b13cc658afae01d34253a65ba78b47f1641dd81c01e9e7971da00588c61a000f424082583901fc8684d555c76d67bb319003dcff7bea8ec1bde809605c45e9f218af3cc658afae01d34253a65ba78b47f1641dd81c01e9e7971da00588c61b000000047699b9cb021a0002bca1031a01601d3d07582076eaf2130c41de3827d3adbaa9e32ca3871694c4ad06695ad059b6c65dbd2925a500818258208091d82d809278eba8f7c106e484222fd17ccca4d6dd06918010f7a725eb675c03018182583901238c5a04f5fe82895cd8ba8ec0f97c5f7df2831ace50c3ff6e9a866ca836d3b75929f3165a76fe83994ece87604a3a54332ba1b72c8f55991a7355ec35021a0002a649031a01601d44048183028200581ca836d3b75929f3165a76fe83994ece87604a3a54332ba1b72c8f5599581cd4405171c1595c338c7ce5dcad8b4ed676aa6eeb1b28749e622f48e9ff9fa10083825820b578ad44d4b6dc61a2155a90fdf469fb22fd13428204b38f873ebe830c447ac35840750b4b9b81201e1113a5245156918733a8471ce83f5627ddcafcca9b7328e40b308e66762cce1c1ae4c60b37f50a6f54a995b0bf54c18800a3dd73e3ee03f203825820b578ad44d4b6dc61a2155a90fdf469fb22fd13428204b38f873ebe830c447ac35840750b4b9b81201e1113a5245156918733a8471ce83f5627ddcafcca9b7328e40b308e66762cce1c1ae4c60b37f50a6f54a995b0bf54c18800a3dd73e3ee03f203825820b578ad44d4b6dc61a2155a90fdf469fb22fd13428204b38f873ebe830c447ac35840750b4b9b81201e1113a5245156918733a8471ce83f5627ddcafcca9b7328e40b308e66762cce1c1ae4c60b37f50a6f54a995b0bf54c18800a3dd73e3ee03f203a1028184582028c7cb101f30cd81ff06f70b4df7f6f166f49449377081d07cad661562a23e2d58409c82b395b8720a15abca4efe36f5df6d0d3811e2088f980dac5cbfffa66c47697c5e9e556bc0a7bf422f038041e893b400104fa3fc81a74e3ff1721c3c5305025820eb49dbf247b0fea00b40ef47034b8931f57a6384971b631374616b73436715bb5822a101581e581c13354432f02c8338aaa56251a8ee28f360eaae7a060dec8a8b11bf6fa100818258202ba92a9b10742d8f4b04c9b3a5a282a3e4214698ef8bee30c4d689566c6a8dd0584039368d754835ad2067bb93e00948e643be47a0ca68674a5b39db6677f60871bfd8ee3612b1fb8156cbada416092784bec6d43aac0991c0b67c0f6280b133f707a10082825820b1b1c2ae30c9861bfbbc51db4a78169cc1c8f7dd5a81749d4d1b6a037a033a3658402032d883b5801c34b2564909f140498b8a77e8c2851ea35301160947364322a31954fddc3458f0b4677656b4895edcb08de614b5711f8c3b153214247497250c8258209d556eac9b29811592f4406d6005c16bf21e1405943f2643fd0754d13a0aa194584083399f7211d58dcea512f91b9dfd64e642fe70e7c2e2dd0c9b88fc51bc859e40e59f51714e1c9435dd78bd0d4b389b8ca7fd636732ed7b1f29d5de933dad7904a10082825820e2870f5b73495d7f7c8db1307ed93ed54459e074b4b55a4abdba1683fe1a5a4c5840ec1e7f11860778e705f0c023aa03738dc13b550da9837009b5d166e99ea2d609a8403de57dbb9003869e57b2a791420c183cbbac7e0ec3ffdb563364f6a0fc06825820ba474effb0b033fbae49905c08a51c55e384b3b8bb8089734ed4cf1ec88222b3584057318c5a2e7e5228c2b849cd9b141ed322bc04ee3533d82987ecc0957c57cfab646e2be1df7f60488a22b849379f569f7fb4f2e6201f3d22ebd8617670b3b901a10082825820469981c33fdc46274aeb96e344a2c8bcf0a57cdd328c3a67da8043b3bf491fc35840a9b31dbe11a5d25f6f64ca5e669f44eab13deef745202ac5a33c913947a2f768319b45e1c65b3469cf6055201f5bd17f93d2602262ac813c8df497cb9f41de0d825820d0165aab74c2263fc0dec5e5d6e4b36247b9fa8d9b5953ad796d3bf84c9d8c535840375e888c44c704aea99e28311c88820c1055eb3a8c4b3da9e74744c622d14404295b2a1b6613c38b86609e3a629eb3e6202397b18a440b51b02747b125527706a1008282582018652291879bec2d0844ee0956592b130805d61847faf89f431839c9a96dc1545840ed4558b9d054044061160af53ceb2557e2e2bf2606f1175e0dd51c491b6693bd1ceeb5a78d1147c2b5dfbb43c5f2eec7c3a4b5ec8406202b1bda4615278f1d0a825820db7125531f7eafed4aa8836e71bddc711ddbcf5279c44eb584d977914c2b66f75840b1e9156985dd52a881c28c0612fb35219963b41597c5c3c1b069d93058771aa078dcc6da0a5df4e82b2a8a4fcd5ae218f805f440cab75be95effa23345727401a10082825820130a7df57db7a74d4da0f0a98a2705f42e83f9d2d6b89ed09b08f1b85d27c0d0584051bba58eab5c549b7e71333f4666121c92571788f6f5582ae2adbeebbc41a6c3ccd9156b56326329d432a6f0f18fc3189f8ce2902809ae2c6de52a54a136f5048258201b44e7c87d8ac2de8d0bc266329d6b90349690d47d6363d984121e2152af119658404ae4ee3a586a2d6740e50e447e5606fcbd302a6edfe5c11a4bcc7e097d1d497ea4b8c0f5dafc9c934ae8ae64d39fd70ebeec9b0602d9e033f9d904d94ddd4307a20082825820f89a1eb82e913f8622f7891cd7c41bae2cd0f1279f290ca4825b9c58e9f407ff584061e4e5821b852ecffaadc664bbfdd905b54520c11e76f04e684b6702a3204797a85b4d3f42cb66669a66623972d102da6f2a78f55a0a5862b0c090a1675a880782582008c2ca6654c9e43b41b0b1560ee6a7bb4997629c2646575982934a51ecd71900584034cb5fb5b8f9f9fd339611a2d82ca35468863cd205684534e104841dca0632f8b3a350e91c6696576252128f338f8636925dd66e398a9e2df25614a89c2c5d0d018182018282051a017290bf8200581ce97316c52c85eab276fd40feacf78bc5eff74e225e744567140070c3a200828258202d31a953490b46557fbf04e0fb0e7705f9dab3cee0ce2fa22f03963971f900c758406610cb2766768795a580f3f8628cc5d5d2338adadacd26ea80839e6b909063c568cfef935742912ba70fa9d40823d67ca1311f4934d3c95f6fe4c558681cf50b82582030ba983cf6c77721a902ce28ad3caf6a26225dcee076b7c67a588f2c8b1bcac65840a42aef6cbab6ddaf37015bbd15edf398899013a547db021b4ec34c76b95fc59892a1716b16e78c7e6fa615683736bdb02e542803d73476fd5b126576f7bc5a0a01818201828200581c26bacc7b88e2b40701387c521cd0c50d5c0cfa4c6c6d7f090139575782051a0160029fa102818458204ccc48fd09bddc4d6e7d10ed604070d9892860a94775edcf2e58f2498ca20d3a5840563eef662d135c58221adf635b9528baa1bd1094b6eab711d6bb50e74276ae2a8dad8df90aa0f2818d7b6aac328d84a702d63ebd4c3147b840063b01296b0c005820675f15246fea49f6be73a4ce77d75a6a89758ba3f688698c423aaf301643e3655822a101581e581c3083349abf53eab0296b1250821d55ddaabd554a052bd9c1e7121aafa10082825820f227d91107250f417d8d08e8b0848dd551e976806f47693485fa79bcedb00e0d5840e3dd7a742684c8e9251e758a7987c476de67ae6e9b163b4de71179b779a270c3405e56ca591535c4bc60433b7d16108ec121bb68c251c6d50b99b1e8b15e940c825820c78ed7745cc8fd8d86a656ad54c66dfaf81cd197cfd3322b776818f79c3d42d75840ab24a626f435e7d7f780f7e6354d98eaf478a057b93fdc06830f47208cfd6a2bfb97a7f421a6bcc5485f90353377ccdff9440eba465f67edca79840740317e0da10082825820b38ee7ae9e93e96a67f9944549d6f6ee11958641312f08db0f2544a056c316ed5840239456ee7392721a8930fe3777541d016d960ef6f245462b957ea8e37aaeaabf8dd5205c3f3eda934c03c8e972ef42d95e9c0062beb0d6997efadc137ed0d80582582005dedfe50237facb1bd47e682560527ade3954156f4d2e5d64ba42a72661399c5840424eb36d43bde5ba3d42cd58e07c89d42c2b53ef909e35054514b3a6a8d433e130e778bf0e6bd68dbb06e88d214ebd5307cbf46425fcdc1fe51ffeb44aad8d0ba200828258204dcb549d62f4393daff6854dd2a28f3eb543d8d487ceaafec1f06db1280297145840955762edb9b09a88d0578e7c8a672dacaace39e35bc00abf87d0e32322e0c7f5b3c083520ce24128dd99af51abc8922193ec90ced47764d55a7e8a3442eb5f05825820ccb2a8e0155835fe7edcf1c35f779b1cc52c42af3529d11724a466ccfad422a358403bc918737723aeb333e69ea1cfca95f4ad92ad390ccd115ae5b3fe34ea2d2af390afb7503a944299885493a745cc7f60574bd17af598cc73a613cf5e0052e00301818200581c89d555c7a028bc560ea44e52a81c866088566f3a99c9989a5a183940a100828258201b811fa49076407607ef4b9e87bd359da13f6ed112419762256a7b2518eca7d15840b2668e06894a593fdd12493fc759054fb651d011dc407fa2328ad16a57eb9ce5ab489c20480b15267d2ea8438b996ee314d00dbcedb651cdf27592a555760400825820e77d357e39485ed9589d32c3a63d33850f9cefc314d340c1d53171e486ab21155840ab12182cafc4019032de1fbd0529e899cc1eb33555aca63e1615085b38f6519ecdfb71c03204c49e9956a442199c638844d01c6edca643a44b3c0c3066847708a2008282582008f696fcb5bf1592fa015eb7cd590c91b99e2a756a76ff608d0e717934c4a94a584000e2d9c20be4fbf547ecf7c2a6d8dd352241916bf19d62fc4ce9838d4aed1ba7bb0919c93e568453dc25aff5644ea8d2252e5b03c05d5191849e1d9e5c17d8058258209ee527fd46eb12e792272f8374d24166007b57fc2eb48faf44f6fc2dd5fc0fd15840ae768c24f4f4378ce27312aa91645aff04c58b9bf3b906e1dfc6a82f76decec1ab0015c6c5b8d8ef43ccdc5947903b43b1f20b724a57acebaf6bcd511480ca0101818200581c8a2da46df46082c69076335093c8062f9f1413f452ec43357f4f2f74a2008282582078ea7b4063f315d8e63fcee59d5cb174dee92530de2829324f37e0e798d0eb2058408bae6afbc4dd6610456fae075bb2e8ed1d2eef8318fd0d95f02f4e0bcb9c33bfc6ca3b1fc3c613d3f7993e47cfc7b4607e6ec45fea5752dd4221a35676869309825820898774aae3d21e44f456a6564dffc3ecf79db05e6048665bbddcfa047619191558405f0e2633b9aeeaa43bd447bf61980d17c6f0844a904fed7f4f812171055479abd0bd0d8b60ec54bd42cf331376c3c0c7e0717ab8c38fa95cc3531d3b7cf0fc0d018182018282051a016002d88200581c24a24be012b30b349f231a99280537f6216dd39568d6f9490bbc68afa10082825820ed753bf31e69892aadc7449863739438a6c65c0f5780e8af49fa2346f5f682af584022ac5d97daba0c0a91e3e2ffcaa11fc8f44a98fa0086b668d7ac6f6bf6169965b1262287859ae96862465f89c79c7bb0a20fe5e839c4d5225e22881333461c098258209826cf9e87fbedc7a5511329bc43f90134aecbe3961b8eb713076daccfba43b3584074d3f44dea445b8a2177a62d36097fe58ae2064d6e6a147643c1a0cf60e1d8be9e763368204f2a622ec835e692fc8eeaba6a1170a6a6031f6d5a1cc946947b0aa10281845820e4f5e0bce508be77652f050aaff468256db19610002882c021ebe98e4bd53f0c5840d92aa8d2092e1242a22004a51357921a296d17a2ad6ec4b256fb03331eddeeb46526f51ba2ae5656930c1bc1ea6410927c524cff6413c4f8d3af856e1c10250d5820267138883ed4f3a2f9204556597c00081d8e63efb8cfe61adaa71885f79381635822a101581e581cd8d97175289f3a4a354f2761ab97c9b2e00f6cbec6e4a38ff141b1f4a10281845820be5fef8d21addb5bdee84bb7eb39f3fb88f7d965b733155cb77deefbb011bf0e58401d8378d6fbf6774f9b2a24a5c1e002e0b7305af7079fc4d63986a04333a0327d9e52140e3e66bed0481d00a04d9d076bb4e38a2ff686235bd578a89c59475c0558202b2e17b3d36a369710e9c06e2c324047b9b800aed84c9e8468a4be492156ff0a5822a101581e581cd8d97175289f3a3053dd116121025a8883dcf8a137d5be07877840e4a10288845820a8e28ca1f0d2347b29d356f7b07971f2ac186e840afdec76602756a1bce5521c5840203d4657dc3794a4ea5133b2d95624a2485448864efe8dba2ffc07b952fd2fe7ffbba48ca6085bd1cd35d1f0af623e340280377dd630cbc4459cd54e3bed960558203f904efc08fe0c918e9a5df3f00a2f0234ebc8a90c5895fc0ed56d2a454739bc5822a101581e581cc9a4c68c6364682423f1545190173b86b84e2868d46f267b0b1e8fe0845820eb54c1c6073ced10be5313ff4cb771fb6306797fa4df4af09435e9f591cd97ee5840ceb3e1683a7d96061d48670470fa4487d213d8e9b5e57803e4ece8e589f5bd73416a2305c9e42ca863319ba758c6ee6c2865e24ba3546490b88ff499ad9c140b5820545084c11a5b28a706bfe75b31e88cf2309a24caffe8fb3bd8ff2277ee2a54785822a101581e581cc9a4c68c6364684c3cc39951df558b6bcc33ce72308b38b6bab88f86845820c858f21a704e4b5a60edde8425af7740dc6d5cf5323530f57ee4be627adb902b58407cbefb2016e2cebaefdbdd4428bdbee5edf74cf7a8c3fbed28987d767a4578996f262280db308b0a3c09898fb82ebd0eba71273b32fd9ae58d79b1db4c1b0f065820d5c265fce9714724f6c0ce8ccd5c0f43f2870323cd5c7ebf9dd5d6b8483c30ea5822a101581e581cc9a4c68c6364685f0392ff5133d2734f45aee261d2df93124dec753e8458205c3fe16d840ece3f698e5227e8711959d940a5a7ce5b8d97e4809875576154a75840269acd199d9d37a23b27f29601d7b54546364e85eccf1d1020d99d79e84f1df3d4b50a6bfc96ee9ac59b9384b882b6f46b1841760080d1708c1026ba256d1a00582045a7e755b9d830da12b309c12af6eecdc0ff50b0509d9217499474a8f0927e005822a101581e581cc9a4c68c636468719a4ba4511c4e348eba058563eac2859a0fead098845820fa256e041ba2d9b952d2906b37d1a12c6c831da2b508a11dbaa6957720dff3645840652cf665d74bc504cc823bbb7988638c789d083500cec4356271939c70ae23626e4c492a912ee3907e2e1fc7fc46c3d7debdc0a0b1b16d1c758dad0c8c184d035820dffce74909e33d00449db037fe758c430bbb79cd02d266257af5008f946e2e8b5822a101581e581cc9a4c68c636468726d933b519f0f1f57f51252c1121352f1275e074d845820ec7ca1ac472a76c5b815ab7bc9fe1826fe0df427b8c52f303c067a93b9f87ed958406779e08868ca456f3918eca343c3879d20f98c41da51396deaa058e6f611914978aa537c815d406188c87eac46c6b8a690740ff775851a96079a9283905b9a0a5820c9414ad96552aa45428d4a5f06cbce6c996f96cc6b8b349f854dadfec3f3a7f05822a101581e581cc9a4c68c63646805845d3251968049f4adbe7640106b0978e18d8d608458207a7745b26a1cfe5bdc99a2824c51e74f593339a1372ad876735cff1c751c5bda584046826bd4533fff11b80f1a3d302b782282253074ea44d8917a054320cf1bc66e5f0f2345b2d193b44bb3b43a6256e5c647c6165150a413107f5adcab962edd09582057f99e2553adab78c940831ffd43577dd2f0eac163ec2ab47c9e963968b18f125822a101581e581cc9a4c68c636468754868a4515b4434e3d50a38ea26e4ad20b6c3af61845820b5711a1e77783c41d00914902fdf1e1071bf5630e3531217b055db2350539dff5840f3d45617a985e0c5578380768f1ced09d7b04f1609b44e4f5498eb0430c527e7d1e30cd4c7f72a1035986032a70436580d0d75e1443bf98033e3a612515d840f5820bc9af5a13a266ab5d3b9516de27ef16a93cff7281fcb187d935cdbdc3707171b5822a101581e581cc9a4c68c636468352b66c1516fe689a3abb33464f690e32881d87b4fa2008282582050c17fe55e0e68fd94b3c2d6034fc3cff03e1228462ffded2c8ad04a816c73795840667c9678704afd1b3a1c3cb424816a223084f803de7942ecaea5ae893505324db6c0977e0b1fe0972fcd917ad7927369a109e0ba9b4cf1f46caea556f5068d05825820db69102d6cb3bfe0c8e08ec9ac127c478efdc4502ba95beaed37c5afd5161c7a5840636cf2854653b55aa61832d75904e957fbb671f3bcc5c6776fc4e25b6c62bdedc420444a24b352e496c2953f599f1a8bdacb8f3682b0b79321ab100af4f89c03018182018282051a01601af38200581c76cb8de7db67a0cf13ba419e7b571d479a92211ef1a51e3174f85b55a20082825820cdd4860d4805b8aba5b46482ca8443f48b09306d6ad363af6c2d16c3c71805d458402d3c2d5d485abf7c05f4e22713dc0cd73b1c1082616f3b008a4e610c08115ac8be399b5ed028605e20c058910f56e0f1b2538139101cdbddac5b4126571c0c08825820440fd71debdf48bf8b4c4ec9857cf2e58b622e75d9e9de43b1145165be2de8205840f1552bef6e910a3bd1056ed84cccd5ae0b55f38b33241607b936ec1c57d6b02e5363ba2add9225887eac3990886629fa91e28d2344ac3c20814cd7d11e7c4b0301818200581c7280bfc12cc21c024a3e8155d3d77227d7bf8f66ab2fa35d5799c163a1008282582021cb753a2984835ce919e28b61503736020d4c9560624c89b9b46c73194e60e15840f1562aae4e8f2bb0af5e584c086a095c16617cbd9a7e58dfbd7aa01aa87419553cfa5297041e5ef640bf0c446a719d25afe3ec908f209b357b4b289f6eb9060882582023bdf259b7ead310df7861c7a07bb5bb2bdb604fa6a830f3d611f46735a53c5a58409920b63962b8fcf0a91f514354696c26079778e584af55cabd1b8240ce05f77187b95fbb3f76205e12f899e3b32cc5d7bf208e3db43acd0347f9dc58bac58705a10084825820fce03faca39047cb29c9898d0ec776578c760cc2f85178dd1c0ec472b771f2695840b88d417d28e23eeccf6df921693546d14dce8b3de0c49d38c8d846091baa01de82c370a0ba6c4a1d1d6a418e50c37f5eff08df81b9d4a15b5ebe9ffe54e89508825820fce03faca39047cb29c9898d0ec776578c760cc2f85178dd1c0ec472b771f2695840b88d417d28e23eeccf6df921693546d14dce8b3de0c49d38c8d846091baa01de82c370a0ba6c4a1d1d6a418e50c37f5eff08df81b9d4a15b5ebe9ffe54e89508825820fce03faca39047cb29c9898d0ec776578c760cc2f85178dd1c0ec472b771f2695840b88d417d28e23eeccf6df921693546d14dce8b3de0c49d38c8d846091baa01de82c370a0ba6c4a1d1d6a418e50c37f5eff08df81b9d4a15b5ebe9ffe54e89508825820fce03faca39047cb29c9898d0ec776578c760cc2f85178dd1c0ec472b771f2695840b88d417d28e23eeccf6df921693546d14dce8b3de0c49d38c8d846091baa01de82c370a0ba6c4a1d1d6a418e50c37f5eff08df81b9d4a15b5ebe9ffe54e89508a100818258209d4c62563c9d49063d58654589c0cede283dc06e435c54f2597a4faf9b30b60258409541aa4f72c2bde14af175154da053b9b84e252a931228da32c7ca176122e361be6378ae107ecce95435194c0e3dac7c8d240e82ca85ced70d4ca566564dda0ba100828258207c27fa62b817fb421deb2aed9c76a9deced7d03d2b32c0419e5e7fc90b858513584078c97069282cc4e00efb1003c686441984405bf40dcc15a3ccc707e6849d523f773db3bdacf7e768b9cc26b1d78e9059cafa3925c3d9940e011711251b37700f825820375ffb7fe50482cfeda9d3a2d26d184e06cbe246935e3d98b609e4c68472fa195840e4a75ed875b291d6c2c918305ccff3198f52d55e11faa1ff74e522fdd7f82b9cd0e119664b107431e3bcf8819da17ac54e975dee36f7355facca9fb79e20230ea20082825820ef47ad69c3eafd8730b72528d067c0aebb5ad435132115db32870a75841d293b58403e334468bc1f6b4cdda952d2239201b9145f24b55d8f04929ec22de4ecb4ca2f034305f23a95ead8d59bcda67c3e6edb4143f0cb22ea59995c0fb6d1341a5902825820a18585026186b87151b1904fc92814ac123381a51b95d0eb7e63bf2cb90fda5e584021772d357b738f132f325b75494d96c1a2ce4b67e09f14e23dc424580f91934044e1b082a321c3255118f7953733e2992319fef5be4c5e41563c5f5b87c12109018182018282051a016002d88200581c3aa01dba02b37245f31d099806af04bb883214181fa899f5e8d34c37a10082825820fd0516dc3af9a6abfb58d34bd7f1f61a3e3985dbc1c961261729ad2565cbc64958405a14afd8f15e3c82119742870326dd10bc7896f2c109b23ca2a60e8122f58aeefe097b97ac4fa607ec932e9f26d8f0426a26589f091529db7cdc506bd320c4038258208e8ca43e788560e538f4466e8e6ac2c5561601240d74add435139a01894978e458404c72d48e6194f5380cc9964c6e35c1a20ae1be4d4da0ee8b572aa7ba788d5ff3fe3295537e2a86197682b46c56398ef5fae2dfad153d7434ed2fe88e1c99670ea10082825820ccec76e927c17585fb12a438afa5e8af7979514bb5a6a0399abb912a68dc6a895840518de30a8ea8f1c517b3e65b6599d14aefdf4f16761a0cbfb0f797c37c71518140aedfe53551c0e594164a180c84b1867351e73162076f87eca426ae6fc2fa008258203afde5867e367af6280f61713e14882fb9965ac99e5ae9e260d2e905294a62d8584037b174ef648db16331ee3c5a337338a94f5051609544a2530952660275c39c63bfadd4eaa86b8b02c1f59d6705ece3f76a86aba8b26e24c709b92a27054b9f0ea10082825820d63992be99ce336eb25748063d781f6a22bf4f1d13dfc401e609d015ea09837f58405ad8d1e444f7a19020557f2563f244ccadf0550ac31dfbaab3d8bfa6ea173fd99b9301ecd64c372995a17ba060f0181e2633d64f09d36ebebf057d712745350f8258202c726f5f75eb413f1bdeb572c7a7f328bcae6afa7b6de20d54ad31ee5f928ac65840cdeb471a5b6a0e077cb5c388d9b9f49d9ecccb8282f70c26b4c15391cde3f19181ee0c7b210e1de06b8540174c31b8d46b72f390b5615ca85e31b09b7957530ea100828258203ce587600b2a7370ee206b9d08f18ca9747493b6f2988e5db3b7ae445757fabd584054e26e2b04d2a0926bc999d3f72d6a88dd328cbd4a69b05f7018e9827db615ee0450bacc606740ec9b1e1f51cac61e73625c6ec27e326775927bc0209384420a8258203bc6300d4164c47737e8f67adf30755ac6bd72280f4ad7c9082492828e28c1b85840bad030e1e24a18863586c4fcd0f15ede43a24194a455a6b28e57857535d14ff45e94a458f05575f7a3bc92a16408cfb91b10774a83d03fbbc54bdc667714ac07a10081825820e13369244c8ac7cb81676de7e36850aae32e509803ccbd577b0ead0fd1a58fb658402eb22bff087d4d7cc5a7174863488a1d4ed50e009174ef256db73e36b54ddf8f6924e837568546624b873978978704ecc5696fdef468b42f02f21f91bc29290da100818258205fd9b2c7614d0e874ef2391728f5a72433cf9cba3c41af219cb1738ed219ceee5840257626783e4e7d22a43eadca34e763ff7c78489b62f9ab46cbb23e00c240c54f7b759585fb273a270c54583cfb9d43f111a4fae1564f3ba069e324504c17de0ea100828258208d1651b48e72722ddd7d7f60986b75e6f282967380df9962896071fd54cba4d058406b3cd308cae001c31110a5641e716444159aa21662ab9c8d1810ac5feb3d306411277519977181c69b643fac8b79d8176eb45cb6c4772f53d231177e0989d20b825820d4e9af2b16ef0dc54bf16c9a4c400d73ead7f483963cdc49c361e2dc831ef7f1584010014ff505276d178f44b95be18ee408abc01008b66528884efd8d2593ab6da568c81c0a22905668974d4e1852b263423070fa56546e6109cef7294ffca19b04ffa3181982a219ef64a3015820f2cdb189a3b0fe5ef9672cda9150138d4cce341fef699a50c1ceeed2858e41b5025820cd69f4dc88aae3bd93425b732d02ce70e02be137a5b602d51b3bd271ae779f450358390145504eba43c81ae5d52d913cbe9296bdb99f9be26ca04d7c5c772cb07e6c95ee0ce36bac4915d1844fb6a26825c10735d79fada8e382fb6f19ef65a10158400a8f985f591e3e1b6c733cb2a8e3e5bba7743639cb93acb0ecc328f19057e6ec2369a2cecf0599284aab759cf635c638368266bce4457105d9d1a5009f46cb0980181b82a119019b8378295468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b781c6f66207365636f6e64206261696c6f757420666f722062616e6b732e715468616e6b20796f75205361746f73686980182182a219ef64a301582015f305f200182cd51c2d6b21c145f867d5a1b48f1af064470284853183157147025820f57b7bd8b3473347b31fe904684b7c3e30c686610e976f0e29de2621097358c1035839019f857b5feff2b13e90142bb4ac20b95063747e1df8ea5cb8b827f8b13cc658afae01d34253a65ba78b47f1641dd81c01e9e7971da00588c619ef65a1015840b9a388a8ca0d6d9242f8968a77c5c5215a5738c5c3a02e8a74656c3a2ce6fb3dbf47e3e00a0d82d7f6c4db5ece29b2ca506d31218f23a58020b6f7ff68cb0d0380").unwrap();
        let _block = MultiEraBlock::from_explicit_network_cbor_bytes(&bytes).unwrap();
    }

    #[test]
    fn alonzo_network_block_parse() {
        let bytes = hex::decode("820585828f19fd871a001518165820af5fddc7d16a349e1a2af8ba89f4f5d3273955a13095b3709ef6e3db576a0b335820a9d974fd26bfaf385749113f260271430276bed6ef4dad6968535de6778471ce582009e142413e6a20c48dcf0bc1e1604d22ec6c1682802212c130bd8b0888fa925a825840e41904a15f3af92d9284737d2c30c2d3358bbac4bf759076b979fea06c1e39b1e81917fc5fc2b5630cbb89a381fb9141ec13757febde945eb5f9f3f7a73f7e7c5850c6e01289e3ec8b3e33c126eaa7bc0430eeaa54a632f1af449dd6d36edb308747333100f3fa9ca9953d0fe3b5b13ee032ce3023d65843d562b2fd3eccd8dbbd9f05247874e3e286312a530db6ab34bf0c8258400172d43b873ad7d44e9f675f9e07ee10ee17d660fecf53cb82c25a370206068cfc50f7d42b1b7dc6393c5d73dd3da1e005de6890d0675a9af4a53b50e54085335850bb00468778899c124a548ff45196cb903ca814abff3c34d40eb6d8f5e1d7891e9deaee26b67f05ee32942f2d7c2eba1f072c2a30601b4e4df944783e9d4f1c19f213ba0b25ae7b474f76cb8fa73f630604582029571d16f081709b3c48651860077bebf9340abb3fc7133443c54f1f5a5edcf15820ac442a832ec9084839a88cfe8ffc99bd22003f6a8cf6bd5b34bb309d0166e82d00005840d2c4345086faa2c56d503d773b49dddc239c599bddf41afabf0ee5410dd8ead37dd547bde8ce918580dd18721afa5f66fd954f5b35be8c24ad696089d222070806005901c02cffe4eb8a1f390974820aae493ac554edda85ce985505854bf382fa402a3ab24e73d28edb5ad6eda3e99b51de20f541ea1d7aecd2afe90608052f12aea0e90f5bab1593fcd77c4419476e27180477fd5f63a494d4914c1116b4278796ff1326a039ffdd789e2ab6943391a0ed144fcba0d521d48f26fd9666b7343731e7d0dbcaa78a1c25750c47bc5802b12ecb084b9558a0b9c6f04155e1fb7448a8a9d090989c20a173e318ebe1cfb6bfe8f701075ef10feb150cbfd9d21aac71a1e09e9d483c45aee59d66e04d4004729b0b2160d40b87e1e83dd394fbf7cd0fbe681865b240d053b80c17920f03ff09feae44700a1fe2b44cc5b6867e56c3fa1597ca952f875b148c493c0b2d358cb70f53ba036c688985b241929ccb10c39243b242d854bf46a36a1ed766c0b935f2bdb49004d9f383b8790d5abebaf5a42e0e1517a726b1efbb04dc051c66c3105437fd1992826ec790623843d026bc3c7827331ed1cea274c1de9b0b293c47e0c664e4d25414d23fc4fc9441c860d46e72e3739c6280d76e7e9752d362f98998eca3fdf032f55bad3125e528acca2fcc24a62cd30fe45889e058430eca4766499f404836687282a875a39922b46ef8795387574d528080a080").unwrap();
        let _block = MultiEraBlock::from_explicit_network_cbor_bytes(&bytes).unwrap();
    }

    #[test]
    fn babbage_network_block_parse() {
        let bytes = hex::decode("820685828a1a0002a1d21a00360d985820f93e682d5b91a94d8660e748aef229c19cb285bfb9830db48941d6a78183d81f58209691ed9d98a5b79d5bc46c4496a6dba7e103f668f525c8349b6b92676cb3eae45820ea49e4652c460b9ee6daafefc999ca667fbe5eb5d7a7aeabbdff6fe19c1a3c9f825840c448c14556645bfb1c2de174b4f375bd8e5c27a41f26661b17d05ab8d30e9f2527681555316ce65a7e4dfb3cf83f57144a95df44d1fa67293c31f0ab15c4f4b55850c15e1be17e969159dd7ea5df78af4154767e6c13e4f41cf67d9bc305f6f4d0e1ec56df71df7e00ab51d9f7de9bdefc88e60a155f4af7225007146c38cff4d5ca7a8067214baba73aa5a2f726da72360904582029571d16f081709b3c48651860077bebf9340abb3fc7133443c54f1f5a5edcf18458200481896957dda8d02d825448eb22aad5e106114d924f49923b2fd68e6b3254d3000058403b0db934233f93c43de54a63f8be99bd21da431dcea996a01213ada37e66c6a3930a95399ce083875ba95ffa58cb8ca9b83c92f7d440adb968d8569d74fe90018207005901c0c8e3bb3da1d72fa8a736a6a66bec894083a7f74b388181c54b8faaaf7ccd020acd7a7fbde4a2505f6fbfa907a8bca491219713841720e122a94a057ad065850e88894f7b7985335bd2e3c6bbed5a3fa7fd526a8afafd78fd04dfaef371ad55d0d3d0f1707db5e6773f3ccb896d9f81b1428c3dbcf693c54580f91bb93364272349c08980f71e74e74f8d7863a29cee3fbd821642c14285d8198c68fdbc0a193bf63d97c7a5481ce7da17c97f43a7b98d0cad8c3af26ae4f9ac695fab06ca34685ec57339673d084e0417f844747025f4f5b42cbd883159a3d52dab9dafdfb37b0136757b67ec4e1a6768c43cee5937b81e03224aa6909a9a3254cdf33887914d67723f9a5a4505ac41b658cec10d6c2949e3d3c58bc7a27b64eb8407cbdad7806db704e7ee172405ddaa634119f878541d2f19674dcd0aebf4e6e1091635ce177489742c28d61a4698edaea3364b92b03a73838ca24ce83913727154dae3c2ee4a331997dacd155753f014bc98cfa25b8d84c808654852b5e1e74e251651e30063a6451a676795aa421ea59daa711a7ff0d4140de86bc89969f717d461690c341da4cd440818205961ed9cc9970f52b10ac1b1c3ed3e0d0170582daec84ef2068080a080").unwrap();
        let _block = MultiEraBlock::from_explicit_network_cbor_bytes(&bytes).unwrap();
    }

    #[test]
    fn conway_network_block_parse() {
        let bytes = hex::decode("820785828a1a0006ddf41a0089a98c58205172bc8843f4b565ecf0f72f86944afb0103d126bd1b6940de33a6125988be665820149174cec3572f88abaa0e3c22aa992f2f254db15ebfea54f93b59c71151801a58207e7f6a8978acb0ed37219cc5a917f8cd5bf6fd53538254ac921790e159baf8ad825840438fe265be4b13d6e1f5af3a6fe8a7a4c0ef1b832b614f697b80def3467db19740640fc9aed4ef158230ad910af46e597de22030a2c47556d17805c13866d7ca5850ab4849c172cf814f266bab8ae5327902bd481a0bfab03141d00c2e4e810732bea4739f94e068621a0573870379c51e76229e8cc345163781f8cb43683e1a79805ed84576fc3bf3fb4b431074241dac0519010858200ea62bbbba0549c668a54579873e3c35e64a899007f191bdf17b43bd01b4c7a28458205f7a4914a82f0120d63e3e63a235ad20ed871d816ba28f032c75f69e3a7df3e20018445840a437a97e0dd7b08a7744107cc329a6a661d6c1c844d0f35e72906f5e5762260b4189bc49be29993aa0b64a338e5fa8462953c55a2b4b4299edf40c8617f1e8038209005901c0aba65daed3986c74218c9e2d026e9285fbc5276efcdab8f41358df8cd7879574257f95b76540de3ac02d24a5e00f05327621e9fd01c6cf8f302771fe069b3b03c7e3637ea1b36208c4ab40b97866d05055ee2d8254fb44f54e26bef97b9e412a1377bb2a7224f8ed5430a83bdaeb963a9c492f721cb499a42a9dc49957315aa7144f304fe3af8f28a619d9b48e57293fd18332d0a277b8beb1651f502f6617d400c63dc3c0f3de0fc2a52b6c05a0557d5f1bf907a15a85ef5bb6a91406f01f2d63801e68e3fb16a8cf654afc9c7f9da3958c79fca1806405802a743377c89bb933ac69598ce1153cc7dda40a504ae5555af78e9052e8b767c7d8dbb943bffbf7093896c7238aeb4031bd72e794c23c2710b973e351ac5004ebf014b7a0d3daaa1070e7ed0288f45e5c776f174f826c0c810cff417b2ce0767f6210dc26710e4958f68800addfe210f1f7388edd1b6b96c9ad9e2a615cdc6d472daa421449c33eedda2a193755b70258fca6a9ae1a4cc0cd16bf1e37664f9562ef82c458cd170a71c2c45ff0cba71736d78d60abb33febace57b324410260f179cb8bd837c80ce114339d89daecbe578a2518877dd2b4109624153c717f1f4c704386a88e3590c81a40081825820917aaf395181a0359ca8ee314fe8355c5e2239d66483b164c62cac2e41ea4a0d000181a200583900db1bc3c3f99ce68977ceaf27ab4dd917123ef9e73f85c304236eab2397a993b0c8166aa8c48345be19257a4025ab764c86e799beab15b303011b0000000253fcbcba021a00028bfd048184108200581c97a993b0c8166aa8c48345be19257a4025ab764c86e799beab15b30300f681a100818258205f89ea8c6ab580e2e7a32c3586869eb95fae54f42ac982639b6665359601f63e58401c012befc2a4d4e22e6c7be4483de4d7ac550050ac3ff7d481c503cef64ce234a76ea6dcbd70f9a79de6adb869b3599d28d2cf351643a5cc6e36205d39efc50da080").unwrap();
        let _block = MultiEraBlock::from_explicit_network_cbor_bytes(&bytes).unwrap();
    }

    #[test]
    fn byron_block_hash() {
        let bytes = hex::decode("820183851a2d964a095820d667b66b3234630d7c43a29d8ac1431cc74929218456688c4833664563d7f3af84830058200e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a85820afc0da64183bf2664f3d4eec7238d524ba607faeeab24fc100eb861dba69971b83025820d36a2619a672494604e11bb447cbcf5231e9f2ba25c2169177edc941bd50ad6c5820d36a2619a672494604e11bb447cbcf5231e9f2ba25c2169177edc941bd50ad6c5820afc0da64183bf2664f3d4eec7238d524ba607faeeab24fc100eb861dba69971b58204e66280cd94d591072349bec0a3090a53aa945562efb6d08d56e53654b0e40988482071945b358400bdb1f5ef3d994037593f2266255f134a564658bb2df814b3b9cefb96da34fa9c888591c85b770fd36726d5f3d991c668828affc7bbe0872fd699136e664d9d8811a00029434820282840058400bdb1f5ef3d994037593f2266255f134a564658bb2df814b3b9cefb96da34fa9c888591c85b770fd36726d5f3d991c668828affc7bbe0872fd699136e664d9d858405fddeedade2714d6db2f9e1104743d2d8d818ecddc306e176108db14caadd441b457d5840c60f8840b99c8f78c290ae229d4f8431e678ba7a545c35607b94ddb5840552741f728196e62f218047b944b24ce4d374300d04b9b281426f55aa000d53ded66989ad5ea0908e6ff6492001ff18ece6c7040a934060759e9ae09863bf20358407ac79f732d0a4956c5214c59d36ce7cd3a7887784c825caea7f424fd0b0984b5a0d5edee4257a8de1c56591b623ff266a64bf3d64d5c63f46120abb1f3ef24038483000000826a63617264616e6f2d736c02a058204ba92aa320c60acc9ad7b9a64f2eda55c4d2ec28e604faf186708b4f0c4e8edf849fff8302a0d90102809fff82809fff81a0").unwrap();
        let block = MultiEraBlock::from_explicit_network_cbor_bytes(&bytes).unwrap();
        assert_eq!(
            hex::encode(block.hash()),
            "42f01b7eb6e7a822427057b5f7578a8f8048fb599642e562f38aa65da1aba0d5"
        );
    }

    #[test]
    fn sancho_testnet_alonzo_block_with_babbage_costmodels() {
        let bytes = hex::decode("820585828f1910e21a000151bc5820ab3a121e39d38cd844ed40988b435d5776d2be3e1a3ff15ee8b6f6099b6eefbc58201aba5fea24ea77d696737416bdfb6136fda5f0fa745f6b4aa682283e7d3e0f2d58200bd8f3ceb4057507f67c7c5730a62d691359983cae18d02f94ca80a020c9a6e682584014bc82eb1ada0c58bf1458140a81d569e9889dc303b9ef71afaa27615bd6b1ef5583ed74a14203b195404977edf201ce92f72bbb915ae5e1a187b74adf1e009f5850bb6a6709fe6fecf871c1f2dd3a657d3d2a8afc1f76f60d7d4d64706396f7b15ed9a628bc9a15a5dd444b316872a9d8bcbe90c01a826ad15f4c8b7f647aaed832d18efa5607b27a17ad432773fbc8430a8258409428e60adabd462ceba6c7ad8a68e8c444b1e66c31ec3e498baddf7f58a7a57b663db391f9b181457148d3704d6fc62ab71615ebf3acf89e90336f2974e72bc8585011bdede95b5b188d5649467ad62ff4df95068ae5d9462f54211d4522dc812cddba98bc7f9e0187e550e4c424d31651a63c1692b6ef335d7346d7e3f499f496beb7f402356510c4b00ab3ed932069ee03190d9158201d39330aaf40049b703f7c1a9b2eb6c9f84f11cc8d8f456aafb6266f757d685a58201dc1cddf59c04f8744d412c5490f6b87367fa109a22140b4a2987edd1433f75f00005840556655ab6e794c101e840475c9fba24c75ee06c7b6932ca0b596e05ea973d7c77d0bb0af98ed08717fec6d365b6462817567dbbe484c1623cdee61c27e5dcd0107025901c04fbcd1a7d282498822e5ce754b2cf976551678ab7984b02d1a6c66556329667b4ac9a702c44e17dd68e678825afb2696d8c56cb94a1619edde2b5b570f818008060b62df089138b25a9130eaf11ce9467830b9194e4cd0fc50a61c190e50011297fdefb16849ab3c53191a8d254eee51109417c3be4605ffe795b6ff96aa0778a949a342a53e93ba4143be6b10df056eb80b60911490a48c54710081f9f2d21d6a93b21952ee892bd3a70954ecc7d3f733321a572df04c1b163b74c7d4280e00e4784b35077b8bac624644de3d4fe3dbf82fb4ff22a8e5ee9f30ac67caef4b8cef2021206c6f2395109fa34d164c926380fd88385a8d166ea7d0de46e179bdfcbf6c201aa8de5a44f211900909910b8add25494f26994ccde53b4812a9ec329e682507a3e272dcb6ebc3973ccb4a23e9dd0eedf3a5d3b713568c4ae02dbeaa9e5b4637ca5c1a03ae030c2b504cfa932476383a790c91e79ee05321113e01447a0783deedd9b6d0a14ea781719f7c2a90f09f400d52621234c40803a3c30ed3c4c0acedd89180822a9fdf6147faee7458f8feb3334878c569f2503925f147948066f34f1b7cd783bec633a5df03bd11f4d5a99b21450b5bfbc13cd29339fc1bbe81a4008182582064ed7c9676c3b2954c188b60def3950bd750f8af90a233b077b6ff434539530803018182581d604da890918cb29f4446d07602a7c3877887326eb9e98394b8d6ce57b31b006983fdc40520ce021a0004b3890682a3581cc1ad22cabb342cbb83ce3859708232f4945ccb669e9b5f932cffc0eda7021a00016000081901f40cd81e8200010e82070012a2009f1a0003236119032c01011903e819023b00011903e8195e7104011903e818201a0001ca761928eb041959d818641959d818641959d818641959d818641959d818641959d81864186418641959d81864194c5118201a0002acfa182019b551041a000363151901ff00011a00015c3518201a000797751936f404021a0002ff941a0006ea7818dc0001011903e8196ff604021a0003bd081a00034ec5183e011a00102e0f19312a011a00032e801901a5011a0002da781903e819cf06011a00013a34182019a8f118201903e818201a00013aac0119e143041903e80a1a00030219189c011a00030219189c011a0003207c1901d9011a000330001901ff0119ccf3182019fd40182019ffd5182019581e18201940b318201a00012adf18201a0002ff941a0006ea7818dc0001011a00010f92192da7000119eabb18201a0002ff941a0006ea7818dc0001011a0002ff941a0006ea7818dc0001011a000c504e197712041a001d6af61a0001425b041a00040c660004001a00014fab18201a0003236119032c010119a0de18201a00033d7618201979f41820197fb8182019a95d1820197df718201995aa18201a0374f693194a1f0aff019f1a0003236119032c01011903e819023b00011903e8195e7104011903e818201a0001ca761928eb041959d818641959d818641959d818641959d818641959d818641959d81864186418641959d81864194c5118201a0002acfa182019b551041a000363151901ff00011a00015c3518201a000797751936f404021a0002ff941a0006ea7818dc0001011903e8196ff604021a0003bd081a00034ec5183e011a00102e0f19312a011a00032e801901a5011a0002da781903e819cf06011a00013a34182019a8f118201903e818201a00013aac0119e143041903e80a1a00030219189c011a00030219189c011a0003207c1901d9011a000330001901ff0119ccf3182019fd40182019ffd5182019581e18201940b318201a00012adf18201a0002ff941a0006ea7818dc0001011a00010f92192da7000119eabb18201a0002ff941a0006ea7818dc0001011a0002ff941a0006ea7818dc0001011a0011b22c1a0005fdde00021a000c504e197712041a001d6af61a0001425b041a00040c660004001a00014fab18201a0003236119032c010119a0de18201a00033d7618201979f41820197fb8182019a95d1820197df718201995aa18201a0223accc0a1a0374f693194a1f0a1a02515e841980b30aff14821a00d59f801b00000002540be40015821a03b20b801b00000004a817c800581cc264bca994a3a5deee5a1d9b92a3d7e9d6cbdb81f2f6989bb7f7b437a7021a00016000081901f40cd81e8200010e82070012a2009f1a0003236119032c01011903e819023b00011903e8195e7104011903e818201a0001ca761928eb041959d818641959d818641959d818641959d818641959d818641959d81864186418641959d81864194c5118201a0002acfa182019b551041a000363151901ff00011a00015c3518201a000797751936f404021a0002ff941a0006ea7818dc0001011903e8196ff604021a0003bd081a00034ec5183e011a00102e0f19312a011a00032e801901a5011a0002da781903e819cf06011a00013a34182019a8f118201903e818201a00013aac0119e143041903e80a1a00030219189c011a00030219189c011a0003207c1901d9011a000330001901ff0119ccf3182019fd40182019ffd5182019581e18201940b318201a00012adf18201a0002ff941a0006ea7818dc0001011a00010f92192da7000119eabb18201a0002ff941a0006ea7818dc0001011a0002ff941a0006ea7818dc0001011a000c504e197712041a001d6af61a0001425b041a00040c660004001a00014fab18201a0003236119032c010119a0de18201a00033d7618201979f41820197fb8182019a95d1820197df718201995aa18201a0374f693194a1f0aff019f1a0003236119032c01011903e819023b00011903e8195e7104011903e818201a0001ca761928eb041959d818641959d818641959d818641959d818641959d818641959d81864186418641959d81864194c5118201a0002acfa182019b551041a000363151901ff00011a00015c3518201a000797751936f404021a0002ff941a0006ea7818dc0001011903e8196ff604021a0003bd081a00034ec5183e011a00102e0f19312a011a00032e801901a5011a0002da781903e819cf06011a00013a34182019a8f118201903e818201a00013aac0119e143041903e80a1a00030219189c011a00030219189c011a0003207c1901d9011a000330001901ff0119ccf3182019fd40182019ffd5182019581e18201940b318201a00012adf18201a0002ff941a0006ea7818dc0001011a00010f92192da7000119eabb18201a0002ff941a0006ea7818dc0001011a0002ff941a0006ea7818dc0001011a0011b22c1a0005fdde00021a000c504e197712041a001d6af61a0001425b041a00040c660004001a00014fab18201a0003236119032c010119a0de18201a00033d7618201979f41820197fb8182019a95d1820197df718201995aa18201a0223accc0a1a0374f693194a1f0a1a02515e841980b30aff14821a00d59f801b00000002540be40015821a03b20b801b00000004a817c800581cd4bf7eb45b72dffa5ac33d5c902fe409e4e611f2e9a52fb0d09784c3a7021a00016000081901f40cd81e8200010e82070012a2009f1a0003236119032c01011903e819023b00011903e8195e7104011903e818201a0001ca761928eb041959d818641959d818641959d818641959d818641959d818641959d81864186418641959d81864194c5118201a0002acfa182019b551041a000363151901ff00011a00015c3518201a000797751936f404021a0002ff941a0006ea7818dc0001011903e8196ff604021a0003bd081a00034ec5183e011a00102e0f19312a011a00032e801901a5011a0002da781903e819cf06011a00013a34182019a8f118201903e818201a00013aac0119e143041903e80a1a00030219189c011a00030219189c011a0003207c1901d9011a000330001901ff0119ccf3182019fd40182019ffd5182019581e18201940b318201a00012adf18201a0002ff941a0006ea7818dc0001011a00010f92192da7000119eabb18201a0002ff941a0006ea7818dc0001011a0002ff941a0006ea7818dc0001011a000c504e197712041a001d6af61a0001425b041a00040c660004001a00014fab18201a0003236119032c010119a0de18201a00033d7618201979f41820197fb8182019a95d1820197df718201995aa18201a0374f693194a1f0aff019f1a0003236119032c01011903e819023b00011903e8195e7104011903e818201a0001ca761928eb041959d818641959d818641959d818641959d818641959d818641959d81864186418641959d81864194c5118201a0002acfa182019b551041a000363151901ff00011a00015c3518201a000797751936f404021a0002ff941a0006ea7818dc0001011903e8196ff604021a0003bd081a00034ec5183e011a00102e0f19312a011a00032e801901a5011a0002da781903e819cf06011a00013a34182019a8f118201903e818201a00013aac0119e143041903e80a1a00030219189c011a00030219189c011a0003207c1901d9011a000330001901ff0119ccf3182019fd40182019ffd5182019581e18201940b318201a00012adf18201a0002ff941a0006ea7818dc0001011a00010f92192da7000119eabb18201a0002ff941a0006ea7818dc0001011a0002ff941a0006ea7818dc0001011a0011b22c1a0005fdde00021a000c504e197712041a001d6af61a0001425b041a00040c660004001a00014fab18201a0003236119032c010119a0de18201a00033d7618201979f41820197fb8182019a95d1820197df718201995aa18201a0223accc0a1a0374f693194a1f0a1a02515e841980b30aff14821a00d59f801b00000002540be40015821a03b20b801b00000004a817c8000181a100848258201aba5fea24ea77d696737416bdfb6136fda5f0fa745f6b4aa682283e7d3e0f2d5840545819f889b346c17b95aafb6a6f81cf63d1149e8571727c3b682e21ce0308562176a2f2d628a6b4680c6d4da80a91631f5028533cf2fb2519bb739ee04e3f0b825820b33fb2984d0a104491afeecdcc327dc1fb0d72fd8bcb2a819999a3a6ec1f38395840e531d044d7c36361ded8e8fb90ce0cb590d2fbe67eb8a581d26e8064a87fde7e720329594f541df0734ca85f18ec3b44bf1731dad406fa7cc73171940215dc09825820d7361dba245e89a9f1e87247810aad8ab6bb5d8e8cfce978b458ff3bb4f34cb75840b0ad3e36ae6d04218cdb44f0849210fb9b52ca4208dd1fa97f5a7970021179faf86959661390afc2c2d70ee7517d6fa22ab0e21c00f2752ab252ea9e883dde0e825820f442e8c87378195b57ac5cc02590290158eb8707eda84155a035e5f5e6ed4f0a58408b8a4fffc01fa067f0d3c9afc2223499528e0f1e6e6f50ff28ba731c3923be96f7c8e4a15419b5abec02a45c96c195d8077d7340a2c1d59e5cf05c22219da508a080").unwrap();
        let _block = MultiEraBlock::from_explicit_network_cbor_bytes(&bytes).unwrap();
    }

    #[test]
    fn mary_block_nonce() {
        let bytes = hex::decode("84828f1a0054dca41a01908b2f582008ecdb54a80c81073ddee790f3cf4f8d4ff4422eb369b41cd5e0e18c13de6bbe5820d498647f2b7a481e6207ee3dfe02f6292ae85224621c5aabf9c0bf84d097390f58203492992128b6388ed9b56ab0329e42c57df5d1ead48436ea26908195f33daf90825840a38e20ecfa89e9804837f88fe8f8cd13b6a9b4eafca581fdceebf545eea0ea7e4caf36af4d7549a36d15598f67dfbed9840b02b4469d37552a9212669a79096458509370fce20873730ca9fdce508a3a9b0d3350a153c0942f6c8bf5dccd4c699aa80a635ceccb965c6327d6ca25ca32727de977c81f9143438e963df7db2d0165423809360fc744a34a099f1f59f01603008258400005b23bdc51c75f8eeb21f83eacac3d0605891801471c6d61515b27810a1bb8094f4c0ae560ba418ef35bba28e09ed64ca671bdea2139087822455f83ac451258504d90ab3807ad8c84b48460ae154e460826c6af5f3f18d05abdb05ea2c478248428c7d4cb9e9a6d82d94df170c1cabead6fa6b7774e0017dbc5ede4d1cd7cbe34cb08a7af450ff449ff9f68b1cf14e8091924c858203d4f6c99d53160a58c879c9ca04a2bde019377456384aa90b92cc00bcdb3671958206c1a25de95c89fe2df4af81e8440ac621b723fcc06c0ddfc0f2c7dd900067c1f0418ad5840d21ef35d27287c1172ae7d7f593f9f84ea925260e8ca9dd913746085cbd2843d10926b87044bedbdaed4f923e543898e3f7f97f6c780ff01270fdf1e604c4a0a04005901c01bb9f5e7ee8f37a4b499ed7abe8114e76c3d6d453c228004103840798ec2c0b85fbc8de65cf68bf154834679cd9bdd90a849afaf9c950c43a30cac0b4eb504077d34b1c4bbdff197ba64266affc385d9a2bcedc6f08559fe93f2cb61d6c70af41f7582c74cbb7866a6c881b139832e250325bbe15674165a2b06e158ba91341427033f9ce3b813a2bff5451bdc3cc9b3ab7952920383d8f2aba9772251db7d6e8135c58df613036c908e405c9277193a36b1d9a4d40bb263696a3de374ab695ce2fd4401bf30e707ee43de4c05d7839f2bb1e8bed4d4e6b0d9cf0906807b4a6b0520b992825d0643add63168848f3fea1fe2ae147a174d5e7939161ba46a293838ab8cda64dc5a6989aea414b4e502d1edb8dd0c5d1421ab7fa92d4581d0b669d1114a5f340568d67c68ed1b07bb3beea0acedbd7ddb9c79478f6adcda27ff31d704c5a998839750d5ac084ce81e05766c4ae18f4f783063b6f3b76be050be55896ed7202695427b3e9c333319150e0dd993f3ba314ef89f53a8689939425c648060a58a5310efb311c6b3f7835e0f1a662f9f2e867a389e3394c7a1cc2f74de2c0330ebf40aab2cf4efbacaac3fed373b4d64d9c56d3cfcec7f28c9bf18926894a500818258209ea2ac72bc4e082b1a47c6914d9d953d73aff9792174cc1f08cf7215544dd1b901018182583901d42900731c039a877000445f88be659e574632707b056fd2960994c5b652ad81db60f5d8605dcec6e73d1ed932009651d9f3e8580ee07d221a1e1b9461021a0002b06d031a0190a6cb0758209a19f90d9ef5545280368b5503eeaa52c1da650f68fb2209255313a34c19a1d6a400818258200682aeb19e9a31ea5f7f5d4078c91ec94831b1498fcebcd30bc37b6800c17f7001018282582b82d818582183581ce03866945a566d9e8e32f27273877d026385a8e7d54d9494906ed530a0001ab897305f1a1fb8358082581d61d91ef01b73f3010bb173945cf5417257c00c002715a13052015ab54f1a26908966021a00029cd5031a01908ee6a4008182582048926e334cd6b9469a3540cfca974230f9fa25d0e90fa043566abef18d2049aa01018282584c82d818584283581c76ddc0051984f84c6be230066710c1aead629f632e718bb3f9c2c145a101581e581c73a2075d4789f0d61873b4c2d309310949ca41e434d97c21388727c8001a37285aec1a08b11eea82581d6179e67550b2ff311da1883ad0ccc6fb2bb7c75e5489acff735fcc68781ad17d4c43021a00029cd5031a01908ee6a40082825820df07e64b7151d6c6c4c15ae3b2b1944d501df2cf54ee5ae98feaf97e49e0067e00825820df07e64b7151d6c6c4c15ae3b2b1944d501df2cf54ee5ae98feaf97e49e0067e01018182581d614926b3e631270b5d5310b5481e0fc82b1a1c706ad278c4bc091e29211a2aab95ba021a0002d644031a05f5e100a40081825820c0cf68370b852bfed85b0a39ce4644c8195511149ab70620a9158206d08e05ce01018282581d61483f3120cfe28f4220f4b96228f8d110e5f2100285b914218e927c251b000000012a70c1c08258390111e921395d6c4ad0c7004ef928d9d0dcf5b33983749a14b59a0e6ee22aeb420cce4b576d6833d513fbb1aaf4b2396a427c9d96aee6d608df1b00000001b8a9ec0b021a00028de1031a0190a71ca400818258200b5dd952875af5c7ff5a251febd31e607ee1ff3be911e93d27a35f94c9e96cbd01018282584c82d818584283581c2be129a2509494c81b8fced13d9eb21529b6c1ef88f5ea3b578c27c7a101581e581c633c7e9a7d0419aaf93a78e0cf2fc2bcbc54a2a12371fd9d00645c8d001a6429c6be1a195edd7082584c82d818584283581c7cd9baffad4c847dce58c53bfaa0ae4d6f19ed9c0182585d3ca54bb9a101581e581cd4688480c96f8332739757b76b843a8c681328ec34c9b5b38d6898fb001a6ebfa4dc1b0000005286323835021a0002a8b1031a0190a72ba50081825820b2bdbe07d5cd4da8b8dbef9eee67ff6fb1877055d0b8309fbd4718591389eac5000181825839016445bb08465c26cf435a4274b8dadff753b3b9a87f4aa8f88479431d4eb646b86f18f6f4266684f37fbcf4027c650747dc45278ab0bf8c8e1a0048f96f021a000351d1031a01909ccd0682a7581c162f94554ac8c225383a2248c245659eda870eaa82d0ef25fc7dcd82a10d82015820d982e06fd33e7440b43cefad529b7ecafbaa255e38178ad4189a37e4ce9bf1fa581c2075a095b3c844a29c24317a94a643ab8e22d54a3a3a72a420260af6a10d82015820d982e06fd33e7440b43cefad529b7ecafbaa255e38178ad4189a37e4ce9bf1fa581c268cfc0b89e910ead22e0ade91493d8212f53f3e2164b2e4bef0819ba10d82015820d982e06fd33e7440b43cefad529b7ecafbaa255e38178ad4189a37e4ce9bf1fa581c60baee25cbc90047e83fd01e1e57dc0b06d3d0cb150d0ab40bbfead1a10d82015820d982e06fd33e7440b43cefad529b7ecafbaa255e38178ad4189a37e4ce9bf1fa581cad5463153dc3d24b9ff133e46136028bdc1edbb897f5a7cf1b37950ca10d82015820d982e06fd33e7440b43cefad529b7ecafbaa255e38178ad4189a37e4ce9bf1fa581cb9547b8a57656539a8d9bc42c008e38d9c8bd9c8adbb1e73ad529497a10d82015820d982e06fd33e7440b43cefad529b7ecafbaa255e38178ad4189a37e4ce9bf1fa581cf7b341c14cd58fca4195a9b278cce1ef402dc0e06deb77e543cd1757a10d82015820d982e06fd33e7440b43cefad529b7ecafbaa255e38178ad4189a37e4ce9bf1fa190102a40081825820a3d07b77d09868b01d07b4c53e40b71200639266b1776512e0981cd66f48eb5d0101828258390134d388e0c5251d087232b3dcc67caa6cce2937c97a270fb4f4a2499999b8e6a0a8c8e173ddd63478e982846ffbbcd165028a1cef78ae9ee71a078ca24982581d616904e8b2c26f3dda6c4a5db4b3ec9e31d581c9960977cfe9c6917a431a2d0faa34021a00029cd5031a01908ee6a40081825820ac9ceff2de86d2387950ed7d1197a517e772c5ddfecb5b94767fae888820b3d801018282584c82d818584283581c15c97f0866ba4f390e283a268b53e180cfdb5e34bcc464a357e36710a101581e581c83d3e2df30edf951c5ba0b8582a1a303432081a4295f18ba44e9a548001a6b9405c71a1392fc8082584c82d818584283581cde349abef27bb239d4ae5e7629c1400d32165097f684148808da3c97a101581e581c2b0b011ba3683d776e9d872a85987bd6e6942a6b8c30d6d7dc3dfb14001a18ff53f01b0000000107647432021a0002a8b1031a0190a729a40082825820bb6baeb11c7472779c3b3acc35db7897f2d36d11846909ba6dd5979239fd067000825820bb6baeb11c7472779c3b3acc35db7897f2d36d11846909ba6dd5979239fd067001018182584c82d818584283581c9c9490179ae2288142fcc6fc0f6aa8ff8c49531ed384740d5c73622ea101581e581c5981b261ab5ccd7bf7f2c1225ad7eefe385edcc839ba55113b3e95db001a0122212c1a0aba8850021a0002d644031a05f5e100a40082825820d4b800d689d733f523fa78361ffb5fade29d9bb13fcfbc344aa458bd0b0b976b018258203b7bfa5f182b349e13c0e1c56c6c81ca8c5cf40d7b72b416a3563783fc0ff134010182825839012d8b354447efe987387f329b63a3c86b6007371699ed5db4ac5cd88e2d8b354447efe987387f329b63a3c86b6007371699ed5db4ac5cd88e1a584abd6e82581d610237be10f5ec0ccb6cbd226b112f0940fed44ae0466d9b53962ba8b11a184eefca021a0002bf35031a01908ee6a400818258201cc75901ee9df32dbc50ce820052038faba13b465b25fe64f887d940c9f9458e0101828258390117c7c19c81dbf1187a5bdf387a320f46bf80246e30d5b97efd59ab9fdd7986e3b08ab70333fa1e4183b8735cf3474a31108fa0943de6b8cb1a034f468282584c82d818584283581c776eb96b96e2cbce528a47278a736ab725dd13ce98c027ccb548ed70a101581e581c1afbc57540db1561adeaccd7f4894065965cd230f6bdc70722aecd8b001a6a14bf9a1b0000000f31da326b021a0004623d031a0190a71ea400818258202c6cca35e5e4458cc35ebd173a6471b3ae29668fd06bd4ba5114955339f219e501018282584c82d818584283581cad8f02b7134781a28997cb23f2a78215322f6c1d8a58e9e47c9f445ea101581e581cc9a4c68c63646852d93de951824def94b774fd58431f7a5618115a19001a389631ac1a060871738258390181bfa0e759547ebf226bbd51a844095d56890ba05aea3a73665cbe881cdeaf2c1028e67b5b360171975136b41c9a58d9220bdd65fd6ac0791b00000003218293c1021a0005f9b0031a0190a71ea40081825820c306f318e9721757d3252cdc5dd31eac6bd6aee236beeb9601a772fa681e554301018282583901a6c024231f3bf4de746511675994db25b664aecc25789de65462eff4739f0761691131d31bd714501d8a6f9f44ac9b5b3a74932753c29efc821a00160a5ba1581c59960c8ca8871af59f70474cbb2c11e3d782614a4c4be7f25e6c783ea148616c666f6e7a6f731913888258390170a84cf34da92bfe2c4da8689f358caeb4d1715c73710aed235be3d71537f3dbdcfedc8079eb6e8c1af93563a6cc2a7ecfbe5b4ba92750ab821a9c11edf0a2581c36e8f59542d444e3afca00cecca881cbdf8257c1b07a2fd84ed25730a1574570737465696e4469646e744b696c6c48696d73656c661a000f4240581c59960c8ca8871af59f70474cbb2c11e3d782614a4c4be7f25e6c783ea148616c666f6e7a6f731984d0021a0002b041031a0190a73ba40081825820e71f446e150a646a1982fce26c0239395dfc52bf029d3b653e06232925148aa10101828258390126141bac8f8fde24eec91011bcffd28b7f221b52aaaddcb24b3b421f265b2a3a895d6d9ebf67b1d1f1b06927caf752d13e40bf4ea11c622a1a004c4b4082584c82d818584283581cfafa1e6485be8a43275f37446d9ec9268682dff27678956c1a01dffca101581e581cd8d97175289f3a7dd17a4b6107633ed785ce2d33a1ee00f20ef4d7fa001a9e288d2c1a3eff155d021a0002a56d031a0190a73ca4008182582020a32c2967acf50e42efa21bc6f2582f26def86152f15b66a0fffc61b1397d0100018282583901a0c0fed7b1d4288835dc2334aba0240d27e10dedb73badb27457ba91ec17628c4600f1d941b206a557581d904584da48ba972d554d3aa4891a03d8312082584c82d818584283581c505f6f99e8b2a44bc02753d04e1745910a28205187e4b743b6be8904a101581e581cd8d97175289f3a326c6a29619d56eb6e22830bd40ee474aea3ea120d001aa030fe341a02048ce2021a0002a56d031a0190a73da40083825820150ae07532b9e62de190ac6955cde4c33ec42ee86c7d8415c7f820d6561f280d08825820293f1fb4adaeded6aa8f3230d6075df6e8ea657fe3f36e3c78bfb71524d3a712098258204dbaa5d17d37aadddce5409a0aa4c299c9468f0cda7f672b3d9553713216e8290b018c82584c82d818584283581c0fea43d9bfa899adfbbf88e57e6798e53466219143130576b4122a44a101581e581c9b55b8bf594b59974d9edc197b6a68a1d5d10327348c8f5c20278339001a2c181f7a1a3b9aca0082584c82d818584283581c7d1d116c1bd0b92d8f307dcfdde3282316105c6249c995c5b4c96f48a101581e581c8f802fa39db6a972b8e0b1c4f7f9649c381f6af5dd53cc3658668cf1001aca1390591a01c52fa082584c82d818584283581c0fea43d9bfa899adfbbf88e57e6798e53466219143130576b4122a44a101581e581c9b55b8bf594b59974d9edc197b6a68a1d5d10327348c8f5c20278339001a2c181f7a1a01c9c38082584c82d818584283581cc5f21c78dba1146cb869e7c243165ea5841970045d4dd3cbec118f01a101581e581c4c119dd2b2270328df974203bca1f8e6d3bb601baebda45374871f21001ae0b305381b000000022ee0828082584c82d818584283581c90438a11280f3692dc611cb03d8d7cb1c3688ad8519b2c52b4acd791a101581e581c4c119dd2b2270357d09868034a444ba63688d21110f3a142d428c203001ae70673711a96d76ae082584c82d818584283581c23cf87cf3f4008fc1f26fd7667e3340325df267dfdb86b22234a11dfa101581e581c633c7e9a7d04199e20a1cae0b6a1c282c0310dba20b93265e8d2dbb9001a00cc23491a6e5404c082584c82d818584283581c7899f9c5244e1b64109325ae19e7f7920eb19f118fcc4cb525400b82a101581e581c84b2ff88d08bc4143a01f5c9795a72f04f627169d47c46873e86096b001a7919a0ea1a1a848ac882584c82d818584283581c700af7d911f9383b9a3011b68fb725dc43ff6b1884627c3e1007a20ba101581e581c84b2ff88d08bc4254f49a9c959dde14cccd5a314ef5d04631b2fc5e6001a1bfa4f131a00d86ce182584c82d818584283581cbe60727988fa9476b309f78e41f74035cfeb53c2c1ae16b3f5562b01a101581e581c84b2ff88d08bc477eb6000c9b770b219910c269ff575e1d78b74a975001aef4865821a00da751182584c82d818584283581cfc8a85f9862b26e153aae414ced247a0767a7e6003cec68d257833f8a101581e581c84b2ff88d08bc45186803ac9607c3c0743e23a1c18866c40919dd748001a58de0f8b1af82445c682584c82d818584283581c8449e1e8e805543715cc807d922f5fedf6c0437e5f410e7eb8e28055a101581e581c84b2ff88d08bc479e9e19dc90ee5e975b064ddf7bfacc41424d8ad82001ae306c9341a4304682b82584c82d818584283581c77970d82501e8dbbe906fdf9f208fbcce238fa93ee6331328d64bd8ca101581e581c84b2ff88d08bc4359d75e8c9aea1d751cd013be81d1b54cc374ed3c5001a4c8fcc631a31089b31021a0003908d031a0190a73fa400818258207c072edb52db7aefc6881f6ced60b7aad9b8fabf590dae34db09b5a4db81702105018282583901ad915af56530a5d452bcd4a88b87b0e1ca375feaab4fe399a1105312ad915af56530a5d452bcd4a88b87b0e1ca375feaab4fe399a11053121a02ec88cf82582b82d818582183581cbc14e6b9754d7eef691230ae09e4c52b03d1d824c80e22b65432d7a8a0001afaf343d61b0000000185309742021a0003656f031a3b9aca00a4008282582069572bee1b68d01c50b619de76461edbb90dd25c66e862c79fa9fa124efa92300082582069572bee1b68d01c50b619de76461edbb90dd25c66e862c79fa9fa124efa923001018282584c82d818584283581cfbfbb9e7f7844a40b2a6465af5ec17b8da45974c738d60d1cb21ba8fa101581e581ce378ee30d568143c4b061b6dfd1875fe5b27cd41917b30ce0107790b001a31db91811a832156008258390114ef538b66f44052ae6dc8c545250eebedf65c491327a9b8c7102e0c14ef538b66f44052ae6dc8c545250eebedf65c491327a9b8c7102e0c1a036cd490021a0002e4db031a05f5e100a4008182582089be624b89aa349ccd11c415695e0421d00fd3106b6334e877961437e51191c00101818258390119690bdc2289666c814de06b1095ca10ecf2092ed237669df9e3ee0619690bdc2289666c814de06b1095ca10ecf2092ed237669df9e3ee061a110c0380021a000add40031a0190a74d94a100818258202669028f7c59934d13efa39435215fc2486e389e35ba6c6b05dda7c65213257e5840b3cd2e84cae259105b7330bf4c871ff8796d42935ef822e3028f4e3128d660d04619443006f87971c3ec2fdd3f0ad9a63070e2d8037b88682c7c91b327c55b0ba100818258200d197b7eeef1b961962a2911d4b553f0d9c3114b8d3e61de3beb4f39dc348eb058407d6f5b2c1cce03a45c5f6d754da293936d7d3a013ef5822e9a36954b635fc0ed148da41a7ad85438d6fe7e32ff6e3a7c2e611a96ea891d08e3d570b14386a906a100818258206687a36f0a4b8abf2671f8affb7febeb8940e9ee83548d2a2924dbc74679c4665840289d23202301208dce8e594b84de249e4e6a77153ef66c7171431df21645f336ed1b37e5786c8e2e8c41b790b1deffa093ddd0cd6937121809abc327b45cc904a10082825820276601a019b13a328b7306c0faac593c064476af30beed0380aa035d4bc1fe315840331b8f8dd2c470f1318a87fb51fc1ab030448166aa61bcfea8052420658836e59036c52befdabb2cf4acb8637bc63f0bd822f21646b35ce1e167269c233bf800825820276601a019b13a328b7306c0faac593c064476af30beed0380aa035d4bc1fe315840331b8f8dd2c470f1318a87fb51fc1ab030448166aa61bcfea8052420658836e59036c52befdabb2cf4acb8637bc63f0bd822f21646b35ce1e167269c233bf800a10081825820783b5406ddd311ad3640afd90eb5bc45f85ff0b57fc5937ee365c8313636239f584070f587d9c080c8a76c5b26059bf3ec4b8dec9a98c955710303038636187aa76c01b386f87192cb2b2e4bb768b6c4a10cef33a631c9236799f034849c1508590aa1028184582063e3a4107a043aa1c7253612aa6e12e3c6231fa11f454aa06046f184cbf49fd758405d946178128a00dd003121c481a5d8f729e9b96b7dbf0bddbe57c94adcf000a83b68d7fc9f64a88d96fa87e1fc6c9cc1af2e1b86719b0d45febb7518459c970a58205bd2b0767d1f7de98f74403718cab3262e11105be4883947ed06785a3b3626825822a101581e581cd4688480c96f83055933ffb71bd2a85ea5ec4f45c5ccc0c98e8ee2bba1008882582061261a95b7613ee6bf2067dad77b70349729b0c50d57bc1cf30de0db4a1e73a85840375bfc568d815bc5d6778d3e2ec6287529a88602dee06e5c694ff8d727d8dc8d325e9c8967ebf992d8df9fd862f7b03a57c9c721a563c783a9847296484135028258209180d818e69cd997e34663c418a648c076f2e19cd4194e486e159d8580bc6cda5840c9664b3f484faa30ebed8308bc009ba2261b4826a6306b1a943cced6f0e00aa2fcbc8da3156db5d7d644196c74d44f81dfe477658ca20733272e4b1a9490fd0f82582089c29f8c4af27b7accbe589747820134ebbaa1caf3ce949270a3d0c7dcfd541b5840e46527dea5301bcb69d943dc3f0d7e97798b4b93addd28907cb08b0fd9741b285069476500407020249b28db467b16a782510e4bac318c1780f562a5a95ccd0e825820f14f712dc600d793052d4842d50cefa4e65884ea6cf83707079eb8ce302efc8558406bd6d8e0538963107baac0977e2b3ec8e3baab8dacf77851d8a7433412d0f3ac2a631cb4b1819e4d735e04ed4fc2ff89843d05de2847fc1a7a6756e19940910c8258208b53207629f9a30e4b2015044f337c01735abe67243c19470c9dae8c7b732798584059854e89e94fb2d6a363884aaf22d117e5fa22f77948d5a15f0aa551c4f97189d3a039467a046c8ead26612619c4ecc48ec88144ff077910891b1d95dd89a20c8258205fddeedade2714d6db2f9e1104743d2d8d818ecddc306e176108db14caadd4415840620c64cdf8d662028fb1ae1da2e9ccaaa81b79f40fae2ac8e425d174f8b6dc72383c61eb4a9ec1c5d10ecc1d643968e2fc00ad9e3d8e317faf3f3c261033f60a825820cbc6b506e94fbefe442eecee376f3b3ebaf89415ef5cd2efb666e06ddae483935840b74f2b37390269a78d45685d3197b4640c5cc9cf82b1c1c53d3f88e2117ce33bc7c6da056a9701818a37b8d657f4c4f25d39573a33fd8f57ceb6e228dd3e0f07825820e8c03a03c0b2ddbea4195caf39f41e669f7d251ecf221fbb2f275c0a5d7e05d158402d8fefb1305c16245613adeee72004c31840267dd01dbfaaa177dbe0da5f542ce52252b3d3da24adc17587443be4d8715d772ca0d9b4af8c97d38678b0eb440da10081825820393b3d3cab6be0897cbc87143cc3a24825dc1d8249df4d37969502931cf856ff584072e1e68e093e0511cd74666cbd04dc085f51824ecd699ccec34b65f67cd455172320e5dff108b5698a90599e95ac97ce29223e8eeb0341c93fc73cb01757ac06a10281845820d1f77134edfa56f240847ead978ca98e8bb5f43f476b6b71ef3e50ceb852b2135840df7a4a2e50bac5285264b9174f51d3f93ad0c309141737d3eef6a884db658ae63ad3e770cab31c93c11f873071fcfbdf0989081cc5a878fa11516e9325f271015820abbd19a5be197b7f66196c02a7a5fd714c3f95e2b280fd85dad81683b995b6cb5822a101581e581c2b0b011ba3683d58dd9d722a2f2cb9679b356ada972029c6b5f2ae61a10082825820ea5dd58ee48288c53c2ab95776da5e35357db5833f8ab3c420baa88dd920e11d58405bd3d606ca8810f5e50aabc09bbb69bd460bd90e6b68e016dd553b32da20ee1ec20ea79826ead8b52b7ca73b673ae5a59b132bb64fe565fcc462187b53f0a807825820ea5dd58ee48288c53c2ab95776da5e35357db5833f8ab3c420baa88dd920e11d58405bd3d606ca8810f5e50aabc09bbb69bd460bd90e6b68e016dd553b32da20ee1ec20ea79826ead8b52b7ca73b673ae5a59b132bb64fe565fcc462187b53f0a807a100818258206c0180db688a9e477b10a7792e3c4efd749cddbe0e05068b6cb586dee54056975840e4f95636842e596b03aaf3379907a96b48e4545ad591246df8ccb7c6c254c30e514f9135e7178d3a7badb521cc583ae458e137195c41ce4f0b2fe19920955000a102818458208b66d7f3bf0629a3cf791984bf0fa4568b83106c9257818e63edf63142e9bf2a58400a13b559254d21194e8fcf77662d0b73b778dc91f0f12aeb639becbcbcc2322df45bd9b2cdc85a6d3b842056191c2903dd9afcdfb234a62a9e0a8eff2f3fd10d582022cc1a6495cb1639e562a07c9aa86a4cf54e3dbb6839aa4ee6f0b6a20dc349ad5822a101581e581c1afbc57540db1561adeaccd7f4894065965cd230f6bdc70722aecd8ba100828258204db5d63d4561edc2b565f50c9be435ffb0bd606030aefba4fe0cd9ff86cbf473584082a9975d71dfd55cf3be8b1fa9da9b07737143e0a2f36aa8e43f793844c4448ed9a07a500e914a0e629dcaede71df676218143a324186c2a1b5e2224e114620f825820e68a43be6ae384bc3d33b2921c89914f5dd99bf088a45c5a31d17734e408d78e584076e9eb5c4a18bc8a5aad2ef4e4db2810fda3b76a6b8639cf82b2ad0141b80a97c0f568f3292fdc67bb378c91301fb06cd8ee4c543509cdb4dda4d1aef9d31a02a10081825820c9ed44b3f86f0beaa05d2e8e611488f3f369ecafbcf2754cfea82c84e248528b5840f7b52620cff9f200274014fca350d74a84d17b819e9cc6d23eb54a51078b7a59750a6c01e3f2b5a05bebe8520d0a80f14f78a32854ce889f99e7eae9c886540ba10281845820d02732f50b7c1c1b7a4e946d8d280195c2c96577f1ba2b90474ad509e35812055840a856245cd143457faabef30d675d0c45d2b63f16cf97c62d50f53d03e221d8e20869caf8f61982d29af7b3082f2e62d47484598495e95ac70a8c1b442aed97015820c227781775746f69bf6644f59dcfa6ac64864e307193ba95c2ea4514babd32d05822a101581e581cd8d97175289f3a7a3431a461b982088fca713f245b33843eeba6ace1a10281845820a0e38b160bd1292273c027fc08fca940abf54ecf3331dca01a5f195dc6b0ea1e5840617a03af56821787ae7a5e9ef0f28455157ee825a7913e1c0f27f35261e05d9916573fdce741d4ff419326c34aed1c02f0c998fbe33c247e1e800c5288e6d7015820937737a634d6ea5a0a2ef0f7658bb5337a21467e5ec565216a56bad4bd602a415822a101581e581cd8d97175289f3a14d655b2617089ce27d2df668f2901bc633cf7fcf4a102838458204a8e7f58407fdd312c4b093fbd30a69e1f6977b741d232e83e0fb99aed0e4cbd5840707b40421b8aa92605e34084d54e30643c253171bca2f5c2d3c56816a0295c025c870d9d0e3fa8df5c21764e33fbbd29d52bcdd77d7e6464f2948baa4e550d0a58203f043217f97312094c938259e57e9fb4d62b5ea373d00db3964b34ff898ac38f5822a101581e581c84b2ff88d08bc4408857d0c933f73033a3c62ced32e46b133ccbabef845820f80b356bd8105ebf65a3ca11b8ec2f8a179ac969b3bd6e9dcabf0e98bcb95a405840ff6e845dedeb967a24b042d839b52fadd4ba9855f13a76540b4de6818b016427b6e8a5512242142d732a5bc3d9df60d6e892293fdf5d33adde9bd22f69d1f6025820e7f9e3a251b0465dd169e2af1eac21512411370c42d5901670c9b6fb31658dc05822a101581e581c84b2ff88d08bc45e1dfb07c906b614497ca6cfe4ef27fdd314676c708458202ed3b6973c6f7fb567a9fefc01b351cbb06998c758c827fd34ff3d4512c55465584082351b0b888fd4854317304dc9e9865d75ab207a2c2bad439ca694a76e49a0224461f5291ee84db1a2377c2258ef8339ae9671e596218e1fd4383fe6ab984f0a5820dde71e8188d0e60442a9241ca91a5026c784677ed7b550d2ee49f12478076a365822a101581e581c84b2ff88d08bc4638430e8c9c992b98178c9a3a86159f877ff48e1f2a1028184582087fe94fa4925fcaa0948ac6b3a4ea590d11710fc999bfb3939864be95a30da8f584022093273e4658d71cf5cb737d1869247e39782e985e236c5b732996c1c6660528df5507fd272f5f15bdbf3578782d2dab8ab74d5b3b0fd48b0d8b8a26589bf005820250cd474ce3d2459197ee7ce25265f41eefc7b0fc37da633d10e4b02e87ad4ad41a0a1008282582086e3337f52290bc4f455da7e83a055c93e2a96b63fb3b7233f20f1a9c46e80a4584039db8d78698986021fc4cbfbe84665e1056dfaf91563a42841520ef5f03f573731bf6afb2ece1c85d8e2808cccc6d2b3a5587bfc72c9d88033d325bd7b6fb90282582086e3337f52290bc4f455da7e83a055c93e2a96b63fb3b7233f20f1a9c46e80a4584039db8d78698986021fc4cbfbe84665e1056dfaf91563a42841520ef5f03f573731bf6afb2ece1c85d8e2808cccc6d2b3a5587bfc72c9d88033d325bd7b6fb902a10281845820f0cd529bf77540b1211aa75a233601e0627951ccd4270d812c3add3c444bb0985840d9ed324cb3e74aec5baa40f3913856a015f78ab4b746b808268020bf2d293beb599eb390165c8299ce4e1a6ab0deece23c4401dc63fc6ad5a7f3c808a72f6f025820a7c5165bf99697dab666d48be507bbc475e282ae9743e72715f6e6b084234e835822a101581e581cba5f3d73c6d17e2e7d13dd396f0a74bf5cd02fd63ec68d0f09d9898fa100a219ef64a4015820ac05963babda59adb08a6ce3ab590199c0baaabd2763484f0bf5a7e72335ef25025820d1a756633fe50049d7cac880d793289e2b16ac4ef31abd411f730770d63c29e903583901d42900731c039a877000445f88be659e574632707b056fd2960994c5b652ad81db60f5d8605dcec6e73d1ed932009651d9f3e8580ee07d22041a01908adb19ef65a1015840787a36aa14eed279069272dd3ade19ea4e304a67da4524098619195e974b3be409fa6aee63a784c9fe9fac77b4718c84c3523c756b0701d10fa2171bec93db01").unwrap();
        let _block = MaryBlock::from_cbor_bytes(&bytes).unwrap();
    }
}
