use rug::Integer;


pub fn fac(n: u32) -> Integer {
    // The factorial function
    if n == 0 {
        return Integer::from(1);
    } else {
        return Integer::from(n) * fac(n-1);
    }
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