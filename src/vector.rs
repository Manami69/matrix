use num::Num;
use std::fmt;
use std::ops;
#[derive(PartialEq, PartialOrd, Eq, Clone, Hash, Debug)]
pub struct Vector<T> {
    data: Vec<T>, 
}

impl<T> Vector<T> {
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.data.len()
    } 
}

impl<T, const N: usize> From<[T; N]> for Vector<T> where T: Num + num::Zero + Copy , {
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
        let mut new = Vector::from([]);
        let mut i = 0;
        while i < self.size()
        {
            new.data.push(self.data[i] + _rhs.data[i]);
            i+= 1;
        }
        new
    }
}