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




// pub fn is_folder_or_file_ignored( entity_name: &str) -> bool {
//     match &self.ignoreby {
//         None => {
//             return false;
//         }
//         Some(ignore_file_name) => {
//             if let Ok(file) = File::open(ignore_file_name) {
//                 let reader = io::BufReader::new(file);
//                 for line in reader.lines() {
//                     if let Ok(line) = line {
//                         if line.trim() == format!("/{}", entity_name)
//                             || line.trim() == entity_name
//                         {
//                             return true;
//                         }
//                     }
//                 }
//             }
//         }
//     }
//
//     false
// }

// pub fn print_folder_contents( folder_path: &str) {
//     if let Ok(entries) = fs::read_dir(folder_path) {
//         for entry in entries {
//             if let Ok(entry) = entry {
//                 let path = entry.path();
//
//                 if path.is_dir() {
//                     println!("on this path: {}", path.to_str().unwrap());
//                     let folder_name = path
//                         .file_name()
//                         .expect("Failed to retrieve folder name")
//                         .to_str()
//                         .unwrap();
//
//                     println!("folder name: {}", folder_name);
//
//                     if !Self::is_folder_or_file_ignored(&self, folder_name) {
//                         Self::print_folder_contents(&self, path.to_str().unwrap());
//                     }
//
//                     println!("end of  this path: {}", path.to_str().unwrap());
//                 } else {
//                     let file_name = path
//                         .file_name()
//                         .unwrap()
//                         .to_str()
//                         .expect("Failed to retrieve file name");
//                     if !Self::is_folder_or_file_ignored(&self, file_name) {
//                         println!("File: {}", file_name);
//                     }
//                 }
//             }
//         }
//     }
// }


// pub fn compare_files(oldfile: &str, newfile: &str) -> definition::Diff {
//     let mut added_vec = Vec::<definition::Line>::new();
//     let mut removed_vec = Vec::<definition::Line>::new();
//
//     // Read Old File and New File line by line into vectors
//     let old_content = Self::read_file_by_lines(oldfile);
//     let new_content = Self::read_file_by_lines(newfile);
//
//     // Determine the maximum length between the two files
//     let max_lines = old_content.len().max(new_content.len());
//
//     // Print added and removed lines along with line numbers
//     println!("Changes between files:");
//     for line_no in 0..max_lines {
//         let old_line = old_content.get(line_no); // Line from the old file
//         let new_line = new_content.get(line_no); // Line from the new file
//
//         match (old_line, new_line) {
//             (Some(old), Some(new)) if old != new => {
//                 println!("- Removed from line {}: {}", line_no + 1, old);
//                 removed_vec.push(definition::Line {
//                     line_number: line_no + 1,
//                     content: old.to_string(),
//                 });
//                 println!("+ Added to line {}: {}", line_no + 1, new);
//                 added_vec.push(definition::Line {
//                     line_number: line_no + 1,
//                     content: new.to_string(),
//                 });
//             }
//             (Some(old), None) => {
//                 println!("- Removed from line {}: {}", line_no + 1, old);
//                 removed_vec.push(definition::Line {
//                     line_number: line_no + 1,
//                     content: old.to_string(),
//                 });
//             }
//             (None, Some(new)) => {
//                 println!("+ Added to line {}: {}", line_no + 1, new);
//                 added_vec.push(definition::Line {
//                     line_number: line_no + 1,
//                     content: new.to_string(),
//                 });
//             }
//             _ => {} // No change
//         }
//     }
//
//     let diff = definition::Diff {
//         added: Some(added_vec),
//         removed: Some(removed_vec),
//     };
//     diff
// }


// pub fn add_nodes(&self, parent_node: Rc<RefCell<RepoNode>>, folder_path: &str) {
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
//                 let node = RepoNode::new(folder_name.to_string(), true);
//
//                 // Mutate parent_node (wrap in RefCell to mutate it)
//                 parent_node
//                     .borrow_mut()
//                     .children
//                     .get_or_insert_with(Vec::new)
//                     .push(node.clone());
//
//                 if !Self::is_folder_or_file_ignored(self, folder_name) {
//                     Self::add_nodes(self, node, path.to_str().unwrap());
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
//                     let node = RepoNode::new(file_name.to_string(), false);
//
//                     parent_node
//                         .borrow_mut()
//                         .children
//                         .get_or_insert_with(Vec::new)
//                         .push(node);
//                 }
//             }
//         }
//     }
// }
