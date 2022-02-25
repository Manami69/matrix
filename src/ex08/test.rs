use crate::classes::{complex::Complexf32, matrix::Matrix};

pub fn ex08() {
    let u = Matrix::from([[1., 0.], [0., 1.]]);
    println!("{}", u.trace());
    // 2.0
    let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    println!("{}", u.trace());
    // 9.0
    let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    println!("{}", u.trace());
    // -21.0
    let u = Matrix::from([
        [Complexf32::new(2., 1.), Complexf32::new(3., 2.)],
        [Complexf32::new(4., -1.), Complexf32::new(2., 1.)],
    ]);
    println!("{}", u.trace());
    // = 2 + 1i + 2 + 1i = 4 + 2i
}
