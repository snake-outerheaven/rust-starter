# Pong em Rust 🏓

Um jogo clássico de Pong implementado em Rust usando a biblioteca Macroquad.

## 📋 Sobre o Projeto

Este é um clone do clássico jogo Pong, onde dois jogadores controlam raquetes para rebater uma bola de um lado para o outro. O objetivo é fazer com que a bola passe pela raquete do oponente para marcar pontos.

## 🎮 Como Jogar

### Controles

- **Jogador 1 (Raquete Azul - Esquerda)**:
  - `W` - Mover para cima
  - `S` - Mover para baixo

- **Jogador 2 (Raquete Vermelha - Direita)**:
  - `↑` (Seta para cima) - Mover para cima
  - `↓` (Seta para baixo) - Mover para baixo

### Regras

1. Cada jogador controla uma raquete na sua lateral da tela
2. A bola rebate nas paredes superior e inferior
3. Quando a bola toca uma raquete, ela muda de direção
4. Se a bola sair pela lateral esquerda ou direita, o oponente marca um ponto
5. Após cada ponto, a bola retorna ao centro com velocidade aleatória

## 🚀 Como Executar

### Pré-requisitos

- [Rust](https://rustup.rs/) instalado no sistema
- Cargo (incluído com a instalação do Rust)

### Instalação e Execução

1. Clone ou baixe este repositório
2. Navegue até o diretório do projeto:
   ```bash
   cd pong
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

O executável estará em `target/release/pong`.

## 🛠️ Tecnologias Utilizadas

- **[Rust](https://www.rust-lang.org/)** - Linguagem de programação
- **[Macroquad](https://macroquad.rs/)** - Framework para jogos 2D
- **[Rand](https://crates.io/crates/rand)** - Geração de números aleatórios

## 📁 Estrutura do Projeto

```
pong/
├── src/
│   └── main.rs          # Código principal do jogo
├── Cargo.toml           # Configurações e dependências
├── Cargo.lock           # Lock das versões das dependências
└── README.md            # Este arquivo
```

## 🎯 Características Técnicas

- **Sistema de Coordenadas**: Arena virtual de 40x30 unidades mapeada para a tela
- **Física Simples**: Colisões básicas com rebotes nas paredes e raquetes
- **Velocidade Variável**: A bola inicia com velocidade aleatória em direções aleatórias
- **60 FPS**: Loop de jogo otimizado para gameplay fluido
- **Input Responsivo**: Controles suaves e precisos

## 🔧 Configurações

As seguintes constantes podem ser modificadas no arquivo `main.rs`:

```rust
const ARENA_WIDTH: f32 = 40.0;      // Largura da arena
const ARENA_HEIGHT: f32 = 30.0;     // Altura da arena
const PADDLE_SPEED: f32 = 15.0;     // Velocidade das raquetes
const PADDLE_WIDTH: f32 = 10.0;     // Largura das raquetes (pixels)
const PADDLE_HEIGHT: f32 = 60.0;    // Altura das raquetes (pixels)
const BALL_SIZE: f32 = 10.0;        // Tamanho da bola (pixels)
```

## 🤝 Contribuições

Sinta-se à vontade para contribuir com melhorias! Algumas ideias:

- [ ] Menu inicial
- [ ] Pausa no jogo
- [ ] Sons e efeitos
- [ ] IA para jogador único
- [ ] Diferentes níveis de dificuldade
- [ ] Sistema de vidas/rounds
- [ ] Melhores gráficos/sprites

## 📝 Licença

Este projeto é de código aberto e está disponível sob a [Licença MIT](LICENSE).

## 🎉 Divirta-se!

Aproveite jogando este clássico jogo de Pong! Perfect para uma partida rápida entre amigos.