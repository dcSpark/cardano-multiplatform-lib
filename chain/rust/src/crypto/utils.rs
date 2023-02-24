//impl BootstrapWitness {
// pub fn to_public_key(&self) -> Result<crypto::Bip32PublicKey, crypto::CryptoError> {
//     crypto::chain_crypto::PublicKey::<chain_crypto::ed25519_derive::Ed25519Bip32>::try_from(self.clone())
//         .map(crypto::Bip32PublicKey)
//         .map_err(Into::into)
// }

// pub fn to_address(&self) -> Result<AddressContent, CryptoError> {
//     AddressContent::try_from(self.clone())
//         .map_err(Into::into)
// }
//}

// impl TryFrom<BootstrapWitness> for chain_crypto::PublicKey<chain_crypto::ed25519_derive::Ed25519Bip32> {
//     type Error = ed25519_bip32::PublicKeyError;

//     fn try_from(wit: BootstrapWitness) -> Result<Self, Self::Error> {
//         combine_pk_and_chaincode(wit.vkey.pubkey.0, &wit.chain_code)
//     }
// }

// impl TryFrom<BootstrapWitness> for AddressContent {
//     type Error = ed25519_bip32::PublicKeyError;

//     fn try_from(wit: BootstrapWitness) -> Result<Self, Self::Error> {
//         let protocol_magic = wit.attributes.protocol_magic;
//         let key = chain_crypto::PublicKey::<chain_crypto::ed25519_derive::Ed25519Bip32>::try_from(wit)?;
//         let address_content = AddressContent::new_simple(&Bip32PublicKey(key), protocol_magic);
//         Ok(address_content)
//     }
// }
