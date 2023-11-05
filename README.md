# Cryptography Notebook

## What is that ?

It's a repository with some of the cryptographic implementations I have done for learning purposes.

Until then by solving cryptography challenges on [CryptoHack](https://cryptohack.org/user/Kioku/) and Root-Me.

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

- [X] [Elliptic Diffie-Hellman Key Exchange](./AsymmetricCiphers/Elliptic_Diffie_Hellman/README.md)

- [X] [Elliptic Elgamal Public Key Cryptosystem](./AsymmetricCiphers/EllipticElgamalPKC/README.md)

- [ ] Menezes-Vanstone variant of the Elliptic Elgamal PKC

- [X] [Tripartite Diffie-Hellman Key Exchange](./AsymmetricCiphers/Tripartite_Diffie_Hellman_Key_Exchange/README.md)

- [X] [The Merkle–Hellman subset-sum cryptosystem](./Mathematics/Lattice/SubsetSum/README.md)

- [X] [GGH cryptosystem](./AsymmetricCiphers/GGH_cryptosystem/README.md)

- [X] [NTRUEncrypt cryptosystem](./AsymmetricCiphers//NTRUEncrypt/README.md)

#### Attacks

##### Diffie-Hellman

- [X] [Small Subgroup Confinement Attack *(Diffie-Hellman)*](./Attacks/README.md#small-subgroup-confinement-attack-diffie-hellman)

##### ElGamal

- [X] [ElGamal with a Diffie-Hellman Oracle](./AsymmetricCiphers/ElGamal/README.md#attacks-using-oracle)

##### RSA

- [X] [Blinding Attack on RSA Digital Signature](./Attacks/RSA/BlindingAttackonRSADigitalSignature.md)
- [X] [Bleinchenbacher's attack (PKCS 1 v1.5)](./Attacks/RSA/BleinchenbacherAttack(PKCS%201v1.5).md)

#### Man-in-the-Middle Attack *(MITM)*

- [ ] Diffie-Hellman MITM

### ID-Based Public Key Cryptosystem

- [ ] The Boneh Franklin Id Based PKC

### Digital Signatures

> **Note** Work in progress

- [X] RSA Digital Signatures

- [X] Elgamal Digital Signatures

- [X] Digital Signature Algorithm (DSA)

- [X] Elliptic Curve Digital Signature Algorithm (ECDSA)

- [X] [GGH Digital Signature Scheme](./DigitalSignatures/GGH_Digital_Signature/README.md)

- [ ] NTRU Modular Lattice Signature Scheme (NTRUMLS)

### Hash Functions

> **Note** Basic knowledge -> Need to learn more

### Message Authentification

- [X] [HMAC](./MessageAuthentification/README.md)

### Key Derivation Function

- [X] [HKDF](./KeyDerivationFunction/HKDF/README.md)

### PRNG

> **Note** Need to learn

- [ ] ANSI X9.17 PRNG

- [ ] DSA PRNG

- [ ] RSAREF PRNG

- [ ] fsrRand

### End-to-end encryption

> **Note** Need to learn more

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

- [X] Addition
- [X] Double-and-Add Algorithm
- [X] Lenstra's Elliptic Curve Factorization Algorithm
- [X] Miller algorithm
- [X] Weil Pairing
- [X] MOV algorithm

#### Lattices

- [X] Create a Lattice class

- [X] Create a Knapsack class

- [X] Gram-Schmidt Algorithm

- [X] Hadamard ratio

- [X] Babai's closest vertex algorithm

- [X] Babai's closest plane algorithm

- [X] Gaussian Lattice reduction

- [X] LLL algorithm

### Learning with errors (LWE)

> **Note** Need to learn

## Resources

- [An Introduction to Mathematical Cryptography (Second Edition)](https://link.springer.com/book/10.1007/978-1-4939-1711-2)

- [CryptoHack](https://cryptohack.org/)