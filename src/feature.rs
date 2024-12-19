use std::fs::File;
use std::io::BufRead;
use std::{env, fs, io};

use crate::definition;
use crate::definition::{RepoNode, VCS};
use sha1::{Digest, Sha1};
use std::io::prelude::*;
use std::path::Path;

impl VCS {
    pub fn init(&mut self) {
        Self::create_folder_if_not_exists("vcs");
        Self::create_folder_if_not_exists("vcs/objects");
        Self::create_folder_if_not_exists("vcs/config");
        println!("VCS initialized successfully.");
    }

    pub fn create_folder_if_not_exists(folder_name: &str) {
        if !fs::metadata(folder_name).is_ok() {
            fs::create_dir(folder_name).expect("cant create folder name {folder_name}");
            println!("Folder '{}' created successfully.", folder_name);
        } else {
            println!("Folder '{}' already exists.", folder_name);
        }
    }

    pub fn get_current_path() -> String {
        let current_dir = env::current_dir().expect("Failed to retrieve current directory");
        current_dir.to_string_lossy().to_string()
    }

    pub fn is_folder_or_file_ignored(&self, entity_name: &str) -> bool {
        match &self.ignoreby {
            None => {
                return false;
            }
            Some(ignore_file_name) => {
                if let Ok(file) = File::open(ignore_file_name) {
                    let reader = io::BufReader::new(file);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            if line.trim() == format!("/{}", entity_name)
                                || line.trim() == entity_name
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    pub fn print_folder_contents(&self, folder_path: &str) {
        if let Ok(entries) = fs::read_dir(folder_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_dir() {
                        println!("on this path: {}", path.to_str().unwrap());
                        let folder_name = path
                            .file_name()
                            .expect("Failed to retrieve folder name")
                            .to_str()
                            .unwrap();

                        println!("folder name: {}", folder_name);

                        if !Self::is_folder_or_file_ignored(&self, folder_name) {
                            Self::print_folder_contents(&self, path.to_str().unwrap());
                        }

                        println!("end of  this path: {}", path.to_str().unwrap());
                    } else {
                        let file_name = path
                            .file_name()
                            .unwrap()
                            .to_str()
                            .expect("Failed to retrieve file name");
                        if !Self::is_folder_or_file_ignored(&self, file_name) {
                            println!("File: {}", file_name);
                        }
                    }
                }
            }
        }
    }

    pub fn create_blob_file(filename: &str) -> String {
        // Read the content of the file
        let file_content = Self::read_file_content(filename);
        let hashstring_as_name = Self::get_hash(filename);
        // Create the blob file
        let blob_filename = format!("vsc/objects/{}.blob", hashstring_as_name);
        let mut blob_file = File::create(&blob_filename).expect("cant create blob file");

        // Write file content to the blob file
        blob_file
            .write_all(file_content.as_bytes())
            .expect("cant write content to blob file");

        println!("Blob file created successfully: {}", blob_filename);
        blob_filename
    }

    pub fn read_file_content(filename: &str) -> String {
        let mut file = File::open(filename).expect("cant open file {filename}");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("cant read file content {filename}");
        content
    }

    pub fn get_hash(filename: &str) -> String {
        let content_string = Self::read_file_content(filename);
        let hashstring = Self::get_hashstring_by_string(&content_string);
        hashstring
    }

    pub fn get_hashstring_by_file(file: File) -> String {
        let buf_reader = io::BufReader::new(file);
        let mut hasher = Sha1::new();
        for line in buf_reader.lines() {
            let line = line.expect("Failed to read line");
            hasher.update(line.as_bytes());
        }
        let result = hasher.finalize();
        let hashvaluehex = Self::hash_array_to_string(&result);
        println!("Hash: {:?}", Self::hash_array_to_string(&result));
        hashvaluehex
    }

    pub fn get_hashstring_by_string(content_string: &String) -> String {
        let mut hasher = Sha1::new();
        hasher.update(content_string.as_bytes());
        let result = hasher.finalize();
        Self::hash_array_to_string(&result)
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
        let old_content = Self::read_file_by_lines(oldfile);
        let new_content = Self::read_file_by_lines(newfile);

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
                    removed_vec.push(definition::Line {
                        line_number: line_no + 1,
                        content: old.to_string(),
                    });
                    println!("+ Added to line {}: {}", line_no + 1, new);
                    added_vec.push(definition::Line {
                        line_number: line_no + 1,
                        content: new.to_string(),
                    });
                }
                (Some(old), None) => {
                    println!("- Removed from line {}: {}", line_no + 1, old);
                    removed_vec.push(definition::Line {
                        line_number: line_no + 1,
                        content: old.to_string(),
                    });
                }
                (None, Some(new)) => {
                    println!("+ Added to line {}: {}", line_no + 1, new);
                    added_vec.push(definition::Line {
                        line_number: line_no + 1,
                        content: new.to_string(),
                    });
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

    fn read_file_by_lines(filename: &str) -> Vec<String> {
        println!("Reading file: {}", filename);
        let mut lines = Vec::<String>::new();

        let file = fs::File::open(filename).expect("cant open file for reading lines {filename}"); // Open the file
        let buf_reader = io::BufReader::new(file); // Wrap it in a buffered reader

        for line in buf_reader.lines() {
            lines.push(line.expect("cant read line"));
        }
        lines
    }

    // pub fn add_nodes(&self, parent_node: &mut RepoNode, folder_path: &str) {
    //     if let Ok(entries) = fs::read_dir(folder_path) {
    //         for entry in entries.flatten() {
    //             let path = entry.path();
    //
    //             if path.is_dir() {
    //                 let folder_name = path
    //                     .file_name()
    //                     .expect("Failed to retrieve folder name")
    //                     .to_str()
    //                     .unwrap();
    //
    //                 println!("on this path: {}", path.to_str().unwrap());
    //                 println!("folder name: {}", folder_name);
    //
    //                 let mut node = RepoNode {
    //                     file_or_folder_name: folder_name.to_string(),
    //                     file_or_folder_path: None,
    //                     current_hash_value: None,
    //                     is_folder: true,
    //                     parent_hash_value: None,
    //                     children: None,
    //                 };
    //
    //                 parent_node
    //                     .children
    //                     .get_or_insert_with(Vec::new)
    //                     .push(node);
    //
    //                 if !Self::is_folder_or_file_ignored(&self, folder_name) {
    //                     Self::add_nodes(&self, node, path.to_str().unwrap());
    //                 }
    //
    //                 println!("end of this path: {}", path.to_str().unwrap());
    //             } else {
    //                 let file_name = path
    //                     .file_name()
    //                     .expect("Failed to retrieve file name")
    //                     .to_str()
    //                     .expect("Failed to retrieve file name");
    //
    //                 if !Self::is_folder_or_file_ignored(self, file_name) {
    //                     println!("File: {}", file_name);
    //
    //                     let node = RepoNode {
    //                         file_or_folder_name: file_name.to_string(),
    //                         file_or_folder_path: None,
    //                         current_hash_value: None,
    //                         is_folder: false,
    //                         parent_hash_value: None,
    //                         children: None,
    //                     };
    //
    //                     parent_node
    //                         .children
    //                         .get_or_insert_with(Vec::new)
    //                         .push(node);
    //                 }
    //             }
    //         }
    //     }
    // }

}
