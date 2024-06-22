# Tripartite Diffie-Hellman Key Exchange

Tripartite Diffie-Hellman Key Exchange is a Diffie-Hellman key exchange based on elliptic curve over a finite field, which enables the generation of a shared secret between 3 different people.

> [!NOTE] 
> The security is based on the **Elliptic Curve Discrete Logarithm Problem** (ECDLP), **Elliptic Curve Diffie-Hellman Problem** and **Discrete Logatithm Problem**

### Prerequisite
- Basic knowledge of elliptic curves, weil pairing and l-distortion map
    - *An Introduction to Mathematical Cryptography (Second edition)*
    - [Cryptography Standford Education](https://web.archive.org/web/20220412170936/https://crypto.stanford.edu/pbc/notes/elliptic/)

## Algorithm

### Public parameter creation

**Trusted party** chooses and publishes:
- Large prime $p$;
- Elliptic curve $E$ over $\mathbb{F}_p$;
- Point $P$ in $E(\mathbb{F}_p)$ of prime order $l$;
- An $l$ distortion map $\phi$ for $P$.

### Private computations

| Alice   |      Bob      |  Carl |
|----------|:-------------:|------:|
| Chooses a secret integer $n_A$. <br/> Computes $Q_A = n_AP$. |  Chooses a secret integer $n_B$. <br/> Computes $Q_B = n_BP$. | Chooses a secret integer $n_C$. <br/> Computes $Q_C = n_CP$. |


Alice's **private key** is $n_A$ and her **public key** is $Q_A$.

Bob's **private key** is $n_B$ and his **public key** is $Q_B$.

Carl's **private key** is $n_C$ and his **public key** is $Q_C$.

### Public exchange of values

Alice sends $Q_A$ to Bob and Carl.

Bob sends $Q_B$ to Alice and Carl.

Carl sends $Q_C$ to Alice and Bob.

### Shared secret computations

> [!NOTE]
> $ê_l(Q, Q') = e_l(Q, \phi(Q'))$

| Alice   |      Bob      |  Carl |
|----------|:-------------:|------:|
| Computes the number $ê_l(Q_B, Q_C)^{n_A}$. |  Computes the number $ê_l(Q_A, Q_C)^{n_B}$. | Computes the number $ê_l(Q_A, Q_B)^{n_C}$. |

The **shared secret** value is $ê_l(P, P)^{n_An_Bn_C}$.

## Resources

- *An Introduction to Mathematical Cryptography (Second edition)*