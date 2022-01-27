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
pub mod ex08;
pub mod ex09;




fn main() {
    // ex00::ex00::ex00();
    // ex01::ex01::ex01();
    // ex02::ex02::ex02();
    // ex03::ex03::ex03();
    // ex04::ex04::ex04();
    // ex05::ex05::ex05();
    // ex06::ex06::ex06();
    // ex07::ex07::ex07();
    ex08::ex08::ex08();
}
