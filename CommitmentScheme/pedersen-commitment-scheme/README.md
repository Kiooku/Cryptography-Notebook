# Pedersen commitment scheme

A **commitment scheme** allows one to commit to a chosen value *(or chosen statement)* while keeping it hidden to others, with the ability to reveal the committed value later. 

## Algorithm

### Public parameter

**Prime numbers**: $p$ and $q$ satisfying $p \text{ (mod }q) = 1$

**Generator**: $g=g_1^{(p-1) / q} \text{(mod }p)$ 

> [!NOTE] 
> $g_1$ is a generator of the group $\mathbb{Z}_p$.

### Initialization

Choose a **private key**: $1 \leq a \leq q-1$.

Compute: $h = g^a \text{ (mod }p)$

**Send** $h$.

### Commitment

Choose a random integer $1 \leq r \leq q-1$.

Compute: $c = g^x\cdot h^r \text{ (mod } p)$.

$c$ is the commitment and $x$ is the value to commit.

**Send** $c$.

### Opening

The first party open the commitment by sending the value $x$ and $r$ to the second party.

The second party firstly *compute the commitment* $c' = g^x \cdot h^r \text{ (mod }p)$ and then *verify* that $c = c'$.

## Resource
- https://cryptographyacademy.com/identification-schemes/
