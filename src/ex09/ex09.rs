use crate::classes::{matrix::Matrix, complex::Complexf32};

pub fn ex09() {
	let m = Matrix::from([[1., 2.]]);
	println!("Base:\n{} \nTransposition:\n{}\n", m, m.transpose());
	let m = Matrix::from([[1., 2.],[3., 4.]]);
	println!("Base:\n{} \nTransposition:\n{}\n", m, m.transpose());
	let m = Matrix::from([[1., 3., 5.], [2., 4., 6.]]);
	println!("Base:\n{} \nTransposition:\n{}\n", m, m.transpose());
	let m = Matrix::from([[1., 2., 3., 4.], [5., 6., 7., 8.],[9., 10., 11., 12.], [13., 14., 15., 16.], [17.,18.,19.,20.]]);
	println!("Base:\n{} \nTransposition:\n{}\n", m, m.transpose());
    let m = Matrix::from([
        [Complexf32::new(2.,1.), Complexf32::new(3., 2.)],
        [Complexf32::new(4., -1.), Complexf32::new(2., 1.)],
        [Complexf32::new(6., -8.), Complexf32::new(5., 7.)]
        ]);
    println!("Base:\n{} \nTransposition:\n{}\n", m, m.transpose());

}