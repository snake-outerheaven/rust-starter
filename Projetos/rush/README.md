# Rush 🦀💻

Um shell minimalista em Rust. Interface direta entre intenção humana e syscalls.

## 📋 Sobre

Rush é um shell Unix simples em Rust usando `std::process::Command`. Código limpo, memory safety, um único arquivo.

### Arquitetura

REPL (Read-Eval-Print Loop) direto e não-escalável:

- **Command Abstraction** - `std::process::Command` sobre fork/exec
- **Código Único** - Todo o shell em `main.rs`

## 💻 Uso

```sh
rsh> ls -la
rsh> cargo build
rsh> exit
```

### Builtin

- **`exit`** — Encerra o shell

### Comandos Externos

Tudo que não for builtin executa via `Command::new()` procurando no PATH do sistema.

## 🚀 Executar

```sh
cargo run
```

ou

```sh
cargo run --release
```

## 🛠️ Stack

- **Rust** 🦀 — Memory safety
- **std::process::Command** — Abstração sobre fork/exec
- **std::io** — I/O com error handling

## 📁 Estrutura

```
rush/
├── src/
│   └── main.rs          # Todo o código
├── Cargo.toml
└── README.md
```

## 🚧 Status

### Fase 1: MVP ✅
- [x] REPL básico
- [x] Parsing simples (split por espaços)
- [x] Execução via `Command`
- [x] Builtin: `exit`

### Fase 2: Builtins
- [ ] `cd` (change directory)
- [ ] `pwd` (print working directory)
- [ ] `export` (variáveis de ambiente)

### Fase 3: Redirecionamento
- [ ] `>`, `<`, `2>` (stdout, stdin, stderr)

### Fase 4: Pipes
- [ ] `|` (pipeline)

### Fase 5: Job Control
- [ ] `&` (background)
- [ ] Signal handling

## 🏆 Características

- ✅ Memory Safety - Ownership do Rust
- ✅ Error Handling - `Result<T, E>`
- ✅ Zero Unsafe - Nenhum bloco unsafe
- ✅ Simplicidade - Não-escalável por design

## 📚 Aprendizado

- Ownership, borrowing, lifetimes
- Process management
- I/O e file descriptors
- Error handling idiomático

## 🎓 Conceitos de SO

- **Process Control Block** - Estrutura do kernel
- **File Descriptors** - 0=stdin, 1=stdout, 2=stderr
- **Fork/Exec** - Modelo Unix de processos
- **Environment Variables** - Contexto entre processos

---

**Desenvolvido com 🦀 Rust**  
**Onde intenção encontra syscall 💻**
