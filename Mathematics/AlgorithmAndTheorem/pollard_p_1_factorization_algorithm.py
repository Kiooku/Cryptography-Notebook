from euclideanAlgorithm import gcd
from sympy import isprime

def pollard_p_minus_1_factorization_algorithm(N: int, bound: int = 10000)-> int:
    """Pollard's p-1 factorization algorithm

    Args:
        N (int): Integer to be factored
        bound (int, optional): Defaults to 10000.

    Returns:
        int: output = -1 -> failure of the algorithm ; output != -1 success (p = d)
    """
    a: int = 2
    for j in range(2, bound+1):
        a = pow(a, j, N)
        d: int = gcd(a-1, N) # Can improve efficiency by choosing an appropriate k and compute the gcd only every kth iteration
        if 1 < d < N:
            return d

    return -1


if __name__ == "__main__":
    N_x: list[int] = [13927189, 168441398857]

    for N in N_x:
        p: int = pollard_p_minus_1_factorization_algorithm(N)
        if p != -1 and isprime(p):
            q: int = N // p
            if isprime(q):
                print(f"For N = {N}, p = {p} and q = {q}")
