use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let package_json_string =
        read_to_string("package.json").expect("No package.json was found in this directory.");

    let package_json: PackageJson =
        from_str(package_json_string.as_str()).expect("package.json may not be a valid JSON.");

    println!("This package contains the following scripts:");

    package_json.scripts.keys().for_each(|script_name| {
        println!("- {}", script_name);
    });
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    name: String,
    scripts: HashMap<String, String>,
}
