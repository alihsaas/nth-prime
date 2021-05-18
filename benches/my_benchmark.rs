use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("100000th prime", |b| b.iter(|| nth_prime::get_nth_prime(black_box(100000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
