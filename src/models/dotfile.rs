use crate::models::PathBehavior;
use crate::services::diff_checker;

#[derive(Debug)]
pub struct Dotfile {
    pub path_behavior: Box<dyn PathBehavior>,
}

impl Dotfile {
    pub fn new(path_behavior: Box<dyn PathBehavior>) -> Self {
        Self {
            path_behavior,
        }
    }

    pub fn copy(&self) {
        println!("copying from {} to {}", &self.path_behavior.from(), &self.path_behavior.to());
        let _ = &self.path_behavior.copy();
    }

    pub fn clean(&self) {
        println!("cleaning {}.*", &self.path_behavior.to());
        let _ = &self.path_behavior.clean();
    }

    pub fn clean_me(&self) {
        println!("cleaning {}.*", &self.path_behavior.from());
        let _ = &self.path_behavior.clean_me();
    }

    pub fn sync(&self) {
        println!("syncing from {} to {}", &self.path_behavior.to(), &self.path_behavior.from());
        let _ = &self.path_behavior.sync();
    }

    pub fn diff(&self) {
        diff_checker::run(&self.path_behavior.from(), &self.path_behavior.to()).unwrap();
    }
}
