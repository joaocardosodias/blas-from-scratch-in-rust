use std::arch::x86_64::*;

pub fn stdot_naive(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let mut acc = 0.0f32;
    for i in 0..a.len() {
        acc += a[i] * b[i];
    }
    acc
}

pub fn stdot_iter(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(ai, bi)| ai * bi).sum()
}

pub fn stdot(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "The vectors must be the same size.");
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            return unsafe { stdot_avx2(a, b) };
        }
    }
    stdot_iter(a, b)
}

#[target_feature(enable = "avx2,fma")]
unsafe fn stdot_avx2(a: &[f32], b: &[f32]) -> f32 {
    let n = a.len();
    let mut acc = _mm256_setzero_ps();
    let simd_end = (n / 8) * 8;

    unsafe {
        for i in (0..simd_end).step_by(8) {
            let va = _mm256_loadu_ps(a.as_ptr().add(i));
            let vb = _mm256_loadu_ps(b.as_ptr().add(i));
            acc = _mm256_fmadd_ps(va, vb, acc);
        }
    }

    let hi = _mm256_extractf128_ps(acc, 1);
    let lo = _mm256_extractf128_ps(acc, 0);
    let sum128 = _mm_add_ps(lo, hi);
    let sum1 = _mm_hadd_ps(sum128, sum128);
    let sum2 = _mm_hadd_ps(sum1, sum1);
    let mut result = _mm_cvtss_f32(sum2);

    for i in simd_end..n {
        result += a[i] * b[i];
    }

    result
}
