use std::fmt::Debug;
use colored::*;
use textwrap;

#[derive(Debug)]
enum Command {
    Copy,
    Sync,
    Clean,
    CleanSelf,
    Help,
}

#[derive(Debug)]
struct CommandInfo {
    name: &'static str,
    description: &'static str,
    example: &'static str,
    command: Command,
}

// HACK: 構造体か何かでよしなにしたい
//       generate_commands叩くのを一回だけにしたい
fn generate_commands() -> Vec<CommandInfo> {
    vec![
        CommandInfo {
            name: "copy",
            description: "Copies dotfiles to your configuration directory.",
            example: "dotfiles/.zshrc -> ~/.zshrc",
            command: Command::Copy,
        },
        CommandInfo {
            name: "sync",
            description: "Copies files from your configuration directory back to dotfiles.",
            example: "~/.zshrc -> dotfiles/.zshrc",
            command: Command::Sync,
        },
        CommandInfo {
            name: "clean",
            description: "Deletes backup files from your configuration directory.",
            example: "deletes like ~/.zshrc.201901010000 files",
            command: Command::Clean,
        },
        CommandInfo {
            name: "clean-me",
            description: "Deletes backup files from the specified directory.",
            example: "deletes like dotfiles/.zshrc.201901010000 files",
            command: Command::CleanSelf,
        },
        CommandInfo {
            name: "help",
            description: "Displays this help message.",
            example: "",
            command: Command::Help,
        },
    ]
}

pub fn available_commands() -> Vec<&'static str> {
    generate_commands().iter().map(|c| c.name).collect()
}

pub fn progress_command(command: &str) -> bool {
    let commands = generate_commands();
    let command_info = commands.iter().find(|c| c.name == command);

    if command_info.is_none() {
        println!("{} is not found. try {}\n", command.red(), "help".green());
        return true;
    }

    match command_info.unwrap().command {
        Command::Help => {
            help();
            true
        },
        _ => false,
    }
}

pub fn main_command(command: &str) {
    let commands = generate_commands();
    let command_info = commands.iter().find(|c| c.name == command).unwrap();

    match command_info.command {
        Command::Copy => crate::copy::run(),
        Command::Sync => crate::sync::run(),
        Command::Clean => crate::clean::run(),
        Command::CleanSelf => crate::clean_self::run(),
        Command::Help => help(),
    }
}

pub fn help() {
    let commands = generate_commands();
    println!("{}", generate_help_message(&commands));
}

fn generate_help_message(commands: &[CommandInfo]) -> String {
    let max_length = commands
        .iter()
        .map(|command| command.name.len())
        .max()
        .unwrap_or(0);

    let indent = " ".repeat(4);
    let indent_double = indent.repeat(2).clone();
    let command_space = " ".repeat(4);

    let mut help_message = textwrap::dedent(&format!("
        dotfiles
        ========

        {}Usage:
        {}dotfiles <command>
        {}
        {}Commands:
    ", indent, indent_double, indent, indent));

    for command in commands {
        let padded_name = format!("{:<width$}", command.name.green().bold(), width = max_length);
        help_message.push_str(&format!("{}{}{}{}\n", indent_double, padded_name, command_space, command.description.magenta()));
        if !command.example.is_empty() {
            let sub_indent = " ".repeat(max_length) + &indent;
            help_message.push_str(&format!("{}{}{}Ex. {}\n", indent_double, sub_indent, command_space, command.example));
        }
        help_message.push_str("\n");
    }

    help_message
}
