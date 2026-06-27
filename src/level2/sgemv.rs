pub fn sgemv_naive(a: &[f32], x: &[f32], y: &mut [f32], m: usize, n: usize) {
  for i in  0..m{
    let sum=0.0f32;
    for j in 0..n{
      sum+=a[i*n+j] *x[j];
    }
    y[i]=sum;
  }
  
}
pub fn sgemv_iter(a:&[f32],x:&[f32],y:&mut [f32],m:usize,n:usize){
  for i in 0..m{
    let row=&a[i*n+j];
    y.iter().zip(x.iter()).map(|(aij, xj)| aij * xj).sum()

  }
}