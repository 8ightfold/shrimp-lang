pub use colored::*;

macro_rules! ERR_MSG {
    () => { "error".red() }
}

macro_rules! print_error {
    ({ $pre_fmt:literal}) => {
        crate::shrimp::utility::print_error_impl1(line!(), file!(), format!($pre_fmt))
    };
    ({ $pre_fmt:literal, $($pre:tt)* }) => {
        crate::shrimp::utility::print_error_impl1(line!(), file!(), format!($pre_fmt, $($pre)*))
    };

    ({ $pre_fmt:literal}, { $post_fmt:literal}) => {
        crate::shrimp::utility::print_error_impl2(line!(), file!(), format!($pre_fmt), format!($post_fmt))
    };
    ({ $pre_fmt:literal, $($pre:tt)* }, { $post_fmt:literal}) => {
        crate::shrimp::utility::print_error_impl2(line!(), file!(), format!($pre_fmt, $($pre)*), format!($post_fmt))
    };
    ({ $pre_fmt:literal}, { $post_fmt:literal, $($post:tt)* }) => {
        crate::shrimp::utility::print_error_impl2(line!(), file!(), format!($pre_fmt), format!($post_fmt, $($post)*))
    };
    ({ $pre_fmt:literal, $($pre:tt)* }, { $post_fmt:literal, $($post:tt)* }) => {
        crate::shrimp::utility::print_error_impl2(line!(), file!(), format!($pre_fmt, $($pre)*), format!($post_fmt, $($post)*))
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