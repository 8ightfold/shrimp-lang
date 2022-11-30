#[allow(non_camel_case_types)]
mod shrimp;
use colored::*;
use std::env::current_dir;
use shrimp::utility::print_error;

fn main() {
    let curr_dir = current_dir();
    match curr_dir {
        Ok(val) => println!("{}: {}", "current directory".blue(), val.display()),
        Err(error) => { print_error!(format!("{error}")); std::process::exit(-1) },
    }

    let files = shrimp::read_files();
    println!();
    match files {
        Some(val) => {
            for file in val {
                println!("{file}\n")
            }
        },
        None => { print_error!("failed to read file".to_string()); std::process::exit(-1) },
    }
}
