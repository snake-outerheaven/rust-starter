use std::{
    io::{self, Write},
    process::Command,
    thread::sleep,
    time::Duration,
};

use rand::prelude::*;

/// função dedicada
pub fn limpar_tela() {
    let cmd: &str = if cfg!(windows) { "cls" } else { "clear" };
    // caso houvesse outros sistemas operacionais, seria um else if cfg!(unix)
    // para linux, mac e outros sistemas baseados em unix.

    if Command::new(cmd).status().is_err() {
        print!("\x1B[2J\x1B[H");
        if io::stdout().flush().is_err() {
            eprintln!("Falha ao limpar tela, com ANSI escape...");
        }
    }
}

pub fn wait(ms: u64) {
    sleep(Duration::from_millis(ms));
}

pub fn xy_generator(width: u32, height: u32) -> (u32, u32) {
    let mut rng = rand::rng();

    let objx = rng.random_range(0..=width);
    let objy = rng.random_range(0..=height);

    (objx, objy)
}
