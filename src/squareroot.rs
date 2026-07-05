pub fn root_of_two(n: i32) -> f64 {
    // don't forget to subtract 1
    if n == 0 {
        return 2.0;
    }
    return 2.0 + (1.0 / root_of_two(n-1))
}


pub fn squareroot(n: f64, epsilon: f64) -> f64 {
    // dichotomy
    let mut a = 0.0;
    let mut b = n;

    while (b-a) > epsilon {
        let m = (a+b) / 2.0;
        if m * m < n {
            a = m;
        } else {
            b = m;
        }
    }

    (a+b) / 2.0
}
