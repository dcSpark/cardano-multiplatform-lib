// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

#![allow(
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::result_large_err
)]

extern crate derivative;
pub mod address;
pub mod assets;
pub mod auxdata;
pub mod block;
pub mod certs;
pub mod crypto;
mod extern_interface_check;
pub mod governance;
mod key_demand_assertions;
pub mod plutus;
pub mod transaction;
pub use cml_core::{Int, IntError};
impl std::ops::Deref for NonemptySetBootstrapWitness {
    type Target = NonEmptyOrderedSet<BootstrapWitness>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for NonemptySetBootstrapWitness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a NonemptySetBootstrapWitness {
    type Item = &'a BootstrapWitness;
    type IntoIter = std::slice::Iter<'a, BootstrapWitness>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for NonemptySetBootstrapWitness {
    type Item = BootstrapWitness;
    type IntoIter = std::vec::IntoIter<BootstrapWitness>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<BootstrapWitness>::from(self.inner).into_iter()
    }
}

impl From<NonemptySetBootstrapWitness> for Vec<BootstrapWitness> {
    fn from(wrapper: NonemptySetBootstrapWitness) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<BootstrapWitness>> for NonemptySetBootstrapWitness {
    type Error = DeserializeError;

    fn try_from(vec: Vec<BootstrapWitness>) -> Result<Self, Self::Error> {
        Ok(NonemptySetBootstrapWitness::new(<NonEmptyOrderedSet<
            BootstrapWitness,
        >>::try_from(vec)?))
    }
}

impl NonemptySetBootstrapWitness {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<BootstrapWitness>) -> Result<Option<Self>, DeserializeError> {
        Ok(<NonEmptyOrderedSet<BootstrapWitness>>::try_opt_from(vec)?
            .map(NonemptySetBootstrapWitness::new))
    }
}

impl std::ops::Deref for NonemptySetCertificate {
    type Target = NonEmptyOrderedSet<Certificate>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for NonemptySetCertificate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a NonemptySetCertificate {
    type Item = &'a Certificate;
    type IntoIter = std::slice::Iter<'a, Certificate>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for NonemptySetCertificate {
    type Item = Certificate;
    type IntoIter = std::vec::IntoIter<Certificate>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<Certificate>::from(self.inner).into_iter()
    }
}

impl From<NonemptySetCertificate> for Vec<Certificate> {
    fn from(wrapper: NonemptySetCertificate) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<Certificate>> for NonemptySetCertificate {
    type Error = DeserializeError;

    fn try_from(vec: Vec<Certificate>) -> Result<Self, Self::Error> {
        Ok(NonemptySetCertificate::new(<NonEmptyOrderedSet<
            Certificate,
        >>::try_from(vec)?))
    }
}

impl NonemptySetCertificate {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<Certificate>) -> Result<Option<Self>, DeserializeError> {
        Ok(<NonEmptyOrderedSet<Certificate>>::try_opt_from(vec)?.map(NonemptySetCertificate::new))
    }
}

impl std::ops::Deref for NonemptySetEd25519KeyHash {
    type Target = NonEmptyOrderedSet<Ed25519KeyHash>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for NonemptySetEd25519KeyHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a NonemptySetEd25519KeyHash {
    type Item = &'a Ed25519KeyHash;
    type IntoIter = std::slice::Iter<'a, Ed25519KeyHash>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for NonemptySetEd25519KeyHash {
    type Item = Ed25519KeyHash;
    type IntoIter = std::vec::IntoIter<Ed25519KeyHash>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<Ed25519KeyHash>::from(self.inner).into_iter()
    }
}

impl From<NonemptySetEd25519KeyHash> for Vec<Ed25519KeyHash> {
    fn from(wrapper: NonemptySetEd25519KeyHash) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<Ed25519KeyHash>> for NonemptySetEd25519KeyHash {
    type Error = DeserializeError;

    fn try_from(vec: Vec<Ed25519KeyHash>) -> Result<Self, Self::Error> {
        Ok(NonemptySetEd25519KeyHash::new(<NonEmptyOrderedSet<
            Ed25519KeyHash,
        >>::try_from(vec)?))
    }
}

impl NonemptySetEd25519KeyHash {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<Ed25519KeyHash>) -> Result<Option<Self>, DeserializeError> {
        Ok(<NonEmptyOrderedSet<Ed25519KeyHash>>::try_opt_from(vec)?
            .map(NonemptySetEd25519KeyHash::new))
    }
}

impl std::ops::Deref for NonemptySetNativeScript {
    type Target = NonEmptyOrderedSet<NativeScript>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for NonemptySetNativeScript {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a NonemptySetNativeScript {
    type Item = &'a NativeScript;
    type IntoIter = std::slice::Iter<'a, NativeScript>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for NonemptySetNativeScript {
    type Item = NativeScript;
    type IntoIter = std::vec::IntoIter<NativeScript>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<NativeScript>::from(self.inner).into_iter()
    }
}

impl From<NonemptySetNativeScript> for Vec<NativeScript> {
    fn from(wrapper: NonemptySetNativeScript) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<NativeScript>> for NonemptySetNativeScript {
    type Error = DeserializeError;

    fn try_from(vec: Vec<NativeScript>) -> Result<Self, Self::Error> {
        Ok(NonemptySetNativeScript::new(<NonEmptyOrderedSet<
            NativeScript,
        >>::try_from(vec)?))
    }
}

impl NonemptySetNativeScript {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<NativeScript>) -> Result<Option<Self>, DeserializeError> {
        Ok(
            <NonEmptyOrderedSet<NativeScript>>::try_opt_from(vec)?
                .map(NonemptySetNativeScript::new),
        )
    }
}

impl std::ops::Deref for NonemptySetPlutusData {
    type Target = NonEmptyOrderedSet<PlutusData>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for NonemptySetPlutusData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a NonemptySetPlutusData {
    type Item = &'a PlutusData;
    type IntoIter = std::slice::Iter<'a, PlutusData>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for NonemptySetPlutusData {
    type Item = PlutusData;
    type IntoIter = std::vec::IntoIter<PlutusData>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<PlutusData>::from(self.inner).into_iter()
    }
}

impl From<NonemptySetPlutusData> for Vec<PlutusData> {
    fn from(wrapper: NonemptySetPlutusData) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<PlutusData>> for NonemptySetPlutusData {
    type Error = DeserializeError;

    fn try_from(vec: Vec<PlutusData>) -> Result<Self, Self::Error> {
        Ok(NonemptySetPlutusData::new(
            <NonEmptyOrderedSet<PlutusData>>::try_from(vec)?,
        ))
    }
}

impl NonemptySetPlutusData {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<PlutusData>) -> Result<Option<Self>, DeserializeError> {
        Ok(<NonEmptyOrderedSet<PlutusData>>::try_opt_from(vec)?.map(NonemptySetPlutusData::new))
    }
}

impl std::ops::Deref for NonemptySetPlutusV1Script {
    type Target = NonEmptyOrderedSet<PlutusV1Script>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for NonemptySetPlutusV1Script {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a NonemptySetPlutusV1Script {
    type Item = &'a PlutusV1Script;
    type IntoIter = std::slice::Iter<'a, PlutusV1Script>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for NonemptySetPlutusV1Script {
    type Item = PlutusV1Script;
    type IntoIter = std::vec::IntoIter<PlutusV1Script>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<PlutusV1Script>::from(self.inner).into_iter()
    }
}

impl From<NonemptySetPlutusV1Script> for Vec<PlutusV1Script> {
    fn from(wrapper: NonemptySetPlutusV1Script) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<PlutusV1Script>> for NonemptySetPlutusV1Script {
    type Error = DeserializeError;

    fn try_from(vec: Vec<PlutusV1Script>) -> Result<Self, Self::Error> {
        Ok(NonemptySetPlutusV1Script::new(<NonEmptyOrderedSet<
            PlutusV1Script,
        >>::try_from(vec)?))
    }
}

impl NonemptySetPlutusV1Script {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<PlutusV1Script>) -> Result<Option<Self>, DeserializeError> {
        Ok(<NonEmptyOrderedSet<PlutusV1Script>>::try_opt_from(vec)?
            .map(NonemptySetPlutusV1Script::new))
    }
}

impl std::ops::Deref for NonemptySetPlutusV2Script {
    type Target = NonEmptyOrderedSet<PlutusV2Script>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for NonemptySetPlutusV2Script {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a NonemptySetPlutusV2Script {
    type Item = &'a PlutusV2Script;
    type IntoIter = std::slice::Iter<'a, PlutusV2Script>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for NonemptySetPlutusV2Script {
    type Item = PlutusV2Script;
    type IntoIter = std::vec::IntoIter<PlutusV2Script>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<PlutusV2Script>::from(self.inner).into_iter()
    }
}

impl From<NonemptySetPlutusV2Script> for Vec<PlutusV2Script> {
    fn from(wrapper: NonemptySetPlutusV2Script) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<PlutusV2Script>> for NonemptySetPlutusV2Script {
    type Error = DeserializeError;

    fn try_from(vec: Vec<PlutusV2Script>) -> Result<Self, Self::Error> {
        Ok(NonemptySetPlutusV2Script::new(<NonEmptyOrderedSet<
            PlutusV2Script,
        >>::try_from(vec)?))
    }
}

impl NonemptySetPlutusV2Script {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<PlutusV2Script>) -> Result<Option<Self>, DeserializeError> {
        Ok(<NonEmptyOrderedSet<PlutusV2Script>>::try_opt_from(vec)?
            .map(NonemptySetPlutusV2Script::new))
    }
}

impl std::ops::Deref for NonemptySetPlutusV3Script {
    type Target = NonEmptyOrderedSet<PlutusV3Script>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for NonemptySetPlutusV3Script {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a NonemptySetPlutusV3Script {
    type Item = &'a PlutusV3Script;
    type IntoIter = std::slice::Iter<'a, PlutusV3Script>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for NonemptySetPlutusV3Script {
    type Item = PlutusV3Script;
    type IntoIter = std::vec::IntoIter<PlutusV3Script>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<PlutusV3Script>::from(self.inner).into_iter()
    }
}

impl From<NonemptySetPlutusV3Script> for Vec<PlutusV3Script> {
    fn from(wrapper: NonemptySetPlutusV3Script) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<PlutusV3Script>> for NonemptySetPlutusV3Script {
    type Error = DeserializeError;

    fn try_from(vec: Vec<PlutusV3Script>) -> Result<Self, Self::Error> {
        Ok(NonemptySetPlutusV3Script::new(<NonEmptyOrderedSet<
            PlutusV3Script,
        >>::try_from(vec)?))
    }
}

impl NonemptySetPlutusV3Script {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<PlutusV3Script>) -> Result<Option<Self>, DeserializeError> {
        Ok(<NonEmptyOrderedSet<PlutusV3Script>>::try_opt_from(vec)?
            .map(NonemptySetPlutusV3Script::new))
    }
}

impl std::ops::Deref for NonemptySetProposalProcedure {
    type Target = NonEmptyOrderedSet<ProposalProcedure>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for NonemptySetProposalProcedure {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a NonemptySetProposalProcedure {
    type Item = &'a ProposalProcedure;
    type IntoIter = std::slice::Iter<'a, ProposalProcedure>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for NonemptySetProposalProcedure {
    type Item = ProposalProcedure;
    type IntoIter = std::vec::IntoIter<ProposalProcedure>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<ProposalProcedure>::from(self.inner).into_iter()
    }
}

impl From<NonemptySetProposalProcedure> for Vec<ProposalProcedure> {
    fn from(wrapper: NonemptySetProposalProcedure) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<ProposalProcedure>> for NonemptySetProposalProcedure {
    type Error = DeserializeError;

    fn try_from(vec: Vec<ProposalProcedure>) -> Result<Self, Self::Error> {
        Ok(NonemptySetProposalProcedure::new(<NonEmptyOrderedSet<
            ProposalProcedure,
        >>::try_from(vec)?))
    }
}

impl NonemptySetProposalProcedure {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<ProposalProcedure>) -> Result<Option<Self>, DeserializeError> {
        Ok(<NonEmptyOrderedSet<ProposalProcedure>>::try_opt_from(vec)?
            .map(NonemptySetProposalProcedure::new))
    }
}

impl std::ops::Deref for NonemptySetTransactionInput {
    type Target = NonEmptyOrderedSet<TransactionInput>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for NonemptySetTransactionInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a NonemptySetTransactionInput {
    type Item = &'a TransactionInput;
    type IntoIter = std::slice::Iter<'a, TransactionInput>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for NonemptySetTransactionInput {
    type Item = TransactionInput;
    type IntoIter = std::vec::IntoIter<TransactionInput>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<TransactionInput>::from(self.inner).into_iter()
    }
}

impl From<NonemptySetTransactionInput> for Vec<TransactionInput> {
    fn from(wrapper: NonemptySetTransactionInput) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<TransactionInput>> for NonemptySetTransactionInput {
    type Error = DeserializeError;

    fn try_from(vec: Vec<TransactionInput>) -> Result<Self, Self::Error> {
        Ok(NonemptySetTransactionInput::new(<NonEmptyOrderedSet<
            TransactionInput,
        >>::try_from(vec)?))
    }
}

impl NonemptySetTransactionInput {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<TransactionInput>) -> Result<Option<Self>, DeserializeError> {
        Ok(<NonEmptyOrderedSet<TransactionInput>>::try_opt_from(vec)?
            .map(NonemptySetTransactionInput::new))
    }
}

impl std::ops::Deref for NonemptySetVkeywitness {
    type Target = NonEmptyOrderedSet<Vkeywitness>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for NonemptySetVkeywitness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a NonemptySetVkeywitness {
    type Item = &'a Vkeywitness;
    type IntoIter = std::slice::Iter<'a, Vkeywitness>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for NonemptySetVkeywitness {
    type Item = Vkeywitness;
    type IntoIter = std::vec::IntoIter<Vkeywitness>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<Vkeywitness>::from(self.inner).into_iter()
    }
}

impl From<NonemptySetVkeywitness> for Vec<Vkeywitness> {
    fn from(wrapper: NonemptySetVkeywitness) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<Vkeywitness>> for NonemptySetVkeywitness {
    type Error = DeserializeError;

    fn try_from(vec: Vec<Vkeywitness>) -> Result<Self, Self::Error> {
        Ok(NonemptySetVkeywitness::new(<NonEmptyOrderedSet<
            Vkeywitness,
        >>::try_from(vec)?))
    }
}

impl NonemptySetVkeywitness {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<Vkeywitness>) -> Result<Option<Self>, DeserializeError> {
        Ok(<NonEmptyOrderedSet<Vkeywitness>>::try_opt_from(vec)?.map(NonemptySetVkeywitness::new))
    }
}

impl std::ops::Deref for SetCommitteeColdCredential {
    type Target = OrderedSet<CommitteeColdCredential>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for SetCommitteeColdCredential {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a SetCommitteeColdCredential {
    type Item = &'a CommitteeColdCredential;
    type IntoIter = std::slice::Iter<'a, CommitteeColdCredential>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for SetCommitteeColdCredential {
    type Item = CommitteeColdCredential;
    type IntoIter = std::vec::IntoIter<CommitteeColdCredential>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<CommitteeColdCredential>::from(self.inner).into_iter()
    }
}

impl From<SetCommitteeColdCredential> for Vec<CommitteeColdCredential> {
    fn from(wrapper: SetCommitteeColdCredential) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<CommitteeColdCredential>> for SetCommitteeColdCredential {
    type Error = DeserializeError;

    fn try_from(vec: Vec<CommitteeColdCredential>) -> Result<Self, Self::Error> {
        Ok(SetCommitteeColdCredential::new(<OrderedSet<
            CommitteeColdCredential,
        >>::try_from(vec)?))
    }
}

impl SetCommitteeColdCredential {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(
        vec: Vec<CommitteeColdCredential>,
    ) -> Result<Option<Self>, DeserializeError> {
        Ok(<OrderedSet<CommitteeColdCredential>>::try_opt_from(vec)?
            .map(SetCommitteeColdCredential::new))
    }
}

impl std::ops::Deref for SetEd25519KeyHash {
    type Target = OrderedSet<Ed25519KeyHash>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for SetEd25519KeyHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a SetEd25519KeyHash {
    type Item = &'a Ed25519KeyHash;
    type IntoIter = std::slice::Iter<'a, Ed25519KeyHash>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for SetEd25519KeyHash {
    type Item = Ed25519KeyHash;
    type IntoIter = std::vec::IntoIter<Ed25519KeyHash>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<Ed25519KeyHash>::from(self.inner).into_iter()
    }
}

impl From<SetEd25519KeyHash> for Vec<Ed25519KeyHash> {
    fn from(wrapper: SetEd25519KeyHash) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<Ed25519KeyHash>> for SetEd25519KeyHash {
    type Error = DeserializeError;

    fn try_from(vec: Vec<Ed25519KeyHash>) -> Result<Self, Self::Error> {
        Ok(SetEd25519KeyHash::new(
            <OrderedSet<Ed25519KeyHash>>::try_from(vec)?,
        ))
    }
}

impl SetEd25519KeyHash {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<Ed25519KeyHash>) -> Result<Option<Self>, DeserializeError> {
        Ok(<OrderedSet<Ed25519KeyHash>>::try_opt_from(vec)?.map(SetEd25519KeyHash::new))
    }
}

impl std::ops::Deref for SetTransactionInput {
    type Target = OrderedSet<TransactionInput>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for SetTransactionInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a SetTransactionInput {
    type Item = &'a TransactionInput;
    type IntoIter = std::slice::Iter<'a, TransactionInput>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl IntoIterator for SetTransactionInput {
    type Item = TransactionInput;
    type IntoIter = std::vec::IntoIter<TransactionInput>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<TransactionInput>::from(self.inner).into_iter()
    }
}

impl From<SetTransactionInput> for Vec<TransactionInput> {
    fn from(wrapper: SetTransactionInput) -> Self {
        Vec::from(wrapper.inner)
    }
}

impl TryFrom<Vec<TransactionInput>> for SetTransactionInput {
    type Error = DeserializeError;

    fn try_from(vec: Vec<TransactionInput>) -> Result<Self, Self::Error> {
        Ok(SetTransactionInput::new(
            <OrderedSet<TransactionInput>>::try_from(vec)?,
        ))
    }
}

impl SetTransactionInput {
    /// Empty input is `Ok(None)` (the optional set field is absent); a non-empty input goes through
    /// the inner uniqueness door wrapped in `Some`, so ONLY a duplicate surfaces as `Err`.
    pub fn try_opt_from(vec: Vec<TransactionInput>) -> Result<Option<Self>, DeserializeError> {
        Ok(<OrderedSet<TransactionInput>>::try_opt_from(vec)?.map(SetTransactionInput::new))
    }
}

pub mod cbor_encodings;
pub mod serialization;

use address::RewardAccount;
use assets::Coin;
use cbor_encodings::{
    DRepVotingThresholdsEncoding, NetworkIdEncoding, NonemptySetBootstrapWitnessEncoding,
    NonemptySetCertificateEncoding, NonemptySetEd25519KeyHashEncoding,
    NonemptySetNativeScriptEncoding, NonemptySetPlutusDataEncoding,
    NonemptySetPlutusV1ScriptEncoding, NonemptySetPlutusV2ScriptEncoding,
    NonemptySetPlutusV3ScriptEncoding, NonemptySetProposalProcedureEncoding,
    NonemptySetTransactionInputEncoding, NonemptySetVkeywitnessEncoding,
    PoolVotingThresholdsEncoding, ProtocolParamUpdateEncoding, RationalEncoding,
    SetCommitteeColdCredentialEncoding, SetEd25519KeyHashEncoding, SetTransactionInputEncoding,
    UnitIntervalEncoding,
};
use certs::{Certificate, CommitteeColdCredential, Credential};
use cml_core::error::*;
use cml_core::ordered_hash_map::OrderedHashMap;
use cml_core::ordered_set::{NonEmptyOrderedSet, OrderedSet};
use crypto::{BootstrapWitness, Ed25519KeyHash, ScriptHash, Vkeywitness};
use governance::ProposalProcedure;
use plutus::{
    CostModels, ExUnitPrices, ExUnits, PlutusData, PlutusV1Script, PlutusV2Script, PlutusV3Script,
};
use transaction::{NativeScript, TransactionInput};

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DRepVotingThresholds {
    pub motion_no_confidence: UnitInterval,
    pub committee_normal: UnitInterval,
    pub committee_no_confidence: UnitInterval,
    pub update_constitution: UnitInterval,
    pub hard_fork_initiation: UnitInterval,
    pub pp_network_group: UnitInterval,
    pub pp_economic_group: UnitInterval,
    pub pp_technical_group: UnitInterval,
    pub pp_governance_group: UnitInterval,
    pub treasury_withdrawal: UnitInterval,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    #[serde(skip)]
    pub encodings: Option<DRepVotingThresholdsEncoding>,
}

impl DRepVotingThresholds {
    pub fn new(
        motion_no_confidence: UnitInterval,
        committee_normal: UnitInterval,
        committee_no_confidence: UnitInterval,
        update_constitution: UnitInterval,
        hard_fork_initiation: UnitInterval,
        pp_network_group: UnitInterval,
        pp_economic_group: UnitInterval,
        pp_technical_group: UnitInterval,
        pp_governance_group: UnitInterval,
        treasury_withdrawal: UnitInterval,
    ) -> Self {
        Self {
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
            encodings: None,
        }
    }
}

pub type DeltaCoin = Int;

pub type Epoch = u64;

#[derive(Clone, Debug)]
pub struct NetworkId {
    pub(crate) inner: u64,
    pub encodings: Option<NetworkIdEncoding>,
}

impl NetworkId {
    pub fn get(&self) -> u64 {
        self.inner
    }

    pub fn new(inner: u64) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<u64> for NetworkId {
    fn from(inner: u64) -> Self {
        NetworkId::new(inner)
    }
}

impl serde::Serialize for NetworkId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NetworkId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <u64 as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NetworkId {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NetworkId")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <u64 as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <u64 as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonemptySetBootstrapWitness {
    pub(crate) inner: NonEmptyOrderedSet<BootstrapWitness>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<NonemptySetBootstrapWitnessEncoding>,
}

impl NonemptySetBootstrapWitness {
    pub fn new(inner: NonEmptyOrderedSet<BootstrapWitness>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<NonEmptyOrderedSet<BootstrapWitness>> for NonemptySetBootstrapWitness {
    fn from(inner: NonEmptyOrderedSet<BootstrapWitness>) -> Self {
        NonemptySetBootstrapWitness::new(inner)
    }
}

impl From<NonemptySetBootstrapWitness> for NonEmptyOrderedSet<BootstrapWitness> {
    fn from(wrapper: NonemptySetBootstrapWitness) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for NonemptySetBootstrapWitness {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NonemptySetBootstrapWitness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <NonEmptyOrderedSet<BootstrapWitness> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NonemptySetBootstrapWitness {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NonemptySetBootstrapWitness")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonEmptyOrderedSet<BootstrapWitness> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <NonEmptyOrderedSet<BootstrapWitness> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonemptySetCertificate {
    pub(crate) inner: NonEmptyOrderedSet<Certificate>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<NonemptySetCertificateEncoding>,
}

impl NonemptySetCertificate {
    pub fn new(inner: NonEmptyOrderedSet<Certificate>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<NonEmptyOrderedSet<Certificate>> for NonemptySetCertificate {
    fn from(inner: NonEmptyOrderedSet<Certificate>) -> Self {
        NonemptySetCertificate::new(inner)
    }
}

impl From<NonemptySetCertificate> for NonEmptyOrderedSet<Certificate> {
    fn from(wrapper: NonemptySetCertificate) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for NonemptySetCertificate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NonemptySetCertificate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner =
            <NonEmptyOrderedSet<Certificate> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NonemptySetCertificate {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NonemptySetCertificate")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonEmptyOrderedSet<Certificate> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <NonEmptyOrderedSet<Certificate> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonemptySetEd25519KeyHash {
    pub(crate) inner: NonEmptyOrderedSet<Ed25519KeyHash>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<NonemptySetEd25519KeyHashEncoding>,
}

impl NonemptySetEd25519KeyHash {
    pub fn new(inner: NonEmptyOrderedSet<Ed25519KeyHash>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<NonEmptyOrderedSet<Ed25519KeyHash>> for NonemptySetEd25519KeyHash {
    fn from(inner: NonEmptyOrderedSet<Ed25519KeyHash>) -> Self {
        NonemptySetEd25519KeyHash::new(inner)
    }
}

impl From<NonemptySetEd25519KeyHash> for NonEmptyOrderedSet<Ed25519KeyHash> {
    fn from(wrapper: NonemptySetEd25519KeyHash) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for NonemptySetEd25519KeyHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NonemptySetEd25519KeyHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <NonEmptyOrderedSet<Ed25519KeyHash> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NonemptySetEd25519KeyHash {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NonemptySetEd25519KeyHash")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonEmptyOrderedSet<Ed25519KeyHash> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <NonEmptyOrderedSet<Ed25519KeyHash> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonemptySetNativeScript {
    pub(crate) inner: NonEmptyOrderedSet<NativeScript>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<NonemptySetNativeScriptEncoding>,
}

impl NonemptySetNativeScript {
    pub fn new(inner: NonEmptyOrderedSet<NativeScript>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<NonEmptyOrderedSet<NativeScript>> for NonemptySetNativeScript {
    fn from(inner: NonEmptyOrderedSet<NativeScript>) -> Self {
        NonemptySetNativeScript::new(inner)
    }
}

impl From<NonemptySetNativeScript> for NonEmptyOrderedSet<NativeScript> {
    fn from(wrapper: NonemptySetNativeScript) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for NonemptySetNativeScript {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NonemptySetNativeScript {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <NonEmptyOrderedSet<NativeScript> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NonemptySetNativeScript {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NonemptySetNativeScript")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonEmptyOrderedSet<NativeScript> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <NonEmptyOrderedSet<NativeScript> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonemptySetPlutusData {
    pub(crate) inner: NonEmptyOrderedSet<PlutusData>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<NonemptySetPlutusDataEncoding>,
}

impl NonemptySetPlutusData {
    pub fn new(inner: NonEmptyOrderedSet<PlutusData>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<NonEmptyOrderedSet<PlutusData>> for NonemptySetPlutusData {
    fn from(inner: NonEmptyOrderedSet<PlutusData>) -> Self {
        NonemptySetPlutusData::new(inner)
    }
}

impl From<NonemptySetPlutusData> for NonEmptyOrderedSet<PlutusData> {
    fn from(wrapper: NonemptySetPlutusData) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for NonemptySetPlutusData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NonemptySetPlutusData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner =
            <NonEmptyOrderedSet<PlutusData> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NonemptySetPlutusData {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NonemptySetPlutusData")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonEmptyOrderedSet<PlutusData> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <NonEmptyOrderedSet<PlutusData> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonemptySetPlutusV1Script {
    pub(crate) inner: NonEmptyOrderedSet<PlutusV1Script>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<NonemptySetPlutusV1ScriptEncoding>,
}

impl NonemptySetPlutusV1Script {
    pub fn new(inner: NonEmptyOrderedSet<PlutusV1Script>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<NonEmptyOrderedSet<PlutusV1Script>> for NonemptySetPlutusV1Script {
    fn from(inner: NonEmptyOrderedSet<PlutusV1Script>) -> Self {
        NonemptySetPlutusV1Script::new(inner)
    }
}

impl From<NonemptySetPlutusV1Script> for NonEmptyOrderedSet<PlutusV1Script> {
    fn from(wrapper: NonemptySetPlutusV1Script) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for NonemptySetPlutusV1Script {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NonemptySetPlutusV1Script {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <NonEmptyOrderedSet<PlutusV1Script> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NonemptySetPlutusV1Script {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NonemptySetPlutusV1Script")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonEmptyOrderedSet<PlutusV1Script> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <NonEmptyOrderedSet<PlutusV1Script> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonemptySetPlutusV2Script {
    pub(crate) inner: NonEmptyOrderedSet<PlutusV2Script>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<NonemptySetPlutusV2ScriptEncoding>,
}

impl NonemptySetPlutusV2Script {
    pub fn new(inner: NonEmptyOrderedSet<PlutusV2Script>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<NonEmptyOrderedSet<PlutusV2Script>> for NonemptySetPlutusV2Script {
    fn from(inner: NonEmptyOrderedSet<PlutusV2Script>) -> Self {
        NonemptySetPlutusV2Script::new(inner)
    }
}

impl From<NonemptySetPlutusV2Script> for NonEmptyOrderedSet<PlutusV2Script> {
    fn from(wrapper: NonemptySetPlutusV2Script) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for NonemptySetPlutusV2Script {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NonemptySetPlutusV2Script {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <NonEmptyOrderedSet<PlutusV2Script> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NonemptySetPlutusV2Script {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NonemptySetPlutusV2Script")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonEmptyOrderedSet<PlutusV2Script> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <NonEmptyOrderedSet<PlutusV2Script> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonemptySetPlutusV3Script {
    pub(crate) inner: NonEmptyOrderedSet<PlutusV3Script>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<NonemptySetPlutusV3ScriptEncoding>,
}

impl NonemptySetPlutusV3Script {
    pub fn new(inner: NonEmptyOrderedSet<PlutusV3Script>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<NonEmptyOrderedSet<PlutusV3Script>> for NonemptySetPlutusV3Script {
    fn from(inner: NonEmptyOrderedSet<PlutusV3Script>) -> Self {
        NonemptySetPlutusV3Script::new(inner)
    }
}

impl From<NonemptySetPlutusV3Script> for NonEmptyOrderedSet<PlutusV3Script> {
    fn from(wrapper: NonemptySetPlutusV3Script) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for NonemptySetPlutusV3Script {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NonemptySetPlutusV3Script {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <NonEmptyOrderedSet<PlutusV3Script> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NonemptySetPlutusV3Script {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NonemptySetPlutusV3Script")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonEmptyOrderedSet<PlutusV3Script> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <NonEmptyOrderedSet<PlutusV3Script> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonemptySetProposalProcedure {
    pub(crate) inner: NonEmptyOrderedSet<ProposalProcedure>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<NonemptySetProposalProcedureEncoding>,
}

impl NonemptySetProposalProcedure {
    pub fn new(inner: NonEmptyOrderedSet<ProposalProcedure>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<NonEmptyOrderedSet<ProposalProcedure>> for NonemptySetProposalProcedure {
    fn from(inner: NonEmptyOrderedSet<ProposalProcedure>) -> Self {
        NonemptySetProposalProcedure::new(inner)
    }
}

impl From<NonemptySetProposalProcedure> for NonEmptyOrderedSet<ProposalProcedure> {
    fn from(wrapper: NonemptySetProposalProcedure) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for NonemptySetProposalProcedure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NonemptySetProposalProcedure {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <NonEmptyOrderedSet<ProposalProcedure> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NonemptySetProposalProcedure {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NonemptySetProposalProcedure")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonEmptyOrderedSet<ProposalProcedure> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <NonEmptyOrderedSet<ProposalProcedure> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonemptySetTransactionInput {
    pub(crate) inner: NonEmptyOrderedSet<TransactionInput>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<NonemptySetTransactionInputEncoding>,
}

impl NonemptySetTransactionInput {
    pub fn new(inner: NonEmptyOrderedSet<TransactionInput>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<NonEmptyOrderedSet<TransactionInput>> for NonemptySetTransactionInput {
    fn from(inner: NonEmptyOrderedSet<TransactionInput>) -> Self {
        NonemptySetTransactionInput::new(inner)
    }
}

impl From<NonemptySetTransactionInput> for NonEmptyOrderedSet<TransactionInput> {
    fn from(wrapper: NonemptySetTransactionInput) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for NonemptySetTransactionInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NonemptySetTransactionInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <NonEmptyOrderedSet<TransactionInput> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NonemptySetTransactionInput {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NonemptySetTransactionInput")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonEmptyOrderedSet<TransactionInput> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <NonEmptyOrderedSet<TransactionInput> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NonemptySetVkeywitness {
    pub(crate) inner: NonEmptyOrderedSet<Vkeywitness>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<NonemptySetVkeywitnessEncoding>,
}

impl NonemptySetVkeywitness {
    pub fn new(inner: NonEmptyOrderedSet<Vkeywitness>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<NonEmptyOrderedSet<Vkeywitness>> for NonemptySetVkeywitness {
    fn from(inner: NonEmptyOrderedSet<Vkeywitness>) -> Self {
        NonemptySetVkeywitness::new(inner)
    }
}

impl From<NonemptySetVkeywitness> for NonEmptyOrderedSet<Vkeywitness> {
    fn from(wrapper: NonemptySetVkeywitness) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for NonemptySetVkeywitness {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for NonemptySetVkeywitness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner =
            <NonEmptyOrderedSet<Vkeywitness> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for NonemptySetVkeywitness {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("NonemptySetVkeywitness")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <NonEmptyOrderedSet<Vkeywitness> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <NonEmptyOrderedSet<Vkeywitness> as schemars::JsonSchema>::inline_schema()
    }
}

pub type PolicyId = ScriptHash;

pub type PolicyIdList = Vec<PolicyId>;

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PoolVotingThresholds {
    pub motion_no_confidence: UnitInterval,
    pub committee_normal: UnitInterval,
    pub committee_no_confidence: UnitInterval,
    pub hard_fork_initiation: UnitInterval,
    pub security_relevant_parameter_voting_threshold: UnitInterval,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    #[serde(skip)]
    pub encodings: Option<PoolVotingThresholdsEncoding>,
}

impl PoolVotingThresholds {
    pub fn new(
        motion_no_confidence: UnitInterval,
        committee_normal: UnitInterval,
        committee_no_confidence: UnitInterval,
        hard_fork_initiation: UnitInterval,
        security_relevant_parameter_voting_threshold: UnitInterval,
    ) -> Self {
        Self {
            motion_no_confidence,
            committee_normal,
            committee_no_confidence,
            hard_fork_initiation,
            security_relevant_parameter_voting_threshold,
            encodings: None,
        }
    }
}

pub type Port = u16;

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ProtocolParamUpdate {
    pub minfee_a: Option<Coin>,
    pub minfee_b: Option<Coin>,
    pub max_block_body_size: Option<u64>,
    pub max_transaction_size: Option<u64>,
    pub max_block_header_size: Option<u64>,
    pub key_deposit: Option<Coin>,
    pub pool_deposit: Option<Coin>,
    pub maximum_epoch: Option<Epoch>,
    pub n_opt: Option<u64>,
    pub pool_pledge_influence: Option<Rational>,
    pub expansion_rate: Option<UnitInterval>,
    pub treasury_growth_rate: Option<UnitInterval>,
    pub min_pool_cost: Option<Coin>,
    pub ada_per_utxo_byte: Option<Coin>,
    pub cost_models_for_script_languages: Option<CostModels>,
    pub execution_costs: Option<ExUnitPrices>,
    pub max_tx_ex_units: Option<ExUnits>,
    pub max_block_ex_units: Option<ExUnits>,
    pub max_value_size: Option<u64>,
    pub collateral_percentage: Option<u64>,
    pub max_collateral_inputs: Option<u64>,
    pub pool_voting_thresholds: Option<PoolVotingThresholds>,
    pub d_rep_voting_thresholds: Option<DRepVotingThresholds>,
    pub min_committee_size: Option<u64>,
    pub committee_term_limit: Option<Epoch>,
    pub governance_action_validity_period: Option<Epoch>,
    pub governance_action_deposit: Option<Coin>,
    pub d_rep_deposit: Option<Coin>,
    pub d_rep_inactivity_period: Option<Epoch>,
    pub min_fee_ref_script_cost_per_byte: Option<Rational>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    #[serde(skip)]
    pub encodings: Option<ProtocolParamUpdateEncoding>,
}

impl ProtocolParamUpdate {
    pub fn new() -> Self {
        Self {
            minfee_a: None,
            minfee_b: None,
            max_block_body_size: None,
            max_transaction_size: None,
            max_block_header_size: None,
            key_deposit: None,
            pool_deposit: None,
            maximum_epoch: None,
            n_opt: None,
            pool_pledge_influence: None,
            expansion_rate: None,
            treasury_growth_rate: None,
            min_pool_cost: None,
            ada_per_utxo_byte: None,
            cost_models_for_script_languages: None,
            execution_costs: None,
            max_tx_ex_units: None,
            max_block_ex_units: None,
            max_value_size: None,
            collateral_percentage: None,
            max_collateral_inputs: None,
            pool_voting_thresholds: None,
            d_rep_voting_thresholds: None,
            min_committee_size: None,
            committee_term_limit: None,
            governance_action_validity_period: None,
            governance_action_deposit: None,
            d_rep_deposit: None,
            d_rep_inactivity_period: None,
            min_fee_ref_script_cost_per_byte: None,
            encodings: None,
        }
    }
}

impl Default for ProtocolParamUpdate {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Rational {
    pub numerator: u64,
    pub denominator: u64,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    #[serde(skip)]
    pub encodings: Option<RationalEncoding>,
}

impl Rational {
    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            numerator,
            denominator,
            encodings: None,
        }
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SetCommitteeColdCredential {
    pub(crate) inner: OrderedSet<CommitteeColdCredential>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<SetCommitteeColdCredentialEncoding>,
}

impl SetCommitteeColdCredential {
    pub fn new(inner: OrderedSet<CommitteeColdCredential>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<OrderedSet<CommitteeColdCredential>> for SetCommitteeColdCredential {
    fn from(inner: OrderedSet<CommitteeColdCredential>) -> Self {
        SetCommitteeColdCredential::new(inner)
    }
}

impl From<SetCommitteeColdCredential> for OrderedSet<CommitteeColdCredential> {
    fn from(wrapper: SetCommitteeColdCredential) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for SetCommitteeColdCredential {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for SetCommitteeColdCredential {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner = <OrderedSet<CommitteeColdCredential> as serde::de::Deserialize>::deserialize(
            deserializer,
        )?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for SetCommitteeColdCredential {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("SetCommitteeColdCredential")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <OrderedSet<CommitteeColdCredential> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <OrderedSet<CommitteeColdCredential> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SetEd25519KeyHash {
    pub(crate) inner: OrderedSet<Ed25519KeyHash>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<SetEd25519KeyHashEncoding>,
}

impl SetEd25519KeyHash {
    pub fn new(inner: OrderedSet<Ed25519KeyHash>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<OrderedSet<Ed25519KeyHash>> for SetEd25519KeyHash {
    fn from(inner: OrderedSet<Ed25519KeyHash>) -> Self {
        SetEd25519KeyHash::new(inner)
    }
}

impl From<SetEd25519KeyHash> for OrderedSet<Ed25519KeyHash> {
    fn from(wrapper: SetEd25519KeyHash) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for SetEd25519KeyHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for SetEd25519KeyHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner =
            <OrderedSet<Ed25519KeyHash> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for SetEd25519KeyHash {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("SetEd25519KeyHash")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <OrderedSet<Ed25519KeyHash> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <OrderedSet<Ed25519KeyHash> as schemars::JsonSchema>::inline_schema()
    }
}

#[derive(Clone, Debug, derivative::Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SetTransactionInput {
    pub(crate) inner: OrderedSet<TransactionInput>,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    pub encodings: Option<SetTransactionInputEncoding>,
}

impl SetTransactionInput {
    pub fn new(inner: OrderedSet<TransactionInput>) -> Self {
        Self {
            inner,
            encodings: None,
        }
    }
}

impl From<OrderedSet<TransactionInput>> for SetTransactionInput {
    fn from(inner: OrderedSet<TransactionInput>) -> Self {
        SetTransactionInput::new(inner)
    }
}

impl From<SetTransactionInput> for OrderedSet<TransactionInput> {
    fn from(wrapper: SetTransactionInput) -> Self {
        wrapper.inner
    }
}

impl serde::Serialize for SetTransactionInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::de::Deserialize<'de> for SetTransactionInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let inner =
            <OrderedSet<TransactionInput> as serde::de::Deserialize>::deserialize(deserializer)?;
        Ok(Self::new(inner))
    }
}

impl schemars::JsonSchema for SetTransactionInput {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("SetTransactionInput")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <OrderedSet<TransactionInput> as schemars::JsonSchema>::json_schema(generator)
    }

    fn inline_schema() -> bool {
        <OrderedSet<TransactionInput> as schemars::JsonSchema>::inline_schema()
    }
}

pub type Slot = u64;

pub type SubCoin = Rational;

pub type TransactionIndex = u16;

pub type TransactionMetadatumLabel = u64;

#[derive(
    Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema, derivative::Derivative,
)]
#[derivative(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnitInterval {
    pub start: u64,
    pub end: u64,
    #[derivative(
        PartialEq = "ignore",
        Ord = "ignore",
        PartialOrd = "ignore",
        Hash = "ignore"
    )]
    #[serde(skip)]
    pub encodings: Option<UnitIntervalEncoding>,
}

impl UnitInterval {
    pub fn new(start: u64, end: u64) -> Self {
        Self {
            start,
            end,
            encodings: None,
        }
    }
}

pub type Withdrawals = OrderedHashMap<RewardAccount, Coin>;

impl From<NetworkId> for u64 {
    fn from(wrapper: NetworkId) -> Self {
        wrapper.inner
    }
}
