use std::fs::File;

use pkg_data::PackageData;
mod pkg_data;

fn main() {
    let file = File::open("Cofyfile").unwrap();

    let data = PackageData::new(file);

    println!("Author: {}", data.author);
    println!("Package Name: {}", data.pkg_name);
    println!("Version: {}", data.version);
}
