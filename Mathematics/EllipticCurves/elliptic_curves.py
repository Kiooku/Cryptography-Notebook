from math import floor
from typing import Union
from sympy import mod_inverse
from Mathematics.EllipticCurves.points import Point

# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"

class EllipticCurves:
    """Elliptic Curves over Finite Fields for cryptography purpose
    """

    def __init__(self, A: int, B: int, F: int) -> None:
        self.discriminant = 4 * pow(A, 3) + 27 * pow(B, 2)
        if self.discriminant == 0:
            raise Exception("4A³ + 27B² should not be equal to 0")

        self.A: int = A
        self.B: int = B
        self.F: int = F
        self.O: Point = Point(0, 0)


    def minus(self, P: Point) -> Point:
        """ Return the minus of the point

        Args:
            P (Point): point

        Returns:
            Point: minus of the point 'P'
        """
        return Point(P.x, -P.y)


    def addition(self, P1: Point, P2: Point) -> Point:
        """ Return the addition of two point

        Args:
            P1 (Point): first point
            P2 (Point): second point

        Returns:
            Point: result
        """
        if P1 == self.O:
            return P2
        elif P2 == self.O:
            return P1
        elif P1 == self.minus(P2):
            return self.O

        l: int = 0
        if P1 != P2:
            l = ((P2.y - P1.y) * mod_inverse(P2.x - P1.x, self.F)) % self.F
        else:
            l = ((3*pow(P1.x, 2) + self.A) * mod_inverse(2*P1.y, self.F)) % self.F

        x3: int = (pow(l, 2) - P1.x - P2.x) % self.F
        y3: int = (l * (P1.x - x3) - P1.y) % self.F

        return Point(x3, y3)


    def double_and_add(self, P: Point, n: int) -> Point:
        """ Double-And-Add algorithm

        Args:
            P (Point): point
            n (int): coefficient

        Raises:
            Exception: 'n' sould be greater or equal to 1

        Returns:
            Point: result
        """
        if n < 1:
            raise Exception("'n' should be greater or equal to 1")

        if not self.belongs_to(P):
            raise Exception("'P' should belongs to the elliptic curve E")

        Q: Point = P
        R: Point = self.O

        while n > 0:
            if n % 2 == 1:
                R = self.addition(R, Q)

            Q = self.addition(Q, Q)
            n = floor(n/2)

        return R


    def belongs_to(self, P: Point) -> bool:
        """ Define if the point is on the elliptic curve

        Args:
            P (Point): point

        Returns:
            bool: True -> belong; False -> doesn't belong
        """
        return pow(P.y, 2, self.F) == (pow(P.x, 3, self.F) + self.A * P.x + self.B) % self.F


    def slope(self, P1: Point, P2: Point) -> Union[int, None]:
        """ Give the slope between two points

        Args:
            P1 (Point): first point
            P2 (Point): second point

        Returns:
            Union[int, None]: slope
        """
        if P1 == self.minus(P2):
            return None
        elif P1 == P2:
            return ((3*pow(P1.x, 2) + self.A) * mod_inverse(2*P1.y, self.F)) % self.F

        return ((P2.y - P1.y) * mod_inverse(P2.x - P1.x, self.F)) % self.F


    def order(self, P: Point) -> int:
        """ Give the order of a point

        Args:
            P (Point): point

        Returns:
            int: order
        """
        n: int = 2
        try:
            while self.double_and_add(P, n) != self.O:
                n += 1
        except ValueError:
            pass

        return n


    def __str__(self) -> str:
        if self.A != 0 and self.B != 0:
            return f"(E_{self.F}): Y² = X³ + {self.A}X + {self.B}"
        elif self.A == 0:
            return f"(E_{self.F}): Y² = X³ + {self.B}"
        return f"(E_{self.F}): Y² = X³ + {self.A}X"


if __name__ == "__main__":
    # An Introduction to Mathematical Cryptography (Second edition) (Example 6.10)
    P: Point = Point(9, 7)
    Q: Point = Point(1, 8)
    E1: EllipticCurves = EllipticCurves(3, 8, 13)
    print("=== Addition and Substraction ===")
    print("Substraction:", E1.minus(P) == Point(9, -7), E1.addition(P, E1.minus(P)) == E1.O)
    res1: Point = E1.addition(P, Q)
    print("Addition:", res1, res1 == Point(2, 10))

    print("== Double-and-Add Algorithm ===")
    n: int = 947
    p: int = 3623
    P: Point = Point(6, 730)
    E2: EllipticCurves = EllipticCurves(14, 19, p)
    expected_res: Point = Point(3492, 60)
    res2: Point = E2.double_and_add(P, n)
    print(res2, res2 == expected_res)

    print(E2)
