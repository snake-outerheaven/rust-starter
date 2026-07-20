use std::{
    process::{Command, exit},
    thread::sleep,
    time::Duration,
};

pub fn wait_ms(tm: u64) {
    sleep(Duration::from_millis(tm))
}

pub fn clear() {
    if cfg!(target_os = "windows") {
        if let Err(e) = Command::new("cmd.exe").args(["/c", "cls"]).status() {
            eprintln!("Não foi possível limpar a tela!");
            wait_ms(250);
            eprintln!("error message: {}", e);
            exit(1);
        }
    } else {
        if let Err(e) = Command::new("clear").status() {
            eprintln!("Não foi possível limpar a tela!");
            wait_ms(250);
            eprintln!("error message{}", e);
            exit(1);
        }
    }
}
