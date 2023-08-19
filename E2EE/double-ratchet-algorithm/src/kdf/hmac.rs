use sha2::{Sha256, Digest};

/// Sha-256 function to remove code duplication
/// 
/// Args:
///     
///     data (Vec<Vec<u8>>): all the data to hash using SHA-256
/// 
/// Returns:
/// 
///     Vec<u8>: hash value
pub fn sha256(data: Vec<Vec<u8>>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    for d in data {
        hasher.update(d);
    }
    hasher.finalize().to_vec()
}

/// XOR two vectors
/// 
/// Args:
///     
///     v1 (&Vec<u8>): first vector
///     v2 (&Vec<u8>): second vector
/// 
/// Returns:
/// 
///     Vec<u8>: XOR vector
pub fn xor_vec(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    v1.iter()
        .zip(v2.iter())
        .map(|(&k, &p)| k ^ p)
        .collect()
}

/// HMAC algorithm (SHA-256)
/// 
/// Args:
///
///     message (Vec<u8>): message to authentify
///     key (Vec<u8>): shared key
///
/// Returns:
/// 
///     String: result of the HMAC algorithm for message authentification
pub fn hmac(message: Vec<u8>, mut key: Vec<u8>) -> String {
    const BLOCK_SIZE: usize = 64;
    let ipad: Vec<u8> = b"\x36".repeat(BLOCK_SIZE);
    let opad: Vec<u8> = b"\x5c".repeat(BLOCK_SIZE);

    if key.len() > BLOCK_SIZE {
        key = sha256(vec![key]);
    }
    key.extend(b"\x00".repeat(BLOCK_SIZE - key.len()));


    let k_ipad: Vec<u8> = xor_vec(&key, &ipad);
    
    let k_opad: Vec<u8> = xor_vec(&key, &opad);
    
    let res = sha256(vec![k_opad, sha256(vec![k_ipad, message])]);

    return res.iter().map(|byte| format!("{:02x}", byte)).collect();
}



#[cfg(test)]
mod tests {
    use crate::hmac;

    #[test]
    fn test_hmac() {
        const K1: &str = "key";
        const M1: &str = "The quick brown fox jumps over the lazy dog";
        let hmac_result = hmac(M1.as_bytes().to_vec(), K1.as_bytes().to_vec());
        const EXPECTED_VALUE: &str = "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8";
    
        assert_eq!(hmac_result, EXPECTED_VALUE);
    }
}
