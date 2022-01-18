mod vector;
mod matrix;
mod complex;
use vector::Vector;
use complex::Complexf64;
use matrix::Matrix;

fn main() {
    // let complex_float = [Complexf64::new(10.1, 20.1), Complexf64::new(10.1, 21.1)];
    // let vec = Vector::from(complex_float);
    // let vec2 = Vector::from(complex_float);
    let matrix = Matrix::from([[1.,2., 3.], [4., 5., 6.]]);
    println!("{}", matrix);
    let v = Vector::from(Matrix::from([[1.,2., 3.], [4., 5., 6.]]));
    let v1 = Vector::from([6.,5.,4.,3.,2.,1.]);
    println!("re {}\n v {}", matrix, v);
    println!{"++ {}\n", v + &v1}

}
