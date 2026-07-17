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