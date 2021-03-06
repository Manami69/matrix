use crate::classes::{complex::Complexf32, vector::Vector};

use super::cosine::angle_cos;

pub fn ex05() {
    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    println!("{:.1}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    println!("{:.1}", angle_cos(&u, &v));
    // 0.0
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([1., -1.]);
    println!("{:.1}", angle_cos(&u, &v));
    // -1.0
    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    println!("{:.1}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));
    // 0.974631846
    let u = Vector::from([Complexf32::new(1., -1.), Complexf32::new(2., 2.)]);
    let v = Vector::from([Complexf32::new(3., -4.), Complexf32::new(5., 2.)]);
    println!("{}", angle_cos(&u, &v));
    // Cosine Similarity between A and B:(0.21516574145596756+0.3012320380383546j)
}
