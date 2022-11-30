# Session 2 Exercise Solutions - Srdtrk

## Ch 3

- **Q1.** How much space would be required to store a table for an entire idealized block cipher that operates on 64-bit blocks and has 80-bit keys?

>**Solution:**
>
> For a fixed key, an entire idealized block cipher that operates on n-bit blocks require
> $$2^n\cdot n$$
> bits of storage. We have to store a block cipher for each key possible key, thus we need a total of,
> $$ 2^{80}\cdot 2^{64}\cdot 64 $$
> bits of storage. Which is a number so large that I don't even know a proper name for it.

- **Q5.** Suppose you have a processor that can perform a single DES encryption or decryption operation in $2^{-26}$ seconds. Suppose you also have a large number of plaintext-ciphertext pairs for DES under a single unknown key. How many hours would it take, on average, to find that DES key, using an exhaustive search approach and a single processor? How many hours would it take, with a collection of $2^{14}$ processors?

>**Solution:**
>
> DES has a key size of 56 bits. Suppose we run our exhaustive search without any key repetition (as we should). Let $N$ be the number of searches we have to run until we find the match. Then,
> $$ \mathbb P(N=i) = \frac{1}{2^{56}} \text{ for } 1\leq i\leq 2^{56}.$$
> $$ \therefore\mathbb E(N) = \sum_{i=1}^{2^{56}} i\cdot\frac{1}{2^{56}} = \frac{2^{56}+1}{2}\approx 2^{55}.$$
> Thus, with one processor, it would take about
> $$ 2^{29}\text{ seconds} \approx 149130\text{ hours.}$$
> Now suppose that $2^{14}$ processors divide the search space evenly among each other. This would mean effectively the search space is reduced to $2^{56-14} = 2^{42}$. Then, the expected number of trials the winning processor has to go through is
> $$ \therefore\mathbb E(N) \approx 2^{41}. $$
> Therefore, it would take about
> $$ 2^{15}\text{ seconds} \approx 9.1\text{ hours}.$$

- **Q6.** Consider a new block cipher, DES2, that consists only of two rounds of the DES block cipher. DES2 has the same block and key size as DES. For this question you should consider the DES $F$ function as a black box that takes two inputs, a 32-bit data segment and a 48-bit round key, and that produces a 32-bit output. Suppose you have a large number of plaintext-ciphertext pairs for DES2 under a single, unknown key. **Give** an algorithm for recovering the 48-bit round key for round 1 and the 48-bit round key for round 2. Your algorithm should require fewer operations than an exhaustive search for an entire 56-bit DES key. Can your algorithm be converted into a distinguishable attack against DES2?

>**Solution:**
>
> (Using notation from the book), the idea here is that we can brute force both $K_1$ and $K_2$, each of which are 48-bit round keys. The key observation is that knowing both the ciphertext and the plaintext allows us to deduce the output of the DES function $F$ in both rounds (we also know their inputs), which leaves only the 48-bit $K_i$ unknown, which we proceed to brute force.
>
> Let $P$ be a plaintext, and let $(L_P, R_P)$ be the splitting of $P$ into the 32-bit halves.
>
> Let $C$ be the ciphertext of $P$, and $(L_C, R_C)$ be the splitting of $C$ with the same algoritm used to split $P$.
>
> Then notice that, according to DES2, we have:
> $$ F(K_1, R_P) = L_P\ \oplus\ R_C$$
> and, $$ F(K_2, R_C) = R_P\ \oplus\ L_C$$
> Thus we can simply brute force $K_1$, and $K_2$. In this case, we have at most $2^{48}$ unique keys to go through (not $2^{49}$ since we can brute force both rounds at the same time), and in the case of standard exhaustive search this would have been $2^{56}$, so indeed, this is more efficient.
>
> And this algorithm (I will not provide a pseudocode as I think the above description suffices), can be turned into a distinguishing attack. Let's show why. Suppose we have two block ciphers (on same block size and key size). First, we pick a random key, then we use this key to generate plaintext ciphertext pairs. Then we compute $K_1$ and $K_2$ using the public key scheduling algorithm. Then we check whether:
> $$ F(K_1, R_P) = L_P\ \oplus\ R_C$$
> and, $$ F(K_2, R_C) = R_P\ \oplus\ L_C$$
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
> This is decrypted in [here](). It is `80706050403020100807060504030201`

**Q9.** Using an existing cryptography library, encrypt the following plaintext (in hex)

```ignore
29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D
```

with the following 256-bit key from problem 8, using AES. Then re-encrypt and decrypt it using a 3072-bit RSA key with GnuPG, or your choice of asymmetric crypto CLI.

>**Solution:**
>
> The plaintext is encrypted [here](). It is `80000000000000000000000000000001`. We used gpg to produce [encrypted.gpg]() from [gpg-plaintext.txt]()
