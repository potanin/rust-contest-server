mod submission1;
mod submission2;
mod submission3;

#[allow(dead_code)]
mod submission4;

fn main() {

    submission1::run();
    submission2::run();
    submission3::run();
    
    // submission4::run(); // This one is interactive so better not run every time.

    println!("And finally: {}.", submission1::fib(42));
}
