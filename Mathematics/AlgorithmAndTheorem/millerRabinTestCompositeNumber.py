from euclideanAlgorithm import gcd

def setup_k_and_q(n: int)-> tuple[int, int]:
    """ Define k and q
    We know that n-1 = 2^k * q with q odd

    Args:
        n (int): integer to be tested

    Returns:
        tuple[int, int]: (k, q)
    """
    k: int = 0
    q: int = n-1

    while q % 2 == 0:
        k += 1
        q //= 2

    return (k, q)

def miller_rabin_test_composite_number(n: int, a: int) -> bool:
    """ Miller-Rabin Test for composite number
    Define if a a number is a composite number or not

    Args:
        n (int): integer to be tested
        a (int): integer as a potential witness

    Returns:
        bool: True->composite number
    """
    if n%2 == 0 or 1 < gcd(a, n) < n:
        print("a")
        return True

    k, q = setup_k_and_q(n)
    a = pow(a, q, n)
    if a % n == 1:
        return False

    for _ in range(k):
        if (a % n) - n == -1:
            return False
        a = pow(a, 2, n)

    return True


if __name__ == "__main__":
    # An Introduction to Mathematical Cryptography (Second edition) (Section 3.4)
    a_1: int = 2
    n_1: int = 561
    res_1: bool = miller_rabin_test_composite_number(n_1, a_1)
    print(f"{a_1} is a Miller-Rabin witness?\n{res_1}", end="\n\n==========\n\n")

    n_2: int = 172947529
    a_x: list[int] = [17, 3, 23]
    for a in a_x:
        res_x: bool = miller_rabin_test_composite_number(n_2, a)
        print(f"{a} is a Miller-Rabin witness?\n{res_x}\n", end="\n==========\n\n")
    