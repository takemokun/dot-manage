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
pub mod command_info;
pub mod commands;

pub use constants::home_path;
