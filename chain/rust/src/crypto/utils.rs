use std::convert::{TryFrom, TryInto};

use crate::byron::AddressContent;
use super::BootstrapWitness;

use cml_crypto::{chain_crypto::{self, derive::combine_pk_and_chaincode}, CryptoError};

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

impl TryInto<chain_crypto::PublicKey<chain_crypto::ed25519_derive::Ed25519Bip32>> for BootstrapWitness {
    type Error = CryptoError;

    fn try_into(self) ->  Result<chain_crypto::PublicKey<chain_crypto::ed25519_derive::Ed25519Bip32>, Self::Error> {
        combine_pk_and_chaincode(self.public_key.0, &self.chain_code)
            .map_err(Into::into)
    }
}

impl TryFrom<BootstrapWitness> for AddressContent {
    type Error = CryptoError;

    fn try_from(wit: BootstrapWitness) -> Result<Self, Self::Error> {
        // let protocol_magic = wit.attributes.protocol_magic;
        // let key = chain_crypto::PublicKey::<chain_crypto::ed25519_derive::Ed25519Bip32>::try_from(wit)?;
        // let address_content = AddressContent::new_simple(Bip32PublicKey(key), protocol_magic);
        // Ok(address_content)
        todo!("redo byron address attributes to not be just a bytestring")
    }
}
