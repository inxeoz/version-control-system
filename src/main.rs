
use std::env;
use crate::definition::{RepoNode, VCS};

mod feature;
mod definition;

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
    ignoreby(ignoreby)
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

    let vcs = VCS {
        current_path: VCS::get_current_path(),
        repo_root: None,
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







