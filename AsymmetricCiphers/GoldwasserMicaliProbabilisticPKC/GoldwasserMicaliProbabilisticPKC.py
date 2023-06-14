from random import randint

class GoldwasserMicaliProbabilisticPKC:
    def __init__(self, p: int, q: int, a: int) -> None:
        if self.legendre_symbol(a, p) != -1 or self.legendre_symbol(a, q) != -1:
            raise Exception("'a' should have the following property: (a/p) = (a/q) = -1")
        self.p = p
        self.q = q
        self.a = a
        self.N = p*q


    def get_public_key(self) -> tuple[int, int]:
        """ Return the public key (N, a)

        Returns:
            tuple[int, int]: public key
        """
        return (self.N, self.a)


    def encryption(self, m: int, public_key: tuple[int, int]) -> int:
        """ Encryption process

        Args:
            m (int): plaintext (0 or 1)
            public_key (tuple[int, int]): public key

        Returns:
            int: ciphertext
        """
        if 0 < m < 1:
            raise Exception("'m' should be in {0, 1}")

        c: int
        N: int = public_key[0]
        a: int = public_key[1]
        r: int = randint(2, N-1)
        if m == 0:
            c = pow(r, 2, N)
        else:
            c = (a * pow(r, 2)) % N

        return c


    def decryption(self, c: int) -> int:
        c_p: int = self.legendre_symbol(c , self.p)
        return 0 if c_p == 1 else 1


    def legendre_symbol(self, a: int, p: int) -> int:
        temp: int = pow(a, (p-1)//2, p)
        return -1 if temp == p-1 else temp



if __name__ == "__main__":
    # Oracle
    p: int = 2309
    q: int = 5651
    a: int =  6283665
    goldwassser_micali_probabilistic_pkc: GoldwasserMicaliProbabilisticPKC = GoldwasserMicaliProbabilisticPKC(p, q, a)

    # Alice
    m0: int = 0
    m1: int = 1
    oracle_public_key: tuple[int, int] = goldwassser_micali_probabilistic_pkc.get_public_key()
    for k in range(10):
        print(f"[{k}]\n")
        r0: int = goldwassser_micali_probabilistic_pkc.encryption(m0, oracle_public_key)
        print(f"Encryption of 0 (c0) equal to {r0}.")
        r1: int = goldwassser_micali_probabilistic_pkc.encryption(m1, oracle_public_key)
        print(f"Encryption of 1 (c1) equal to {r1}.")
        print(f"Decryption of c0 successful? {goldwassser_micali_probabilistic_pkc.decryption(r0) == 0}")
        print(f"Decryption of c1 successful? {goldwassser_micali_probabilistic_pkc.decryption(r1) == 1}\n")
        print("="*25)
        print("\n")