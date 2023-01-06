use std::io;
use rand::Rng;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

enum Coin {
    Penny, Nickel, Dime, Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin:: Quarter => 25
    }
}

enum IPAddrKind {
    V4,
    V6
}

pub fn run() {
    println!("Running submission 4.");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Guess the number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        
        match guess.trim().parse::<i32>() {
            Ok(n) =>
                if n == secret {
                    println!("You guessed right!");
                    break;
                } else {
                    println!("You guessed wrong!");
                    if n > secret {
                        println!("Try lower.");
                    } else {
                        println!("Try higher.");
                    }
                },
            Err(e) =>
                println!("You didn't enter a number: {e}"),
        }
    }
}