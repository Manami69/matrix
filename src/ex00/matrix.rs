use crate::classes::matrix::Matrix;
use crate::types::number_type::Number;

impl<K> Matrix<K>
where
    K: Number,
{
    pub fn add(&mut self, v: &Matrix<K>) {
        if self.shape() != v.shape() {
            panic!("cannot add 2 matrixes with different shapes");
        }
        self.data = self
            .data
            .iter()
            .zip(v.data.iter())
            .map(|x| K::clone(x.0) + K::clone(x.1))
            .collect();
    }
    pub fn sub(&mut self, v: &Matrix<K>) {
        if self.shape() != v.shape() {
            panic!("cannot sub 2 matrixes with different shapes");
        }
        self.data = self
            .data
            .iter()
            .zip(v.data.iter())
            .map(|x| K::clone(x.0) - K::clone(x.1))
            .collect();
    }
    pub fn scl(&mut self, a: K)
    where
        K: Copy,
    {
        self.data = self.data.iter().map(|x| K::clone(x) * a).collect();
    }
}
