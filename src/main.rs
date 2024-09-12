use clap::{Arg, Command};
use serde::Serialize;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::fs::File;
use walkdir::WalkDir;
use rpassword::read_password;

// Structure to hold the information for each .env file
#[derive(Serialize)]
struct EnvFile {
    file_name: String,
    relative_file_path: String,
    project_name: String,
    project_path: String,
    contents: String,
}

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
                let env_files = find_and_read_env_files(&path);
                save_to_json("content.json", &env_files);
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
        login(&username, &password, &hexkey);
    }
    // Handle the "register" subcommand
    else if let Some(_) = matches.subcommand_matches("register") {
        let username = prompt_for_input("Enter your username: ");
        let password = prompt_for_secure_input("Enter your password: ");
        let hexkey = prompt_for_secure_input("Enter your hexkey: ");
        register(&username, &password, &hexkey);
    } else {
        println!("Usage: rnv <command>");
    }
}

// Function to find and read all .env files, returning a Vec<EnvFile>
fn find_and_read_env_files(dir: &Path) -> Vec<EnvFile> {
    let mut env_files = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name == ".env" || file_name.starts_with(".env") {
                match fs::read_to_string(path) {
                    Ok(contents) => {
                        let project_path = path.parent().unwrap_or(dir);
                        let project_name = project_path.file_name().unwrap().to_str().unwrap().to_string();

                        let relative_file_path = normalize_path(path);
                        let project_path_str = normalize_path(project_path);

                        let env_file = EnvFile {
                            file_name: file_name.to_string(),
                            relative_file_path,
                            project_name,
                            project_path: project_path_str,
                            contents: contents.trim().to_string(),
                        };
                        env_files.push(env_file);
                    }
                    Err(e) => {
                        eprintln!("Failed to read {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    env_files
}

// Function to save the Vec<EnvFile> to a JSON file
fn save_to_json(filename: &str, data: &[EnvFile]) {
    match File::create(filename) {
        Ok(mut file) => {
            match serde_json::to_string_pretty(&data) {
                Ok(json_data) => {
                    if let Err(e) = file.write_all(json_data.as_bytes()) {
                        eprintln!("Failed to write to {}: {}", filename, e);
                    } else {
                        println!("Data successfully saved to {}", filename);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to serialize data: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create file {}: {}", filename, e);
        }
    }
}

// Function to normalize the path and convert to Unix-style format
fn normalize_path(path: &Path) -> String {
    let path_str = path.display().to_string();
    
    // On Windows, replace backslashes with forward slashes and convert to Unix-style paths
    #[cfg(windows)]
    let normalized = path_str.replace("\\", "/").replace("C:", "/C");

    // For Unix-based systems, keep the path as it is
    #[cfg(not(windows))]
    let normalized = path_str;

    normalized
}

// Prompt for user input
fn prompt_for_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();  // Ensure the prompt is displayed before input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Prompt for secure input (passwords/hexkeys)
fn prompt_for_secure_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let input = read_password().expect("Failed to read password/hexkey");
    input.trim().to_string()
}

// Placeholder login function
fn login(username: &str, password: &str, hexkey: &str) {
    println!("Logging in with username: {}, password: [REDACTED], hexkey: [REDACTED]", username);
    // Implement actual login logic here
}

// Placeholder register function
fn register(username: &str, password: &str, hexkey: &str) {
    println!("Registering with username: {}, password: [REDACTED], hexkey: [REDACTED]", username);
    // Implement actual register logic here
}
