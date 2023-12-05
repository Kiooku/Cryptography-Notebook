//! X3DH *(Extended Triple Diffie-Hellman)* Key Agreement Protocol
//! 
//! Curve: 25519
//! Hash: Sha256
//! 
//! The implementation is based on Signal recommendation: https://signal.org/docs/specifications/x3dh/

use std::fmt;
use hkdf::Hkdf;
use rand::rngs::OsRng;
use sha2::Sha256;
use x25519_dalek::{SharedSecret, PublicKey, ReusableSecret, EphemeralSecret, StaticSecret};
use ed25519_dalek::{Signature, SigningKey, Signer, VerifyingKey, Verifier};

#[derive(PartialEq)]
pub enum X3DHError {
    SignatureInvalid,
}

const F: [u8; 32] = [0xFF; 32];
const SALT: [u8; 64] = [0x00; 64];
const INFO: &[u8; 14] = b"RedWheelbarrow";

#[derive(Clone)]
pub struct IdentityKey {
    public_key: PublicKey,
    private_key: StaticSecret,
}

impl IdentityKey {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let private_key: StaticSecret = StaticSecret::random_from_rng(&mut csprng);
        IdentityKey { public_key: PublicKey::from(&private_key), private_key: private_key }
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.public_key
    }
}

#[derive(Clone)]
pub struct SignedPrekey {
    public_key: PublicKey,
    private_key: ReusableSecret,
}

impl SignedPrekey {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let private_key: ReusableSecret = ReusableSecret::random_from_rng(&mut csprng);
        SignedPrekey { public_key: PublicKey::from(&private_key), private_key: private_key }
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn get_private_key(&self) -> ReusableSecret {
        self.private_key.clone()
    }
}

pub struct OneTimePrekey {
    public_key: PublicKey,
    private_key: EphemeralSecret,
}

impl OneTimePrekey {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let private_key: EphemeralSecret = EphemeralSecret::random_from_rng(&mut csprng);
        OneTimePrekey { public_key: PublicKey::from(&private_key), private_key: private_key }
    }

    pub fn generate_opk_bundle(n: u8) -> Vec<OneTimePrekey> {
        let mut opk_set: Vec<OneTimePrekey> = Vec::new();
        for _ in 0..n {
            opk_set.push(Self::new());
        }

        opk_set
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.public_key
    }
}

pub struct EphemeralKey {
    public_key: PublicKey,
    private_key: ReusableSecret,
}

impl EphemeralKey {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let private_key: ReusableSecret = ReusableSecret::random_from_rng(&mut csprng);
        EphemeralKey { public_key: PublicKey::from(&private_key), private_key: private_key }
    }
}

pub fn create_prekey_signature(ik: &IdentityKey, spk: &SignedPrekey) -> (Signature, VerifyingKey) {
    let signing_key: SigningKey = SigningKey::from_bytes(&ik.private_key.to_bytes());
    let signature: Signature = signing_key.sign(spk.public_key.as_bytes());
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    (signature, verifying_key)
}

pub fn create_prekey_bundle(ik: &IdentityKey, spk: &SignedPrekey, opk_bundle: &Vec<OneTimePrekey>, signature: Signature, verifying_key: VerifyingKey) -> (PublicKey, PublicKey, Vec<PublicKey>, Signature, VerifyingKey) {
    let mut opk_public_bundle: Vec<PublicKey> = Vec::new();
    for key in opk_bundle {
        opk_public_bundle.push(key.public_key);
    };

    (ik.public_key, spk.public_key, opk_public_bundle, signature, verifying_key)
}

pub fn x3dh_sender(ika: IdentityKey, ikb: PublicKey, spkb: PublicKey, signature: Signature, verifying_key: VerifyingKey, opkb: Option<PublicKey>) -> Result<([u8; 32], PublicKey, Option<PublicKey>), X3DHError> {
    // Verify the signature
    if verifying_key.verify(spkb.as_bytes(), &signature).is_err() {
        return Err(X3DHError::SignatureInvalid)
    }
    
    // Compute the shared secret
    let eka: EphemeralKey = EphemeralKey::new();

    let dh1: SharedSecret = ika.private_key.diffie_hellman(&spkb);
    let dh2: SharedSecret = eka.private_key.diffie_hellman(&ikb);
    let dh3: SharedSecret = eka.private_key.diffie_hellman(&spkb);

    let mut ikm: Vec<u8> = Vec::new();
    ikm.extend_from_slice(&F);
    ikm.extend_from_slice(dh1.as_bytes());
    ikm.extend_from_slice(dh2.as_bytes());
    ikm.extend_from_slice(dh3.as_bytes());

    // Verify that the bundle contain a one-time prekey
    if let Some(key) = opkb {
        let dh4: SharedSecret = eka.private_key.diffie_hellman(&key);
        ikm.extend_from_slice(dh4.as_bytes())
    }

    let hk = Hkdf::<Sha256>::new(Some(&SALT), &ikm);
    let mut sk: [u8; 32] = [0u8; 32];
    hk.expand(INFO, &mut sk)
        .expect("Error during the creation of the share secret");
    
    Ok((sk, eka.public_key, opkb))
}

pub fn x3dh_receiver(ika: PublicKey, eka: PublicKey, ikb: IdentityKey, spkb: SignedPrekey, opkb: Option<OneTimePrekey>) -> [u8; 32] {
    // Compute the shared secret
    let dh1: SharedSecret = spkb.private_key.diffie_hellman(&ika);
    let dh2: SharedSecret = ikb.private_key.diffie_hellman(&eka);
    let dh3: SharedSecret = spkb.private_key.diffie_hellman(&eka);

    let mut ikm: Vec<u8> = Vec::new();
    ikm.extend_from_slice(&F);
    ikm.extend_from_slice(dh1.as_bytes());
    ikm.extend_from_slice(dh2.as_bytes());
    ikm.extend_from_slice(dh3.as_bytes());

    // Verify that the bundle contain a one-time prekey
    if let Some(key) = opkb {
        let dh4: SharedSecret = key.private_key.diffie_hellman(&eka);
        ikm.extend_from_slice(dh4.as_bytes())
    }

    let hk = Hkdf::<Sha256>::new(Some(&SALT), &ikm);
    let mut sk: [u8; 32] = [0u8; 32];
    hk.expand(INFO, &mut sk)
        .expect("Error during the creation of the share secret");

    sk
}

pub fn get_ad(first_ik_pk: PublicKey, second_ik_pk: PublicKey, additional_information: Option<Vec<u8>>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let first_ik_pk_data: [u8; 32] = first_ik_pk.to_bytes();
    let second_ik_pk_data: [u8; 32] = second_ik_pk.to_bytes();
    res.extend_from_slice(&first_ik_pk_data);
    res.extend_from_slice(&second_ik_pk_data);

    if let Some(additional_information) = additional_information {
        res.extend(additional_information);
    }

    res
}

impl fmt::Display for X3DHError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            X3DHError::SignatureInvalid => write!(f, "Verification of the signature failed"),
        }
    }
}