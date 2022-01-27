use crate::classes::matrix::Matrix;
use crate::types::number_type::Number;
use num::Zero;

impl<K> Matrix::<K> where K : Number {
    fn transpose(&mut self) -> Matrix::<K>
    {
        todo!()
    }   
}