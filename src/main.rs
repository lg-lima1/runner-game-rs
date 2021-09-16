use std::process;

fn main() {
    println!("Welcome to runner game!");

    if let Err(e) = runner_game::run() {
        eprintln!("Application error : {}", e);
        process::exit(1);
    }
}
