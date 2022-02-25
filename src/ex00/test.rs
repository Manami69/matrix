use crate::classes::{complex::Complexf32, matrix::Matrix, vector::Vector};

// /// find functions in the vector and Matrix classes

pub fn ex00() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
    println!("VECTORS\nADD\n{}", u);
    // [7.0]
    // [10.0]
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.sub(&v);
    println!("SUB\n{}", u);
    // [-3.0]
    // [-4.0]
    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    println!("SCL\n{}", u);
    // [4.0]
    // [6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.add(&v);
    println!("MATRIX\nADD\n{}", u);
    // [8.0, 6.0]
    // [1.0, 6.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let v = Matrix::from([[7., 4.], [-2., 2.]]);
    u.sub(&v);
    println!("SUB\n{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    println!("SCL\n{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]
    // COMPLEX NUMBERS
    // https://wims.univ-cotedazur.fr/wims/en_tool~linear~vector.en.html test with
    let mut u = Vector::from([Complexf32::new(2., 1.), Complexf32::new(4., 3.)]);
    let v = Vector::from([Complexf32::new(5., 1.), Complexf32::new(2., 2.)]);
    u.add(&v);
    println!("COMPLEX VECTORS\nADD\n{}", u);
    // [7+2i]
    // [6+5i]
    let mut u = Vector::from([Complexf32::new(2., 1.), Complexf32::new(4., 3.)]);
    let v = Vector::from([Complexf32::new(5., 1.), Complexf32::new(2., 2.)]);
    u.sub(&v);
    println!("SUB\n{}", u);
    // [-3]
    // [2 + i]
    let mut u = Vector::from([Complexf32::new(2., 1.), Complexf32::new(4., 3.)]);
    u.scl(Complexf32::new(3., 4.));
    println!("SCL\n{}", u);
    // [2 + 11i]
    // [25i]
    let mut u = Matrix::from([
        [Complexf32::new(1., 2.), Complexf32::new(3., 4.)],
        [Complexf32::new(5., 6.), Complexf32::new(7., 8.)],
    ]);
    let v = Matrix::from([
        [Complexf32::new(1., 2.), Complexf32::new(3., 4.)],
        [Complexf32::new(5., 6.), Complexf32::new(7., 8.)],
    ]);
    u.add(&v);
    println!("COMPLEX MATRIX\nADD\n{}", u);
    // [2 + 4i, 6 + 8i]
    // [10 + 12 i, 14 + 16i]
    let mut u = Matrix::from([
        [Complexf32::new(1., 2.), Complexf32::new(3., 4.)],
        [Complexf32::new(5., 6.), Complexf32::new(7., 8.)],
    ]);
    let v = Matrix::from([
        [Complexf32::new(11., 12.), Complexf32::new(13., 14.)],
        [Complexf32::new(15., 16.), Complexf32::new(17., 18.)],
    ]);
    u.sub(&v);
    println!("SUB\n{}", u);
    // -10 -10 -10 -10 -10 -10 -10 -10 -10 -10 LOL LA FLEMME
    let mut u = Matrix::from([
        [Complexf32::new(1., 2.), Complexf32::new(3., 4.)],
        [Complexf32::new(5., 6.), Complexf32::new(7., 8.)],
    ]);
    u.scl(Complexf32::new(10., 100.));
    // Multiplication https://ncalculators.com/algebra/complex-numbers-multiplication-calculator.htm
    println!("SCL\n{}", u);
    // [-190 + 120i , -370 + 340i]
    // [-550 + 560i , -730 + 780i]
}
