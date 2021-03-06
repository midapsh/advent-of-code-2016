use advent_of_code_2016::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("2-1", |b| b.iter(p2_1));
    c.bench_function("2-2", |b| b.iter(p2_2));
    c.bench_function("3-1", |b| b.iter(p3_1));
    c.bench_function("3-2", |b| b.iter(p3_2));
    c.bench_function("4-1", |b| b.iter(p4_1));
    c.bench_function("4-2", |b| b.iter(p4_2));
    {
        let mut slow = c.benchmark_group("slow");
        slow.sample_size(10);
        slow.bench_function("5-1", |b| b.iter(p5_1));
        slow.bench_function("5-2", |b| b.iter(p5_2));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
