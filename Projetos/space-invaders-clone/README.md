# Space Conquerors 🦀🚀

Um clone moderno do clássico Space Invaders, implementado em Rust com arquitetura robusta e memory safety garantido.

## 📋 Sobre o Projeto

Space Conquerors é uma reimaginação do icônico arcade Space Invaders (1978), desenvolvido em Rust utilizando o framework Macroquad. O projeto prioriza código limpo, segurança de memória e abstrações de custo zero, características fundamentais do Rust.

### Arquitetura

O jogo utiliza uma arquitetura baseada em traits e enums para garantir flexibilidade e type safety:

- **Enums de Estado**: Controle preciso do ciclo de vida das entidades
- **Sistema de Tipos**: Diferentes tipos de balas e inimigos
- **Trait `Shooter`**: Interface comum para entidades que podem disparar
- **Pattern MVC**: Separação clara entre lógica (Controller), dados (Models) e renderização (View)

## 🎮 Como Jogar

### Controles

- **Seta para esquerda `←`** ou **A** — Move a nave para esquerda
- **Seta para direita `→`** ou **D** — Move a nave para direita
- **Espaço** — Dispara projétil

### Regras

1. Destrua todos os invasores alienígenas antes que alcancem o solo
2. Os invasores se movem em formação, descendo gradualmente
3. Diferentes tipos de inimigos oferecem desafios únicos:
   - **Boss** 👑 — Mais resistente e perigoso
   - **Mini-Boss** 💀 — Inimigo intermediário
   - **Thug** 👾 — Invasor básico
4. O jogo termina se sua nave for destruída ou os invasores alcançarem o solo

## 🚀 Como Executar

### Pré-requisitos

- [Rust](https://rustup.rs/) (versão 1.70 ou superior recomendada)
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

4. Ou execute em modo release (otimizado):

   ```bash
   cargo run --release
   ```

### Compilação para Distribuição

Para gerar um executável otimizado:

```bash
cargo build --release
```

O executável estará disponível em `target/release/space-invaders-clone` (ou `.exe` no Windows).

## 🛠️ Tecnologias Utilizadas

- **[Rust](https://www.rust-lang.org/)** 🦀 — Linguagem de programação systems-level com memory safety
- **[Macroquad](https://macroquad.rs/)** 🎮 — Framework minimalista para jogos 2D
- **Edition 2021** — Utilizando as features mais recentes do Rust

## 📁 Estrutura do Projeto

```
space-conquerors/
├── src/
│   └── main.rs          # Código principal (Models, Controller, View)
├── Cargo.toml           # Manifesto de dependências
├── Cargo.lock           # Lock das versões das dependências
└── README.md            # Este arquivo
```

### Organização do Código

O arquivo `main.rs` está estruturado em seções claras:

1. **Constantes**: Dimensões da grid, velocidades e parâmetros do jogo
2. **Enumerações**: Estados e tipos das entidades
3. **Models**: Estruturas de dados (`Player`, `Enemy`, `Bullet`)
4. **Traits**: Comportamentos compartilhados (`Shooter`)
5. **Controller**: Lógica do jogo (`Game`)
6. **View**: Loop principal e renderização com Macroquad

## 🔧 Configurações

Personalize a experiência do jogo editando as constantes em `main.rs`:

```rust
const GRID_WIDTH: f32 = 800.0;        // Largura da área de jogo
const GRID_HEIGHT: f32 = 600.0;       // Altura da área de jogo
const PLAYER_SPEED: f32 = 4.0;        // Velocidade da nave
const BULLET_SPEED: f32 = 8.0;        // Velocidade dos projéteis
const ENEMY_STEP_X: f32 = 12.0;       // Movimento horizontal dos inimigos
const ENEMY_STEP_Y: f32 = 18.0;       // Descida vertical dos inimigos
```

## 🎯 Sistema de Balas

O jogo implementa três tipos de projéteis, cada um com características únicas:

- 🔴 **Red Bullet** — Projétil padrão
- 🔵 **Blue Bullet** — Projétil especial
- 🟢 **Green Bullet** — Projétil avançado

## 🏆 Características Técnicas

- ✅ **Memory Safety**: Garantido pelo sistema de ownership do Rust
- ✅ **Zero Runtime Overhead**: Abstrações sem custo de performance
- ✅ **Type Safety**: Sistema de tipos forte previne bugs em tempo de compilação
- ✅ **Pattern Matching**: Uso extensivo de enums para estados do jogo
- ✅ **Trait System**: Interface flexível para comportamentos compartilhados

## 🚧 Status do Projeto

⚠️ **Em Desenvolvimento** — Este projeto está em fase inicial. As implementações das structs, traits e game loop estão pendentes.

### Próximos Passos

- [ ] Implementar lógica do `Player`
- [ ] Implementar lógica dos `Enemy`
- [ ] Implementar trait `Shooter`
- [ ] Desenvolver `Game` controller
- [ ] Adicionar renderização visual
- [ ] Implementar sistema de colisão
- [ ] Adicionar efeitos sonoros
- [ ] Sistema de high scores

## 📝 Licença

Este projeto é de código aberto sob a [Licença MIT](LICENSE).

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para:

1. Fazer fork do projeto
2. Criar uma branch para sua feature (`git checkout -b feature/NovaFeature`)
3. Commit suas mudanças (`git commit -m 'Adiciona nova feature'`)
4. Push para a branch (`git push origin feature/NovaFeature`)
5. Abrir um Pull Request

---

**Desenvolvido com 🦀 Rust**  
**E muita determinação 🚀**
