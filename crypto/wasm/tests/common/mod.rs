//! Shared keygen runtime assertions, used by the native/node and browser test targets.
//! (Lives under `common/` so cargo does not treat it as its own test target.)

use cml_crypto_wasm::{Bip32PrivateKey, PrivateKey};

/// Generating two ed25519 keys must yield distinct, non-zero output — i.e. the RNG
/// (rand -> getrandom) actually produced entropy on whatever target this runs on.
pub fn ed25519_keygen_is_random() {
    let a = PrivateKey::generate_ed25519().to_raw_bytes();
    let b = PrivateKey::generate_ed25519().to_raw_bytes();
    assert_eq!(a.len(), 32, "ed25519 raw key must be 32 bytes");
    assert!(a.iter().any(|&x| x != 0), "key must not be all-zero (RNG failed)");
    assert_ne!(a, b, "two generated keys must differ (no real entropy?)");
}

pub fn bip32_keygen_is_random() {
    let a = Bip32PrivateKey::generate_ed25519_bip32().to_raw_bytes();
    let b = Bip32PrivateKey::generate_ed25519_bip32().to_raw_bytes();
    assert!(a.len() >= 64, "bip32 xprv should be at least 64 bytes");
    assert!(a.iter().any(|&x| x != 0), "key must not be all-zero (RNG failed)");
    assert_ne!(a, b, "two generated bip32 keys must differ");
}
