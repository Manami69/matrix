use crate::classes::{vector::Vector, complex::Complexf32};
use num::Zero;
use super::linearcombination::linear_combination;
pub fn ex01() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("{}", linear_combination(&[e1, e2, e3], &[10., -2.,
    0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}", linear_combination(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]
    let e1 = Vector::from([Complexf32::new(1., 2.), Complexf32::zero(), Complexf32::zero()]);
    let e2 = Vector::from([Complexf32::zero(), Complexf32::new(1., 2.), Complexf32::zero()]);
    let e3 = Vector::from([Complexf32::zero(), Complexf32::zero(), Complexf32::new(1., 2.)]);
    println!("{}", linear_combination(&[e1, e2, e3], &[Complexf32::new(1., 2.), Complexf32::new(3., 4.), Complexf32::new(5., 6.)]));
    // -3 + 4i     + 0          + 0         =  -3 + 4i
    // 0           + -5 + 10i   + 0         =  -5 + 10i
    // 0           + 0          + -7 + 16i  =  -7 + 16i
}