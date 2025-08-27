use crate::utils::Direcao;

/// estrutura que representa a raquete do jogador
pub struct Racket {
    pub x: u32,
    pub y: u32,
    pub height: u32,
    pub body: char,
}

impl Racket {
    /// construtor da raquete
    pub fn new(posx: u32, posy: u32) -> Self {
        Self {
            x: posx,
            y: posy,
            height: 3,
            body: '|',
        }
    }

    /// método para o "deslizar" da raquete.
    pub fn slide(&mut self, dir: Direcao) {
        match dir {
            Direcao::Up => {
                self.y += 2;
            }
            Direcao::Down => {
                self.y -= 2;
            }
        }
    }
}
