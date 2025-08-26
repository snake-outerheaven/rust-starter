mod utils;
use std::{thread::sleep, time::Duration};
use utils::limpar_tela;

fn main() {
    println!("Oi! Eu sou um Rustáceo!");
    sleep(Duration::from_millis(750));
    limpar_tela();
    sleep(Duration::from_millis(750));
    println!("Ei! Eu continuo sendo um Rustáceo!");
}
