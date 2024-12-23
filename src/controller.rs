use crate::read_write::{
    create_file_if_not_exists, create_folder_if_not_exists, get_current_path,
    read_file_and_get_hash,
};
use crate::snapshot::{compare_files, traverse, traverse_and_update};
use serde_json::Value;
use std::fs;

pub struct ConfigDetails {
    pub version_control_system_folder: String,
    pub version_control_system_config_folder: String,
    pub version_control_system_objects_folder: String,
    pub struct_file_path: String,
    pub config_file_path: String,
    pub ignore_file_name: String,
    pub working_folder: String,
}

pub fn init() {
    let new_config_details = ConfigDetails {
        version_control_system_folder: "version_control_system".parse().unwrap(),
        version_control_system_config_folder: "version_control_system/config".parse().unwrap(),
        version_control_system_objects_folder: "version_control_system/objects".parse().unwrap(),
        struct_file_path: "version_control_system/config/struct.json".parse().unwrap(),
        config_file_path: "version_control_system/config/config.txt".parse().unwrap(),
        ignore_file_name: ".vcs.ignore".parse().unwrap(),
        working_folder: "test_fold".parse().unwrap(),
    };
    create_folder_if_not_exists(&*new_config_details.version_control_system_folder);
    create_folder_if_not_exists(&*new_config_details.version_control_system_objects_folder);
    create_folder_if_not_exists(&*new_config_details.version_control_system_config_folder);
    create_file_if_not_exists(
        "config.txt",
        &*new_config_details.version_control_system_config_folder,
    );
    create_file_if_not_exists(
        "struct.json",
        &*new_config_details.version_control_system_config_folder,
    );
    create_hierarchy_from_dir_and_save(&new_config_details);
 //   create_hierarchy_from_dir_and_save(&new_config_details);
}

pub fn read_hierarchy_from_file_and_update(details: &ConfigDetails) {
    let json_data =
        fs::read_to_string(details.struct_file_path.clone()).expect("Failed to read JSON");
    let parsed_json: Value = serde_json::from_str(&*json_data).expect("Failed to parse JSON");
    println!("\n\n");

    let new_json = traverse_and_update(
        Option::from(&parsed_json),
        get_current_path().join("test_fold"),
        &details
    );

    let new_serde_json = serde_json::to_string_pretty(&new_json).expect("Failed to serialize JSON");
    create_file_if_not_exists(
        "struct2.json",
        &*details.version_control_system_config_folder,
    );
    fs::write("version_control_system/config/struct2.json", new_serde_json)
        .expect("Failed to write JSON to file");
}

pub fn create_hierarchy_from_dir_and_save(details: &ConfigDetails) {
    let new_json = traverse(
        get_current_path().join("test_fold"),
        &details
    );
    let new_serde_json = serde_json::to_string_pretty(&new_json).expect("Failed to serialize JSON");
    create_file_if_not_exists(
        "struct.json",
        &*details.version_control_system_config_folder,
    );
    fs::write(&*details.struct_file_path, new_serde_json).expect("Failed to write JSON to file");
}

pub fn compare() {
    let result = compare_files("test_fold/f5.txt", "test_fold/f6.txt");
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
