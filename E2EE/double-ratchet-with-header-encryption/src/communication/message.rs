use x25519_dalek::PublicKey;

#[derive(Clone, Debug)]
pub struct Message {
    username: String,
    header_he: HeaderHE, 
    ciphertext: Ciphertext, 
    ek_sender: Option<PublicKey>, 
    opk_used: Option<PublicKey>,
}

impl Message {
    pub fn new(username: String, header_he: HeaderHE, ciphertext: Ciphertext, ek_sender: Option<PublicKey>, opk_used: Option<PublicKey>) -> Self {
        Message { username: username, header_he: header_he, ciphertext: ciphertext, ek_sender: ek_sender, opk_used: opk_used}
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_header_he(&self) -> HeaderHE {
        self.header_he.clone()
    }

    pub fn get_ciphertext(&self) -> Ciphertext {
        self.ciphertext.clone()
    }

    pub fn get_ek_sender(&self) -> Option<PublicKey> {
        self.ek_sender
    }

    pub fn get_opk_used(&self) -> Option<PublicKey> {
        self.opk_used
    }
}

#[derive(Clone, Debug)]
pub struct Ciphertext {
    ciphertext: Vec<u8>,
    nonce: Vec<u8>,
}

impl Ciphertext {
    pub fn new(ciphertext: Vec<u8>, nonce: Vec<u8>) -> Self {
        Ciphertext { ciphertext: ciphertext, nonce: nonce }
    }

    pub fn get_ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }

    pub fn get_nonce(&self) -> Vec<u8> {
        self.nonce.clone()
    }
}

#[derive(Clone, Debug)]
pub struct HeaderHE { // Duplication but make the code easier to understand
    ciphertext: Vec<u8>, 
    nonce: Vec<u8>,
}

impl HeaderHE {
    pub fn new(ciphertext: Vec<u8>, nonce: Vec<u8>) -> Self {
        HeaderHE { ciphertext: ciphertext, nonce: nonce }
    }

    pub fn get_ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }

    pub fn get_nonce(&self) -> Vec<u8> {
        self.nonce.clone()
    }
}