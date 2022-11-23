# Session 1 Exercise Solutions - Srdtrk

## Ch 1

- **Q10.** Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.

>**Solution:**
>
> Consider the example given in $\S1.2$. Suppose that we have an office building where none of the office doors are locked every night. And also, the building has a false ceiling, i.e. you could lift up the ceiling panes and climb over any door or wall.
>
> In this case, if an attacker wanted to gain access to a specific office, they would have to avoid the security guard and just walk in to the office through its door. If we wanted to increase the security of the building by locking all the doors every night, this would increase the likelyhood for an attacker to climb through the false ceiling whereas previously, they would most likely just walk through the door.

## Ch 2

- **Q3.** Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total?

> **Solution:**
>
> $$\sum_{i=1}^{30} (30-i) = 210$$
>
> In general, for $n$ people, a total of $\frac{1}{2}n(n-1)$ keys need to be exchanged.
>
> $$\sum_{i=1}^{n} (n-i) = \frac{1}{2}n(n-1)$$

- **Q4.** Suppose Bob receives a messages signed using a digital signature scheme with Alice's secret signing key. Does it prove that Alice saw the message and chose to sign.

> **Solution:**
>
> No, because a good digital signature scheme is too complicated to be computed by Alice herself. Instead, she has her computer generate the signature. A comprimised computer could sign or trick Alice into signing foreign messages.

- **Q6.** Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?

>**Solution:**
>
> No. There are other types of attacks that could reveal some information about the messages encrypted by the scheme. Some of these other attack methods are discussed in $\S$ 2.6.

- **Q7.** Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack?

> **Solution:**
>
> To perform a birthday attack, the attacker must record approximately $2^{n/2}$ keys (the birthday bound). Thus, we choose $n=256$ to provide 128 bits of security against this attack.

## General

- Suppose you read about RSA encryption and wanted to find it's standard specification. Where would you look?

> **Solution:**
>
> We first look at [here](https://datatracker.ietf.org/doc/html/rfc8017). We may also look at [here](https://github.com/RustCrypto/formats/tree/master/pkcs1) for a Pure Rust format of the specification.

- Find two libraries for each of RSA, TLS/SSL, and AEAD. Evaluate the maturity each library, and skim the code. What about the library structure makes sense? How is their documentation? These links may help:
    - https://cryptography.rs/
    - https://lib.rs/ (librs is equivalent to crates.io, with a different interface)

> **Solution: (RSA)**
>
> We consider the following two libraries for RSA:
> - [RSA](https://github.com/RustCrypto/RSA) by @RustCrypto, and
> - [rsa-oaep-pss](https://github.com/hakhenaton/rsa-oaep-pss/) by @Hakhenaton
>
> A quick glance indicates that the library by @RustCrypto is more matured since it is audited, seems to have better documentation, and has much more contributors and users.

> **Solution: (TLS/SSL)**
>
> - [rustls](https://github.com/rustls/rustls) by @rustls, and
> 
> This seems to be the standard tls library used by rust developers ranking #1 most popular crate in Network Programming category. Most other tls packages seems to be focused on niche hardware requirements.

> **Solution: (AEAD)**
>
> We consider two implementations of the similar AEADs.
> - [chacha20poly1305](https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305) by @RustCrypto, and
> - [chacha](https://github.com/PeterReid/chacha) by @PeterReid
>
> Documentation is lacking in PeterReid's package and it hasn't been updated in over 4 years. Also, @RustCrypto's package is audited.

- Benchmark the speed of an algorithm in the two different implementations with [Criterion](https://lib.rs/crates/criterion).

> **Solution:**
>
> For this exercise, I decided to benchmark two rsa-oaep algorithms from
>
> - [RSA](https://github.com/RustCrypto/RSA) by @RustCrypto, and
> - [rsa-oaep-pss](https://github.com/hakhenaton/rsa-oaep-pss/) by @Hakhenaton
>
> The details of the benchmark can be found in the Readme file [here](https://github.com/srdtrk/open-cryptography-course/tree/main/session-1/benchmark). Surprisingly, the more mature package turned out to be about 2.6 times slower than the latter. I'd still use the more mature package because I choose security over performance.

- You're implementing a [Tweakable Encryption](https://en.wikipedia.org/wiki/Disk_encryption_theory) scheme. You need to know what standard API users will expect. Find a reference for the standard API and write the function signatures for encryption and decryption.

> **No Solution: This exercise was depreciated.**

- You want to understand a paper on a new polynomial commitment scheme, but you've been trying for more than an hour, and the math is over your head. What do you do?

> Study the math or ask a colleague that know more about it.

- Implement the [Vignère cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher) in 100 lines or less.

> **Solution:**
>
> https://github.com/srdtrk/vigenere-cipher

- What is a side channel attack? Is your cipher implementation constant time?

> **Solution:**
>
> A side-channel attack is when the attacker has one or more additional channels of information about the system. For example (from $\S$ 8.5), an attacker that can make detailed measurements of the time it takes to encrypt a message.
>
> No, the longer the message is, the longer it takes to encrypt it. Our vigenere cipher should be linear in time with respect to the length of the plaintext.
- Extra: Read [New Directions in Cryptography](https://ieeexplore.ieee.org/document/1055638).
- Extra: Consider ways to contribute what you learned this week to the [Uncloak](https://uncloak.org) knowledge graph.