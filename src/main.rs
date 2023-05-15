use std::io;

fn main() {
    let mut command = String::new();

    println!("実行したい内容を入力してください\n(copy, clean, clean_self, sync)");

    io::stdin().read_line(&mut command).unwrap();
    let command: String = command.trim().parse().expect("文字列を入力してください");

    match command.as_str() {
        "copy" => dotfiles::copy::run(),
        "clean" => dotfiles::clean::run(),
        "clean_self" => dotfiles::clean_self::run(),
        "sync" => dotfiles::sync::run(),
        _ => println!("error"),
    }
    // fs::copy(".zshrc", &copy_path).expect("failed to copy file");
    // println!("{}", timestamp);
}
