/*
* Como construir uma arena?
* Simples, uma arena é apenas um conjunto de coordenadas limites.
*/
const ARENA_MAX_WIDTH: f32 = 40.0;
const ARENA_MAX_LENGHT: f32 = 30.0;

/*
* Uma raquete possui posição fixa na arena e dimensões definidas
*
*/

/// struct que representa a raquete
struct Paddle {
    x: f32,
    y: f32,
    height: f32,
    vy: f32,
}

/// métodos da raquete
impl Paddle {
    pub fn new(posx: f32) -> Self {
        let y: f32 = ARENA_MAX_LENGHT / 2.0;
        Self {
            x: posx,
            y: y,
            height: 1.5,
            vy: 0.0,
        }
    }
}

/// struct que representa a bola do jogo
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

/// métodos da bola
impl Ball {
    pub fn new() -> Self {
        Self {
            x: ARENA_MAX_WIDTH / 2.0,
            y: ARENA_MAX_LENGHT / 2.0,
            vx: rand::random_range(3.0..=5.0),
            vy: rand::random_range(3.0..=5.0),
        }
    }

    pub fn mover(&mut self, dt: f32) {
        // posição
        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }

    pub fn quicar(&mut self, p1: &Paddle, p2: &Paddle) {
        // se a bola tocar nos limites superiores
        if self.y == ARENA_MAX_LENGHT - 1.0 || self.y == 0.0 {
            self.vy = -self.vy;
        }

        // se a bola tocar em qualquer parte do corpo de p1
        if self.x == p1.x && (p1.y - p1.height..=p1.y + p1.height).contains(&self.y) {
            self.vx = -self.vx;
        }
        // se a bola tocar em qualquer parte do corpo de p2
        if self.x == p2.x && (p2.y - p2.height..=p2.y + p2.height).contains(&self.y) {
            self.vx = -self.vx;
        }
    }
}

/// estrutura que representa o jogo.
struct Game {
    ball: Ball,
    p1: Paddle,
    p2: Paddle,
    score1: u8, // pontuação do jogador 1
    score2: u8, // pontuação do jogador 2
}

/// métodos do jogo
impl Game {
    pub fn new() -> Self {
        Self {
            ball: Ball::new(),
            p1: Paddle::new(2.0),
            p2: Paddle::new(38.0),
            score1: 0,
            score2: 0,
        }
    }
}

fn main() {}
