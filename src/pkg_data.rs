use std::{fs::File, io::Read};

use toml::de::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PackageData {
    pub author: String,
    pub pkg_name: String,
    pub version: String   
}

impl PackageData {
    pub fn new(mut file: File) -> PackageData {
        let mut file_str = String::new();
        let _ = file.read_to_string(&mut file_str);

        let res: Result<PackageData, Error> = toml::from_str(&file_str);    

        res.unwrap()
    }
}