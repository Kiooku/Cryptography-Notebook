import math
from tools import modular_multiplicative_inverse

def chinese_remainder_theorem(a: list, p: list)-> int:
    N: int = math.prod(p)
    x: int = 0
    
    for i in range(len(p)):
        n_i: int = N//p[i]
        x += a[i] * n_i * modular_multiplicative_inverse(n_i, p[i])
        
    return x % N

if __name__ == "__main__":
    # x = 1 (mod 5) ||| x = 9 (mod 11)
    res1: int = chinese_remainder_theorem([1,9], [5,11])
    print(res1, res1%5 == 1, res1%11 == 9)
    # x = 2 (mod 3) ||| x = 3 (mod 7) ||| x = 4 (mod 16)
    res2: int = chinese_remainder_theorem([2,3,4],[3,7,16])
    print(res2, res2%3 == 2, res2 % 7 == 3, res2 % 16 == 4)
    print(chinese_remainder_theorem([1,4,511],[2,9,625]))