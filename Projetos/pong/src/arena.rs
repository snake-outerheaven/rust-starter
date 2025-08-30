use crate::{ball::Ball, player::Player, racket::Racket};

/// estrutura que representa a arena
pub struct Arena {
    pub width: u32,
    pub height: u32,
    pub p1: Player,
    pub p2: Player,
    pub ball: Ball,
}

impl Arena {
    /// construtor da arena
    pub fn new(arena_width: u32, arena_height: u32, p1_racket: Racket, p2_racket: Racket) -> Self {
        Self {
            width: arena_width,
            height: arena_height,
            p1: Player::new(1, p1_racket),
            p2: Player::new(2, p2_racket),
            ball: Ball::new(arena_width, arena_height),
        }
    }
}
