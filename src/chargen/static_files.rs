extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn read_json(file_path: &str) -> HashMap<String, String> {
    let mut file = File::open(file_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json: HashMap<String, String> = serde_json::from_str(&data).unwrap();
    return json
}

