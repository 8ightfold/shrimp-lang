use std::env::{args, current_dir};
use std::fs::File;
use std::path::Path;

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
    let mut files: Vec<std::io::Result<File>> = args.iter().map(|pathname| File::create(pathname)).collect();
    let mut valid_files: Vec<File_handle> = Vec::new();
    valid_files.reserve(files.len());

    let working_directory = match current_dir() {
        Ok(val) => val,
        Err(error) => {
            print_error!(format!("{error}"));
            std::process::exit(-1)
        }
    };

    for(idx, file) in files.drain(..).enumerate() {
        match file {
            Ok(_) => {
                let mut path = working_directory.clone();
                path.push(&args[idx]);
                let file_box = File::create( Path::new(&path.as_path()));
                match file_box {
                    Ok(val) => {
                        let path_str = path.to_str().unwrap();
                        let mut cloned_path = String::new();
                        cloned_path.reserve(path_str.len());
                        cloned_path.insert_str(0, path_str);
                        valid_files.push(File_handle::new(&cloned_path, val.try_clone().unwrap()))
                    },
                    Err(_) => return None,
                }
            },
            Err(error) => {
                print_error!(format!("couldn't open {}", args[idx]), format!("{error}"));
                return None
            },
        }
    }

    Some(valid_files)
}