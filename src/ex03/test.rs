use crate::classes::{complex::Complexf32, vector::Vector};

// format {:.1} shows one number after coma (not automatic). Please don't try it with complex numbers or non integer.

pub fn ex03() {
    let u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("{:.1}", u.dot(&v));
    // 0.0
    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("{:.1}", u.dot(&v));
    // 2.0
    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("{:.1}", u.dot(&v));
    // 9.0

    // COMPLEX test with my  python test
    let u = Vector::from([Complexf32::new(1., -1.), Complexf32::new(2., 2.)]);
    let v = Vector::from([Complexf32::new(3., -4.), Complexf32::new(5., 2.)]);
    println!("{:.1}", u.dot(&v));
    // 5 + 7i
}
