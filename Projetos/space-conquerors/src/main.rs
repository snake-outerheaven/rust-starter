use macroquad::prelude::*;

// Constantes
const GRID_WIDTH: f32 = 800.0;
const GRID_HEIGHT: f32 = 600.0;
const PLAYER_START_Y: f32 = 560.0;
const PLAYER_SPEED: f32 = 4.0;
const BULLET_SPEED: f32 = 8.0;
const ENEMY_STEP_X: f32 = 12.0;
const ENEMY_STEP_Y: f32 = 18.0;
const MAX_PLAYER_BULLETS: usize = 3;

// Enums
#[derive(PartialEq, Clone, Copy)]
enum EntityState {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
enum BulletType {
    Red,
    Blue,
    Green,
}

#[derive(Clone, Copy)]
enum EnemyType {
    Boss,
    MiniBoss,
    Thug,
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    Victory,
    GameOver,
}

// Models
struct Bullet {
    pos: Vec2,
    bullet_type: BulletType,
}

struct Player {
    pos: Vec2,
    status: EntityState,
}

struct Enemy {
    pos: Vec2,
    enemy_type: EnemyType,
    status: EntityState,
}

// Controller
struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    player_bullets: Vec<Bullet>,
    enemy_bullets: Vec<Bullet>, // ✅ Simplificado
    score: u32,
    enemy_moving_right: bool, // ✅ Nome mais claro
    enemy_timer: f32,
    state: GameState,
}

impl Game {
    fn new() -> Self {
        // TODO: Implementar inicialização
        todo!()
    }

    fn update(&mut self, dt: f32) {
        // TODO: Implementar lógica
        todo!()
    }

    fn draw(&self) {
        // TODO: Implementar renderização
        todo!()
    }
}

// Main loop
#[macroquad::main("Space Conquerors")]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update(get_frame_time());

        clear_background(BLACK);
        game.draw();

        next_frame().await
    }
}
