// use std::io;

mod trigo;
use trigo::cos_complementary;

fn main() {
    let x = 67.0;
    println!("cos({}) = {}", x, cos_complementary(x));
}