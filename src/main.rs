use std::io;

// mod squareroot;
// use squareroot::squareroot;

mod others;
use others::fib;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let number: i32 = input.trim().parse().unwrap();
    for n in fib(number).iter() {
        println!("{}", n)
    }

}