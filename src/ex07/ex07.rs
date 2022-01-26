use crate::classes::{matrix::Matrix, vector::Vector};

pub fn ex07() {
    let u = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
        let v = Vector::from([4., 2.]);
        println!("{}\n", u.mul_vec(&v));
        // [4.]
        // [2.]
        let u = Matrix::from([
        [2., 0.],
        [0., 2.],
        ]);
        let v = Vector::from([4., 2.]);
        println!("{}\n", u.mul_vec(&v));
        // [8.]
        // [4.]
        let u = Matrix::from([
        [2., -2.],
        [-2., 2.],
        ]);
        let v = Vector::from([4., 2.]);
        println!("{}\n", u *v);
        // [4.]
        // [-4.]
        let u = Matrix::from([[1., -1., 2.],[0., -3., 1.]]);
        let v = Vector::from([2., 1., 0.]);
        println!("{}\n", u.mul_vec(&v));
        // [1.]
        // [-3.]
        // example from https://mathinsight.org/matrix_vector_multiplication
        let u = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
        let v = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
        println!("{}\n", &u *&v);
        // [1., 0.]
        // [0., 1.]
        let u = Matrix::from([
        [1., 0.],
        [0., 1.],
        ]);
        let v = Matrix::from([
        [2., 1.],
        [4., 2.],
        ]);
        println!("{}\n", u.mul_mat(&v));
        // [2., 1.]
        // [4., 2.]
        let u = Matrix::from([
        [3., -5.],
        [6., 8.],
        ]);
        let v = Matrix::from([
        [2., 1.],
        [4., 2.],
        ]);
        println!("{}", u.mul_mat(&v));
        // [-14., -7.]
        // [44., 22.]
}