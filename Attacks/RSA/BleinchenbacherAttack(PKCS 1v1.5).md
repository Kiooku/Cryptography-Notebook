# Bleinchenbacher's attack (PKCS 1 v1.5)

## Attack goal

Attack on SSL, using oracle error message..

The goal is to recover the message.

---

## Attack

In **PKCS#1 v1.5** padding look like that: `00 02 [padding string] 00 [data block]`.

1. Eve capture the cipher in the handshake and which contains the SSL pre-shared key ($M$): $C = M^e \text{ (mod } N)$.

2. She tampered the ciphertext: $C' = C \cdot s^e \text{ (mod } N)$.

3. The server decrypts and gets: $$M' = (C(s^e))^d \text{ (mod } N) = C^d \cdot  s^{ed} \text{ (mod }N) = M \cdot s \text{ (mod } N)$$
4. $M = \frac{C'}{s}$


When the server reads this, the first two bytes are likely to be incorrect, so it responds with an **error message**.
Then Eve try with different $s$ values, until the server gives her a positive response.

As we have $16 $bits at start, it will take use between $30 000$ ($1$ in $2^{15} which is 1-in-32728) and $130000$ attelots ($1$ in $2^{17}$ which is 1-in-131073) to get a successful access

---

## How to prevent the attack?

To prevent this attack, the SSL servers do not inform the client about padding woes. 

If decryption fails because of a bad padding, then the server continues with a random pre-master secret (the true failure will then occur when processing the `Finished` message).

---

## Resources
- https://asecuritysite.com/encryption/c_c3
- https://crypto.stackexchange.com/questions/12688/can-you-explain-bleichenbachers-cca-attack-on-pkcs1-v1-5