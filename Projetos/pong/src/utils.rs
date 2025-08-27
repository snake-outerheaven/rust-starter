use rand::prelude::*;
use std::{
    io::{self, Write},
    process::Command,
    thread::sleep,
    time::Duration,
};

// um simples lembrete, este arquivo contém utilitários, então tudo deve ter pub na frente.

// area das enumeraçoes que podem ser úteis.
pub enum Direcao {
    Up,
    Down,
}

// area das structs que podem ser úteis.

// área dos métodos das structs

// area das funções
/// utilitário que busca limpar a tela da forma mais portátil possível.
pub fn limpar_tela() {
    let cmd: &str = if cfg!(windows) { "cls" } else { "clear" };
    // caso houvesse outros sistemas operacionais, seria um else if cfg!(unix)
    // para linux, mac e outros sistemas baseados em unix.

    if Command::new(cmd).status().is_err() {
        print!("\x1B[2J\x1B[H");
        if io::stdout().flush().is_err() {
            eprintln!("Falha ao limpar tela com ANSI escape...");
        }
    }
}
/// sim, apenas para facilitar o desenvolvimento de software
pub fn wait(ms: u64) {
    sleep(Duration::from_millis(ms));
}

pub fn rand_generator(bot: i32, top: i32) -> i32 {
    let mut rngesus = rand::rng();

    let out: i32 = rngesus.random_range(bot..=top);

    out
}
