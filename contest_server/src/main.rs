fn main() {

    contest_server::submission1::run();
    contest_server::submission2::run();
    contest_server::submission3::run();
    
    // submission4::run(); // This one is interactive so better not run every time.

    println!("And finally: {}.", contest_server::submission1::fib(42));
}
