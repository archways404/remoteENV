use clap::{Arg, Command};
use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use walkdir::WalkDir;
use std::fs::File;
use std::io::Write;

// Structure to hold the information for each .env file
#[derive(Serialize)]
struct EnvFile {
    name: String,
    path: String,
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
        .get_matches();

    // Check if the "init" subcommand is invoked
    if let Some(_) = matches.subcommand_matches("init") {
        // Get the current working directory
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
    } else {
        println!("Usage: rnv init");
    }
}

// Function to find and read all .env files, returning a Vec<EnvFile>
fn find_and_read_env_files(dir: &Path) -> Vec<EnvFile> {
    let mut env_files = Vec::new();

    // Use walkdir to recursively go through all directories and files
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            // Check if the file is named exactly ".env" or has a ".env" extension
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name == ".env" || file_name.starts_with(".env") {
                // Read the file contents
                match fs::read_to_string(path) {
                    Ok(contents) => {
                        let env_file = EnvFile {
                            name: file_name.to_string(),
                            path: path.display().to_string(),
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
