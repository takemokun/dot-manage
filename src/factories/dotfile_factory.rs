use crate::constants;
use std::fs::File;
use std::io::BufReader;

use crate::entities::{
    Dotfile,
    PathBehavior,
    AppendBehavior,
    ReplaceBehavior,
};

use crate::entities::mapping::{Mapping, CopyType};

pub fn create_from_mappings() -> Vec<Dotfile> {
    let file_path = constants::FILE_PATH;
    let file = File::open(file_path).expect("ファイルが存在しません");
    let buf_reader = BufReader::new(file);
    let data: Vec<Mapping> = serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました");

    if data.len() == 0 {
        panic!("データが存在しません");
    }

    data.iter().map(|mapping| {
        let path_behavior = create_path_behavior(mapping);
        Dotfile::new(path_behavior)
    }).collect()
}

fn create_path_behavior(mapping: &Mapping) -> Box<dyn PathBehavior> {
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
