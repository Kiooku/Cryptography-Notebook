from random import randint

class DSA:
    def __init__(self, p: int, q: int, g: int, a: int):
        self.p = p
        self.q = q
        self.g = g
        if 1 > a > q-1:
            raise Exception("'a' need to be 1 <= a <= q-1")
        self.a = a
        self.A = pow(g, a, p)


    def get_verification_key(self) -> int:
        """ Return the verification key

        Returns:
            int: verification key
        """
        return self.A


    def signing(self, D: int) -> tuple[int, int]:
        """ Sign the document

        Args:
            D (int): Document

        Returns:
            tuple[int, int]: Signature (S1, S2)
        """
        new_D: int = D % self.q
        k: int = randint(2, self.q - 1)
        S1: int = pow(self.g, k, self.p) % self.q
        S2: int = ((new_D + self.a * S1) * pow(k, -1, self.q)) % self.q
        return (S1, S2)


class DSA_verification:
    def __init__(self, verification_key: int, p: int, q: int, g: int):
        self.p = p
        self.q = q
        self.g = g
        self.A = verification_key


    def verification(self, signature: tuple[int, int], D: int) -> bool:
        """Check if the document 'D' is signed by the signature (S1, S2)

        Args:
            signature (tuple[int, int]): Signature
            D (int): Document

        Returns:
            bool
        """
        V1: int = (D * pow(signature[1], -1, self.q)) % self.q
        V2: int = (signature[0] * pow(signature[1], -1, self.q)) % q
        return ((pow(self.g, V1, self.p) * pow(self.A, V2, self.p)) % self.q) == signature[0]


if __name__ == "__main__":
    p: int = 48731
    q: int = 443
    g: int = 5260
    # Samantha
    a: int = 242
    dsa: DSA = DSA(p, q, g, a)
    D: int = 343
    signature: tuple[int, int] = dsa.signing(D)

    # Victor
    verification_key: int = dsa.get_verification_key()
    dsa_verification: DSA_verification = DSA_verification(verification_key, p, q, g)
    is_sign: bool = dsa_verification.verification(signature, D)
    print(f"Is D={D} the signed document by Samantha?\n{is_sign}")
