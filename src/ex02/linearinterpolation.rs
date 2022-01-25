use std::ops::{Add, Mul, Sub};

/// LINEAR INTERPOLATION
/// fr - interpolation lineaire
/// 
/// Let t ∈ [0; 1](⊂ R) (ie, t is real, and 0 ≤ t ≤ 1) be a scalar
/// Let f : (V × V × [0; 1]) → V be the function to implement.
/// 
/// The value of t allows us to "slide" between the two values.
/// 
/// • If t = 0, then f (u, v, t) = u.
/// • If t = 1, then f (u, v, t) = v.
/// • If t = 0.5, then the function returns a value at the exact middle in between of u
///   and v (the isobarycenter, which can be understood as the center of gravity, of the
///   two points)
/// 
/// Formula : https://en.wikipedia.org/wiki/Linear_interpolation
/// lerp function exlanations https://lunarlabs.pt/blog/post/the_art_of_lerp
///
/// there is 2 different formula but they don't have the same precision for a given t
/// so I choose to follow one of the comments here :
/// https://stackoverflow.com/questions/4353525/floating-point-linear-interpolation

pub fn lerp<V>(u: V, v: V, t: f32) -> V where V: Add<V, Output=V> + Sub<V, Output=V> + Mul<f32, Output=V> + Clone, {

    if t <= 0.5 { u.clone() + (v - u) * t }
    else {v.clone() - (v - u) * (1.0 - t) }
}

