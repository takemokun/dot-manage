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

pub fn run(command: &str) {
    let commands = generate_commands();
    let command_info = commands.iter().find(|c| c.name == command).unwrap();

    match command_info.command {
        Command::Copy => crate::commands::copy(),
        Command::Sync => crate::commands::sync(),
        Command::Clean => crate::commands::clean(),
        Command::CleanSelf => crate::commands::clean_me(),
        Command::Help => display_help_message(&commands),
    }
}

fn display_help_message(commands: &[CommandInfo]) {
    let max_length = commands
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
        {}dotfiles <command>
        {}
        {}Commands:
    ", indent, indent_double, indent, indent));

    for command in commands {
        let padded_name = format!("{:<width$}", command.name.green().bold(), width = max_length);
        help_message.push_str(&format!("{}{}{}{}\n", indent_double, padded_name, indent, command.description.magenta()));

        if !command.example.is_empty() {
            let indent_to_description_line = " ".repeat(max_length);
            help_message.push_str(&format!("{}{}{}Ex. {}\n", indent_double, indent_to_description_line, indent_double, command.example));
        }

        help_message.push_str("\n");
    }

    println!("{}", help_message);
}
