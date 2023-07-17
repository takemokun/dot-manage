use std::fmt::Debug;
use std::path::Path;
use crate::services::file_manager;

pub trait PathBehavior: Debug {
    fn from(&self) -> &String;
    fn to(&self) -> &String;
    fn apply(&self);
    fn sync(&self);
    fn clean(&self);
    fn clean_me(&self);
    fn append_targets(&self) -> Vec<(String, String)> {
        Vec::new()
    }
}

#[derive(Debug)]
pub struct AppendBehavior {
    pub from: String,
    pub to: String,
}

#[derive(Debug)]
pub struct ReplaceBehavior {
    pub from: String,
    pub to: String,
}

impl PathBehavior for ReplaceBehavior {
    fn from(&self) -> &String {
        &self.from
    }

    fn to(&self) -> &String {
        &self.to
    }

    fn apply(&self) {
        file_manager::copy(&self.from, &self.to);
    }

    fn sync(&self) {
        file_manager::copy(&self.to, &self.from);
    }

    fn clean(&self) {
        file_manager::clean(&self.to);
    }

    fn clean_me(&self) {
        file_manager::clean(&self.from);
    }
}

impl PathBehavior for AppendBehavior {
    fn from(&self) -> &String {
        &self.from
    }

    fn to(&self) -> &String {
        &self.to
    }

    fn apply(&self) {
        for target in &self.append_targets() {
            file_manager::copy(&target.0, &target.1);
        }
    }

    fn sync(&self) {
        for target in &self.append_targets() {
            file_manager::copy(&target.1, &target.0);
        }
    }

    fn clean(&self) {
        for target in &self.append_targets() {
            file_manager::clean(&target.1);
        }
    }

    fn clean_me(&self) {
        for target in &self.append_targets() {
            file_manager::clean(&target.0);
        }
    }

    fn append_targets(&self) -> Vec<(String, String)> {
        let from_dir_path = Path::new(&self.from);
        let to_path = Path::new(&self.to);
        from_dir_path.read_dir().unwrap().map(|entry| {
            let entry_path_name = entry.unwrap().file_name();
            let entry_from = from_dir_path.join(&entry_path_name);
            let entry_from = entry_from.to_str().unwrap().to_string();
            let entry_to = to_path.join(&entry_path_name);
            let entry_to = entry_to.to_str().unwrap().to_string();
            (entry_from, entry_to)
        }).collect()
    }
}
