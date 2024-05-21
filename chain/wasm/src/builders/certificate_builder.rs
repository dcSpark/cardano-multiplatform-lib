use crate::builders::witness_builder::PartialPlutusWitness;
use crate::*;
use cml_core_wasm::impl_wasm_conversions;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

use super::witness_builder::NativeScriptWitnessInfo;

use crate::{certs::Certificate, RequiredSigners};

#[wasm_bindgen]
#[derive(Clone)]
pub struct CertificateBuilderResult(
    cml_chain::builders::certificate_builder::CertificateBuilderResult,
);

impl_wasm_conversions!(
    cml_chain::builders::certificate_builder::CertificateBuilderResult,
    CertificateBuilderResult
);

#[wasm_bindgen]
#[derive(Clone)]
pub struct SingleCertificateBuilder(
    cml_chain::builders::certificate_builder::SingleCertificateBuilder,
);

impl_wasm_conversions!(
    cml_chain::builders::certificate_builder::SingleCertificateBuilder,
    SingleCertificateBuilder
);

#[wasm_bindgen]
impl SingleCertificateBuilder {
    pub fn new(cert: &Certificate) -> Self {
        cml_chain::builders::certificate_builder::SingleCertificateBuilder::new(cert.clone().into())
            .into()
    }

    /// note: particularly useful for StakeRegistration which doesn't require witnessing
    pub fn skip_witness(&self) -> CertificateBuilderResult {
        self.0.clone().skip_witness().into()
    }

    pub fn payment_key(&self) -> Result<CertificateBuilderResult, JsError> {
        self.0
            .clone()
            .payment_key()
            .map(Into::into)
            .map_err(Into::into)
    }

    /** Signer keys don't have to be set. You can leave it empty and then add the required witnesses later */
    pub fn native_script(
        &self,
        native_script: &NativeScript,
        witness_info: &NativeScriptWitnessInfo,
    ) -> Result<CertificateBuilderResult, JsError> {
        self.0
            .clone()
            .native_script(native_script.clone().into(), witness_info.clone().into())
            .map(Into::into)
            .map_err(Into::into)
    }

    pub fn plutus_script(
        self,
        partial_witness: &PartialPlutusWitness,
        required_signers: &RequiredSigners,
    ) -> Result<CertificateBuilderResult, JsError> {
        self.0
            .plutus_script(
                partial_witness.clone().into(),
                required_signers.clone().into(),
            )
            .map(Into::into)
            .map_err(Into::into)
    }
}
