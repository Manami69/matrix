use crate::classes::{matrix::Matrix, complex::Complexf32};
use num::Zero;

pub fn ex13() {
    let u = Matrix::from([
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
        ]);
        println!("{}", u.rank());
        // 3
        let u = Matrix::from([
        [ 1., 2., 0., 0.],
        [ 2., 4., 0., 0.],
        [-1., 2., 1., 1.],
        ]);
        println!("{}", u.rank());
        // 2
        let u = Matrix::from([
        [ 8., 5., -2.],
        [ 4., 7., 20.],
        [ 7., 6., 1.],
        [21., 18., 7.],
        ]);
        println!("{}", u.rank());
        // 3
        // example from https://www.youtube.com/watch?v=2ogdwpHD3V8
        let u = Matrix::from([
        [ 1., 1., 2., 4., 2.],
        [ 2., 1., 3., 5., 4.],
        [ 1., 1., 2., 4., 2.],
        [ 0., 1., 1., 3., 0.],
        ]);
        println!("{}", u.rank());
        // 2
        let u = Matrix::from([
            [Complexf32::new(3., 1.), Complexf32::new(4., 2.), Complexf32::zero()],
            [Complexf32::new(5., 4.), Complexf32::new(2., 0.), Complexf32::new(5., 0.)],
            [Complexf32::zero(), Complexf32::new(4.,1.), Complexf32::zero()]
        ]);
        println!("{}", u.rank());
        // 3 tested on https://matrix.reshish.com/rank.php
}