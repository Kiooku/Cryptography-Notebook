from math import ceil, sqrt

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

# https://stackoverflow.com/questions/53135373/how-to-compare-each-element-in-two-arrays-with-time-complexity-less-than-on2
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
    baby_steps: list = [pow(g, i, p) for i in range(n+1)]
    u: int = pow(g, -n, p)
    giant_steps: list = [(h*pow(u, j))%p for j in range(n+1)]
    
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