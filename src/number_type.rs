use std::ops::{Add, Div, Mul, Sub};


pub trait Number<Rhs = Self, Output = Self> : Clone +  Add<Rhs, Output = Output>   + Mul<Rhs, Output= Output> + Div<Rhs, Output= Output> + Sub<Rhs, Output= Output> {}
impl<K, Rhs, Output> Number<Rhs, Output> for K where K : Clone +  Add<Rhs, Output = Output>   + Mul<Rhs, Output= Output> + Div<Rhs, Output= Output> + Sub<Rhs, Output= Output> {}