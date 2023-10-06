class TripartiteDiffieHellman:
    """ Tripartite Diffie Hellman Key Exchange
    """
    def __init__(self, E, P):
        self.E = E
        self.P = P
        self.order = P.order()

    
    def modified_weil_pairing(self, Q, Q_prime):
        """ Modified Weil-Pairing (l-distortion map)
        
        φ(x, y) = (-x, iy) AND  φ(O) = O
        Note: i² = −1
        ê_l(Q, Q' ) = e_l Q, φ(Q')
        ê_l(Q, Q') = 1 if and only if Q = O or Q' = O

        Args:
            Q (Point): First Point
            Q_prime (Point): Second Point

        Returns:
            res: result of the modified weil pairing
        """
        return Q.weil_pairing(self.E(-Q_prime[0], Q_prime[1] * i), self.order)

    
    def get_public_key(self, private_key: int):
        """ Tripartite Diffie-Hellman key creation

        Args:
            secret (int): private key

        Returns:
            Point: public key
        """
        return private_key * self.P

    
    def get_shared_secret(self, k_pub1, k_pub2, k_priv):
        """ Tripartite Diffie-Hellman share secret creation

        Args:
            k_pub1 (Point): other public key
            k_pub2 (Point): other public key
            k_priv (int): own private key

        Returns:
            Point: shared secret
        """
        return pow(self.modified_weil_pairing(k_pub1, k_pub2), k_priv)



if __name__ == "__main__":
    # An Introduction to Mathematical Cryptography (Second edition) (Example 6.57)
    p: int = 1303
    assert p % 4 == 3
    a, b = 1, 0
    Fp2.<i> = GF(p^2, modulus=x^2+1) # modulues=x^2+1 allow to have i^2 = -1
    assert i^2 == -1
    E = EllipticCurve(Fp2, [1, 0])
    P = E.random_point() #E(334, 920)
    while not P.order().is_prime():
        P = E.random_point()

    tripartite_diffie_hellman: TripartiteDiffieHellman = TripartiteDiffieHellman(E, P)

    # Alice
    nA: int = 71
    Qa = tripartite_diffie_hellman.get_public_key(nA)
    print(f"Alice private key: {nA}; Alice public key: {Qa}\n")

    # Bob
    nB: int = 3
    Qb = tripartite_diffie_hellman.get_public_key(nB)
    print(f"Bob private key: {nB}; Bob public key: {Qb}\n")

    # Carl
    nC: int = 126
    Qc = tripartite_diffie_hellman.get_public_key(nC)
    print(f"Carl private key: {nC}; Carl public key: {Qc}\n")

    # Shared secret
    alice_shared_secret = tripartite_diffie_hellman.get_shared_secret(Qb, Qc, nA)
    bob_shared_secret = tripartite_diffie_hellman.get_shared_secret(Qa, Qc, nB)
    carl_shared_secret = tripartite_diffie_hellman.get_shared_secret(Qa, Qb, nC)

    assert alice_shared_secret == bob_shared_secret == carl_shared_secret
    print(f"Shared secret: {alice_shared_secret}")
    
    # Example to understand how to do modified weil pairing with sagemath (No link with Tripartite Diffie-Hellman)
    # https://eprint.iacr.org/2022/1283.pdf
    # https://github.com/GiacomoPope/Castryck-Decru-SageMath (https://github.com/GiacomoPope/Castryck-Decru-SageMath/blob/main/SIKEp434.sage ||| https://github.com/GiacomoPope/Castryck-Decru-SageMath/blob/main/public_values_aux.py)
    # https://math.mit.edu/~drew/CastryckDecruSikeAttack.pdf
