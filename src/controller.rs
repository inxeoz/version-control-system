use std::fs;
use serde_json::Value;
use crate::read_write::{create_file_if_not_exists, create_folder_if_not_exists, get_current_path, read_file_and_get_hash};
use crate::snapshot::{compare_files, save_hierarchy_to_file, traverse, traverse_and_update};

pub fn init() {
    create_folder_if_not_exists("version_control_system");
    create_folder_if_not_exists("version_control_system/objects");
    create_folder_if_not_exists("version_control_system/config");
    create_file_if_not_exists("config.txt", "version_control_system/config");
    create_file_if_not_exists("struct.json", "version_control_system/config");
    create_hierarchy_from_dir_and_save();
}



pub fn read_hierarchy_from_file_and_update() {

    let json_data = fs::read_to_string("version_control_system/config/struct.json")
        .expect("Failed to read JSON");
    let parsed_json: Value = serde_json::from_str(&*json_data).expect("Failed to parse JSON");
    println!("\n\n");

    let new_json =traverse_and_update(Option::from(&parsed_json), get_current_path().join("test_fold"));

    let new_serde_json = serde_json::to_string_pretty(&new_json).expect("Failed to serialize JSON");
    create_file_if_not_exists("struct2.json", "version_control_system/config");
    fs::write("version_control_system/config/struct2.json", new_serde_json).expect("Failed to write JSON to file");
}

pub fn create_hierarchy_from_dir_and_save() {
    let new_json =traverse(get_current_path().join("test_fold"));
    let new_serde_json = serde_json::to_string_pretty(&new_json).expect("Failed to serialize JSON");
    create_file_if_not_exists("struct.json", "version_control_system/config");
    fs::write("version_control_system/config/struct.json", new_serde_json).expect("Failed to write JSON to file");
}


pub fn compare() {

    let result = compare_files("test_fold/f5.txt", "test_fold/f6.txt");
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

