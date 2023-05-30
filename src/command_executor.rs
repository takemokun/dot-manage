use std::fmt::Debug;
use colored::*;
use textwrap;
use once_cell::sync::Lazy;

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

static COMMANDS: Lazy<Vec<CommandInfo>> = Lazy::new(|| {
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
});

pub fn run(command: &str, file_name: Option<&str>) {
    let command_info = COMMANDS.iter().find(|c| c.name == command).unwrap();

    match command_info.command {
        Command::Copy => crate::commands::copy(file_name),
        Command::Sync => crate::commands::sync(file_name),
        Command::Clean => crate::commands::clean(file_name),
        Command::CleanSelf => crate::commands::clean_me(file_name),
        Command::Help => display_help(),
    }
}

fn display_help() {
    let max_length: usize = COMMANDS
        .iter()
        .map(|command| command.name.len())
        .max()
        .unwrap_or(0);

    let indent = " ".repeat(4);
    let indent_double = indent.repeat(2).clone();

    let mut help_message = textwrap::dedent(&format!("
        =====================================================
        ==                      Help                       ==
        =====================================================

        {}Usage:
        {}dotfiles [Command] [filename]

        {}filename:
        {}If you want to execute command only for specific file, you can specify filename. (optional)

        {}Commands:
    ", indent, indent_double, indent, indent_double, indent));

    for command in COMMANDS.iter() {
        let padded_name = format!("{:<width$}", command.name.green().bold(), width = max_length);
        help_message.push_str(&format!("{}{}{}{}\n", indent_double, padded_name, indent, command.description.yellow()));

        if !command.example.is_empty() {
            let indent_to_description_line = " ".repeat(max_length);
            help_message.push_str(&format!("{}{}{}Ex. {}\n", indent_double, indent_to_description_line, indent_double, command.example));
        }

        help_message.push_str("\n");
    }

    println!("{}", help_message);
}
