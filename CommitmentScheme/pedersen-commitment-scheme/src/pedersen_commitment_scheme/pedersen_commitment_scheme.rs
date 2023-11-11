use primes::is_prime;
use num_bigint::BigUint;

pub struct PedersonCommitmentScheme {
    p: BigUint,
    q: BigUint,
    g: BigUint,
}

impl PedersonCommitmentScheme {
    pub fn new(p: &BigUint, q: &BigUint, g: &BigUint) -> Self {
        if !is_prime(q.try_into().unwrap()) || !is_prime(p.try_into().unwrap()) {
            panic!("'p' and 'q' should be prime numbers")
        } else if p % q != BigUint::from(1 as u8) {
            panic!("'p' should be congruent to 1 modulo 'q'")
        }
        PedersonCommitmentScheme { p: p.clone(), q: q.clone(), g: g.clone() }
    }

    /// Returns the public key
    ///
    /// # Arguments
    ///
    /// * `a` (&BigUint) - Private key
    ///
    /// # Output
    /// 
    /// * `h` (BigUint) - Public key
    pub fn init(&self, a: &BigUint) -> BigUint {
        if BigUint::from(0 as u8) == a.clone() && a.clone() >= self.q {
            panic!("The secret key ('a') should be between 0 < a < q")
        }
        self.g.modpow(a, &self.p)
    }

    /// Returns the commitment
    ///
    /// # Arguments
    ///
    /// * `x` (&BigUint) - Value to commit
    /// * `r` (&BigUint) - Random integer (0 < r < q)
    /// * `h` (&BigUint) - Public key
    ///
    /// # Output
    /// 
    /// * `c` (BigUint) - Commitment
    pub fn commitment(&self, x: &BigUint, r: &BigUint, h: &BigUint) -> BigUint {
        if BigUint::from(0 as u8) == r.clone() && r.clone() >= self.q {
            panic!("The random value ('r') should be between 0 < r < q")
        }
        (self.g.modpow(x, &self.p) * h.modpow(r, &self.p)) % &self.p
    }

    /// Returns the commitment
    ///
    /// # Arguments
    ///
    /// * `c` (&BigUint) - Commitment
    /// * `x` (&BigUint) - Value to commit
    /// * `r` (&BigUint) - Random integer (0 < r < q)
    /// * `h` (&BigUint) - Public key
    ///
    /// # Output
    /// 
    /// * boolean
    pub fn opening(&self, c: &BigUint, x: &BigUint, r: &BigUint, h: &BigUint) -> bool {
        let c_prime: BigUint = (self.g.modpow(&x, &self.p) * h.modpow(&r, &self.p)) % &self.p;
        c_prime == c.clone()
    }
}