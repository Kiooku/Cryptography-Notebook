load("Mathematics/Lattice/lattice.sage")
load("AsymmetricCiphers/GGH_cryptosystem/ggh.sage")

class GGHDigitalSignature(GGH):
    """
    """

    def key_creation(self, private_key: Lattice, U: Matrix = None) -> Lattice:
        """
        """
        return GGH.key_creation(self, private_key, U)

    
    def signing(self, d: vector, public_key: Lattice, private_key: Lattice) -> vector:
        """
        """
        s: vector = alice_private_key.babai_algorithm(d)
        return public_key.basis.solve_left(s)


    def verification(self, s: vector, public_key: Lattice) -> vector:
        """
        """
        return s * public_key.basis


if __name__ == "__main__":
    print("=== GGH Digital Signature output ===")
    # An introduction to mathematical cryptography (Second edition) [Example 7.36]
    ggh_digital_signature: GGHDigitalSignature = GGHDigitalSignature()

    # Samantha (Key creation)
    v1: vector = vector([-97, 19, 19])
    v2: vector = vector([-36, 30, 86])
    v3: vector = vector([-184, -64, 78])
    samantha_private_key: Lattice = Lattice(Matrix([v1, v2, v3]))
    U: Matrix = Matrix([[4327, -15447, 23454], [3297, -11770, 17871], [5464, -19506, 29617]])
    
    samantha_public_key: Lattice = ggh_digital_signature.key_creation(samantha_private_key, U)
    print(f"Public key:\n{samantha_public_key}\n")

    # Samantha (Signing)
    d: vector = vector([678846, 651685, 160467])
    
    signature: vector = ggh_digital_signature.signing(d, samantha_public_key, samantha_private_key)
    print(f"Signature: {signature}")

    # Victor (Verification)
    verification: vector = ggh_digital_signature.verification(signature, samantha_public_key)
    print(verification, round((verification - d).norm(), 2))
    is_sign: bool = float((verification - d).norm()) < 50
    print(f"Is d={d} the signed document by Samantha?\n{is_sign}")