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

impl <K> From<Matrix<K>> for Vector<K> where K: Zero + Copy + Add<K, Output= K> + Mul<K, Output= K> + Div<K, Output= K> + Sub<K, Output= K>, {
    fn from (d: Matrix<K>) -> Self {
        Self { data : d.get_data() }
    }
}

impl <K> From<&Matrix<K>> for Vector<K> where K: Zero + Copy + Add<K, Output= K> + Mul<K, Output= K> + Div<K, Output= K> + Sub<K, Output= K>, {
    fn from (d: &Matrix<K>) -> Self {
        Self { data : d.get_data() }
    }
}


impl <K> From<&Vector<K>> for Vector<K> where K: Zero + Copy + Add<K, Output= K> + Mul<K, Output= K> + Div<K, Output= K> + Sub<K, Output= K>, {
    fn from (d: &Vector<K>) -> Self {
        Self { data : d.data.clone() }
    }
}

/// Display Vector
impl<K> fmt::Display for Vector<K> where K: fmt::Display + Copy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_display = self.data.iter().fold(String::new(), |acc, &num| acc + &format!("[{:.1}]\n", &num));
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


impl<K> ops::Add<&Vector<K>> for &Vector<K> where K: Copy + Clone +ops::Add<Output=K> {
    type Output = Vector<K>;
    fn add(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        let mut new = Vec::<K>::new();
        let mut i = 0;
        while i < self.size()
        {
            new.push(self.data[i] + _rhs.data[i]);
            i+= 1;
        }
        self.from_vec(new)
    }
}

impl<K> ops::Add<Vector<K>> for &Vector<K> where K: Copy + Clone +ops::Add<Output=K> {
    type Output = Vector<K>;
    fn add(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        let mut new = Vec::<K>::new();
        let mut i = 0;
        while i < self.size()
        {
            new.push(self.data[i] + _rhs.data[i]);
            i+= 1;
        }
        self.from_vec(new)
    }
}

impl<K> ops::Add<&Vector<K>> for Vector<K> where K: Copy + Clone +ops::Add<Output=K> {
    type Output = Vector<K>;
    fn add(self, _rhs: &Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        let mut new = Vec::<K>::new();
        let mut i = 0;
        while i < self.size()
        {
            new.push(self.data[i] + _rhs.data[i]);
            i+= 1;
        }
        self.from_vec(new)
    }
}

impl<K> ops::Add<Vector<K>> for Vector<K> where K: Copy + Clone +ops::Add<Output=K> {
    type Output = Vector<K>;
    fn add(self, _rhs: Vector<K>) -> Vector<K> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        let mut new = Vec::<K>::new();
        let mut i = 0;
        while i < self.size()
        {
            new.push(self.data[i] + _rhs.data[i]);
            i+= 1;
        }
        self.from_vec(new)
    }
}