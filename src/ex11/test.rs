use crate::classes::{complex::Complexf32, matrix::Matrix};

pub fn ex11() {
    let u = Matrix::from([[1., -1.], [-1., 1.]]);
    println!("{}", u.determinant());
    // 0.0
    let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    println!("{}", u.determinant());
    // 8.0
    let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    println!("{}", u.determinant());
    // -174.0
    let u = Matrix::from([
        [8., 5., -2., 4.],
        [4., 2.5, 20., 4.],
        [8., 5., 1., 4.],
        [28., -4., 17., 1.],
    ]);
    println!("{}", u.determinant());
    // 1032
    let u = Matrix::from([
        [Complexf32::new(2., 1.), Complexf32::new(3., 2.)],
        [Complexf32::new(4., -1.), Complexf32::new(2., 1.)],
    ]);
    println!("{}", u.determinant());

    //  -11-i https://matrix.reshish.com/determinant.php
}
