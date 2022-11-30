use colored::*;
use crate::shrimp::utility::print_error;


pub struct File_handle {
    filepath: String,
    filedata: String,
}

impl std::fmt::Display for File_handle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return if self.filedata.len() != 0 {
            write!(f, "{}: {}\n{}", "file".blue(), self.filepath, self.filedata)
        }
        else { write!(f, "{}: {}\n{}", "file".blue(), self.filepath, "<empty>".red()) }
    }
}

impl File_handle {
    pub fn new(path: &String) -> Self {
        let mut data = String::new();
        let read_result = std::fs::read_to_string(path);
        match read_result {
            Err(error) => {
                print_error!(format!("error reading file {path}"), format!("{error}"));
            },
            Ok(val) => {
                data.clone_from(&val);
            }
        }
        return Self { filepath: path.clone(), filedata: data }
    }

    #[allow(dead_code)]
    pub fn display(&self) -> &String {
        &self.filepath
    }

    #[allow(dead_code)]
    pub fn handle(&self) -> &String { &self.filedata }
}