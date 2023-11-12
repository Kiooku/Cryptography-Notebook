use primes::is_prime;
use num_bigint::{BigUint, RandBigInt};

pub struct SchnorrSigmaProtocol {
    rng: rand::rngs::ThreadRng,
    p: BigUint,
    q: BigUint,
    g: BigUint,
}

impl SchnorrSigmaProtocol {
    pub fn new(p: &BigUint, q: &BigUint, g: &BigUint) -> Self {
        if !is_prime(q.try_into().unwrap()) || !is_prime(p.try_into().unwrap()) {
            panic!("'p' and 'q' should be prime numbers")
        } else if p % q != BigUint::from(1 as u8) {
            panic!("'p' should be congruent to 1 modulo 'q'")
        }
        let rng = rand::thread_rng();
        SchnorrSigmaProtocol { rng, p: p.clone(), q: q.clone(), g: g.clone() }
    }

    /// Returns the public key
    ///
    /// # Arguments
    ///
    /// * `w` (&BigUint) - Private key
    ///
    /// # Output
    /// 
    /// * `h` (BigUint) - Public key
    pub fn init(&self, w: &BigUint) -> BigUint {
        if BigUint::from(0 as u8) == w.clone() && w.clone() >= self.q {
            panic!("The secret key ('w') should be between 0 < w < q")
        }
        self.g.modpow(w, &self.p)
    }

    /// Returns the first message
    ///
    /// # Output
    /// 
    /// * `(r, a)` (BigUint, BigUint) - (Random value, First message)
    pub fn craft_first_message(&mut self) -> (BigUint, BigUint) {
        let r: BigUint = self.rng.gen_biguint_below(&self.q);
        (r.clone(), self.g.modpow(&r, &self.p))
    }

    /// Returns the challenge
    ///
    /// # Output
    /// 
    /// * `e` (BigUint) - (Random challenge)
    pub fn get_challenge(&mut self) -> BigUint {
        self.rng.gen_biguint_below(&self.q)
    }

    /// Returns the response
    ///
    /// # Arguments
    ///
    /// * `r` (&BigUint) - Random value
    /// * `e` (&BigUint) - Random challenge
    /// * `w` (&BigUint) - Private key
    ///
    /// # Output
    /// 
    /// * `z` (BigUint) - Response
    pub fn response(&self, r: &BigUint, e: &BigUint, w: &BigUint) -> BigUint {
        (r + e * w) % &self.q
    }

    /// Verify the identity
    ///
    /// # Arguments
    ///
    /// * `z` (&BigUint) - Response
    /// * `a` (&BigUint) - First message
    /// * `h` (&BigUint) - Public key
    /// * `e` (&BigUint) - Random challenge
    ///
    /// # Output
    /// 
    /// * (bool) - identity
    pub fn verification(&self, z: &BigUint, a: &BigUint, h: &BigUint, e: &BigUint) -> bool {
        self.g.modpow(z, &self.p) == (a * h.modpow(e, &self.p)) % &self.p
    }
}