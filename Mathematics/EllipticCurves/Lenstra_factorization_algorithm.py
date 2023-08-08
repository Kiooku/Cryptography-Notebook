from random import randint
from math import gcd
from Mathematics.EllipticCurves.elliptic_curves import EllipticCurves
from Mathematics.EllipticCurves.points import Point

# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"

def Lenstra_elliptic_curve_factorization_algorithm(N: int, bound: int = 100) -> int:
    """Lenstra's elliptic curve factorization algorithm
    
    Algorithm to factorize a number using elliptic curve

    Args:
        N (int): number to be factored
        bound (int, optional): boundary. Defaults to 100.

    Returns:
        int: factor
    """
    while True:
        A: int = randint(0, N - 1)
        a: int = randint(0, N - 1)
        b: int = randint(0, N - 1)
        P: Point = Point(a, b)
        B: int = (pow(b, 2) - pow(a, 3) - A * a) % N
        E: EllipticCurves = EllipticCurves(A, B, N)

        for j in range(2, bound):
            try:
                E.double_and_add(P, j)
            except ValueError as error:
                error_message = error.args[0]
                start_index = error_message.index('of') + 3
                end_index = error_message.index('(')
                d: int = gcd(int(error_message[start_index:end_index]), N)
                if 1 < d < N:
                    return d


if __name__ == "__main__":
    # An Introduction to Mathematical Cryptography (Second edition) (Example 6.22 / 6.24)
    # Example 1
    N1: int = 187
    res1: int = Lenstra_elliptic_curve_factorization_algorithm(N1)
    print(f"{N1} = {res1} * {N1 // res1}")

    # Example 2
    N2: int = 6887
    res2: int = Lenstra_elliptic_curve_factorization_algorithm(N2)
    print(f"{N2} = {res2} * {N2 // res2}")

    # Example 3
    N3: int = 9788111
    res3: int = Lenstra_elliptic_curve_factorization_algorithm(N3)
    print(f"{N3} = {res3} * {N3 // res3}")
