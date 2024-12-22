use serde_json::{json, Map, Value};
use sha1::{Digest, Sha1};
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs, path};
use crate::snapshot;

pub fn write_to_file(file_path: &str, content: &str) {
    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|_| eprintln!("Failed to create folder '{}'.", parent.display()));
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap_or_else(|_| panic!("Failed to open file '{}'.", file_path));
    write!(file, "{}", content).expect("Failed to write on file");
}

pub fn get_current_path() -> PathBuf {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    // current_dir.join("version_control_system")
    current_dir
}

pub fn create_folder_if_not_exists(folder_path_from_root: &str) {
    let folder_path = get_current_path().join(folder_path_from_root);
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path).expect("Cannot create folder.");
        println!("Folder '{}' created successfully.", folder_path.display());
    }
}

pub fn create_file_if_not_exists(file_name: &str, relative_path: &str) {
    let folder_path = get_current_path().join(relative_path);
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path).expect("Failed to create folder.");
    }

    let file_path = folder_path.join(file_name);
    if !file_path.exists() {
        File::create(&file_path).expect("Failed to create file.");
        println!("File '{}' created successfully.", file_path.display());
    }
}

pub fn delete_folder_if_exists(folder_path_from_root: &str) {
    let folder_path = get_current_path().join(folder_path_from_root);
    if folder_path.exists() {
        fs::remove_dir_all(&folder_path).expect("Failed to delete folder.");
        println!("Folder '{}' deleted successfully.", folder_path.display());
    } else {
        println!("Folder '{}' does not exist.", folder_path.display());
    }
}

pub fn delete_file_if_exists(file_name: &str, relative_path: &str) {
    let file_path = get_current_path().join(relative_path).join(file_name);
    if file_path.exists() {
        fs::remove_file(&file_path).expect("Failed to delete file.");
        println!("File '{}' deleted successfully.", file_path.display());
    } else {
        println!("File '{}' does not exist.", file_path.display());
    }
}


pub fn read_file_and_get_hash(file_path_from_root: &str) -> Result<(Vec<u8>, String), String> {
    // Open and read the file content as raw bytes
    let mut file =
        File::open(file_path_from_root).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Compute the SHA1 hash
    let mut hasher = Sha1::new();
    hasher.update(&content);
    let hash_array = hasher.finalize();

    // Convert the hash to a hexadecimal string
    let hash_string = hash_array
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    Ok((content, hash_string))
}

// Create blob file and save raw bytes


