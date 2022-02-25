use crate::classes::vector::Vector;
use crate::types::number_type::Number;
use crate::types::trait_absolute::Absolute;
use num::{Zero};
use std::cmp::Ordering;

/// VECTOR NORM
/// 
///  Let v be a vector of a vector space V .
/// You must implement the following norms:
/// - 1-norm: ‖v‖1 (also called the Taxicab norm or Manhattan norm)
/// The sum of the absolute values of the components of a vector.
/// The name derives from the distance a taxi has to drive in a rectangular street grid 
/// to get from the origin to a particular point.
/// It is also known as the Manhattan norm because Manhattan has perhaps the most famous rectangular street grid.
/// 
/// - 2-norm: ‖v‖ or ‖v‖2 (also called the Euclidean norm)
/// - ∞-norm: ‖v‖∞ (also called the supremum norm)
/// 
/// Norms always return real numbers, even for complex-valued vectors.
/// 
/// https://en.wikipedia.org/wiki/Norm_(mathematics) 

impl<V> Vector<V> where V : Number + Absolute,  {
    /// Taxicab Norm
    pub fn norm_1(&self) -> f32 {
        self.data.iter().fold(f32::zero(), |mut acc, num| {acc += num.abs(); acc})
    }
    /// Euclidian Norm
    /// 
    /// I used absolute value of each coordinate in order to be able to use complex numbers (Cf : https://mathcs.clarku.edu/~ma130/inner2.pdf)
    pub fn norm(&self) -> f32 {
        self.data.iter().fold(f32::zero(), |mut acc, num| {acc += num.abs() * num.abs(); acc}).sqrt()
    }
    /// Supremum Norm
    pub fn norm_inf(&self) -> f32 {
        self.data.iter().max_by(| last, next | {
            if last.abs() >= next.abs() { Ordering::Greater }
            else { Ordering::Less }
        }).unwrap().abs()
    }
}