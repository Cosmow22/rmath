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

use crate::others::fac;

pub fn ramanujan(n: u32, k: u32) -> f64{
    let num = fac((4 * k) as u32) as f64 * (1103 + 26390*k) as f64;

    let den = (fac(k as u32) as f64).powi(4) * 396.0_f64.powf((4 * k) as f64);

    if n == 0 {
        return num / den
    }
    return num / den + ramanujan(n-1, k+1)
}   