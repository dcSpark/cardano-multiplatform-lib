//! Module provides cryptographic utilities and types related to
//! the user keys.
//!
use crate::chain_crypto as crypto;
use crate::chain_crypto::SecretKey;
use rand::{CryptoRng, Rng};

#[derive(Clone)]
pub enum EitherEd25519SecretKey {
    Extended(crypto::SecretKey<crypto::Ed25519Extended>),
    Normal(crypto::SecretKey<crypto::Ed25519>),
}

impl EitherEd25519SecretKey {
    /// # Security
    /// `rng` MUST be a CSPRNG seeded from OS entropy; a deterministically-seeded RNG yields
    /// predictable, INSECURE keys (test-only). See [`crate::chain_crypto::AsymmetricKey::generate`]
    /// and prefer the high-level `cml_crypto::PrivateKey` generators for production keys.
    pub fn generate<R: Rng + CryptoRng>(rng: R) -> Self {
        EitherEd25519SecretKey::Extended(SecretKey::generate(rng))
    }

    pub fn to_public(&self) -> crypto::PublicKey<crypto::Ed25519> {
        match self {
            EitherEd25519SecretKey::Extended(sk) => sk.to_public(),
            EitherEd25519SecretKey::Normal(sk) => sk.to_public(),
        }
    }

    pub fn sign<T: AsRef<[u8]>>(&self, dat: &T) -> crypto::Signature<T, crypto::Ed25519> {
        match self {
            EitherEd25519SecretKey::Extended(sk) => sk.sign(dat),
            EitherEd25519SecretKey::Normal(sk) => sk.sign(dat),
        }
    }

    pub fn sign_slice<T: ?Sized>(&self, dat: &[u8]) -> crypto::Signature<T, crypto::Ed25519> {
        match self {
            EitherEd25519SecretKey::Extended(sk) => sk.sign_slice(dat),
            EitherEd25519SecretKey::Normal(sk) => sk.sign_slice(dat),
        }
    }
}

pub type SpendingPublicKey = crypto::PublicKey<crypto::Ed25519>;
pub type SpendingSignature<T> = crypto::Signature<T, crypto::Ed25519>;

pub type Ed25519Signature<T> = crypto::Signature<T, crypto::Ed25519>;
