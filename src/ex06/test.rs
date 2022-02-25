use crate::classes::{vector::Vector, complex::Complexf32};
use super::crossproduct::cross_product;

pub fn ex06() {
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
    // Complex https://wims.univ-cotedazur.fr/wims/en_tool~linear~vector.en.html
    let u = Vector::from([Complexf32::new(1., -6.), Complexf32::new(-2.,2.), Complexf32::new(-7.,5.)]);
    let v = Vector::from([Complexf32::new(3., -4.), Complexf32::new(5., 2.), Complexf32::new(8.,-8.)]);
    println!("{}", cross_product(&u, &v));
    // Vector product: w=v1∧v2= (45+21i,39+99i,15−42i).
}