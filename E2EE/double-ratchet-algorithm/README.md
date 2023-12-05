# Double Ratchet Algorithm

"The Double Ratchet algorithm is used by two parties to exchange encrypted messages based on a shared secret key."

*[(source)](https://signal.org/docs/specifications/doubleratchet/#introduction)*

This protocol is used to create an End to End Encryption (E2EE).

## Double Ratchet parameters used

|       |  Parameters     |
|-------|-----------------|
| curve | Curve25519      |
| hash  | SHA-256         |
| aead  | AES-GCM-SIV-256 |
| init  | X3DH (Curve25519, SHA-256) |

## Algorithm

The algorithm is well described on [Signal](https://signal.org/docs/specifications/doubleratchet/).

## Resource
- https://signal.org/docs/specifications/doubleratchet/