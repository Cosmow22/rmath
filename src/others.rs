pub fn fac(n: i32) -> i32 {
    // The factorial function
    if n == 0 {
        return 1;
    } else {
        return n * fac(n-1);
    }
}
