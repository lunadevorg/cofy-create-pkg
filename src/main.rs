use anyhow::{bail, Context, Result};
use std::{
    io::{self, Write},
    path::Path,
    process::exit,
};

mod pkg_data;
use pkg_data::PackageData;

fn main() -> Result<()> {
    let file = Path::new("Cofyfile");
    match file.try_exists() {
        Ok(false) => bail!("Cofyfile doesn't exist"),
        Err(err) => bail!("Failed to check Cofyfile existance: {err}"),
        _ => (),
    }

    let data = PackageData::new(file).with_context(|| "Couldn't read information from Cofyfile")?;

    let mut handle = io::stdout().lock();
    write!(
        handle,
        "\x1b[32mauthor:\x1b[0m {}\
        \n\x1b[32mpackage name:\x1b[0m {}\
        \n\x1b[32mversion:\x1b[0m {}\
        \nis this data right? (y/n) ",
        data.author(),
        data.pkg_name(),
        data.version()
    )
    .with_context(|| "Failed to write meta information into handle")?;

    handle.flush()?;

    loop {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .with_context(|| "Failed to read input")?;
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

    handle.write_all(b"Enter package server: ")?;
    handle.flush()?;

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .with_context(|| "Failed to read input")?;

    print!("All done! package is sent for verification to {inp}");
    Ok(())
}
