from sympy import isprime, gcd

class RSA_Digital_Signature:
    def __init__(self, p: int, q: int, e: int):
        if not isprime(p) or not isprime(q):
            raise Exception("'p' and 'q' need to be large prime number")

        self.totient: int = (p-1)*(q-1)

        if gcd(e, self.totient) != 1:
            raise Exception("gcd(e, (p-1)(q-1)) need to be equal to 1")

        if e < 3:
            raise Exception("'e' need to be at least greater or equal to 3 (e.g: e = 65537)")

        self.p = p
        self.q = q
        self.e = e
        self.N = p*q


    def get_verification_key(self) -> tuple[int, int]:
        """ Return the verification key

        Returns:
            tuple[int, int]: verification key (N, e)
        """
        return (self.N, self.e)


    def signing(self, D: int) -> int:
        """ Sign the document

        Args:
            D (int): Document

        Returns:
            int: Document signed
        """
        d: int = pow(self.e, -1, self.totient)
        S: int = pow(D, d, self.N)
        return S



class RSA_Digital_Signature_Verificaton:
    def __init__(self, verification_key: tuple[int, int]):
        self.e = verification_key[1]
        self.N = verification_key[0]


    def verification(self, S: int, D: int) -> bool:
        """ Check if the document 'D' is signed by the signature 'S'

        Args:
            S (int): Signature
            D (int): Document

        Returns:
            bool
        """
        return pow(S, self.e, self.N) == D


if __name__ == "__main__":
    # Samantha (RSA Signing)
    p: int = 1223
    q: int = 1987
    e: int = 948047
    rsa_digital_signature: RSA_Digital_Signature = RSA_Digital_Signature(p, q, e)
    D: int = 1070777
    S: int = rsa_digital_signature.signing(D)

    # Victor (RSA Verification)
    verification_key: tuple[int, int] = rsa_digital_signature.get_verification_key()
    rsa_digital_signature_verification: RSA_Digital_Signature_Verificaton = RSA_Digital_Signature_Verificaton(verification_key)
    is_sign: bool = rsa_digital_signature_verification.verification(S, D)
    print(f"Is D={D} the signed document by Samantha?\n{is_sign}")
