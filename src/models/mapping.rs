use serde::{Deserialize, Serialize};

use crate::{constants};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mapping {
    pub from: String,
    #[serde(deserialize_with = "deserialize_to")]
    pub to: String,
    #[serde(deserialize_with = "deserialize_copy_type")]
    pub copy_type: CopyType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CopyType {
    Replace,
    Append,
}

impl Mapping {}

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
