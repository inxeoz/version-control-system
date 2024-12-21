use serde_json::Value;
use crate::feature::{create_file_if_not_exists, create_folder_if_not_exists, print_hierarchy, read_hierarchy_from_file, save_hierarchy_to_file};
mod feature;
mod parse;

fn main() {
   // traverse_json();
    feature::main();
    init();

}


fn init() {
    create_folder_if_not_exists("version_control_system");
    create_folder_if_not_exists("version_control_system/objects");
    create_folder_if_not_exists("version_control_system/config");
   create_file_if_not_exists("config.txt", "version_control_system/config");
    create_file_if_not_exists("struct.json", "version_control_system/config");
   save_hierarchy_to_file("test_fold","version_control_system/config/struct.json", );
 //
 //    let json_file_path = "version_control_system/config/struct.json";
 //    let hierarchy = read_hierarchy_from_file(json_file_path);
 //    println!("Folder Hierarchy:");
 // print_hierarchy(&hierarchy, 0);

}


