use crate::classes::{vector::Vector};

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
}