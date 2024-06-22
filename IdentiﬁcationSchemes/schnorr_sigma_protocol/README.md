# Schnorr Sigma Protocol

Identification scheme used to prove that we talk to the good person.

## Algorithm

### Public parameters

**Prime numbers**: $p$ and $q$.

$p$ satisfies that $p\equiv 1 \text{ (mod } q)$

**Generator**: $g=g_1^{(p-1) / q} \text{(mod }p)$ 

> [!NOTE] 
> $g_1$ is a generator of the group $\mathbb{Z}^*_p$.

### Initialization

Choose a **private key**: $1 \leq privKey \leq q-1$

Compute the **public key**: $g^{privKey}\text{ (mod }p)$

**Send** the *public key*.

### First message

Choose a random value $r$: $1 \leq r \leq q-1$

Compute the **first message**: $g^r\text{ (mod }p)$

**Send** the *first message*.

### Challenge

Choose a random $challenge$: $1\leq challenge \leq q-1$

**Send** the *challenge*.

### Response

Compute the **response**: $response = r + challenge \cdot privKey \text{ (mod }q)$

**Send** the *response*.

### Verification

Verify the identity by checking that $g^{response}\text{ (mod }p) = firstMessage \cdot pubKey^{challenge}\text{ (mod }p)$

## Resource

- https://cryptographyacademy.com/identification-schemes/