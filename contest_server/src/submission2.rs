pub fn run() {
    println!("Running submission 2. Fast fib of 10 is {}.", fib(10));
}

fn fib(n: i128) -> i128 {
    if n < 3 {
        1
    } else {
        let mut i: i128 = 3;
        let mut n1: i128 = 1;
        let mut n2: i128 = 1;
        while i <= n {
            let temp = n1;
            n1 = n2;
            n2 = n1 + temp;
            i += 1;
        };
        n2
    }
}