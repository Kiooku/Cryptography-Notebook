# RSA Public Key Cryptosystem  (1977)

**Creators:** *Ron Rivest*, *Adi Shamir* and *Leonard Adleman*

RSA (**R**ivest-**S**hamir-**A**dleman) is <ins>one of the oldest public key cryptosystem</ins>, that is widely used for secure data transmission.

An equivalent system was developed secretly in 1973 at Government Communications Headquarters (GCHQ) by the English mathematician **Clifford Cocks**. That system was declassified in 1997.

The security of the algorithm is base on the principle: $x^e\equiv c\text{ (mod }N)$ is easy to solve for a person who possesses certain extra information, but it is apparently hard without this information.

> **Warning** RSA is not post-quantum secure.

## Algorithm

> **Note** Alice wants to send a message to Bob using RSA PKC.

### Key creation

Bob choose two secret primes $p$ and $q$.

Then, he choose an encryption exponent $e$ with $\gcd(e,(p-1)(q-1))=1$.

Finaly, he publishes $N=pq$ and $e$.

### Encryption

Alice write a message $m$.

Then she uses **Bob's public key** $(N,e)$ to compute $c\equiv m^e\text{ (mod }N)$.

Finaly, she sends the **ciphertext** $c$ to Bob.

### Decryption

Bob computes $d$ satisfying $ed\equiv 1\text{ (mod }(p-1)(q-1))$, which is equal to $d\equiv e^{-1}\text{ (mod }(p-1)(q-1))$

Then he compute $m'\equiv c^d\text{ (mod }N)$.

$m'$ is Alice plaintext $m$.

## Attacks

There are several possible attacks on RSA if the implementation is poorly done.

[CryptoHack](https://cryptohack.org/challenges/rsa/) offers many interesting challenges on this subject.

## Resource

- *An Introduction to Mathematical Cryptography (Second edition)*

- *[RSA Wikipedia](https://en.wikipedia.org/wiki/RSA_(cryptosystem))*