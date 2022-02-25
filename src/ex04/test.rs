use crate::classes::{complex::Complexf32, vector::Vector};

pub fn ex04() {
    let u = Vector::from([0., 0., 0.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    let u = Vector::from([1., 2., 3.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    let u = Vector::from([-1., -2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
    // COMPLEX test with my  python test
    let u = Vector::from([Complexf32::new(1., -1.), Complexf32::new(2., 2.)]);
    let v = Vector::from([Complexf32::new(3., -4.), Complexf32::new(5., 2.)]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    println!("{}, {}, {}", v.norm_1(), v.norm(), v.norm_inf());
}
