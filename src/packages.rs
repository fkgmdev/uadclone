use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Package {
    pub list: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub neededBy: Vec<String>,
    pub labels: Vec<String>,
    pub removal: String,
}

pub fn load_package_list() -> HashMap<String, Package> {
    let json_string = include_str!("uad_lists.json");
    serde_json::from_str(json_string)
        .expect("Failed to parse json")
}