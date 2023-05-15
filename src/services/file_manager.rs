use std::path::Path;
use regex::Regex;
use std::fs;
use std::io;
use chrono::Local;

pub fn copy(from: &str, to: &str) {
    let from = Path::new(from);
    let to = Path::new(to);
    if to.is_dir() {
        if to.exists() {
            backup(&to);
        }
        match copy_recursively(&from, &to) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        if to.exists() {
            backup(&to);
        }
        copy_file(&from, &to);
    }
}

pub fn clean(to: &str) {
    let to = Path::new(to);
    let parent = to.parent().unwrap();
    let backup_file_name = format!(r#"{}\."#, to.file_name().unwrap().to_str().unwrap());
    let re = Regex::new(&backup_file_name).unwrap();
    let dir = if parent.file_name().is_none() {
        Path::new(".")
    } else {
        parent
    };
    for entry in dir.read_dir().unwrap() {
        if let Ok(entry) = entry {
            if re.is_match(entry.file_name().to_str().unwrap()) {
                if entry.file_type().unwrap().is_dir() {
                    fs::remove_dir_all(entry.path()).unwrap();
                } else {
                    fs::remove_file(entry.path()).unwrap();
                }
            }
        }
    }
}

fn backup(to: &Path) {
    let timestamp = Local::now().format("%Y%m%d%H%M").to_string();
    let backup_to = format!("{}.{}", to.to_str().unwrap(), timestamp);
    fs::rename(&to, &backup_to).unwrap();
}

fn copy_file(from: &Path, to: &Path) {
    fs::copy(&from, &to).unwrap();
}

/// Copy files from source to destination recursively.
fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
