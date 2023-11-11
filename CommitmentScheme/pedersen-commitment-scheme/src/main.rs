//! # Pedersen commitment scheme
//! 
//! Implementation of the Pedersen commitment scheme
//! 
//! Create for learning purpose

mod pedersen_commitment_scheme;
use pedersen_commitment_scheme::pedersen_commitment_scheme::PedersonCommitmentScheme;
use num_bigint::{BigUint, RandBigInt};

fn main() {
    // Public parameters
    let g1: BigUint = BigUint::from(1252u16);
    let p: BigUint = BigUint::from(1447u16);
    let q: BigUint = BigUint::from(241u8);
    let g: BigUint = g1.modpow(&((&p - BigUint::from(1u8)) / &q), &p);
    println!("g: {}", g);
    let pedersen_commitment_scheme: PedersonCommitmentScheme = PedersonCommitmentScheme::new(&p, &q, &g);
    
    // Initialization
    let private_key: BigUint = BigUint::from(161u8);
    let public_key: BigUint = pedersen_commitment_scheme.init(&private_key);

    // Commitment
    let mut rng = rand::thread_rng();
    let r: BigUint = rng.gen_biguint_below(&q);
    let x: BigUint = BigUint::from(52 as u8);
    let c: BigUint = pedersen_commitment_scheme.commitment(&x, &r, &public_key);

    // Opening
    let success: bool = pedersen_commitment_scheme.opening(&c, &x, &r, &public_key);
    println!("The commitment is a success: {}", success);
    
}
