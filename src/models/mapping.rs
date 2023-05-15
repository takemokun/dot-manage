use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use std::fs;
use std::io;
use std::path::Path;
use chrono::Local;
use regex::Regex;

use crate::constants;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mapping {
    pub from: String,
    #[serde(deserialize_with = "deserialize_to")]
    pub to: String,
    #[serde(deserialize_with = "deserialize_copy_type")]
    pub copy_type: CopyType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CopyType {
    Replace,
    Append,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MappingFile {
    pub mappings: HashMap<String, HashMap<String, String>>,
}

fn deserialize_copy_type<'de, D>(deserializer: D) -> Result<CopyType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match s.as_str() {
        "replace" => Ok(CopyType::Replace),
        "append" => Ok(CopyType::Append),
        _ => Err(serde::de::Error::custom("invalid copy type")),
    }
}

fn deserialize_to<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.replace("$HOME", &constants::home_path()))
}

impl Mapping {
    pub fn copy(&self) {
        match &self.copy_type {
            CopyType::Append => self.copy_append(),
            CopyType::Replace => self.copy_replace(),
        }
    }

    pub fn clean(&self) {
        match &self.copy_type {
            CopyType::Append => self.clean_append(),
            CopyType::Replace => self.clean_replace(),
        }
    }

    pub fn clean_self(&self) {
        match &self.copy_type {
            CopyType::Append => self.clean_self_append(),
            CopyType::Replace => self.clean_self_replace(),
        }
    }

    pub fn sync(&self) {
        match &self.copy_type {
            CopyType::Append => self.sync_append(),
            CopyType::Replace => self.sync_replace(),
        }
    }

    fn copy_replace(&self) {
        CopyItem::new(&self.from, &self.to).copy();
    }

    fn copy_append(&self) {
        let targets = self.append_targets();

        for target in &targets {
            CopyItem::new(&target.0, &target.1).copy();
        }
    }

    fn clean_replace(&self) {
        CopyItem::new(&self.from, &self.to).clean();
    }

    fn clean_append(&self) {
        let targets = self.append_targets();

        for target in &targets {
            CopyItem::new(&target.0, &target.1).clean();
        }
    }

    fn sync_replace(&self) {
        CopyItem::new(&self.to, &self.from).copy();
    }

    fn sync_append(&self) {
        let targets = self.append_targets();

        for target in &targets {
            CopyItem::new(&target.1, &target.0).copy();
        }
    }

    fn clean_self_replace(&self) {
        CopyItem::new(&self.to, &self.from).clean();
    }

    fn clean_self_append(&self) {
        let targets = self.append_targets();

        for target in &targets {
            CopyItem::new(&target.1, &target.0).clean();
        }
    }

    fn append_targets(&self) -> Vec<(String, String)> {
        let from_path = Path::new(&self.from);
        let to_path = Path::new(&self.to);
        from_path.read_dir().unwrap().map(|entry| {
            let entry = entry.unwrap();
            let entry_path_name = entry.file_name();
            let entry_from = from_path.join(&entry_path_name);
            let entry_from = entry_from.to_str().unwrap().to_string();
            let entry_to = to_path.join(&entry_path_name);
            let entry_to = entry_to.to_str().unwrap().to_string();
            (entry_from, entry_to)
        }).collect()
    }
}

pub struct CopyItem<'a> {
    pub from: &'a Path,
    pub to: &'a Path,
}

impl<'a> CopyItem<'a> {
    pub fn new(from: &'a str, to: &'a str) -> CopyItem<'a> {
        CopyItem {
            from: Path::new(from.clone()),
            to: Path::new(to.clone()),
        }
    }

    pub fn copy(&self) {
        if self.to.is_dir() {
            if self.to.exists() {
                self.backup();
            }
            match Self::copy_recursively(&self.from, &self.to) {
                Ok(_) => (),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            if self.to.exists() {
                self.backup();
            }
            self.copy_file();
        }
    }

    pub fn clean(&self) {
        let parent = self.to.parent().unwrap();
        let backup_file_name = format!(r#"{}\."#, self.to.file_name().unwrap().to_str().unwrap());
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

    fn backup(&self) {
        let copy_to = &self.to.to_str().unwrap();
        let timestamp = Local::now().format("%Y%m%d%H%M").to_string();
        let backup_to = format!("{}.{}", copy_to, timestamp);
        fs::rename(&copy_to, &backup_to).unwrap();
    }

    fn copy_file(&self) {
        fs::copy(&self.from, &self.to).unwrap();
    }

    /// Copy files from source to destination recursively.
    fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let filetype = entry.file_type()?;
            if filetype.is_dir() {
                Self::copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }
}
