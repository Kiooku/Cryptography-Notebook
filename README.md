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

- [X] [Diffie-Hellman Key Exchange](./AsymmetricCiphers/Diffie_Hellman/)

- [X] [ElGamal PKC](./AsymmetricCiphers/ElGamal/)

- [X] [RSA](./AsymmetricCiphers/RSA/)

- [X] [Goldwasser–Micali Probabilistic PKC](./AsymmetricCiphers/GoldwasserMicaliProbabilisticPKC/)

- [X] [Elliptic Diffie-Hellman Key Exchange](./AsymmetricCiphers/Elliptic_Diffie_Hellman/)

- [X] [Elliptic Elgamal Public Key Cryptosystem](./AsymmetricCiphers/EllipticElgamalPKC/)

- [ ] Menezes-Vanstone variant of the Elliptic Elgamal PKC

- [X] [Tripartite Diffie-Hellman Key Exchange](./AsymmetricCiphers/Tripartite_Diffie_Hellman_Key_Exchange/)

- [X] [The Merkle–Hellman subset-sum cryptosystem](./Mathematics/Lattice/SubsetSum/)

- [X] [GGH cryptosystem](./AsymmetricCiphers/GGH_cryptosystem/)

- [X] [NTRUEncrypt cryptosystem](./AsymmetricCiphers/NTRUEncrypt/)

- [X] [X3DH](./AsymmetricCiphers/x3dh/)

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

### Commitment Schemes

- [X] [The Pedersen commitment scheme](./CommitmentScheme/pedersen-commitment-scheme/)

### Identiﬁcation Schemes

- [X] [Feige–Fiat–Shamir identification scheme](./IdentiﬁcationSchemes/feige-fiat-shamir-identification-scheme/)

- [X] [Schnorr's sigma protocol](./IdentiﬁcationSchemes/schnorr_sigma_protocol/)

### Digital Signatures

> **Note** Work in progress

- [X] [RSA Digital Signatures](./DigitalSignatures/RSA_Digital_Signatures/)

- [X] Elgamal Digital Signatures

- [X] Digital Signature Algorithm (DSA)

- [X] Elliptic Curve Digital Signature Algorithm (ECDSA)

- [X] [GGH Digital Signature Scheme](./DigitalSignatures/GGH_Digital_Signature/)

- [ ] NTRU Modular Lattice Signature Scheme (NTRUMLS)

### Hash Functions

> **Note** Basic knowledge -> Need to learn more

### Message Authentification

- [X] [HMAC](./MessageAuthentification/)

### Key Derivation Function

- [X] [HKDF](./KeyDerivationFunction/HKDF/)

### Secret Sharing Schemes

- [ ] Shamir's secret sharing

- [ ] Blakley's scheme

### PRNG

> **Note** Need to learn

- [ ] ANSI X9.17 PRNG

- [ ] DSA PRNG

- [ ] RSAREF PRNG

- [ ] fsrRand

### One-Time pad

> **Note** One-Time pad can be really simple to code, so I did it in assembly

- [X] [XOR based one-time pad](./OneTimePad/)

### End-to-end encryption

> **Note** Need to learn more

- [X] [Double Ratchet Algorithm](./E2EE/double-ratchet-algorithm/)

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

- [Cryptography Academy](https://cryptographyacademy.com/identification-schemes/)

- [Signal](https://signal.org/docs/)