extern crate core;

use std::io;

fn main() {
    println!("Enter fahrenheit");

    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: i32 = match fahrenheit.trim().parse() {
        Ok(f) => f,
        Err(_) => panic!("Failed to parse"),
    };

    let result = celsius_to_fahrenheit(fahrenheit);
    println!("Celsius is {}°C", result);
}

fn celsius_to_fahrenheit(c: i32) -> f32 {
    let base = 10.0;
    let result = (c - 32) as f32 / 1.8;
    (result * base).floor() / base
}
