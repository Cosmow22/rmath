use std::f64::consts::LN_2;


pub fn taylor_ln(a: f64, n: u32) -> f64 {
    // Gives an estimate of the ln function
    // using Taylor series
    // for numbers between -1 and 1  
    // 
    // # How to use :
    //  taylor_ln(number-1.0, 20)
    
    let mut sum = 0.0;

    for k in 1..=n {
        let term = a.powi(k as i32) / k as f64;

        if k % 2 == 0 {
            sum -= term;
        } else {
            sum += term;
        }
    }

    sum
}

pub fn newton_ln(a: f64, n:u32) -> f64 {
    // Newton's method to estimate the ln function
    let x0: f64 = a.log2() * LN_2;
    let mut x = x0;
    for _ in 0..n {
        x = x - 1.0 + a / x.exp();
    }

    x
}

pub fn ln(a: f64, n:i32) -> f64 {
    // Logarithm using the arctanh series
    let y = (a-1.0) / (a+1.0);
    let mut sum = 0.0;

    for k in 0..=n {
        let p = 2 * k + 1;
        sum += y.powi(p as i32) / p as f64;
    }
    
    2.0 * sum
}