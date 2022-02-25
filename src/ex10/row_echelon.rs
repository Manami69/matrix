use crate::classes::matrix::Matrix;
use crate::types::number_type::Number;

/// ROW REDUCED ECHELON FORM
/// fr - Matrice échelonnée
/// (je pas compri)
/// 
/// mais si :
/// https://www.youtube.com/watch?v=wY4TAheBsj4
/// https://youtu.be/dlvNcr33M_Y
/// why https://www.youtube.com/watch?v=0MF5iJ9NY0Q
/// 
impl<K> Matrix::<K> where K: Number {

 	pub fn row_echelon(&self) -> Matrix<K> {
        let mut new = self.data.clone();
        let nm = self.shape();
        //let mut cur_pivot_place = 0;
        let mut row = 0;
        let mut pivot: K;
        for cur_pivot_place in 0 .. nm[1] {
            if row >= nm[0] { break ;}

            // Step 1 - Swap if needed
            if new[row * nm[1] + cur_pivot_place].is_zero() {
                for r in row .. nm[0] {
                    if !new[r * nm[1] + cur_pivot_place].is_zero() {
                        for i in 0 .. nm[1] { 
                            new.swap(row * nm[1] + i, r *nm[1] + i);
                        }
                        break ;
                    }
                }
            }
            pivot = new[row * nm[1] + cur_pivot_place];
            if !pivot.is_zero() {
            // Step 2 - Scale the row to make the pivot 1 (by divising)
                for i in 0 .. nm[1] {new[row * nm[1] + i] =  new[row * nm[1] + i] / pivot}
        
            // Step 3 - get zeros above and below the pivot (substract by the first row multiplied by the number we need to get to zero and do it to all the row)

                for r in 0 .. nm[0] {
                    if r == row {continue;}
                    let to_make_zero = new[r * nm[1] + cur_pivot_place];
                    for col in 0 .. nm[1] {
                        new[r * nm[1] + col] = new[r * nm[1] + col] - (new[row * nm[1] + col] * to_make_zero);
                    }
                }
                row += 1;
            }
        }
        Matrix::from((new, nm))
    }
}