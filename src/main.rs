use std::io;

// mod squareroot;
// use squareroot::squareroot;

mod sequences;
use sequences::conway;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let number: i32 = input.trim().parse().unwrap();
    conway(number);
    // for u_n in collatz(number).iter() {
    //     println!("{}", u_n)
    // }

}