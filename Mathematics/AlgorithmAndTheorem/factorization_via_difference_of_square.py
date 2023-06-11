from math import sqrt
from euclideanAlgorithm import gcd

def fermat_factorization(N: int)-> tuple[int, int]:
    """ Factorize an integer by searching an integer 'b' such that N + b^2 = a^2
    Then N = a^2 - b^2 = (a + b) (a - b)

    Args:
        N (int): Integer to factorize

    Returns:
        tuple[int, int]: factors
    """
    temp: int = N
    b: int = 0
    a: float = sqrt(temp)

    while int(a) != a:
        b += 1
        temp = N + pow(b, 2)
        a = sqrt(temp)

    return (int(a) + b, int(a) - b)


def kraitchik_factorization(N: int, k: int)-> tuple[int, int]:
    """ Factorize an integer by searching an integer 'b' such that kN + b^2 = a^2
    Then kN = a^2 - b^2 = (a + b) (a - b)

    Args:
        N (int): Integer to factorize
        k (int): Coefficient

    Returns:
        tuple[int, int]: factors
    """
    temp: int = N
    b: int = 0
    a: float = sqrt(temp)

    while int(a) != a:
        b += 1
        temp = k * N + pow(b, 2)
        a = sqrt(temp)

    return (gcd(N, int(a) + b), gcd(N, int(a) - b))



if __name__ == "__main__":
    n1: int = 25217
    p1, q1 = fermat_factorization(n1)
    print(f"{n1} = {p1} * {q1}; {n1 == p1*q1}")

    n2: int = 203299
    k: int = 3
    p2, q2 = kraitchik_factorization(n2, k)
    print(f"{n2} = {p2} * {q2}; {n2 == p2*q2}")
    