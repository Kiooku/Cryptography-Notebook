from math import sqrt
from tools import get_all_prime_numbers
from bisect import bisect_left

def f(T: int, N: int)-> int:
    """ Polynomial for quadratic sieve

    Args:
        T (int): initial value
        N (int): number to factorize (modulo)

    Returns:
        int: T² - N
    """
    return pow(T, 2) - N


def get_alpha(list_of_T: list, p: int, t: int) -> int:
    """Return alpha to get the fisrt solution


    Args:
        list_of_T (list): list of initial values
        p (int): prime factor
        t (int): congruence of N modulo p

    Returns:
        int: initial value if possible else (list_of_T[-1] + 1)
    """
    a: int = list_of_T[0]
    while pow(a, 2, p) != t and a <= list_of_T[-1]:
        a += 1

    return a


def get_beta(list_of_T: list, p: int, t: int, a: int) -> int:
    """ Return beta to get the second solution

    Args:
        list_of_T (list): list of initial values
        p (int): prime factor
        t (int): congruence of N modulo p
        a (int): alpha (first solution)

    Returns:
        int: initial value if possible else (list_of_T[-1] + 1)
    """
    b: int = list_of_T[0]
    while (pow(b, 2, p) != t and b <= list_of_T[-1]) or b == a:
        b += 1

    return b


def sieve(x: int, values: dict, list_of_T: list, p: int, small_primes_products: dict):
    """ Sieve the number to find smooth numbers

    Args:
        x (int): initial number
        values (dict): key: initial number, value: corresponding value with sieve operations
        list_of_T (list): list of initial values
        p (int): prime factor
        small_primes_products (dict): key: initial number, value: corresponding small primes products
    """
    while x <= list_of_T[-1]:
        small_primes_products[x] = small_primes_products.get(x, []) + [p]
        values[x] //= p
        x += p


def quadratic_sieve(N: int, b: int, B: int) -> dict[int, list[int]]:
    """ Quadratic sieve algorithm

    Args:
        N (int): Integer to factorize
        b (int): bound
        B (int): smoothness

    Returns:
        dict[int, list[int]]: list of smooth numbers
    """
    a: int = round(sqrt(N))
    initial_list: list = []
    f_result: dict = {}
    factor_base: list = get_all_prime_numbers(B)
    small_primes_products: dict = {}
    # Setup
    for T in range(a, b+1):
        initial_list.append(T)
        f_result[T] = f(T, N)

    pow_to_add: list[tuple[int, int]] = []
    for prime in factor_base:
        temp: int = pow(prime, factor_base.count(prime) + 1)
        while temp <= B:
            pow_to_add.append((bisect_left(factor_base, temp), prime))
            temp = pow(temp, 2)

    pow_to_add.reverse()
    for index, prime in pow_to_add:
        factor_base.insert(index, prime)

    # Algorithm
    while len(factor_base) != 0:
        p: int = factor_base[0]
        t: int = N % p
        alpha: int = get_alpha(initial_list, p, t)
        sieve(alpha, f_result, initial_list, p, small_primes_products)
        if alpha <= initial_list[-1]:
            if p != 2:
                beta: int = get_beta(initial_list, p, t, alpha)
                sieve(beta, f_result, initial_list, p, small_primes_products)
        factor_base.pop(0)

    smooth_numbers: dict = {key: small_primes_products[key] for key, value in f_result.items() if value == 1}
    return smooth_numbers


if __name__ == "__main__":
    N: int = 221
    b: int = 30
    B: int = 11
    res: dict = quadratic_sieve(N, b, B)
    print("Smooth-number:")
    for key, values in res.items():
        temp: str = " * ".join([str(v) for v in values])
        print(f"{key}² ≡ {temp} (mod {N})")
        
    # TODO use Elimination & GCD to solve the result given by the quadratic sieve