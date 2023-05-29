from math import ceil, sqrt
from tools import find_order, binary_search

def babystep_giantstep(g: int, h: int, p: int)-> int:
    """Shank's Babystep-Giantstep Algorithm
    Solve the discrete logarithm problem g^x = h 
    in O(sqrt(N) * log N) steps using O(sqrt(N)) storage

    Args:
        g (int): base
        h (int)
        p (int): modulo

    Returns:
        int
    """
    N: int = find_order(g,p)
    n: int = ceil(sqrt(N))
    baby_steps: dict = {i: pow(g, i, p) for i in range(n+1)}
    u: int = pow(g, -n, p)
    giant_steps: dict = {j: (h*pow(u, j, p))%p for j in range(n+1)}
    
    i, j = binary_search(baby_steps, giant_steps)
    x: int = i + j*n
    
    return x

if __name__ == "__main__":
    g: int = 9704
    h: int = 13896
    p: int = 17389
    res: int = babystep_giantstep(g, h, p)
    print(f"x={res} is a solution for g^x=h(mod p).")
    print(pow(g, res, p), pow(g, res, p) == h)