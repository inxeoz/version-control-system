use serde_json::{json, Map, Value};
use sha1::{Digest, Sha1};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs, path};

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
pub fn create_blob_file_and_save(filename_from_root: String) -> String {
    let (content, hashstring) =
        read_file_and_get_hash(&filename_from_root).expect("Failed to read file.");
    let hashname = &hashstring[0..6];

    let blob_filename = format!(
        "version_control_system/objects/{}/{}.blob",
        hashname, hashstring
    );
    let foldername = format!("version_control_system/objects/{}", hashname);
    create_folder_if_not_exists(&foldername); // Assuming `create_folder_if_not_exists` creates the folder

    let mut blob_file = File::create(&blob_filename).expect("Cannot create blob file");
    blob_file
        .write_all(&content)
        .expect("Cannot write content to blob file");
    hashstring
}

/// Helper function to read a `.version_control_system.ignore` file and return a Vec of strings to ignore.
fn read_ignore_file(ignore_file_path: &str) -> HashSet<String> {
    let mut ignore_list = HashSet::<String>::new();
    if let Ok(contents) = fs::read_to_string(ignore_file_path) {
        for line in contents.lines() {
            if !line.trim().is_empty() {
                println!("Ignoring file: {}", line);
                ignore_list.insert(line.trim().to_string());
            }
        }
    }
    ignore_list
}

/// Checks if the given path (file or folder) should be ignored based on `.version_control_system.ignore`.
fn is_ignored(path: &Path, ignore_list: &HashSet<String>, is_folder: bool) -> bool {
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    if is_folder {
        return ignore_list.contains(&format!("/{}", file_name).to_string());
    }

    ignore_list.contains(&file_name)
}
/// Recursively create the folder hierarchy and add file hashes, considering ignored files and directories.
pub fn create_hierarchy_of_folders(folder_path: &str) -> HashMap<String, Value> {
    println!("####################### {:?}", folder_path);
    let mut folder_structure = HashMap::new();

    // Initialize the ignore list
    let mut ignore_list =HashSet::<String>::new();

    // Check if `.version_control_system.ignore` exists in the current folder
    let ignore_file_path = Path::new(folder_path).join(".vcs.ignore");
    if ignore_file_path.exists() {
        // Load ignore list if `.version_control_system.ignore` file is found
        ignore_list = read_ignore_file(ignore_file_path.to_str().unwrap());
    }

    // First pass: collect subdirectories and files
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.filter_map(Result::ok) {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                // If the directory is not in the ignore list, enqueue it for processing
                if !is_ignored(&entry_path, &ignore_list, true) {
                    let folder_name = entry_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    let subfolder_structure =
                        create_hierarchy_of_folders(entry_path.to_str().unwrap());

                    // Convert HashMap to serde_json::Map
                    let map: Map<String, Value> = subfolder_structure.into_iter().collect();
                    folder_structure.insert(folder_name, Value::Object(map)); // Insert into folder structure
                }
            } else {
                // Process files and add their hash directly, if not ignored
                let file_name = entry_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                if !is_ignored(&entry_path, &ignore_list, false) {
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

pub fn get_current_dir_entities(current_path: PathBuf) -> HashSet<String>{
    let mut entity_list = HashSet::<String>::new();
    println!("####################### {:?}", current_path.display());
    if let Ok(entries) = fs::read_dir(current_path) {
        for entry in entries.filter_map(Result::ok) {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                let folder_name = entry_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                entity_list.insert(folder_name);
            } else {
                let file_name = entry_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                entity_list.insert(file_name);
            }
        }
    }

    entity_list
}

pub fn traverse_json(json: &Value, current_path: PathBuf) {
    match json {
        Value::Object(map) => {
            // Check if the object is empty
            if map.is_empty() {
                println!("{}: {{}}", current_path.display()); // Print empty object
            }

            // Traverse each key-value pair in the object
            for (key, value) in map {
                let mut new_path = current_path.clone();
                new_path.push(key); // Append the key to the current path
                traverse_json(value, new_path); // Recurse with the updated path
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                let mut new_path = current_path.clone();
                new_path.push(format!("[{}]", index)); // Append array index to the path
                traverse_json(value, new_path); // Recurse with the updated path
            }
        }
        _ => {
            println!("{}: {}", current_path.display(), json); // Print the path and value
        }
    }
}


pub fn traverse_json3(json: &Value, current_path: PathBuf) {
    let mut ignore_list = HashSet::<String>::new();
    let mut entity_list = get_current_dir_entities(current_path.clone());

    // Check if `.version_control_system.ignore` exists in the current folder and load the ignore list
    let ignore_file_path = current_path.join(".vcs.ignore");
    if ignore_file_path.exists() {
        ignore_list = read_ignore_file(ignore_file_path.to_str().unwrap());

        println!("{:?}", entity_list);
        println!("{:?}", ignore_list);
        println!("\n\n\n");
    }

    match json {
        Value::Object(map) => {
            // Traverse the object (folder) content
            for (key, value) in map {
                println!("key: {}", key);
                if !ignore_list.contains(&key.to_string()) {

                    let mut new_path = current_path.clone();
                    new_path.push(key);
                    println!("{} ------>", new_path.display());

                    if entity_list.contains(&key.to_string()) {
                        println!("key: {}", key);
                    }
                }
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                println!("value : {}", value);
                let mut new_path = current_path.clone();
                new_path.push(format!("[{}]", index)); // Append array index to the path

                traverse_json3(value, new_path); // Recurse with the updated path
            }
        }
        _ => {
            if !is_ignored(&current_path, &ignore_list, false) {
                println!("{}: {}", current_path.display(), json); // Print file path and content
            }
        }
    }
}

pub fn main() {
    let json_data = fs::read_to_string("version_control_system/config/struct.json")
        .expect("Failed to read JSON");
    let parsed_json: Value = serde_json::from_str(&*json_data).expect("Failed to parse JSON");
    println!("\n\n");

    traverse_json3(&parsed_json, get_current_path().join("test_fold"));
}
