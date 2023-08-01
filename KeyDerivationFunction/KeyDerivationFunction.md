# Key derivation function (KDF)

> **Definition:** Cryptographic algorithm that derives one or more secret keys from a secret value such as a master key, a password, or a passphrase using a pseudorandom function (which typically uses a cryptographic hash function or block cipher).

## Examples of KDF
As of May 2023, OWASP recommends the following KDFs for password hashing, listed in order of priority:
- Argon2id
- scrypt if Argon2id is unavailable
- bcrypt for legacy systems
- PBKDF2 if FIPS-140 compliance is required
- [HKDF (HMAC-based Extract-and-Expand Key Derivation Function)](./HKDF/HKDF.md)


## Resources
- https://en.wikipedia.org/wiki/Key_derivation_function