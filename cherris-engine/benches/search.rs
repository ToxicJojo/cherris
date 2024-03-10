use cherris_core::{generate_lookup_tables, Position};
use cherris_engine::{
    iterative_deepening::iterative_deepening, time_managment::TimeManagment,
    transposition_table::TranspositionTable,
};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};

pub fn search_benchmark(c: &mut Criterion) {
    generate_lookup_tables();

    let position = Position::new();

    let mut group = c.benchmark_group("search");
    let max_depth = 7;
    let max_nodes = u64::MAX;
    let time_managment = TimeManagment::new(u128::MAX, 0, Some(40));

    group.measurement_time(Duration::from_secs(10));
    group.bench_function("iterative deepening 7", |b| {
        b.iter_batched(
            || Arc::new(Mutex::new(TranspositionTable::new(2_u64.pow(24)))),
            |tt| iterative_deepening(position, max_depth, max_nodes, time_managment, tt),
            BatchSize::SmallInput,
        );
    });

    let kiwi_pete =
        Position::from_str("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
            .unwrap();
    let max_depth = 5;
    group.bench_function("iterative deepening kiwipete 5", |b| {
        b.iter_batched(
            || Arc::new(Mutex::new(TranspositionTable::new(2_u64.pow(24)))),
            |tt| iterative_deepening(kiwi_pete, max_depth, max_nodes, time_managment, tt),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, search_benchmark);
criterion_main!(benches);
