# GGH Digital Signature

Digital signature base on the [GGH cryptosystem](../../AsymmetricCiphers/GGH_cryptosystem/README.md).

> **Warning** This signature is subject to [transcript attack](https://link.springer.com/chapter/10.1007/11761679_17).

## Algorithm

### Key creation

> **Note** Same as GGH cryptosystem

**Private key**: Orthogonal basis $V$.

Choose an integer matrix $U$ with $\det(U) = \pm 1$

**Public key**: Non orthogonal basis $W = UV$.

### Signing

Choose document $d\in \mathbb{Z}^n$ to sign.

Use Babai's algorithm with the good basis to compute a vector $s \in L$ that is close to $d$.

Write $s = a_1w_1 + \cdots + a_nw_n$.

**Publish the signature** $(a_1, \cdots, a_n)$.

### Verification

Compute $s = a_1w_1 + \cdots + a_nw_n$.

**Verify** that $s$ is sufficiently close to $d$.

## Resource

- *An Introduction to Mathematical Cryptography (Second edition)*