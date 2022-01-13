use num::{Zero};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
pub struct Matrix<T> {
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
    data: Vec<T>,
    m: usize,
    n: usize,
}

// impl<T> Matrix<T> {
//     #[allow(dead_code)]
//     pub fn size(&self) -> usize {
//         self.data.len()
//     }
//     fn from_vec(&self, vec:Vec<T>) -> Self {
//         Self {data: vec}
//     }
// }

impl<T, const N: usize, const M: usize> From<[[T; N]; M]> for Matrix<T> where T: Zero + Copy + Add<T, Output= T> + Mul<T, Output= T> + Div<T, Output= T> + Sub<T, Output= T>, {
    fn from(d: [[T; N]; M]) -> Self {
        let mut vec = Vec::<T>::new();
        for item in d.into_iter().enumerate() {
            let mut next = Vec::<T>::from(item.1);
            vec.append(&mut next);
        }
        Self { data : vec, m: d.len(), n: d[0].len()}
    }
}

impl<T> fmt::Display for Matrix<T> where T: fmt::Display  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_display = String::new();
        to_display.push('[');
        for (pos, e) in self.data.iter().enumerate() {
            to_display += &e.to_string();
            if (pos + 1) % self.n == 0 
            {
                to_display += "]\n[";
            }
            else { to_display += ", "}
        }
        to_display.pop();
        to_display.pop();
        write!(f, "{}", to_display)
    }
}