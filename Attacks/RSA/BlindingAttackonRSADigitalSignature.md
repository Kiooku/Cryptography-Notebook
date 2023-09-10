# Blinding Attack on RSA Digital Signature

## Attack goal

Oracle that does not want to sign a particular message (i.e.  "admin=True") with RSA digital signature.

We want to get the signature of this particular message.

---

## Attack

### Definitions

**M**: Message not authorized by Oracle.
**M'**: Modified message.
**r**: Blinding factor.
**S**: Message signature not authorized by Oracle.
**S'**: Message signature of the modified message.

### Scenario

#### Step 1:

Create a modified message **M'**.

$$M' = r^eM \text{ (mod } N)$$

$r$ is a prime number (i.e. $r = 2$).

#### Step 2:

Sign the modified message.

$$S' = (M')^d \text{ (mod } N)$$

##### Details
$$ S' = (r^eM)^d \text{ (mod } N) $$
$$ S' = r^{ed}M^d \text{ (mod } N) $$
$$ S' = rM^d \text{ (mod } N) $$

#### Step 3:

Forge the original signature.

$$ S = r^{-1} \cdot S' \text{ (mod } N)$$

##### Details
$$ S = r^{-1} \cdot rM^d \text{ (mod } N)$$
$$ S = M^d \text{ (mod } N)$$

---

## How to prevent the attack?

To prevent this attack, we can hash the message before signing it.

Moreover, this makes it easy to sign long messages, as each hashed message will have the same length.

---

## Resources
- https://masterpessimistaa.wordpress.com/2017/07/10/blinding-attack-on-rsa-digital-signatures/
- https://wiki.bi0s.in/crypto/rsa-blinding-attack/
