mod vector;
mod complex;
use vector::Vector;
use complex::Complexf64;
fn main() {
    let complex_float = [Complexf64::new(10.1, 20.1), Complexf64::new(10.1, 21.1)];
    let vec = Vector::from(complex_float);
    let vec2 = Vector::from(complex_float);
    println!("{}",vec + vec2); 

}
