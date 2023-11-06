# RSA Digital Signatures

Allow to sign document using the RSA PKC.

## Algorithm

### Key Creation

Choose secret prime $p$ and $q$.

Choose verification exponent $e$ satisfying: $\gcd(e, (p-1)(q-1)) = 1$

**Public key**: $N = pq$ and $e$.

### Signing

Compute $d$: $de\equiv1(\text{mod(}(p-1)(q-1))$.

**Signature**: $S\equiv D^d\text{(mod } N)$ with $D$ the document to sign.

### Verification

Compute $S^e \text{(mod }N)$ and verify that it's equal to $D$.

## How do you convert it into a blind digital signature scheme?

> **Note** A blind digital signature is when the document to be signed is ﬁrst blinded (concealed) and then signed. In this way, the signed document is not revealed to the signing authority.

The only thing to change is to slighly modify the document before the signature.

1. Choose a random number $R \text{ (mod }N)$.
2. Compute $D'\equiv R^eD\text{ (mod }N)$
3. Ask the authority to sign the document: $S\equiv (D')^d\equiv(R^eD)^d\equiv R^{ed}D^d\equiv RD^d\text{ (mod }N)$
4. Retrieve the document signed: $R^{-1}S\equiv D^d\text{ (mod }N)$

## Resources

- *An Introduction to Mathematical Cryptography (Second edition)*