class NTRUEncrypt:
    """NTRUEncrypt cipher
    """
    def __init__(self, N: int, p: int, q: int, d: int) -> None:
        if gcd(p, q) != 1 or gcd(N, q) != 1:
            raise Exception("Error: Should have gcd(p, q) = gcd(N, q) = 1")
        if q <= (6*d + 1) * p:
            raise Exception("Error: Should have q > (6d + 1)*p")
        
        self.N: int = N
        self.p: int = p
        self.q: int = q
        self.d: int = d
        R = PolynomialRing(ZZ, "x")
        ideal = R.ideal(x^N - 1)
        self.R_N: QuotientRing = QuotientRing(PolynomialRing(ZZ, "x"), ideal)
        self.Rp: QuotientRing = QuotientRing(PolynomialRing(GF(p), "x"), ideal)
        self.Rq: QuotientRing = QuotientRing(PolynomialRing(GF(q), "x"), ideal)

    
    def key_creation(self, f: list[int], g: list[int]):
        """ Key creation for the NTRUEncrypt cipher

        Args:
            f (list[int]): coefficients of the polynomial f
            g (list[int]): coefficients of the polynomial g

        Output:
            [(priv_key), pub_key]: private and public key
        """
        f_q = self.Rq(f)
        g_q = self.Rq(g)
        Fq = f_q.inverse_mod(g_q)

        f_p = self.Rp(f)
        g_p = self.Rp(g)
        Fp = f_p.inverse_mod(g_p)

        h = Fq * g_q
        return [(f_p, Fp), h]


    def encryption(self, m: list[int], r: list[int], h: list[int]):
        """ Encryption for the NTRUEncrypt cipher

        Args:
            m (list[int]): coefficients of the message polynomial
            r (list[int]): coefficients of the random polynomial
            h (list[int]): coefficients of the public key

        Output:
            (polynomial): ciphertext
        """
        e = (self.p * self.Rq(r)) * self.Rq(h) + self.Rq(m)
        return e


    def decryption(self, f: list[int], e: list[int], Fp: list[int]):
        """ Decryption for the NTRUEncrypt cipher

        Args:
            f (list[int]): coefficients of the f polynomial [first part of the private key]
            e (list[int]): coefficients of the ciphertext polynomial
            Fp (list[int]): coefficients of the Fp polyonmial [second part of the private key]

        Output:
            (polynomial): plaintext
        """
        temp = self.Rq(f) * self.Rq(e)
        # Center-lift
        a: list[int] = temp.lift().map_coefficients(lambda c: c.lift_centered(), ZZ).list()
        m = self.Rp(Fp) * self.Rp(a)
        # Center-lift
        print(m)
        return m.lift().map_coefficients(lambda c: c.lift_centered(), ZZ)



if __name__ == "__main__":
    # An introduction to mathematical cryptography (Second edition) [Example 7.53]
    N: int = 7
    p: int = 3
    q: int = 41
    d: int = 2
    ntru_encrypt: NTRUEncrypt = NTRUEncrypt(N, p, q, d)

    # Alice (Key creation)
    f: list[int] = [-1, 0, 1, 1, -1, 0, 1]
    g: list[int] = [0, -1, -1, 0, 1, 0, 1]
    priv_key, pub_key = ntru_encrypt.key_creation(f, g)
    print(f"Private Key: {priv_key}")
    print(f"Public Key: {pub_key}")

    # Bob (Encryption)
    m: list[int] = [1, -1, 1, 1, 0, -1]
    r: list[int] = [-1, 1, 0, 0, 0, -1, 1]
    ciphertext = ntru_encrypt.encryption(m, r, pub_key)
    print(f"Ciphertext: {ciphertext}")

    # Alice (Decryption)
    plaintext = ntru_encrypt.decryption(f, ciphertext.list(), priv_key[1].list())
    print(f"Plaintext: {plaintext}")
    print(plaintext.list() == m)
