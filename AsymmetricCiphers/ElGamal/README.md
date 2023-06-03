# Elgamal Public Key Cryptosystem (1985)

**Creator:** *TAHER ELGAMAL*

The Elgamal public key encryption algorithm is based on the **discrete log problem** and is closely related to **Diﬃe–Hellman key exchange**.

## Algorithm

> **Note** Bob wants to send a message to Alice using Elgamal PKC.

### Public parameter creation

A trusted party chooses and publishes a large prime $p$ and an element $g$ modulo $p$ of large (prime) order.

### Key creation

Alice choose a private key ($1\le a \le p-1$).

Then she compute $A = g^a\text{ (mod }p\text{)}$ and publish the **public key** $A$.

### Encryption

Bob write a **plaintext** $m$ and choose a **random element** $k$.

Then he uses Alice's public key $A$ to compute $c_1=g^k\text{ (mod }p\text{)}$ and $c_2=mA^k\text{ (mod }p\text{)}$

Finally, he sends the **ciphertext** $(c_1,c_2)$ to Alice.

### Decryption

Alice computes $(c_1^a)^{-1}\cdot c_2\text{ (mod }p\text{)}$ which is equal to $m$.

> **Note** The operation can be broken down as follows:
> 
> $x\equiv (c_1^a)^{-1}\equiv c_1^{p-1-a}$
>
> $c_2x$


## Attacks using oracle

> **Note** [Click here to see the code corresponding to this attack](./ElGamalAttack.py)

In this scenario, the oracle decrypts arbitrary Elgamal ciphertexts encrypted using arbitrary Elgamal public keys.

[*If you want to read the decryption oracle definition.*](../../Attacks/README.md#decryption-oracle)

### Stupid decryption oracle

Eve know the public key of Alice ($A\equiv g^a\text{ (mod }p\text{)}$) and Bob ($B\equiv g^b\text{ (mod }p\text{)}$).

She wants to compute the value of $g^{ab}\text{ (mod }p\text{)}$.

Eve can send the oracle a **prime** $p$, a **base** $g$, a **public key**, and a **cipher text** $(c_1,c_2)$.

\
With $c_1=B=g^b$ and $c_2=1$ the oracle will compute the following thing: 

$(c_1^a)^{-1}\cdot c_2 \equiv (B^a)^{-1}\cdot 1 \equiv (g^{ba})^{-1} \text{ (mod } p\text{)}$

\
Then Eve can take the inverse modulo $p$ to obtain $g^{ab}\text{ (mod }p\text{)}$.

### Smarter decryption oracle

> **Note** Now the oracle know that it should never decrypt ciphertexts having $c_2=1$.

So Even chooses an arbitrary value for $c_2$ and set $c_1=B$. Then she tells the oracle that the public key is $A$ and give the ciphertext $(c_1,c_2)$.

The oracle retain the plaintext $m$ that satisfies

$m\equiv(c_1^a)^{-1}\cdot c_2\equiv(B^a)^{-1}\cdot c_2 \equiv (g^{ab})^{-1}\cdot\text{ (mod }p\text{)}$

Now Eve has just to computes $m^{-1}\cdot c_2\equiv g^{ab}\text{ (mod }p\text{)}$.

\
These two methods only solve the Diffie-Helman problem, not the discrete logarithm problem.

## Resource

- *An Introduction to Mathematical Cryptography (Second edition)*