use std::{fs, io};
use std::fs::File;
use std::io::BufRead;

use sha1::{Sha1, Digest};
use crate::definition;
use crate::definition::Object;

pub fn get_hash(filename: &str) -> String{
    let file = fs::File::open(filename).expect("Failed to open file for reading lines");
    let buf_reader = io::BufReader::new(file);

    let mut hasher = Sha1::new();

    for line in buf_reader.lines() {
        let line = line.expect("Failed to read line");
        hasher.update(line.as_bytes());
    }

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    let result = hasher.finalize();
    let hashvaluehex = hash_array_to_string(&result);
    println!("Hash: {:?}", hash_array_to_string(&result));
    hashvaluehex
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

    let file = fs::File::open(filename).expect("cant open file for reading lines "); // Open the file
    let buf_reader = io::BufReader::new(file); // Wrap it in a buffered reader

    for line in buf_reader.lines() {
        lines.push(line.expect("cant read line"));
    }
    lines
}

impl Object {


    pub fn save(file: &File) {
       create_folder_if_not_exists("vsc/objects").expect("TODO: panic message");


    }


}

pub fn create_folder_if_not_exists(folder_name: &str) -> Result<(), std::io::Error> {
    if !fs::metadata(folder_name).is_ok() {
        fs::create_dir(folder_name)?;
        println!("Folder '{}' created successfully.", folder_name);
    } else {
        println!("Folder '{}' already exists.", folder_name);
    }

    Ok(())
}

