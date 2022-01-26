// pub mod classes;
// pub mod matrix;
// pub mod complex;
// pub mod number_type;
// use vector::Vector;
// use complex::Complexf32;
// use matrix::Matrix;
pub mod types;
pub mod classes;
use crate::classes::vector::Vector;
use crate::types::trait_absolute::Absolute;
use classes::complex::Complexf32;
use num::complex::Complex;
/// Exercices Includes
pub mod ex00;
pub mod ex01;
pub mod ex02;
pub mod ex03;
pub mod ex04;
pub mod ex05;
pub mod ex06;
pub mod ex07;

fn main() {
    // ex00::ex00::ex00();
    // ex01::ex01::ex01();
    // ex02::ex02::ex02();
    // ex03::ex03::ex03();
    // ex04::ex04::ex04();
    // let a = Complexf32::new(1., 7.);
    // let b = Complexf32::new(2., -6.);
    // let v = Vector::from([a, b]);
    // println!("{} et {}", v.norm(), (90.0 as f32).sqrt());
    // let a1 = Complexf32::new(1., 0.);
    // let b2 = Complexf32::new(1., 0.);
    // let c = Complex::new(1., 1.);
    // let d = Complex::new(0., 0.);
    // let e = Complex::new(1., 0.);
    // let f = Complex::new(1., 0.);
    // println!("{} ET {}", (a * a1 + b * b2),( c * e + d * f));
    // println!("{}", a.abs());
    // ex05::ex05::ex05();
    // ex06::ex06::ex06();
    ex07::ex07::ex07();
}
