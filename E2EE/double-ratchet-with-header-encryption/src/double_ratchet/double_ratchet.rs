use crate::double_ratchet::state::State;
use crate::double_ratchet::aead::{encrypt as aead_encrypt, decrypt as aead_decrypt, hencrypt, hdecrypt};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use hkdf::Hkdf;
use rand_core::OsRng;
use x25519_dalek::{ReusableSecret, PublicKey};

use super::aead::CryptoError;


const MAX_SKIP: u16 = 1000;
const BYTE_MESSAGE_KEY: &[u8] = &[0x01];
const BYTE_NEXT_CHAIN_KEY: &[u8] = &[0x02];
const INFO: &[u8] = &[0x73];
type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct DoubleRatchetHE {
    state: State,
}

impl DoubleRatchetHE {
    pub fn new() -> Self {
        DoubleRatchetHE { state: State::new() }
    }

    /// Initialize the sender Double Ratchet
    /// 
    /// # Arguments
    /// 
    /// * `sk` (\[u8; 32\]): Shared Key *(X3DH shared secret)*
    /// * `receiver_public_key` (PublicKey): Receiver public key
    /// * `shared_hk` (\[u8; 32\]): Shared Header Keys *(HKDF derivation of the shared secret)*
    /// * `shared_nhk` (\[u8; 32\]): Shared Next Header Keys *(HKDF derivation of the shared secret)*
    pub fn init_sender_he(&mut self, sk: [u8; 32], receiver_public_key: PublicKey, shared_hk: [u8; 32], shared_nhk: [u8; 32]) -> () {
        self.generate_dh(); // Set dh_s
        self.state.dh_r = Some(receiver_public_key);
        let (rk_result, ck_r_result, nhk_s_result) = self.kdf_rk_he(sk, self.dh(self.state.dh_s.as_ref().unwrap(), self.state.dh_r.unwrap()));
        (self.state.rk, self.state.ck_s, self.state.nhk_s) = (Some(rk_result), Some(ck_r_result), Some(nhk_s_result));
        self.state.hk_s = Some(shared_hk);
        self.state.nhk_r = Some(shared_nhk);
    }

    /// Initialize the receiver Double Ratchet
    /// 
    /// # Arguments
    /// 
    /// * `sk` (\[u8; 32\]): Shared Key *(X3DH shared secret)*
    /// * `receiver_pair` (ReusableSecret, PublicKey): Receiver pair
    /// * `shared_hk` (\[u8; 32\]): Shared Header Keys *(HKDF derivation of the shared secret, info different from shared_nhk)*
    /// * `shared_nhk` (\[u8; 32\]): Shared Next Header Keys *(HKDF derivation of the shared secret, info different from shared_hk)*
    pub fn init_receiver_he(&mut self, sk: [u8; 32], receiver_pair: (ReusableSecret, PublicKey), shared_hk: [u8; 32], shared_nhk: [u8; 32]) -> () {
        self.state.dh_s = Some(receiver_pair);
        self.state.rk = Some(sk);
        self.state.nhk_s = Some(shared_nhk);
        self.state.nhk_r = Some(shared_hk);
    }
    
    /// Create and set a new Diffie-Hellman *(Curve25519)* key pair to `dh_s`
    fn generate_dh(&mut self) -> () {
        let private_key: ReusableSecret = ReusableSecret::random_from_rng(OsRng);
        let public_key: PublicKey = PublicKey::from(&private_key);
        self.state.dh_s = Some((private_key, public_key));
    }
    
    /// Returns the output from the Diffie-Hellman calculation between the private key from the DH key pair `dh_pair` and the DH public key `dh_pub`.
    /// 
    /// # Arguments
    /// 
    /// * `dh_pair` ((EphemeralSecret, PublicKey)): Diffie-Hellman key pair
    /// * `dh_pub` (PublicKey): Diffie-Hellman public key
    /// 
    /// # Output
    /// 
    /// * `dh_out` (SharedSecret): Diffie-Hellman output
    fn dh(&self, dh_pair: &(ReusableSecret, PublicKey), dh_pub: PublicKey) -> [u8; 32] {
        *dh_pair.0.diffie_hellman(&dh_pub).as_bytes()
    }

    /// Returns a new **root key**, **chain key**, and **next header key** as the output of applying a KDF keyed by root key `rk` to a Diffie-Hellman output `dh_out`.
    /// 
    /// # Arguments
    /// 
    /// * `rk` (\[u8; 32\]): 32-byte root key
    /// * `dh_out` (\[u8; 32\]): Diffie-Hellman output
    /// 
    /// # Output
    /// 
    /// * `rk` (\[u8; 32\]): 32-byte root key
    /// * `ck` (\[u8; 32\]): 32-byte chain key
    /// * `nhk` (\[u8; 32\]): 32-byte next header Keys
    fn kdf_rk_he(&self, rk: [u8; 32], dh_out: [u8; 32]) -> ([u8; 32], [u8; 32], [u8;32]) {
        let ikm = dh_out;
        let salt = rk;

        let hk = Hkdf::<Sha256>::new(Some(&salt[..]), &ikm);
        let mut okm = [0u8; 96];
        hk.expand(INFO, &mut okm)
            .expect("Output length invalid KDF_RK");

        let (new_rk, temp) = okm.split_at(32);
        let (new_ck, new_nhk) = temp.split_at(32);
        
        (new_rk.try_into()
            .expect("Incorrect length"),
        new_ck.try_into()
            .expect("Incorrect length"),
        new_nhk.try_into()
            .expect("Incorrect lenght"))
    }

    /// Returns the output of applying a KDF keyed by a 32-byte chain key `ck` to some constant.
    /// 
    /// # Arguments
    /// 
    /// * `ck` (\[u8; 32\]): 32-byte chain key
    /// 
    /// # Output
    /// 
    /// * `ck` (\[u8; 32\]): 32-byte chain key
    /// * `mk` (\[u8; 32\]): 32-byte message key
    fn kdf_ck(&self, ck: [u8; 32]) -> (Option<[u8; 32]>, [u8; 32]) {
        // HMAC for the chain key
        let mut mac_ck = HmacSha256::new_from_slice(&ck)
            .expect("HMAC can take key of any size");
        mac_ck.update(BYTE_NEXT_CHAIN_KEY);

        let new_chain_key = mac_ck.finalize().into_bytes().as_slice().try_into().expect("slice to array conversion failed");
        
        // HMAC for the message key
        let mut mac_mk = HmacSha256::new_from_slice(&ck)
            .expect("HMAC can take key of any size");
        mac_mk.update(BYTE_MESSAGE_KEY);

        let new_message_key = mac_mk.finalize().into_bytes().as_slice().try_into().expect("slice to array conversion failed");

        (Some(new_chain_key), new_message_key)
    }
    
    /// Returns an AEAD (AES-GCM-SIV-256) encryption of plaintext with message key `mk`.
    /// 
    /// # Arguments
    /// 
    /// * `plaintext` (&\[u8\]): Plaintext
    /// * `ad` (&\[u8\]): Associated Data
    /// 
    /// # Output
    /// 
    /// * `(enc_header, res)` ((Vec<u8>, Vec<u8>), (Vec\<u8\>, Vec\<u8\>)): Encrypted header and ciphertext
    pub fn encrypt_he(&mut self, plaintext: &[u8], ad: &[u8]) -> ((Vec<u8>, Vec<u8>), (Vec<u8>, Vec<u8>)) {
        let mk: [u8; 32];
        (self.state.ck_s, mk) = self.kdf_ck(self.state.ck_s.unwrap());
        let header: (PublicKey, u8, u8) = self.header(self.state.dh_s.as_ref().unwrap(), self.state.pn, self.state.n_s);
        let enc_header: (Vec<u8>, Vec<u8>) = match hencrypt(self.state.hk_s.unwrap(), header) {
            Ok((encrypted_header, header_nonce)) => (encrypted_header, header_nonce),
            Err(error) => panic!("Error header (AES-GCM-SIV): {:?}", error),
        };
        self.state.n_s += 1;
        let res = match aead_encrypt(mk, plaintext, &self.concat(ad, header)) {
            Ok((ciphertext, nonce)) => (ciphertext, nonce),
            Err(error) => panic!("Error (AES-GCM-SIV): {:?}", error),
        };
        (enc_header, res)
    }
    
    /// Returns the AEAD (AES-GCM-SIV-256) decryption of ciphertext with message key mk.
    /// 
    /// # Arguments
    /// 
    /// * `enc_header` ((Vec<u8>, Vec<u8>)): Encrypted Header
    /// * `ciphertext` (&\[u8\]): Ciphertext
    /// * `nonce` (Vec\<u8\>): Nonce
    /// * `ad` (&\[u8\]): Associated Data
    /// 
    /// # Output
    /// 
    /// * `plaintext` (Vec\<u8\>): Plaintext
    pub fn decrypt_he(&mut self, enc_header: (Vec<u8>, Vec<u8>), ciphertext: Vec<u8>, nonce: Vec<u8>, ad: &[u8]) -> Vec<u8> {
        let plaintext: Option<Vec<u8>> = self.try_skipped_message_keys_he(enc_header.clone(), &ciphertext, &nonce, ad);
        if plaintext.is_some() {
            return plaintext.unwrap()
        }
        let (header, dh_ratchet): ((PublicKey, u8, u8), bool) = match self.decrypt_header(enc_header.clone()) {
            Ok((current_header, current_dh_ratchet)) => (current_header, current_dh_ratchet),
            Err(error) => panic!("Error header (AES-GCM-SIV): {:?}", error),
        };
        if dh_ratchet {
            self.skip_message_keys_he(header.1);
            self.state.pn = self.state.n_s;
            (self.state.n_s, self.state.n_r) = (0, 0);
            self.state.hk_s = self.state.nhk_s;
            self.state.hk_r = self.state.nhk_r;
            self.state.dh_r = Some(header.0);
            let (rk_result, ck_r_result, nhk_r_result) = self.kdf_rk_he(self.state.rk.unwrap(), self.dh(self.state.dh_s.as_ref().unwrap(), self.state.dh_r.unwrap()));
            (self.state.rk, self.state.ck_r, self.state.nhk_r) = (Some(rk_result), Some(ck_r_result), Some(nhk_r_result));
            self.generate_dh(); // New dh_s
            let (rk_result, ck_s_result, nhk_s_result) = self.kdf_rk_he(self.state.rk.unwrap(), self.dh(self.state.dh_s.as_ref().unwrap(), self.state.dh_r.unwrap()));
            (self.state.rk, self.state.ck_s, self.state.nhk_s) = (Some(rk_result), Some(ck_s_result), Some(nhk_s_result));
        }
        self.skip_message_keys_he(header.2);
        let mk: [u8; 32];
        (self.state.ck_r, mk) = self.kdf_ck(self.state.ck_r.unwrap());
        self.state.n_r += 1;
        
        let res = match aead_decrypt(mk, &ciphertext, &nonce, &self.concat(ad, header)) {
            Ok(plaintext) => plaintext,
            Err(error) => panic!("Error (AES-GCM-SIV): {:?}", error),
        };
        res
    }
    
    /// Check if the message corresponds to a skipped message key. 
    /// 
    /// If it's a skipped message, this function decrypts the message, deletes the message key, and return the plaintext.
    /// 
    /// # Arguments
    /// * `enc_header` ((Vec<u8>, Vec<u8>)): Encrypted Header
    /// * `ciphertext` (&Vec\<u8\>): Ciphertext
    /// * `nonce` (&Vec\<u8\>): Nonce
    /// * `ad` (&\[u8\]): Associated Data
    /// 
    /// # Output
    /// 
    /// `plaintext` (Option\<Vec\<u8\>\>): Optional plaintext
    fn try_skipped_message_keys_he(&mut self, enc_header: (Vec<u8>, Vec<u8>), ciphertext: &Vec<u8>, nonce: &Vec<u8>,  ad: &[u8]) -> Option<Vec<u8>> {
        for ((hk, n), mk) in self.state.mkskipped.clone().iter() {
            let header: Option<(PublicKey, u8, u8)> = hdecrypt(*hk, &enc_header.0, &enc_header.1);
            if header.is_some() && header.unwrap().2 == *n {
                self.state.mkskipped.remove(&(*hk, *n));
                let res = match aead_decrypt(*mk, ciphertext, &nonce, &self.concat(ad, header.unwrap())) {
                    Ok(plaintext) => plaintext,
                    Err(error) => panic!("Error (AES-GCM-SIV): {:?}", error),
                };
                return Some(res)
            }
        }
        None
    }

    /// Decrypt the header and define if we need to applies a DH ratchet step
    /// 
    /// # Arguments
    /// * `enc_header` ((Vec<u8>, Vec<u8>)): Encrypted Header
    /// 
    /// # Output
    /// 
    /// `(header, dh_ratchet)` (Result\<((PublicKey, u8, u8), bool), CryptoError\>): Header and boolean to tell if we need to applies a DH ratchet step
    fn decrypt_header(&self, enc_header: (Vec<u8>, Vec<u8>)) -> Result<((PublicKey, u8, u8), bool), CryptoError> {
        let mut header: Option<(PublicKey, u8, u8)> = hdecrypt(self.state.hk_r.unwrap_or_default(), &enc_header.0, &enc_header.1);
        if header.is_some() {
            return Ok((header.unwrap(), false))
        }
        header = hdecrypt(self.state.nhk_r.unwrap(), &enc_header.0, &enc_header.1);
        if header.is_some() {
            return Ok((header.unwrap(), true))
        }
        Err(CryptoError::DecryptionError)
    }
    
    /// Stores any skipped message keys from the current receiving chain.
    /// 
    /// # Arguments
    /// * `until` (u8)
    fn skip_message_keys_he(&mut self, until: u8) -> () {
        if self.state.n_r as u16 + MAX_SKIP < until as u16 {
            panic!("Error: state.Nr + MAX_SKIP < until");
        }
        if self.state.ck_r != None {
            while self.state.n_r < until {
                let mk: [u8; 32];
                (self.state.ck_r, mk) = self.kdf_ck(self.state.ck_r.unwrap());
                self.state.mkskipped.insert((self.state.hk_r.unwrap(), self.state.n_r), mk);
                self.state.n_r += 1;
            }
        }
    }
    
    /// Returns the output of applying a KDF keyed by a 32-byte chain key `ck` to some constant.
    /// 
    /// # Arguments
    /// 
    /// * `dh_pair` (&(ReusableSecret, PublicKey)): Diffie-Hellman key pair
    /// * `pn` (u8): Number of messages in previous sending chain
    /// * `n` (u8): Message numbers for sending and receiving
    /// 
    /// # Output
    /// 
    /// * `header` ((PublicKey, u8, u8)): Header
    fn header(&self, dh_pair: &(ReusableSecret, PublicKey), pn: u8, n: u8) -> (PublicKey, u8, u8) {
        (dh_pair.1, pn, n)
    }
     
    /// Return the concatenation of the Associated data and the Header
    /// 
    /// # Arguments
    /// 
    /// * `ad` (&\[u8\]): Associated Data
    /// * `header` ((PublicKey, u8, u8)): Header
    /// 
    /// # Output
    /// 
    /// * `res` (Vec\<u8\>): Concatenation
    fn concat(&self, ad: &[u8], header: (PublicKey, u8, u8)) -> Vec<u8> {
        let public_key: &[u8; 32] = header.0.as_bytes();
        let nb_messages_previous_chain: u8 = header.1;
        let message_number: u8 = header.2;

        [ad, public_key, &nb_messages_previous_chain.to_be_bytes(), &message_number.to_be_bytes()].concat()
    }
}