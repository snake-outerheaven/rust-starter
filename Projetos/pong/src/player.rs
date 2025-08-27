use crate::racket::Racket;

/// estrutura que representa o jogador

pub struct Player {
    pub nome: String,
    pub placar: u32,
    pub racket: Racket,
}
