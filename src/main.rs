use std::io;

// mod others;
// mod pi;
mod squareroot; 

use squareroot::root_of_two;

fn main() {
    println!("Enter n and precision (space separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let mut parts = input.split_whitespace();

    let n: u32 = parts.next().unwrap().parse().unwrap();
    let prec: u32 = (parts.next().unwrap().parse::<f64>().unwrap() * 3.32 ) as u32;
    //let number: u32 = input.trim().parse().unwrap();
    // println!("{}", 1.0 /  ((2.0*squareroot(2.0, 1e-15)/9801.0) as f64 * pi::ramanujan(number, 0)))
    println!("{}", root_of_two(n, prec));
}