use blas_from_scratch_in_rust::{saxpy, saxpy_iter};
use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};

fn bench_saxpy(c: &mut Criterion) {
    let n = 1024usize;
    let alpha = 2.0f32;
    let x: Vec<f32> = (0..n).map(|x| x as f32).collect();
    let y_initial: Vec<f32> = (0..n).map(|x| x as f32 * 2.0).collect();

    let mut group = c.benchmark_group("saxpy_f32_1024");
    group.throughput(Throughput::Elements(n as u64));

    group.bench_function("iter", |bencher| {
        let mut y = y_initial.clone();
        bencher.iter(|| {
            y.copy_from_slice(&y_initial);
            saxpy_iter(black_box(alpha), black_box(&x), black_box(&mut y))
        })
    });

    group.bench_function("avx2", |bencher| {
        let mut y = y_initial.clone();
        bencher.iter(|| {
            y.copy_from_slice(&y_initial);
            saxpy(black_box(alpha), black_box(&x), black_box(&mut y))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_saxpy);
criterion_main!(benches);
