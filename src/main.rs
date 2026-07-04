use std::io;
use std::f64::consts::LN_2;


fn fac(n: i32) -> i32 {
    // The factorial function
    if n == 0 {
        return 1;
    } else {
        return n * fac(n-1);
    }
}

fn taylor_ln(a: f64, n: u32) -> f64 {
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

fn newton_ln(a: f64, n:u32) -> f64 {
    // Newton's method to estimate the ln function
    let x0: f64 = a.log2() * LN_2;
    let mut x = x0;
    for _ in 0..n {
        x = x - 1.0 + a / x.exp();
    }

    x
}

fn ln(a: f64, n:i32) -> f64 {
    // Logarithm using the arctanh series
    let y = (a-1.0) / (a+1.0);
    let mut sum = 0.0;

    for k in 0..=n {
        let p = 2 * k + 1;
        sum += y.powi(p as i32) / p as f64;
    }
    
    2.0 * sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let number: f64 = input.trim().parse().unwrap();
    println!("{}", taylor_ln(number-1.0, 20));

}