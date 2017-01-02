fn main() {
    let x = fib(90) as f64;
    let y = fib(91) as f64; // Beyond 91 gives an overflow!
    println!("phi: {:.20}", y / x);
}

fn fib(n: i32) -> i64 {
    let mut a = 0;
    let mut b = 1;
    let mut sum = 0;

    for i in 0..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    sum
}
