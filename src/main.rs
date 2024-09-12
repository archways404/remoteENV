use clap::{Arg, Command};
use std::env;
use std::process;
use walkdir::WalkDir; // Import walkdir for recursive directory traversal

fn main() {
    let matches = Command::new("My CLI App")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A simple CLI tool in Rust")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Sets the name to greet")
                .required(false),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Prints debug information")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Get the value of the "name" argument using get_one::<String>()
    if let Some(name) = matches.get_one::<String>("name") {
        println!("Hello, {}!", name);
    } else {
        println!("Hello, world!");
    }

    // Check if the "debug" flag is set using get_flag()
    if matches.get_flag("debug") {
        println!("Debug mode is ON");
    }

    // Get the current working directory
    match env::current_dir() {
        Ok(path) => {
            println!("Current directory: {}", path.display());
            find_all_env_files(&path);
        }
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            process::exit(1);
        }
    }
}

// Function to find and print all .env files in the current directory and subdirectories
fn find_all_env_files(dir: &std::path::Path) {
    let mut found = false;

    // Use walkdir to recursively go through all directories and files
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            // Check if the file is named exactly ".env" or has a ".env" extension
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name == ".env" || file_name.starts_with(".env") {
                println!("Found .env file: {}", path.display());
                found = true;
            }
        }
    }

    if !found {
        println!("No .env files found in the current directory and subdirectories.");
    }
}
