// goto line 50 to continue
use macroquad::{
    math::Vec2,
    window::{screen_height, screen_width},
};
use rand::{Rng, rngs::ThreadRng, thread_rng};

/// constante que representa o limite lateral do mundo do jogo
const MAX_ARENA_WIDTH: f32 = 100.0;

/// constante que representa o limite vertical do mundo do jogo
const MAX_ARENA_HEIGHT: f32 = 65.0;

/// constante que controla a velocidade máxima da cobra
const MAX_SNAKE_SPEED: f32 = 8.1;

/// struct que representa uma maçã no jogo.

struct Apple {
    pos: Vec2,
    apple_type: AppleType,
}

/// enum que serve para criar tipos de maçãs diferentes que oferecem diferentes buffs
enum AppleType {
    RED,
    BLUE,
    YELLOW,
}

/// métodos da maçã.

impl Apple {
    /// construtor da maçã
    pub fn new() -> Self {
        let mut rng: ThreadRng = thread_rng();
        let posx: f32 = rng.gen_range(0.0..=(MAX_ARENA_WIDTH - 1.0));
        let posy: f32 = rng.gen_range(0.0..=(MAX_ARENA_HEIGHT - 1.0));
        let apple_type = match rng.gen_range(0..3) {
            0 => AppleType::RED,
            1 => AppleType::BLUE,
            2 => AppleType::YELLOW,
            _ => unreachable!(),
        };

        Self {
            pos: Vec2::new(posx, posy),
            apple_type,
        }
    }
    // bolar mais funcionalidades para a maçã.
}

/// struct que representa a cobrinha do jogo

struct Snake {
    head: Vec2,
    body: Vec<Vec2>,
    tail: Vec2,
    speed: Vec2,
}

/// struct que representa o jogo em si.

struct Game {
    snake: Snake,
    apple: Apple,
    score: u8,
    lives: u8,
}

/// Converte coordenadas da arena para pixels da tela
fn to_screen_x(x: f32) -> f32 {
    x / MAX_ARENA_WIDTH * screen_width()
}
fn to_screen_y(y: f32) -> f32 {
    y / MAX_ARENA_HEIGHT * screen_height()
}

/// Função principal, que roda a janela do jogo.
#[macroquad::main("Snake")]
async fn main() {}
