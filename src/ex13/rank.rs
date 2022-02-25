use crate::classes::matrix::Matrix;
use crate::types::number_type::Number;

/// RANK
///
/// https://en.wikipedia.org/wiki/Rank_(linear_algebra)
///
/// The rank is the column space dimension of the matrix A.
/// Each column of A represents a vector.
/// the Rank is the number of column vector which are linearly independant.
/// To know the rank of A we need to find every linear combination
/// between all colomn vectors.
///
/// to calculate the row it's easier to use the row reduced form rref(A) and
/// count the number of pivot since every other vector can be found as
/// a linear combination of the vectors before
impl<K> Matrix<K>
where
    K: Number,
{
    pub fn rank(&self) -> usize {
        let rref = self.row_echelon();
        let nm = rref.shape();
        let mut count: usize = 0;
        for row in 0..nm[0] {
            for col in 0..nm[1] {
                if !rref.get_val(row, col).is_zero() {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
}
