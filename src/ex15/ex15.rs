use num::complex::Complex;
use crate::classes::complex::Complexf32;
use crate::types::trait_absolute::Absolute;
/// COMPLEX NUMBERS
/// 
/// Complex are tested all over the project,
/// to use it we just need to create a class that can do operations 
/// that we need for this project which are :
/// addition, substraction, multiplication, division and absolute.
/// There is a complex type in rust to test my own class
/// but absolute value is not supported
pub fn ex15() {
    let numbers = [3., 4., 43., 5.];
    let my_a = Complexf32::new(numbers[0], numbers[1]);
    let my_b = Complexf32::new(numbers[2], numbers[3]);
    let rust_a = Complex::new(numbers[0], numbers[1]);
    let rust_b = Complex::new(numbers[2], numbers[3]);
    println!("ADDITION\nMine : \n{} + {} = {}\nRust :\n{} + {} = {}", my_a, my_b, my_a + my_b, rust_a, rust_b, rust_a + rust_b);
    println!("SUBSTRACTION\nMine : \n{} - {} = {}\nRust :\n{} - {} = {}", my_a, my_b, my_a - my_b, rust_a, rust_b, rust_a - rust_b);
    println!("MULTIPLICATION\nMine : \n{} * {} = {}\nRust :\n{} * {} = {}", my_a, my_b, my_a * my_b, rust_a, rust_b, rust_a * rust_b);
    println!("DIVISION\nMine : \n{} / {} = {}\nRust :\n{} / {} = {}", my_a, my_b, my_a / my_b, rust_a, rust_b, rust_a / rust_b);
    // test absolute value here https://keisan.casio.com/exec/system/1223049325
    println!("ABSOLUTE VALUE of {} is {}", my_a, my_a.abs());


}