//! Benchmark de YAML parsing.
// TODO: Implementar

use criterion::{criterion_group, criterion_main, Criterion};

fn yaml_parsing_benchmark(c: &mut Criterion) {
    c.bench_function("yaml_noop", |b| b.iter(|| {
        // TODO
    }));
}

criterion_group!(benches, yaml_parsing_benchmark);
criterion_main!(benches);
