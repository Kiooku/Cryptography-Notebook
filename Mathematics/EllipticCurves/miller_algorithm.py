from sympy import mod_inverse
from Mathematics.EllipticCurves.elliptic_curves import EllipticCurves
from Mathematics.EllipticCurves.points import Point

# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"

def g(E: EllipticCurves, V: Point, P: Point, Q: Point) -> int:
    """
    Args:
        E (EllipticCurves): Elliptic curve
        V (Point): Point of the f_P
        P (Point): First point
        Q (Point): Second point

    Returns:
        int
    """
    res: int
    slope: int | None = E.slope(P, Q)
    if slope is not None:
        res = ((V.y - P.y - slope * (V.x - P.x)) * mod_inverse(V.x + P.x + Q.x - pow(slope, 2), E.F)) % E.F
    else:
        res = (V.x - P.x) % E.F

    return res


#https://crypto.stanford.edu/pbc/notes/ep/miller.html
#https://crypto.stanford.edu/miller/miller.pdf (3. Application - Weil pairing)
def miller_algorithm(E: EllipticCurves, P: Point, Q: Point, m: int) -> int:
    """ Miller algorithm

    Args:
        E (EllipticCurves): Elliptic curve
        P (Point): Point in F_P(Q)
        Q (Point): Point to compute
        m (int): order of the generator point

    Returns:
        int
    """
    T: Point = P
    f: int = 1
    n: list = list(bin(m)[2:])
    for i in n:
        f = pow(f, 2) * g(E, Q, T, T)
        T = E.addition(T, T)
        if i == "1":
            f = f * g(E, Q, T, P)
            T = E.addition(T, P)

    return f % E.F



if __name__ == "__main__":
    # An Introduction to Mathematical Cryptography (Second edition) (Example 6.43)
    E: EllipticCurves = EllipticCurves(30, 34, 631)
    P: Point = Point(36, 60)
    S: Point = Point(0, 36)
    Q: Point = Point(121, 387)
    m: int = 2 # Order of the generator point
    r1: int = miller_algorithm(E, P, E.addition(Q, S), m)
    print(r1, r1 == 103)

    r2: int = miller_algorithm(E, P, S, m)
    print(r2, r2 == 219)

    r3: int = miller_algorithm(E, Q, E.addition(P, E.minus(S)), m)
    print(r3, r3 == 284)

    r4: int = miller_algorithm(E, Q, E.minus(S), m)
    print(r4, r4 == 204)
