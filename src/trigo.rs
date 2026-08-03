use std::io;
use std::f64::consts::PI;

use crate::others::fac;


pub fn cos_complementary(x: f64) -> f64 {
    let x = x * PI / 180.0;
    println!("{} rad", x);
    let (cos, sin) = cordic(30, PI/2.0 - x);
    return sin;
}

pub fn maclaurin_sin(n: u32, k: u32, x: f64) -> f64 {
    // let x = 60.0 * PI / 180.0;
    // println!("{}", maclaurin_sin(10, 0, x));
    let exp = 2 * k + 1;
    let term = (-1.0f64).powi(k as i32) * x.powi(exp as i32) / fac(exp).to_f64();
    if n == 0 {
        return term
    }
    return term + maclaurin_sin(n-1, k+1, x)
}

pub fn maclaurin_cos(n:u32, k:u32, x: f64) -> f64 {
    // let x = 60.0 * PI / 180.0;
    // println!("{}", taylor_cos(10, 0, x));
    let exp = 2 * k;
    let term = (-1.0f64).powi(k as i32) * x.powi(exp as i32) / fac(exp).to_f64();
    if n == 0 {
        return term
    }
    return term + maclaurin_cos(n-1, k+1, x)
}


fn reduce(mut angle_rad: f64) -> (f64, i8, i8) {

    let mut cos_sign: i8 = 1;
    let mut sin_sign: i8 = 1;

    while angle_rad > PI {
        angle_rad -= 2.0*PI;  
    }
    while angle_rad < -PI {
        angle_rad += 2.0*PI;
    }
    
    if angle_rad  > PI / 2.0 {
        angle_rad = PI - angle_rad;
        cos_sign = -1;   
    }
    
    if angle_rad < -PI / 2.0 {
        angle_rad += PI;
        cos_sign = -1;   
        sin_sign = -1;   
    }
    
    return (angle_rad, cos_sign, sin_sign)
}

pub fn cordic(n: i32, angle_rad:f64) -> (f64, f64) {
    
    let (reduced_angle_rad, cos_sign, sin_sign) = reduce(angle_rad);
    println!("reduced angle : {}", reduced_angle_rad);
    println!("cos_sign : {}", cos_sign);
    println!("sin_sign : {}", sin_sign);
    
    let K = 0.607252935;
    let mut cos = K;
    let mut sin = 0.0;
    let mut z = reduced_angle_rad;
    let mut d;

    for i in 0..n {
        if z >= 0.0 {
            d = 1.0;
        } else {
            d = -1.0;
        }

        let cos_new = cos - d * sin * 2.0_f64.powi(-i);
        let sin_new = sin + d * cos * 2.0_f64.powi(-i);
        z = z - d * 2.0_f64.powi(-i).atan();

        cos = cos_new;
        sin = sin_new;  
    }

    (cos * cos_sign as f64, sin * sin_sign as f64)
}

pub fn run_cordic() {
    println!("Enter n and x (space separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let mut parts = input.split_whitespace();

    let n: i32 = parts.next().unwrap().parse().unwrap();
    let mut angle: f64 = parts.next().unwrap().parse().unwrap();

    angle = angle * PI / 180.0;
    let (cos, sin) = cordic(n, angle);
    println!("cos : {}", cos);
    println!("sin : {}", sin);
}