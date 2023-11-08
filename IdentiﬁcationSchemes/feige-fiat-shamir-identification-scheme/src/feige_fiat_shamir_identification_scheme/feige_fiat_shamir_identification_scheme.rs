use rand::Rng;

pub struct FeigeFiatShamirIdentificationScheme {
    n: u128,
    y: u128,
    a: u128,
}

impl FeigeFiatShamirIdentificationScheme {
    pub fn new(n: u128, y: u128, a: u128) -> Self {
        FeigeFiatShamirIdentificationScheme { n, y, a }
    }

    pub fn challenge(&self) -> u32 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0..2)
    }

    pub fn response(&self, x: u128, b: u32, r: u128) -> u128 {
        return (x.pow(b) * r) % self.n
    }

    pub fn verification(&self, z: u128, b: u32) -> bool {
        return (z.pow(2) % self.n) == ((self.y.pow(b) * self.a) % self.n)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::FeigeFiatShamirIdentificationScheme;
    const P: u128 = 197;
    const Q: u128 = 281;
    const N: u128 = P * Q;
    const X: u128 = 57;

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

    #[test]
    fn test_feige_fiat_shamir_identification_scheme_successful() {
        let y: u128 = X.pow(2) % N;
        let mut rng = rand::thread_rng();
        let r: u128 = rng.gen_range(1..N);
        let a: u128 = r.pow(2) % N;
        let feige_fiat_shamir_identification_scheme: FeigeFiatShamirIdentificationScheme = FeigeFiatShamirIdentificationScheme::new(N, y, a);
        let expected_value: &str = "Successful verification";

        assert_eq!(round(&feige_fiat_shamir_identification_scheme, X, r), expected_value)
    }

    #[test]
    fn test_feige_fiat_shamir_identification_scheme_unsuccessful() {
        let y: u128 = X.pow(2) % N;
        let mut rng = rand::thread_rng();
        let r: u128 = rng.gen_range(1..N);
        let a: u128 = r.pow(2) % N;
        let feige_fiat_shamir_identification_scheme: FeigeFiatShamirIdentificationScheme = FeigeFiatShamirIdentificationScheme::new(N, y, a);
        let expected_value: &str = "Verification failed";

        assert_eq!(round(&feige_fiat_shamir_identification_scheme, X-1, r), expected_value)
    }
}