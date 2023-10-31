# NTRUEncrypt cryptosystem

The **NTRUEncrypt** cryptosystem use convolution polynomial rings, but the underlying hard mathematical problem can also be interpreted as $SVP$ or $CVP$
in a lattice.

## Prerequisite

**Ternary** *(or **trinary***) **polynomials**:

$$
\mathcal{T}(d_1, d_2) =
\begin{cases}
& a(x) \text{ has $d_1$ coefficients equal to $1$,} \\
a(x) \in R: & a(x) \text{ has $d_2$ coefficients equal to $-1$} \\
& a(x) \text{ has all other coefficients equal to $0$}
\end{cases}
$$

## Algorithm

> **Note** Bob wants to send a message to Alice using NTRUEncrypt cryptosystem.

### Public parameter creation (Chosen by a trusted party)

Three **convolution polynomial rings**:
$$R = \frac{\mathbb{Z}[x]}{(x^N - 1)}\qquad R_p = \frac{(\mathbb{Z}/p\mathbb{Z})[x]}{(x^N - 1)}\qquad R_q = \frac{(\mathbb{Z}/q\mathbb{Z}[x])}{(x^N - 1)}$$

$N$ and $p$: **prime numbers**.

$q$ and $d$: **integers**

**Conditions**: 
- $\gcd(p,q)=\gcd(N,q)=1$
- $q > (6d + 1)p$

### Key Creation

1. Two ***randomly chosen polynomials***:
- $f(x)\in\mathcal{T}(d+1,d)$ invertible in $R_q$ and $R_p$
- $g(x)\in\mathcal{T}(d,d)$

2. Compute:
- $F_q(x) = f(x)^{-1} \quad\text{in } R_q$
- $F_p(x) = f(x)^{-1} \quad\text{in } R_p$

**Private key**: $(f(x), F_p)$

**Public key**: $h = F_q \star g$

Publish the public key.

### Encryption

Choose a **plaintext** polynomial $m \in R_p$.

Choose a random polynomial $r \in \mathcal{T}(d,d)$.

Compute **ciphertext**: $e \equiv pr\star h+m\text{(mod }q)$.

Send the ciphertext $e$ to Alice.

### Decryption

1. $temp = f\star e \equiv pg\star r + f \star m \text{(mod }q)$.

2. Center_lift: $a = CenterLift(temp)$ with $a \in R$.

3. $m \equiv F_p\star a\text{(mod }p)$

4. **Plaintext**: $CenterLift(m)$

## Resource

- *An Introduction to Mathematical Cryptography (Second edition)*
