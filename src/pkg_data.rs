use std::{fs::read_to_string, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PackageData {
    author: String,
    pkg_name: String,
    version: String,
}

impl PackageData {
    #[inline]
    pub fn new(file: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file_str = read_to_string(file)?;

        Ok(toml::from_str(&file_str)?)
    }

    #[inline]
    #[must_use]
    pub fn author(&self) -> &str {
        &self.author
    }

    #[inline]
    #[must_use]
    pub fn pkg_name(&self) -> &str {
        &self.pkg_name
    }

    #[inline]
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}
