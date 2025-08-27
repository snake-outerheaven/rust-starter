use crate::{arena::Arena, player::Player, racket::Racket, utils::rand_generator};

/// estrutura pública que representa a bola do jogo
pub struct Ball {
    pub x: u32,
    pub y: u32,
    pub xvel: i32,
    pub yvel: i32,
    pub body: char,
}

// métodos (funções associadas) à bola.
impl Ball {
    /// "construtor" da bola.
    pub fn new(arena_width: u32, arena_height: u32) -> Self {
        Self {
            // logicamente a arena do jogo deve ter lados pares
            x: arena_width / 2,
            y: arena_height / 2,
            xvel: rand_generator(5, 10),
            yvel: rand_generator(5, 10),
            body: 'O',
        }
    }

    /// método que busca representar o movimento da bola
    pub fn rolling(&mut self) {
        self.x = (self.x as i32 + self.xvel) as u32;
        self.y = (self.y as i32 + self.yvel) as u32;
    }

    /// método que faz a bola quicar se interagir com as paredes superiores
    /// e raquete do jogador.
    pub fn check_if_collide(&mut self, env: &Arena, p1: &mut Player, p2: &mut Player) -> bool {
        let mut collide: bool = false;
        // se bater na parede superior ou inferior da arena.
        if self.y == 0 || self.y == env.height {
            self.yvel = -self.yvel;
            collide = true;
        }

        // se colidir com a raquete do jogador 1
        if self.x == p1.racket.x && (p1.racket.y..=p1.racket.height).contains(&self.y) {
            let racket_center: u32 = p1.racket.y + p1.racket.height / 2;
            let dist = self.y as i32 - racket_center as i32;
            let dist_normalizada = dist as f32 / (p1.racket.height as f32 / 2.0);
            self.xvel = -self.xvel;
            self.yvel = (self.yvel as f32 * dist_normalizada) as i32;
            collide = true;
        }
        // Colisão com a raquete do jogador 2
        if self.x == p2.racket.x && (p2.racket.y..p2.racket.y + p2.racket.height).contains(&self.y)
        {
            let racket_center = p2.racket.y + p2.racket.height / 2;
            let dist = self.y as i32 - racket_center as i32;
            let dist_normalizada = dist as f32 / (p2.racket.height as f32 / 2.0);

            self.xvel = -self.xvel; // inverte direção horizontal
            self.yvel = (self.yvel as f32 * dist_normalizada) as i32; // ajusta vertical
            collide = true;
        }

        collide
    }
}
