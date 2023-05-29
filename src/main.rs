use std::process;
use dotfiles::config::Config;
use dotfiles::command_executor;
use std::env;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    command_executor::run(&config.command, config.file_name.as_deref());
}
