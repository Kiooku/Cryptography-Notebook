use crate::communication;
use std::collections::HashMap;
use communication::key_collection::{ClientKeyCollection, ServerKeyCollection};
use hex_literal::hex;
use hkdf::Hkdf;
use sha2::Sha256;
use crate::x3dh::x3dh::X3DHError;
use crate::double_ratchet::double_ratchet::DoubleRatchetHE;
use x25519_dalek::PublicKey;

use super::key_collection::KeyError;
use super::message::{Ciphertext, HeaderHE, Message};

const INFO_CLIENT: &[u8] = &hex!("0bd4acb230e3990fd3a6");
const SALT_CLIENT: &[u8] = &hex!("47194bfb6a93dd4f2cae");

pub struct Client {
    name: String,
    communications: HashMap<String, (Vec<u8>, DoubleRatchetHE)>, // Each communication has a different double ratchet (Key: username, ad) (Value: double ratchet for the communication)
    keys: ClientKeyCollection,
}

impl Client {
    // Multiple double ratchet for the multiple messages that can be send between users
    pub fn new(name: String) -> Self {
        // Generate the X3DH key for the server
        let keys: ClientKeyCollection = ClientKeyCollection::new();

        // Create the client object
        Client {
            name: name,
            communications: HashMap::new(),
            keys: keys,
        }
    }

    pub fn get_server_keys(&self) -> ServerKeyCollection {
        ServerKeyCollection::from(self.keys.get_ik(), self.keys.get_spk(), self.keys.get_opk_bundle(), self.keys.get_signature(), self.keys.get_verifying_key())
    }

    pub fn get_client_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_keys(&self) -> &ClientKeyCollection {
        &self.keys
    }

    /// Read all the messages sent by one user
    /// 
    /// # Arguments
    /// 
    /// * `receiver_name` (&String): Name of the person that will receive the message
    /// * `messages` (&[u8]): Message(s) sent by the user *(can have multiple ciphertext when you are offline)*
    /// * `r_keys`: (&ServerKeyCollection)
    /// 
    /// # Output
    /// 
    /// * `ciphertext` (Result\<((PublicKey, Option\<PublicKey\>), (Header, Ciphertext)), X3DHError\>): ((Public Ephemeral Key, Public One Time Prekey used), (Header, Ciphertext))
    fn send_first_message(&mut self, receiver_name: &String, message: &[u8], r_keys: &ServerKeyCollection) -> Result<((PublicKey, Option<PublicKey>), (HeaderHE, Ciphertext)), X3DHError> {
        // X3DH: Sending the initial message
        let (sk, ad, ek_pub, opk_used): ([u8; 32], Vec<u8>, PublicKey, Option<PublicKey>);
        (sk, ad, ek_pub, opk_used) = match self.keys.generate_sender_shared_secret(&r_keys) {
            Ok((sk, ad, ek, opk)) => (sk, ad, ek, opk),
            Err(error) => return Err(error)
        };

        // Double Ratchet
        let mut double_ratchet: DoubleRatchetHE = DoubleRatchetHE::new();

        let (shared_hk, shared_nhk): ([u8; 32], [u8; 32]) = self.generate_shared_hk_and_nhk(sk);
        double_ratchet.init_sender_he(sk, r_keys.get_spk(), shared_hk, shared_nhk);
        
        let (encrypted_header, ciphertext): ((Vec<u8>, Vec<u8>), (Vec<u8>, Vec<u8>));
        (encrypted_header, ciphertext) = double_ratchet.encrypt_he(message, &ad);
        self.communications.insert(receiver_name.clone(), (ad, double_ratchet));

        Ok(((ek_pub, opk_used), (HeaderHE::new(encrypted_header.0,encrypted_header.1), Ciphertext::new(ciphertext.0, ciphertext.1))))
    }

    /// Read the first messages sent by one user *(Double ratchet not initialize yet)*
    /// 
    /// # Arguments
    /// 
    /// * `sender_name` (&String): Name of the person that sent you the message
    /// * `ik_sender` (PublicKey): Public Identity Key of the sender (input when you want to initialize the communication)
    /// * `messages` (& Message): Message sent by the user
    /// 
    /// # Output
    /// 
    /// * `plaintext_received` (Result\<Vec\<Vec\<u8\>\>, KeyError\>): All the plaintext received *(can have multiple plaintext when you are offline)*
    fn read_first_message(&mut self, sender_name: &String, ik_sender: PublicKey, message: &Message) -> Result<Vec<u8>, KeyError> {
        // X3DH: Receiving the initial message
        let (sk, ad): ([u8; 32], Vec<u8>);
        (sk, ad) = match self.keys.generate_receiver_shared_secret(ik_sender, &message) {
            Ok((sk, ad)) => (sk, ad),
            Err(error) => return Err(error),
        };

        // Double Ratchet
        let mut double_ratchet: DoubleRatchetHE = DoubleRatchetHE::new();

        let (shared_hk, shared_nhk): ([u8; 32], [u8; 32]) = self.generate_shared_hk_and_nhk(sk);
        double_ratchet.init_receiver_he(sk, (self.keys.get_spk_private(), self.keys.get_spk_public()), shared_hk, shared_nhk); // Let like this to allow simple DH instead of X3DH to start

        let plaintext: Vec<u8> = double_ratchet.decrypt_he((message.get_header_he().get_ciphertext(), message.get_header_he().get_nonce()), 
                    message.get_ciphertext().get_ciphertext(), 
                    message.get_ciphertext().get_nonce(), 
                    &ad);
        self.communications.insert(sender_name.clone(), (ad, double_ratchet));

        Ok(plaintext)
    }
    
    /// Read all the messages sent by one user
    /// 
    /// # Arguments
    /// 
    /// * `receiver_name` (&String): Name of the person that will receive the message
    /// * `messages` (&[u8]): Message(s) sent by the user *(can have multiple ciphertext when you are offline)*
    /// * `r_keys`: (&ServerKeyCollection)
    /// 
    /// # Output
    /// 
    /// * `ciphertext` (Result\<(Option\<(PublicKey, Option<PublicKey>)>, (Header, Ciphertext)), X3DHError>): ((Public Ephemeral Key, Public One Time Prekey used), (Header, Ciphertext))
    pub fn send_message(&mut self, receiver_name: &String, message: &[u8], r_keys: &ServerKeyCollection) -> Result<(Option<(PublicKey, Option<PublicKey>)>, (HeaderHE, Ciphertext)), X3DHError> {
        // Send a message to the define user (check if the first message has already been sends, otherwise use first message instead)
        if !self.communications.contains_key(receiver_name) {
            match self.send_first_message(receiver_name, message, r_keys) {
                Ok(((ek_pub, opk_used), (header, ciphertext))) => return Ok((Some((ek_pub, opk_used)), (header, ciphertext))),
                Err(error) => return Err(error),
            }
        } else {
            if let Some((ad, double_ratchet)) = self.communications.get_mut(receiver_name) {
                let (encrypted_header, ciphertext): ((Vec<u8>, Vec<u8>), (Vec<u8>, Vec<u8>));
                (encrypted_header, ciphertext) = double_ratchet.encrypt_he(message, &ad);
                // Update communication
                *self.communications.get_mut(receiver_name).unwrap() = (ad.clone(), double_ratchet.clone());
                return Ok((None, (HeaderHE::new(encrypted_header.0, encrypted_header.1), Ciphertext::new(ciphertext.0, ciphertext.1))))
            };
        }

        panic!("User not found, which is not normal");
    }
    
    /// Read all the messages sent by one user
    /// 
    /// # Arguments
    /// 
    /// * `sender_name` (&String): Name of the person that sent you the message
    /// * `ik_sender` (Option\<PublicKey\>): Public Identity Key of the sender (input when you want to initialize the communication)
    /// * `messages` (mut Vec\<Message\>): Message(s) sent by the user *(can have multiple ciphertext when you are offline)*
    /// 
    /// # Output
    /// 
    /// * `plaintext_received` (Result\<Vec\<Vec\<u8\>\>, KeyError\>): All the plaintext received *(can have multiple plaintext when you are offline)*
    pub fn read_messages(&mut self, sender_name: &String, ik_sender: Option<PublicKey>, mut messages: Vec<Message>) -> Result<Vec<Vec<u8>>, KeyError> {
        // If it's the first message init the double ratchet with X3DH
        let mut plaintext_received: Vec<Vec<u8>> = Vec::new();
        if messages.len() > 0 { 
            if !self.communications.contains_key(sender_name) {
                if let Some(ik) = ik_sender {
                    let first_message: Message = messages.pop().unwrap();
                    match self.read_first_message(sender_name, ik, &first_message) {
                        Ok(plaintext) => plaintext_received.push(plaintext),
                        Err(error) => return Err(error),
                    };
                } else {
                    return Err(KeyError::IdentityKeyAbsent)
                }
                
            }
            
            if let Some((ad, double_ratchet)) = self.communications.get_mut(sender_name) {
                for message in messages {
                    let current_plaintext: Vec<u8> = double_ratchet.decrypt_he((message.get_header_he().get_ciphertext(), message.get_header_he().get_nonce()), 
                        message.get_ciphertext().get_ciphertext(), 
                        message.get_ciphertext().get_nonce(), 
                        ad);
                    plaintext_received.push(current_plaintext);                    
                }
                *self.communications.get_mut(sender_name).unwrap() = (ad.clone(), double_ratchet.clone());
            }
        }

        Ok(plaintext_received)
    }

    /// Returns the initial **header key** and **next header key**
    /// 
    /// # Arguments
    /// 
    /// * `x3dh_shared_secret` (\[u8; 32\]): X3DH shared secret
    /// 
    /// # Output
    /// 
    /// * `hk` (\[u8; 32\]): 32-byte header key
    /// * `nhk` (\[u8; 32\]): 32-byte next header Keys
    fn generate_shared_hk_and_nhk(&self, x3dh_shared_secret: [u8; 32]) -> ([u8; 32], [u8; 32]) {
        let ikm = x3dh_shared_secret;
        let salt = SALT_CLIENT;

        let hk = Hkdf::<Sha256>::new(Some(&salt[..]), &ikm);
        let mut okm = [0u8; 64];
        hk.expand(INFO_CLIENT, &mut okm)
            .expect("Output length invalid KDF_RK");

        let (shared_hk, shared_nhk) = okm.split_at(32);
        
        (shared_hk.try_into()
            .expect("Incorrect length"),
        shared_nhk.try_into()
            .expect("Incorrect length"))
    }
}