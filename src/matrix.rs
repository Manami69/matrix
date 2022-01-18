use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
pub struct Matrix<V> {
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
    data: Vec<V>,
    m: usize,
    n: usize,
}


impl<V> Matrix<V> where V : Clone {
    #[allow(dead_code)]
    pub fn shape(&self) -> [usize; 2] {
        [self.m, self.n]
    }
    pub fn get_data(&self) -> Vec<V> {
        let ret = self.data.clone();
        ret
    }
}

impl<V, const N: usize, const M: usize> From<[[V; N]; M]> for Matrix<V> where V: Add<V, Output= V> + Mul<V, Output= V> + Div<V, Output= V> + Sub<V, Output= V>, {
    fn from(d: [[V; N]; M]) -> Self {
        let mut vec = Vec::<V>::new();
        for item in d.into_iter().enumerate() {
            let mut next = Vec::<V>::from(item.1);
            vec.append(&mut next);
        }
        Self { data : vec, m: M, n: N}
    }
}

impl<V> fmt::Display for Matrix<V> where V: fmt::Display  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_display = String::new();
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
        write!(f, "{}", to_display)
    }
}