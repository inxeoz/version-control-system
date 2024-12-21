use crate::feature::{create_file_if_not_exists, create_folder_if_not_exists, print_hierarchy, read_hierarchy_from_file, save_hierarchy_to_file};
mod feature;
mod parse;

fn main() {
   init();

}


fn init() {
    create_folder_if_not_exists("vcs");
    create_folder_if_not_exists("vcs/objects");
    create_folder_if_not_exists("vcs/config");
    create_file_if_not_exists("config.txt", "vcs/config");
    create_file_if_not_exists("struct.json", "vcs/config");
    save_hierarchy_to_file("test_fold","vcs/config/struct.json", );
 //
 //    let json_file_path = "vcs/config/struct.json";
 //    let hierarchy = read_hierarchy_from_file(json_file_path);
 //    println!("Folder Hierarchy:");
 // print_hierarchy(&hierarchy, 0);

}
