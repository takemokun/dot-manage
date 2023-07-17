use std::fmt::Debug;
use colored::*;
use textwrap;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub enum Command {
    Apply,
    Sync,
    Clean,
    CleanSelf,
    Help,
}

#[derive(Debug)]
pub struct CommandInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub example: &'static str,
    pub command: Command,
}

pub static COMMANDS: Lazy<Vec<CommandInfo>> = Lazy::new(|| {
    vec![
        CommandInfo {
            name: "apply",
            description: "Copies dotfiles to your configuration directory.",
            example: "dotfiles/.zshrc -> ~/.zshrc",
            command: Command::Apply,
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

pub fn display_help() {
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
