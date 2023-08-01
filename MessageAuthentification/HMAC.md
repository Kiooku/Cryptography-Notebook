# HMAC: Keyed-Hashing for Message Authentication


> **Definition:** Mechanism for message authentification using cryptographic hash functions *(any iterative cryptographic hash function: MD5, SHA-1...)* in combination with a secret shared key. The cryptographic strength of HMAC depends on the properties of the underlying hash function.

## Algorithm

### Initialisation
- Cryptographic hash function: **H**
- Secret key: **K**
- Byte-length of each block of **H**: **B**
- Byte-length of hash outputs: **L**

**ipad**: the byte `0x36` repeated B times
**opad**: the byte `0x5C` repeated B times.
> **Note**: *(the 'i' and 'o' are mnemonics for inner and outer)*

### Compute

`H(K XOR opad, H(K XOR ipad, text))`

### Full Algorithm

```
HMAC(Bytes M, Bytes K, Hash H) -> Hexa
    INT B = H.block_size
    Bytes ipad = "0x36" * B
    Bytes opad = "0x5c" * B

    IF key.length > B THEN
        K = H(K)
    ELSE IF key.length < B THEN
        K = Padding(K, B - K.length) // Add zero at the end of K
    
    RETURN H(K XOR opad, H(K XOR ipad, M))
```

## Resources
- https://www.ietf.org/rfc/rfc5869.txt
- https://en.wikipedia.org/wiki/HMAC