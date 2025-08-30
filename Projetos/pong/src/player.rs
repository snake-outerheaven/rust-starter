use crate::{
    racket::Racket,
    utils::{Direcao, limpar_tela, wait},
};
use std::io::stdin;

/// estrutura que representa o jogador

pub struct Player {
    pub nome: String,
    pub score: u32,
    pub racket: Racket,
}

impl Player {
    /// construtor do jogador
    pub fn new(p_nmb: u8, raquete: Racket) -> Self {
        let mut name: String = String::new(); // inicializando o nome como uma String
        let mut resposta: String = String::with_capacity(3); // criando o nome como usuário
        wait(440);
        // inicio do loop
        loop {
            name.clear();
            resposta.clear();
            println!("Bem vindo jogador {p_nmb}, por favor, digite o seu nome abaixo");
            stdin()
                .read_line(&mut name)
                .expect("Não foi possível obter o nome do jogador, estranho.");
            name = name.trim().to_string();
            println!("Voce confirma o nome '{name}'? (S/N)");
            stdin()
                .read_line(&mut resposta)
                .expect("Não foi possível ler resposta do jogador, estranho");

            match resposta.trim() {
                "S" => {
                    wait(550);
                    println!("Ok, nome '{name}' confirmado!");
                    break;
                }

                "N" => {
                    wait(550);
                    println!("Certo, aguarde para digitar o seu nome..");
                    limpar_tela();
                    continue;
                }

                _ => {
                    wait(550);
                    println!(
                        "Por favor, digite S ou N, digite o seu nome novamente por causas de segurança."
                    );
                    limpar_tela();
                    continue;
                }
            }
        }

        Self {
            nome: name,
            score: 0,
            racket: raquete,
        }
    }

    pub fn move_racket(&mut self, dir: Direcao) {
        self.racket.slide(dir);
    }
}
