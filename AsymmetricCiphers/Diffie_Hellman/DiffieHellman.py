from copy import Error
from re import A
from sympy import isprime
from os import linesep

class DiffieHellman:
    def __init__(self, p: int, g: int):
        if not isprime(p):
            raise Exception("'p' need to be a prime number")
        self.p = p
        self.g = g

    def create_public_key(self, secret: int)-> int:
        return pow(self.g, secret, self.p)

    def create_share_secret(self, k_pub: int, k_priv: int)-> int:
        return pow(k_pub, k_priv, self.p)
    
if __name__ == "__main__":
    # An Introduction to Mathematical Cryptography (Second edition) (Section 2.3)
    p: int = 941
    g: int = 627

    dh: DiffieHellman = DiffieHellman(p, g)

    a_priv: int = 347
    A_pub: int = dh.create_public_key(a_priv)

    b_priv: int = 781
    B_pub: int = dh.create_public_key(b_priv)

    alice_shared_secret: int = dh.create_share_secret(B_pub, a_priv)
    bob_shared_secret: int = dh.create_share_secret(A_pub, b_priv)

    print(f"p: {p}; g: {g}",
          f"Alice private key: {a_priv}; Alice public key: {A_pub}",
          f"Bob private key: {b_priv}; Bob public key: {B_pub}",
          f"Shared secret: {alice_shared_secret}",
          sep=linesep)

    print(f"Alice shared a secret with Bob: {alice_shared_secret == bob_shared_secret}")

    try:
        dh_err: DiffieHellman = DiffieHellman(1000, 627)
    except Exception:
        print("Prime number detection works!")