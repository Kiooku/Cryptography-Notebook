class MerkleHellman:
    """ The Merkle–Hellman subset-sum cryptosystem
    """

    def key_creation(self, r: list[int], A: int, B: int) -> list[int]:
        """ Key creation for the Merkle-Hellman subset-sum cryptosystem

            Args:
                r (list[int]): superincreasing sequence
                A (int): large secret integer
                B (int): large secret integer

            Returns:
                list[int]: public key (not superincreasing sequence)
        """
        if gcd(A, B) != 1:
            raise Exception("GCD(A, B) should be equal to 1")
        elif B <= 2 * r[-1]:
            raise Exception("B should be greater than 2*r_n")
        M: list[int] = []
        for r_i in r:
            M.append((r_i * A) % B)

        return M

    
    def encryption(self, M: list[int], x: list[int]) -> int:
        """ Encryption for the Merkle-Hellman subset-sum cryptosystem

            Args:
                M (list[int]): public key
                x (int): plaintext => binary vector

            Returns:
                int: ciphertext
        """
        assert len(x) == len(M)
        S: int = 0
        for i in range(len(M)):
            S += x[i] * M[i]
        
        return S


    def decryption(self, S: int, A: int, B: int, r: list[int]) -> list[int]:
        """ Decryption for the Merkle-Hellman subset-sum cryptosystem

            Args:
                S (int): ciphertext
                A (int): large secret integer
                B (int): large secret integer
                r (list[int]): superincreasing sequence

            Returns:
                list[int]: plaintext => binary vector
        """
        S_prime: int = (pow(A, -1, B) * S) % B
        plaintext: list[int] = []
        for i in range(len(r)-1, -1, -1):
            if S_prime >= r[i]:
                plaintext.append(1)
                S_prime -= r[i]
            else:
                plaintext.append(0)
        
        plaintext.reverse()
        return plaintext




if __name__ == "__main__":
    # An introduction to mathematical cryptography (Second edition) [Example 7.7]
    merkle_hellman: MerkleHellman = MerkleHellman()
    
    # Alice [Public key generation]
    r: list[int] = [3, 11, 24, 50, 115]
    A: int = 113
    B: int = 250

    M: list[int] = merkle_hellman.key_creation(r, A, B)
    print(f"Alice public key: {M}")

    # Bob [Encryption]
    x: list[int] = [1, 0, 1, 0, 1]
    S: int = merkle_hellman.encryption(M, x)
    print(f"Bob ciphertext: {S}")

    # Alice [Decryption]
    plaintext: list[int] = merkle_hellman.decryption(S, A, B, r)
    assert plaintext == x
    print(f"Bob plaintext: {plaintext}")
