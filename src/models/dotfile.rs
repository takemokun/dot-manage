use crate::models::{
    Mapping,
    CopyType,
    PathBehavior,
    AppendBehavior,
    ReplaceBehavior,
};

#[derive(Debug)]
pub struct Dotfile {
    pub path_behavior: Box<dyn PathBehavior>,
}

impl Dotfile {
    pub fn new(mapping: Mapping) -> Self {
        let path_behavior = Self::create_path_behavior(mapping);

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

    fn create_path_behavior(mapping: Mapping) -> Box<dyn PathBehavior> {
        match &mapping.copy_type {
            CopyType::Append => {
                Box::new(AppendBehavior {
                    from: mapping.from.clone(),
                    to: mapping.to.clone(),
                })
            },
            CopyType::Replace => {
                Box::new(ReplaceBehavior {
                    from: mapping.from.clone(),
                    to: mapping.to.clone(),
                })
            },
        }
    }
}
