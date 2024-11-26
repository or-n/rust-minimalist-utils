use dependency_db::*;
use std::collections::HashMap;
use std::fs;
use to_seq::*;

fn main() {
    let mut table = HashMap::new();
    let value = (vec![], vec![]);
    table.insert(2137, value);
    let mut bytes = Vec::new();
    Deps { table }.to_seq(&mut bytes, ()).unwrap();
    let hex: String = bytes.iter().map(|byte| format!("{:02X} ", byte)).collect();
    println!("{}", hex);
    fs::write(DEPENDENCIES, bytes).unwrap();
}
