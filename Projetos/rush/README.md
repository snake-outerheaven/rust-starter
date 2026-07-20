# Rush 🦀⚡
Um shell minimalista escrito em Rust. Arquitetura limpa, memory safety garantido, syscalls diretos.

## 📋 Sobre
Rush é um shell Unix moderno implementado em Rust, demonstrando os princípios de ownership, borrowing e error handling idiomático. Código estruturado com separação de responsabilidades e safety-first design.

### Arquitetura
Design baseado em componentes com clara separação de concerns:
- **Parser** - Tokenização de entrada via `split_whitespace()`
- **Runner** - Execução de comandos com `std::process::Command`
- **Shell** - Orquestração do REPL e gerenciamento de estado
- **ShellStatus** - Enum para controle de fluxo type-safe

```
Input → Parser → Runner → ShellStatus → Shell
                    ↓
              Command/Builtin
```

## 💻 Uso
```sh
snake@outerheaven /home/snake! ls -la
snake@outerheaven /home/snake! cd /tmp
snake@outerheaven /tmp! pwd
/tmp
snake@outerheaven /tmp! exit
```

### Builtins
- **`cd [path]`** — Muda diretório (default: $HOME)
- **`exit`** — Encerra o shell

### Comandos Externos
Qualquer comando não-builtin executa via `Command::new()` buscando no PATH do sistema.

## 🚀 Executar
```sh
cargo run
```

Ou compilar para release:
```sh
cargo build --release
./target/release/rush
```

## 🛠️ Stack
- **Rust** 🦀 — Memory safety sem garbage collector
- **std::process::Command** — Abstração sobre fork/exec
- **std::env** — Variáveis de ambiente e navegação de diretórios
- **gethostname** — Obtenção do hostname do sistema

## 📁 Estrutura
```
rush/
├── src/
│   └── main.rs          # Implementação completa
├── Cargo.toml           # Dependências e metadata
└── README.md
```

## 🚧 Status

### Fase 1: Core ✅
- [x] REPL funcional
- [x] Parser com tokenização
- [x] Runner com pattern matching
- [x] Enum-based status handling
- [x] Builtin: `exit`
- [x] Builtin: `cd` com fallback para $HOME
- [x] Prompt customizado (user@hostname path)
- [x] Fail-safe mechanism (MAX_SHELL_TRIES)

### Fase 2: Error Handling
- [ ] Custom error types (`ShellError`)
- [ ] `Result<T, E>` propagation com `?`
- [ ] Graceful degradation

### Fase 3: Command Registry
- [ ] HashMap de builtins
- [ ] Trait `Executable` para comandos
- [ ] Plugin system para extensões

### Fase 4: Redirecionamento
- [ ] `>`, `<`, `2>` (stdout, stdin, stderr)
- [ ] `>>` (append)

### Fase 5: Pipes
- [ ] `|` (pipeline entre processos)

### Fase 6: Job Control
- [ ] `&` (background jobs)
- [ ] Signal handling (SIGINT, SIGTSTP)
- [ ] `jobs`, `fg`, `bg` builtins

### Fase 7: Advanced Features
- [ ] Command history
- [ ] Tab completion
- [ ] Environment variable expansion (`$VAR`)
- [ ] Config file (`~/.rushrc`)

## 🏆 Características

- ✅ **Memory Safety** - Ownership e borrowing do Rust
- ✅ **Type Safety** - Enum-based state machine
- ✅ **Error Handling** - Pattern matching extensivo
- ✅ **Zero Unsafe** - Nenhum bloco `unsafe`
- ✅ **Modular** - Separação clara: Parser, Runner, Shell
- ✅ **Fail-Safe** - Proteção contra loops infinitos de erro
- ✅ **Cross-Platform** - Funciona em qualquer Unix-like

## 📚 Conceitos Demonstrados

### Rust
- Ownership e borrowing
- Pattern matching com `match`
- Enums para state machines
- Trait objects (`dyn Executable` - futuro)
- Error handling idiomático
- Zero-cost abstractions

### Sistemas Operacionais
- **Process Management** - fork/exec via `Command`
- **File Descriptors** - stdin/stdout/stderr
- **Environment Variables** - Propagação de contexto
- **Working Directory** - `chdir()` via `set_current_dir()`
- **System Calls** - Abstração Rust sobre libc

## 🎯 Filosofia de Design

1. **Simplicidade** - Código claro sobre cleverness
2. **Safety** - Compiler-enforced correctness
3. **Modularidade** - Componentes independentes
4. **Iterativo** - Funcionalidade incremental
5. **Educational** - Código como documentação

## 🧪 Aprendizados

Este projeto demonstra:
- Como Rust gerencia processos de forma segura
- Padrões de arquitetura em programação de sistemas
- Trade-offs entre ergonomia e performance
- A diferença entre abstrações de alto nível (Rust) e baixo nível (C)

## 🔗 Dependências

```toml
[dependencies]
gethostname = "0.5"
```

## 📖 Referências

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust CLI Book](https://rust-cli.github.io/book/)
- [Unix Process Model](https://en.wikipedia.org/wiki/Process_(computing))

## 🤝 Contribuindo

Rush é um projeto educacional. Pull requests são bem-vindos para:
- Novos builtins
- Melhorias de error handling
- Otimizações de performance
- Documentação

## 📜 Licença

MIT License - Faça o que quiser, aprenda e compartilhe.

---

**Desenvolvido com 🦀 Rust e ⚡ paixão por programação de sistemas**  
*"Onde ownership encontra syscalls"*
