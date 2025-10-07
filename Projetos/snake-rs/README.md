
# Snake em Rust 🐍

Um clone do clássico jogo Snake, implementado em Rust para estudos e diversão!

## 📋 Sobre o Projeto

Este projeto é uma implementação simples do jogo Snake, ideal para quem quer aprender programação em Rust, lógica de jogos e manipulação de gráficos. O objetivo é controlar a cobrinha, comer o máximo de frutas possível e crescer sem bater nas paredes ou em si mesma.

## 🎮 Como Jogar

### Controles

- **Seta para cima (`↑`)** — Move a cobrinha para cima
- **Seta para baixo (`↓`)** — Move a cobrinha para baixo
- **Seta para esquerda (`←`)** — Move a cobrinha para a esquerda
- **Seta para direita (`→`)** — Move a cobrinha para a direita

### Regras

1. A cobrinha cresce cada vez que come uma fruta
2. O jogo acaba se a cobrinha bater nas paredes ou em si mesma
3. Tente alcançar a maior pontuação possível!

## 🚀 Como Executar

### Pré-requisitos

- [Rust](https://rustup.rs/) instalado no sistema
- Cargo (incluído com a instalação do Rust)

### Instalação e Execução

1. Clone este repositório:
   ```bash
   git clone https://github.com/snake-outerheaven/rust-starter.git
   ```
2. Navegue até o diretório do projeto:
   ```bash
   cd rust-starter
   ```
3. Execute o jogo:
   ```bash
   cargo run
   ```

### Compilação para Release

Para uma versão otimizada:
```bash
cargo build --release
```
O executável estará em `target/release/rust-starter`.

## 🛠️ Tecnologias Utilizadas

- **[Rust](https://www.rust-lang.org/)** — Linguagem de programação
- **[Macroquad](https://macroquad.rs/)** — Framework para gráficos 2D (confirme qual está usando
- **[Rand](https://crates.io/crates/rand)** — Geração de números aleatórios

## 📁 Estrutura do Projeto

```
rust-starter/
├── src/
│   └── main.rs          # Código principal do jogo
├── Cargo.toml           # Manifesto de dependências
├── Cargo.lock           # Lock das versões das dependências
└── README.md            # Este arquivo
```

## 🎯 Características Técnicas

- **Arena Dinâmica:** Tamanho ajustável para diferentes resoluções
- **Movimentação Suave:** Controle responsivo da cobrinha
- **Frutas Aleatórias:** Cada fruta aparece em uma posição aleatória
- **Pontuação:** Placar exibido durante o jogo

## 🔧 Configurações

Você pode modificar constantes como tamanho da arena, velocidade da cobrinha e cores editando o arquivo `main.rs`:

```rust
const GRID_SIZE: usize = 20;      // Tamanho da grade
const SNAKE_SPEED: f32 = 5.0;     // Velocidade da cobrinha
const FRUIT_COLOR: Color = RED;   // Cor da fruta
```

## 🤝 Contribuições

Sinta-se à vontade para contribuir! Algumas ideias de melhoria:

- [ ] Tela de início/menu
- [ ] Pausa e reinício rápido
- [ ] Efeitos sonoros ao comer frutas
- [ ] Diferentes níveis de dificuldade
- [ ] Obstáculos extras no mapa
- [ ] Ranking de pontuação

## 📝 Licença

Este projeto é de código aberto sob a [Licença MIT](LICENSE).

## 🎉 Divirta-se!

Aproveite para aprender Rust e desafiar seus reflexos com este clássico dos jogos!
