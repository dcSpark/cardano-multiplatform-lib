#![allow(
    clippy::len_without_is_empty,
    clippy::too_many_arguments,
    clippy::new_without_default
)]

// Seeded by cddl-codegen on first export; never overwritten after that.
// All regenerated code lives in the `generated` module. Add your own
// modules/re-exports/attrs here freely (e.g. `pub mod utils;`).
mod generated;
pub use generated::*;

pub mod address;
pub mod builders;
pub mod byron;
pub mod deposit;
pub mod fees;
pub mod genesis;
pub mod json;
pub mod min_ada;
pub mod utils;

// Facade modules (see cddl-codegen docs "Per-scope hand modules: the facade pattern"):
// an explicit `pub mod <scope>` here shadows the glob-imported generated scope module,
// merging the machine-generated items with the hand files at src/<scope>/*.rs under the
// scope's public path. Scopes without hand files keep flowing through `pub use generated::*;`.
pub mod assets {
    pub use crate::generated::assets::*;
    pub mod utils;
    pub use utils::*;
}
pub mod auxdata {
    pub use crate::generated::auxdata::*;
    pub mod metadata;
    pub use metadata::*;
    pub mod utils;
}
pub mod certs {
    pub use crate::generated::certs::*;
    pub mod utils;
}
pub mod crypto {
    pub use crate::generated::crypto::*;
    pub mod hash;
    pub mod utils;
}
pub mod governance {
    pub use crate::generated::governance::*;
    pub mod utils;
}
pub mod plutus {
    pub use crate::generated::plutus::*;
    pub mod utils;
    pub use utils::{ConstrPlutusData, PlutusMap, PlutusScript};
}
pub mod transaction {
    pub use crate::generated::transaction::*;
    pub mod utils;
}

// Extern-type re-exports so the generated glue (`pub use crate::X;`) resolves.
pub use cml_crypto_wasm::{
    AnchorDocHash, AuxiliaryDataHash, BlockBodyHash, BlockHeaderHash, DatumHash, Ed25519KeyHash,
    Ed25519Signature, GenesisDelegateHash, GenesisHash, KESVkey, NonceHash, PoolMetadataHash,
    ScriptDataHash, ScriptHash, TransactionHash, VRFKeyHash, VRFVkey,
};
pub type Vkey = cml_crypto_wasm::PublicKey;
pub use crate::assets::utils::Value;
pub use crate::auxdata::metadata::Metadata;
pub use crate::plutus::utils::{ConstrPlutusData, PlutusMap};
pub use crate::transaction::RequiredSigners;
pub use address::{Address, RewardAccount};
pub use byron::AddrAttributes;
pub use cml_chain::Coin;
pub use utils::BigInteger;

// Imports/aliases the hand-written modules pull in via `use super::*` /
// `use crate::*` or reference bare as `crate::X` (these were private root
// imports in the pre-thin-root lib.rs).
use crate::assets::AssetName;
use crate::crypto::Vkeywitness;
use crate::transaction::NativeScript;
use ::wasm_bindgen::prelude::{JsError, wasm_bindgen};
