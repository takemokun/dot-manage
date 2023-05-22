use std::io::Write;
use std::io;
use colored::*;

use crate::models::Dotfile;
use crate::factories::dotfile_factory;

pub fn copy() {
    run_on_all_data(Dotfile::copy, "copy");
}

pub fn sync() {
    run_on_all_data(Dotfile::sync, "sync");
}

pub fn clean() {
    run_on_all_data(Dotfile::clean, "clean");
}

pub fn clean_me() {
    run_on_all_data(Dotfile::clean_me, "clean_me")
}

fn run_on_all_data<F: Fn(&Dotfile)>(operation: F, operation_name: &str) {
    println!("starting {}...", operation_name);

    let dotfiles_data = dotfile_factory::create_from_mappings();
    let mut is_all = false;

    for dotfile in &dotfiles_data {
        if is_all {
            operation(dotfile);
            continue;
        }

        let mut command = String::new();
        loop {
            print!("> {} to {} (y, n, a, q, h): ", operation_name.bold(), dotfile.path_behavior.from().red().bold());

            io::stdout().flush().unwrap();
            command.clear();
            io::stdin().read_line(&mut command).unwrap();
            let current_command: String = command.trim().parse().expect("文字列を入力してください");

            match current_command.as_str() {
                "y" => operation(dotfile),
                "n" => {},
                "a" => {
                    is_all = true;
                    operation(dotfile)
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
