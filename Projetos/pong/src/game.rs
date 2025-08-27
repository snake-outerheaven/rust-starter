use crate::arena::Arena;

/// estrutura que representa o jogo.

pub struct Game {
    pub name: String,
    pub arena: Arena,
    pub game_time: u32,
    pub n_of_games: u32,
}
