# Elliptic Elgamal Public Key Cryptography

The Elliptic Elgamal public key cryptography algorithm is based on the **Elliptic Elgamal Discrete Log Problem**.

> **Warning**: There are some practical difficulties
> 
> 1) No obvious way to attach plaintext messages to points in $E(\mathbb{F}_p)$.
> 2) This cryptosystem has $4$-to-$1$ message expansion

## Algorithm

> [!NOTE] 
> Bob wants to send a message to Alice using Elliptic Elgamal PKC.

### Public parameter creation

**Trusted party** chooses and publishes:
- Large prime $p$;
- Elliptic curve $E$ over $\mathbb{F}_p$;
- Point $P$ in $E(\mathbb{F}_p)$.

### Key creation

Alice choose a private key $n_A$.

Then she computes $Q_A = n_AP$ in $E(\mathbb{F}_p)$ and publish the **public key** $Q_A$.

### Encryption

Bob write a **plaintext** $M\in E(\mathbb{F}_p)$ and choose a **random element** $k$.

Then he uses Alice's public key $Q_A$ to compute $C_1 = kP \in E(\mathbb{F}_p)$ and $C_2 = M + kQ_A \in E(\mathbb{F}_p)$

Finally, he sends the **ciphertext** $(C_1,C_2)$ to Alice.

### Decryption

Alice computes $C_2 - n_AC_1 \in E(\mathbb{F}_p)$ which is equal to $M$.

> [!NOTE] 
> The operation can be broken down as follows:
> 
> $C_2 - n_AC_1 = (M + kQ_A) - n_A(kP) = M + k(n_AP) - n_A(kP) = M$

## Resource

- *An Introduction to Mathematical Cryptography (Second edition)*