import math

def modular_multiplicative_inverse(a: int, b: int)-> int:
    if math.gcd(a,b) != 1:
        raise Exception(f"{a} and {b} are not coprime !")
    
    return pow(a, b-2, b)

def chinese_remainder_theorem(a: list, p: list)-> int:
    N: int = math.prod(p)
    e: list = []
    x: int
    
    for i in range(len(p)):
        ni = math.prod([p[j] for j in range(len(p)) if j != i])
        e.append(ni * modular_multiplicative_inverse(ni, p[i]))
        
    x = sum([a[i] * e[i] for i in range(len(a))]) % N
    return x

if __name__ == "__main__":
    # x = 1 (mod 5) ||| x = 9 (mod 11)
    res1: int = chinese_remainder_theorem([1,9], [5,11])
    print(res1, res1%5 == 1, res1%11 == 9)
    # x = 2 (mod 3) ||| x = 3 (mod 7) ||| x = 4 (mod 16)
    res2: int = chinese_remainder_theorem([2,3,4],[3,7,16])
    print(res2, res2%3 == 2, res2 % 7 == 3, res2 % 16 == 4)