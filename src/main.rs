use std::process;
use dotfiles::config::Config;
use dotfiles::command_executor;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    command_executor::run(&config.command);
}
