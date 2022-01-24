use crate::classes::vector::Vector;
// use crate::ex00::vector;
use crate::types::number_type::Number;
use num::{Zero};

/// LINEAR COMBINATION
/// fr => combinaison linéaire
/// 
/// ** Wikipedia recap
///
/// a linear combination is an expression constructed from a set of terms 
/// by multiplying each term by a constant and adding the results 
/// (e.g. a linear combination of x and y would be any expression 
/// of the form ax + by, where a and b are constants).
/// 
/// Take an arbitrary field K, 
/// an arbitrary vector space V, 
/// and let v1,...,vn be vectors (in V). 
/// It’s interesting to consider the set of all linear combinations of these vectors.
/// This set is called the linear span of the vectors
/// fr for span => Sous-espace vectoriel engendré
/// 
/// ** 3Blue1Brown course on linear combination : 
/// https://www.youtube.com/watch?v=k7RM-ot2NWY 

pub fn linear_combination<K, const N: usize>(u: &[Vector<K>;N], coefs: &[K; N]) -> Vector<K> where K: Number + std::fmt::Display + Zero + Copy,  {
    u.iter().zip(coefs.iter()).fold(Vector::<K>::from(u[0].size()), |mut sum, plus| {sum = sum + plus.0 * plus.1; sum})
}