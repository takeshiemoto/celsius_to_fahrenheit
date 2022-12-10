extern crate core;
mod foo;

use std::io;

fn main() {
    // println!("Enter fahrenheit");
    //
    // let mut fahrenheit = String::new();
    // io::stdin()
    //     .read_line(&mut fahrenheit)
    //     .expect("Failed to read line");
    //
    // let fahrenheit: i32 = match fahrenheit.trim().parse() {
    //     Ok(f) => f,
    //     Err(_) => panic!("Failed to parse"),
    // };
    //
    // foo::bar::celsius_to_fahrenheit(fahrenheit);

    foo::bar::run();
}
