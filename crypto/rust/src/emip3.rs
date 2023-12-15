use cryptoxide::chacha20poly1305::ChaCha20Poly1305;
use cryptoxide::hmac::Hmac;
use cryptoxide::pbkdf2::pbkdf2;
use cryptoxide::sha2::Sha512;
use hex::ToHex;

use std::iter::repeat;

// taken from js-cardano-wasm

mod password_encryption_parameter {
    pub const ITER: u32 = 19_162;
    pub const SALT_SIZE: usize = 32;
    pub const NONCE_SIZE: usize = 12;
    pub const KEY_SIZE: usize = 32;
    pub const TAG_SIZE: usize = 16;

    pub const METADATA_SIZE: usize = SALT_SIZE + NONCE_SIZE + TAG_SIZE;

    pub const SALT_START: usize = 0;
    pub const SALT_END: usize = SALT_START + SALT_SIZE;
    pub const NONCE_START: usize = SALT_END;
    pub const NONCE_END: usize = NONCE_START + NONCE_SIZE;
    pub const TAG_START: usize = NONCE_END;
    pub const TAG_END: usize = TAG_START + TAG_SIZE;
    pub const ENCRYPTED_START: usize = TAG_END;
}

#[derive(Debug, thiserror::Error)]
pub enum EmIP3Error {
    #[error("Invalid hex: {0}")]
    InvalidHex(#[from] hex::FromHexError),
    #[error("Salt len must be {0}, found {1} bytes")]
    SaltLen(usize, usize),
    #[error("Nonce len must be {0}, found {1} bytes")]
    NonceLen(usize, usize),
    #[error("Password len cannot be 0")]
    EmptyPassword,
    #[error("Missing input data: Needed {0}, found {1} bytes")]
    MissingInputData(usize, usize),
    #[error("Decryption failed")]
    DecryptionFailed,
}

/// Encrypt using Emip3: https://github.com/Emurgo/EmIPs/blob/master/specs/emip-003.md
pub fn emip3_encrypt_with_password(
    password: &str,
    salt: &str,
    nonce: &str,
    data: &str,
) -> Result<String, EmIP3Error> {
    use password_encryption_parameter::*;

    let password = hex::decode(password)?;
    let salt = hex::decode(salt)?;
    let nonce = hex::decode(nonce)?;
    let data = hex::decode(data)?;

    if salt.len() != SALT_SIZE {
        return Err(EmIP3Error::SaltLen(SALT_SIZE, salt.len()));
    }
    if nonce.len() != NONCE_SIZE {
        return Err(EmIP3Error::NonceLen(NONCE_SIZE, nonce.len()));
    }
    if password.is_empty() {
        return Err(EmIP3Error::EmptyPassword);
    }

    let key = {
        let mut mac = Hmac::new(Sha512::new(), &password);
        let mut key: Vec<u8> = repeat(0).take(KEY_SIZE).collect();
        pbkdf2(&mut mac, &salt[..], ITER, &mut key);
        key
    };

    let mut tag = [0; TAG_SIZE];
    let mut encrypted: Vec<u8> = repeat(0).take(data.len()).collect();
    {
        ChaCha20Poly1305::new(&key, &nonce, &[]).encrypt(&data, &mut encrypted, &mut tag);
    }

    let mut output = Vec::with_capacity(data.len() + METADATA_SIZE);
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce);
    output.extend_from_slice(&tag);
    output.extend_from_slice(&encrypted);

    Ok(output.encode_hex::<String>())
}

/// Decrypt using Emip3: https://github.com/Emurgo/EmIPs/blob/master/specs/emip-003.md
pub fn emip3_decrypt_with_password(password: &str, data: &str) -> Result<String, EmIP3Error> {
    use password_encryption_parameter::*;
    let password = hex::decode(password)?;
    let data = hex::decode(data)?;

    if data.len() <= METADATA_SIZE {
        // not enough input to decrypt.
        return Err(EmIP3Error::MissingInputData(METADATA_SIZE, data.len()));
    }

    let salt = &data[SALT_START..SALT_END];
    let nonce = &data[NONCE_START..NONCE_END];
    let tag = &data[TAG_START..TAG_END];
    let encrypted = &data[ENCRYPTED_START..];

    let key = {
        let mut mac = Hmac::new(Sha512::new(), &password);
        let mut key: Vec<u8> = repeat(0).take(KEY_SIZE).collect();
        pbkdf2(&mut mac, salt, ITER, &mut key);
        key
    };

    let mut decrypted: Vec<u8> = repeat(0).take(encrypted.len()).collect();
    let decryption_succeed =
        { ChaCha20Poly1305::new(&key, nonce, &[]).decrypt(encrypted, &mut decrypted, tag) };

    if decryption_succeed {
        Ok(decrypted.encode_hex::<String>())
    } else {
        Err(EmIP3Error::DecryptionFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encryption() {
        let password = "70617373776f7264";
        let salt = "50515253c0c1c2c3c4c5c6c750515253c0c1c2c3c4c5c6c750515253c0c1c2c3";
        let nonce = "50515253c0c1c2c3c4c5c6c7";
        let data = "736f6d65206461746120746f20656e6372797074";
        let encrypted_data = emip3_encrypt_with_password(password, salt, nonce, data).unwrap();
        let decrypted_data = emip3_decrypt_with_password(password, &encrypted_data).unwrap();
        assert_eq!(data, decrypted_data);
    }
}
