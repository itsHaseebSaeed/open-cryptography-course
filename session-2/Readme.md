# Session 2 Exercise Solutions - Srdtrk

## Ch 3

**Q1.** How much space would be required to store a table for an entire idealized block cipher that operates on 64-bit blocks and has 80-bit keys?

>**Solution:**
>
> For a fixed key, an entire idealized block cipher that operates on n-bit blocks require
> $$2^n\cdot n$$
> bits of storage. We have to store a block cipher for each key possible key, thus we need a total of,
> $$2^{80}\cdot 2^{64}\cdot 64 $$
> bits of storage. Which is a number so large that I don't even know a proper name for it.

**Q5.** Suppose you have a processor that can perform a single DES encryption or decryption operation in $2^{-26}$ seconds. Suppose you also have a large number of plaintext-ciphertext pairs for DES under a single unknown key. How many hours would it take, on average, to find that DES key, using an exhaustive search approach and a single processor? How many hours would it take, with a collection of $2^{14}$ processors?

>**Solution:**
>
> DES has a key size of 56 bits. Suppose we run our exhaustive search without any key repetition (as we should). Let $N$ be the number of searches we have to run until we find the match. Then,
> $$\mathbb P(N=i) = \frac{1}{2^{56}} \text{ for } 1\leq i\leq 2^{56}$$
> $$\therefore\mathbb E(N) = \sum_{i=1}^{2^{56}} i\cdot\frac{1}{2^{56}} = \frac{2^{56}+1}{2}\approx 2^{55}$$
> Thus, with one processor, it would take about
> $$2^{29}\text{ seconds} \approx 149130\text{ hours.}$$
> Now suppose that $2^{14}$ processors divide the search space evenly among each other. This would mean effectively the search space is reduced to $2^{56-14} = 2^{42}$. Then, the expected number of trials the winning processor has to go through is
> $$\therefore\mathbb E(N) \approx 2^{41}$$
> Therefore, it would take about
> $$2^{15}\text{ seconds} \approx 9.1\text{ hours}$$

**Q6.** Consider a new block cipher, DES2, that consists only of two rounds of the DES block cipher. DES2 has the same block and key size as DES. For this question you should consider the DES $F$ function as a black box that takes two inputs, a 32-bit data segment and a 48-bit round key, and that produces a 32-bit output. Suppose you have a large number of plaintext-ciphertext pairs for DES2 under a single, unknown key. **Give** an algorithm for recovering the 48-bit round key for round 1 and the 48-bit round key for round 2. Your algorithm should require fewer operations than an exhaustive search for an entire 56-bit DES key. Can your algorithm be converted into a distinguishable attack against DES2?

>**Solution:**
>
> (Using notation from the book), the idea here is that we can brute force both $K_1$ and $K_2$, each of which are 48-bit round keys. The key observation is that knowing both the ciphertext and the plaintext allows us to deduce the output of the DES function $F$ in both rounds (we also know their inputs), which leaves only the 48-bit $K_i$ unknown, which we proceed to brute force.
>
> Let $P$ be a plaintext, and let $(L_P, R_P)$ be the splitting of $P$ into the 32-bit halves.
>
> Let $C$ be the ciphertext of $P$, and $(L_C, R_C)$ be the splitting of $C$ with the same algoritm used to split $P$.
>
> Then notice that, according to DES2, we have:
> $$F(K_1, R_P) = L_P\ \oplus\ R_C$$
> and, $$F(K_2, R_C) = R_P\ \oplus\ L_C$$
> Thus we can simply brute force $K_1$, and $K_2$. In this case, we have at most $2^{48}$ unique keys to go through (not $2^{49}$ since we can brute force both rounds at the same time), and in the case of standard exhaustive search this would have been $2^{56}$, so indeed, this is more efficient.
>
> And this algorithm (I will not provide a pseudocode as I think the above description suffices), can be turned into a distinguishing attack. Let's show why. Suppose we have two block ciphers (on same block size and key size). First, we pick a random key, then we use this key to generate plaintext ciphertext pairs. Then we compute $K_1$ and $K_2$ using the public key scheduling algorithm. Then we check whether:
> $$F(K_1, R_P) = L_P\ \oplus\ R_C$$
> and, $$F(K_2, R_C) = R_P\ \oplus\ L_C$$
> for whichever block cipher this is false, it must be the ideal block cipher.

**Q8.** Familiarize yourself with a cryptographic CLI tools. A popular open source package is [OpenSSL](https://docs.rs/openssl/latest/openssl/aes/index.html). Using an existing cryptographic library, decrypt the following ciphertext (in hex)

```ignore
53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07
```

 with the following 256-bit key (also in hex):

```ignore
80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
```

using AES.

>**Solution:**
>
> This is decrypted in [here](https://github.com/srdtrk/open-cryptography-course/blob/main/session-2/openssl-example/src/main.rs). It is `80706050403020100807060504030201`

**Q9.** Using an existing cryptography library, encrypt the following plaintext (in hex)

```ignore
29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D
```

with the following 256-bit key from problem 8, using AES. Then re-encrypt and decrypt it using a 3072-bit RSA key with GnuPG, or your choice of asymmetric crypto CLI.

>**Solution:**
>
> The plaintext is encrypted [here](https://github.com/srdtrk/open-cryptography-course/blob/main/session-2/openssl-example/src/main.rs). It is `80000000000000000000000000000001`. We used gpg to produce [encrypted.gpg](https://github.com/srdtrk/open-cryptography-course/blob/main/session-2/encrypted.gpg) from [gpg-plaintext.txt](https://github.com/srdtrk/open-cryptography-course/blob/main/session-2/gpg-plaintext.txt)

**Q10.** Write a program that experimentally demonstrates the complementation property for DES. This program should take as input a key $K$ and a plaintext $P$ and demonstrate that the DES complementation property holds for this key and plaintext. You may use an existing cryptography library for this exercise.

> **Solution:**
>
> The test I've written in [here](https://github.com/srdtrk/open-cryptography-course/blob/main/session-2/openssl-example/src/main.rs) passes.

## Chapter 4 (p. 107)

**Q1.** Let $P$ be a plaintext and let $\ell(P)$ be the length of $P$ in bytes. Let $b$ be the block size of the block cipher in bytes. Explain why the following is not a good padding scheme: 
    - Determine the minimum number of padding bytes necessary in order to pad the plaintext to a block boundary. This is a number $n$ which satisfies $0 ≤ n ≤ b − 1$ and $n + l(P)$ is a multiple of $b$. Pad the plaintext by appending $n$ bytes, each with value $n$.

> **Solution:**
>
> If the plaintext's last couple of bytes happens to have the value $n$, then the decryption algorithm might not be able to tell where the padding begins and the plaintext ends.

**Q3.** Suppose you, as an attacker, observe the following 32-byte ciphertext $C$ (in hex)

```hex
46 64 DC 06 97 BB FE 69 33 07 15 07 9B A6 C2 3D
2B 84 DE 4F 90 8D 7D 34 AA CE 96 8B 64 F3 DF 75
```

and the following 32-byte ciphertext $C'$ (also in hex)

```hex
51 7E CC 05 C3 BD EA 3B 33 57 0E 1B D8 97 D5 30
7B D0 91 6B 8D 82 6B 35 B7 8B BB 8D 74 E2 C7 3B.
```

Suppose you know these ciphertexts were generated using CTR mode with the same nonce. The nonce is implicit, so it is not included in the ciphertext. You also know that the plaintext $P$ corresponding to $C$ is

```hex
43 72 79 70 74 6F 67 72 61 70 68 79 20 43 72 79
70 74 6F 67 72 61 70 68 79 20 43 72 79 70 74 6F.
```

What information, if any, can you infer about the plaintext $P'$ corresponding to $C'$?

> **Solution:**
>
> We can figure out $P'$ immediately. Assuming that the block size of the underlying cipher is 32-bytes, we have
> $$K_1 = E(K, \text{Nonce}\ ||\ 1)$$
> $$C = P\ \oplus\ K_1$$
> $$C' = P'\ \oplus\ K_1$$
> $$\therefore\ C\ \oplus\ C' = P\ \oplus\ P'$$
> Then, solving for $P'$ gives `54686973206973206120736563726574202020436f6e666964656e7469616c21`. Which when converted to text gives `This is a secret   Confidential!`

**Q4.** The ciphertext (in hex)

```hex
87 F3 48 FF 79 B8 11 AF 38 57 D6 71 8E 5F 0F 91
7C 3D 26 F7 73 77 63 5A 5E 43 E9 B5 CC 5D 05 92
6E 26 FF C5 22 0D C7 D4 05 F1 70 86 70 E6 E0 17
```

was generated with the 256-bit AES key (also in hex)

```hex
80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
```

using CBC mode with a random IV. The IV is included at the beginning of the ciphertext. Decrypt this ciphertext. You may use an existing cryptography library for this exercise.

> **Solution:**
>
> This is decrypted in [here](https://github.com/srdtrk/open-cryptography-course/blob/main/session-2/openssl-example/src/main.rs). I took the first 16 bytes to be the iv. In which case we have the following plaintext `416e6f7468657220736563726574212020416e6420616e6f746865722e202020`. Which when converted to text gives: `Another secret!  And another.`

**Q6.** Let $P_1$, $P_2$ be a message that is two blocks long, and let $P'_1$ be a message that is one block long. Let $C_0, C_1, C_2$ be the encryption of $P_1, P_2$ using CBC mode with a random IV and a random key, and let $C'_0, C'_1$ be the encryption of $P'_1$ using CBC mode with a random IV and the same key. Suppose an attacker knows $P_1, P_2$ and suppose the attacker intercepted and thus know $C_0, C_1, C_2$ and $C'_0, C'_1$. Further suppose that, by random chance, $C'_1 = C_2$. Show that the attacker can compute $P'_1$.

> **Solution:**
>
> We know that
> $$C'_1 = E(K, P'_1\ \oplus\ C'_0)\text{, and}$$
> $$C_2 = E(K, P_2\ \oplus\ C_1)$$
> Then, applying decryption to $C'_1 = C_2$ gives us
> $$P'_1 = P_2\ \oplus\ C_1\ \oplus\ C'_0$$

## General

- Implement a pair of functions: A [PKCS](https://en.wikipedia.org/wiki/PKCS_7) message padding function, and a padding validation function that takes a message and validates whether it has a correct padding.