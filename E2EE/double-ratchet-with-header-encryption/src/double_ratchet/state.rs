use std::collections::HashMap;
use x25519_dalek::{ReusableSecret, PublicKey as PublicKey25519};

// split dh_s to two variable, because EphemeralSecret does not implement the Copy trait
#[derive(Clone)]
pub struct State {
    pub dh_s: Option<(ReusableSecret, PublicKey25519)>, // DH Ratchet key pair (the "sending" or "self" ratchet key)
    pub dh_r: Option<PublicKey25519>, // DH Ratchet public key (the "received" or "remote" key)
    pub rk: Option<[u8; 32]>, // 32-byte Root Key
    pub ck_s: Option<[u8; 32]>, // 32-byte Chain Keys for sending
    pub ck_r: Option<[u8; 32]>, // 32-byte Chain Keys for receiving
    pub hk_s: Option<[u8; 32]>, // 32-byte Header Keys for sending
    pub hk_r: Option<[u8; 32]>, // 32-byte Header Keys for receiving
    pub nhk_s: Option<[u8; 32]>, // 32-byte Next Header Keys for sending
    pub nhk_r: Option<[u8; 32]>, // 32-byte Next Header Keys for receiving
    pub n_s: u8, // Message numbers for sending
    pub n_r: u8, // Message numbers for receiving
    pub pn: u8, // Number of messages in previous sending chain
    pub mkskipped: HashMap<([u8; 32], u8), [u8; 32]>, // Dictionary of skipped-over message keys, indexed by header key and message number.
}

impl State {
    pub fn new() -> Self {
        State { 
            dh_s: None,
            dh_r: None, 
            rk: None, 
            ck_s: None, 
            ck_r: None, 
            hk_s: None,
            hk_r: None,
            nhk_s: None,
            nhk_r: None,
            n_s: 0, 
            n_r: 0, 
            pn: 0, 
            mkskipped: HashMap::new() }
    }
}