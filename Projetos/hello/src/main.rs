use std::process::Command;

fn main() {
    Command::new("clear").status().unwrap();
    println!("Hello, world!");
}
