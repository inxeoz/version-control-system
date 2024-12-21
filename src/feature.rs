
use sha1::{Digest, Sha1};
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};
use std::{env, fs};
use std::collections::{HashMap, VecDeque};
use serde_json::{json, Map, Value};
use std::io::{Read, Write};


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
    hashstring
}


/// Helper function to read a `.vcs.ignore` file and return a Vec of strings to ignore.
fn read_ignore_file(ignore_file_path: &str) -> Vec<String> {
    let mut ignore_list = Vec::new();
    if let Ok(contents) = fs::read_to_string(ignore_file_path) {
        for line in contents.lines() {
            if !line.trim().is_empty() {
                ignore_list.push(line.trim().to_string());
            }
        }
    }
    ignore_list
}

/// Checks if the given path (file or folder) should be ignored based on `.vcs.ignore`.
fn is_ignored(path: &Path, ignore_list: &Vec<String>) -> bool {
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    ignore_list.contains(&file_name)
}
/// Recursively create the folder hierarchy and add file hashes, considering ignored files and directories.
pub fn create_hierarchy_of_folders(folder_path: &str) -> HashMap<String, Value> {
    let mut folder_structure = HashMap::new();

    // Initialize the ignore list
    let mut ignore_list = Vec::<String>::new();

    // Check if `.vcs.ignore` exists in the current folder
    let ignore_file_path = Path::new(folder_path).join(".vcs.ignore");
    if ignore_file_path.exists() {
        // Load ignore list if `.vcs.ignore` file is found
        ignore_list = read_ignore_file(ignore_file_path.to_str().unwrap());
    }

    // First pass: collect subdirectories and files
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.filter_map(Result::ok) {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                // If the directory is not in the ignore list, enqueue it for processing
                if !is_ignored(&entry_path, &ignore_list) {
                    let folder_name = entry_path.file_name().unwrap().to_str().unwrap().to_string();
                    let subfolder_structure = create_hierarchy_of_folders(entry_path.to_str().unwrap());

                    // Convert HashMap to serde_json::Map
                    let map: Map<String, Value> = subfolder_structure.into_iter().collect();
                    folder_structure.insert(folder_name, Value::Object(map)); // Insert into folder structure
                }
            } else {
                // Process files and add their hash directly, if not ignored
                let file_name = entry_path.file_name().unwrap().to_str().unwrap().to_string();
                if !is_ignored(&entry_path, &ignore_list) {
                    let hashstring = create_blob_file_and_save(entry_path.display().to_string());
                    folder_structure.insert(file_name, Value::String(hashstring));
                }
            }
        }
    }

    return folder_structure;
}

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



