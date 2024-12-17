use dependency_db::dep::{DepValue, DEPENDENCIES};
use dependency_db::*;
use spit::*;
use std::collections::HashMap;
use std::fs;

fn main() {
    let mut table = HashMap::new();
    let value = DepValue::<u32> {
        bytes: vec![],
        deps: vec![],
    };
    table.insert(2137, value);
    let mut bytes = Vec::new();
    bytes = Table { table }.spit(bytes).unwrap();
    let hex: String = bytes.iter().map(|byte| format!("{:02X} ", byte)).collect();
    println!("{}", hex);
    fs::write(DEPENDENCIES, bytes).unwrap();
}
