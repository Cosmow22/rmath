use crate::others::fac;
use rug::Float;
use rug::ops::Pow;


pub fn brouncker(n: i32, k: f64) -> f64 {
    // Estimate pi with Brouncker continued fraction.
    // 
    // How to use :
    // println!("{}", 4.0 / (pi::brouncker(number, 1.0)-1.0))
    if n == 0 {
        return 2.0;
    }
    return 2.0 + k*k / brouncker(n-1, k+2.0)   
}

pub fn liebniz(n: i32, k: i32) -> f64 {
    // Estimate pi with the Liebniz forumula.
    // Multiply by 4.0 at the end.
    if n == 0 {
        return (-1.0f64).powi(k) / (2*k + 1) as f64;
    }
    return (-1.0f64).powi(k) / (2*k + 1) as f64 + liebniz(n-1, k+1)
}

pub fn ramanujan(n: u32, k: u32, prec: u32) -> Float {
    // Estimate pi with the Ramanujan forumula.
    // 
    // How to use :
    // println!("{}", Float::with_val(prec, 1 /  (2*Float::with_val(prec, 2).sqrt() / 9801  *  ramanujan(n, 0, prec))));
    // adds 8 new decimal places to each new term
    // so, divide the number of digits you want by 8 to get the number of terms to use
    // for example, to get 1000 digits of pi, use 1000/8 = 125 terms

    let num = Float::with_val(prec, fac((4 * k) as u32))
        * Float::with_val(prec, 1103 + 26390 * k);

    let den = Float::with_val(prec, fac(k as u32))
        .pow(4)
        * Float::with_val(prec, 396).pow(4 * k);

    if n == 0 {
        return num / den
    }
    return num / den + ramanujan(n-1, k+1, prec)
}   