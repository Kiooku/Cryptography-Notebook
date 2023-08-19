//! # Double Ratchet Algorithm
//! 
//! Implementation of the algorithm use in the E2EE
//! 
//! Create for learning purpose

#![deny(missing_docs,
    missing_debug_implementations, missing_copy_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_variables, unused_mut, unused_parens, 
    unused_import_braces, unused_qualifications)]

mod kdf;

use kdf::hmac::hmac;

fn main() {
    let k1: &str = "key";
    let m1: &str = "The quick brown fox jumps over the lazy dog";
    let hmac_result = hmac(m1.as_bytes().to_vec(), k1.as_bytes().to_vec());

    println!("HMAC Result: {}", hmac_result);
    println!("Hello, world!");
}

