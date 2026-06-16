use std::arch::x86_64::*;

pub fn snrm2_naive(x: &[f32]) -> f32 {
    let mut result = 0.0f32;
    for i in 0..x.len() {
        result += x[i] * x[i];
    }

    result.sqrt()
}

pub fn snrm_iter(x: &[f32]) -> f32 {
    x.iter().map(|x| x * x).sum::<f32>().sqrt()
}

pub fn snrm(x: &[f32]) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            return unsafe { snrm2_avx2(x) };
        }
    }
    snrm_iter(x)
}

#[target_feature(enable = "avx2,fma")]
unsafe fn snrm2_avx2(x: &[f32]) -> f32 {
    let mut acc = _mm256_setzero_ps();
    let n = x.len();
    let simd_end = (n / 8) * 8;
    unsafe {
        for i in (0..simd_end).step_by(8) {
            let vx = _mm256_loadu_ps(x.as_ptr().add(i));
            acc = _mm256_fmadd_ps(vx, vx, acc);
        }
    }
    let high = _mm256_extractf128_ps(acc, 1);
    let low = _mm256_extractf128_ps(acc, 0);
    let sum128 = _mm_add_ps(high, low);
    let sum1 = _mm_hadd_ps(sum128, sum128);
    let sum2 = _mm_hadd_ps(sum1, sum1);
    let mut result = _mm_cvtss_f32(sum2);

    for i in simd_end..n {
        result += x[i] * x[i];
    }

    result.sqrt()
}
