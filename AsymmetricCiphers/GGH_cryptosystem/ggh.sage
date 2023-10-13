from sage.matrix.constructor import random_unimodular_matrix
load("Mathematics/Lattice/lattice.sage")

class GGH:
    """ GGH cryptosystem
    """
    def generate_private_basis(self, size: int, d: int) -> Lattice:
        """ Generate an orthogonal basis for GGH private key

        Args:
            size (int): size of the square matrix for the private key
            d (int): parameter to create the basis with coordinate between '-d' and 'd'

        Output:
            (Lattice): private basis
        """
        while True:
            vectors: list[vector] = [vector([randint(-d, d) for _ in range(size)]) for _ in range(size)]
            res: Lattice = Lattice(Matrix(vectors))
            if round(res.hadamard_ratio(*vectors), 2) > 0.70:
                return res


    def key_creation(self, private_key: Lattice, U: Matrix = None) -> Lattice:
        """ Key creation for the GGH cryptosystem

        Args:
            private_key (Lattice): Private key (Orthogonal basis)
            U (Matrix): Unimodular matrix

        Output:
            public_key (Lattice): public key (non-orthogonal basis)
        """
        if U == None:
            U = random_unimodular_matrix(MatrixSpace(ZZ, len(private_key.basis[0])))
        
        assert len(private_key.basis[0]) == len(U[0])
        assert abs(det(U)) == 1

        return Lattice(U * private_key.basis)

    
    def encryption(self, public_key: Lattice, m: vector) -> vector:
        """ GGH encryption

        Args:
            public_key (Lattice): public key of the other party
            m (vector): small plaintext vector

        Output:
            ciphertext (vector): small ciphertext vector
        """
        delta: int = 10 # Need to check about security (seems too small)
        r: vector = vector([randint(-delta, delta) for _ in range(len(m))])
        return m * public_key.basis + r

    
    def decryption(self, private_key: Lattice, public_key: Lattice, c: vector) -> vector:
        """ GGH decryption

        Args:
            private_key (Lattice): private key
            public_key (Lattice): public key
            c (vector): small ciphertext vector

        Output:
            plaintext (vector): small plaintext vector
        """
        v: vector = private_key.babai_algorithm(c)
        return v * public_key.basis.inverse()


if __name__ == "__main__":
    print("=== GGH output ===")
    # An introduction to mathematical cryptography (Second edition) [Example 7.36]
    ggh: GGH = GGH()

    # Alice (Key creation)
    v1: vector = vector([-97, 19, 19])
    v2: vector = vector([-36, 30, 86])
    v3: vector = vector([-184, -64, 78])
    alice_private_key: Lattice = Lattice(Matrix([v1, v2, v3]))
    print(f"Hadamard ratio of Alice private key: {round(alice_private_key.hadamard_ratio(v1 , v2, v3), 5)}")
    
    U: Matrix = Matrix([[4327, -15447, 23454], [3297, -11770, 17871], [5464, -19506, 29617]])
    alice_public_key: Lattice = ggh.key_creation(alice_private_key, U)

    print(f"\nAlice public key:\n{alice_public_key}\n")
    print(f"Hadamard ratio of Alice public key: {round(alice_public_key.hadamard_ratio(alice_public_key.basis[0], alice_public_key.basis[1], alice_public_key.basis[2]), 7)}")

    # Bob (Encryption)
    m: vector = vector([86, -35, -32])
    bob_ciphertext: vector = ggh.encryption(alice_public_key, m)
    print(f"\nBob ciphertext: {bob_ciphertext}\n")

    # Alice (Decryption)
    plaintext: vector = ggh.decryption(alice_private_key, alice_public_key, bob_ciphertext)
    print(f"Alice decryption of Bob message: {plaintext} (Success: {plaintext == m})\n")

    # Test GGH function
    test_private_key_generation: Lattice = ggh.generate_private_basis(3, 200)
    print("*-" * 20)
    print(test_private_key_generation)
