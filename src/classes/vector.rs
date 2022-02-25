use crate::classes::{matrix, complex::Complexf32};
use crate::types::number_type::Number;
use matrix::Matrix;
use num::{Zero};
use std::fmt;
use std::ops::{Add, Mul, Sub};
/// VECTOR CLASS

#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
pub struct Vector<K> where K : Number{
    pub data: Vec<K>, 
}

impl<K> Vector<K> where K: Number {
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.data.len()
    }
    fn from_vec(&self, vec:Vec<K>) -> Self {
        Self {data: vec}
    }
	pub fn get_data(&self) -> Vec<K> {
		self.data.clone()
	}
    pub fn is_zero(&self) -> bool where K : Zero, {
        for d in &self.data {
            if !d.is_zero() {
                return false;
            }
        }
        true
    }
    // pub fn zero(&self, len: usize) -> Self where K: Zero {
    //     Self {data: vec![K::zero(); len]}
    // }
}


/// CONSTRUCTOR FROM :
/// * An Array of K
/// * a simple Matrix
/// * a simple Vector
/// * a Vector Array 
impl<K, const N: usize> From<[K; N]> for Vector<K> where K: Number {
    fn from(d: [K; N]) -> Self {
        Self { data : Vec::<K>::from(d)}
    }
}

impl <K> From<Matrix<K>> for Vector<K> where K: Number {
    fn from (d: Matrix<K>) -> Self {
        Self { data : d.get_data() }
    }
}

impl <K> From<&Matrix<K>> for Vector<K> where K: Number {
    fn from (d: &Matrix<K>) -> Self {
        Self { data : d.get_data() }
    }
}


impl <K> From<&Vector<K>> for Vector<K> where K: Number {
    fn from (d: &Vector<K>) -> Self {
        Self { data : d.get_data() }
    }
}


impl <K> From<usize> for Vector<K> where K: Number  {
    fn from (d: usize) -> Self {
        Self { data : vec![K::zero(); d] }
    }
}

impl <K> From<Vec<K>> for Vector<K> where K: Number  {
    fn from (d: Vec<K>) -> Self {
        Self { data : d }
    }
}
/// Display Vector
impl<K> fmt::Display for Vector<K> where K: Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_display = self.data.iter().fold(String::new(), |acc, num| acc + &format!("[{}]\n", &num));
        to_display.pop();
        write!(f, "{}", to_display)
    }
}


/// OPERATOR + / - / * OVERLOADING
/// 
/// *** Use references(&) to not consume the Vectors ***
/// 
/// * All operation can be made with or without references if you are finished with your old Vector.s

/// ADD
impl<K> Add<&Vector<K>> for &Vector<K> where K: Number {
    type Output = Vector<K>;
    fn add(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect())
    }
}

impl<K> Add<Vector<K>> for &Vector<K> where K: Number {
    type Output = Vector<K>;
    fn add(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect())
    }
}

impl<K> Add<&Vector<K>> for Vector<K> where K: Number {
    type Output = Vector<K>;
    fn add(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect())
    }
}

impl<K> Add<Vector<K>> for Vector<K> where K: Number {
    type Output = Vector<K>;
    fn add(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect())
    }
}

/// SUB
impl<K> Sub<&Vector<K>> for &Vector<K> where K:  Number {
    type Output = Vector<K>;
    fn sub(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect())
    }
}

impl<K> Sub<Vector<K>> for &Vector<K> where K: Number {
    type Output = Vector<K>;
    fn sub(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect())
    }
}

impl<K> Sub<&Vector<K>> for Vector<K> where K: Number {
    type Output = Vector<K>;
    fn sub(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect())
    }
}

impl<K> Sub<Vector<K>> for Vector<K> where K: Number {
    type Output = Vector<K>;
    fn sub(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot sub 2 vector with different dimensions");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect())
    }
}

// SCALE

impl<K> Mul<K> for Vector<K> where K: Number  {
    type Output = Vector<K>;
    fn mul(self, _rhs: K) -> Vector<K> {
        self.from_vec(self.data.iter().map(|x| K::clone(x) * _rhs).collect())
    }
}

impl<K> Mul<K> for &Vector<K> where K: Number {
    type Output = Vector<K>;
    fn mul(self, _rhs: K) -> Vector<K> {
        self.from_vec(self.data.iter().map(|x| K::clone(x) * _rhs).collect())
    }
}

impl Mul<f32> for Vector<Complexf32> {
    type Output = Vector<Complexf32>;
    fn mul(self, _rhs: f32) -> Vector<Complexf32> {
        self.from_vec(self.data.iter().map(|x| *x * _rhs).collect())
    }
}
impl Mul<f32> for &Vector<Complexf32> {
    type Output = Vector<Complexf32>;
    fn mul(self, _rhs: f32) -> Vector<Complexf32> {
        self.from_vec(self.data.iter().map(|x| *x * _rhs).collect())
    }
}

impl<K> Mul<&K> for Vector<K> where K: Number  {
    type Output = Vector<K>;
    fn mul(self, _rhs: &K) -> Vector<K> {
        self.from_vec(self.data.iter().map(|x| K::clone(x) * K::clone(_rhs)).collect())
    }
}

impl<K> Mul<&K> for &Vector<K> where K: Number {
    type Output = Vector<K>;
    fn mul(self, _rhs: &K) -> Vector<K> {
        self.from_vec(self.data.iter().map(|x| K::clone(x) * K::clone(_rhs)).collect())
    }
}