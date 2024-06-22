# Feige-Fiat-Shamir Identification Scheme

Zero-knowledge proof used to prove that Alice is really her for Bob.

The **prover** claims that $y$ is a square modulo $N$. And hence, this is what the **prover** wants to convince the **verifier** about without revealing any information *(the value of* $x$*)* such that the **verifier** can convince other people about that $y$ is a square modulo $N$.

## Algorithm

### Initialization *(Prover)*

Secret prime numbers $p$ and $q$.

Compute $N = p\cdot q$

Choose a secret value $x$. This value is what you want to convince the verifier that you have the knowledge about.

Compute $y = x^2\text{ (mod }N)$

Choose a random integer $1 \leq r \leq N+1$.

Compute $a = r^2\text{ (mod }N)$

---

> [!NOTE] 
> The **challenge**, **response** and **verification** should be perform $i$ times to make sure that you really have the knowledge of the secret value.

### Challenge *(Verifier)*

Choose randomly a challenge $b\in\{0;1\}$.



### Response *(Prover)*

Compute $z=x^b\cdot r\text{ (mod }N)$

### Verification *(Verifier)*

Verifify that $y$ is a square modulo $N$: $z^2\text{ (mod } N) = y^b\cdot a\text{ (mod }N)$

## Resource

- https://cryptographyacademy.com/identification-schemes/
