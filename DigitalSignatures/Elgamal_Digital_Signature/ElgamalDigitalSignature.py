from random import randint
from sympy import gcd

class Elgamal_Digital_Signature:
    def __init__(self, p: int, g: int, a: int):
        self.p = p
        self.g = g
        self.a = a
        self.A = pow(g, a, p)


    def get_public_key(self)-> int:
        """ Return the public key

        Returns:
            int: public key
        """
        return self.A


    def signing(self, D: int)-> tuple[int, int]:
        new_D: int = D % self.p
        k: int = 2
        while gcd(k, self.p-1) != 1:
            k = randint(2, self.p - 1)
        S1: int = pow(self.g, k, self.p)
        S2: int = ((new_D - self.a * S1) * pow(k, -1, self.p-1)) % (self.p - 1)

        return (S1, S2)


class Elgamal_Digital_Signature_Verification:
    def __init__(self, p: int, g: int, public_key: int):
        self.p = p
        self.g = g
        self.A = public_key


    def verification(self, signature: tuple[int, int], D: int) -> bool:
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
    public_key: int = elgamal_digital_signature.get_public_key()
    elgamal_digital_signature_verification: Elgamal_Digital_Signature_Verification = Elgamal_Digital_Signature_Verification(p, g, public_key)
    is_sign: bool = elgamal_digital_signature_verification.verification(signature, D)
    print(f"Is D={D} the signed document by Samantha?\n{is_sign}")