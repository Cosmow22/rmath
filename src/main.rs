use std::io;

// mod squareroot;
// use squareroot::squareroot;

mod others;
use others::dividers;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    for div in dividers(number).iter() {
        println!("{}", div)
    }

}