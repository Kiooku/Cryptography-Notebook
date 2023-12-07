use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng, Payload, generic_array::GenericArray},
    Aes256GcmSiv, AeadCore,
};
use x25519_dalek::PublicKey;

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
/// * `plaintext` (Result\<Vec\<u8\>, CryptoError\>): Plaintext
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

/// Returns the AEAD encryption of plaintext with header key `hk`.
/// 
/// # Arguments
/// 
/// * `hk` (\[u8; 32\]): Header Keys
/// * `header` ((PublicKey, u8, u8)): Header
/// 
/// # Output
/// 
/// * `(encrypted_header, nonce)` (Result\<(Vec\<u8\>, Vec\<u8\>), CryptoError\>): Encrypted Header and Nonce used
pub fn hencrypt(hk: [u8; 32], header: (PublicKey, u8, u8)) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
    let cipher = Aes256GcmSiv::new(&GenericArray::clone_from_slice(&hk));    
    let nonce = &Aes256GcmSiv::generate_nonce(&mut OsRng);

    let serialized_header: Vec<u8> = {
        let public_key_bytes = header.0.as_bytes();
        let mut serialized = Vec::with_capacity(public_key_bytes.len() + 2);
        serialized.extend_from_slice(public_key_bytes);
        serialized.push(header.1);
        serialized.push(header.2);
        serialized
    };

    let ciphertext = cipher
        .encrypt(nonce, serialized_header.as_ref())
        .map_err(|_| CryptoError::EncryptionError)?;

    Ok((ciphertext, nonce.to_vec()))
}

/// Returns the authenticated decryption of ciphertext with header key `hk`.
/// 
/// # Arguments
/// 
/// * `hk` (\[u8; 32\]): Header Keys
/// * `ciphertext` (&Vec\<u8\>): Ciphertext
/// * `nonce` (&Vec\<u8\>): Nonce
/// 
/// # Output
/// 
/// * `header decrypted` (Result\<(PublicKey, u8, u8), CryptoError\>): Header
pub fn hdecrypt(hk: [u8; 32], ciphertext: &Vec<u8>, nonce: &Vec<u8>) -> Option<(PublicKey, u8, u8)> {
    let cipher = Aes256GcmSiv::new(&GenericArray::clone_from_slice(&hk));

    let decrypted_header = cipher
        .decrypt(&GenericArray::clone_from_slice(&nonce), ciphertext.as_ref())
        .ok();

    if decrypted_header.is_some() {
        let public_key_bytes: [u8; 32] = decrypted_header.clone().unwrap()[0..32].try_into().ok()?;
        let public_key: PublicKey = PublicKey::from(public_key_bytes);
        let pn: u8 = decrypted_header.clone().unwrap()[32];
        let n: u8 = decrypted_header.clone().unwrap()[33];
        return Some((public_key, pn, n))
    }

    None
}