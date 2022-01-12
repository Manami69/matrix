mod vector;
use vector::Vector;
use num_complex::Complex;
fn main() {
    let complex_float = [Complex::new(10.1, 20.1), Complex::new(10.1, 21.1)];
    let vec = Vector::from(complex_float);
    let vec2 = Vector::from([1, 2,3]);
    println!("{}",vec); 

}
