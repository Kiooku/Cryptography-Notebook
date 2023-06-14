# Goldwasser-Micali Probabilistic Cryptosystem

**Creators:** *Shafi Goldwasser* and *Silvio Micali*

Probabilistic encryption allow to encrypt any message $m\in M = \{0, 1\}$ "randomly" from among the possible ciphertexts.

Consequently, each bit (0 or 1) can be encrypted differently each time, using the same public key.

To do that, Alice chooses a plaintext $m$ and a random string of data $r$, and then she uses Bob's public key to encrypt the pair $(m,r)$.

> **Note** This is an impractical PKC, because the encryption is done bit by bit. Moreover, this PKC has a ***message expansion ratio*** of 1000. Which mean that a ciphertext is 1000 times as long as the plaintext.

## Algorithm

> **Note** Alice wants to send a bit to Bob using the Goldwasser-Micali Probabilistic Cryptosystem.

### Key creation

Bob chooses two primes $p$ and $q$ which are his **private key**.

Then he chooses a number $a$ with $(\frac{a}{p})=(\frac{a}{q})=-1*.

Finaly, he publishes $N=pq$ and $a$ which are his **public key**.

### Encryption

Alice chooses a bit $m\in\{0,1\}$ and a random $r$ with $1<r<N$.

Then she uses Bob's public key $(N, a)$ to compute $c$.

$c = r^2 \mod N$ if $m=0$.

$c = ar^2 \mod N$ if $m=1$.

Finaly, she sends the ***cyphertext*** $c$ to Bob. 

### Decryption

> **Warning** It's a Legendre symbol, it's not a fraction.

Bob computes $(\frac{c}{p})$.

$m = 0$ if $(\frac{c}{p}) = 1$.

$m = 1$ if $(\frac{c}{p}) = -1$.

## Resource

- *An Introduction to Mathematical Cryptography (Second edition)*