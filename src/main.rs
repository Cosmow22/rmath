use std::io;

mod squareroot;
use squareroot::squareroot;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let number: f64 = input.trim().parse().unwrap();
    println!("{}", squareroot(number, 0.001));

}