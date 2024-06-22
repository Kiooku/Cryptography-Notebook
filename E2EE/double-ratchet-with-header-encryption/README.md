# Double Ratchet Algorithm with header encryption

This is the **header encryption** variant of the [Double Ratchet Algorithm](../double-ratchet-algorithm/).

## Algorithm

> [!NOTE] 
>
> Initializing the two parts requires a `shared_hka` and a `shared_nhkb`.
>
> However, Signal doesn't describe how to generate them, and I haven't not been able to find a resource on the internet explaining the correct way to generate them.
> 
> Making three different X3DH for the `sk`, `shared_hka` and `shared_nhkb` seems overkill. Thuse, I decided to derive `sk` using HKDF to create the `shared_hka` and `shared_nhkb`.
>
> If you find the proper way to do this, I'd be happy to hear from you.

The algorithm is well described on [Signal](https://signal.org/docs/specifications/doubleratchet/#double-ratchet-with-header-encryption).

We can now compare the information that can be intercepted by an attacker:

### 1. Using the Double Ratchet Algorithm without header encryption

```
{ username: "Alice", 
header: 
    Header { 
        dh_pub: PublicKey(MontgomeryPoint([144, 29, 190, 132, 143, 37, 119, 152, 41, 218, 184, 67, 2, 143, 116, 149, 240, 116, 247, 92, 111, 157, 243, 82, 153, 115, 121, 183, 197, 250, 138, 51])), 
        pn: 3, 
        n: 0 }, 
ciphertext: 
    Ciphertext { 
            ciphertext: [218, 243, 173, 68, 220, 198, 46, 18, 118, 0, 60, 115, 60, 124, 156, 54, 104, 170, 132, 176, 86, 127, 78, 21, 116, 18], 
            nonce: [153, 0, 43, 44, 45, 88, 53, 232, 21, 230, 40, 246] }, 
ek_sender: None, 
opk_used: None }
```

***The attacker knows the Diffie-Hellman public key used, the number of messages in previous sending chain and the message number.***

This information does not enable the attacker to decrypt anything, but it does allow him to know the order of messages within a session, for example.

### 2. Using the Double Ratchet Algorithm with header encryption

```
{ username: "Alice", 
header_he: 
    HeaderHE { 
            ciphertext: [45, 32, 136, 65, 70, 181, 37, 10, 1, 76, 210, 38, 96, 140, 210, 154, 122, 3, 109, 171, 139, 24, 88, 45, 236, 39, 82, 202, 64, 197, 70, 27, 150, 149, 216, 242, 79, 241, 192, 167, 143, 202, 232, 159, 182, 39, 164, 62, 19, 65], 
            nonce: [48, 83, 34, 247, 39, 162, 33, 10, 248, 217, 191, 182] }, 
ciphertext: 
    Ciphertext { 
            ciphertext: [226, 196, 140, 206, 61, 87, 31, 237, 214, 209, 205, 25, 29, 86, 242, 127, 221, 165, 42, 128, 180, 129, 181, 130, 5, 76], 
            nonce: [9, 94, 190, 177, 87, 62, 57, 45, 127, 81, 63, 236] }, 
ek_sender: None, 
opk_used: None }
```

With header encryption, the attacker doesn't know the *Diffie-Hellman public key used*, the *number of messages in previous sending chain* and the *message number*.

He therefore has no way of knowing the order of messages within a session.

## Resource
- https://signal.org/docs/specifications/doubleratchet/#double-ratchet-with-header-encryption