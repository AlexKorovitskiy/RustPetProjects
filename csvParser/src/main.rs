use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    position: Positions,
}

#[derive(Debug)]
enum Positions {
    Dev,
    Qa,
    DevOps,
    Lead,
}

impl FromStr for Positions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "dev" => Ok(Positions::Dev),
            "qa" => Ok(Positions::Qa),
            "devops" => Ok(Positions::DevOps),
            "lead" => Ok(Positions::Lead),
            _ => Err(format!("Unknown Operation: {}", s)),
        }
    }
}

fn main() {
    let path = "users.csv";

    let content = fs::read_to_string(path).unwrap();

    println!("Content:");
    println!("{}", content);
    let mut users: Vec<User> = Vec::new();

    for line in content.lines() {
        let line_parts: Vec<&str> = line.split(',').collect();
        if line_parts.len() != 3 {
            println!("Incorrect data");
            return;
        }

        let last_item: &str = &line_parts[2][..line_parts[2].len() - 1];
        users.push(User {
            name: line_parts[0].trim().to_string(),
            age: match line_parts[1].trim().parse::<i32>() {
                Ok(value) => value,
                Err(message) => panic!("Not able to parse. {}", message),
            },
            position: match last_item.parse::<Positions>() {
                Ok(value) => value,
                Err(message) => panic!("Error: {message}"),
            },
        });
    }

    println!("Users:");
    for user in &users {
        println!("{:?}", user);
    }
}
