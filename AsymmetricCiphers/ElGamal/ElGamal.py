from sympy import isprime
from os import linesep

class ElGamal:
    def __init__(self, p: int, g: int):
        if not isprime(p):
            raise Exception("'p' need to be a prime number")
        self.p = p
        self.g = g

    def create_public_key(self, secret: int)-> int:
        """ ElGamal key creation

        Args:
            secret (int): private key

        Returns:
            int: public key
        """
        return pow(self.g, secret, self.p)

    def encryption(self, k_pub: int, m: str, k: int)-> tuple:
        """ ElGamal encryption

        Args:
            k_pub (int): public key
            m (hex): plaintext encoded
            k (int): random element

        Returns:
            tuple (hex, hex): ciphertext
        """
        converted_m: int = int(m, 16)
        c1: int = pow(self.g, k, self.p)
        c2: int = (converted_m * pow(k_pub, k)) % self.p
        return (hex(c1), hex(c2))

    def decryption(self, k_priv: int, pair: tuple)-> str:
        """ ElGamal decryption

        Args:
            k_priv (int): private key
            pair (tuple[hex, hex]): ciphertext

        Returns:
            int: plaintext encoded
        """
        c1: int = int(pair[0], 16)
        c2: int = int(pair[1], 16)
        x: int = pow(c1, self.p - 1 - k_priv, self.p)
        m: int = c2 * x % self.p
        return hex(m)
    
if __name__ == "__main__":
    p: int = 467
    g: int = 2

    elgamal1: ElGamal = ElGamal(p, g)

    a_priv: int = 153
    A_pub: int = elgamal1.create_public_key(a_priv)

    k: int = 197
    m1: int = 331
    ciphertext: tuple = elgamal1.encryption(A_pub, hex(m1), k)

    plaintext_received: int = int(elgamal1.decryption(a_priv, ciphertext), 16)

    print(f"p: {p}; g: {g}",
          f"Alice private key: {a_priv}; Alice public key: {A_pub}",
          f"Bob plaintext: 331; Bob ciphertext: {ciphertext}",
          f"Alice decryption: {plaintext_received}",
          sep=linesep)
    
    print("="*25)

    p2: int = 117477667918738952579183719876352811442282667176975299658506388983916794266542270944999203435163206062215810775822922421123910464455461286519153688505926472313006014806485076205663018026742480181999336912300022514436004673587192018846621666145334296696433207116469994110066128730623149834083870252895489152123
    g2: int = 104831378861792918406603185872102963672377675787070244288476520132867186367073243128721932355048896327567834691503031058630891431160772435946803430038048387919820523845278192892527138537973452950296897433212693740878617106403233353998322359462259883977147097970627584785653515124418036488904398507208057206926

    elgamal2: ElGamal = ElGamal(p2, g2)
    
    a2: int = 153
    A2: int = elgamal2.create_public_key(a2)

    k: int = 197
    m2: str = "I am Bob".encode().hex()
    ciphertext2: tuple = elgamal2.encryption(A2, m2, k)
    
    plaintext_received2: str = bytes.fromhex(elgamal2.decryption(a2, ciphertext2)[2:]).decode()
    
    print(f"p: {p2}; g: {g2}",
          f"Alice private key: {a2}; Alice public key: {A2}",
          f"Bob plaintext: 'I am Bob'; Bob ciphertext: {ciphertext2}",
          f"Alice decryption: {plaintext_received2}",
          sep=linesep)