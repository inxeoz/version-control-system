
use sha1::{Digest, Sha1};
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};
use std::{env, fs};
use std::collections::HashMap;
use serde_json::{json, Value};

pub fn write_to_file(file_path: &str, content: &str) {
    let path = Path::new(file_path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|_| eprintln!("Failed to create folder '{}'.", parent.display()));
    }

    let mut file = OpenOptions::new().write(true).create(true).append(true).open(path)
        .unwrap_or_else(|_| panic!("Failed to open file '{}'.", file_path));

    write!(file, "{}", content).expect("Failed to write on file");
}

pub fn print_current_path() {
    env::current_dir().map_or_else(
        |e| eprintln!("Error getting current path: {}", e),
        |path| println!("Current path: {}", path.display()),
    );
}

pub fn create_folder_if_not_exists(folder_path_from_root: &str) {
    let folder_path = env::current_dir().expect("Error getting current directory").join(folder_path_from_root);
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path).expect("Cannot create folder.");
        println!("Folder '{}' created successfully.", folder_path.display());
    }
}

pub fn create_file_if_not_exists(file_name: &str, relative_path: &str) {
    let folder_path = env::current_dir().expect("Error getting current directory").join(relative_path);
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path).expect("Failed to create folder.");
    }

    let file_path = folder_path.join(file_name);
    if !file_path.exists() {
        File::create(&file_path).expect("Failed to create file.");
        println!("File '{}' created successfully.", file_path.display());
    }
}





pub fn read_file_and_get_hash(file_path_from_root: &str) -> Result<(Vec<u8>, String), String> {
    // Open and read the file content as raw bytes
    let mut file = File::open(file_path_from_root)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Compute the SHA1 hash
    let mut hasher = Sha1::new();
    hasher.update(&content);
    let hash_array = hasher.finalize();

    // Convert the hash to a hexadecimal string
    let hash_string = hash_array.iter().map(|byte| format!("{:02x}", byte)).collect();

    Ok((content, hash_string))
}


// Create blob file and save raw bytes
pub fn create_blob_file_and_save(filename_from_root: String) -> String{
    let (content, hashstring) = read_file_and_get_hash(&filename_from_root).expect("Failed to read file.");
    let hashname = &hashstring[0..6];

    let blob_filename = format!("vcs/objects/{}/{}.blob", hashname, hashstring);
    let foldername = format!("vcs/objects/{}", hashname);
    create_folder_if_not_exists(&foldername); // Assuming `create_folder_if_not_exists` creates the folder

    let mut blob_file = File::create(&blob_filename).expect("Cannot create blob file");
    blob_file
        .write_all(&content)
        .expect("Cannot write content to blob file");

    println!("Blob file created successfully: {}", blob_filename);

    hashstring
}


/// Reads a JSON file and deserializes it into a `Value`
///
pub fn create_hierarchy_of_folders(folder_path: &str) -> Value {
    let mut folder_structure = HashMap::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let folder_name = path.file_name().unwrap().to_str().unwrap().to_string();
                // Recursively process subfolder
                folder_structure.insert(
                    folder_name,
                    create_hierarchy_of_folders(path.to_str().unwrap()),
                );
            } else {
                let hashstring = create_blob_file_and_save(path.display().to_string());
                let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                // Add file with hash to the current folder
                folder_structure.insert(file_name, Value::String(hashstring));
            }
        }
    }

    json!(folder_structure)
}

/// Saves the folder hierarchy as a JSON file
pub fn save_hierarchy_to_file(folder_path: &str, output_file: &str) {
    let hierarchy = create_hierarchy_of_folders(folder_path);
    let json_string = serde_json::to_string_pretty(&hierarchy).expect("Failed to serialize JSON");
    fs::write(output_file, json_string).expect("Failed to write JSON to file");
}

pub fn read_hierarchy_from_file(file_path: &str) -> Value {
    let file_content = fs::read_to_string(file_path).expect("Failed to read JSON file");
    serde_json::from_str(&file_content).expect("Failed to parse JSON file")
}

/// Recursively prints the JSON hierarchy in a readable format
pub fn print_hierarchy(json_value: &Value, indent: usize) {
    let indent_str = "  ".repeat(indent);

    if let Some(map) = json_value.as_object() {
        for (key, value) in map {
            if value.is_object() {
                println!("{}[{}]", indent_str, key);
                print_hierarchy(value, indent + 1);
            } else if value.is_string() {
                println!("{}[{} = {}]", indent_str, key, value.as_str().unwrap());
            }
        }
    }
}



