pub fn fac(n: i32) -> i32 {
    // The factorial function
    if n == 0 {
        return 1;
    } else {
        return n * fac(n-1);
    }
}

pub fn fib(n: i32) -> Vec<i32> {
    // the Fibonacci sequence
    let mut a = 0;
    let mut b = 1;

    let mut result = Vec::new();

    for _ in 0..n {
        (a, b) = (b, a + b);
        result.push(a);
    }

    result
}

pub fn dividers(n: i32) -> Vec<i32> {
    //for div in dividers(number).iter() {
    //println!("{}", div)
    //}
    let mut result = Vec::new();
    let mut i = 1;

    while i*i < n {
        if n % i == 0 {
            result.push(i);
            result.push(n/i)
        }
        i += 1;
    }

    result
}