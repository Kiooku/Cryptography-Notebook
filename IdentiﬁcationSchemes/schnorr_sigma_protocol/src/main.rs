//! # Schnorr sigma protocol
//! 
//! Implementation of the Schnorr sigma protocol
//! 
//! Create for learning purpose

mod schnorr_sigma_protocol;
use schnorr_sigma_protocol::schnorr_sigma_protocol::SchnorrSigmaProtocol;
use num_bigint::BigUint;

fn main() {
    // Public parameters
    let g1: BigUint = BigUint::from(439u16);
    let q: BigUint = BigUint::from(233u8);
    let p: BigUint = BigUint::from(2u8) * &q + BigUint::from(1u8);
    let g: BigUint = g1.modpow(&((&p - BigUint::from(1u8)) / &q), &p);
    println!("g: {}", g);
    let mut schnorr_sigma_protocol: SchnorrSigmaProtocol = SchnorrSigmaProtocol::new(&p, &q, &g);

    // Initialization
    let private_key: BigUint = BigUint::from(20u8);
    let public_key: BigUint = schnorr_sigma_protocol.init(&private_key);

    // First message
    let (r, first_message): (BigUint, BigUint) = schnorr_sigma_protocol.craft_first_message();

    // Challenge
    let challenge: BigUint = schnorr_sigma_protocol.get_challenge();

    // Response
    let response: BigUint = schnorr_sigma_protocol.response(&r, &challenge, &private_key);

    // Verification
    let verification: bool = schnorr_sigma_protocol.verification(&response, &first_message, &public_key, &challenge);
    println!("Identity check: {}", verification);
}
