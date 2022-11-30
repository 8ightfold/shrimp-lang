pub use colored::*;

macro_rules! ERR_MSG {
    () => { "error".red() }
}

macro_rules! print_error {
    ($pre:expr) => {
        crate::shrimp::utility::print_error_impl1(line!(), file!(), $pre)
    };
    ($pre:expr, $post:expr) => {
        crate::shrimp::utility::print_error_impl2(line!(), file!(), $pre, $post)
    };
}


pub(crate) use ERR_MSG;
pub(crate) use print_error;


#[allow(dead_code)]
pub fn print_error_impl1(line: u32, file: &str, pre: String) {
    eprintln!("{}: {pre}\n   {}", ERR_MSG!(), format!("---> {file}:{line}").blue());
}

#[allow(dead_code)]
pub fn print_error_impl2(line: u32, file: &str, pre: String, post: String) {
    eprintln!("{}: {pre}\n   {}\n{post}", ERR_MSG!(), format!("---> {file}:{line}").blue());
}