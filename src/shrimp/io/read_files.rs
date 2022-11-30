use std::env::{args, current_dir};

use crate::shrimp;
use shrimp::utility::File_handle;
use shrimp::utility::print_error;


fn get_filenames() -> Vec<String> {
    let args: Vec<String> = args().collect();
    let ret_slice = &args[1..];
    ret_slice.to_vec()
}

pub fn read_files() -> Option<Vec<File_handle>> {
    let args = get_filenames();
    let mut files: Vec<File_handle> = Vec::new();
    files.reserve(args.len());

    let working_directory = match current_dir() {
        Ok(val) => val,
        Err(error) => {
            print_error!(format!("{error}"));
            std::process::exit(-1)
        }
    };

    for arg in args {
        match arg {
            _ => {
                let mut path = working_directory.clone();
                path.push(arg);

                let path_str = path.to_str().unwrap();
                let mut cloned_path = String::new();
                cloned_path.reserve(path_str.len());
                cloned_path.insert_str(0, path_str);
                files.push(File_handle::new(&cloned_path))
            }
        }
    }

    Some(files)
}