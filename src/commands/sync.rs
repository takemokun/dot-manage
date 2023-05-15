use crate::read_mapping_or_panic;

pub fn run() {
    println!("starting sync...");

    let data = read_mapping_or_panic();
    for d in &data {
        d.sync();
    }
}
