use serde::Serialize;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use walkdir::WalkDir;

#[derive(Serialize)]
pub struct EnvFile {
    pub file_name: String,
    pub relative_file_path: String,
    pub project_name: String,
    pub project_path: String,
    pub contents: String,
}

pub fn find_and_read_env_files(dir: &Path) -> Vec<EnvFile> {
    let mut env_files = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name == ".env" || file_name.starts_with(".env") {
                match fs::read_to_string(path) {
                    Ok(contents) => {
                        let project_path = path.parent().unwrap_or(dir);
                        let project_name =
                            project_path.file_name().unwrap().to_str().unwrap().to_string();

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

pub fn save_to_json(filename: &str, data: &[EnvFile]) {
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

pub fn normalize_path(path: &Path) -> String {
    let path_str = path.display().to_string();

    #[cfg(windows)]
    let normalized = path_str.replace("\\", "/").replace("C:", "/C");

    #[cfg(not(windows))]
    let normalized = path_str;

    normalized
}
