use blas_from_scratch_in_rust::{stdot,saxpy};

fn main() {
  let mut y = vec![10.0, 20.0, 30.0];
  let x = vec![1.0, 2.0, 3.0];
  saxpy(2.0, &x, &mut y);
  println!("{:?}", y); // [12, 24, 36]
}
