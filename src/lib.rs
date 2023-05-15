mod constants {
    use std::env;

    pub const FILE_PATH: &str = "mapping.json";

    pub fn home_path() -> String {
        dotenv::dotenv().ok();
        env::var("HOME_PATH").unwrap()
    }
}

mod commands;
mod models;
mod services;

pub use constants::home_path;
pub use commands::*;

use std::fs::File;
use std::io::BufReader;

pub fn read_mapping_or_panic() -> Vec<models::Mapping> {
    let file_path = constants::FILE_PATH;
    let file = File::open(file_path).expect("ファイルが存在しません");
    let buf_reader = BufReader::new(file);
    let data: Vec<models::Mapping> = serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました");

    if data.len() == 0 {
        panic!("データが存在しません");
    }

    data
}
