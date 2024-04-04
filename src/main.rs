use std::{fs::File, io::{Read, Write}, process::exit};

use pkg_data::PackageData;
mod pkg_data;

fn main() {
    let res = File::open("Cofyfile");

    if res.is_err() {
        println!("\x1b[31merror:\x1b[0m cannot find Cofyfile");
        exit(1);
    }

    let file = res.unwrap();
    let data = PackageData::new(file);

    println!("\x1b[32mauthor:\x1b[0m {}", data.author);
    println!("\x1b[32mpackage name:\x1b[0m {}", data.pkg_name);
    println!("\x1b[32mversion:\x1b[0m {}", data.version);
    print!("is this data right? (y/n) ");
    let _ = std::io::stdout().flush();

    let mut byte = [0_u8];
    let _ = std::io::stdin().lock().read_exact(&mut byte);

    match byte[0] {
        b'n' => {
            println!("exiting...");
            exit(1);
        },
        b'y' => {
            println!("\x1b[32mcontinuing...\x1b[0m");
        },
        _ => {
            println!("exiting...");
            exit(1);
        }
    }

    print!("enter package server: ");
    let _ = std::io::stdout().flush();

    let mut inp = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut inp);

    //Install path or smth

}
