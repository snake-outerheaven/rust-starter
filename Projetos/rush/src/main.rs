use std::{
    env::{current_dir, set_current_dir, var},
    io::{Write, stdin, stdout},
    process::Command,
};

use gethostname::gethostname;

fn main() {
    loop {
        let user: String = var("USER").unwrap_or_else(|_| "user".to_string());
        let home: String = var("HOME").expect("Couldnt get home variable");
        let hostname: String = gethostname().to_string_lossy().to_string();
        let cwd: String = current_dir().unwrap().display().to_string();
        let prompt: String = format!("{}@{}: {}! ", user, hostname, cwd);
        let mut input: String = String::new();
        print!("{}", &prompt);
        stdout().flush().expect("f rush.");
        stdin().read_line(&mut input).expect("f rsh");

        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        match tokens[0] {
            "exit" => break,
            "cd" => {
                let path: &str = tokens.get(1).map(|s| *s).unwrap_or(&home);
                if let Err(e) = set_current_dir(&path) {
                    eprintln!("cd: {}", e);
                }
            }
            cmd => {
                Command::new(cmd).args(&tokens[1..]).status().ok();
            }
        }
    }
}
