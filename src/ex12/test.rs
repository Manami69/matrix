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
        // 0.3-0.1i	        0	    -0.34117647058823529412+0.03529411764705882354i
        // 0	            0	    0.23529411764705882353-0.058823529411764705882i
        // -0.38-0.14i	    0.2	    0.27529411764705882355+0.26117647058823529413i

}