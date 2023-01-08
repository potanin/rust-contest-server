pub fn run() {
    println!("Running submission 1. Slow fib of 10 is {}.", fib(10));
}

pub fn fib(n: i128) -> i128 {
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}