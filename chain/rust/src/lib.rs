// This recently introduced lint does not play well with the derivative crate.
// We have both Ord and PartialOrd derive automatically by derivative's proc macros
// but clippy sees these as hand implementations.
// Putting this allow locally where it's found did not seem to supress it,
// likely due to the structure of how the proc macro derives the code.
// Doing what is suggested by this lint would just result in us actually doing
// hand implementations of the PartialOrd (an maybe PartialEq) when there's no need,
// possibly impacting PartialOrd performance on top of being unnecessary and occuring in generated code.
// Possibly the derivative crate could get updated to suppress this lint
// from within their proc macros itself. Issue: https://github.com/mcarton/rust-derivative/issues/115
#![allow(clippy::non_canonical_partial_ord_impl)]

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
pub mod plutus {
    pub use crate::generated::plutus::*;
    pub mod utils;
    pub use utils::{ConstrPlutusData, PlutusMap, PlutusScript};
}
pub mod transaction {
    pub use crate::generated::transaction::*;
    pub mod utils;
}

//pub mod legacy_address;

pub use crate::assets::{Coin, Value};
pub use utils::BigInteger;

// Re-exported at the root because downstream crates' generated code imports these
// through their extern dep (`use cml_chain::{LenEncoding, ...}`).
pub use cml_core::ordered_hash_map::OrderedHashMap;
pub use cml_core::serialization::{Deserialize, LenEncoding, Serialize, StringEncoding};

// Extern-type re-exports so the generated glue (`pub use crate::X;`) resolves.
pub use cml_crypto::{
    AnchorDocHash, AuxiliaryDataHash, BlockBodyHash, BlockHeaderHash, DatumHash, Ed25519KeyHash,
    Ed25519Signature, GenesisDelegateHash, GenesisHash, KESVkey, NonceHash, PoolMetadataHash,
    ScriptDataHash, ScriptHash, TransactionHash, VRFKeyHash, VRFVkey,
};
pub type Vkey = cml_crypto::PublicKey;
pub use crate::auxdata::metadata::Metadata;
pub use crate::plutus::utils::{ConstrPlutusData, PlutusMap};
pub use address::{Address, RewardAccount};
pub use byron::AddrAttributes;
pub use transaction::RequiredSigners;

// Crate-root aliases the hand-written builder/util modules reference bare
// (these were private root imports in the pre-thin-root lib.rs).
use crate::plutus::ExUnitPrices;
use crate::transaction::{NativeScript, Script};
