use crate::classes::vector::Vector;
use crate::types::number_type::Number;
use crate::types::trait_absolute::Absolute;
use std::ops::{Div};
use num::{Zero};


/// COS
/// 
/// the dot product of 2 vector can be calculated with both vectors' norms and cos θ
/// ⟨u,v⟩ = ‖u‖ ‖v‖ cos(θ)
/// with u != 0 and v != 0
/// 
/// then cos(θ) = u.v / ‖u‖ ‖v‖
pub fn angle_cos<K>(u: &Vector::<K>, v: &Vector::<K>) -> K where K : Number + Copy + Zero + Absolute + Div<f32, Output=K>, {
    u.dot(v) / (u.norm() * v.norm())
}