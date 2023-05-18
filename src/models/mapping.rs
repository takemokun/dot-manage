use serde::{Deserialize, Serialize};

use crate::{constants};
use crate::models::path_behavior::{PathBehavior, AppendBehavior, ReplaceBehavior};

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

impl Mapping {
    pub fn copy(&self) {
        println!("copying from {} to {}", &self.from, &self.to);
        let _ = &self.path_behavior().copy();
    }

    pub fn clean(&self) {
        println!("cleaning {}.*", &self.to);
        let _ = &self.path_behavior().clean();
    }

    pub fn clean_me(&self) {
        println!("cleaning {}.*", &self.from);
        let _ = &self.path_behavior().clean_me();
    }

    pub fn sync(&self) {
        println!("syncing from {} to {}", &self.to, &self.from);
        let _ = &self.path_behavior().sync();
    }

    pub fn path_behavior(&self) -> Box<dyn PathBehavior> {
        match &self.copy_type {
            CopyType::Append => {
                Box::new(AppendBehavior {
                    from: self.from.clone(),
                    to: self.to.clone(),
                })
            },
            CopyType::Replace => {
                Box::new(ReplaceBehavior {
                    from: self.from.clone(),
                    to: self.to.clone(),
                })
            },
        }
    }
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
