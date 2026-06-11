use blas_from_scratch_in_rust::stdot;

fn main() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![4.0, 5.0, 6.0];
    println!("dot([1,2,3], [4,5,6]) = {}", stdot(&a, &b)); // 32

    let a: Vec<f32> = (0..16).map(|x| x as f32).collect();
    let b: Vec<f32> = vec![1.0; 16];
    println!("dot(0..16, ones) = {}", stdot(&a, &b)); // 120

    let a: Vec<f32> = (0..20).map(|x| x as f32).collect();
    let b: Vec<f32> = vec![1.0; 20];
    println!("dot(0..20, ones) = {}", stdot(&a, &b)); // 190
}
