use crate::classes::{matrix::Matrix, complex::Complexf32};
use num::Zero;


pub fn ex12() {

    let u = Matrix::from([
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
        ]);
        println!("{}", u.inverse().unwrap());
        // [1.0, 0.0, 0.0]
        // [0.0, 1.0, 0.0]
        // [0.0, 0.0, 1.0]
        let u = Matrix::from([
        [2., 0., 0.],
        [0., 2., 0.],
        [0., 0., 2.],
        ]);
        println!("{}", u.inverse().unwrap());
        // [0.5, 0.0, 0.0]
        // [0.0, 0.5, 0.0]
        // [0.0, 0.0, 0.5]
        let u = Matrix::from([
        [8., 5., -2.],
        [4., 7., 20.],
        [7., 6., 1.],
        ]);
        println!("{}", u.inverse().unwrap());
        // [0.649425287, 0.097701149, -0.655172414]
        // [-0.781609195, -0.126436782, 0.965517241]
        // [0.143678161, 0.074712644, -0.206896552]

        // find exemple in https://matrix.reshish.com/inverCalculation.php
        let u = Matrix::from([
            [Complexf32::new(3., 1.), Complexf32::new(4., 2.), Complexf32::zero()],
            [Complexf32::new(5., 4.), Complexf32::new(2., 0.), Complexf32::new(5., 0.)],
            [Complexf32::zero(), Complexf32::new(4.,1.), Complexf32::zero()]
        ]);
        println!("{}", u.inverse().unwrap());

}