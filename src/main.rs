mod vector;
mod matrix;
mod complex;
use vector::Vector;
use complex::Complexf64;
use matrix::Matrix;

fn main() {
    let complex_float = [Complexf64::new(10.1, 20.1), Complexf64::new(10.1, 21.1)];
    let vec = Vector::from(complex_float);
    let vec2 = Vector::from(complex_float);
    let matrix = Matrix::from([[1, 2, 3], [4, 5, 6]]);
    println!("{}",vec + vec2); 
    println!("{}", Complexf64::new(02., 02.));
    println!("{}", matrix);
}
