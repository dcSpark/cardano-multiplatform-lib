use crate::allegra::{
    AllegraCertificate, MIRAction, MoveInstantaneousReward, MoveInstantaneousRewardsCert,
};
use crate::alonzo::{AlonzoCostmdls, AlonzoProtocolParamUpdate};
use crate::babbage::{BabbageCostModels, BabbageProtocolParamUpdate, BabbageTransactionOutput};
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
use cml_chain::plutus::cbor_encodings::CostModelsEncoding;
use cml_chain::plutus::{CostModels, ExUnitPrices, ExUnits};
use cml_chain::transaction::{
    AlonzoFormatTxOut, RequiredSigners, TransactionInput, TransactionOutput, TransactionWitnessSet,
};
use cml_chain::{
    Coin, DRepVotingThresholds, LenEncoding, NetworkId, OrderedHashMap, PoolVotingThresholds,
    ProtocolParamUpdate, Rational, UnitInterval, Value, Withdrawals,
};
use cml_core::error::{DeserializeError, DeserializeFailure};
use cml_core::serialization::*;
use cml_core::{Epoch, Int, TransactionIndex};
use cml_crypto::{
    blake2b256, AuxiliaryDataHash, BlockBodyHash, BlockHeaderHash, GenesisHash, RawBytesEncoding,
    ScriptDataHash, TransactionHash, VRFVkey,
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
            Self::Shelley(header) => Some(&header.body.v_r_f_vkey),
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

    pub fn mint(&self) -> Option<&Mint> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(tx) => tx.mint.as_ref(),
            Self::Alonzo(tx) => tx.mint.as_ref(),
            Self::Babbage(tx) => tx.mint.as_ref(),
            Self::Conway(tx) => tx.mint.as_ref(),
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

    pub fn collateral_inputs(&self) -> Option<&Vec<TransactionInput>> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(tx) => tx.collateral_inputs.as_ref(),
            Self::Babbage(tx) => tx.collateral_inputs.as_ref(),
            Self::Conway(tx) => tx.collateral_inputs.as_ref(),
        }
    }

    pub fn required_signers(&self) -> Option<&RequiredSigners> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(tx) => tx.required_signers.as_ref(),
            Self::Babbage(tx) => tx.required_signers.as_ref(),
            Self::Conway(tx) => tx.required_signers.as_ref(),
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

    pub fn reference_inputs(&self) -> Option<&Vec<TransactionInput>> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(_tx) => None,
            Self::Babbage(tx) => tx.reference_inputs.as_ref(),
            Self::Conway(tx) => tx.reference_inputs.as_ref(),
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

    pub fn proposal_procedures(&self) -> Option<&Vec<ProposalProcedure>> {
        match self {
            Self::Byron(_tx) => None,
            Self::Shelley(_tx) => None,
            Self::Allegra(_tx) => None,
            Self::Mary(_tx) => None,
            Self::Alonzo(_tx) => None,
            Self::Babbage(_tx) => None,
            Self::Conway(tx) => tx.proposal_procedures.as_ref(),
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
            ShelleyCertificate::PoolRegistration(cert) => Self::PoolRegistration(cert),
            ShelleyCertificate::PoolRetirement(cert) => Self::PoolRetirement(cert),
            ShelleyCertificate::GenesisKeyDelegation(cert) => Self::GenesisKeyDelegation(cert),
            ShelleyCertificate::ShelleyMoveInstantaneousRewardsCert {
                shelley_move_instantaneous_rewards_cert,
                ..
            } => Self::MoveInstantaneousRewardsCert(MoveInstantaneousRewardsCert::new(
                MoveInstantaneousReward::new(
                    shelley_move_instantaneous_rewards_cert
                        .shelley_move_instantaneous_reward
                        .pot,
                    MIRAction::new_to_stake_credentials(
                        shelley_move_instantaneous_rewards_cert
                            .shelley_move_instantaneous_reward
                            .to_stake_credentials
                            .iter()
                            .map(|(k, v)| (k.clone(), Int::from(*v)))
                            .collect(),
                    ),
                ),
            )),
        }
    }
}

impl From<AllegraCertificate> for MultiEraCertificate {
    fn from(cert: AllegraCertificate) -> Self {
        match cert {
            AllegraCertificate::StakeRegistration(cert) => Self::StakeRegistration(cert),
            AllegraCertificate::StakeDeregistration(cert) => Self::StakeDeregistration(cert),
            AllegraCertificate::StakeDelegation(cert) => Self::StakeDelegation(cert),
            AllegraCertificate::PoolRegistration(cert) => Self::PoolRegistration(cert),
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

impl From<AlonzoCostmdls> for CostModels {
    fn from(cost_models: AlonzoCostmdls) -> Self {
        Self {
            plutus_v1: Some(cost_models.plutus_v1),
            plutus_v2: None,
            plutus_v3: None,
            encodings: cost_models.encodings.map(|encs| CostModelsEncoding {
                len_encoding: encs.len_encoding,
                orig_deser_order: encs.orig_deser_order,
                plutus_v1_encoding: encs.plutus_v1_encoding,
                plutus_v1_key_encoding: encs.plutus_v1_key_encoding,
                plutus_v2_encoding: LenEncoding::default(),
                plutus_v2_key_encoding: None,
                plutus_v3_encoding: LenEncoding::default(),
                plutus_v3_key_encoding: None,
            }),
        }
    }
}

impl From<BabbageCostModels> for CostModels {
    fn from(cost_models: BabbageCostModels) -> Self {
        Self {
            plutus_v1: cost_models.plutus_v1,
            plutus_v2: cost_models.plutus_v2,
            plutus_v3: None,
            encodings: cost_models.encodings.map(|encs| CostModelsEncoding {
                len_encoding: encs.len_encoding,
                orig_deser_order: encs.orig_deser_order,
                plutus_v1_encoding: encs.plutus_v1_encoding,
                plutus_v1_key_encoding: encs.plutus_v1_key_encoding,
                plutus_v2_encoding: encs.plutus_v2_encoding,
                plutus_v2_key_encoding: encs.plutus_v2_key_encoding,
                plutus_v3_encoding: LenEncoding::default(),
                plutus_v3_key_encoding: None,
            }),
        }
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
}
