use std::process::Command;
// forma bonita de chamar variáveis

pub fn limpar_tela() {
    let cmd: &'static str = if cfg!(windows) { "cls" } else { "clear" };
    // caso houvesse outros sistemas operacionais, seria um else if cfg!(unix)
    // para linux, mac e outros sistemas baseados em unix.

    if Command::new(cmd).status().is_err() {
        println!("\x1B[2J\x1B[H");
    }
}
