# Cryptography Notebook

## What is that ?

It's a repository with some of the cryptographic implementations I have done for learning purposes.

## Why the repository is almost empty ?

I'm learning cryptography by myself, until then by solving cryptography challenges on [CryptoHack](https://cryptohack.org/user/Kioku/) and Root-Me.

Currently, I'm reading "An Introduction to Mathematical Cryptography" (Second edition), and in order to apply the mathematical knowledge and get a better understanding, I decided to create this repository.

Learn, understand, and create project is a long process, which is why the repository will contain more implementation month after month.

## Content of the repository

### Symmetric Ciphers

> **Note** Work in progress

- [ ] AES (ECB / CBC / CFB / OFB / CTR)

### Asymmetric Ciphers

> **Note** Work in progress

#### Implementations

- [X] [Diffie-Hellman Key Exchange](./AsymmetricCiphers/Diffie_Hellman/README.md)

- [X] [ElGamal PKC](./AsymmetricCiphers/ElGamal/README.md)

- [X] [RSA](./AsymmetricCiphers/RSA/README.md)

- [X] [Goldwasser–Micali Probabilistic PKC](./AsymmetricCiphers/GoldwasserMicaliProbabilisticPKC/README.md)

#### Attacks

##### Oracle Attack

- [X] [ElGamal with a Diffie-Hellman Oracle](./AsymmetricCiphers/ElGamal/README.md#attacks-using-oracle)

#### Man-in-the-Middle Attack *(MITM)*

- [ ] Diffie-Hellman MITM

### Digital Signatures

> **Note** Work in progress

- [X] RSA Digital Signatures

- [X] Elgamal Digital Signatures

- [X] Digital Signature Algorithm (DSA)

- [ ] Elliptic Curve Digital Signature Algorithm (ECDSA)

- [ ] GGH Digital Signature Scheme

- [ ] NTRU Modular Lattice Signature Scheme (NTRUMLS)

### Hash Functions

> **Note** Basic knowledge -> Need to learn more

### Message Authentification

- [X] [HMAC](./MessageAuthentification/README.md)

### Key Derivation Function

- [X] [HKDF](./KeyDerivationFunction/HKDF/README.md)

### PRNG

> **Note** Need to learn

### End-to-end encryption

> **Note** Need to learn

- [ ] Signal Double Ratchet Algorithm

### Zero-Knowledge Proofs

> **Note** Need to learn

### Homomorphic Encryption

> **Note** Ongoing learning of lattices to understand homomorphic encryption

### Post-Quantum

> **Note** Ongoing learning of lattices and learning with errors to understand Post-Quantum cryptography

### Mathematics

#### Algorithm and Theorem

> **Note** Work in progress

- [X] Euclidean algorithm

- [X] Shanks’s Babystep–Giantstep Algorithm

- [X] Chinese Remainder Theorem

- [X] Pohlig-Hellman Algorithm

- [X] Miller–Rabin test for composite numbers

- [X] Pollard’s p − 1 factorization algorithm

- [ ] Gaussian elimination

- [X] Fermat's Factorization

- [X] Kraitchik’s Factorization

- [ ] A three step factorization procedure (Relation Building / Elimination / GCD Computation)

- [X] Quadratic sieve

- [ ] Index calculus

- [ ] Pollard's $\rho$ Method

#### Elliptic Curves

> **Note** Need to learn more

#### Lattices

> **Note** Work in progress

> **Note** The [lattice folder](./Mathematics/Lattice/) is the start of another project. The code tries to follow good code practices.

> **Warning** I'm still learning by myself Lattice, so I may have made a few mistakes

- [X] Create a Lattice class

- [ ] Create a Knapsack class

- [X] Gram-Schmidt Algorithm

- [X] Hadamard ratio

- [ ] Babai's closest vertext algorithm

- [ ] Babai's closest plane algorithm

- [X] Gaussian Lattice reduction

- [ ] LLL algorithm

- [ ] BKZ-LLL algorithm

### Learning with errors (LWE)

> **Note** Need to learn

## Resources

- [An Introduction to Mathematical Cryptography (Second Edition)](https://link.springer.com/book/10.1007/978-1-4939-1711-2)

- [CryptoHack](https://cryptohack.org/)