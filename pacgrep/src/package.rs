use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub size: u64,
    pub depends: Vec<String>, // List of dependencies
}
