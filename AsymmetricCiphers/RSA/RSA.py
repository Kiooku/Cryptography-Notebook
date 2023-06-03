from sympy import isprime, gcd

class RSA:
    def __init__(self, p: int, q: int, e: int):
        if not isprime(p) or not isprime(q):
            raise Exception("'p' and 'q' need to be large prime number")

        self.totient: int = (p-1)*(q-1)

        if gcd(e, self.totient) != 1:
            raise Exception("gcd(e, (p-1)(q-1)) need to be equal to 1")

        if e < 3:
            raise Exception("'e' need to be at least greater or equal to 3 (e.g: e = 65537)")

        self.p: int = p
        self.q: int = q
        self.e: int = e
        self.N: int = p*q


    def get_public_key(self)->tuple[int, int]:
        """ Return the public key

        Returns:
            tuple[int, int]: public key (N,e)
        """
        return (self.N, self.e)


    def encryption(self, m: str, public_key: tuple[int, int])-> str:
        """ Encryption using RSA

        Args:
            m (str): plaintext
            public_key (tuple[int, int]): public key (N,e)

        Returns:
            str: ciphertext
        """
        m_converted: int = int(m.encode().hex(), 16)
        c: int = pow(m_converted, public_key[1], public_key[0])
        return hex(c)


    def decryption(self, c: str)-> str:
        """ Decryption usin RSA

        Args:
            c (str): ciphertext

        Returns:
            str: plaintext
        """
        d: int = pow(self.e, -1, self.totient)
        m: int = pow(int(c, 16), d, self.N)
        return bytes.fromhex(hex(m)[2:]).decode()


if __name__ == "__main__":
    ### An Introduction to Mathematical Cryptography (Second edition) (Example 3.9, page 124)

    # Bob RSA Key Creation
    p: int = 1223
    q: int = 1987
    e: int = 948047
    bob_rsa: RSA = RSA(p, q, e)

    # Alice RSA Encryption
    m: str = "A2" # Need to increase p and q to encrypt larger messages
    alice_rsa: RSA = RSA(3, 11, 7)
    c: str = alice_rsa.encryption(m, bob_rsa.get_public_key())
    print(f"Ciphertext: {c}")

    # Bob RSA Decryption
    m_prime: str = bob_rsa.decryption(c)
    print(f"m = {m_prime}")

    ## Test init exceptions
    print("\n== TEST ==")
    try:
        RSA(61, 53, 1)
    except Exception as err:
        print(err)
    
    try:
        RSA(3, 4, 1)
    except Exception as err:
        print(err)
        
    try:
        RSA(61, 53, 106)
    except Exception as err:
        print(err)