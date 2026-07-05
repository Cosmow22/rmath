use std::io;

// mod squareroot;
// use squareroot::squareroot;

mod others;
use others::conway;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let number: i32 = input.trim().parse().unwrap();
    println!("{}", conway(number))

}