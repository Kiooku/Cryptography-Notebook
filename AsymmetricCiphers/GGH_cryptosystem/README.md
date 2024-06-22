# GGH cryptosystem

The GGH cryptosystem is a lattice-based cryptosystem.

## Algorithm

> [!NOTE] 
> Bob wants to send a message to Alice using GGH cryptosystem.

### Key Creation

**Private key**: Orthogonal basis $V$.

Choose an integer matrix $U$ with $\det(U) = \pm 1$

**Public key**: Non orthogonal basis $W = UV$.

Publish the public key.

### Encryption

Choose a small **plaintext** vector $m$.

Choose a random small vector $r$.

Compute **ciphertext**: $c = Wm + r$.

Send the ciphertext $c$ to Alice.

### Decryption

Use **Babai's algorithm** to compute the vector $v\in L$ closest to $c$.

Recover the **plaintext**: $m = vW^{-1}$.

## Resource

- *An Introduction to Mathematical Cryptography (Second edition)*