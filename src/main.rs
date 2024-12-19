use crate::definition::{RepoNode, VCS};
use std::env;

mod definition;
mod feature;

use argh::FromArgs;

#[derive(FromArgs)]
/// CLI program with subcommands
struct TopLevel {
    #[argh(subcommand)]
    command: Command,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Command {
    ignoreby(ignoreby),
}

#[derive(FromArgs)]
/// Upload files
#[argh(subcommand, name = "ignoreby")]
struct ignoreby {
    #[argh(positional)]
    file_name: String,
}

fn main() {
    let args: TopLevel = argh::from_env();
    match args.command {
        Command::ignoreby(ignore) => println!("Uploading file: {}", ignore.file_name),
    }

    let path = VCS::get_current_path();
    let path_folder_name = path.split("/").last().unwrap_or("").trim().to_string();

    let root_node = RepoNode {
        file_or_folder_name: path,
        file_or_folder_path: Some ( path_folder_name ),
        current_hash_value: None,
        is_folder: true,
        parent_hash_value: None,
        children: None,
    };

    let vcs = VCS {
        current_path: VCS::get_current_path(),
        repo_root: Some(root_node),
        ignoreby: Some("vcs.ignore".to_string()),
    };

    vcs.print_folder_contents(vcs.current_path.as_str())
}

// fn main() {
//     let oldfile = "old_file.txt";
//     let newfile = "new_file.txt";
//
//     let diff = feature::compare_files(oldfile, newfile);
//     definition::print_diff(&diff);
//     let hashvalue = feature::get_hash(newfile);
//
//
//
//     let version =definition:: Version {
//         hash_value: feature::get_hash(newfile),
//         diff_from_previous: diff
//     };
//
//
//
// }
