mod vector;

fn main() {
    let popo = [1, 2, 3, 4];

    let vec = vector::Vector::from(popo);
    
    println!("{}", vec); 

}
