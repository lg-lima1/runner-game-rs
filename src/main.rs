use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();

    let config = runner_game::config(&args).unwrap_or_else(|err| {
        eprintln!("Arguments error: {}", err);
        process::exit(1);
    });

    println!("Welcome to runner game!");

    if let Err(e) = runner_game::run(config) {
        eprintln!("Application error : {}", e);
        process::exit(1);
    }
}
