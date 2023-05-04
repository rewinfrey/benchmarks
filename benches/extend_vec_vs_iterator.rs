use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn extend_with_iterator(mut base: Vec<u64>, i: impl Iterator<Item = u64>) {
    base.extend(i);
}

fn extend_with_vec(mut base: Vec<u64>, vec: Vec<u64>) {
    base.extend(vec);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("extend_vec_vs_iterator");
    for limit in [100, 1000, 10000, 100000, 1000000].iter() {
        group.bench_with_input(BenchmarkId::new("iterator", limit), limit, |b, &limit| {
            b.iter(|| {
                extend_with_iterator(
                    black_box(vec![1, 2, 3]),
                    black_box((4..=limit).collect::<Vec<u64>>().into_iter()),
                )
            })
        });
        group.bench_with_input(BenchmarkId::new("vec", limit), limit, |b, &limit| {
            b.iter(|| {
                extend_with_vec(
                    black_box(vec![1, 2, 3]),
                    black_box((4..=limit).collect::<Vec<u64>>()),
                )
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
