# Space Conquerors 🦀🚀

Um clone minimalista do clássico Space Invaders, implementado em Rust com foco em simplicidade e memory safety.

## 📋 Sobre o Projeto

Space Conquerors é uma reimaginação direta do icônico arcade Space Invaders (1978), desenvolvido em Rust utilizando o framework Macroquad. O projeto prioriza código limpo, arquitetura simples e as garantias de segurança de memória do Rust.

### Arquitetura

O jogo utiliza uma arquitetura direta e não-escalável, ideal para projetos educacionais:

- **Enums de Estado**: Controle preciso do ciclo de vida das entidades (`Alive`/`Dead`)
- **Sistema de Tipos**: Diferentes tipos de balas e inimigos através de enums simples
- **Pattern MVC Simplificado**: Separação clara entre dados (Models), lógica (Controller) e renderização (View)
- **Código Único**: Todo o jogo em um único arquivo `main.rs` (~500 linhas)

## 🎮 Como Jogar

### Controles

- **Seta para esquerda `←`** ou **A** — Move a nave para esquerda
- **Seta para direita `→`** ou **D** — Move a nave para direita
- **Espaço** — Dispara projétil (máximo 3 simultâneos)

### Regras

1. Destrua todos os invasores alienígenas antes que alcancem o solo
2. Os invasores se movem em formação horizontal, descendo ao atingir as bordas
3. Você pode ter no máximo 3 balas ativas na tela simultaneamente
4. Diferentes tipos de inimigos oferecem pontuações diferentes:
   - **Boss** 👑 — Inimigo principal (maior pontuação)
   - **Mini-Boss** 💀 — Inimigo intermediário
   - **Thug** 👾 — Invasor básico
5. O jogo termina em vitória se todos os inimigos forem destruídos
6. O jogo termina em derrota se os invasores alcançarem o solo

## 🚀 Como Executar

### Pré-requisitos

- [Rust](https://rustup.rs/) (versão 1.70 ou superior)
- Cargo (incluído com a instalação do Rust)

### Instalação e Execução

1. Clone este repositório:

   ```bash
   git clone https://github.com/seu-usuario/space-conquerors.git
   ```

2. Navegue até o diretório do projeto:

   ```bash
   cd space-conquerors
   ```

3. Execute o jogo em modo desenvolvimento:

   ```bash
   cargo run
   ```

4. Ou execute em modo release (performance otimizada):

   ```bash
   cargo run --release
   ```

### Compilação para Distribuição

Para gerar um executável otimizado:

```bash
cargo build --release
```

O executável estará disponível em:
- **Linux/macOS**: `target/release/space-invaders-clone`
- **Windows**: `target/release/space-invaders-clone.exe`

## 🛠️ Tecnologias Utilizadas

- **[Rust](https://www.rust-lang.org/)** 🦀 — Linguagem de programação systems-level com memory safety
- **[Macroquad](https://macroquad.rs/)** 🎮 — Framework minimalista para jogos 2D
- **Edition 2021** — Utilizando as features mais recentes do Rust

## 📁 Estrutura do Projeto

```
space-conquerors/
├── src/
│   └── main.rs          # Todo o código do jogo (~500 linhas)
├── Cargo.toml           # Manifesto de dependências
├── Cargo.lock           # Lock das versões das dependências
└── README.md            # Este arquivo
```

### Organização do Código (main.rs)

O arquivo está estruturado em seções bem definidas:

```rust
// 1. Constantes
const GRID_WIDTH: f32 = 800.0;
const MAX_PLAYER_BULLETS: usize = 3;
// ...

// 2. Enumerações
enum EntityState { Alive, Dead }
enum BulletType { Red, Blue, Green }
enum EnemyType { Boss, MiniBoss, Thug }
enum GameState { Playing, Victory, GameOver }

// 3. Models (Structs)
struct Player { pos: Vec2, status: EntityState }
struct Enemy { pos: Vec2, enemy_type: EnemyType, status: EntityState }
struct Bullet { pos: Vec2, bullet_type: BulletType }

// 4. Controller
struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    player_bullets: Vec<Bullet>,
    enemy_bullets: Vec<Bullet>,
    score: u32,
    enemy_moving_right: bool,
    enemy_timer: f32,
    state: GameState,
}

impl Game {
    fn new() -> Self { /* ... */ }
    fn update(&mut self, dt: f32) { /* ... */ }
    fn draw(&self) { /* ... */ }
}

// 5. Main Loop
#[macroquad::main("Space Conquerors")]
async fn main() { /* ... */ }
```

## 🔧 Configurações

Personalize a experiência do jogo editando as constantes no início do `main.rs`:

```rust
const GRID_WIDTH: f32 = 800.0;           // Largura da janela
const GRID_HEIGHT: f32 = 600.0;          // Altura da janela
const PLAYER_SPEED: f32 = 4.0;           // Velocidade da nave
const BULLET_SPEED: f32 = 8.0;           // Velocidade dos projéteis
const ENEMY_STEP_X: f32 = 12.0;          // Movimento horizontal dos inimigos
const ENEMY_STEP_Y: f32 = 18.0;          // Descida vertical dos inimigos
const MAX_PLAYER_BULLETS: usize = 3;     // Limite de balas na tela
```

## 🎯 Sistema de Balas

O jogo suporta três tipos de projéteis (cores diferentes):

- 🔴 **Red Bullet** — Projétil vermelho
- 🔵 **Blue Bullet** — Projétil azul (padrão do jogador)
- 🟢 **Green Bullet** — Projétil verde

> **Nota**: Atualmente todos os tipos têm o mesmo comportamento, mas a estrutura permite fácil diferenciação futura.

## 🏆 Características Técnicas

- ✅ **Memory Safety**: Sistema de ownership do Rust previne memory leaks
- ✅ **Type Safety**: Enums fortemente tipados previnem bugs de estado inválido
- ✅ **Zero Allocations Desnecessárias**: Uso eficiente de `Vec` e tipos stack-allocated
- ✅ **Pattern Matching**: Uso idiomático de `match` para controle de fluxo
- ✅ **Simplicidade**: Arquitetura não-escalável focada em clareza

## 🎮 Detalhes de Implementação

### Sistema de Colisão
- Detecção simples baseada em distância euclidiana
- Verificação de sobreposição de bounding boxes

### Movimento dos Inimigos
- Movimento em grupo sincronizado
- Mudança de direção ao atingir bordas
- Descida progressiva a cada mudança de direção

### Controle de Estado
- Estados discretos: `Playing`, `Victory`, `GameOver`
- Transições automáticas baseadas em condições do jogo

## 🚧 Status do Projeto

✅ **Estrutura Completa** — Todos os tipos e arquitetura definidos

### Em Desenvolvimento

- [ ] Implementar `Game::new()` - Inicializar grid de inimigos
- [ ] Implementar `Game::update()` - Loop de jogo e física
- [ ] Implementar `Game::draw()` - Renderização visual
- [ ] Sistema de colisão bala-inimigo
- [ ] Movimento sincronizado dos inimigos
- [ ] Condições de vitória e derrota
- [ ] Sistema de pontuação

### Features Futuras (Opcional)

- [ ] Sons e música
- [ ] Efeitos visuais (explosões, partículas)
- [ ] Sistema de high scores
- [ ] Múltiplas ondas/fases
- [ ] Power-ups

## 📝 Licença

Este projeto é de código aberto sob a [Licença MIT](LICENSE).

## 🤝 Contribuindo

Este é um projeto educacional focado em simplicidade. Contribuições são bem-vindas desde que mantenham a filosofia de código direto e não-escalável.

### Como Contribuir

1. Fork o projeto
2. Crie uma branch (`git checkout -b feature/MinhaFeature`)
3. Commit suas mudanças (`git commit -m 'Adiciona MinhaFeature'`)
4. Push para a branch (`git push origin feature/MinhaFeature`)
5. Abra um Pull Request

### Diretrizes

- Mantenha todo o código em `main.rs`
- Evite abstrações desnecessárias
- Priorize legibilidade sobre performance
- Documente apenas o não-óbvio

## 📚 Aprendizado

Este projeto é ideal para aprender:

- Fundamentos de Rust (ownership, borrowing, enums)
- Desenvolvimento de jogos simples
- Arquitetura MVC básica
- Framework Macroquad
- Game loops e delta time

## 🙏 Créditos

Inspirado no clássico **Space Invaders** (1978) por Tomohiro Nishikado.

---

**Desenvolvido com 🦀 Rust**  
**Mantido simples por design 🎯**
