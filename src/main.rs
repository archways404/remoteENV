mod login;
mod register;
mod files;

use clap::{Arg, Command};
use std::env;
use std::process;
use std::io::Write; // Add this line


fn main() {
    let matches = Command::new("rnv")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A CLI tool for managing .env files")
        .subcommand(
            Command::new("init")
                .about("Initialize by searching for .env files and saving contents to content.json"),
        )
        .subcommand(
            Command::new("login")
                .about("Log in with username, password, and hexkey"),
        )
        .subcommand(
            Command::new("register")
                .about("Register with username, password, and hexkey"),
        )
        .get_matches();

    // Handle the "init" subcommand
    if let Some(_) = matches.subcommand_matches("init") {
        match env::current_dir() {
            Ok(path) => {
                println!("Current directory: {}", path.display());
                let env_files = files::find_and_read_env_files(&path);
                files::save_to_json("content.json", &env_files);
            }
            Err(e) => {
                eprintln!("Error getting current directory: {}", e);
                process::exit(1);
            }
        }
    }
    // Handle the "login" subcommand
    else if let Some(_) = matches.subcommand_matches("login") {
        let username = prompt_for_input("Enter your username: ");
        let password = prompt_for_secure_input("Enter your password: ");
        let hexkey = prompt_for_secure_input("Enter your hexkey: ");
        login::login(&username, &password, &hexkey);
    }
    // Handle the "register" subcommand
    else if let Some(_) = matches.subcommand_matches("register") {
        let username = prompt_for_input("Enter your username: ");
        let password = prompt_for_secure_input("Enter your password: ");
        let hexkey = prompt_for_secure_input("Enter your hexkey: ");
        register::register(&username, &password, &hexkey);
    } else {
        println!("Usage: rnv <command>");
    }
}

// Prompt for user input
fn prompt_for_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap(); // Ensure the prompt is displayed before input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Prompt for secure input (passwords/hexkeys)
fn prompt_for_secure_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let input = rpassword::read_password().expect("Failed to read password/hexkey");
    input.trim().to_string()
}
