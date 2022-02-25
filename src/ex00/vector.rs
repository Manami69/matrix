use crate::classes::vector::Vector;
use crate::types::number_type::Number;

impl<K> Vector<K>
where
    K: Number,
{
    pub fn add(&mut self, v: &Vector<K>) {
        if self.size() != v.size() {
            panic!("cannot add 2 vector with different dimensions");
        }
        self.data = self
            .data
            .iter()
            .zip(v.data.iter())
            .map(|x| K::clone(x.0) + K::clone(x.1))
            .collect();
    }
    pub fn sub(&mut self, v: &Vector<K>) {
        if self.size() != v.size() {
            panic!("cannot sub 2 vector with different dimensions");
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
