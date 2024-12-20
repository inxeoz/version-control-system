use sha1::{Digest, Sha1};
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};
use std::{env, fs};

fn write_to_file(file_path: &str, content: &str) {
    let path = Path::new(file_path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|_| eprintln!("Failed to create folder '{}'.", parent.display()));
    }

    let mut file = OpenOptions::new().write(true).create(true).append(true).open(path)
        .unwrap_or_else(|_| panic!("Failed to open file '{}'.", file_path));

    writeln!(file, "{}", content).expect("Failed to write on file");
}

fn print_current_path() {
    env::current_dir().map_or_else(
        |e| eprintln!("Error getting current path: {}", e),
        |path| println!("Current path: {}", path.display()),
    );
}

fn create_folder_if_not_exists(folder_path_from_root: &str) {
    let folder_path = env::current_dir().expect("Error getting current directory").join(folder_path_from_root);
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path).expect("Cannot create folder.");
        println!("Folder '{}' created successfully.", folder_path.display());
    }
}

fn create_file_if_not_exists(file_name: &str, relative_path: &str) {
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

fn create_hierarchy_of_folders(folder_path: &str) {
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let folder_name = path.file_name().unwrap().to_str().unwrap().to_string();
                write_to_file("vcs/config/struct.txt", &format!("[{}]{{\n", folder_name));
                create_hierarchy_of_folders(path.to_str().unwrap());
                write_to_file("vcs/config/struct.txt", "}\n");
            } else {
                create_blob_file_and_save(path.display().to_string());
                let file_name = path.file_name().unwrap().to_str().unwrap();
                write_to_file("vcs/config/struct.txt", file_name);
            }
        }
    }
}

fn init() {
    create_folder_if_not_exists("vcs");
    create_folder_if_not_exists("vcs/objects");
    create_folder_if_not_exists("vcs/config");
    create_file_if_not_exists("config.txt", "vcs/config");
    create_file_if_not_exists("struct.txt", "vcs/config");
    create_hierarchy_of_folders("test_fold");

    println!("VCS initialized successfully.");
}

fn get_hash(filename: &str) -> String {
    let content = read_from_file(filename);
    get_hashstring_by_string(&content)
}

fn get_hashstring_by_string(content: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content.as_bytes());
    hash_array_to_string(&hasher.finalize())
}

fn hash_array_to_string(hash_array: &[u8]) -> String {
    hash_array.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn read_from_file(file_path_from_root: &str) -> String {
    let mut file = File::open(file_path_from_root).expect("Failed to open file.");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file.");
    content
}

fn create_blob_file_and_save(filename: String) {
    let file_content = read_from_file(&filename);
    println!("File content: {}", file_content);
    let hashstring = get_hash(&filename);
    let hashname = &hashstring[0..6];

    let blob_filename = format!("vcs/objects/{}/{}.blob", hashname, hashname);
    let foldername = format!("vcs/objects/{}", hashname);
    create_folder_if_not_exists(&foldername);

    let mut blob_file = File::create(&blob_filename).expect("Cannot create blob file");
    blob_file.write_all(file_content.as_bytes()).expect("Cannot write content to blob file");

    println!("Blob file created successfully: {}", blob_filename);
}

fn main() {
    init();
}
