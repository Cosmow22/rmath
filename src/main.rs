use std::io;
use std::f64::consts::PI;

// mod squareroot;
// mod pi;
// mod others;

// use pi::ramanujan;
// use squareroot::root_of_two;
// use rug::Float;

mod trigo;
// use trigo::cos_complementary;
use trigo::cordic;

fn main() {
    println!("Enter n and x (space separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let mut parts = input.split_whitespace();

    let n: i32 = parts.next().unwrap().parse().unwrap();
    let mut angle: f64 = parts.next().unwrap().parse().unwrap();
    // let prec: u32 = (parts.next().unwrap().parse::<f64>().unwrap() * 3.32 ) as u32;
    // println!("{}", Float::with_val(prec, 1 /  (2.0*squareroot(2.0, 0.0001)/9801.0 * ramanujan(n, 0, prec))))
    // println!("{}", Float::with_val(prec, 1 /  (2*(root_of_two(n, prec)-1)/9801 * ramanujan(n, 0, prec))));
    // println!("{}", Float::with_val(prec, 1 /  (2*Float::with_val(prec, 2).sqrt() / 9801  *  ramanujan(n, 0, prec))));
    angle = angle * PI / 180.0;
    let (cos, sin) = cordic(n, angle);
    println!("cos : {}", cos);
    println!("sin : {}", sin);
}