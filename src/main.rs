mod vector;
use vector::Vector;
fn main() {
    let popo = [1., 2.];

    let vec = Vector::from(popo);
    let vec2 = Vector::from([1, 2,3]);
    println!("{}",vec2); 

}
