// use std::io;
use std::f64::consts::PI;

mod others;
mod trigo;
use trigo::taylor_cos;

fn main() {
    let x = 149.0 * PI / 180.0;
    println!("{}", taylor_cos(10, 0, x));
}