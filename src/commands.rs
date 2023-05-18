use crate::constants;
use crate::models;
use std::fs::File;
use std::io::BufReader;

pub fn copy() {
    run_on_all_data(models::Mapping::copy, "copy");
}

pub fn sync() {
    run_on_all_data(models::Mapping::sync, "sync");
}

pub fn clean() {
    run_on_all_data(models::Mapping::clean, "clean");
}

pub fn clean_me() {
    run_on_all_data(models::Mapping::clean_me, "clean_me")
}

fn run_on_all_data<F: Fn(&models::Mapping)>(operation: F, operation_name: &str) {
    println!("starting {}...", operation_name);

    let mappings = read_mapping_or_panic();
    for mapping in &mappings {
        operation(mapping);
    }
}

fn read_mapping_or_panic() -> Vec<models::Mapping> {
    let file_path = constants::FILE_PATH;
    let file = File::open(file_path).expect("ファイルが存在しません");
    let buf_reader = BufReader::new(file);
    let data: Vec<models::Mapping> = serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました");

    if data.len() == 0 {
        panic!("データが存在しません");
    }

    data
}
