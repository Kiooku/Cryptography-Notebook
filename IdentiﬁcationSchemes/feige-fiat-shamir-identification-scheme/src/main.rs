//! # Feige–Fiat–Shamir identification scheme
//! 
//! Implementation of the Feige–Fiat–Shamir identification scheme
//! 
//! Create for learning purpose

use rand::Rng;
mod feige_fiat_shamir_identification_scheme;
use feige_fiat_shamir_identification_scheme::feige_fiat_shamir_identification_scheme::FeigeFiatShamirIdentificationScheme;

fn round(ffs_identification_scheme: &FeigeFiatShamirIdentificationScheme, x: u128, r: u128) -> &str {
    for _ in 0..80 {
        // Victor Challenge
        let b: u32 = ffs_identification_scheme.challenge();
        // Peggy Response
        let z: u128 = ffs_identification_scheme.response(x, b, r);
        // Victor Verification
        let is_valid: bool = ffs_identification_scheme.verification(z, b);
        if !is_valid {
            return "Verification failed"
        }
    }
    return "Successful verification"
}

fn main() {
    // Peggy Initialization
    let p: u128 = 197;
    let q: u128 = 281;
    let n: u128 = p * q;
    let x: u128 = 57;
    let y: u128 = x.pow(2) % n;
    let mut rng = rand::thread_rng();
    let r: u128 = rng.gen_range(1..n);
    let a: u128 = r.pow(2) % n;
    let feige_fiat_shamir_identification_scheme: FeigeFiatShamirIdentificationScheme = FeigeFiatShamirIdentificationScheme::new(n, y, a);
    // Unsuccessful verification
    println!("{}", round(&feige_fiat_shamir_identification_scheme, x-1, r));
    // Successful verification
    println!("{}", round(&feige_fiat_shamir_identification_scheme, x, r));
}
