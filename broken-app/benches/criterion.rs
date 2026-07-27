use broken_app::{algo, sum_even};
use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn bench_sum_even(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_even_broken");

    for size in [50_000, 1_000_000] {
        let data: Vec<i64> = (0..size).collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| sum_even(black_box(data)));
        });
    }

    group.finish();
}

fn bench_fib(c: &mut Criterion) {
    let mut group = c.benchmark_group("slow_fib_broken");

    for n in [20_u64, 28, 32] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| algo::slow_fib(black_box(n)));
        });
    }

    group.finish();
}

fn bench_dedup(c: &mut Criterion) {
    let mut group = c.benchmark_group("slow_dedup_broken");

    for size in [5_000_u64, 50_000] {
        let data: Vec<u64> = (0..size).flat_map(|n| [n, n]).collect();

        group.bench_with_input(BenchmarkId::from_parameter(data.len()), &data, |b, data| {
            b.iter_batched(
                || data.clone(),
                |v| {
                    black_box(algo::slow_dedup(black_box(&v)));
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_sum_even, bench_fib, bench_dedup);
criterion_main!(benches);
