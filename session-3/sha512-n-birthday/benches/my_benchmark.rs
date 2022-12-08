use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sha512_n_birthday::sha512_n_bytes_bithday_attack;

fn bench(c: &mut Criterion) {
    for n in 1..=6_usize {
        let group_name = format!("sha-512-{}", n * 8);
        let mut group = c.benchmark_group(group_name);
        group.sample_size(10);
        group.bench_function("birthday-attack", |b| {
            b.iter(|| sha512_n_bytes_bithday_attack(black_box(n)))
        });
        group.finish();
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
