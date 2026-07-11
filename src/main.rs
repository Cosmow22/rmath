use std::io;

// mod squareroot;
// use squareroot::squareroot;

// mod sequences;
// use sequences::conway;

mod pi;


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let number: i32 = input.trim().parse().unwrap();
    println!("{}", 4.0 / (pi::brouncker(number, 1.0)-1.0))
}