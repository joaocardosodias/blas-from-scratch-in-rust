use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use blas_from_scratch_in_rust::{stdot,stdot_iter,stdot_naive};
fn bench_dot(c: &mut Criterion) {
    let n = 1024usize;
    let a: Vec<f32> = (0..n).map(|x| x as f32).collect();
    let b: Vec<f32> = (0..n).map(|x| x as f32).collect();

    let mut group = c.benchmark_group("dot_f32_1024");
    group.throughput(Throughput::Elements(n as u64));

    group.bench_function("naive", |bencher| {
        bencher.iter(|| stdot_naive(black_box(&a), black_box(&b)))
    });

    group.bench_function("iter", |bencher| {
        bencher.iter(|| stdot_iter(black_box(&a), black_box(&b)))
    });

    group.bench_function("avx2", |bencher| {
        bencher.iter(||stdot(black_box(&a), black_box(&b)))
    });

    group.finish();
}

criterion_group!(benches, bench_dot);
criterion_main!(benches);
