use crate::classes::{matrix::Matrix, vector::Vector, complex::Complexf32};

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
        println!("{}\n", u.mul_mat(&v));
        // [-14., -7.]
        // [44., 22.]
        let u = Matrix::from([
            [0., 4., -2.],
            [-4., -3., 0.],
            ]);
        let v = Matrix::from([
            [0., 1.],
            [1., -1.],
            [2., 3.]
            ]);
        println!("{}", u.mul_mat(&v));
            // [0., -10.]
            // [-3., -1.]

        // Complex https://matrix.reshish.com/multiplication.php
        let u = Matrix::from([
            [Complexf32::new(2.,1.), Complexf32::new(3., 2.)],
            [Complexf32::new(4., -1.), Complexf32::new(2., 1.)],
            [Complexf32::new(6., -8.), Complexf32::new(5., 7.)]
            ]);
        let v = Matrix::from([
            [Complexf32::new(-2., 4.), Complexf32::new(-4., -4.), Complexf32::new(3., 0.), Complexf32::new(3., 4.), Complexf32::new(1., -8.)],
            [Complexf32::new(7., 8.), Complexf32::new(2., -3.), Complexf32::new(6., 9.), Complexf32::new(-8., 0.), Complexf32::new(12., -3.)],
            ]);
            println!("{}", u.mul_mat(&v));
            // 
            // -3+44i	8-17i	 6+42i	 -22-5i	52
            // 2+41i	-13-16i	 15+21i	 5i	    23-27i
            // -1+129i	-25+7i	 -15+63i 10-56i	23+13i
}