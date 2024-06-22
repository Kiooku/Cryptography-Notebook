# One-Time Pad

One-Time Pad *(OTP)* guarantees **perfect secrecy**.

It's Claude Shannon that introduced the concept in 1946.

**Definition of *perfect secrecy***:

"A ciphertext maintains perfect secrecy if the attacker’s knowledge of the contents of the message is the same both before and after the adversary inspects the ciphertext, attacking it with *unlimited resources*. That is, the message gives the adversary precisely no information about the message contents." *([source](https://www.sciencedirect.com/topics/computer-science/perfect-secrecy))*

However, OTP is not practical for our everyday messages, as it requires a key as long as the plaintext and a new random key for each new message or data group.

## Algorithme

The usual form of OTP is the following one: $C = P \oplus K$.

If you're not working on bits, you can use a variant that is the **Vigenère cipher**, however, the key should be at least as long as the message for both of them.

*Vigenère variante*:

> [!NOTE] 
> We use a latin alphabet, hence the modulo 26.

**Encryption**: $C = M + K \text{ (mod } 26)$

**Decryption**: $C = M - K \text{ (mod } 26)$

## Code

I've coded a XOR based one time pad, and as you can see it's pretty straightforward, so I decided to do it in assembly *(which is less straightforward)*.

### Instruction

> [!NOTE] 
> on a Linux machine using NASM

- Execute `./xorBasedOneTimePad`
- Re-build the executable:
    - `nasm -f elf xorBasedOneTimePad.asm`
    - `ld -m elf_i386 -s -o xorBasedOneTimePad xorBasedOneTimePad.o`

The script checks if a message is entered.

Verify if the key is at least as long as the message.

You can let the script create a random key using `/dev/urandom`.

It's a XOR based OTP so you can get a result with strange character.

Example of inputs:
```
Plaintext: hello
Key: FUZZY
Ciphertext: .0666

[OR]

Plaintext: jump
Key TEST
Ciphertext: >0>$
```

# Resources
- Serious Cryptography, Jean-Philippe Aumasson *(OTP)*
- Assembly:
    - https://www.tutorialspoint.com/assembly_programming/index.htm
    - https://wiki.osdev.org/Random_Number_Generator
    - https://stackoverflow.com/questions/55209441/generate-256-numbers-using-dev-urandom-in-assembly