import math

def find_order(g: int, p: int)-> int:
    """ Find the order of an element g of a group G
    
    Args:
        g (int): element
        p (int): modulo

    Returns:
        int: order
    """
    n: int = 1
    while pow(g,n,p) != 1:
        n += 1

    return n


def binary_search(A: list, B: list)-> tuple:
    """ Binary search to have a complexity of O(n log(n))

    Args:
        A (list): baby step list
        B (list): giant step list

    Returns:
        tuple: index for the solution
    """
    B.sort()
    i: int = 0
    for T in A:
        L: int = 0
        R: int = len(A) - 1
        while L <= R:
            m: int = (L + R) // 2
            if (B[m] < T):
                L = m + 1
            elif B[m] > T:
                R = m -1
            else:
                return (i, m)
        i += 1
        
    return (-1, -1)


# TODO search a more optimize algorithm for large number
def is_prime(n: int) -> bool:
    """Define if it's a prime number

    Args:
        n (int)

    Returns:
        bool: True: prime, False: not prime
    """
    if n < 2:
        return False

    for i in range(2, int(math.sqrt(n)) + 1):
        if n % i == 0:
            return False

    return True


def next_prime(p: int) -> int:
    """Give the next prime

    Args:
        p (int)

    Returns:
        int: next prime number
    """
    while True:
        p += 1
        if is_prime(p):
            return p


def get_product_of_prime_powers(N: int) -> dict[int, int]:
    """ N factors into a product of prime powers

    Args:
        N (int)

    Returns:
        dict[int, int]: product of prime powers
    """
    res: dict[int, int] = {}
    current_prime: int = 2
    while N != 1:
        if N % current_prime == 0:
            N //= current_prime
            if current_prime in res:
                res[current_prime] += 1
            else:
                res[current_prime] = 1
        else:
            current_prime = next_prime(current_prime)

    return res