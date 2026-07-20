use ::rand::rngs::ThreadRng;
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use rodio::source::{Amplify, TakeDuration};
use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Dimensões da arena em unidades arbitrárias
const ARENA_WIDTH: f32 = 40.0;
const ARENA_HEIGHT: f32 = 30.0;

/// Velocidade de movimento da raquete (unidades por segundo)
const PADDLE_SPEED: f32 = 22.5;

/// Largura e altura da raquete em pixels
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 60.0;

/// Tamanho da bola em pixels
const BALL_SIZE: f32 = 10.0;

/// Aceleração da bola (unidades por segundo²)
const BALL_ACCELERATION: f32 = 0.4;

/// Velocidade máxima da bola
const MAX_BALL_SPEED: f32 = 10.0;

/// Sistema de áudio 8-bit usando rodio
struct AudioSystem {
    collision_timer: f32,
    music_timer: f32,
    music_index: usize,
    background_notes: Vec<(f32, f32, &'static str)>, // (frequência, duração, nome)
    _stream: OutputStream,
    sink: Arc<Mutex<Sink>>,
}

impl AudioSystem {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Inicializa áudio com rodio
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink: Sink = Sink::try_new(&stream_handle)?;

        // Progressão vi-IV-I-V clássica dos anos 80 em E menor - tons agudos
        let background_notes: Vec<(f32, f32, &'static str)> = vec![
            // Primeira volta: C#m - A - E - B (vi - IV - I - V)
            (554.37, 1.2, "C#5"), // C# minor (vi) - início melancólico
            (880.00, 1.0, "A5"),  // A major (IV) - resolução
            (659.25, 1.0, "E5"),  // E major (I) - casa
            (987.77, 1.4, "B5"),  // B major (V) - tensão para loop
            // Segunda volta com variação: C#m - A - E - B
            (554.37, 0.8, "C#5"), // C# minor mais rápido
            (880.00, 1.2, "A5"),  // A major prolongado
            (659.25, 0.8, "E5"),  // E major rápido
            (987.77, 1.2, "B5"),  // B major sustentado
            // Build-up: Am - F - C - G (transposição momentânea)
            (880.00, 0.6, "A5"), // A como menor
            (698.46, 0.6, "F5"), // F major
            (523.25, 0.6, "C5"), // C major
            (783.99, 0.6, "G5"), // G major
            // Volta ao E menor: C#m - A - E - B (clássico final)
            (554.37, 1.5, "C#5"), // C# minor climático
            (880.00, 1.0, "A5"),  // A major
            (659.25, 1.0, "E5"),  // E major
            (987.77, 2.0, "B5"),  // B major final longo
        ];

        Ok(Self {
            collision_timer: 0.0,
            music_timer: 0.0,
            music_index: 0,
            background_notes,
            _stream,
            sink: Arc::new(Mutex::new(sink)),
        })
    }

    pub fn play_collision(&mut self) {
        if self.collision_timer <= 0.0 {
            self.collision_timer = 0.15; // Cooldown

            // Toca beep de colisão
            if let Ok(sink) = self.sink.lock() {
                let source: Amplify<TakeDuration<SineWave>> = SineWave::new(800.0)
                    .take_duration(Duration::from_millis(100))
                    .amplify(0.3);
                sink.append(source);
            }
            println!("🔊 BEEP! (800Hz collision sound)");
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.collision_timer > 0.0 {
            self.collision_timer -= dt;
        }

        // Música de fundo (sequenciador simples)
        self.music_timer += dt;
        let (_, duration, _) = self.background_notes[self.music_index];
        // forma interessante de desestruturas apenas determinados valores de uma tupla

        if self.music_timer >= duration {
            self.music_timer = 0.0;
            self.music_index = (self.music_index + 1) % self.background_notes.len();

            let (freq, dur, note_name): (f32, f32, &'static str) =
                self.background_notes[self.music_index];

            // Toca a nota da melodia
            if let Ok(sink) = self.sink.lock() {
                let source: Amplify<TakeDuration<SineWave>> = SineWave::new(freq)
                    .take_duration(Duration::from_secs_f32(dur * 0.8)) // Um pouco mais curto
                    .amplify(0.1); // Volume baixo para música de fundo
                sink.append(source);
            }

            println!("🎵 80s vi-IV-I-V: {} ({:.0}Hz)", note_name, freq);
        }
    }

    pub fn get_current_note(&self) -> &str {
        self.background_notes[self.music_index].2
    }
}

/// Estrutura que representa uma raquete
struct Paddle {
    x: f32,
    y: f32,
    height: f32,
    vy: f32,
    dir: Direction,
}

/// Enum para direção do movimento da raquete
enum Direction {
    Up,
    Down,
    None,
}

impl Paddle {
    /// Cria uma nova raquete na posição horizontal `posx`
    pub fn new(posx: f32) -> Self {
        Self {
            x: posx,
            y: ARENA_HEIGHT / 2.0,
            height: 1.5,
            vy: 0.0,
            dir: Direction::None,
        }
    }

    /// Define a velocidade da raquete baseada na direção atual
    pub fn set_dir(&mut self) {
        self.vy = match self.dir {
            Direction::Up => -PADDLE_SPEED,
            Direction::Down => PADDLE_SPEED,
            Direction::None => 0.0,
        }
    }

    /// Atualiza a posição da raquete usando o tempo delta
    pub fn update(&mut self, dt: f32) {
        self.y += self.vy * dt;
        // limita dentro da arena
        self.y = self.y.clamp(0.0, ARENA_HEIGHT);
    }
}

/// Estrutura que representa a bola
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Ball {
    /// Cria uma nova bola com velocidade inicial aleatória
    pub fn new() -> Self {
        let mut rng: ThreadRng = thread_rng();
        let vx: f32 = if rng.gen_bool(0.5) {
            rng.gen_range(2.4..=4.0)
        } else {
            -rng.gen_range(2.4..=4.0)
        };
        let vy: f32 = if rng.gen_bool(0.5) {
            rng.gen_range(2.4..=4.0)
        } else {
            -rng.gen_range(2.4..=4.0)
        };

        Self {
            x: ARENA_WIDTH / 2.0,
            y: ARENA_HEIGHT / 2.0,
            vx,
            vy,
        }
    }

    /// Move a bola de acordo com sua velocidade e tempo delta
    pub fn update(&mut self, dt: f32) {
        // Aplica aceleração gradual
        let current_speed: f32 = (self.vx.powi(2) + self.vy.powi(2)).sqrt();
        if current_speed < MAX_BALL_SPEED {
            let acceleration_factor = 1.0 + (BALL_ACCELERATION * dt);
            self.vx *= acceleration_factor;
            self.vy *= acceleration_factor;
        }

        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }

    /// Acelera a bola ligeiramente após rebater em uma raquete
    fn accelerate(&mut self) {
        let speed_increase: f32 = 1.1; // Aumenta 10% na velocidade
        self.vx *= speed_increase;
        self.vy *= speed_increase;

        // Limita a velocidade máxima
        if self.vx.abs() > MAX_BALL_SPEED {
            self.vx = self.vx.signum() * MAX_BALL_SPEED;
        }
        if self.vy.abs() > MAX_BALL_SPEED {
            self.vy = self.vy.signum() * MAX_BALL_SPEED;
        }
    }

    /// Reseta a bola para o centro com velocidade aleatória
    pub fn reset(&mut self) {
        self.x = ARENA_WIDTH / 2.0;
        self.y = ARENA_HEIGHT / 2.0;

        let mut rng = thread_rng();
        self.vx = if rng.gen_bool(0.5) {
            rng.gen_range(2.4..=4.0)
        } else {
            -rng.gen_range(2.4..=4.0)
        };
        self.vy = if rng.gen_bool(0.5) {
            rng.gen_range(2.4..=4.0)
        } else {
            -rng.gen_range(2.4..=4.0)
        };
    }
}

/// Estrutura que representa o estado do jogo
struct Game {
    ball: Ball,
    p1: Paddle,
    p2: Paddle,
    score1: u8,
    score2: u8,
    audio: AudioSystem,
}

impl Game {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let audio: AudioSystem = AudioSystem::new()?;

        Ok(Self {
            ball: Ball::new(),
            p1: Paddle::new(2.0),
            p2: Paddle::new(ARENA_WIDTH - 2.0),
            score1: 0,
            score2: 0,
            audio,
        })
    }

    /// Atualiza pontuações e retorna true se um ponto foi marcado
    pub fn score_up(&mut self) -> bool {
        if self.ball.x <= 0.0 {
            self.score2 += 1;
            return true;
        }
        if self.ball.x >= ARENA_WIDTH {
            self.score1 += 1;
            return true;
        }
        false
    }

    /// Atualiza todos os objetos do jogo
    pub fn update(&mut self, dt: f32) {
        self.p1.set_dir();
        self.p2.set_dir();
        self.p1.update(dt);
        self.p2.update(dt);

        let old_ball_x = self.ball.x;
        let old_ball_y = self.ball.y;
        self.ball.update(dt);

        // Verifica colisões e toca sons
        let had_collision = self.check_collisions(old_ball_x, old_ball_y);
        if had_collision {
            self.audio.play_collision();
        }

        if self.score_up() {
            self.ball.reset();
        }

        self.audio.update(dt);
    }

    /// Verifica colisões e retorna true se houve alguma
    fn check_collisions(&mut self, old_x: f32, old_y: f32) -> bool {
        let mut had_collision = false;

        // Colisões com paredes superior e inferior
        if (self.ball.y <= 0.0 || self.ball.y >= ARENA_HEIGHT)
            && (old_y > 0.0 && old_y < ARENA_HEIGHT)
        {
            self.ball.vy = -self.ball.vy;
            had_collision = true;
        }

        // Colisões com raquetes
        if (self.ball.x - self.p1.x).abs() < 0.5
            && (self.p1.y - self.p1.height..=self.p1.y + self.p1.height).contains(&self.ball.y)
            && old_x > self.p1.x
        {
            // Evita colisões múltiplas
            self.ball.vx = -self.ball.vx;
            self.ball.accelerate();
            had_collision = true;
        }
        if (self.ball.x - self.p2.x).abs() < 0.5
            && (self.p2.y - self.p2.height..=self.p2.y + self.p2.height).contains(&self.ball.y)
            && old_x < self.p2.x
        {
            // Evita colisões múltiplas
            self.ball.vx = -self.ball.vx;
            self.ball.accelerate();
            had_collision = true;
        }

        had_collision
    }
}

/// Converte coordenadas da arena para pixels da tela
fn to_screen_x(x: f32) -> f32 {
    x / ARENA_WIDTH * screen_width()
}
fn to_screen_y(y: f32) -> f32 {
    y / ARENA_HEIGHT * screen_height()
}

/// Loop principal assíncrono usando Macroquad
#[macroquad::main("Pong")]
async fn main() {
    let mut game = Game::new().expect("Falha ao inicializar áudio");

    loop {
        // Entrada (Input)
        game.p1.dir = if is_key_down(KeyCode::W) {
            Direction::Up
        } else if is_key_down(KeyCode::S) {
            Direction::Down
        } else {
            Direction::None
        };

        game.p2.dir = if is_key_down(KeyCode::Up) {
            Direction::Up
        } else if is_key_down(KeyCode::Down) {
            Direction::Down
        } else {
            Direction::None
        };

        // Atualização
        let dt = get_frame_time();
        game.update(dt);

        // Desenho
        clear_background(BLACK);

        // Desenha as raquetes
        draw_rectangle(
            to_screen_x(game.p1.x),
            to_screen_y(game.p1.y) - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            BLUE,
        );
        draw_rectangle(
            to_screen_x(game.p2.x),
            to_screen_y(game.p2.y) - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            RED,
        );

        // Desenha a bola
        draw_rectangle(
            to_screen_x(game.ball.x),
            to_screen_y(game.ball.y),
            BALL_SIZE,
            BALL_SIZE,
            GREEN,
        );

        // Desenha as pontuações
        draw_text(
            &format!("{}", game.score1),
            screen_width() * 0.25,
            50.0,
            50.0,
            BLUE,
        );
        draw_text(
            &format!("{}", game.score2),
            screen_width() * 0.75,
            50.0,
            50.0,
            RED,
        );

        // Mostra nota atual da música (80s Synth Theme)
        draw_text(
            &format!("♪ vi-IV-I-V: {}", game.audio.get_current_note()),
            20.0,
            screen_height() - 30.0,
            20.0,
            PINK,
        );

        next_frame().await;
    }
}
