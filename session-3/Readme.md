# Session 2 Exercise Solutions - Srdtrk

**Exercise 5.3** Consider SHA-512-n, a hash function that first runs SHA-512 and then outputs only the first $n$ bits of the result. Write a program that uses a birthday attack to find and output a collision on SHA-512-n, where n is a multiple of 8 between 8 and 48. Your program may use an existing cryptography library. Time how long your program takes when n is 16, averaged over five runs for each $n.$ How long would you expect your program to take for SHA-512-256? For SHA-512?

> **Solution:**
>
> You can find the attack in [here](). The following is the location of the collision.
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

```
Benchmarking sha-512-8/birthday-attack: Collecting 10 samples in estimated 5.0001 s (2.4M iterat                                                                                                sha-512-8/birthday-attack                        
                        time:   [2.1030 µs 2.1156 µs 2.1351 µs]
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe

Benchmarking sha-512-16/birthday-attack: Collecting 10 samples in estimated 5.0025 s (49k iterat                                                                                                sha-512-16/birthday-attack                        
                        time:   [100.54 µs 100.82 µs 101.15 µs]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking sha-512-24/birthday-attack: Collecting 10 samples in estimated 5.0567 s (3245 itera                                                                                                sha-512-24/birthday-attack                        
                        time:   [1.5391 ms 1.5535 ms 1.5840 ms]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking sha-512-32/birthday-attack: Collecting 10 samples in estimated 6.0244 s (220 iterat                                                                                                sha-512-32/birthday-attack                        
                        time:   [27.185 ms 27.589 ms 28.268 ms]
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe

Benchmarking sha-512-40/birthday-attack: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 5.3s.
Benchmarking sha-512-40/birthday-attack: Collecting 10 samples in estimated 5.3280 s (10 iterati                                                                                                sha-512-40/birthday-attack                        
                        time:   [530.83 ms 535.86 ms 543.29 ms]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe
```
