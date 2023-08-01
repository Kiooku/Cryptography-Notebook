from hashlib import sha1, sha256
from math import ceil
from MessageAuthentification.HMAC import hmac

# If you getting ModuleNotFoundError do the following command:
# export PYTHONPATH="${PYTHONPATH}:/path/to/project_root/"


def HKDF(IKM: bytes, L: int, hash_func, info: bytes = b"", salt: bytes = b"") -> bytes:
    """ HKDF (HMAC-based Extract-and-Expand Key Derivation Function)
    A key derivation function base on HMAC and a hash function (e.g. SHA-1, SHA-256, SHA-512...)

    Args:
        IKM (bytes): input keying material
        L (int): length of output keying material in octets
        hash_func: a hash function
        info (bytes, optional): context and application specific information. Defaults to b"".
        salt (bytes, optional): salt value (a non-secret random value). Defaults to b"".

    Returns:
        OKM (bytes): output keying material
    """
    return HKDF_expand(HKDF_extract(IKM, hash_func, salt), L, hash_func, info)


def HKDF_extract(IKM: bytes, hash_func, salt: bytes = b"") -> bytes:
    """ First stage "extracts": Create a fixed-length pseudorandom key K.

    Args:
        IKM (bytes): input keying material
        hash_func: a hash function
        salt (bytes, optional): a non-secret random value. Defaults to b"".

    Returns:
        PRK (str): a pseudorandom key
    """
    PRK: str = hmac(IKM, salt, hash_func)
    return bytes.fromhex(PRK)


def HKDF_expand(PRK: bytes, L: int, hash_func, info: bytes = b"") -> bytes:
    """ Second stage "expands": The pseudorandom key to the desired length

    Args:
        PRK (bytes): a pseudorandom key of at least HashLen octets (usually, the output from the extract step)
        L (int): length of output keying material in octets (<= 255*HashLen)
        hash_func: a hash function
        info (bytes, optional): context and application specific information. Defaults to b"".

    Returns:
        OKM (str): output keying material (of L octets)
    """
    N: int = ceil(L / hash_func().digest_size)
    T: bytes = b""
    OKM: bytes = b""
    for k in range(N):
        T = bytes.fromhex(hmac(T + info + bytes([k+1]), PRK, hash_func))
        OKM += T

    return OKM[:L]


if __name__ == "__main__":
    # https://www.rfc-editor.org/rfc/rfc5869.html
    # Test Case 1 (sha-256)
    IKM_1: bytes = bytes.fromhex("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b")
    salt_1: bytes = bytes.fromhex("000102030405060708090a0b0c")
    info_1: bytes = bytes.fromhex("f0f1f2f3f4f5f6f7f8f9")
    L_42: int = 42

    res1: str = HKDF(IKM_1, L_42, sha256, info_1, salt_1).hex()
    OKM_1: str = "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865"
    print(f"[1] HKDF (SHA-256): {res1}; Success: {res1 == OKM_1}\n")

    # Test Case 2 (sha-256)
    IKM_2: bytes = bytes.fromhex("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f")
    salt_2: bytes = bytes.fromhex("606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeaf")
    info_2: bytes = bytes.fromhex("b0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff")
    L_82: int = 82

    res2: str = HKDF(IKM_2, L_82, sha256, info_2, salt_2).hex()
    OKM_2: str = "b11e398dc80327a1c8e7f78c596a49344f012eda2d4efad8a050cc4c19afa97c59045a99cac7827271cb41c65e590e09da3275600c2f09b8367793a9aca3db71cc30c58179ec3e87c14c01d5c1f3434f1d87"
    print(f"[2] HKDF (SHA-256): {res2}; Success: {res2 == OKM_2}\n")

    # Test Case 3 (sha-256)
    res3: str = HKDF(IKM_1, L_42, sha256).hex()
    OKM_3: str = "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d9d201395faa4b61a96c8"
    print(f"[3] HKDF (SHA-256): {res3}; Success: {res3 == OKM_3}\n")

    # Test Case 4 (sha-1)
    IKM_4: bytes = bytes.fromhex("0b0b0b0b0b0b0b0b0b0b0b")
    salt_4: bytes = bytes.fromhex("000102030405060708090a0b0c")
    info_4: bytes = bytes.fromhex("f0f1f2f3f4f5f6f7f8f9")

    res4: str = HKDF(IKM_4, L_42, sha1, info_4, salt_4).hex()
    OKM_4: str = "085a01ea1b10f36933068b56efa5ad81a4f14b822f5b091568a9cdd4f155fda2c22e422478d305f3f896"
    print(f"[4] HKDF (SHA-1): {res4}; Success: {res4 == OKM_4}\n")

    # Test Case 5 (sha-1)
    res5: str = HKDF(IKM_2, L_82, sha1, info_2, salt_2).hex()
    OKM_5: str = "0bd770a74d1160f7c9f12cd5912a06ebff6adcae899d92191fe4305673ba2ffe8fa3f1a4e5ad79f3f334b3b202b2173c486ea37ce3d397ed034c7f9dfeb15c5e927336d0441f4c4300e2cff0d0900b52d3b4"
    print(f"[5] HKDF: {res5}; Success: {res5 == OKM_5}\n")

    # Test Case 6 (sha-1)
    res6: str = HKDF(IKM_1, L_42, sha1).hex()
    OKM_6: str = "0ac1af7002b3d761d1e55298da9d0506b9ae52057220a306e07b6b87e8df21d0ea00033de03984d34918"
    print(f"[6] HKDF (SHA-1): {res6}; Success: {res6 == OKM_6}\n")

    # Test Case 7 (sha-1)
    IKM_7: bytes = bytes.fromhex(
        "0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c")

    res7: str = HKDF(IKM_7, L_42, sha1).hex()
    OKM_7: str = "2c91117204d745f3500d636a62f64f0ab3bae548aa53d423b0d1f27ebba6f5e5673a081d70cce7acfc48"
    print(f"[7] HKDF (SHA-1): {res7}; Success: {res7 == OKM_7}")
