use crate::classes::matrix::Matrix;
use crate::types::number_type::Number;

/// TRACE
///
/// the trace of a square matrix A, denoted tr(A),
/// is defined to be the sum of elements on the main diagonal
/// (from the upper left to the lower right) of A.
/// The trace is only defined for a square matrix (n × n).

impl<K> Matrix<K>
where
    K: Number,
{
    pub fn trace(&self) -> K {
        if !self.is_square() {
            panic!("The trace is only defined for a square matrix (n × n).");
        }
        let mn = self.shape();
        let mut num = K::zero();
        for i in 0..mn[0] {
            num = num + self.get_val(i, i);
        }
        num
    }
}
