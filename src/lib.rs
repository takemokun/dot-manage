mod constants {
    use std::env;

    pub const FILE_PATH: &str = "mapping.json";

    pub fn home_path() -> String {
        dotenv::dotenv().ok();
        env::var("HOME_PATH").unwrap()
    }
}

mod models;
mod services;
mod factories;

pub mod command_executor;
pub mod commands;
pub mod config;
pub mod mapping;

pub use constants::home_path;
