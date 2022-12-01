use std::env::args;

use crate::shrimp;
use shrimp::utility::File_handle;


fn get_filenames() -> Vec<String> {
    let args: Vec<String> = args().collect();
    let ret_slice = &args[1..];
    ret_slice.to_vec()
}

pub fn read_files() -> Option<Vec<File_handle>> {
    let args = get_filenames();
    let mut files: Vec<File_handle> = Vec::new();
    files.reserve(args.len());

    for arg in args {
        match arg.as_str() {
            "-P" | "-p" | "--package" => {},
            "-B" | "-b" | "--binary" => {},
            _ => {
                files.push(File_handle::new(&arg.clone()))
            }
        }
    }

    Some(files)
}