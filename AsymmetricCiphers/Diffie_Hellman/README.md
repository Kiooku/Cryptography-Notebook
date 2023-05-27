# Diffie-Hellman Key Exchange (1976)

**Creators:** *WHITFIELD DIFFIE* and *MARTIN E. HELLMAN*

The Diffie-Helman key exchange algorithm provides a **method of publicly sharing a random secret key**, it does not achieve the full goal of being a public key cryptosystem.

> **Note** The security is based on the **Discrete Logarithms Problem** (DLP) and **Diffie-Hellman Problem** (DHP)

The **Diffie-Hellman Problem** is the problem of computing the value of $g^{ab}\text{ (mod }p\text{)}$ from the known values of $g^a\text{ (mod }p\text{)}$ and $g^b\text{ (mod }p\text{)}$.

## Algorithm

### Public parameter creation

A trusted party chooses and publishes a (large) prime $p$
and an integer $g$ having large prime order in $\mathbb{F}^∗_p$ .

### Private computations

|                        Alice                        |                         Bob                         |
|:---------------------------------------------------:|:---------------------------------------------------:|
| Choose a secret integer $a$. <br/> Compute $A \equiv g^a \text{(mod }p\text{)}$. | Choose a secret integer $b$. <br/> Compute $B \equiv g^b \text{(mod }p\text{)}$. |

Alice's **private key** is $a$ and her **public key** is $A$.

Bob's **private key** is $b$ and his **public key** is $B$.

### Public exchange of values

Alice sends $A$ to Bob.

Bob sends $B$ to Alice.

### Further private computations

|              Alice              |               Bob              |
|:-------------------------------:|:------------------------------:|
| Compute the number $B^a \text{ (mod }p\text{)}$. | Compute the number $A^b \text{ (mod }p\text{)}$. |

The **shared secret** value is $B^a \equiv (g^b)^a \equiv g^{ab} \equiv (g^a)^b \equiv A^b \text{ (mod }p\text{)}.$

## Resource

- *An Introduction to Mathematical Cryptography (Second edition)*
