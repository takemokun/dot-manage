use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::io;
use colored::*;

use crate::constants;
use crate::models;

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
    let mut is_all = false;

    for mapping in &mappings {
        if is_all {
            operation(mapping);
            continue;
        }

        let mut command = String::new();
        loop {
            print!("> {} to {} (y, n, a, q, h): ", operation_name.bold(), mapping.from.red().bold());

            io::stdout().flush().unwrap();
            command.clear();
            io::stdin().read_line(&mut command).unwrap();
            let current_command: String = command.trim().parse().expect("文字列を入力してください");

            match current_command.as_str() {
                "y" => operation(mapping),
                "n" => {},
                "a" => {
                    is_all = true;
                    operation(mapping)
                },
                "q" => {
                    println!("quit");
                    std::process::exit(0)
                },
                "h" => {
                    print_help_message();
                    continue;
                },
                _ => {
                    println!("{} is not found\n", current_command);
                    print_help_message();
                    continue;
                },
            }

            break
        }
    }
}

fn print_help_message() {
    println!("help:");
    println!("    y: execute this file");
    println!("    n: skip this file");
    println!("    a: execute all files");
    println!("    q: quit");
    println!("    h: help");
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
