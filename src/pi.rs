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