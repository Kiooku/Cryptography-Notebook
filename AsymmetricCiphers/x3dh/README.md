# X3DH *(Extended Triple Diffie-Hellman)* Key Agreement Protocol

"X3DH establishes a shared secret key between two parties who mutually authenticate each other based on public keys. X3DH provides **forward secrecy** and **cryptographic deniability**.

X3DH is designed for **asynchronous settings** where one user ("Bob") is offline but has published some information to a **server**. Another user ("Alice") wants to use that information to send encrypted data to Bob, and also establish a shared secret key for future communication." 

*[(source)](https://signal.org/docs/specifications/x3dh/#introduction)*

This algorithm is recommended for implementing the **Double Ratchet algorithm** *(E2EE)*  initialization.

> **Note**: There's a rust library for [x3dh](https://github.com/dione-software/x3dh-ke), however, the library use the **NIST P-256** elliptic curve instead of **Curve25519** or **Curve448** according to Signal recommendations.
>
> The use of another curve than the one expected lead to different key size, which make the implementation of the Double Ratchet algorithm using the recommended curve impossible. That's why I decided to implemented my own X3DH using the **Curve25519** for learning purpose.

## X3DH parameters used

|       |  Parameters    |
|-------|----------------|
| curve | Curve25519     |
| hash  | SHA-512        |
| info  | RedWheelbarrow |

## Algorithm

The algorithm is well described on [Signal](https://signal.org/docs/specifications/x3dh/).

However, I have tried to draw a diagram to make it easier to understand the protocol.

## Resource
- https://signal.org/docs/specifications/x3dh/