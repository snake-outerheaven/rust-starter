use macroquad::prelude::*;

const GRID_WIDTH: f32 = 800.0;
const GRID_HEIGHT: f32 = 600.0;
const PLAYER_START_Y: f32 = 560.0;
const PLAYER_SPEED: f32 = 4.0;
const BULLET_SPEED: f32 = 8.0;
const ENEMY_STEP_X: f32 = 12.0;
const ENEMY_STEP_Y: f32 = 18.0;

/*
 * Enumerações de Estado
 */

enum EntityState {
    Alive,
    Dead,
    Unknown,
}

enum BulletType {
    Red,
    Blue,
    Green,
}

enum EnemyType {
    Boss,
    MiniBoss,
    Thug,
}

enum GameState {
    Over,
    Running,
    Unknown,
}

/*
 * Models
 */
struct Bullet {
    pos: Vec2,
    bullet: BulletType,
}

struct Player {
    name: String,
    pos: Vec2,
    inv: Vec<Bullet>,
    status: EntityState,
}

struct Enemy {
    name: String,
    pos: Vec2,
    enemy: EnemyType,
}

/*
 * Traços úteis para determinar comportamento comum entre as entidades
 */
trait Shooter {
    fn shoot(&self) -> Bullet;
}

/*
 * Implementação dos models
 */

/*
 * Controller do Jogo
 */
struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    score: u32,
    state: GameState,
}
/*
 * Implementação do Controller
 */

/*
 * View do jogo com Macroquad
 */
#[macroquad::main("Space Conquerors")]
async fn main() {
    loop {
        next_frame().await
    }
}
