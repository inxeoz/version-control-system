
    use std::fs;





    pub struct Line {
        pub line_number: usize,
        pub content: String,
    }
    pub struct Diff {
        pub added: Option<Vec<Line>>,
        pub removed:  Option<Vec<Line>>
    }

    pub struct Version<HashType> {
        pub hash_value: HashType,
        pub diff_from_previous: Diff,
    }


    pub struct Object {
        pub path: String,
        pub file_main: fs::File,  // Main file content
        pub versions: Option<Vec<Version<String>>>,
    }


    pub fn print_line(line: &Line) {
        println!("Line Number: {}, Content: {}", line.line_number, line.content);
    }

    pub fn print_diff(diff: &Diff) {
        if let Some(added_lines) = &diff.added {
            println!("Added Lines:");
            for line in added_lines {
                print_line(line);
            }
        }

        if let Some(removed_lines) = &diff.removed {
            println!("Removed Lines:");
            for line in removed_lines {
                print_line(line);
            }
        }
    }

    pub fn print_version<HashType>(version: &Version<HashType>)
    where
        HashType: Clone + std::fmt::Debug,
    {
        println!("Hash Value: {:?}", version.hash_value);
        print_diff(&version.diff_from_previous);
    }

    pub fn print_object(object: &Object) {
        // Print main file content or other relevant information from Object struct
        // Note: You may need to handle printing a file in a different manner than described here
        if let Some(versions) = &object.versions {
            for version in versions {
                print_version(version);
            }
        }
    }


