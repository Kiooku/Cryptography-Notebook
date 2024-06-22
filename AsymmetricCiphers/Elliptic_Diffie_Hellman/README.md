# Elliptic Diffie-Hellman Key Exchange

Elliptic Diffie-Hellman Key Exchange is a Diffie-Hellman key exchange based on elliptic curve over a finite field.

> [!NOTE] 
> The security is based on the **Elliptic Curve Discrete Logarithm Problem** (ECDLP) and **Elliptic Curve Diffie-Hellman Problem**

### Prerequisite
- Basic knowledge of elliptic curves
    - *An Introduction to Mathematical Cryptography (Second edition)*
    - [Cryptography Standford Education](https://web.archive.org/web/20220412170936/https://crypto.stanford.edu/pbc/notes/elliptic/)

## Algorithm

### Public parameter creation

**Trusted party** chooses and publishes:
- Large prime $p$;
- Elliptic curve $E$ over $\mathbb{F}_p$;
- Point $P$ in $E(\mathbb{F}_p)$.

### Private computations

|                        Alice                        |                         Bob                         |
|:---------------------------------------------------:|:---------------------------------------------------:|
| Chooses a secret integer $n_A$. <br/> Computes $Q_A \equiv n_AP$. | Chooses a secret integer $n_B$. <br/> Computes $Q_B \equiv n_BP$. |

Alice's **private key** is $n_A$ and her **public key** is $Q_A$.

Bob's **private key** is $n_B$ and his **public key** is $Q_B$.

### Public exchange of values

Alice sends $Q_A$ to Bob.

Bob sends $Q_B$ to Alice.

> [!NOTE] 
> The public exchange can be with only the $x_a$ and $x_b$ value of the points $Q_A$ and $Q_B$.

### Shared secret computations

|              Alice              |               Bob              |
|:-------------------------------:|:------------------------------:|
| Computes the number $n_AQ_B$. | Computes the number $n_BQ_A$. |

The **shared secret** value is $n_AQ_B \equiv n_A(n_BP) \equiv n_B(n_AP) \equiv n_BQ_A$.

## Resources

- *An Introduction to Mathematical Cryptography (Second edition)*