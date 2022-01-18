use crate::matrix;
use matrix::Matrix;
use num::{Zero};
use std::fmt;
use std::ops;
use std::ops::{Add, Div, Mul, Sub};


/// VECTOR CLASS

#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
pub struct Vector<K> {
    data: Vec<K>, 
}

impl<K> Vector<K> {
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.data.len()
    }
    fn from_vec(&self, vec:Vec<K>) -> Self {
        Self {data: vec}
    }
}


/// CONSTRUCTOR FROM :
/// * An Array of K
/// * a simple Matrix
/// * a simple Vector
/// * a Vector Array 
impl<K, const N: usize> From<[K; N]> for Vector<K> where K:  Add<K, Output= K> + Mul<K, Output= K> + Div<K, Output= K> + Sub<K, Output= K>, {
    fn from(d: [K; N]) -> Self {
        Self { data : Vec::<K>::from(d)}
    }
}

impl <K> From<Matrix<K>> for Vector<K> where K: Zero + Clone + Add<K, Output= K> + Mul<K, Output= K> + Div<K, Output= K> + Sub<K, Output= K>, {
    fn from (d: Matrix<K>) -> Self {
        Self { data : d.get_data() }
    }
}

impl <K> From<&Matrix<K>> for Vector<K> where K: Zero + Clone + Add<K, Output= K> + Mul<K, Output= K> + Div<K, Output= K> + Sub<K, Output= K>, {
    fn from (d: &Matrix<K>) -> Self {
        Self { data : d.get_data() }
    }
}


impl <K> From<&Vector<K>> for Vector<K> where K: Zero + Clone + Add<K, Output= K> + Mul<K, Output= K> + Div<K, Output= K> + Sub<K, Output= K>, {
    fn from (d: &Vector<K>) -> Self {
        Self { data : d.data.clone() }
    }
}

/// Display Vector
impl<K> fmt::Display for Vector<K> where K: fmt::Display  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_display = self.data.iter().fold(String::new(), |acc, num| acc + &format!("[{:.1}]\n", &num));
        to_display.pop();
        write!(f, "{}", to_display)
    }
}

/// Exercise 00 - Add, Subtract and Scale



 

/// OPERATOR + / - / * OVERLOADING
/// 
/// *** Use references(&) to not consume the Vectors ***
/// 
/// * All operation can be made with or without references if you are finished with your old Vector.s

/// ADD
impl<K> ops::Add<&Vector<K>> for &Vector<K> where K: Copy + Clone +ops::Add<Output=K> {
    type Output = Vector<K>;
    fn add(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect())
    }
}

impl<K> ops::Add<Vector<K>> for &Vector<K> where K: Clone + ops::Add<Output=K> {
    type Output = Vector<K>;
    fn add(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect())
    }
}

impl<K> ops::Add<&Vector<K>> for Vector<K> where K: Clone +ops::Add<Output=K> {
    type Output = Vector<K>;
    fn add(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect())
    }
}

impl<K> ops::Add<Vector<K>> for Vector<K> where K:Copy + Clone +ops::Add<Output=K> {
    type Output = Vector<K>;
    fn add(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect())
    }
}

/// SUB
impl<K> Sub<&Vector<K>> for &Vector<K> where K: Copy + Clone +Sub<Output=K> {
    type Output = Vector<K>;
    fn sub(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect())
    }
}

impl<K> Sub<Vector<K>> for &Vector<K> where K: Clone + Sub<Output=K> {
    type Output = Vector<K>;
    fn sub(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect())
    }
}

impl<K> Sub<&Vector<K>> for Vector<K> where K: Clone +Sub<Output=K> {
    type Output = Vector<K>;
    fn sub(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect())
    }
}

impl<K> Sub<Vector<K>> for Vector<K> where K:Copy + Clone +Sub<Output=K> {
    type Output = Vector<K>;
    fn sub(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect())
    }
}

// SCALE

impl<K> Mul<K> for Vector<K> where K: Copy + Clone + Mul<Output=K> + Into<f64> {
    type Output = Vector<K>;
    fn mul(self, _rhs: K) -> Vector<K> {
        self.from_vec(self.data.iter().map(|x| K::clone(x) * _rhs).collect())
    }
}

impl<K> Mul<K> for &Vector<K> where K: Copy + Clone + Mul<Output=K> + Into<f64>{
    type Output = Vector<K>;
    fn mul(self, _rhs: K) -> Vector<K> {
        self.from_vec(self.data.iter().map(|x| K::clone(x) * _rhs).collect())
    }
}