use crate::classes::{vector::Vector, complex::Complexf32};
use crate::types::number_type::Number;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
pub struct Matrix<K> where K : Number{
    /// Returns a Matrix 
    /// 
    /// # Arguments
    /// 
    ///  * number arrays of same size inside one array
    /// 
    /// # values
    /// 
    /// * data : vec with all elements of the Matrix
    /// * n : number of column
    /// * m : number of rows 
    pub data: Vec<K>,
    m: usize,
    n: usize,
}


impl<K> Matrix<K> where K : Number {
    #[allow(dead_code)]
    pub fn shape(&self) -> [usize; 2] {
        [self.m, self.n]
    }
    pub fn get_data(&self) -> Vec<K> {
        self.data.clone()
    }
	fn from_vec(&self, vec:Vec<K>, shape:[usize; 2]) -> Self {
		Self {data: vec, m: shape[0], n: shape[1]}
	}
	pub fn is_square(&self) -> bool {
		self.m == self.n
	}
}


/// CONSTRUCTOR FROM 
///
impl<K, const N: usize, const M: usize> From<[[K; N]; M]> for Matrix<K> where K: Number {
    fn from(d: [[K; N]; M]) -> Self {
        let mut vec = Vec::<K>::new();
        for item in d.into_iter().enumerate() {
            let mut next = Vec::<K>::from(item.1);
            vec.append(&mut next);
        }
        Self { data : vec, m: M, n: N}
    }
}
impl<K> From<(Vector<K>, usize)> for Matrix<K> where K: Number {
    fn from(d : ( Vector<K>, usize)) -> Self {
		if d.0.size() % d.1 != 0 { panic!("The vector you're trying to convert has wrong dimension"); }
        Self { data : d.0.get_data(), m: d.0.size() / d.1, n: d.1}
    }
}
impl<K> From<(&Vector<K>, usize)> for Matrix<K> where K: Number {
    fn from(d : ( &Vector<K>, usize)) -> Self {
		if d.0.size() % d.1 != 0 { panic!("The vector you're trying to convert has wrong dimension"); }
        Self { data : d.0.get_data(), m: d.0.size() / d.1, n: d.1}
    }
}

impl<K> From<Vector<K>> for Matrix<K> where K: Number {
    fn from(d :  Vector<K>) -> Self {
        Self { data : d.get_data(), m: d.size(), n: 1}
    }
}

impl<K> From<&Vector<K>> for Matrix<K> where K: Number {
    fn from(d :  &Vector<K>) -> Self {
        Self { data : d.get_data(), m: d.size(), n: 1}
    }
}

///
/// DISPLAY
/// 
impl<K> fmt::Display for Matrix<K> where K: Number + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_display = String::new();
        to_display.push('[');
        to_display.push('[');
        for (pos, e) in self.data.iter().enumerate() {
            to_display += &format!("{:.1}", e);
            if (pos + 1) % self.n == 0 
            {
                to_display += "]\n[";
            }
            else { to_display += ", "}
        }
        to_display.pop();
        to_display.pop();
        to_display.push(']');
        write!(f, "{}", to_display)
    }
}


/// OPERATOR + / - / * OVERLOADING
/// 
/// *** Use references(&) to not consume the Vectors ***
/// 
/// * All operation can be made with or without references if you are finished with your old Vector.s

/// ADD
impl<K> Add<&Matrix<K>> for &Matrix<K> where K: Number {
    type Output = Matrix<K>;
    fn add(self, _rhs: &Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot add 2 matrixes with different shapes");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect(), self.shape())
    }
}

impl<K> Add<Matrix<K>> for &Matrix<K> where K: Number {
    type Output = Matrix<K>;
    fn add(self, _rhs: Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot add 2 matrixes with different shapes");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect(), self.shape())
    }
}

impl<K> Add<&Matrix<K>> for Matrix<K> where K: Number {
    type Output = Matrix<K>;
    fn add(self, _rhs: &Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot add 2 matrixes with different shapes");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect(), self.shape())
    }
}

impl<K> Add<Matrix<K>> for Matrix<K> where K: Number {
    type Output = Matrix<K>;
    fn add(self, _rhs: Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot add 2 matrixes with different shapes");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) + K::clone(x.1)).collect(), self.shape())
    }
}

/// SUB
impl<K> Sub<&Matrix<K>> for &Matrix<K> where K: Number {
    type Output = Matrix<K>;
    fn sub(self, _rhs: &Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot sub 2 matrixes with different shapes");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect(), self.shape())
    }
}

impl<K> Sub<Matrix<K>> for &Matrix<K> where K: Number {
    type Output = Matrix<K>;
    fn sub(self, _rhs: Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot sub 2 matrixes with different shapes");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect(), self.shape())
    }
}

impl<K> Sub<&Matrix<K>> for Matrix<K> where K: Number {
    type Output = Matrix<K>;
    fn sub(self, _rhs: &Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot sub 2 matrixes with different shapes");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect(), self.shape())
    }
}

impl<K> Sub<Matrix<K>> for Matrix<K> where K: Number {
    type Output = Matrix<K>;
    fn sub(self, _rhs: Matrix<K>) -> Matrix<K> {
        if self.shape() != _rhs.shape() {
            panic!("cannot sub 2 matrixes with different shapes");
        }
        self.from_vec(self.data.iter().zip(_rhs.data.iter()).map(|x| K::clone(x.0) - K::clone(x.1)).collect(), self.shape())
    }
}

// SCALE

impl<K> Mul<K> for Matrix<K> where K: Number + Copy {
    type Output = Matrix<K>;
    fn mul(self, _rhs: K) -> Matrix<K> {
        self.from_vec(self.data.iter().map(|x| K::clone(x) * _rhs).collect(), self.shape())
    }
}

impl<K> Mul<K> for &Matrix<K> where K: Number + Copy {
    type Output = Matrix<K>;
    fn mul(self, _rhs: K) -> Matrix<K> {
        self.from_vec(self.data.iter().map(|x| K::clone(x) * _rhs).collect(), self.shape())
    }
}