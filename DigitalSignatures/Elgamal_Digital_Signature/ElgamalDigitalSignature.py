from random import randint
from sympy import gcd

class Elgamal_Digital_Signature:
    def __init__(self, p: int, g: int, a: int):
        self.p = p
        self.g = g
        self.a = a
        self.A = pow(g, a, p)


    def get_verification_key(self)-> int:
        """ Return the verification key

        Returns:
            int: verification key
        """
        return self.A


    def signing(self, D: int)-> tuple[int, int]:
        """ Sign the document

        Args:
            D (int): Document

        Returns:
            tuple[int, int]: Signature (S1, S2)
        """
        new_D: int = D % self.p
        k: int = 2
        while gcd(k, self.p-1) != 1:
            k = randint(2, self.p - 1)
        S1: int = pow(self.g, k, self.p)
        S2: int = ((new_D - self.a * S1) * pow(k, -1, self.p-1)) % (self.p - 1)

        return (S1, S2)


class Elgamal_Digital_Signature_Verification:
    def __init__(self, p: int, g: int, verification_key: int):
        self.p = p
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
        res: int = (pow(self.A, signature[0]) * pow(signature[0], signature[1])) % self.p
        return pow(self.g, D, self.p) == res


if __name__ == "__main__":
    p: int = 21739
    g: int = 7
    D: int = 5331
    # Samantha
    a: int = 15140
    elgamal_digital_signature: Elgamal_Digital_Signature = Elgamal_Digital_Signature(p, g, a)
    signature: tuple[int, int] = elgamal_digital_signature.signing(D)

    # Victor
    verification_key: int = elgamal_digital_signature.get_verification_key()
    elgamal_digital_signature_verification: Elgamal_Digital_Signature_Verification = Elgamal_Digital_Signature_Verification(p, g, verification_key)
    is_sign: bool = elgamal_digital_signature_verification.verification(signature, D)
    print(f"Is D={D} the signed document by Samantha?\n{is_sign}")
