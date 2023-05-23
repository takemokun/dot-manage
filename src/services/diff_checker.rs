extern crate diff;
use std::fs::File;
use std::io::{Read, Error};
use std::collections::HashSet;
use colored::Colorize;

pub fn run(path_from: &str, path_to: &str) -> Result<(), Error> {
    let left = read_file(&path_from)?;
    let right = read_file(&path_to)?;
    let diff_lines = diff::lines(&left, &right);
    let print_lines: Vec<u32> = pick_display_lines(&diff_lines);

    if print_lines.is_empty() {
        println!("no diff");
        return Ok(());
    }

    let mut is_displayed_previous = false;

    for (current_line, diff) in diff_lines.iter().enumerate() {
        is_displayed_previous = match diff {
            diff::Result::Left(l) => {
                println!("{}{}", "+".green(), l.green());
                true
            },
            diff::Result::Both(l, _) if print_lines.contains(&(current_line as u32)) => {
                if !is_displayed_previous {
                    println!("{}", "...diff".yellow());
                }
                println!("{}", l);
                true
            },
            diff::Result::Both(_, _) => false,
            diff::Result::Right(r)   => {
                println!("{}{}", "-".red(), r.red());
                true
            }
        };
    }

    Ok(())
}

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = String::new();
    let mut f = File::open(&path)?;
    f.read_to_string(&mut file)?;

    Ok(file.to_string())
}

// diffに表示する行を抽出する
// ※ 差分だけの表示だと分かりづらいため、差分の前後何行かを出すための対応
fn pick_display_lines(diff_lines: &Vec<diff::Result<&str>>) -> Vec<u32> {
    diff_lines.iter().enumerate()
        .filter_map(|(i, diff)| {
            match diff {
                diff::Result::Left(_) | diff::Result::Right(_) => Some(i),
                _ => None,
            }
        })
        .flat_map(|i| display_range(i as u32))
        .map(|i| i as u32)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
}

fn display_range(i: u32) -> Vec<u32> {
    (i.saturating_sub(3)..=i + 3).collect()
}
