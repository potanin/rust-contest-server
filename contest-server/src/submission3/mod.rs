use std::str;

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn run() {
    println!("Running submission 3.");

    let first = first_word("Hello world!");

    println!("First word is: {first}");

    let r = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of {:?} is {}.", r, r.area());

    let s: Rectangle = Rectangle::square(42);

    println!("Can s fit in r? {}", r.can_hold(&s));
}