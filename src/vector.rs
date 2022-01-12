use num::Num;
use std::fmt;

pub struct Vector<T> {
    data: Vec<T>, 
}

impl<T> Vector<T> where T: Num + Copy ,  {
    pub fn size(&self) -> usize {
        self.data.len()
    } 
}

impl<T, const N: usize> From<[T; N]> for Vector<T> where T: Num + Copy, {
    fn from(d: [T; N]) -> Self {
        Self { data : Vec::<T>::from(d)}
    }
}
impl<T> fmt::Display for Vector<T> where T: fmt::Display + Copy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.data.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", "))
    }
}