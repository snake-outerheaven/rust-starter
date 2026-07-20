use std::process::Command;

use chrono::Local;

use crate::utils::clear;

mod task;
mod utils;
fn main() {
    clear();

    let test_task = task::Task::new(
        1,
        "Show Myself".to_string(),
        "Literally show Myself".to_string(),
        Local::now(),
        Some(Local::now()),
    );

    Command::new("cmd")
        .args(["/C", "cls"])
        .status()
        .expect("Crash and burn!");

    println!("{:.?}", test_task);

    if test_task.finished_at.is_some() {
        println!("Missão concluida!")
    }
}
