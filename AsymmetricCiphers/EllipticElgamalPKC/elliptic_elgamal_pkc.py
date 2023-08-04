from Mathematics.EllipticCurves.elliptic_curves import EllipticCurves
from Mathematics.EllipticCurves.points import Point

# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"

class ElgamalECC:
    """ Elliptic Elgamal Public Key Cryptography
    """
    def __init__(self, E: EllipticCurves, P: Point, n: int) -> None:
        self.E: EllipticCurves = E
        self.P: Point = P
        self.n: int = n
        self.Q: Point = self.E.O


    def get_public_key(self) -> Point:
        """ Return the public key

        Returns:
            Point: public key
        """
        if self.Q == self.E.O:
            self.Q = self.E.double_and_add(self.P, self.n)

        return self.Q


    def encryption(self, M: Point, k: int, Q: Point) -> tuple[Point, Point]:
        """ Encryption algorithm

        Args:
            M (Point): plaintext
            k (int): random number
            Q (Point): public key

        Returns:
            tuple[Point, Point]: ciphertext
        """
        C1: Point = self.E.double_and_add(self.P, k)
        C2: Point = self.E.addition(M, self.E.double_and_add(Q, k))

        return (C1, C2)


    def decryption(self, ciphertext: tuple[Point, Point]) -> Point:
        """ Decryption algorithm

        Args:
            ciphertext (tuple[Point, Point]): ciphertext

        Returns:
            Point: plaintext
        """
        M: Point = self.E.addition(ciphertext[1], self.E.minus(self.E.double_and_add(ciphertext[0], self.n)))
        return M


if __name__ == "__main__":
    # Setting
    p: int = 3851
    E: EllipticCurves = EllipticCurves(324, 1287, p)
    P: Point = Point(920, 303)

    # Alice public key creation
    nA1: int = 1194
    alice_elliptic_elgamal: ElgamalECC = ElgamalECC(E, P, nA1)
    alice_public_key: Point = alice_elliptic_elgamal.get_public_key()

    # Bob encryption
    nB1: int = 1759
    k: int = 1446
    M: Point = Point(123, 789)
    bob_elliptic_elgamal: ElgamalECC = ElgamalECC(E, P, nB1)
    ciphertext: tuple[Point, Point] = bob_elliptic_elgamal.encryption(M, k, alice_public_key)

    # Alice decryption
    plaintext: Point = alice_elliptic_elgamal.decryption(ciphertext)

    print(f"Plaintext: {M}")
    print(f"Ciphertext: ({ciphertext[0]}, {ciphertext[1]})")
    print(f"Decipher text: {plaintext}; Success: {plaintext == M}")
