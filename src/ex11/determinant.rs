use crate::classes::matrix::Matrix;
use crate::types::number_type::Number;


/// DETERMINANT
/// 
/// Notation
/// det(B) = |B| =
/// | x   x' |
/// | y   y' |
/// 
/// https://fr.wikipedia.org/wiki/D%C3%A9terminant_(math%C3%A9matiques)
/// 2x2 https://www.youtube.com/watch?v=FqAaLGP3Fxo
/// 
/// 3x3 https://www.youtube.com/watch?v=JoC4b_trftM
/// 2 manieres : avec les diagonales ou avec les matrices 2 * 2
/// ////// avec les diagonales
/// | a  b  c | a  b
/// | d  e  f | d  e
/// | g  h  i | g  h
/// 
/// = aei + bfg + cdh - ceg - afh - bdi
/// ///// avec les 2*2
///   | e   f |     | d   f |     | d   e | 
/// a |       | - b |       | - c |       | 
///   | h   i |     | g   i |     | g   h |
/// 
/// 4x4
/// | a b c d |     | (a) - - - |    | - (b) - - |    | - - (c) - |    | - - - (d) |
/// | e f g h |     |  -  f g h |    | e  -  g h |    | e f  -  h |    | e f g  -  |
/// | i j k l | => a|  -  j k l | - b| i  -  k l | + c| i j  -  l | - d| i j k  -  |
/// | m n o p |     |  -  n o p |    | m  -  o p |    | m n  -  p |    | m n o  -  |
///  ...
impl<K> Matrix::<K> where K: Number {
    pub fn determinant(&self) -> K {

        if !self.is_square() { panic!("can't calculate determinant of a non square Matrix");}
        let nm = self.shape();
        
        if nm[0] > 4 { panic!("determinant of n*n matrixes is only supported for n <= 4");}
        // 1x1 Matrix
        if nm[0] == 1 { self.data[0]}
        else if nm[0] == 2 {
            // 2x2 Matrix det(A) is the aera of the parallelogram formed by the image of the basis vectors
            self.data[0] * self.data[3] - self.data[1] * self.data[2]
        }
        else if nm[0] == 3 {
            // 3x3 Matrix
            let add:[[usize; 3]; 3] = [[0, 4, 8], [1, 5, 6], [2, 3, 7]];
            let sub:[[usize; 3]; 3] = [[1, 3, 8], [0, 5, 7], [2, 4, 6]];
            let to_add = add.into_iter().fold( K::zero(), |mut acc, num| {acc = acc + self.data[num[0]] * self.data[num[1]] * self.data[num[2]]; acc});
            let to_sub = sub.into_iter().fold( K::zero(), |mut acc, num| {acc = acc + self.data[num[0]] * self.data[num[1]] * self.data[num[2]]; acc});
            to_add - to_sub
        }
        else {
            // 4x4 Matrix
            let mut num = K::zero();
            for i in 0 .. 4
            {
                let mut vec :Vec<K> = Vec::new();
                self.data.iter().enumerate().for_each(|(j, &x)| {if !(j % 4 == i || j / 4 == 0)  { vec.push(x);}} );
                if i % 2 == 0 {
                    num = num + self.data[i] * Matrix::<K>::from((vec, [3,3])).determinant();
                }
                else {
                    num = num - self.data[i] * Matrix::<K>::from((vec, [3,3])).determinant();
                }
            }
            num
        }
    }
}