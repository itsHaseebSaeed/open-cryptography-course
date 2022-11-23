# Benchmarking Exercise - Session 1

For this exercise, I benchmarked two rsa-oaep algorithms, trying to give them a level playing field as much as possible. The two packages compared here are:

- [RSA](https://github.com/RustCrypto/RSA) by @RustCrypto, and
- [rsa-oaep-pss](https://github.com/hakhenaton/rsa-oaep-pss/) by @Hakhenaton

The former of which is a way more mature package. You can see my benchmarking implementation in [my_benchmark.rs](https://github.com/srdtrk/open-cryptography-course/blob/main/session-1/benchmark/benches/my_benchmark.rs).

To run the benchmark, run

```ignore
cargo bench
```

This gives us the following output:

```ignore
Benchmarking rsa-encrypt 'oaep-sha256': Collecting 100 samples in estimated 5.53                                                                                rsa-encrypt 'oaep-sha256'                        
                        time:   [157.07 µs 157.19 µs 157.35 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

Benchmarking rsa_oaep_pss-encrypt 'oaep-sha256': Collecting 100 samples in estim                                                                                rsa_oaep_pss-encrypt 'oaep-sha256'                        
                        time:   [59.113 µs 59.218 µs 59.327 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
```

Here, the more mature package's test is labeled `rsa-encrypt 'oaep-sha256'`. And, surprisingly, the mature package is about 2.6 times slower than the latter package! I'd still use the more mature package due to it being audited and it actually being used by professionals.
