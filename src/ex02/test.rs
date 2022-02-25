use super::linearinterpolation::lerp;
use crate::classes::{vector::Vector, matrix::Matrix, complex::Complexf32};


pub fn ex02() {
    println!("{}", lerp(0., 1., 0.));
    // 0.0
    println!("{}", lerp(0., 1., 1.));
    // 1.0
    println!("{}", lerp(0., 1., 0.5));
    // 0.5
    println!("{}", lerp(21., 42., 0.3));
    // 27.3
    println!("{}", lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3));
    // [2.6]
    // [1.3]
    println!("{}", lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20.,
    10.], [30., 40.]]), 0.5));
    // [[11., 5.5]
    // [16.5, 22.]]

    // Complex :
    // havenn't found exemple but this forum confirm that it's the same than for the real numbers https://www.eevblog.com/forum/projects/interpolating-complex-numbers/msg589737/#msg589737
    println!("{}", lerp(Complexf32::new(1., 1.), Complexf32::new(2., 2.), 0.5));
    println!("{}", lerp(Vector::from([Complexf32::new(1., 1.), Complexf32::new(2., 2.)]), Vector::from([Complexf32::new(3., 3.), Complexf32::new(4., 4.)]), 0.3));
    println!("{}", lerp(Matrix::from([[Complexf32::new(1., 1.), Complexf32::new(2., 2.)],
                                      [Complexf32::new(1., 1.), Complexf32::new(2., 2.)]]),
                        Matrix::from([[Complexf32::new(3., 3.), Complexf32::new(4., 4.)],
                                      [Complexf32::new(3., 3.), Complexf32::new(4., 4.)]]), 0.3));

}