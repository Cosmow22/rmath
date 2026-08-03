// use std::io;
use std::f64::consts::PI;

mod others;
mod trigo;
use trigo::taylor_sin;

fn main() {
    let x = 60.0 * PI / 180.0;
    println!("{}", taylor_sin(10, 0, x));
}