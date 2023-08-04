from Mathematics.EllipticCurves.points import Point
from Mathematics.EllipticCurves.elliptic_curves import EllipticCurves

# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"

class EllipticDiffieHellmanKeyExchange:
    def __init__(self, E: EllipticCurves, P: Point, n: int) -> None:
        self.E: EllipticCurves = E
        self.P: Point = P
        self.n: int = n
        self.Q: Point = self.E.O


    def get_public_key(self) -> Point:
        if self.Q == self.E.O:
            self.Q = self.E.double_and_add(self.P, self.n)

        return self.Q


    def get_shared_secret_point(self, Q_prime: Point) -> Point:
        return self.E.double_and_add(Q_prime, self.n)


    def get_shared_secret(self, x: int) -> int:
        y_square: int = (pow(x, 3) + self.E.A * x + self.E.B) % self.E.F
        y: int = pow(y_square, (self.E.F + 1) // 4, self.E.F)
        return self.get_shared_secret_point(Point(x, y)).get_x()


if __name__ == "__main__":
    # Example 1: An Introduction to Mathematical Cryptography (Second edition) (Example 6.19)
    # Setting
    p: int = 3851
    E: EllipticCurves = EllipticCurves(324, 1287, p)
    P: Point = Point(920, 303)

    # Alice
    nA1: int = 1194
    alice_elliptic_diffie_hellman: EllipticDiffieHellmanKeyExchange = EllipticDiffieHellmanKeyExchange(E, P, nA1)
    alice_public_key: Point = alice_elliptic_diffie_hellman.get_public_key()
    # Bob
    nB1: int = 1759
    bob_elliptic_diffie_hellman: EllipticDiffieHellmanKeyExchange = EllipticDiffieHellmanKeyExchange(E, P, nB1)
    bob_public_key: Point = bob_elliptic_diffie_hellman.get_public_key()

    # Shared secret
    alice_shared_secret: Point = alice_elliptic_diffie_hellman.get_shared_secret_point(bob_public_key)
    bob_shared_secret: Point = bob_elliptic_diffie_hellman.get_shared_secret_point(alice_public_key)

    print(f"Alice public key: {alice_public_key}; Bob public key: {bob_public_key}")
    print(f"Bob shared secret: {bob_shared_secret}")
    print(f"Alice shared secret: {alice_shared_secret}")
    print(f"Elliptic Diffie-Hellman Key Exchange - Success: {alice_shared_secret == bob_shared_secret}")
    print("="*25)

    # Example 2: An Introduction to Mathematical Cryptography (Second edition) (Example 6.21)
    # Alice
    nA2: int = 2489
    alice_elliptic_diffie_hellman2: EllipticDiffieHellmanKeyExchange = EllipticDiffieHellmanKeyExchange(E, P, nA2)
    alice_public_key2: int = alice_elliptic_diffie_hellman2.get_public_key().get_x()
    # Bob
    nB2: int = 2286
    bob_elliptic_diffie_hellman2: EllipticDiffieHellmanKeyExchange = EllipticDiffieHellmanKeyExchange(E, P, nB2)
    bob_public_key2: int = bob_elliptic_diffie_hellman2.get_public_key().get_x()

    # Shared secret
    alice_shared_secret2: int = alice_elliptic_diffie_hellman2.get_shared_secret(bob_public_key2)
    bob_shared_secret2: int = bob_elliptic_diffie_hellman2.get_shared_secret(alice_public_key2)

    print(f"Alice public key: {alice_public_key2}; Bob public key: {bob_public_key2}")
    print(f"Bob shared secret: {bob_shared_secret2}")
    print(f"Alice shared secret: {alice_shared_secret2}")
    print(f"Elliptic Diffie-Hellman Key Exchange - Success: {alice_shared_secret2 == bob_shared_secret2}")