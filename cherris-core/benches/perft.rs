use cherris_core::{generate_lookup_tables, perft, Position};
use criterion::{criterion_group, criterion_main, Criterion};
use std::{str::FromStr, time::Duration};

pub fn perft_benchmark(c: &mut Criterion) {
    generate_lookup_tables();

    let mut position =
        Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    let mut group = c.benchmark_group("perft5");

    group.measurement_time(Duration::from_secs(10));
    group.bench_function("perft 5", |b| b.iter(|| perft(5, &mut position)));
}

criterion_group!(benches, perft_benchmark);
criterion_main!(benches);
