from random import randint
from hashlib import sha256

# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"

class ECDSA:
    def __init__(self, E, G, q: int) -> None:
        self.E = E
        self.G = G
        self.q: int = q
        self.s: int = randint(2, self.q - 2)


    def key_creation(self):
        """ Create the verification key

        Returns:
            Point: verification key
        """
        self.s = randint(2, self.q - 2)
        V = self.s * self.G
        return V


    def signing(self, D: int) -> tuple[int, int]:
        """ Sign a document

        Args:
            D (int): document

        Returns:
            tuple[int, int]: signature
        """
        new_D: int = D % self.q
        e: int = randint(2, self.q - 1)
        eG = e * self.G
        s1: int = int(eG[0]) % self.q
        s2: int = ((new_D + self.s*s1) * pow(e, -1, self.q)) % self.q

        return (s1, s2)


class ECDSA_verification:
    def __init__(self, E, G, q: int) -> None:
        self.E = E
        self.G = G
        self.q: int = q


    def verification(self, D: int, signature: tuple[int, int], V) -> bool:
        """ Verification of the signed document

        Args:
            D (int): document
            signature (tuple[int, int]): signature
            V (Point): verification key

        Returns:
            bool: True -> Valid; False: Invalid
        """
        v1: int = (D * pow(signature[1], -1, self.q)) % self.q
        v2: int = (signature[0] * pow(signature[1], -1, self.q)) % self.q
        temp = (int(v1) * self.G) + (int(v2) * V)
        return (int(temp[0]) % self.q) == signature[0]


if __name__ == "__main__":
    # An Introduction to Mathematical Cryptography (Second edition)
    # secp256k1 (Source: https://www.secg.org/sec2-v2.pdf)
    # Setting (Trusted party)
    p: int = int("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16)
    a: int = int("0000000000000000000000000000000000000000000000000000000000000000", 16)
    b: int = int("0000000000000000000000000000000000000000000000000000000000000007", 16)
    E = EllipticCurve(GF(p), [a, b])
    G = E([int("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16),
                int("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16)])
    q: int = int("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16)

    # Samantha want to sign the document
    document: int = int(sha256("Lenstra".encode()).hexdigest(), 16)
    ecdsa: ECDSA = ECDSA(E, G, q)
    verification_key = ecdsa.key_creation()
    signature: tuple[int, int] = ecdsa.signing(document)
    print(f"Verification key: {verification_key}\n\nSignature: \n\t- s1: {signature[0]}\n\t- s2: {signature[1]}")

    # Victor want to verify the document
    ecdsa_verification: ECDSA_verification = ECDSA_verification(E, G, q)
    is_sign: bool = ecdsa_verification.verification(document, signature, verification_key)

    print(f"\nIs D={document} the signed document by Samantha?\n{is_sign}")
