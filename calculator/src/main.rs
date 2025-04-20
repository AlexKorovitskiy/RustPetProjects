use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;

enum Operations {
    Add,
    Multiply,
    Subtract,
    Divide,
}

impl FromStr for Operations {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "add" => Ok(Operations::Add),
            "+" => Ok(Operations::Add),
            "multiply" => Ok(Operations::Multiply),
            "*" => Ok(Operations::Multiply),
            "subtract" => Ok(Operations::Subtract),
            "-" => Ok(Operations::Subtract),
            "divide" => Ok(Operations::Divide),
            "/" => Ok(Operations::Divide),
            _ => Err(format!("Unknown Operation: {}", s)),
        }
    }
}

impl fmt::Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Operations::Add => "add",
            Operations::Subtract => "subtract",
            Operations::Multiply => "multiply",
            Operations::Divide => "divide",
        };

        write!(f, "{}", text)
    }
}

fn main() {
    loop {
        let first_number: f32 = ask_number();
        let operation: Operations = ask_operation();
        let second_number: f32 = ask_number();

        let result = get_result(&first_number, &second_number, &operation);

        match result {
            Ok(value) => {
                println!("{first_number} {operation} {second_number} = {value}");
                break;
            }
            Err(er) => {
                println!("{er}");
            }
        }
    }
}

fn ask_number() -> f32 {
    loop {
        println!("Print number:");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading the string");

        let input = input.trim();

        match input.parse::<f32>() {
            Ok(num) => break num,
            Err(_) => println!("Wrong number"),
        };
    }
}

fn ask_operation() -> Operations {
    loop {
        println!("Print operation:");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error while reading the string");

        let input = input.trim();

        match input.parse::<Operations>() {
            Ok(operation_value) => break operation_value,
            Err(_) => println!("Wrong operation"),
        }
    }
}

fn divide(first: &f32, second: &f32) -> Result<f32, String> {
    if *second == 0.0 {
        Err("Cannot divide by zero.".to_string())
    } else {
        Ok(first / second)
    }
}

fn get_result(
    first_number: &f32,
    second_number: &f32,
    operation: &Operations,
) -> Result<f32, String> {
    match operation {
        Operations::Add => Ok(first_number + second_number),
        Operations::Subtract => Ok(first_number - second_number),
        Operations::Multiply => Ok(first_number * second_number),
        Operations::Divide => divide(&first_number, &second_number),
    }
}
