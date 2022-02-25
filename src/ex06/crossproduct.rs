use crate::classes::vector::Vector;
use crate::types::number_type::Number;


/// CROSS PRODUCT
/// fr - Produit vectoriel
/// 
/// Given two linearly independent vectors a and b, the cross product,
/// a × b (read "a cross b"), is a vector that is perpendicular to both a and b,
/// and thus normal to the plane containing them. 
/// 
///     Vector u         Vector v
/// [u.0, u.1, u.2] X [v.0, v.1, v2]
///  => Cross Product
/// [u.1 * v.2 - v.1 * u.2 , u.2 * v.0 - v.2 * u.0, u.0 * v.1 - v.0 * u.1]
/// 3Blue1brown https://www.youtube.com/watch?v=eu6i7WJeinw&list=PLZHQObOWTQDPD3MizzM2xVFitgF8hE_ab&index=10

pub fn cross_product<K>(u: &Vector::<K>, v: &Vector::<K>) -> Vector::<K> where K: Number {
    if u.size() != 3 || v.size() != 3 { panic!("cross product can only be done with 3 dimension vectors"); }
    let a: K = u.data[1] * v.data[2] - v.data[1] * u.data[2];
    let b: K = u.data[2] * v.data[0] - v.data[2] * u.data[0];
    let c: K = u.data[0] * v.data[1] - v.data[0] * u.data[1];
    
    Vector::from([a, b, c])

}