use crate::chain_crypto::algorithms::{
    ed25519::Pub, ed25519_derive::Ed25519Bip32, ed25519_extended::ExtendedPriv, Ed25519,
};
use crate::chain_crypto::key::{PublicKey, SecretKey};
use crate::chain_crypto::Ed25519Extended;
use cryptoxide::hmac::Hmac;
use cryptoxide::pbkdf2::pbkdf2;
use cryptoxide::sha2::Sha512;
use ed25519_bip32::{DerivationError, DerivationScheme, PublicKeyError};
use ed25519_bip32::{XPrv, XPRV_SIZE};

pub fn derive_sk_ed25519(key: &SecretKey<Ed25519Bip32>, index: u32) -> SecretKey<Ed25519Bip32> {
    let new_key = key.0.derive(DerivationScheme::V2, index);
    SecretKey(new_key)
}

pub fn derive_pk_ed25519(
    key: &PublicKey<Ed25519Bip32>,
    index: u32,
) -> Result<PublicKey<Ed25519Bip32>, DerivationError> {
    key.0.derive(DerivationScheme::V2, index).map(PublicKey)
}

pub fn to_raw_sk(key: &SecretKey<Ed25519Bip32>) -> SecretKey<Ed25519Extended> {
    SecretKey(ExtendedPriv::from_xprv(&key.0))
}

pub fn to_raw_pk(key: &PublicKey<Ed25519Bip32>) -> PublicKey<Ed25519> {
    PublicKey(Pub::from_xpub(&key.0))
}

pub fn combine_pk_and_chaincode(
    key: PublicKey<super::ed25519::Ed25519>,
    chaincode: &[u8],
) -> Result<PublicKey<Ed25519Bip32>, PublicKeyError> {
    let mut buf = [0; ed25519_bip32::XPUB_SIZE];
    buf[0..cryptoxide::ed25519::PUBLIC_KEY_LENGTH].clone_from_slice(key.as_ref());
    buf[cryptoxide::ed25519::PUBLIC_KEY_LENGTH..ed25519_bip32::XPUB_SIZE]
        .clone_from_slice(&chaincode);
    let xpub = ed25519_bip32::XPub::from_slice(&buf)?;
    Ok(PublicKey(xpub))
}

pub fn from_bip39_entropy(entropy: &[u8], password: &[u8]) -> SecretKey<Ed25519Bip32> {
    let mut pbkdf2_result = [0; XPRV_SIZE];

    const ITER: u32 = 4096;
    let mut mac = Hmac::new(Sha512::new(), password);
    pbkdf2(&mut mac, entropy, ITER, &mut pbkdf2_result);

    SecretKey(XPrv::normalize_bytes_force3rd(pbkdf2_result))
}
