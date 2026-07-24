use std::{
    env::{current_dir, set_current_dir, var},
    io::{Write, stdin, stdout},
    process::Command,
};

use gethostname::gethostname;

const MAX_SHELL_TRIES: u8 = 10;

struct Parser;

impl Parser {
    fn parse(&self, input: &str) -> Vec<String> {
        input.split_whitespace().map(String::from).collect()
    }
}

struct Runner; // can be expanded to many stuff, such as registry for builtins with Hashmap and other stuff

enum ShellStatus {
    Ok,
    EmptyInput,
    CommandFailed,
    Dead, // if there is a catastrophic failure, marks the shell as dead and redo the loop process until constant is reached
}

impl Runner {
    fn run_parsed_input(&self, tokens: &[String], home: &str) -> ShellStatus {
        if tokens.is_empty() {
            return ShellStatus::EmptyInput;
        }
        match tokens[0].as_str() {
            "cd" => {
                let path: &str = tokens.get(1).map(|s| s.as_str()).unwrap_or(home);
                if let Err(e) = set_current_dir(path) {
                    eprintln!("cd: {e}");
                    ShellStatus::CommandFailed
                } else {
                    ShellStatus::Ok
                }
            }
            "pwd" => {
                if let Some(pwd) = current_dir().expect("f rush").to_str() {
                    println!("{}", pwd);
                    ShellStatus::Ok
                } else {
                    eprintln!("rush: couldnt show pwd, what have you done?");
                    ShellStatus::CommandFailed
                }
            }
            "exit" => ShellStatus::Dead,
            "sudo" => {
                eprintln!("rush: sudo support is not implemented yet.");
                ShellStatus::Ok
            }
            _ => {
                let cmd = tokens[0].as_str();
                let args = tokens[1..].iter().map(|s| s.as_str());
                if let Err(e) = Command::new(cmd).args(args).status() {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        eprintln!("rush: {} not found", cmd);
                    } else {
                        eprintln!("rush: {}: {}", cmd, e);
                    }
                    ShellStatus::CommandFailed
                } else {
                    ShellStatus::Ok
                }
            }
        }
    }
}

struct Shell {
    prompt: String,
    parser: Parser,
    runner: Runner,
    status: ShellStatus,
}

impl Shell {
    fn new() -> Self {
        let user: String = var("USER").unwrap_or_else(|_| "user".to_string());
        let hostname: String = gethostname().to_string_lossy().to_string();
        let pwd: String = current_dir().unwrap().display().to_string();
        let prompt: String = format!("{}@{} {}! ", user, hostname, pwd);

        Self {
            prompt,
            parser: Parser,
            runner: Runner,
            status: ShellStatus::Ok,
        }
    }
    fn update(&mut self) {
        let user: String = var("USER").unwrap_or_else(|_| "user".to_string());
        let hostname: String = gethostname().to_string_lossy().to_string();
        let pwd: String = current_dir().unwrap().display().to_string();
        self.prompt = format!("{}@{} {}! ", user, hostname, pwd);
    }
    fn printprompt(&self) {
        print!("{}", self.prompt);
        stdout().flush().expect("f rush.");
    }
}

fn main() {
    let home: String = var("HOME").expect("Couldnt get home variable");
    let mut main_fail_counter: u8 = 0;
    let mut sh: Shell = Shell::new();
    let mut input: String = String::new();
    loop {
        if !input.is_empty() {
            input.clear();
        }
        if main_fail_counter == MAX_SHELL_TRIES {
            break;
        }
        sh.update();
        sh.printprompt();
        stdin().read_line(&mut input).expect("Rush is out of hush!");

        let tokens: Vec<String> = sh.parser.parse(input.as_str());

        if tokens.is_empty() {
            continue;
        }
        sh.status = sh.runner.run_parsed_input(&tokens, &home);

        match sh.status {
            ShellStatus::Ok => continue,
            ShellStatus::EmptyInput => continue,
            ShellStatus::CommandFailed => {
                main_fail_counter += 1;
                continue;
            }
            ShellStatus::Dead => break,
        }
    }
}
