use crate::x3dh::x3dh::{IdentityKey, SignedPrekey, OneTimePrekey,  x3dh_sender, x3dh_receiver, create_prekey_signature, create_prekey_bundle, X3DHError, get_ad};
use ed25519_dalek::{Signature, VerifyingKey};
use x25519_dalek::{PublicKey, ReusableSecret};
use std::fmt;

use super::message::Message;

const BASIC_AMOUNT_OF_OPK: u8 = 50; // Change base on the average user behaviour

pub enum KeyError {
    EphemeralKeyAbsent,
    IdentityKeyAbsent,
}

pub struct ClientKeyCollection {
    ik: IdentityKey,
    spk: SignedPrekey,
    opk_bundle: Vec<OneTimePrekey>,
    signature: Signature,
    verifying_key: VerifyingKey,
}

pub struct ServerKeyCollection {
    ik: PublicKey,
    spk: PublicKey,
    opk_bundle: Vec<PublicKey>,
    signature: Signature,
    verifying_key: VerifyingKey,
}

impl ClientKeyCollection {
    pub fn new() -> Self {
        let ik: IdentityKey = IdentityKey::new();
        let spk: SignedPrekey = SignedPrekey::new();
        let opk_bundle: Vec<OneTimePrekey> = OneTimePrekey::generate_opk_bundle(BASIC_AMOUNT_OF_OPK);
        let (signature, verification_key): (Signature, VerifyingKey) = create_prekey_signature(&ik, &spk);
        
        ClientKeyCollection { ik: ik, spk: spk, opk_bundle: opk_bundle, signature: signature, verifying_key: verification_key }
    }

    pub fn from(ik: IdentityKey, spk: SignedPrekey, opk_bundle: Vec<OneTimePrekey>, signature: Signature, verifying_key: VerifyingKey) -> Self {
        ClientKeyCollection { ik: ik, spk: spk, opk_bundle: opk_bundle, signature: signature, verifying_key: verifying_key }
    }

    /// Generate the sender shared secret
    /// 
    /// # Arguments
    /// 
    /// * `r_keys` (&ServerKeyCollection): All the public key of the receiver on the server
    /// 
    /// # Output
    /// 
    /// * `(shared_secret, associated_data, ephemeral_key_sender, one_time_prekey_used` (Result\<([u8; 32], Vec\<u8\>, PublicKey, Option\<PublicKey\>), X3DHError\>): (Shared Secret, Associated Data, EphemeralKey sender, OneTimePrekey used)
    pub fn generate_sender_shared_secret(&self, r_keys: &ServerKeyCollection) -> Result<([u8; 32], Vec<u8>, PublicKey, Option<PublicKey>), X3DHError> {
        let sk: [u8; 32];
        let eka: PublicKey;
        let opk_used: Option<PublicKey>;
        match x3dh_sender(self.get_ik(), r_keys.get_ik(), r_keys.get_spk(), r_keys.signature, r_keys.verifying_key, r_keys.get_opk_bundle().pop()) {
            Ok((current_sk, current_eka, current_opkb)) => {
                sk = current_sk;
                eka = current_eka;
                opk_used = current_opkb;
            },
            Err(error) => return Err(error),
        };

        let ad: Vec<u8> = get_ad(self.get_ik_public(), r_keys.get_ik(), None);

        Ok((sk, ad, eka, opk_used))
    }

    /// Generate the receiver shared secret
    /// 
    /// # Arguments
    /// 
    /// * `ik_sender` (PublicKey): Public Identity Key of the sender
    /// * `message` (&Message): Ciphertext
    /// 
    /// # Output
    /// 
    /// * `(shared_secret, associated_data)` (Result\<([u8; 32], Vec\<u8\>): (Shared Secret, Associated Data)
    pub fn generate_receiver_shared_secret(&mut self, ik_sender: PublicKey, message: &Message) -> Result<([u8; 32], Vec<u8>), KeyError>  {
        let ek_sender: PublicKey = message.get_ek_sender().ok_or(KeyError::EphemeralKeyAbsent)?;
        let mut opk_used: Option<OneTimePrekey> = None;
        if message.get_opk_used().is_some() {
            opk_used = self.get_opk_used(message.get_opk_used().unwrap());
        }
        
        let sk: [u8; 32] = x3dh_receiver(ik_sender, ek_sender, self.get_ik(), self.get_spk(), opk_used);
        let ad: Vec<u8> = get_ad(ik_sender, self.get_ik_public(), None);

        Ok((sk, ad))
    }

    pub fn get_ik(&self) -> IdentityKey {
        self.ik.clone()
    }

    pub fn get_ik_public(&self) -> PublicKey {
        self.ik.get_public_key()
    }

    pub fn get_spk(&self) -> SignedPrekey {
        self.spk.clone()
    }

    pub fn get_spk_public(&self) -> PublicKey {
        self.spk.get_public_key()
    }

    pub fn get_spk_private(&self) -> ReusableSecret {
        self.spk.get_private_key()
    }

    pub fn get_opk_bundle(&self) -> &Vec<OneTimePrekey> {
        &self.opk_bundle
    }

    pub fn get_signature(&self) -> Signature {
        self.signature
    }

    pub fn get_verifying_key(&self) -> VerifyingKey {
        self.verifying_key
    }

    pub fn get_opk_used(&mut self, opkb_used: PublicKey) -> Option<OneTimePrekey> {
        if let Some(index) = self.opk_bundle.iter().position(|key| key.get_public_key() == opkb_used) {
            // Item found, remove it and return it
            return Some(self.opk_bundle.swap_remove(index));
        }
        None
    }
}

impl ServerKeyCollection {
    pub fn from(ik: IdentityKey, spk: SignedPrekey, opk_bundle: &Vec<OneTimePrekey>, signature: Signature, verifying_key: VerifyingKey) -> Self {
        let (ik_server, spk_server, opk_bundle_server, signature_server, verifying_key_server): (PublicKey, PublicKey, Vec<PublicKey>, Signature, VerifyingKey) = create_prekey_bundle(&ik, &spk, opk_bundle, signature, verifying_key);
        ServerKeyCollection { ik: ik_server, spk: spk_server, opk_bundle: opk_bundle_server, signature: signature_server, verifying_key: verifying_key_server }
    }

    pub fn get_ik(&self) -> PublicKey {
        self.ik
    }

    pub fn get_spk(&self) -> PublicKey {
        self.spk
    }

    pub fn get_opk_bundle(&self) -> Vec<PublicKey> {
        self.opk_bundle.clone()
    }
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyError::EphemeralKeyAbsent => write!(f, "No ephemeral key to initialize the receiver X3DH"),
            KeyError::IdentityKeyAbsent => write!(f, "No identity key to initialize the receiver X3DH"),
        }
    }
}