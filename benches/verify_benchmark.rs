//! Benchmark de verify.
// TODO: Implementar

use criterion::{criterion_group, criterion_main, Criterion};

fn verify_benchmark(c: &mut Criterion) {
    c.bench_function("verify_noop", |b| {
        b.iter(|| {
            // TODO
        })
    });
}

criterion_group!(benches, verify_benchmark);
criterion_main!(benches);
