use std::{
    io::{Write, stdin, stdout},
    process::Command,
};

fn main() {
    loop {
        print!("ferris@rusty$: ");
        stdout().flush().expect("f rsh.");
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("f rsh");

        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "exit" => break,
            cmd => {
                Command::new(cmd).args(&tokens[1..]).status().ok();
            }
        }
    }
}
