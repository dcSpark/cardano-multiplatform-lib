// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

// Collection-wrapper index for this crate: one `pub use` re-export per collection
// wrapper class defined here (list/map wrappers minted from `[* T]` / `{* K => V}`
// shapes, including their NonEmpty variants). Compiled as part of this crate, so a
// line naming a removed wrapper fails this crate's own build — the index cannot
// drift. Downstream crates point `--extern-wrapper-index <dep>=<this file>` here to
// avoid re-minting these wrappers (a wasm duplicate-symbol link error otherwise).
pub use crate::generated::AllegraCertificateList;
pub use crate::generated::AllegraTransactionBodyList;
pub use crate::generated::AllegraTransactionWitnessSetList;
pub use crate::generated::AlonzoFormatTxOutList;
pub use crate::generated::AlonzoRedeemerList;
pub use crate::generated::AlonzoTransactionBodyList;
pub use crate::generated::AlonzoTransactionWitnessSetList;
pub use crate::generated::BabbageTransactionBodyList;
pub use crate::generated::BabbageTransactionOutputList;
pub use crate::generated::BabbageTransactionWitnessSetList;
pub use crate::generated::BootstrapWitnessList;
pub use crate::generated::Ed25519KeyHashList;
pub use crate::generated::GenesisHashList;
pub use crate::generated::MapAssetNameToI64;
pub use crate::generated::MapStakeCredentialToCoin;
pub use crate::generated::MapStakeCredentialToDeltaCoin;
pub use crate::generated::MapTransactionIndexToAllegraAuxiliaryData;
pub use crate::generated::MapTransactionIndexToAlonzoAuxiliaryData;
pub use crate::generated::MapTransactionIndexToBabbageAuxiliaryData;
pub use crate::generated::MapTransactionIndexToMetadata;
pub use crate::generated::MaryTransactionBodyList;
pub use crate::generated::MaryTransactionOutputList;
pub use crate::generated::MultisigScriptList;
pub use crate::generated::ShelleyCertificateList;
pub use crate::generated::ShelleyRelayList;
pub use crate::generated::ShelleyTransactionBodyList;
pub use crate::generated::ShelleyTransactionOutputList;
pub use crate::generated::ShelleyTransactionWitnessSetList;
pub use crate::generated::StakeCredentialList;
pub use crate::generated::TransactionInputList;
pub use crate::generated::VkeywitnessList;
pub use crate::generated::alonzo::AlonzoProposedProtocolParameterUpdates;
pub use crate::generated::babbage::BabbageProposedProtocolParameterUpdates;
pub use crate::generated::shelley::ShelleyProposedProtocolParameterUpdates;
