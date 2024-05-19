use std::io::{self, Write};

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    println!("Enter operation");
    let operation = read_input("> ");

    match operation.as_str() {
        "+" | "-" | "*" | "/" => (),
        _ => panic!("Invalid operation: {}", operation),
    }

    let first_input = read_input("Enter first input: ");
    let second_input = read_input("Enter second input: ");

    let first: f64 = first_input.parse().expect("Invalid value for first input");
    let second: f64 = second_input
        .parse()
        .expect("Invalid value for second input");

    match operation.as_str() {
        "+" => println!("{} + {} = {}", first, second, first + second),
        "-" => println!("{} - {} = {}", first, second, first - second),
        "*" => println!("{} * {} = {}", first, second, first * second),
        "/" => {
            if second != 0.0 {
                println!("{} / {} = {}", first, second, first / second)
            } else {
                panic!("Cannot divide by zero");
            }
        }
        _ => unreachable!(),
    }
}
