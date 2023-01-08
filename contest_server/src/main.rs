fn main() {

    contest_server::submission1::run();
    contest_server::submission2::run();
    contest_server::submission3::run();
    
    // submission4::run(); // Submission 4 uses "std::io" - how can we stop it???

    println!("And finally: {}.", contest_server::submission1::fib(42));
}
