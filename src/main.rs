use std::io;

mod others;
mod pi;
mod squareroot; 

use squareroot::squareroot;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let number: u32 = input.trim().parse().unwrap();
    println!("{}", 1.0 /  ((2.0*squareroot(2.0, 1e-15)/9801.0) as f64 * pi::ramanujan(number, 0)))
    // println!("{}", others::fac(number))
}