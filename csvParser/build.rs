use std::fs;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    fs::copy("users.csv", format!("{}/users.csv", out_dir)).unwrap();
}
