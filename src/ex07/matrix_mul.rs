use crate::classes::{matrix::Matrix, vector::Vector};
use crate::types::number_type::Number;

/// Matrix mul image representation :
/// https://en.wikipedia.org/wiki/Matrix_multiplication#/media/File:Matrix_multiplication_diagram_2.svg
impl<K> Matrix<K>
where
    K: Number,
{
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        let mn = self.shape();
        let size = vec.size();
        if mn[1] != size {
            panic!("For matrix and vector multiplication, the number of col in the matrix must be equal to the vector dimension.")
        }
        let mut new = Vec::<K>::new();
        for row in 0..mn[0] {
            let mut num = K::zero();

            for col in 0..mn[1] {
                num = num + self.get_val(row, col) * vec.data[col];
            }
            new.push(num);
        }
        Vector::from(new)
    }
    /// Matrix mul
    ///
    /// If A is an m × n matrix and B is an n × p matrix,
    /// the matrix product C = AB (denoted without multiplication signs or dots)
    /// is defined to be the m × p matrix
    /// https://en.wikipedia.org/wiki/Matrix_multiplication#Definition
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        let mn = self.shape();
        let np = mat.shape();
        if mn[1] != np[0] {
            panic!("For matrix multiplication, the number of columns in the first matrix must be equal to the number of rows in the second matrix.")
        }
        let mut new = Vec::<K>::new();
        for i in 0..mn[0] * np[1] {
            let row = i / np[1];
            let col = i % np[1];
            let mut num = K::zero();
            for j in 0..mn[1] {
                num = num + self.get_val(row, j) * mat.get_val(j, col);
            }
            new.push(num);
        }
        Matrix::from((new, [mn[0], np[1]]))
    }
}
