def gcd(a: int, b: int)-> int:
    """ Euclidean algorithm

    Args:
        a (int)
        b (int)

    Returns:
        int: Greatest Common divisor
    """
    if b == 0:
        return a
    return gcd(b, a%b)

def extended_euclidean_algorithm(a: int, b: int)-> tuple:
    """Extended Euclidean algorithm

    Args:
        a (int):
        b (int):

    Returns:
        tuple: Solution to: au + bv = gcd(a,b)
    """
    # An Introduction to Mathematical Cryptography (Second edition) (Exercice 1.12)
    u: int = 1
    g: int = a
    x: int = 0
    y: int = b
    if b == 0:
        return (g, a, b)
    while y != 0:
        q: int = g // y
        t: int = g % y
        s: int = u - q*x
        u, g = x, y
        x, y = s, t
    v: int = (g - a*u)//b
    return (g, u, v)

if __name__ == "__main__":
    print(gcd(2024, 748))
    res = extended_euclidean_algorithm(2024, 748)
    print(f"{res[1]}·2024 + {res[2]}·748 = {res[0]} = gcd(2024, 748)")