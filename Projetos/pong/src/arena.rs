use crate::{
    ball::{self, Ball},
    player::{self, Player},
};

/// estrutura que representa a arena
pub struct Arena {
    pub width: u32,
    pub height: u32,
    pub p1: Player,
    pub p2: Player,
    pub ball: Ball,
}
