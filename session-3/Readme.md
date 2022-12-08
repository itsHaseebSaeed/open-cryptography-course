# Session 2 Exercise Solutions - Srdtrk

## Chapter 5

**Exercise 5.3** Consider SHA-512-n, a hash function that first runs SHA-512 and then outputs only the first $n$ bits of the result. Write a program that uses a birthday attack to find and output a collision on SHA-512-n, where n is a multiple of 8 between 8 and 48. Your program may use an existing cryptography library. Time how long your program takes when n is 16, averaged over five runs for each $n.$ How long would you expect your program to take for SHA-512-256? For SHA-512?

> **Solution:**
>
> You can find the attack in [here](https://github.com/srdtrk/open-cryptography-course/blob/main/session-3/sha512-n-birthday/src/lib.rs). The following is the location of the collision.
>
> ```ignore
> Collision of sha-512-8 found at (7, 4)
> Collision of sha-512-16 found at (341, 335)
> Collision of sha-512-24 found at (5165, 1182)
> Collision of sha-512-32 found at (84939, 35)
> Collision of sha-512-40 found at (1081013, 812104)
> Collision of sha-512-48 found at (29984355, 8978078)
> ```
>
> Based on the results bellow, making an extremely conservative estimate (by many orders of magnitude), at least $10^{20}$ years in my computer. Note that I didn't put much effort to making this estimate. You can find the benchmarking [here](https://github.com/srdtrk/open-cryptography-course/blob/main/session-3/sha512-n-birthday/benches/my_benchmark.rs). Note that I used 10 trials instead of 5 for each.

```
Benchmarking sha-512-8/birthday-attack: Collecting 10 samples in estimated 5.0000 s (2.4M iterat                                                                                                sha-512-8/birthday-attack                        
                        time:   [2.1065 µs 2.1105 µs 2.1156 µs]
                        change: [-9.1710% -4.0460% -0.3248%] (p = 0.12 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking sha-512-16/birthday-attack: Collecting 10 samples in estimated 5.0004 s (48k iterat                                                                                                sha-512-16/birthday-attack                        
                        time:   [103.55 µs 103.82 µs 104.52 µs]
                        change: [-7.4320% -2.1375% +2.8969%] (p = 0.51 > 0.05)
                        No change in performance detected.
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking sha-512-24/birthday-attack: Collecting 10 samples in estimated 5.0552 s (3190 itera                                                                                                sha-512-24/birthday-attack                        
                        time:   [1.5632 ms 1.5728 ms 1.5852 ms]
                        change: [-6.8903% -1.9931% +4.3486%] (p = 0.57 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe

Benchmarking sha-512-32/birthday-attack: Collecting 10 samples in estimated 6.1265 s (220 iterat                                                                                                sha-512-32/birthday-attack                        
                        time:   [27.341 ms 27.385 ms 27.477 ms]
                        change: [-8.3497% -2.5062% +3.7529%] (p = 0.49 > 0.05)
                        No change in performance detected.
Found 2 outliers among 10 measurements (20.00%)
  2 (20.00%) high severe

Benchmarking sha-512-40/birthday-attack: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 6.0s.
Benchmarking sha-512-40/birthday-attack: Collecting 10 samples in estimated 5.9902 s (10 iterati                                                                                                sha-512-40/birthday-attack                        
                        time:   [537.19 ms 556.01 ms 578.59 ms]
                        change: [-4.6953% -0.5600% +3.7070%] (p = 0.82 > 0.05)
                        No change in performance detected.

Benchmarking sha-512-48/birthday-attack: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 185.2s.
Benchmarking sha-512-48/birthday-attack: Collecting 10 samples in estimated 185.20 s (10 iterati                                                                                                sha-512-48/birthday-attack                        
                        time:   [21.986 s 22.378 s 22.914 s]
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe
```

**Exercise 5.4** Let SHA-512-n be as in the previous exercise. Write a program that finds a message M that hashes to the following value under SHA-512-16 (in hex):  `3D 4B`. How many tries would you expect the algorithm to need? Running the algorithm 5 times, How many tries did it take on average?

> **Solution:**
>
> I think it would take, on average, $2^{16} = 65536$ tries. In my 5 trials [here](https://github.com/srdtrk/open-cryptography-course/blob/main/session-3/sha512-n-birthday/src/lib.rs), I got an average of 84962 tries.

**Exercise+** With command line tools or Criterion, benchmark the [blake3 hash](https://docs.rs/blake3/latest/blake3/) (default is 256 bit output), and compare it to benches of [SHA3-256](https://docs.rs/sha3/latest/sha3/) and [SHA-256](https://docs.rs/sha2/latest/sha2/) (when written without a number, SHA is assumed to be SHA2).

> **Solution:**
>
> You can find the benchmark here, and run it with `cargo bench`. We find that blake3 is the fastest with sha2 being a close second, and sha3 being the slowest.

```ignore
blake3                  time:   [53.667 ns 53.694 ns 53.722 ns]                   
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

sha2-256                time:   [56.701 ns 56.730 ns 56.761 ns]                     
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

sha3-256                time:   [327.52 ns 327.71 ns 327.91 ns]                     
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  3 (3.00%) high mild
  5 (5.00%) high severe
```

## Chapter 6

**Exercise 6.3** Suppose a and b are both one block long, and suppose the sender MACs a, b, and $a || b$ with CBC-MAC. An attacker who intercepts the MAC tags for these messages can now forge the MAC for the message $m=b || (M(b) ⊕ M(a) ⊕ b)$, which the sender never sent. The forged tag for this message is equal to $M(a || b)$, the tag for $a || b$. Justify mathematically why this is true.

> **Solution**
>
> We will prove that both $M(m)$ and $M(a || b)$ are equal to $E_K(M(a)\ \oplus\ b)$.
>
> **(i) Proof that $M(a||b) = E_K(M(a)\ \oplus\ b)$**:
>
> Let $m' = a||b$ be our plaintext message $P'$. Then since both $a$ and $b$ are one block long, we have $P'_1 = a$ and $P'_2 = b$. Then, by definition:
> $$M(m') = E_K(b\ \oplus\ H'_1)$$
> where $H'_1 = E_K(a\ \oplus \text{IV}) = E_K(a) = M(a)$ since $\text{IV}$ is fixed at $0$.
> $$\therefore M(m') = E_K(b\ \oplus M(a))$$
>
> **(ii) Proof that $M(m) = E_K(M(a)\ \oplus\ b)$**: 
>
> Let $m$ be our plaintext message $P$. Then since both $b$ and $(M(b) ⊕ M(a) ⊕ b)$ are one block long, we have $P_1 = b$ and $P_2 = M(b) ⊕ M(a) ⊕ b$. Then, by definition
> $$M(m) = E_K(M(b) ⊕ M(a) ⊕ b\ \oplus\ H_1)$$
> where $H_1 = E_K(b\ \oplus\ \text{IV}) = E_K(b) = M(b)$ since $\text{IV}$ is fixed at $0$.
> $$\therefore M(m) = E_K(M(b) ⊕ M(a) ⊕ b\ \oplus\ M(b)) = E_K(M(a)\ \oplus b)$$
> We are done.

**Exercise 6.4** Suppose message $a$ is one block long. Suppose that an attacker has received the MAC $t$ for $a$ using CBC-MAC under some random key unknown to the attacker. Explain how to forge the MAC for a two-block message of your choice. What is the two-block message that you chose? What is the tag that you chose? Why is your chosen tag a valid tag for your two-block message?

> **Solution:**
>
> The message we choose is $m' = a\ ||\ (a\ \oplus\ M(a))$. Here, the first block is $P'_1 = a$, and the second block is $P'_2 = a\ \oplus\ M(a)$. The tag we choose is $t' = t = M(a)$. Here is the proof that this tag matches the CBC-MAC of $m'$:
>
> $$M(m') = E_K(P_2\ \oplus H_1) = E_K(a\ \oplus\ M(a)\ \oplus M(a)) = E_K(a) = M(a) = t = t'$$

**Exercise 6.5** Using an existing cryptography library, compute the MAC of the message:

```hex
4D 41 43 73 20 61 72 65 20 76 65 72 79 20 75 73 65 66 75 6C 20 69 6E 20 63 72 79 70 74 6F 67 72 61 70 68 79 21 20 20 20 20 20 20 20 20 20 20 20
```

using CBC-MAC with AES and the 256-bit key:

```hex
80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
```

> **Solution:**
>
> According to my computation in [here](), the mac is `0d820e3a1e105d307216fc00c7a5b449`.