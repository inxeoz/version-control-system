use std::{fs, io};
use std::fs::File;
use std::io::BufRead;

use sha1::{Sha1, Digest};
use crate::definition;
use crate::definition::{Object, VCS};
use std::io::prelude::*;

pub  fn create_blob_file(filename: &str) -> String {
    // Read the content of the file
    let file_content = read_file_content(filename);
    let hashstring_as_name = get_hash(filename);
    // Create the blob file
    let blob_filename = format!("vsc/objects/{}.blob", hashstring_as_name);
    let mut blob_file = File::create(&blob_filename).expect("cant create blob file");

    // Write file content to the blob file
    blob_file.write_all(file_content.as_bytes()).expect("cant write content to blob file");

    println!("Blob file created successfully: {}", blob_filename);
   blob_filename
}

pub fn read_file_content(filename: &str) -> String {
    let mut file = File::open(filename).expect("cant open file {filename}");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cant read file content {filename}");
   content
}

pub fn get_hash(filename: &str) -> String{
   let content_string = read_file_content(filename);
    let hashstring = get_hashstring_by_string(&content_string);
    hashstring
}

pub fn get_hashstring_by_file(file: File) -> String{
    
    let buf_reader = io::BufReader::new(file);
    let mut hasher = Sha1::new();
    for line in buf_reader.lines() {
        let line = line.expect("Failed to read line");
        hasher.update(line.as_bytes());
    }
    let result = hasher.finalize();
    let hashvaluehex = hash_array_to_string(&result);
    println!("Hash: {:?}", hash_array_to_string(&result));
    hashvaluehex
}


pub fn get_hashstring_by_string(content_string: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content_string.as_bytes());
    let result = hasher.finalize();
    hash_array_to_string(&result)
}


fn hash_array_to_string(hash_array: &[u8]) -> String {
    let mut hash_string = String::new();
    for &byte in hash_array {
        hash_string.push_str(&format!("{:02x}", byte));
    }
    hash_string
}



pub fn compare_files(oldfile: &str, newfile: &str) -> definition::Diff {
    let mut added_vec = Vec::<definition::Line>::new();
    let mut removed_vec = Vec::<definition::Line>::new();

    // Read Old File and New File line by line into vectors
    let old_content = read_file_by_lines(oldfile);
    let new_content = read_file_by_lines(newfile);

    // Determine the maximum length between the two files
    let max_lines = old_content.len().max(new_content.len());

    // Print added and removed lines along with line numbers
    println!("Changes between files:");
    for line_no in 0..max_lines {
        let old_line = old_content.get(line_no); // Line from the old file
        let new_line = new_content.get(line_no); // Line from the new file

        match (old_line, new_line) {
            (Some(old), Some(new)) if old != new => {
                println!("- Removed from line {}: {}", line_no + 1, old);
                removed_vec.push(definition::Line { line_number: line_no + 1, content: old.to_string() });
                println!("+ Added to line {}: {}", line_no + 1, new);
                added_vec.push(definition::Line { line_number: line_no + 1, content: new.to_string() });
            }
            (Some(old), None) => {
                println!("- Removed from line {}: {}", line_no + 1, old);
                removed_vec.push(definition::Line { line_number: line_no + 1, content: old.to_string() });
            }
            (None, Some(new)) => {
                println!("+ Added to line {}: {}", line_no + 1, new);
                added_vec.push(definition::Line { line_number: line_no + 1, content: new.to_string() });
            }
            _ => {} // No change
        }
    }

    let diff = definition::Diff {
        added: Some(added_vec),
        removed: Some(removed_vec),
    };
    diff
}


fn read_file_by_lines(filename: &str) -> Vec<String>{
    println!("Reading file: {}", filename);
    let mut lines = Vec::<String>::new();

    let file = fs::File::open(filename).expect("cant open file for reading lines {filename}"); // Open the file
    let buf_reader = io::BufReader::new(file); // Wrap it in a buffered reader

    for line in buf_reader.lines() {
        lines.push(line.expect("cant read line"));
    }
    lines
}

pub fn create_folder_if_not_exists(folder_name: &str) {
    if !fs::metadata(folder_name).is_ok() {
        fs::create_dir(folder_name).expect("cant create folder name {folder_name}");
        println!("Folder '{}' created successfully.", folder_name);
    } else {
        println!("Folder '{}' already exists.", folder_name);
    }

 
}


impl VCS {
    pub fn init(mut self) {
        create_folder_if_not_exists("vsc");
        create_folder_if_not_exists("vsc/objects");
        create_folder_if_not_exists("vcs/config");

        self.current_path =


    }

    pub fn add(filename: &str) {
        let filename = create_blob_file(filename);
    }
}

