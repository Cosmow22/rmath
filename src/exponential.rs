use crate::others::fac;


pub fn maclaurin_exp(n: u32, k: u32, x: f64) -> f64 {
    let term = x.powi(k as i32) / fac(k).to_f64();
    if n == 0 {
        return term
    }
    return term + maclaurin_exp(n-1, k+1, x)
}
