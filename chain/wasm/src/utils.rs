use crate::SubCoin;

use super::{Int, Script, ScriptHash};
use cml_chain::plutus::Language;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use cml_core_wasm::{impl_wasm_cbor_json_api, impl_wasm_conversions, impl_wasm_list};

impl_wasm_list!(Language, Language, LanguageList, true, true);

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BigInteger(cml_chain::utils::BigInteger);

impl_wasm_conversions!(cml_chain::utils::BigInteger, BigInteger);

impl_wasm_cbor_json_api!(BigInteger);

#[wasm_bindgen]
impl BigInteger {
    pub fn from_int(x: &Int) -> Self {
        Self(cml_chain::utils::BigInteger::from_int(x.as_ref()))
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<BigInteger, JsError> {
        use std::str::FromStr;
        cml_chain::utils::BigInteger::from_str(s)
            .map(Self)
            .map_err(Into::into)
    }

    pub fn to_str(&self) -> String {
        self.0.to_string()
    }

    /// Converts to a u64
    /// Returns None if the number was negative or too big for a u64
    pub fn as_u64(&self) -> Option<u64> {
        self.0.as_u64()
    }

    /// Converts to an Int
    /// Returns None when the number is too big for an Int (outside +/- 64-bit unsigned)
    /// Retains encoding info if the original was encoded as an Int
    pub fn as_int(&self) -> Option<Int> {
        self.0.as_int().map(Into::into)
    }
}

#[wasm_bindgen]
impl Script {
    pub fn hash(&self) -> ScriptHash {
        self.0.hash().into()
    }

    // Returns which language the script is if it's a Plutus script
    // Returns None otherwise (i.e. NativeScript)
    pub fn language(&self) -> Option<Language> {
        self.0.language()
    }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct NetworkId(cml_chain::NetworkId);

impl_wasm_cbor_json_api!(NetworkId);

impl_wasm_conversions!(cml_chain::NetworkId, NetworkId);

#[wasm_bindgen]
impl NetworkId {
    pub fn new(network: u64) -> Self {
        cml_chain::NetworkId::new(network).into()
    }

    pub fn mainnet() -> Self {
        cml_chain::NetworkId::mainnet().into()
    }

    pub fn testnet() -> Self {
        cml_chain::NetworkId::testnet().into()
    }

    pub fn network(&self) -> u64 {
        self.0.network
    }
}

#[wasm_bindgen]
impl SubCoin {
    /// Converts base 10 floats to SubCoin.
    /// This is the format used by blockfrost for ex units
    /// Warning: If the passed in float was not meant to be base 10
    /// this might result in a slightly inaccurate fraction.
    pub fn from_base10_f32(f: f32) -> Self {
        cml_chain::SubCoin::from_base10_f32(f).into()
    }
}

// we provide direct From/Into conversions between NonemptySet<T> and TList
// to allow the auto-generated code to work directly without changes
macro_rules! impl_wasm_conversions_into {
    ($rust:ty, $wasm:ty) => {
        impl From<$rust> for $wasm {
            fn from(native: $rust) -> Self {
                Self(native.into())
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<$rust> for $wasm {
            fn into(self) -> $rust {
                self.0.into()
            }
        }
    };
}

impl_wasm_conversions_into!(cml_chain::NonemptySetCertificate, crate::CertificateList);

impl_wasm_conversions_into!(
    cml_chain::NonemptySetPlutusV1Script,
    crate::PlutusV1ScriptList
);

impl_wasm_conversions_into!(
    cml_chain::NonemptySetPlutusV2Script,
    crate::PlutusV2ScriptList
);

impl_wasm_conversions_into!(
    cml_chain::NonemptySetPlutusV3Script,
    crate::PlutusV3ScriptList
);

impl_wasm_conversions_into!(cml_chain::NonemptySetPlutusData, crate::PlutusDataList);

impl_wasm_conversions_into!(
    cml_chain::NonemptySetBootstrapWitness,
    crate::BootstrapWitnessList
);

impl_wasm_conversions_into!(cml_chain::NonemptySetVkeywitness, crate::VkeywitnessList);

impl_wasm_conversions_into!(
    cml_chain::NonemptySetProposalProcedure,
    crate::ProposalProcedureList
);

impl_wasm_conversions_into!(
    cml_chain::NonemptySetTransactionInput,
    crate::TransactionInputList
);

impl_wasm_conversions_into!(cml_chain::NonemptySetNativeScript, crate::NativeScriptList);

impl_wasm_conversions_into!(
    cml_chain::utils::NonemptySetRawBytes<cml_crypto::Ed25519KeyHash>,
    crate::Ed25519KeyHashList
);

impl_wasm_conversions_into!(
    cml_chain::SetCommitteeColdCredential,
    crate::CommitteeColdCredentialList
);
