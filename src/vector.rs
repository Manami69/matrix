use num::{Num, Zero, FromPrimitive};
use std::fmt;
use std::ops;
use std::ops::{Add, Div, Mul, Sub};
#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
pub struct Vector<T> {
    data: Vec<T>, 
}

impl<T> Vector<T> {
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.data.len()
    }
    fn from_vec(&self, vec:Vec<T>) -> Self {
        Self {data: vec}
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T> where T: Zero + Copy + Add<T, Output= T> + Mul<T, Output= T> + Div<T, Output= T> + Sub<T, Output= T> + FromPrimitive, {
    fn from(d: [T; N]) -> Self {
        Self { data : Vec::<T>::from(d)}
    }
}

impl<T> fmt::Display for Vector<T> where T: fmt::Display + Copy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut to_display = self.data.iter().fold(String::new(), |acc, &num| acc + &format!("[{:.1}]\n", &num));
        to_display.pop();
        write!(f, "{}", to_display)
    }
}

impl<T> ops::Add<Vector<T>> for Vector<T> where T:Num + Copy + Clone +ops::Add<Output=T> {
    type Output = Vector<T>;
    fn add(self, _rhs: Vector<T>) -> Vector<T> {
        if self.size() != _rhs.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        let mut new = Vec::<T>::new();
        let mut i = 0;
        while i < self.size()
        {
            new.push(self.data[i] + _rhs.data[i]);
            i+= 1;
        }
        self.from_vec(new)
    }
}