use std::io::Write;
use std::io;
use std::process::exit;

fn main() {
    let mut command = String::new();
    let available_commands = dotfiles::command_info::available_commands();
    println!("実行したい内容を入力してください({})", available_commands.join(", "));

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        command.clear(); // Clear the previous command
        io::stdin().read_line(&mut command).unwrap();

        let current_command: String = command.trim().parse().expect("文字列を入力してください");
        let is_progress_command = dotfiles::command_info::progress_command(&current_command);

        if !is_progress_command {
            command = current_command;
            break;
        }
    }

    dotfiles::command_info::main_command(&command);
}
