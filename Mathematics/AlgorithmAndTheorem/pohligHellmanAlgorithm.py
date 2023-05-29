from tools import get_product_of_prime_powers, is_prime
from babystepGiantstepAlgorithm import babystep_giantstep
from chineseRemainderTheorem import chinese_remainder_theorem

def get_solution(p: int, g: int, h: int, x: int, e: int)-> int:
    if g == h:
        return 1

    x_i: list = [0 for _ in range(e+1)]
    gamma: int = pow(g, pow(x, e-1), p)
    for k in range(e):
        h_k: int = pow(pow(g, -x_i[k], p) * h, pow(x, e-1-k), p)
        d_k: int = babystep_giantstep(gamma, h_k, p)
        x_i[k+1] = x_i[k] + pow(x, k) * d_k
        
    return x_i[e]

def pohlig_hellman_algorithm(p: int, g: int, h: int) -> int:
    N = p - 1 # Because p is prime
    product_prime_powers_factors: dict[int, int] = get_product_of_prime_powers(N)
    solution_i: list = []
    modulo_i: list = []

    for q, e in product_prime_powers_factors.items():
        g_i: int = pow(g, N//pow(q, e), p)
        h_i: int = pow(h, N//pow(q, e), p)
        x_i: int = get_solution(p, g_i, h_i, q, e)
        solution_i.append(x_i)
        modulo_i.append(pow(q, e))
        
    x: int = chinese_remainder_theorem(solution_i, modulo_i)
    
    return x


if __name__ == "__main__":
    # An Introduction to Mathematical Cryptography (Second edition) (Example 2.36)
    p: int = 11251
    g: int = 23
    h: int = 9689

    x: int = pohlig_hellman_algorithm(p, g, h)
    print(x, pow(g,x,p) == h)
    