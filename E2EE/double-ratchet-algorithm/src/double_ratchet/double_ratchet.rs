use crate::double_ratchet::state::State;
use crate::double_ratchet::aead::{encrypt as aead_encrypt, decrypt as aead_decrypt};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use hkdf::Hkdf;
use rand_core::OsRng;
use x25519_dalek::{ReusableSecret, PublicKey};


const MAX_SKIP: u16 = 1000;
const BYTE_MESSAGE_KEY: &[u8] = &[0x01];
const BYTE_NEXT_CHAIN_KEY: &[u8] = &[0x02];
const INFO: &[u8] = &[0x73];
type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct DoubleRatchet {
    state: State,
}

impl DoubleRatchet {
    pub fn new() -> Self {
        DoubleRatchet { state: State::new() }
    }

    /// Initialize the sender Double Ratchet
    /// 
    /// # Arguments
    /// 
    /// * `sk` (\[u8; 32\]): Shared Key *(X3DH shared secret)*
    /// * `receiver_public_key` (PublicKey): Receiver public key
    pub fn init_sender(&mut self, sk: [u8; 32], receiver_public_key: PublicKey) -> () {
        self.generate_dh(); // Set dh_s
        self.state.dh_r = Some(receiver_public_key);
        let (rk_result, ck_r_result) = self.kdf_rk(sk, self.dh(self.state.dh_s.as_ref().unwrap(), self.state.dh_r.unwrap()));
        (self.state.rk, self.state.ck_s) = (Some(rk_result), Some(ck_r_result));
    }

    /// Initialize the receiver Double Ratchet
    /// 
    /// # Arguments
    /// 
    /// * `sk` (\[u8; 32\]): Shared Key *(X3DH shared secret)*
    /// * `receiver_pair` (ReusableSecret, PublicKey): Receiver pair
    pub fn init_receiver(&mut self, sk: [u8; 32], receiver_pair: (ReusableSecret, PublicKey)) -> () {
        self.state.dh_s = Some(receiver_pair);
        self.state.rk = Some(sk);
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
    
    /// Returns the output of applying a KDF keyed by a 32-byte root key `rk` to a Diffie-Hellman output `dh_out`.
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
    fn kdf_rk(&self, rk: [u8; 32], dh_out: [u8; 32]) -> ([u8; 32], [u8; 32]) {
        let ikm = dh_out;
        let salt = rk;

        let hk = Hkdf::<Sha256>::new(Some(&salt[..]), &ikm);
        let mut okm = [0u8; 64];
        hk.expand(INFO, &mut okm)
            .expect("Output length invalid KDF_RK");

        let (new_rk, new_ck) = okm.split_at(32);

        (new_rk.try_into()
            .expect("Incorrect length"),
        new_ck.try_into()
            .expect("Incorrect length"))
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
    /// * `(header, res)` ((PublicKey, u8, u8), (Vec\<u8\>, Vec\<u8\>)): Header and ciphertext
    pub fn encrypt(&mut self, plaintext: &[u8], ad: &[u8]) -> ((PublicKey, u8, u8), (Vec<u8>, Vec<u8>)) {
        let mk: [u8; 32];
        (self.state.ck_s, mk) = self.kdf_ck(self.state.ck_s.unwrap());
        let header: (PublicKey, u8, u8) = self.header(self.state.dh_s.as_ref().unwrap(), self.state.pn, self.state.n_s);
        self.state.n_s += 1;
        let res = match aead_encrypt(mk, plaintext, &self.concat(ad, header)) {
            Ok((ciphertext, nonce)) => (ciphertext, nonce),
            Err(error) => panic!("Error (AES-GCM-SIV): {:?}", error),
        };
        (header, res)
    }
    
    /// Returns the AEAD (AES-GCM-SIV-256) decryption of ciphertext with message key mk.
    /// 
    /// # Arguments
    /// 
    /// * `header` ((PublicKey, u8, u8)): Header
    /// * `ciphertext` (&\[u8\]): Ciphertext
    /// * `nonce` (Vec\<u8\>): Nonce
    /// * `ad` (&\[u8\]): Associated Data
    /// 
    /// # Output
    /// 
    /// * `plaintext` (Vec\<u8\>): Plaintext
    pub fn decrypt(&mut self, header: (PublicKey, u8, u8), ciphertext: Vec<u8>, nonce: Vec<u8>, ad: &[u8]) -> Vec<u8> {
        let plaintext = self.try_skipped_message_keys(header, &ciphertext, &nonce, ad);
        if plaintext.is_some() {
            return plaintext.unwrap()
        }
        if self.state.dh_r.is_none() || header.0 != self.state.dh_r.unwrap() {
            self.skip_message_keys(header.1);
            self.state.pn = self.state.n_s;
            (self.state.n_s, self.state.n_r) = (0, 0);
            self.state.dh_r = Some(header.0);
            let (rk_result, ck_r_result) = self.kdf_rk(self.state.rk.unwrap(), self.dh(self.state.dh_s.as_ref().unwrap(), self.state.dh_r.unwrap()));
            (self.state.rk, self.state.ck_r) = (Some(rk_result), Some(ck_r_result));
            self.generate_dh(); // New dh_s
            let (rk_result, ck_s_result) = self.kdf_rk(self.state.rk.unwrap(), self.dh(self.state.dh_s.as_ref().unwrap(), self.state.dh_r.unwrap()));
            (self.state.rk, self.state.ck_s) = (Some(rk_result), Some(ck_s_result));
        }
        self.skip_message_keys(header.2);
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
    /// * `header` ((PublicKey, u8, u8)): Header
    /// * `ciphertext` (&Vec\<u8\>): Ciphertext
    /// * `nonce` (&Vec\<u8\>): Nonce
    /// * `ad` (&\[u8\]): Associated Data
    /// 
    /// # Output
    /// 
    /// `plaintext` (Option\<Vec\<u8\>\>): Optional plaintext
    fn try_skipped_message_keys(&mut self, header: (PublicKey, u8, u8), ciphertext: &Vec<u8>, nonce: &Vec<u8>,  ad: &[u8]) -> Option<Vec<u8>> {
        if let Some(mk) = self.state.mkskipped.remove(&(header.0, header.2)) {
            let res = match aead_decrypt(mk, ciphertext, &nonce, &self.concat(ad, header)) {
                Ok(plaintext) => plaintext,
                Err(error) => panic!("Error (AES-GCM-SIV): {:?}", error), // TODO deal with the error in a better way
            };
            return Some(res)
        }
        None
    }
    
    /// Stores any skipped message keys from the current receiving chain.
    /// 
    /// # Arguments
    /// * `until` (u8)
    fn skip_message_keys(&mut self, until: u8) -> () {
        if self.state.n_r as u16 + MAX_SKIP < until as u16 {
            panic!("Error: state.Nr + MAX_SKIP < until");
        }
        if self.state.ck_r != None {
            while self.state.n_r < until {
                let mk: [u8; 32];
                (self.state.ck_r, mk) = self.kdf_ck(self.state.ck_r.unwrap());
                self.state.mkskipped.insert((self.state.dh_r.unwrap(), self.state.n_r), mk); // TODO check this unwrap if it's working 100% of the time (does dh_r can be None?)
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