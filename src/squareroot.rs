use rug::Float;


pub fn root_of_two(n: u32, prec: u32) -> Float {
    // don't forget to subtract 1
    if n == 0 {
        return Float::with_val(prec, 2);
    }
    return Float::with_val(prec, 2 + (1 / root_of_two(n-1, prec)))
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
