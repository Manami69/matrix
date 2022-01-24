use crate::classes::{vector::Vector, matrix::Matrix};

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
    let mut u = Matrix::from([
    [1., 2.],
    [3., 4.]
    ]);
    let v = Matrix::from([
    [7., 4.],
    [-2., 2.]
    ]);
    u.add(&v);
    println!("MATRIX\nADD\n{}", u);
    // [8.0, 6.0]
    // [1.0, 6.0]
    let mut u = Matrix::from([
    [1., 2.],
    [3., 4.]
    ]);
    let v = Matrix::from([
    [7., 4.],
    [-2., 2.]
    ]);
    u.sub(&v);
    println!("SUB\n{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]
    let mut u = Matrix::from([
    [1., 2.],
    [3., 4.]
    ]);
    u.scl(2.);
    println!("SCL\n{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]
}
