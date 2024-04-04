use std::{fs::File, io::Write, process::exit};

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

    let mut inp = String::new();
    let _ = std::io::stdin().read_line(&mut inp);

    match inp.chars().nth(0).unwrap() {
        'n' => {
            println!("exiting...");
            exit(1);
        },
        'y' => {
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
    let _ = std::io::stdin().read_line(&mut inp);

    print!("all done! package is sent for verification to {inp}");
    exit(0);

}
