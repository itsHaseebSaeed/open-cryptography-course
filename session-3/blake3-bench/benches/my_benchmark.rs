use blake3_bench::{blake3, sha2_256, sha3_256};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let message = b"this message is made for hashing";
    c.bench_function("blake3", |b| b.iter(|| blake3(black_box(message))));
    c.bench_function("sha2-256", |b| b.iter(|| sha2_256(black_box(message))));
    c.bench_function("sha3-256", |b| b.iter(|| sha3_256(black_box(message))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
