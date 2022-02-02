use crate::classes::{matrix::Matrix};
/// PROJECTION MATRIX
/// 
/// fov => angle in radian , ratio => h / w, near and far => normalization of image space between - 1 and 1 in z
// tuto : https://www.youtube.com/watch?v=EqNcqBdrNyI
// calculs : http://www.terathon.com/gdc07_lengyel.pdf
pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix::<f32> {
    let focal_length : f32 = 1. / (fov / 2.).tan();
    Matrix::from([[focal_length,         0.,                  0.,                            0.                          ],
                  [0.,                   focal_length/ ratio, 0.,                            0.                          ],
                  [0.,                   0.,                  -((far + near)/ (far - near)), -((2. * far * near) / (far - near))],
                  [0.,                   0.,                  -1.,                           0.                          ]
                ])
}
