use crate::read_write::{create_file_if_not_exists, create_folder_if_not_exists, read_file_and_get_hash};
use serde_json;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::controller;


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

pub fn is_ignored(path: &Path, ignore_list: &HashSet<String> ) -> bool {
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    ignore_list.contains(&file_name)
}

pub fn create_blob_file_and_save(filename_from_root: &str, details: &controller::ConfigDetails) -> String {
    let (content, hashstring) =
        read_file_and_get_hash(&filename_from_root).expect("Failed to read file.");

    let foldername = format!("{}/{}", details.version_control_system_objects_folder, hashstring);
    let blob_filename = format!("{}/{}.blob", foldername, hashstring);

    create_folder_if_not_exists(&foldername); // Assuming `create_folder_if_not_exists` creates the folder

    let mut blob_file = File::create(&blob_filename).expect("Cannot create blob file");
    blob_file
        .write_all(&content)
        .expect("Cannot write content to blob file");
    hashstring
}

pub fn compare_files(original_file: &str, new_file: &str) -> serde_json::Value {
    // Read the contents of both files
    let original_content = fs::read(original_file).expect("Failed to read original file");
    let new_content = fs::read(new_file).expect("Failed to read new file");

    // Split the content into lines (or use bytes for binary data)
    let original_lines: Vec<_> = original_content.split(|&b| b == b'\n').collect();
    let new_lines: Vec<_> = new_content.split(|&b| b == b'\n').collect();

    let mut commands = vec![];

    let mut original_index = 0;
    let mut new_index = 0;

    while original_index < original_lines.len() && new_index < new_lines.len() {
        if original_lines[original_index] == new_lines[new_index] {
            // If the lines are identical, use "copy"
            commands.push(serde_json::json!({ "command": "copy", "line": String::from_utf8_lossy(original_lines[original_index]) }));
            original_index += 1;
            new_index += 1;
        } else {
            // If the lines differ, use "add" for the new content
            commands.push(serde_json::json!({ "command": "add", "line": String::from_utf8_lossy(new_lines[new_index]) }));
            new_index += 1;
        }
    }

    // Handle any remaining lines in the new file (add commands)
    while new_index < new_lines.len() {
        commands.push(serde_json::json!({ "command": "add", "line": String::from_utf8_lossy(new_lines[new_index]) }));
        new_index += 1;
    }

    // Handle any remaining lines in the original file (not needed, as they're "ignored")

    // Return the JSON map
    serde_json::json!(commands)
}

pub fn traverse_and_update(
    serde_json: Option<&serde_json::Value>,
    current_path: PathBuf,
    details: &controller::ConfigDetails
) -> serde_json::Map<String, serde_json::Value> {
    let mut folder_structure = serde_json::Map::new();
    let mut ignore_list = HashSet::<String>::new();
    let mut whole_list = get_current_dir_entities(current_path.clone());
    let mut actual_list = HashSet::<String>::new();

    // Check if `.vcs.ignore` exists in the current folder and load the ignore list
    let ignore_file_path = current_path.join(&details.ignore_file_name);
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
                let file_hash = create_blob_file_and_save(&*new_path.display().to_string(), &details);
                if previous_list_key.contains(&entity) && previous_list_map.get(&entity).unwrap().as_str().unwrap() != file_hash {
                    let previous_file_hash = previous_list_map.get(&entity).unwrap().as_str().unwrap();
                    //TODO when previous_file_hash has chnaged
                   // let diff = compare_files(&format!("{}/{}/{}", details.version_control_system_objects_folder,  previous_file_hash, previous_file_hash), new_path.to_str().unwrap());
                   //
                   //  create_file_if_not_exists("diff.txt", format!("{}/{}", details.version_control_system_objects_folder, file_hash).as_str() );
                   //  fs::write( format!("{}/{}/diff.txt", file_hash).as_str(), diff.to_string()).expect("Unable to write file");

                    folder_structure.insert(entity, serde_json::Value::String(file_hash));
                }else {

                    create_file_if_not_exists("diff.txt", format!("{}/{}", details.version_control_system_objects_folder, file_hash).as_str() );
                    folder_structure.insert(entity, serde_json::Value::String(file_hash));
                }

            } else if new_path.is_dir() {

                if previous_list_key.contains(&entity) {
                    let nested_folder_structure = traverse_and_update(previous_list_map.get(&entity), new_path.clone(),  details);
                    folder_structure.insert(entity, serde_json::Value::Object(nested_folder_structure));
                }else {
                    let nested_folder_structure = traverse( new_path.clone(), details);
                    folder_structure.insert(entity,serde_json:: Value::Object(nested_folder_structure));

                }

            }
        }
    }
    folder_structure
}

pub fn traverse(current_path: PathBuf, details: &controller::ConfigDetails) -> serde_json::Map<String, serde_json::Value> {

    let mut folder_structure = serde_json::Map::new();
    let mut ignore_list = HashSet::<String>::new();
    let mut whole_list = get_current_dir_entities(current_path.clone());
    let mut actual_list = HashSet::<String>::new();

    // Check if `.vcs.ignore` exists in the current folder and load the ignore list
    let ignore_file_path = current_path.join(&details.ignore_file_name);

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
                let file_hash = create_blob_file_and_save(&*new_path.display().to_string(), details);
                folder_structure.insert(entity, serde_json::Value::String(file_hash));
            } else if new_path.is_dir() {
                // If the entity is a folder, recursively traverse and update the folder structure
                let nested_folder_structure = traverse(new_path, details);
                folder_structure.insert(entity, serde_json::Value::Object(nested_folder_structure));
            }
        }
    }
    folder_structure
}


pub fn read_hierarchy_from_file(file_path: &str) -> serde_json::Value {
    let file_content = fs::read_to_string(file_path).expect("Failed to read JSON file");
    serde_json::from_str(&file_content).expect("Failed to parse JSON file")
}
/// Recursively prints the JSON hierarchy in a readable format
pub fn print_hierarchy(json_value: &serde_json::Value, indent: usize) {
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
