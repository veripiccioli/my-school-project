// A simple calculator program using Rust
use std::io;

fn main() {
    println!("Welcome to the Calculator Program!");

    // Get input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input and perform the calculation
    let num1: f64 = input[..].parse().unwrap();
    let num2: f64 = input[..].parse().unwrap();
    let sum: f64 = num1 + num2;

    println!("{} + {} = {}", num1, num2, sum);
}
