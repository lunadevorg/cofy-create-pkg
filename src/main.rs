use std::{
    io::{self, Write},
    path::Path,
    process::exit,
};

mod pkg_data;
use pkg_data::PackageData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = Path::new("Cofyfile");
    if !file.exists() {
        eprintln!("\x1b[31merror:\x1b[0m cannot find Cofyfile");
        exit(1);
    }

    let data = PackageData::new(file)?;

    let mut handle = io::stdout().lock();
    write!(
        handle,
        "\x1b[32mauthor:\x1b[0m {}\
        \n\x1b[32mpackage name:\x1b[0m {}\
        \n\x1b[32mversion:\x1b[0m {}\
        \nis this data right? (y/n) ",
        data.author(), data.pkg_name(), data.version()
    )?;

    handle.flush()?;

    loop {
        let mut inp = String::new();
        io::stdin().read_line(&mut inp)?;
        match inp.trim() {
            "n" | "N" => {
                println!("exiting...");
                exit(1);
            }
            "y" | "Y" => {
                println!("\x1b[32mcontinuing...\x1b[0m");
                break;
            }
            _ => {
                continue;
            }
        }
    }

    handle.write_all(b"enter package server: ")?;
    handle.flush()?;

    let mut inp = String::new();
    io::stdin().read_line(&mut inp)?;

    print!("all done! package is sent for verification to {inp}");
    Ok(())
}
