use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs::read_to_string, path::Path};

#[derive(Deserialize)]
pub struct PackageData {
    author: String,
    pkg_name: String,
    version: String,
}

impl PackageData {
    #[inline]
    pub fn new(file: &Path) -> Result<Self> {
        let file_str = read_to_string(file).with_context(|| "Failed to read Cofyfile contents")?;

        toml::from_str(&file_str).with_context(|| "Failed to parse Cofyfile data")
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
