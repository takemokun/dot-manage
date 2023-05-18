use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // 引数があるかどうかをチェック
    if args.len() <= 1 {
        dotfiles::command_info::help();
        exit(0)
    }

    let command = &args[1];

    dotfiles::command_info::main_command(&command);
}
