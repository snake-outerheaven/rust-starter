use macroquad::{
    color::RED,
    input::{is_key_down, KeyCode},
    math::Vec2,
    time::get_frame_time,
    window::{clear_background, next_frame, screen_height, screen_width},
};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::collections::VecDeque;

/// constante que representa o limite lateral do mundo do jogo
const MAX_ARENA_WIDTH: f32 = 100.0;

/// constante que representa o limite vertical do mundo do jogo
const MAX_ARENA_HEIGHT: f32 = 65.0;

/// struct que representa a maçã no jogo.
struct Apple {
    pos: Vec2,
    apple_type: AppleType,
}

/// enum que serve para criar tipos de maçãs diferentes que oferecem diferentes buffs
enum AppleType {
    Red,
    Blue,
    Yellow,
}

/// métodos da maçã.
impl Apple {
    /// construtor da maçã
    pub fn new() -> Self {
        let mut rng: ThreadRng = thread_rng();
        let posx: f32 = rng.gen_range(1.0..=(MAX_ARENA_WIDTH - 1.0));
        let posy: f32 = rng.gen_range(1.0..=(MAX_ARENA_HEIGHT - 1.0));
        let apple_type: AppleType = match rng.gen_range(0..3) {
            0 => AppleType::Red,
            1 => AppleType::Blue,
            2 => AppleType::Yellow,
            _ => unreachable!(),
        };

        Self {
            pos: Vec2::new(posx, posy),
            apple_type,
        }
    }
}

/// struct que representa a cobrinha do jogo
#[derive(Debug)]
struct Snake {
    body: VecDeque<Vec2>,
    speed: Vec2,
    dir: Direction,
    should_grow: bool, // flag para indicar se deve crescer
}

/// enumeração que representa o estado da direção da cobra
#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

/// métodos da cobra
impl Snake {
    /// construtor da cobra
    pub fn new() -> Self {
        let mut rng: ThreadRng = thread_rng();
        let head: Vec2 = Vec2::new(
            rng.gen_range(10.0..=(MAX_ARENA_WIDTH - 10.0)),
            rng.gen_range(10.0..=(MAX_ARENA_HEIGHT - 10.0)),
        );

        let dir: Direction = match rng.gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Right,
            3 => Direction::Left,
            _ => unreachable!(),
        };

        // Calcula offset baseado na direção para posicionar corpo atrás da cabeça
        let offset: Vec2 = match dir {
            Direction::Up => Vec2::new(0.0, 1.0), // corpo abaixo da cabeça
            Direction::Down => Vec2::new(0.0, -1.0), // corpo acima da cabeça
            Direction::Right => Vec2::new(-1.0, 0.0), // corpo à esquerda da cabeça
            Direction::Left => Vec2::new(1.0, 0.0), // corpo à direita da cabeça
        };

        // Cria fila completa: cabeça, corpo, cauda
        let mut body: VecDeque<Vec2> = VecDeque::new();
        body.push_front(head); // cabeça = primeira posição
        body.push_back(head + offset); // primeiro segmento do corpo
        body.push_back(head + offset * 2.0); // cauda = última posição

        Self {
            body,
            speed: Vec2::new(0.0, 0.0),
            dir,
            should_grow: false,
        }
    }

    /// getter da cabeça
    pub fn head(&self) -> Vec2 {
        *self.body.front().unwrap() // * dereferencia os valores da cabeça do corpo
    }

    /// atualiza movimento da cobra baseado no tempo
    pub fn update(&mut self, dt: f32) {
        // Atualiza velocidade baseada na direção atual
        self.speed = match self.dir {
            Direction::Up => Vec2::new(0.0, -1.0),
            Direction::Down => Vec2::new(0.0, 1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
        };

        let new_head: Vec2 = self.head() + self.speed * dt;

        self.body.push_front(new_head);

        if !self.should_grow {
            self.body.pop_back();
        } else {
            self.should_grow = false;
        }
    }

    /// faz a cobra crescer no próximo update
    pub fn grow(&mut self) {
        self.should_grow = true;
    }

    /// define a direção da cobra (interface para input do teclado)
    pub fn set_dir(&mut self, new_direction: Direction) {
        // Evita que a cobra se mova na direção oposta (colidir consigo mesma)
        let opposite_direction: bool = matches!(
            (&self.dir, new_direction),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        );

        if !opposite_direction {
            self.dir = new_direction;
        }
    }
}

/// struct que representa o jogo em si.
struct Game {
    snake: Snake,
    apple: Apple,
    score: u8,
    lives: u8,
    is_over: bool,
}

/// métodos do jogo, dá pra dizer que são a engine.
impl Game {
    /// construtor do jogo, colocando todos os atores do jogo em posição.
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            apple: Apple::new(),
            score: 0,
            lives: 10,
            is_over: false,
        }
    }

    /// método que conecta a API de controle da cobra ao teclado.
    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.snake.set_dir(Direction::Up);
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.snake.set_dir(Direction::Down);
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.snake.set_dir(Direction::Left);
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.snake.set_dir(Direction::Right);
        }
    }
}

/// funções que convertem as coordenadas da grade do jogo para as coodernadas da tela.
fn to_screen_x(x: f32) -> f32 {
    x / MAX_ARENA_WIDTH * screen_width()
}
fn to_screen_y(y: f32) -> f32 {
    y / MAX_ARENA_HEIGHT * screen_height()
}

/// Função principal, que roda a janela do jogo.
#[macroquad::main("Snake")]
async fn main() {
    let mut game: Game = Game::new(); // jogo instanciado!

    loop {
        let time: f32 = get_frame_time();
        game.handle_input();
        clear_background(RED);
        next_frame().await;
    }
}
