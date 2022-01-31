use crate::classes::{matrix::Matrix, complex::Complexf32};
use num::{ Zero};


pub fn ex10() {
    let u = Matrix::from([
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
        ]);
        println!("{}", u.row_echelon());
        // [1.0, 0.0, 0.0]
        // [0.0, 1.0, 0.0]
        // [0.0, 0.0, 1.0]
        let u = Matrix::from([
        [1., 2.],
        [3., 4.],
        ]);
        println!("{}", u.row_echelon());
        // [1.0, 0.0]
        // [0.0, 1.0]
        let u = Matrix::from([
        [1., 2.],
        [2., 4.],
        ]);
        println!("{}", u.row_echelon());
        // [1.0, 2.0]
        // [0.0, 0.0]
        let u = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
        ]);
        println!("{}", u.row_echelon());
        // [1.0, 0.625, 0.0, 0.0, -12.1666667]
        // [0.0, 0.0, 1.0, 0.0, -3.6666667]
        // [0.0, 0.0, 0.0, 1.0, 29.5 ]

        // exemple from https://adrianstoll.com/linear-algebra/row-reduction.html petard de ouf
        let u = Matrix::from([
            [Complexf32::new(3., 1.), Complexf32::new(4., 2.), Complexf32::zero()],
            [Complexf32::new(5., 4.), Complexf32::new(2., 0.), Complexf32::new(5., 0.)],
            [Complexf32::zero(), Complexf32::new(4.,1.), Complexf32::zero()]
        ]);
        println!("{}", u.row_echelon());
        // 1 0 0
        // 0 1 0
        // 0 0 1


}