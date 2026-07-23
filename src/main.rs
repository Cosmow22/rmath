use std::io;

mod squareroot;
mod pi;
mod others;

use pi::ramanujan;
use squareroot::root_of_two;
use rug::Float;


fn main() {
    println!("Enter n and precision (space separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let mut parts = input.split_whitespace();

    let n: u32 = parts.next().unwrap().parse().unwrap();
    let prec: u32 = (parts.next().unwrap().parse::<f64>().unwrap() * 3.32 ) as u32;
    //println!("{}", Float::with_val(prec, 1.0 /  ((2.0*squareroot(2.0, n)/9801.0) as f64 * ramanujan(n, 0, prec))));
    println!("{}", Float::with_val(prec, 1 /  (2*(root_of_two(n, prec)-1)/9801 * ramanujan(n, 0, prec))));
}