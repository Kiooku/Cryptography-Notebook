from sage.arith.functions import LCM_list

def MOV(E: EllipticCurve, P, Q) -> int:
    """ MOV algorithm to solve the ECDLP

    Args:
        E (EllipticCurves): Elliptic curve
        P (Point): Generator
        Q (Point): Public key

    Returns:
        int
    """
    print("Step 1")
    N: int = E.order()
    k: int = 1
    infinite_point = E(0, 1, 0)
    while (p**k - 1) % N:
        k += 1

        
    Fpk = GF(p^k)
    Em = EllipticCurve(Fpk, E.a_invariants())

    Pm = Em(P)
    Qm = Em(Q)
    
    while True:
        print("Step 2")
        T = Em.random_point()
        m: int = T.order()
        l: int = gcd(m, P.order())
        print("Step 3")
        T_prime = (m // l) * T

        if P.order() / T_prime.order() in ZZ and P.order() == T_prime.order():
            print("Step 4")
            n: int = P.order()
            alpha = Pm.weil_pairing(T_prime, n)
            beta = Qm.weil_pairing(T_prime, n)

            if alpha != 1:
                print("Step 5 (Take a few seconds/minutes)")
                return beta.log(alpha)


if __name__ == "__main__":
    p = 1331169830894825846283645180581
    a = -35
    b = 98
    E = EllipticCurve(GF(p), [a,b])
    G = E(479691812266187139164535778017, 568535594075310466177352868412)
    P1 = E(1110072782478160369250829345256, 800079550745409318906383650948)
    P2 = E(1290982289093010194550717223760, 762857612860564354370535420319)
    print(f"d: {MOV(E, G, P1)}") # d: 29618469991922269