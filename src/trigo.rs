use std::f64::consts::PI;


pub fn cordic(n: i32, angle:f64) {
    let K = 0.607252935;
    let mut cos = K;
    let mut sin = 0.0;
    let mut z = angle * PI / 180.0;
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

    println!("cos({}) = {}", angle, cos);
    println!("sin({}) = {}", angle, sin);
}