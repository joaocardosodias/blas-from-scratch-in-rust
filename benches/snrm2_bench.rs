use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use blas_from_scratch_in_rust::{snrm2_naive, snrm_iter, snrm};

fn bench_nrm2(c: &mut Criterion) {
    let n = 1024usize;
    let x: Vec<f32> = (0..n).map(|i| i as f32).collect();

    let mut group = c.benchmark_group("nrm2_f32_1024");
    group.throughput(Throughput::Elements(n as u64));

    group.bench_function("naive", |bencher| {
        bencher.iter(|| snrm2_naive(black_box(&x)))
    });

    group.bench_function("iter", |bencher| {
        bencher.iter(|| snrm_iter(black_box(&x)))
    });

    group.bench_function("avx2", |bencher| {
        bencher.iter(|| snrm(black_box(&x)))
    });

    group.finish();
}

criterion_group!(benches, bench_nrm2);
criterion_main!(benches);
