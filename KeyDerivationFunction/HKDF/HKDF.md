# HKDF (HMAC-based Extract-and-Expand Key Derivation Function)

> **Definition:** [Key derivation function (KDF)](../KeyDerivationFunction.md) that follows the **"extract-then-expand"** paradigm, where the KDF  logically consists of two modules.  The first stage takes the **input**  keying material and **"extracts"** from it a fixed-length pseudorandom key $\mathcal{K}$.  The second stage **"expands"** the key $\mathcal{K}$ into several additional pseudorandom keys (the output of the KDF).

HKDF use [HMAC](../../MessageAuthentification/HMAC.md) and a hash function (e.g. SHA-1, SHA-256, SHA-516...).

## Algorithm

> `HMAC(BYTES key, BYTES message, HASH hash_func)`

### Step 1: Extract

```
HKDF_extract(BYTES salt = b"", BYTES IKM, HASH hash_func) -> BYTES
	BYTES PKM = HMAC(salt, IKM, hash_func)
	RETURN PKM
```


### Step2: Expand

```
HKDF_expand(BYTES PRK, BYTES info = b"", INT L, HASH hash_func) -> BYTES
	INT N = ceil(L / hash_func.len)
	BYTES T = b""
	FOR k IN 1 TO N
		T += HMAC(PRK, T + info + BYTES(k), hash_func)

	BYTES OKM = T[:L]
	RETURN OKM
```

### Extract-then-Expand

```
HKDF(BYTES IKM, BYTES salt = b"", BYTES info = b"", INT L, HASH hash_func) -> BYTES
	RETURN HKDF_expand(HKDF_extract(salt, IKM, hash_func), info, L, hash_func)
```

## Resources
- https://www.ietf.org/rfc/rfc5869.txt
- https://en.wikipedia.org/wiki/HKDF