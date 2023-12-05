use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng, Payload, generic_array::GenericArray},
    Aes256GcmSiv, AeadCore,
};

#[derive(Debug)]
pub enum CryptoError {
    EncryptionError,
    DecryptionError,
}

/// Encrypt the message using AES-GCM-SIV-256
/// 
/// # Arguments
/// 
/// * `mk` (\[u8; 32\]): Message key
/// * `plaintext` (&\[u8\]): Plaintext
/// * `ad` (&\[u8\]): Associated Data
/// 
/// # Output
/// 
/// * `(ciphertext, nonce)` (Result\<(Vec\<u8\>, Vec\<u8\>), CryptoError\>): Ciphertext and Nonce used
pub fn encrypt(mk: [u8; 32], plaintext: &[u8], ad: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    let cipher = Aes256GcmSiv::new(&GenericArray::clone_from_slice(&mk));    
    let nonce = &Aes256GcmSiv::generate_nonce(&mut OsRng);
    let payload = Payload {
        msg: plaintext,
        aad: ad,
    };
     
    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|_| CryptoError::EncryptionError)?;

    Ok((ciphertext, nonce.to_vec()))
}

/// Decrypt the message using AES-GCM-SIV-256
/// 
/// # Arguments
/// 
/// * `mk` (\[u8; 32\]): Message key
/// * `ciphertext` (&Vec\<u8\>): Ciphertext
/// * `nonce` (&Vec\<u8\>): Nonce
/// * `ad` (&\[u8\]): Associated Data
/// 
/// # Output
/// 
/// * `(ciphertext, nonce)` (Result\<Vec\<u8\>, CryptoError\>): Plaintext
pub fn decrypt(mk: [u8; 32], ciphertext: &Vec<u8>, nonce: &Vec<u8>, ad: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let cipher = Aes256GcmSiv::new(&GenericArray::clone_from_slice(&mk));
    let payload = Payload {
        msg: &ciphertext,
        aad: ad,
    };

    let plaintext = cipher
        .decrypt(&GenericArray::clone_from_slice(&nonce), payload)
        .map_err(|_| CryptoError::DecryptionError)?;

    Ok(plaintext)
}