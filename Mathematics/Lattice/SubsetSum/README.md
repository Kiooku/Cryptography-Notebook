# The Merkle–Hellman subset-sum cryptosystem

The Merkle–Hellman subset-sum cryptosystem is a public key cryptosystem based on a superincreasing subset-sum problem that is disguised using congruences.

## Subset-Sum problem

You have a list of positive integer $(M_1, M2, \cdots, M_n)$ and another interger $S$.

Find the subset of the elements in the list whose sum is $S$. *(We assusme that there is at least one subset)*.

### Superincreasing Subset-Sum problem

Same problem as before, except that this time the list of positive integer $(M_1, M2, \cdots, M_n)$ has the following property: $M_i > M_{i-1} + \cdots + M_2 + M_1$ for all $2 \le i \le n$.

### How to solve the Subset-Sum problem?

It's a simple but fast algorithm:

*(Python/Sagemath implementation)*
```python
def solve(M: list[int], S: int) -> list[int]:
    """ Solve the Subset-Sum Problem for superincreasing sequence

        Args:
            M (list[int]): superincreasing sequence
            S (int): sum to find

        Returns:
            list[int]: solution
    """
    # An introduction to mathematical cryptography (Second edition) [Proposition 7.5]
    solution: list[int] = []
    current_S: int = S
    for i in range(len(M)-1, -1, -1):
        if S >= M[i]:
            solution.append(M[i])
            S -= M[i]
    
    return solution
```

## Algorithm

### Key creation

Choose a superincreasing $r = (r_1, \cdots, r_n)$.

Choose two large secret integers $A$ and $B$ with the following properties: $B > 2r_n$ and $\gcd(A, B) = 1$.

Compute $M_i = Ar_i \ (\text{mod } B)$ for $1 \le i \le n$.

Publish your public key $M = (M_1,\cdots, M_n)$.

### Encryption

Choose a binary plaintext $x$.

Use the public key $M$ to compute $S = x\cdot M$. *

Send **ciphertext** $S$.

### Decryption

Compute $S' = A^{-1}S \text{ (mod }B)$.

Solve the subset-sum problem $S'$ using the superincreasing sequence $r$ to recover the **plaintext**.

## Resources

- *An Introduction to Mathematical Cryptography (Second edition)*