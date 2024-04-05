use std::{fs::read_to_string, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PackageData {
    pub author: String,
    pub pkg_name: String,
    pub version: String,
}

impl PackageData {
    pub fn new(file: &Path) -> Result<PackageData, Box<dyn std::error::Error>> {
        let file_str = read_to_string(file)?;

        Ok(toml::from_str(&file_str)?)
    }
}
