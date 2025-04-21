use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut rng = rand::rng();

    println!("Random value is generated from:");
    io::stdout().flush();
    let from: i32 = read_number();

    println!("Random value is generated to:");
    io::stdout().flush();
    let to: i32 = read_number();

    let rndm: i32 = rng.random_range(from..to);

    println!("Random number is {rndm}");
}

fn read_number() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Not able to read input string");

    let input: i32 = input.trim().parse().expect("Not able to parse");

    input
}
