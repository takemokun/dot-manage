use std::process::exit;

fn main() {
    // let mut command = String::new();
    // let available_commands = dotfiles::command_info::available_commands();
    // println!("実行したい内容を入力してください({})", available_commands.join(", "));

    let args: Vec<String> = std::env::args().collect();

    // 引数があるかどうかをチェック
    if args.len() <= 1 {
        println!("🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈\n");

        dotfiles::command_info::help();
        exit(0)
    }

    let command = &args[1];

    dotfiles::command_info::main_command(&command);
}
