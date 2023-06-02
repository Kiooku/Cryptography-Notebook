from ElGamal import ElGamal

# Both attacks are based on the assumption that Eve can consult an Elgamal oracle.
# Read An Introduction to Mathematical Cryptography (Second edition) (page 73) for more information

class Oracle:
    """ Diffie-Hellman decryption oracle
    """
    def __init__(self, a, A, b, B, p, g):
        self.a = a
        self.A = A
        self.b = b
        self.B = B
        self.g = g
        self.p = p
        self.elgamal = ElGamal(self.p, self.g)


class StupidOracle(Oracle):
    """ Decryption oracle for Elgamal PKC that accept c_2 = 1 

    Args:
        Oracle: Diffie-Hellman decryption oracle
    """
    def decryption(self, encrypted_message: tuple)-> str:
        """Elgamal decryption function

        Args:
            encrypted_message (tuple): ciphertext

        Returns:
            str: plaintext
        """
        return self.elgamal.decryption(self.a, encrypted_message)


class SmarterOracle(Oracle):
    """ Decryption oracle for Elgamal PKC that doesn't accept c_2 = 1

    Args:
        Oracle: Diffie-Hellman decryption oracle
    """
    def decryption(self, encrypted_message: tuple)-> str:
        """Elgamal decryption function

        Args:
            encrypted_message (tuple): ciphertext

        Returns:
            str: plaintext
        """
        if int(encrypted_message[1], 16) == 1:
            raise Exception("Invalid value, c_2 is equal to 1")

        return self.elgamal.decryption(self.a, encrypted_message)

if __name__ == "__main__":
    p: int = 467
    g: int = 2
    elgamal: ElGamal = ElGamal(p,g)

    a_priv: int = 153
    A_pub: int = elgamal.create_public_key(a_priv)

    b_priv: int = 325
    B_pub: int = elgamal.create_public_key(b_priv)


    ### Stupid Oracle
    stupid_oracle: StupidOracle = StupidOracle(a_priv, A_pub, b_priv, B_pub, p, g)

    # Eve tampered ciphertext
    tampered_ciphertext1: tuple = (hex(B_pub), hex(1))

    # Send A, p, g, (c_1, c_2) to the stupid oracle
    stupid_oracle_decryption: str = stupid_oracle.decryption(tampered_ciphertext1)

    # Compute the inverse of the stupid_oracle_decryption to get g^(ab)
    res1: int = pow(int(stupid_oracle_decryption, 16), -1, p)
    print(res1, pow(g, a_priv * b_priv, p), res1 == pow(2, a_priv * b_priv, p))


    ### Smarter Oracle
    smarter_oracle: SmarterOracle = SmarterOracle(a_priv, A_pub, b_priv, B_pub, p, g)

    # Eve tampered ciphertext
    c2: int = 432
    tampered_ciphertext2: tuple = (hex(B_pub), hex(c2))

    # Send A, p, g, (c_1, c_2) to the smarter oracle
    smarter_oracle_decryption: str = smarter_oracle.decryption(tampered_ciphertext2)

    # Compute m^(-1) * c_2 to get g^(ab)
    res2: int = (pow(int(smarter_oracle_decryption, 16), -1, p) * c2) % p
    print(res2, pow(g, a_priv * b_priv, p), res2 == pow(2, a_priv * b_priv, p))

    # Test the smarter oracle with c2 = 1
    try:
        smarter_oracle.decryption((hex(B_pub), 1))
    except Exception:
        print("c2 = 1 is not accepted")
