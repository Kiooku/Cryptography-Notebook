from Mathematics.EllipticCurves.miller_algorithm import miller_algorithm
from Mathematics.EllipticCurves.elliptic_curves import EllipticCurves
from Mathematics.EllipticCurves.points import Point
from sympy import mod_inverse

# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"

def weil_pairing(m: int, P: Point, Q: Point, S: Point, E: EllipticCurves) -> int:
    """ Weil pairing algorithm

    Args:
        m (int): order of the generator point
        P (Point): First point
        Q (Point): Second point
        S (Point): Point that is not in the subgroup of P and Q
        E (EllipticCurves): Elliptic curve

    Returns:
        int: pairing of the points P and Q
    """
    r1: int = (miller_algorithm(E, P, E.addition(Q, S), m) * mod_inverse(miller_algorithm(E, P, S, m), E.F)) % E.F
    r2: int = (miller_algorithm(E, Q, E.addition(P, E.minus(S)), m) * mod_inverse(miller_algorithm(E, Q, E.minus(S), m), E.F)) % E.F
    print(r1, r2)
    return (r1 * mod_inverse(r2, E.F)) % E.F

if __name__ == "__main__":
    # An Introduction to Mathematical Cryptography (Second edition) (Example 6.43)
    E: EllipticCurves = EllipticCurves(30, 34, 631)
    P: Point = Point(36, 60)
    S: Point = Point(0, 36)
    Q: Point = Point(121, 387)
    m: int = 2

    res1: int = weil_pairing(m, P, Q, S, E)
    print(res1, res1 == 242)

    P_bis = Point(617, 5)
    Q_bis = Point(121, 244)

    res2: int = weil_pairing(m, P_bis, Q_bis, S, E)
    print(res2, res2 == 512)
