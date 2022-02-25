use std::fmt;
use crate::classes::matrix::Matrix;
use crate::types::number_type::Number;

/// INVERSE
/// 
/// computation https://www.mathsisfun.com/algebra/matrix-inverse-row-operations-gauss-jordan.html
/// for a matrix A, the inverse of that matrix is A-1.
/// I(n) is the identity matrix of size n * n
/// for a matrix A of size n*n, A * A-1 = I(n)
/// then we'll use the same method (Gauss-Jordan Method) as the row echelon computation 
/// but doing it on the identity matrix in the same time to get the inverse.
/// 
/// If the Matrix determinant is zero, the matrix is singular and an error is returned;
/// https://www.mathsisfun.com/algebra/matrix-inverse.html
impl<K> Matrix::<K> where K : Number + From<f32> {
	/// IDENTITY
	/// 
	/// return identity matrix Vec dataset of size n*n
    fn identity(&self, n: usize) -> Vec<K> {
		let mut id = vec![K::zero(); n*n];
        for i in 0 .. n
        {
			id[i * n + i] = (1.).into();
        }
        id
    }

    pub fn inverse(&self) -> Result<Matrix::<K>, SingularMatrix> {
        if !self.is_square() { panic!("Cannot compute non square matrix inverse")}
        let nm = self.shape();
        if self.determinant().is_zero() { return  Err(SingularMatrix)}
        let mut last = Vec::from(self.data.clone());
        let mut next = self.identity(nm[0]);
        let mut pivot: K;
        for i in 0 .. nm[0] {
            // Step 1 - Swap if needed
            if last[i * nm[1] + i].is_zero() {
                for r in i .. nm[0] {
                    if !last[r * nm[1] + i].is_zero() {
                        for j in 0 .. nm[1] { 
                            last.swap(i * nm[1] + j, r *nm[1] + j);
                            next.swap(i * nm[1] + j, r *nm[1] + j);
                        }
                        break ;
                    }
                }
            }
            pivot = last[i * nm[1] + i];
            // Step 2 - Scale the row to make the pivot 1 (by divising)
            for j in 0 .. nm[1] {
                last[i * nm[1] + j] =  last[i * nm[1] + j] / pivot;
                next[i * nm[1] + j] =  next[i * nm[1] + j] / pivot;
            }
            // Step 3 - get zeros above and below the pivot
            for r in 0 .. nm[0] {
                if r == i {continue;}
                let to_make_zero = last[r * nm[1] + i];
                for col in 0 .. nm[1] {
                    last[r * nm[1] + col] = last[r * nm[1] + col] - (last[i * nm[1] + col] * to_make_zero);
                    next[r * nm[1] + col] = next[r * nm[1] + col] - (next[i * nm[1] + col] * to_make_zero);
                }
            }

        }
        Ok(Matrix::from((next, nm)))
    }
}
#[derive(Debug, Clone)]
pub struct SingularMatrix;

impl fmt::Display for SingularMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot find inverse of a singular matrix.")
    }
}
