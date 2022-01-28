use crate::classes::matrix::Matrix;
use crate::types::number_type::Number;
use num::Zero;
/// TRANSPOSE
/// 
/// the transpose of a matrix is an operator which flips a matrix over its diagonal.
/// https://en.wikipedia.org/wiki/Transpose
impl<K> Matrix::<K> where K : Number + Zero + Copy {
    pub fn transpose(&self) -> Matrix::<K>
    {
		let nm : [usize; 2] = self.shape();
		let mut data = vec![K::zero(); nm[0] * nm[1]];
		//let mut new = Matrix::from((data, [nm[1], mn[0]]));

		for rowtocol in 0 .. nm[0] {
			for coltorow in 0 .. nm[1] {
				data[coltorow * nm[0] + rowtocol]= self.get_val(rowtocol, coltorow);
			}
		}
		Matrix::from((data, [nm[1], nm[0]]))
		//todo!();
    }
}