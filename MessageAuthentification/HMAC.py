from hashlib import md5, sha1, sha256, sha512

def hmac(message: bytes, key: bytes, hash_func) -> str:
    """HMAC algorithm

    Args:
        message (str): message to authentify
        key (str): shared key
        hash_func (bool): hash function use for HMAC

    Returns:
        str: result of the HMAC algorithm for message authentification
    """
    block_size: int = hash_func().block_size
    ipad: bytes = b"\x36" * block_size
    opad: bytes = b"\x5c" * block_size

    if len(key) > block_size:
        key = hash_func(key).digest()
    key = key + b"\x00" * (block_size - len(key))

    k_ipad: bytes = bytes(k ^ p for k, p in zip(key, ipad))
    k_opad: bytes = bytes(k ^ p for k, p in zip(key, opad))

    return hash_func(k_opad + hash_func(k_ipad + message).digest()).hexdigest()

if __name__ == "__main__":
    # All examples are from https://datatracker.ietf.org/doc/html/rfc2104 and https://en.wikipedia.org/wiki/HMAC
    k1: str = "key"
    m1: str = "The quick brown fox jumps over the lazy dog"
    expected_value1_md5: str = "80070713463e7749b90c2dc24911e275"
    expected_value1_sha1: str = "de7c9b85b8b78aa6bc8a7a36f70a90701c9db4d9"
    expected_value1_sha256: str = "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8"
    expected_value1_sha512: str = "b42af09057bac1e2d41708e48a902e09b5ff7f12ab428a4fe86653c73dd248fb82f948a549f7b791a5b41915ee4d1ec3935357e4e2317250d0372afa2ebeeb3a"
    res1: str = hmac(m1.encode(), k1.encode(), md5)
    print(f"HMAC: {res1}; Success: {res1 == expected_value1_md5}")

    k2: str = "Jefe"
    m2: str = "what do ya want for nothing?"
    expected_value2: str = "750c783e6ab0b503eaa86e310a5db738"
    res2: str = hmac(m2.encode(), k2.encode(), md5)
    print(f"HMAC: {res2}; Success: {res2 == expected_value2}")

    res3: str = hmac(m1.encode(), k1.encode(), sha1)
    print(f"HMAC: {res3}; Success: {res3 == expected_value1_sha1}")

    res4: str = hmac(m1.encode(), k1.encode(), sha256)
    print(f"HMAC: {res4}; Success: {res4 == expected_value1_sha256}")

    res5: str = hmac(m1.encode(), k1.encode(), sha512)
    print(f"HMAC: {res5}; Success: {res5 == expected_value1_sha512}")
