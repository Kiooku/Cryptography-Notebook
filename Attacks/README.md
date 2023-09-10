# Definitions of cryptographic attacks

> **Warning** There will only be descriptions of attacks against cryptosystems. The implementation will not be included in order to respect the spirit of CTF.

## Chosen ciphertext attack

> **Note** Some examples will follow later

*"A chosen ciphertext attack can gather information by obtaining the decryption of chosen ciphertexts. From these pieces of information the adversary can attempt to recover the hidden secret key used for decryption."*

## Oracle attack

<ins>General definition:</ins> *In the context of cryptography, an oracle is a black box or a computational entity that provides certain services or functionalities related to a cryptographic algorithm.*

### Decryption Oracle

> **Note** It's not an attack, but it can be an attack vector.

A decryption oracle is a computational tool or a function that is capable of performing decryption operations on ciphertexts.

A decryption oracle specifically focuses on the decryption process. It takes an encrypted ciphertext as input and applies the necessary cryptographic operations to recover the original plaintext. The oracle has access to the required decryption key or possesses the knowledge necessary to perform the decryption operation.

*In most cryptographic systems, a decryption oracle is not openly available to everyone. It is typically a controlled resource accessible only by authorized entities who possess the appropriate cryptographic keys or credentials.*

### Padding oracle attack

> **Note** Need to write this section

### Code example

- [Diffie-Hellman Oracle on Elgamal PKC *(use decryption oracle)*](../AsymmetricCiphers/ElGamal/README.md#attacks-using-oracle)

## Small Subgroup Confinement Attack *(Diffie-Hellman)*

*[Click here to see the repository that explain this attack](https://github.com/Kiooku/Small-Subgroup-Confinement-Attack)*

## Resources

 - *[Chosen ciphertext attack](https://en.wikipedia.org/wiki/Chosen-ciphertext_attack)*
 - *[Oracle attack](https://en.wikipedia.org/wiki/Oracle_attack)*