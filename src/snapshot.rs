use crate::read_write::create_blob_file_and_save;
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub fn read_ignore_file(ignore_file_path: &str) -> HashSet<String> {
    let mut ignore_list = HashSet::<String>::new();
    if let Ok(contents) = fs::read_to_string(ignore_file_path) {
        for line in contents.lines() {
            if !line.trim().is_empty() {
                if line.starts_with("/") && line.len() > 1 {
                    ignore_list.insert(line[1..].to_string());
                } else if line.starts_with("#") {
                    continue;
                } else {
                    ignore_list.insert(line.trim().to_string());
                }
            }
        }
    }
    ignore_list
}

pub fn is_ignored(path: &Path, ignore_list: &HashSet<String>, is_folder: bool) -> bool {
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    ignore_list.contains(&file_name)
}
pub fn traverse_and_update(
    serde_json: Option<&Value>,
    current_path: PathBuf,
) -> Map<String, Value> {
    let mut folder_structure = Map::new();
    let mut ignore_list = HashSet::<String>::new();
    let mut whole_list = get_current_dir_entities(current_path.clone());
    let mut actual_list = HashSet::<String>::new();

    // Check if `.vcs.ignore` exists in the current folder and load the ignore list
    let ignore_file_path = current_path.join(".vcs.ignore");
    println!("current path {:?}", current_path.display());
    ignore_list = read_ignore_file(ignore_file_path.to_str().unwrap());
    actual_list = whole_list.difference(&ignore_list).cloned().collect();
    println!("whole list {:?}", whole_list);
    println!("ignore list {:?}", ignore_list);
    println!("actual list {:?}\n\n", actual_list);


    let mut previous_list_map = serde_json.expect("JSON data is None").as_object().expect("cant convert to object");
    let mut previous_list_key: HashSet<String> =
        previous_list_map.keys().map(|x| x.to_string()).collect();




    if actual_list.is_empty() {
        println!("Nothing to add at path {}", current_path.display());
    } else {
        // Add new files/folders
        for entity in actual_list {
            let mut new_path = current_path.clone();
            new_path.push(&entity);
            println!("Adding new entity at path {}", new_path.display());

            // If the entity is a file, add it to the current JSON structure
            if new_path.is_file() {
                // You would typically hash the file content here and add it to the JSON structure
                let file_hash = create_blob_file_and_save(new_path.display().to_string());
                if previous_list_key.contains(&entity) && previous_list_map.get(&entity).unwrap().as_str().unwrap() != file_hash {
                    //TODO when previous_file_hash has chnaged
                    folder_structure.insert(entity, Value::String(file_hash));
                }else {
                    folder_structure.insert(entity, Value::String(file_hash));
                }

            } else if new_path.is_dir() {

                if previous_list_key.contains(&entity) {
                    let nested_folder_structure = traverse_and_update(previous_list_map.get(&entity), new_path.clone());
                    folder_structure.insert(entity, Value::Object(nested_folder_structure));
                }else {
                    let nested_folder_structure = traverse( new_path.clone());
                    folder_structure.insert(entity, Value::Object(nested_folder_structure));

                }

            }
        }
    }
    folder_structure
}

pub fn traverse(current_path: PathBuf) -> Map<String, Value> {

    let mut folder_structure = Map::new();
    let mut ignore_list = HashSet::<String>::new();
    let mut whole_list = get_current_dir_entities(current_path.clone());
    let mut actual_list = HashSet::<String>::new();

    // Check if `.vcs.ignore` exists in the current folder and load the ignore list
    let ignore_file_path = current_path.join(".vcs.ignore");

    println!("traverse current path {:?}", current_path.display());
    ignore_list = read_ignore_file(ignore_file_path.to_str().unwrap());
    actual_list = whole_list.difference(&ignore_list).cloned().collect();

    println!("whole list {:?}", whole_list);
    println!("ignore list {:?}", ignore_list);
    println!("actual list {:?}\n\n", actual_list);

    if actual_list.is_empty() {
        println!("Nothing to add at path {}", current_path.display());
    } else {
        // Add new files/folders
        for entity in actual_list {
            let mut new_path = current_path.clone();
            new_path.push(&entity);
            println!("Adding new entity at path {}", new_path.display());

            // If the entity is a file, add it to the current JSON structure
            if new_path.is_file() {
                // You would typically hash the file content here and add it to the JSON structure
                let file_hash = create_blob_file_and_save(new_path.display().to_string());
                folder_structure.insert(entity, Value::String(file_hash));
            } else if new_path.is_dir() {
                // If the entity is a folder, recursively traverse and update the folder structure
                let nested_folder_structure = traverse(new_path);
                folder_structure.insert(entity, Value::Object(nested_folder_structure));
            }
        }
    }
    folder_structure
}

pub fn create_hierarchy_of_folders(folder_path: &str) -> HashMap<String, Value> {
    let mut folder_structure = HashMap::new();

    // Initialize the ignore list
    let mut ignore_list = HashSet::<String>::new();

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

pub fn read_hierarchy_from_file(file_path: &str) -> Value {
    let file_content = fs::read_to_string(file_path).expect("Failed to read JSON file");
    serde_json::from_str(&file_content).expect("Failed to parse JSON file")
}
pub fn save_hierarchy_to_file(folder_path: &str, output_file: &str) {
    let hierarchy = create_hierarchy_of_folders(folder_path);
    let json_string = serde_json::to_string_pretty(&hierarchy).expect("Failed to serialize JSON");
    fs::write(output_file, json_string).expect("Failed to write JSON to file");
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

pub fn get_current_dir_entities(current_path: PathBuf) -> HashSet<String> {
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
