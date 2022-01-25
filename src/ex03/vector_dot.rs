use crate::classes::vector::Vector;
use crate::types::number_type::Number;
use num::{Zero};

/// DOT PRODUCT
/// fr - produit scalaire 
/// 
///  Let u, v ∈ V , where V is a vector space of finite dimension over the real numbers R
/// (represented as the type f32).
/// The function must compute and return the scalar 〈u|v〉 = u·v, called the dot product,
/// or inner product, of u and v.
/// If both vectors have different dimensions, the behaviour is undefined.

impl<K> Vector<K> where K : Number + Zero + Copy , {
    pub fn dot(&self, v: Vector::<K>) -> K {
        if self.size() != v.size() { panic!("cannot compute 2 Vectors of different dimension.")}
        self.data.iter().zip(v.data.iter()).fold(K::zero(), |mut acc, num| {acc = acc + *num.0 * *num.1; acc})
    }
}