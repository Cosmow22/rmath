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

pub fn conway(n: i32) -> String {
    // the Conway sequence
    let mut u = String::from("1");
    for _ in 0..n {
        let mut next_u = String::from("");
        let mut i = 0;
        let bytes = u.as_bytes();
        while i < bytes.len() {
            let mut count = 1;
            while i + 1 < bytes.len() && bytes[i] == bytes[i + 1] {
                i += 1;
                count += 1;
            }

            next_u.push_str(&count.to_string());
            next_u.push(bytes[i] as char);
            
            i += 1;
        }
        u = next_u;
    }
    u
}