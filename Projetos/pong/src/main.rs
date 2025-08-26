mod utils;

fn main() {
    println!("Oi! Eu sou um Rustáceo!");
    utils::wait(750);
    utils::limpar_tela();
    utils::wait(750);
    println!("Ei! Eu continuo sendo um Rustáceo!");
    utils::wait(750);
    println!("Ok, vamos testar as novas ferramentas!");
    let x: (u32, u32) = utils::xy_generator(10, 10);
    let (objx, objy) = x;
    println!("{objx}, {objy}");
}
