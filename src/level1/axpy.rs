use std::arch::x86_64::{
    _mm256_fmadd_ps, _mm256_loadu_ps, _mm256_set1_ps, _mm256_storeu_ps,
};

pub fn saxpy_iter(alpha: f32, x: &[f32], y: &mut [f32]) {
    assert_eq!(x.len(), y.len(), "The vectors must be the same size.");
    y.iter_mut()
        .zip(x.iter())
        .for_each(|(yi, xi)| *yi += *xi * alpha);
}

pub fn saxpy(alpha: f32, x: &[f32], y: &mut [f32]) {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            return unsafe {
                saxpy_avx2(alpha, x, y);
            };
        }
        
    }
    saxpy_iter(alpha, x, y);
}

#[target_feature(enable = "avx2,fma")]
unsafe fn saxpy_avx2(alpha: f32, x: &[f32], y: &mut [f32]) {
    let n = x.len();
    let valpha = _mm256_set1_ps(alpha);
    let simd_end = (n / 8) * 8;
    
    unsafe {
        for i in (0..simd_end).step_by(8) {
            let vx = _mm256_loadu_ps(x.as_ptr().add(i));
            let vy = _mm256_loadu_ps(y.as_ptr().add(i));
            let vy_new = _mm256_fmadd_ps(valpha, vx, vy);
            _mm256_storeu_ps(y.as_mut_ptr().add(i), vy_new);
        }
    }
    
    for i in simd_end..n {
        y[i] += alpha * x[i];
    }
}
